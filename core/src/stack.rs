use crate::entry::*;

pub struct EntryStack(Vec<BoxedEntry>);
impl EntryStack {
    fn new(entry: BoxedEntry) -> Self {
        EntryStack(vec![entry])
    }

    fn push(&mut self, args: EntryArgs) {
        let Some(last) = self.0.last_mut() else {
            return;
        };
        last.push(args);
    }

    fn read(&self, index: usize) -> Option<Description> {
        let Some(last) = self.0.last() else {
            if index == 0 {
                return Some(Description {
                    title: "EntryStack is empty.".into(),
                    information: "No entry here yet".into(),
                });
            } else {
                return None;
            }
        };

        last.read(index)
    }

    fn call(&mut self, index: usize) -> StackCall {
        let Some(last) = self.0.last() else {
            if index == 0 {
                return StackCall::Exit;
            } else {
                return StackCall::None;
            }
        };

        match last.call(index) {
            Some(Invoke::Exit) => return StackCall::Exit,
            Some(Invoke::Leave) => {
                self.0.pop();
                if self.0.is_empty() {
                    return StackCall::Exit;
                }
            }
            Some(Invoke::Remain) => (),
            Some(Invoke::Update(args)) => {
                self.push(args);
            }
            Some(Invoke::Raise(entry)) => {
                self.0.push(entry);
            }
            None => ()
        }
        StackCall::None
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum StackCall {
    None,
    Exit,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stack() {
        let mut stack = EntryStack::new(Box::new(TestEntry("EntryStackTest".into())));
        assert_eq!(stack.read(0).unwrap().title, "EntryStackTest");
        assert_eq!(stack.read(2).unwrap().title, "EntryStackTest * 2");

        stack.push("foo2bar".into());

        assert_eq!(stack.read(0).unwrap().title, "foo2bar");
        assert_eq!(stack.read(2).unwrap().title, "foo2bar * 2");

        assert_eq!(stack.call(4), StackCall::None);

        assert_eq!(stack.read(0).unwrap().title, "Test");
        assert_eq!(stack.read(1).unwrap().title, "Test");
        assert_eq!(stack.read(2).unwrap().title, "Test * 2");
        assert_eq!(stack.read(3).unwrap().title, "..");
        assert_eq!(stack.read(4).unwrap().title, "Raise");

        stack.push("baz3oxy4".into());

        assert_eq!(stack.read(0).unwrap().title, "baz3oxy4");
        assert_eq!(stack.read(1).unwrap().title, "baz3oxy4");
        assert_eq!(stack.read(2).unwrap().title, "baz3oxy4 * 2");

        assert_eq!(stack.call(2), StackCall::None);
        assert_eq!(stack.read(0).unwrap().title, "baz3oxy4baz3oxy4");

        assert_eq!(stack.call(3), StackCall::None);
        assert_eq!(stack.read(0).unwrap().title, "foo2bar");

        assert_eq!(stack.call(3), StackCall::Exit);
    }
}
