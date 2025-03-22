use std::sync::Arc;

use wgpu::util::{BufferInitDescriptor, DeviceExt};
use winit::{dpi::PhysicalSize, window::Window};

use crate::vertex::{VERTEXT_LIST, create_vertex_buffer_layout};

pub const DRAG_HANDLE_WIDTH: u32 = 10;
pub const WINDOW_SIZE: [u32; 2] = [300, 20];

pub struct WgpuCtx<'window> {
    surface: wgpu::Surface<'window>,
    surface_config: wgpu::SurfaceConfiguration,
    _adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
}

impl<'window> WgpuCtx<'window> {
    pub fn new(window: Arc<Window>) -> Self {
        pollster::block_on(WgpuCtx::new_async(window))
    }

    pub async fn new_async(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone()).unwrap();

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

        let bytes: &[u8] = bytemuck::cast_slice(&VERTEXT_LIST);
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });

        WgpuCtx {
            surface,
            surface_config,
            _adapter: adapter,
            device,
            queue,
            render_pipeline,
            vertex_buffer,
        }
    }

    pub fn draw(&mut self) {
        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("failed to get next surface!");
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..VERTEXT_LIST.len() as u32, 0..1);
        }
        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.surface_config.width = size.width.max(1);
        self.surface_config.height = size.height.max(1);
        self.surface.configure(&self.device, &self.surface_config);
    }
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
