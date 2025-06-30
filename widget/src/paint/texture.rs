pub struct Texture {
    data: wgpu::Texture,
}

impl Texture {
    pub(crate) fn from_texture(texture: wgpu::Texture) -> Self {
        Self { data: texture }
    }
}
