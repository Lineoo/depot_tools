use std::sync::Arc;

use glam::Mat4;
use wgpu::{
    CommandEncoder,
    util::{BufferInitDescriptor, DeviceExt},
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::vertex::{VERTEX_LIST, create_vertex_buffer_layout};

pub const DRAG_HANDLE_WIDTH: u32 = 10;
pub const WINDOW_SIZE: [u32; 2] = [300, 20];

/// Drawing context for widget module
///
/// This struct should be re-created every frame, and dropped after the frame is done.
/// Generally, it should live with a `Window` and be distributed to the widgets for painting.
pub struct WgpuCtx {
    // pub(crate) surface: wgpu::Surface<'window>,
    pub(crate) surface_config: wgpu::SurfaceConfiguration,
    pub(crate) surface_texture: wgpu::SurfaceTexture,
    pub(crate) _adapter: wgpu::Adapter,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) render_pipeline: wgpu::RenderPipeline,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) encoder: Option<CommandEncoder>,
}

#[derive(Debug, Clone, Copy)]
pub struct Color(u32, u32, u32, u32);

#[derive(Debug, Clone, Copy)]
pub struct Triangle<T> {
    pub vertices: [T; 3],
    pub indices: [usize; 3],
    pub color: Color,
}

impl WgpuCtx {
    pub fn new(window: &Window) -> Self {
        pollster::block_on(WgpuCtx::new_async(window))
    }

    pub async fn new_async(window: &Window) -> Self {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("failed to find an adapter!");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                    memory_hints: wgpu::MemoryHints::default(),
                },
                None,
            )
            .await
            .expect("failed to create device!");

        let size = window.inner_size();
        let width = size.width.max(1);
        let height = size.height.max(1);

        let surface_config = surface.get_default_config(&adapter, width, height).unwrap();
        surface.configure(&device, &surface_config);

        let render_pipeline = create_pipeline(&device, surface_config.format);

        let bytes: &[u8] = bytemuck::cast_slice(VERTEX_LIST);
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });

        WgpuCtx {
            // surface,
            surface_config,
            surface_texture: surface.get_current_texture().unwrap(),
            _adapter: adapter,
            device,
            queue,
            render_pipeline,
            vertex_buffer,
            encoder: None,
        }
    }

    pub(crate) fn to_texture(&mut self) -> wgpu::Texture {
        self.surface_texture.texture.clone()
    }

    pub(crate) fn clear(&mut self) {
        todo!()
    }

    // #[deprecated]
    // pub fn draw(&mut self) {
    //     let surface_texture = self
    //         .surface
    //         .get_current_texture()
    //         .expect("failed to get next surface!");
    //     let texture_view = surface_texture
    //         .texture
    //         .create_view(&wgpu::TextureViewDescriptor::default());
    //     let mut encoder = self
    //         .device
    //         .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    //     let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    //         label: None,
    //         color_attachments: &[Some(wgpu::RenderPassColorAttachment {
    //             view: &texture_view,
    //             resolve_target: None,
    //             ops: wgpu::Operations {
    //                 load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
    //                 store: wgpu::StoreOp::Store,
    //             },
    //         })],
    //         depth_stencil_attachment: None,
    //         timestamp_writes: None,
    //         occlusion_query_set: None,
    //     });
    //     render_pass.set_pipeline(&self.render_pipeline);
    //     render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
    //     render_pass.draw(0..VERTEX_LIST.len() as u32, 0..1);
    //     drop(render_pass);

    //     self.queue.submit(Some(encoder.finish()));
    //     surface_texture.present();
    // }

    // pub fn resize(&mut self, size: PhysicalSize<u32>) {
    //     self.surface_config.width = size.width.max(1);
    //     self.surface_config.height = size.height.max(1);
    //     self.surface.configure(&self.device, &self.surface_config);
    // }

    // pub fn begin(&mut self) {
    //     // FIXME: ai gen
    //     let surface_texture = self
    //         .surface
    //         .get_current_texture()
    //         .expect("failed to get next surface!");
    //     let texture_view = surface_texture
    //         .texture
    //         .create_view(&wgpu::TextureViewDescriptor::default());
    //     let mut encoder = self
    //         .device
    //         .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    //     let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    //         label: None,
    //         color_attachments: &[Some(wgpu::RenderPassColorAttachment {
    //             view: &texture_view,
    //             resolve_target: None,
    //             ops: wgpu::Operations {
    //                 load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
    //                 store: wgpu::StoreOp::Store,
    //             },
    //         })],
    //         depth_stencil_attachment: None,
    //         timestamp_writes: None,
    //         occlusion_query_set: None,
    //     });
    //     render_pass.set_pipeline(&self.render_pipeline);
    //     render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
    //     render_pass.draw(0..VERTEX_LIST.len() as u32, 0..1);
    //     drop(render_pass);

    //     self.encoder = Some(encoder);
    //     // TODO: complete
    //     // self.queue.submit(Some(encoder.finish()));
    // }

    // pub fn end(&mut self) {
    //     let Some(encoder) = self.encoder.take() else {
    //         return;
    //     };
    //     self.queue.submit(Some(encoder.finish()));
    //     todo!() // finish end()
    // }

    /*     pub fn triangle_f32(&mut self, triangle: Triangle<f32>) {
        // FIXME: ai gen
        let bytes: &[u8] = bytemuck::cast_slice(&VERTEX_LIST);
        self.vertex_buffer = self.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });
    }

    pub fn triangle_u32(&mut self, triangle: Triangle<u32>) {
        todo!()
    } */
}

fn create_pipeline(
    device: &wgpu::Device,
    swap_chain_format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
            "../shaders/shader.wgsl"
        ))),
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: None,
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[create_vertex_buffer_layout()],
            compilation_options: Default::default(),
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            ..Default::default()
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            compilation_options: Default::default(),
            targets: &[Some(swap_chain_format.into())],
        }),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    })
}

/// Generate a proper matrix for window at given size
fn make_mat(w: u32, h: u32) -> Mat4 {
    /* let scale = glam::Mat4::from_scale(glam::Vec3::new(w as f32, h as f32, 1.0));
    let translate = glam::Mat4::from_translation(glam::Vec3::new(0.0, 0.0, 0.0));
    let rotate = glam::Mat4::from_rotation_z(0.0);
    scale * rotate * translate */
    Mat4::orthographic_lh(0.0f32, w as f32, 0.0f32, h as f32, 0.0f32, 100.0f32)
}
