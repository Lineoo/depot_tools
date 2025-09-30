use std::{cell::RefCell, ptr::NonNull};

use crate::ui_control::{control::Control, handle::handles::HandleRcInner};

pub struct HandleUntyped {
    data: NonNull<RefCell<dyn Control>>,
    inner: NonNull<HandleRcInner>,
}

impl Clone for HandleUntyped {
    fn clone(&self) -> Self {
        unsafe {
            let strong = &self.inner.as_ref().strong;
            strong.set(strong.get() + 1);
        }
        Self {
            data: self.data,
            inner: self.inner,
        }
    }
}
