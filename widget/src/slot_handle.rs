use std::any::Any;

pub trait SlotHandle {
    fn call(&mut self, arg: Box<dyn Any>) -> Result<(), SlotInvokeArgMismatch>;
}

pub struct SlotHandleImpl<Arg> {
    func: Box<dyn FnMut(Arg) + 'static>,
}

impl<Arg> SlotHandleImpl<Arg> {
    pub fn new<F: FnMut(Arg) + 'static>(func: F) -> Self {
        Self {
            func: Box::new(func),
        }
    }
}

impl<Arg: 'static> SlotHandle for SlotHandleImpl<Arg> {
    fn call(&mut self, arg: Box<dyn Any>) -> Result<(), SlotInvokeArgMismatch> {
        if let Ok(arg) = arg.downcast::<Arg>() {
            (self.func)(*arg);
            Ok(())
        } else {
            Err(SlotInvokeArgMismatch)
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SlotInvokeArgMismatch;
