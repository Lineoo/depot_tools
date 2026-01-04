/// An *active* entry with full functions.
pub trait ActiveEntry {
    fn push(&mut self, args: EntryArgs);
    fn read(&self, index: usize) -> Option<Read>;
    fn call(&self, index: usize) -> Option<Invoke>;
}

#[derive(Clone, Debug)]
pub struct Read {
    pub title: String,
    pub description: String,
}

pub type EntryArgs = String;

pub type BoxedEntry = Box<dyn ActiveEntry>;

pub enum Invoke {
    Raise(BoxedEntry),
    Update(EntryArgs),
    Leave,
    Exit,
    Remain,
}