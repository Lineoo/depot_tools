use std::rc::{Rc, Weak};

use crate::{control::*, window::Window};

pub struct Button {
    label: String,
    width: u32,
    height: u32,
    parent: Weak<dyn Control>,
    ancestor: *const Window,
}

impl Button {
    pub fn new(label: &str, width: u32, height: u32) -> Rc<Self> {
        Rc::new(Button {
            label: label.to_string(),
            width,
            height,
            parent: Weak::<Self>::new(),
            ancestor: std::ptr::null(),
        })
    }

    pub fn text(&mut self, label: &str) {
        self.label = label.to_string();
    }
}

impl Control for Button {
    fn set_parent(self: &mut Self, parent: Weak<dyn Control>) {
        self.parent = parent;
    }

    fn get_parent(self: &Self) -> Weak<dyn Control> {
        self.parent.clone()
    }

    fn get_parent_mut(self: &mut Self) -> Weak<dyn Control> {
        self.parent.clone()
    }

    fn ancestor(self: &Self) -> *const crate::window::Window {
        self.ancestor
    }

    fn ancestor_mut(self: &Self) -> *mut crate::window::Window {
        self.ancestor as *mut Window
    }

    fn paint(self: &Self, painter: &mut crate::wgpu_ctx::WgpuCtx) {
        todo!();
    }

    fn reg_tree(self: &Self, window: &mut Window) {
        todo!();
        // window.add_control(self as *const dyn Control as *mut dyn Control);
    }

    fn address(self: &Self) -> *const dyn Control {
        self as *const dyn Control
    }

    fn address_mut(self: &mut Self) -> *mut dyn Control {
        self as *mut dyn Control
    }
}
