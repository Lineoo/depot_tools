use crate::size::Size;

pub struct Texture {
    texture: wgpu::Texture,
    size: Size,
}
