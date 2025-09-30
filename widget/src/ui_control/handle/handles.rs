use std::{
    cell::{Cell, Ref, RefCell},
    marker::PhantomData,
    ops::Deref,
    ptr::{NonNull, drop_in_place},
};

use crate::ui_control::control::Control;

pub(crate) struct HandleRcInner {
    pub(crate) strong: Cell<usize>,
    pub(crate) all: Cell<usize>,
}

pub struct Handle<T: Control + 'static> {
    data: NonNull<RefCell<dyn Control>>,
    inner: NonNull<HandleRcInner>,
    phantom: PhantomData<T>,
}

impl<T: Control + 'static> Handle<T> {
    pub(crate) fn new(data: T) -> Self {
        let inner = Box::new(HandleRcInner {
            strong: Cell::new(1),
            all: Cell::new(1),
        });
        Self {
            data: NonNull::new(Box::into_raw(Box::new(RefCell::new(data)))).unwrap(),
            inner: NonNull::new(Box::into_raw(inner)).unwrap(),
            phantom: PhantomData,
        }
    }

    pub(crate) fn downgrade(&self) -> WeakHandleRc<T> {
        unsafe {
            let all = &self.inner.as_ref().all;
            all.set(all.get() + 1);
        }
        WeakHandleRc {
            data: self.data.as_ptr(),
            inner: self.inner,
            phantom: PhantomData,
        }
    }
}

impl<T: Control> Clone for Handle<T> {
    fn clone(&self) -> Self {
        unsafe {
            let strong = &self.inner.as_ref().strong;
            strong.set(strong.get() + 1);
        }
        Self {
            data: self.data,
            inner: self.inner,
            phantom: PhantomData,
        }
    }
}

impl<T: Control> Deref for Handle<T> {
    type Target = RefCell<T>;

    fn deref(&self) -> &Self::Target {
        unsafe { (self.data.as_ptr() as *const RefCell<T>).as_ref() }.unwrap()
    }
}

impl<T: Control> Drop for Handle<T> {
    fn drop(&mut self) {
        if unsafe { self.inner.as_ref() }.strong.get() == 1 {
            unsafe {
                drop_in_place(self.data.as_ptr());
            }
        } else {
            unsafe {
                let strong = &self.inner.as_ref().strong;
                strong.set(strong.get() - 1);
            }
        }

        if unsafe { self.inner.as_ref() }.all.get() == 1 {
            unsafe {
                drop_in_place(self.inner.as_ptr());
            }
        } else {
            unsafe {
                let all = &self.inner.as_ref().all;
                all.set(all.get() - 1);
            }
        }
    }
}

pub struct WeakHandleRc<T: Control + 'static> {
    data: *mut RefCell<dyn Control>,
    inner: NonNull<HandleRcInner>,
    phantom: PhantomData<T>,
}

impl<T: Control + 'static> WeakHandleRc<T> {
    pub fn upgrade(&self) -> Option<Handle<T>> {
        if unsafe { self.inner.as_ref() }.strong.get() == 0 {
            return None;
        }
        unsafe {
            let strong = &self.inner.as_ref().strong;
            strong.set(strong.get() + 1);
        }
        Some(Handle {
            data: NonNull::new(self.data).unwrap(),
            inner: self.inner,
            phantom: PhantomData,
        })
    }
}

impl<T: Control + 'static> Clone for WeakHandleRc<T> {
    fn clone(&self) -> Self {
        unsafe {
            let all = &self.inner.as_ref().all;
            all.set(all.get() + 1);
        }
        Self {
            data: self.data,
            inner: self.inner,
            phantom: PhantomData,
        }
    }
}

impl<T: Control + 'static> Drop for WeakHandleRc<T> {
    fn drop(&mut self) {
        if unsafe { self.inner.as_ref() }.all.get() == 1 {
            unsafe {
                drop_in_place(self.inner.as_ptr());
            }
        } else {
            unsafe {
                let all = &self.inner.as_ref().all;
                all.set(all.get() - 1);
            }
        }
    }
}
