use crate::control::Control;

#[cfg(feature = "sync_ref")]
type ControlRef<T: Control> =  std::sync::Weak<T>;

#[cfg(not(feature = "sync_ref"))]
type ControlRef<T: Control> =  std::rc::Weak<T>;
