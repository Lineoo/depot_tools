/// An *active* entry with full functions.
/// ## Description Behavior ##
/// We contact with parent first, so the children's descriptions are given by parent.
pub trait ActiveEntry {
    /// *Update* the current active entry with given args
    fn push(&mut self, args: EntryArgs);
    /// Return a information of one sub-entry
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

/// ## Expected Functions ##
/// - `EntrySpace`: use input to search, cached
/// - `Enum`: choose variants
/// - `Calculator`: parse input to result
/// - `Files`: result an *infinite* number of entries
/// - `FFmpeg Util`: need to choose multiple files/parameters
/// - `Color Picker`: completely control the UI pass
mod plan {}
