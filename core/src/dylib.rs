use libloading::Library;
use std::ffi::{CString, c_char};

use crate::entry::{ActiveEntry, EntryArgs, Invoke, Read};

pub struct DylibEntry {
    pub lib: Library,
}
impl ActiveEntry for DylibEntry {
    fn push(&mut self, args: EntryArgs) {
        unsafe {
            let func = self
                .lib
                .get::<unsafe extern "C" fn(*mut c_char)>(b"push")
                .unwrap();
            let args = CString::new(args).unwrap();
            func(args.into_raw());
        }
    }

    fn read(&self, index: usize) -> Option<Read> {
        unsafe {
            let read = self
                .lib
                .get::<unsafe extern "C" fn(usize) -> CRead>(b"read")
                .unwrap();
            let CRead { title, description } = read(index);
            if title.is_null() || description.is_null() {
                return None;
            }
            let title = CString::from_raw(title).into_string().unwrap();
            let description = CString::from_raw(description).into_string().unwrap();
            Some(Read { title, description })
        }
    }

    fn call(&self, index: usize) -> Option<Invoke> {
        unsafe {
            let func = self
                .lib
                .get::<unsafe extern "C" fn(usize)>(b"call")
                .unwrap();
            todo!()
        }
    }
}

#[repr(C)]
pub struct CRead {
    pub title: *mut c_char,
    pub description: *mut c_char,
}

#[repr(C)]
pub enum CInvoke {
    Raise,
    Update,
    Leave,
    Exit,
    Remain,
}
