use wgpu::{Extent3d, Texture, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView};

pub(super) struct DepthBuffer {
    _texture: Texture,
    view: TextureView
}

pub(super) const DEPTH_FORMAT: TextureFormat =
    TextureFormat::Depth32Float;

impl DepthBuffer {
    pub(super) fn new(
        device: &wgpu::Device,
        width: u32,
        height: u32,
    ) -> Self {
        let descriptor = TextureDescriptor {
            label: Some("depth texture"),

            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },

            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: DEPTH_FORMAT,
            usage: TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        };

        let _texture = device.create_texture(&descriptor);
        let view = _texture.create_view(&Default::default());

        Self {
            _texture,
            view,
        }
    }

    pub(super) fn view(&self) -> &TextureView {
        &self.view
    }
}