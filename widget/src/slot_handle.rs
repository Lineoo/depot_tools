use std::any::{Any, TypeId};

pub trait Slot {
    fn get_arg_type_id(&self) -> TypeId;
    fn call(&mut self, arg: Box<dyn Any>) -> Result<(), SlotInvokeArgMismatch>;
}

impl dyn Slot {
    pub fn arg_type_is<ArgType: 'static>(&self) -> bool {
        self.get_arg_type_id() == TypeId::of::<ArgType>()
    }
}

pub struct SlotHandle<Arg> {
    func: Box<dyn FnMut(Arg) + 'static>,
}

impl<Arg> SlotHandle<Arg> {
    pub fn new<F: FnMut(Arg) + 'static>(func: F) -> Self {
        Self {
            func: Box::new(func),
        }
    }
}

impl<Arg: 'static> Slot for SlotHandle<Arg> {
    fn get_arg_type_id(&self) -> TypeId {
        TypeId::of::<Arg>()
    }

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
