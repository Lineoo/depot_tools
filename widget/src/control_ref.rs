use crate::control::Control;

#[cfg(feature = "sync_ref")]
type ControlRef<'w, T: Control<'w>> = std::sync::Weak<T>;

#[cfg(not(feature = "sync_ref"))]
type ControlRef<'w, T: Control<'w>> = std::rc::Weak<T>;
