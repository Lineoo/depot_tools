use std::rc::Weak;

use crate::{wgpu_ctx::WgpuCtx, window::Window};

pub trait Control {
    fn set_parent(self: &mut Self, parent: Weak<dyn Control>);
    fn get_parent(self: &Self) -> Weak<dyn Control>;
    fn get_parent_mut(self: &mut Self) -> Weak<dyn Control>;
    fn paint(self: &Self, painter: &mut WgpuCtx);
    fn ancestor(self: &Self) -> *const Window;
    fn ancestor_mut(self: &Self) -> *mut Window;
    fn reg_tree(self: &Self, window: &mut Window);
    fn address(self: &Self) -> *const dyn Control;
    fn address_mut(self: &mut Self) -> *mut dyn Control;
}

pub trait Container: Control {
    fn get_child(self: &Self) -> Weak<dyn Control>;
    fn get_child_mut(self: &mut Self) -> Weak<dyn Control>;
}

pub trait Group: Control {
    fn rm_children(self: &mut Self);
}
