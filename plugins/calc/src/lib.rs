use calculator_rs::Calculate;
use depot_core::entry::{ActiveEntry, EntryArgs, Invoke, Read};

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

#[unsafe(no_mangle)]
fn entry() -> Box<dyn ActiveEntry> {
    Box::new(Calculator(String::new()))
}
