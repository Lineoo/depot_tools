use std::{
    any::{Any, TypeId},
    cell::RefCell,
    hash::Hash,
    num::NonZero,
    panic,
    rc::{Rc, Weak},
};

use anyhow::Error;

use crate::{
    application::IdType,
    control::{ctrl_mgr::CtrlMgr, util::focus_mgr::FocusMgr},
    event::{Event, SysEvent, win_init::WinInitEvent},
    paint::{painter::Painter, shapes::Rect},
    slot_handle::{Slot, SlotHandle},
    window::WindowDirector,
};

pub mod ctrl_ctx;
pub mod ctrl_mgr;
pub mod font;
// pub mod font_mgr;
pub mod handle;
pub mod image;
pub mod input_box;
pub mod list_widget;
pub mod rich_list_widget;
pub mod util;
pub mod vbox;

pub type Handle<T> = crate::control::handle::Handle<T>;
pub type WeakHandle<T> = crate::control::handle::WeakHandle<T>;
pub type EventArg = Option<Box<dyn Any>>;

pub type EventProc = Box<dyn FnMut(String, WeakHandle<dyn Control>, WeakHandle<dyn Control>)>; // name, provider, receiver

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ControlCapability {
    CanInsertChild,
    CanInsertMultiChildren,
    TextEdit,
    Focus,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SlotInsertErr {
    SlotNotExist,
    SlotArgMismatch,
}

pub trait Control: 'static {
    fn query_capability(&self, cap: ControlCapability) -> bool;

    fn window_id(&self) -> Option<IdType>;

    fn parent(&self) -> WeakHandle<dyn Control>;
    fn id(&self) -> IdType;

    /// true on success and false on failure
    fn set_parent(&mut self, parent: WeakHandle<dyn Control>) -> bool;

    fn paint(&mut self, painter: &mut Painter);

    fn set_pos(&mut self, x: i32, y: i32);
    fn set_size(&mut self, width: u32, height: u32);
    fn set_preferred_size(&mut self, _preferred_size: (u32, u32)) {}
    fn set_max_size(&mut self, _max_size: (u32, u32)) {}
    fn set_min_size(&mut self, _min_size: (u32, u32)) {}
    fn set_geometry(&mut self, r: Rect) {
        self.set_pos(r.x, r.y);
        self.set_size(r.w, r.h);
    }

    fn pos(&self) -> (i32, i32);
    fn size(&self) -> (u32, u32);
    fn preferred_size(&self) -> (u32, u32);

    fn subscribe(
        &mut self,
        _event: String,
        _provider: WeakHandle<dyn Control>,
        _proc: EventProc,
    ) -> bool {
        panic!("Not implemented");
    }

    fn add_slot(
        &mut self,
        _signal_name: String,
        _slot: Box<dyn Slot>,
    ) -> Result<(), SlotInsertErr> {
        Err(SlotInsertErr::SlotNotExist)
    }

    // fn on_event(&mut self, event: String, arg: EventArg);

    fn add_child(&mut self, child: WeakHandle<dyn Control>) -> anyhow::Result<IdType>;
    fn remove_child(&mut self, child: WeakHandle<dyn Control>);
    fn remove_child_by_id(&mut self, child_id: IdType);

    fn get_children(&mut self); // add return type
    fn destroy_children(&mut self);

    fn on_init(&mut self, event: &WinInitEvent) {}

    fn receive_sys_event(&mut self, _event: SysEvent) {}

    fn receives_event(&self, _event_type: TypeId) -> bool {
        false
    }

    fn process_event(&mut self, event: Box<dyn Event>) -> bool;

    fn insert_tree(&self, focus_mgr: &mut FocusMgr);

    fn attach_window(&mut self, _win: Weak<RefCell<WindowDirector>>) {}

    fn x(&self) -> i32 {
        self.pos().0
    }

    fn y(&self) -> i32 {
        self.pos().1
    }

    fn width(&self) -> u32 {
        self.size().0
    }

    fn height(&self) -> u32 {
        self.size().1
    }

    fn preferred_width(&self) -> u32 {
        self.preferred_size().0
    }

    fn preferred_height(&self) -> u32 {
        self.preferred_size().1
    }

    fn max_size(&self) -> (u32, u32) {
        (u32::MAX, u32::MAX)
    }

    fn max_width(&self) -> u32 {
        self.max_size().0
    }

    fn max_height(&self) -> u32 {
        self.max_size().1
    }

    fn min_size(&self) -> (u32, u32) {
        (0, 0)
    }

    fn min_width(&self) -> u32 {
        self.min_size().0
    }

    fn min_height(&self) -> u32 {
        self.min_size().1
    }

    fn geometry(&self) -> Rect {
        let (x, y) = self.pos();
        let (w, h) = self.size();
        Rect { x, y, w, h }
    }
}

impl dyn Control {
    pub fn connect<Arg: 'static, F: FnMut(Arg) + 'static>(
        &mut self,
        signal_name: &str,
        slot: F,
    ) -> Result<(), SlotInsertErr> {
        self.add_slot(signal_name.to_string(), Box::new(SlotHandle::new(slot)))
    }
}

pub trait Eventful: Control {
    fn slot_connect<Arg: 'static>(&mut self, name: String, slot: Box<dyn FnMut(Arg) + 'static>);
    fn raise_signal<Arg: 'static>(&mut self, name: String, arg: Arg);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AddChildErr;

impl Hash for dyn Control {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}
