use std::any::Any;

use as_any::AsAny;

pub trait Context: AsAny {
  fn internal(&self) -> &dyn Any;

  fn create(&mut self) -> crate::Result<()>;
  fn destroy(&mut self) -> crate::Result<()>;
}
