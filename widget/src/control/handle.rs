use std::{
    any::TypeId,
    hash::Hash,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::{drop_in_place, null_mut},
};

use crate::control::{Control, PhantomControl};

struct HandleInner<T: Control + ?Sized = dyn Control> {
    strong: usize,
    weak: usize,
    borrow: isize,
    data: *mut dyn Control,
    real: TypeId,
    _marker: PhantomData<T>,
}

#[derive(Eq)]
pub struct Handle<T: Control + ?Sized = dyn Control> {
    inner: *mut HandleInner<T>,
}

pub struct WeakHandle<T: Control + ?Sized = dyn Control> {
    inner: *mut HandleInner<T>,
}

pub struct Ref<'handle, T: Control + ?Sized> {
    owner: &'handle Handle<T>,
}

pub struct RefMut<'handle, T: Control + ?Sized> {
    owner: &'handle Handle<T>,
}

pub struct WeakRef<'handle, T: Control + ?Sized> {
    owner: &'handle WeakHandle<T>,
}

pub struct WeakRefMut<'handle, T: Control + ?Sized> {
    owner: &'handle WeakHandle<T>,
}

impl<T: Control + ?Sized> Clone for Handle<T> {
    fn clone(&self) -> Self {
        unsafe { (*self.inner).strong += 1 };
        Self { inner: self.inner }
    }
}

impl<T: Control + ?Sized> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { (*self.inner).data == (*other.inner).data }
    }
}

impl<T: Control + ?Sized> Hash for Handle<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        unsafe { (*(*self.inner).data).id() }.hash(state);
    }
}

impl<T: Control + ?Sized> Clone for WeakHandle<T> {
    fn clone(&self) -> Self {
        unsafe { (*self.inner).weak += 1 };
        Self { inner: self.inner }
    }
}
impl<T: Control + ?Sized> PartialEq for WeakHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        if unsafe { (*self.inner).strong } != 0 && unsafe { (*other.inner).strong } != 0 {
            unsafe { (*self.inner).data == (*other.inner).data }
        } else {
            return unsafe { (*self.inner).strong } == 0 && unsafe { (*other.inner).strong } == 0;
        }
    }
}

impl<T: Control + ?Sized> Eq for WeakHandle<T> {}

impl<T: Control + ?Sized> Hash for WeakHandle<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if unsafe { (*self.inner).strong } != 0 {
            unsafe { (*(*self.inner).data).id() }.hash(state)
        } else {
            state.write_usize(0);
        }
    }
}

impl<T: Control + ?Sized> Handle<T> {
    pub fn downcast<U: Control + 'static>(&self) -> Option<Handle<U>> {
        if unsafe { (*self.inner).real != TypeId::of::<U>() } {
            return None;
        }
        unsafe { (*self.inner).strong += 1 };
        Some(Handle {
            inner: self.inner as *mut HandleInner<U>,
        })
    }
}

impl<T: Control + ?Sized> WeakHandle<T> {
    pub fn downcast<U: Control + 'static>(&self) -> Option<WeakHandle<U>> {
        if unsafe { (*self.inner).real != TypeId::of::<U>() } {
            return None;
        }
        unsafe { (*self.inner).weak += 1 };
        Some(WeakHandle {
            inner: self.inner as *mut HandleInner<U>,
        })
    }
}

impl<T: Control + ?Sized + 'static> WeakHandle<T> {
    pub fn new() -> Self {
        WeakHandle {
            inner: Box::into_raw(Box::new(HandleInner {
                strong: 0,
                weak: 1,
                borrow: 0,
                data: null_mut::<PhantomControl>() as *mut dyn Control,
                real: TypeId::of::<T>(),
                _marker: PhantomData::<T>,
            })),
        }
    }
}

impl<T: Control + ?Sized + 'static> Default for WeakHandle<T> {
    fn default() -> Self {
        Self::new()
    }
}

// functions below only apply to concrete type

impl<T: Control + 'static> Handle<T> {
    pub fn new(data: T) -> Self {
        Handle {
            inner: Box::into_raw(Box::new(HandleInner {
                strong: 1,
                weak: 0,
                borrow: 0,
                data: Box::into_raw(Box::new(data)),
                real: TypeId::of::<T>(),
                _marker: PhantomData::<T>,
            })),
        }
    }

    pub fn clone_untyped(&self) -> Handle {
        unsafe { (*self.inner).strong += 1 };
        Handle {
            inner: self.inner as *mut HandleInner,
        }
    }

    pub fn into_untyped(self) -> Handle {
        self.clone_untyped()
    }

    pub fn downgrade(&self) -> WeakHandle<T> {
        unsafe { (*self.inner).weak += 1 };
        WeakHandle { inner: self.inner }
    }
}

