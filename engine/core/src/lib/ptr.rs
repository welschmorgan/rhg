use std::{
  cell::RefCell,
  fmt::Debug,
  fmt::Display,
  ops::{Deref, DerefMut},
  rc::Rc,
};

pub struct Ptr<T: ?Sized>(Rc<RefCell<T>>);

impl<T> Ptr<T> {
  pub fn new(val: T) -> Self {
    Self(Rc::new(RefCell::new(val)))
  }
}

impl<T> Clone for Ptr<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<T: Default> Default for Ptr<T> {
  fn default() -> Self {
    Self::new(T::default())
  }
}

impl<T: Debug> Debug for Ptr<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("Ptr").field(&self.0).finish()
  }
}

impl<T: Display> Display for Ptr<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.0.borrow().fmt(f)
  }
}

impl<T> Deref for Ptr<T> {
  type Target = Rc<RefCell<T>>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T> DerefMut for Ptr<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
