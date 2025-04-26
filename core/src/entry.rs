/// An *active* entry with full functions.
/// ## Description Behavior ##
/// We contact with parent first, so the children's descriptions are given by parent.
pub trait ActiveEntry {
    fn push(&mut self, args: EntryArgs);
    fn read(&self, index: usize) -> Option<Description>;
    fn call(&self, index: usize) -> Option<Invoke>;
}

#[derive(Clone, Debug)]
pub struct Description {
    pub title: String,
    pub information: String,
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

/// # Behavior
/// ```text
///   0. <user input>       - User input (Exit)
///   1. <user input>       - User input (Remain)
///   2. <user input> * 2   - Repeat it twice
///   3. ..                 - Back to parent entry
///   4. Raise              - Raise entry   - Entry: TestEntry("Test")
/// ```
#[cfg(test)]
pub(crate) struct TestEntry(pub String);

#[cfg(test)]
impl ActiveEntry for TestEntry {
    fn push(&mut self, args: EntryArgs) {
        self.0.clone_from(&args);
    }

    fn read(&self, index: usize) -> Option<Description> {
        match index {
            0 => Some(Description {
                title: self.0.clone(),
                information: "User input (Exit)".into(),
            }),
            1 => Some(Description {
                title: self.0.clone(),
                information: "User input (Remain)".into(),
            }),
            2 => Some(Description {
                title: format!("{} * 2", self.0),
                information: "Repeat it twice".into(),
            }),
            3 => Some(Description {
                title: "..".into(),
                information: "Back to parent entry".into(),
            }),
            4 => Some(Description {
                title: "Raise".into(),
                information: "Raise entry".into(),
            }),
            _ => None,
        }
    }

    fn call(&self, index: usize) -> Option<Invoke> {
        match index {
            0 => Some(Invoke::Exit),
            1 => Some(Invoke::Remain),
            2 => Some(Invoke::Update(self.0.clone().repeat(2))),
            3 => Some(Invoke::Leave),
            4 => Some(Invoke::Raise(Box::new(TestEntry("Test".into())))),
            _ => None,
        }
    }
}

/// ## Expected Functions ##
/// - `EntrySpace`: use input to search, cached
/// - `Enum`: choose variants
/// - `Calculator`: parse input to result
/// - `Files`: result an *infinite* number of entries
/// - `FFmpeg Util`: need to choose multiple files/parameters
/// - `Color Picker`: completely control the UI pass
mod plan {}