impl<T: Control + ?Sized> Handle<T> {
    pub fn borrow(&self) -> Ref<'_, T> {
        unsafe { assert!((*self.inner).borrow >= 0, "handle is mutably borrowed") };
        unsafe { (*self.inner).borrow += 1 };
        Ref { owner: self }
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        unsafe { assert!((*self.inner).borrow == 0, "handle is borrowed") };
        unsafe { (*self.inner).borrow -= 1 };
        RefMut { owner: self }
    }

    pub fn try_borrow(&self) -> Option<Ref<'_, T>> {
        if unsafe { (*self.inner).borrow < 0 } {
            return None;
        }
        unsafe { (*self.inner).borrow += 1 };
        Some(Ref { owner: self })
    }

    pub fn try_borrow_mut(&self) -> Option<RefMut<'_, T>> {
        if unsafe { (*self.inner).borrow != 0 } {
            return None;
        }
        unsafe { (*self.inner).borrow -= 1 };
        Some(RefMut { owner: self })
    }
}

impl<T: Control + 'static> WeakHandle<T> {
    pub fn clone_untyped(&self) -> WeakHandle {
        unsafe { (*self.inner).weak += 1 };
        WeakHandle {
            inner: self.inner as *mut HandleInner,
        }
    }

    pub fn into_untyped(self) -> WeakHandle {
        self.clone_untyped()
    }
}

impl<T: Control + ?Sized> WeakHandle<T> {
    pub fn borrow(&self) -> Option<WeakRef<'_, T>> {
        if unsafe { (*self.inner).strong == 0 } {
            return None;
        }
        unsafe { assert!((*self.inner).borrow >= 0, "handle is mutably borrowed") };
        unsafe { (*self.inner).borrow += 1 };
        Some(WeakRef { owner: self })
    }

    pub fn borrow_mut(&self) -> Option<WeakRefMut<'_, T>> {
        if unsafe { (*self.inner).strong == 0 } {
            return None;
        }
        unsafe { assert!((*self.inner).borrow == 0, "handle is borrowed") };
        unsafe { (*self.inner).borrow -= 1 };
        Some(WeakRefMut { owner: self })
    }

    pub fn upgrade(&self) -> Option<Handle<T>> {
        if unsafe { (*self.inner).strong == 0 } {
            return None;
        }
        unsafe { (*self.inner).strong += 1 };
        Some(Handle { inner: self.inner })
    }
}

impl<T: Control> Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { ((*self.owner.inner).data as *mut T).as_ref().unwrap() }
    }
}

impl Deref for Ref<'_, dyn Control> {
    type Target = dyn Control;
    fn deref(&self) -> &Self::Target {
        unsafe { (*self.owner.inner).data.as_ref().unwrap() }
    }
}

impl<T: Control> Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { ((*self.owner.inner).data as *mut T).as_ref().unwrap() }
    }
}
impl<T: Control> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { ((*self.owner.inner).data as *mut T).as_mut().unwrap() }
    }
}

impl Deref for RefMut<'_, dyn Control> {
    type Target = dyn Control;
    fn deref(&self) -> &Self::Target {
        unsafe { (*self.owner.inner).data.as_ref().unwrap() }
    }
}
impl DerefMut for RefMut<'_, dyn Control> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { (*self.owner.inner).data.as_mut().unwrap() }
    }
}

// consider counter situations like one strong and one weak, and the strong handle
// is dropped before the weak guard is used.

impl<T: Control> WeakRef<'_, T> {
    pub fn try_deref(&self) -> Option<&T> {
        if unsafe { (*self.owner.inner).strong == 0 } {
            return None;
        }
        unsafe { Some(((*self.owner.inner).data as *mut T).as_ref().unwrap()) }
    }
}

impl<T: Control> WeakRefMut<'_, T> {
    pub fn try_deref(&self) -> Option<&T> {
        if unsafe { (*self.owner.inner).strong == 0 } {
            return None;
        }
        unsafe { Some(((*self.owner.inner).data as *mut T).as_ref().unwrap()) }
    }

    pub fn try_deref_mut(&mut self) -> Option<&mut T> {
        if unsafe { (*self.owner.inner).strong == 0 } {
            return None;
        }
        unsafe { Some(((*self.owner.inner).data as *mut T).as_mut().unwrap()) }
    }
}

impl<T: Control + ?Sized> Drop for Handle<T> {
    fn drop(&mut self) {
        unsafe { (*self.inner).strong -= 1 };
        if unsafe { (*self.inner).strong == 0 } {
            unsafe { drop_in_place((*self.inner).data) };
            if unsafe { (*self.inner).weak == 0 } {
                unsafe { drop_in_place(self.inner) };
            }
        }
    }
}

impl<T: Control + ?Sized> Drop for WeakHandle<T> {
    fn drop(&mut self) {
        unsafe { (*self.inner).weak -= 1 };
        if unsafe { (*self.inner).strong == 0 && (*self.inner).weak == 0 } {
            unsafe { drop_in_place(self.inner) };
        }
    }
}

impl<T: Control + ?Sized> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        unsafe { (*self.owner.inner).borrow -= 1 };
    }
}

impl<T: Control + ?Sized> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        unsafe { (*self.owner.inner).borrow += 1 };
    }
}

impl<T: Control + ?Sized> Drop for WeakRef<'_, T> {
    fn drop(&mut self) {
        unsafe { (*self.owner.inner).borrow -= 1 };
    }
}

impl<T: Control + ?Sized> Drop for WeakRefMut<'_, T> {
    fn drop(&mut self) {
        unsafe { (*self.owner.inner).borrow += 1 };
    }
}
