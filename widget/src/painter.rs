//! This module contains the [`Painter`] struct, which is used to create drawing commands.

use wgpu::CommandEncoder;

use crate::wgpu_ctx::WgpuCtx;

/// Makes drawing commands on an existing Window.
///
/// It cannot be created by users. Instead, drawing functions will give the proper Painter.
///
/// Note that any drawing commands will not be applied immediately, but be submitted when
/// [`Painter::flush()`] is called or when the painter is dropped.
pub struct Painter<'ctx> {
    ctx: &'ctx mut WgpuCtx,
    encoder: Option<CommandEncoder>,
}

impl<'ctx> Painter<'ctx> {
    pub fn new(ctx: &'ctx mut WgpuCtx) -> Self {
        let mut r = Self { ctx, encoder: None };
        r.begin();
        r
    }

    #[doc(hidden)]
    /// Begin a new command encoder.
    /// This should be called before any drawing commands.
    /// It will automatically be called when the encoder is flushed.
    fn begin(&mut self) {
        self.encoder = Some(self.ctx.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Command Encoder"),
            },
        ));
    }

    /// Submit all commands and make itself ready for further paintings.
    pub fn flush(&mut self) {
        self.ctx
            .queue
            .submit(Some(self.encoder.take().unwrap().finish()));
        self.begin();
    }
}
