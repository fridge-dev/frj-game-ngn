/// A simplified interface for an Option where you expect to
/// alternate between `Some` and `None` by taking the inner
/// value out.
pub struct Holder<T>(Option<T>);

impl<T> Holder<T> {
    pub fn new(item: T) -> Self {
        Holder(Some(item))
    }

    pub fn take(&mut self) -> T {
        self.0.take().expect("Invalid state: Holder.take() called when it was empty")
    }

    pub fn put(&mut self, item: T) {
        if self.0.is_some() {
            panic!("Invalid state: Holder.put() called when it was full");
        }
        self.0.replace(item);
    }
}
