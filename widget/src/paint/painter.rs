//! This module contains the [`Painter`] struct, which is used to create drawing commands.

use wgpu::{CommandEncoder, Texture, TextureView};

/// Makes drawing commands on an existing Window.
///
/// It cannot be created by users. Instead, drawing functions will give the proper Painter.
///
/// Note that any drawing commands will not be applied immediately, but be submitted when
/// [`Painter::flush()`] is called or when the painter is dropped.
pub struct Painter<'x> {
    texture: &'x Texture,
    view: TextureView,
    encoder: Option<CommandEncoder>,
}

impl<'x> Painter<'x> {
    pub(crate) fn new(texture: &'x Texture) -> Self {
        Self {
            texture,
            view: texture.create_view(&wgpu::TextureViewDescriptor::default()),
            encoder: None,
        }
    }
}
