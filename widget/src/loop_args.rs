pub struct LoopArgs<'a> {
    pub(crate) active_evt_loop: &'a winit::event_loop::ActiveEventLoop,
}

impl<'a> LoopArgs<'a> {
    pub(crate) fn new(active_evt_loop: &'a winit::event_loop::ActiveEventLoop) -> Self {
        LoopArgs { active_evt_loop }
    }
}
