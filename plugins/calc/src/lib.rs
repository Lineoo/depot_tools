use std::{
    ffi::{CString, c_char},
    ptr::{null, null_mut},
};

use calculator_rs::Calculate;
use depot_core::{
    dylib::CRead,
    entry::{ActiveEntry, EntryArgs, Invoke, Read},
};

struct Calculator(String);

impl ActiveEntry for Calculator {
    fn push(&mut self, args: EntryArgs) {
        self.0.clone_from(&args);
    }
    fn read(&self, index: usize) -> Option<Read> {
        if index == 0 {
            match self.0.calculate() {
                Ok(x) => Some(Read {
                    title: x.to_string(),
                    description: "Press Enter to lift the result to the input bar.".into(),
                }),
                Err(e) => Some(Read {
                    title: e.to_string(),
                    description: "Press Enter to clear the input bar.".into(),
                }),
            }
        } else {
            None
        }
    }
    fn call(&self, index: usize) -> Option<Invoke> {
        if index == 0 {
            match self.0.calculate() {
                Ok(x) => Some(Invoke::Update(x.to_string())),
                Err(_) => Some(Invoke::Update(String::new())),
            }
        } else {
            None
        }
    }
}

static mut EXT: Option<Calculator> = None;

#[unsafe(no_mangle)]
extern "C" fn init() {
    unsafe { EXT = Some(Calculator(String::new())) };
}

#[unsafe(no_mangle)]
extern "C" fn push(args: *mut c_char) {
    #[expect(static_mut_refs)]
    if let Some(ext) = unsafe { EXT.as_mut() } {
        ext.push(unsafe { CString::from_raw(args).into_string().unwrap() });
    }
}

#[unsafe(no_mangle)]
extern "C" fn read(index: usize) -> CRead {
    #[expect(static_mut_refs)]
    if let Some(ext) = unsafe { EXT.as_mut() } {
        if let Some(read) = ext.read(index) {
            return CRead {
                title: CString::new(read.title).unwrap().into_raw(),
                description: CString::new(read.description).unwrap().into_raw(),
            };
        }
    }
    CRead {
        title: null_mut(),
        description: null_mut(),
    }
}

#[unsafe(no_mangle)]
extern "C" fn call(index: usize) {
    #[expect(static_mut_refs)]
    if let Some(ext) = unsafe { EXT.as_mut() } {
        ext.call(index);
    }
}
