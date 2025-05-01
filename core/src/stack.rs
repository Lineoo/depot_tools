use crate::entry::*;

struct StackElement {
    args: EntryArgs,
    active: BoxedEntry,
}

#[derive(PartialEq, Eq, Debug)]
pub enum StackCall {
    With(String),
    Exit,
}

pub struct Stack(Vec<StackElement>);
impl Stack {
    pub fn new(entry: BoxedEntry) -> Self {
        Stack(vec![StackElement {
            args: "".into(),
            active: entry,
        }])
    }

    pub fn write(&mut self, args: EntryArgs) {
        let Some(last) = self.0.last_mut() else {
            return;
        };

        last.args.clone_from(&args);
        last.active.push(args);
    }

    pub fn read(&self, index: usize) -> Option<Description> {
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

        last.active.read(index)
    }

    pub fn call(&mut self, index: usize) -> StackCall {
        let Some(last) = self.0.last() else {
            return StackCall::Exit;
        };

        match last.active.call(index) {
            Some(Invoke::Exit) => StackCall::Exit,
            Some(Invoke::Leave) => {
                self.0.pop();
                if let Some(back) = self.0.last() {
                    StackCall::With(back.args.clone())
                } else {
                    StackCall::Exit
                }
            }
            Some(Invoke::Remain) => StackCall::With(last.args.clone()),
            Some(Invoke::Update(args)) => {
                self.write(args.clone());
                StackCall::With(args)
            }
            Some(Invoke::Raise(entry)) => {
                self.0.push(StackElement {
                    args: "".into(),
                    active: entry,
                });
                StackCall::With("".into())
            }
            None => StackCall::With(last.args.clone()),
        }
    }
    pub fn iter(&self) -> IntoIter {
        IntoIter(0, self)
    }
}

pub struct IntoIter<'stack>(usize, &'stack Stack);
impl Iterator for IntoIter<'_> {
    type Item = Description;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.1.read(self.0);
        self.0 += 1;
        result
    }
}

impl<'stack> IntoIterator for &'stack Stack {
    type Item = Description;

    type IntoIter = IntoIter<'stack>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(0, self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stack() {
        let mut stack = Stack::new(Box::new(TestEntry("EntryStackTest".into())));
        assert_eq!(stack.read(0).unwrap().title, "EntryStackTest");
        assert_eq!(stack.read(2).unwrap().title, "EntryStackTest * 2");

        stack.write("foo2bar".into());

        assert_eq!(stack.read(0).unwrap().title, "foo2bar");
        assert_eq!(stack.read(2).unwrap().title, "foo2bar * 2");

        assert_eq!(stack.call(4), StackCall::With("".into()));

        assert_eq!(stack.read(0).unwrap().title, "Test");
        assert_eq!(stack.read(1).unwrap().title, "Test");
        assert_eq!(stack.read(2).unwrap().title, "Test * 2");
        assert_eq!(stack.read(3).unwrap().title, "..");
        assert_eq!(stack.read(4).unwrap().title, "Raise");

        stack.write("baz3oxy4".into());

        assert_eq!(stack.read(0).unwrap().title, "baz3oxy4");
        assert_eq!(stack.read(1).unwrap().title, "baz3oxy4");
        assert_eq!(stack.read(2).unwrap().title, "baz3oxy4 * 2");

        assert_eq!(stack.call(2), StackCall::With("baz3oxy4baz3oxy4".into()));
        assert_eq!(stack.read(0).unwrap().title, "baz3oxy4baz3oxy4");

        assert_eq!(stack.call(3), StackCall::With("foo2bar".into()));
        assert_eq!(stack.read(0).unwrap().title, "foo2bar");

        assert_eq!(stack.call(3), StackCall::Exit);
    }
}
