use std::{
  any::Any,
  cell::RefCell,
  ops::{Deref, DerefMut},
  rc::Rc,
};

use rhg_core::{AsGenericContext, Context};

pub struct GLContext(glow::Context);

impl GLContext {
  pub fn new(ctx: glow::Context) -> Self {
    Self(ctx)
  }
}

impl Context for GLContext {
  fn internal(&self) -> &dyn Any {
    &self.0
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
    &self.0
  }
}

impl DerefMut for GLContext {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl AsGenericContext for GLContext {
  fn as_generic_context(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn Context>> {
    it
  }
}

#[cfg(test)]
mod tests {}
