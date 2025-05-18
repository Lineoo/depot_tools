use crate::{painter::Painter, wgpu_ctx::WgpuCtx};

pub struct Window {
    pub(crate) win: winit::window::Window,
    pub(crate) ctx: WgpuCtx,
    pub on_size: Option<fn((i32, i32))>,
    pub before_close: Option<fn() -> bool>,
    pub on_paint: Option<fn()>,
    pub on_key_down: Option<fn(u32)>, // u32 is the virtual key code
    pub on_key_up: Option<fn(u32)>,   // key code
    pub on_mouse_down: Option<fn(u32, i32, i32)>, // mouse button, x, y
    pub on_mouse_up: Option<fn(u32, i32, i32)>, // mouse button, x, y
    pub on_mouse_move: Option<fn(i32, i32)>, // x, y
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WinHandle {
    pub(crate) id: winit::window::WindowId,
}

impl Window {
    pub(crate) fn from_native_win(win: winit::window::Window) -> Self {
        let ctx = WgpuCtx::new(&win);
        Window {
            win,
            ctx,
            on_size: None,
            before_close: None,
            on_paint: None,
            on_key_down: None,
            on_key_up: None,
            on_mouse_down: None,
            on_mouse_up: None,
            on_mouse_move: None,
        }
    }

    pub(crate) fn make_painter<'w>(&'w self) -> Painter<'w> {
        todo!()
    }

    pub fn handle(&self) -> WinHandle {
        WinHandle { id: self.win.id() }
    }

    pub fn show(&self, visible: bool) {
        self.win.set_visible(visible);
    }
}
