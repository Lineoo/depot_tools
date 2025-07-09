use std::rc::Weak;

use crate::{wgpu_ctx::WgpuCtx, window::Window};

pub trait Control<'w> {
    fn set_parent(&mut self, parent: Weak<dyn Control<'w>>);
    fn get_parent(&self) -> Weak<dyn Control<'w>>;
    fn get_parent_mut(&mut self) -> Weak<dyn Control<'w>>;
    fn paint(&self, painter: &mut WgpuCtx);
    fn ancestor(&self) -> *const Window;
    fn ancestor_mut(&self) -> *mut Window;
    fn reg_tree(&self, window: &mut Window);
    // fn address(&self) -> *const dyn Control<'w>;
    // fn address_mut(&mut self) -> *mut dyn Control<'w>;
}

pub trait Container<'w>: Control<'w> {
    fn get_child(&self) -> Weak<dyn Control<'w>>;
    fn get_child_mut(&mut self) -> Weak<dyn Control<'w>>;
}

pub trait Group<'w>: Control<'w> {
    fn rm_children(&mut self);
}
