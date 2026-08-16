use wgpu::util::{BufferInitDescriptor, DeviceExt};
use crate::renderer::vertex::Vertex;

pub struct Mesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    indexes: u32,
}

impl Mesh {
    pub fn new(device: &wgpu::Device, vertex: &[Vertex], indexes: &[u16]) -> Self {
        let vertex_buffer =
            device.create_buffer_init(&BufferInitDescriptor {
                label: Some("main vertex buffer"),
                contents: bytemuck::cast_slice(&vertex),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer =
            device.create_buffer_init(&BufferInitDescriptor {
                label: Some("main index buffer"),
                contents: bytemuck::cast_slice(&indexes),
                usage: wgpu::BufferUsages::INDEX,
            });

        Self {
            vertex_buffer,
            index_buffer,
            indexes: indexes.len() as u32,
        }
    }

    pub fn vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }

    pub fn index_buffer(&self) -> &wgpu::Buffer {
        &self.index_buffer
    }

    pub fn index_count(&self) -> u32 {
        self.indexes
    }
}