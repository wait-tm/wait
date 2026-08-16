mod vertex;
mod uniforms;
mod depth;
mod mesh;
mod debug_geometry;

use uniforms::CameraUniform;
use glam::Mat4;
use std::default::Default;
use std::sync::Arc;
use wgpu::{CompareFunction, DepthStencilState, Face, FrontFace, PrimitiveState, RenderPassDepthStencilAttachment};
use wgpu::PrimitiveTopology::TriangleList;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use vertex::Vertex;
use winit::window::Window;
use crate::renderer::debug_geometry::{CUBE_INDICES, CUBE_VERTICES};
use crate::renderer::depth::{DepthBuffer, DEPTH_FORMAT};
use crate::renderer::mesh::Mesh;

pub struct Renderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    mesh: Mesh,
    camera_buffer: wgpu::Buffer,
    depth_buffer: DepthBuffer,
    camera_bind_group: wgpu::BindGroup,
}

impl Renderer {
    pub async fn new(window: Arc<Window>, view_projection: Mat4) -> Self {
        let size = window.inner_size();

        let mut instance_descriptor =
            wgpu::InstanceDescriptor::new_without_display_handle();
        instance_descriptor.backends = wgpu::Backends::PRIMARY;

        let instance = wgpu::Instance::new(instance_descriptor);

        let surface = instance
            .create_surface(window)
            .expect("failed to create surface");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
                apply_limit_buckets: true
            })
            .await
            .expect("failed to find gpu adapter");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("main device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                memory_hints: wgpu::MemoryHints::default(),
                trace: wgpu::Trace::Off
            })
            .await
            .expect("failed to create wgpu device");

        let config = surface
            .get_default_config(
                &adapter,
                size.width.max(1),
                size.height.max(1),
            )
            .expect("surface is not supported by adapter");

        surface.configure(&device, &config);

        let mesh = Mesh::new(&device, CUBE_VERTICES, CUBE_INDICES);

        let camera_uniform = CameraUniform::new(view_projection);

        let camera_buffer =
            device.create_buffer_init(&BufferInitDescriptor {
                label: Some("main mod buffer"),
                contents: bytemuck::bytes_of(&camera_uniform),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
        
        let depth_buffer = DepthBuffer::new(&device, config.width, config.height);

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("mod bind group layout"),

                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,

                        visibility: wgpu::ShaderStages::VERTEX,

                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None
                        },

                        count: None,
                    }
                ]
            });

        let camera_bind_group =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("mod bind group"),
                layout: &camera_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: camera_buffer.as_entire_binding(),
                    }
                ]
            });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("basic shader"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("shaders/basic.wgsl").into()
            ),
        });

        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("main pipeline layout"),
                bind_group_layouts: &[Some(&camera_bind_group_layout)],
                immediate_size: 0,
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("main render pipeline"),
            layout: Some(&pipeline_layout),

            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex_main"),
                compilation_options: Default::default(),
                buffers: &[Some(Vertex::LAYOUT)],
            },

            primitive: PrimitiveState {
                topology: TriangleList,
                strip_index_format: Default::default(),
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: Default::default(),
                polygon_mode: Default::default(),
                conservative: Default::default(),
            },
            depth_stencil: Some(DepthStencilState {
                format: DEPTH_FORMAT,
                depth_write_enabled: Some(true),
                depth_compare: Some(CompareFunction::Less),
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),

            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })]
            }),

            multiview_mask: None,
            cache: None
        });


        Self {
            surface,
            device,
            queue,
            config,
            render_pipeline,
            mesh,
            camera_buffer,
            depth_buffer,
            camera_bind_group,
        }
    }

    pub fn render(&mut self, view_projection: Mat4) {
        let frame = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame) => frame,

            wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,

            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => {
                return;
            }

            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return;
            }

            wgpu::CurrentSurfaceTexture::Lost => {
                self.surface.configure(&self.device, &self.config);
                return;
            }
        };

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("main render encoder")
            });

        let camera_uniform = CameraUniform::new(view_projection);

        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::bytes_of(&camera_uniform)
        );

        {
            let mut render_pass = encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("main render pass"),

                    color_attachments: &[Some(
                        wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            depth_slice: None,

                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.1,
                                    g: 0.15,
                                    b: 0.2,
                                    a: 1.0
                                }),

                                store: wgpu::StoreOp::Store,
                            }
                        }
                    )],

                    depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                        view: self.depth_buffer.view(),
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });

            render_pass.set_pipeline(&self.render_pipeline);

            render_pass.set_bind_group(
                0,
                &self.camera_bind_group,
                &[]
            );

            render_pass.set_vertex_buffer(
                0,
                self.mesh.vertex_buffer().slice(..)
            );
            render_pass.set_index_buffer(
                self.mesh.index_buffer().slice(..),
                wgpu::IndexFormat::Uint16
            );

            render_pass.draw_indexed(
                0..self.mesh.index_count(),
                0,
                0..1
            );
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        self.queue.present(frame);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.config.width = width;
        self.config.height = height;

        self.depth_buffer = DepthBuffer::new(
            &self.device,
            self.config.width,
            self.config.height
        );

        self.surface.configure(&self.device, &self.config);
    }
}