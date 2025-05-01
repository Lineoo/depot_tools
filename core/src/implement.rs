use crate::entry::{ActiveEntry, EntryArgs, Invoke, Read};

impl ActiveEntry for &str {
    fn push(&mut self, _: EntryArgs) {}

    fn read(&self, index: usize) -> Option<Read> {
        if index == 0 {
            Some(Read {
                title: "Message".into(),
                description: self.to_string(),
            })
        } else {
            None
        }
    }

    fn call(&self, index: usize) -> Option<Invoke> {
        if index == 0 {
            Some(Invoke::Remain)
        } else {
            None
        }
    }
}
