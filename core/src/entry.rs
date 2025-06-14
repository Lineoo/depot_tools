/// An *active* entry with full functions.
/// ## Description Behavior ##
/// We contact with parent first, so the children's descriptions are given by parent.
#[expect(unused_variables)]
pub trait ActiveEntry {
    fn push(&mut self, args: EntryArgs);
    fn read(&self, index: usize) -> Option<Read>;
    fn call(&self, index: usize) -> Option<Invoke>;
    fn raise(&self, index: usize) -> Option<BoxedEntry> {
        None
    }
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
    // TODO: Invoke::Home
}

// TODO: `PureFunctionEntry` that implement a buffer for use and plugin maker only
//      need to write a function. That'll be easy.

// TODO: ULTIMATE-ULTRA-PLUS-STRING-CLONING-COSTS-
//      That's cool. Ownership? Clone. 
