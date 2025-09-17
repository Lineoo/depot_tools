pub struct Handle<T: ?Sized> {
    data: *mut T,
}
