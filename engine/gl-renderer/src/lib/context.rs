use std::{
  any::Any,
  cell::RefCell,
  ops::{Deref, DerefMut},
  rc::Rc,
};

use rhg_core::Context;

pub struct GLContext(Option<glow::Context>);

impl GLContext {
  pub fn new(ctx: glow::Context) -> Self {
    Self(Some(ctx))
  }

  pub fn empty() -> Self {
    Self(None)
  }
}

impl Context for GLContext {
  fn internal(&self) -> &dyn Any {
    self.0.as_ref().unwrap()
  }

  fn create(&mut self) -> rhg_core::Result<()> {
    Ok(())
  }

  fn destroy(&mut self) -> rhg_core::Result<()> {
    Ok(())
  }
}

impl Deref for GLContext {
  type Target = glow::Context;

  fn deref(&self) -> &Self::Target {
    self.0.as_ref().unwrap()
  }
}

impl DerefMut for GLContext {
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.0.as_mut().unwrap()
  }
}

#[cfg(test)]
mod tests {
  use std::{
    cell::{Ref, RefCell},
    ops::Deref,
    rc::Rc,
  };

  use rhg_core::{borrow_downcast, BorrowUpcast, Context};

  use super::GLContext;

}
