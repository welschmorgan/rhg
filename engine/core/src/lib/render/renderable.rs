use std::{cell::RefCell, rc::Rc};

use as_any::AsAny;

use crate::Context;

pub type ContextPtr = Rc<RefCell<dyn Context>>;

pub trait Renderable: AsAny {
  fn was_created(&self) -> bool;

  fn name(&self) -> Option<&String> {
    return None;
  }

  fn create(&mut self, ctx: &ContextPtr) -> crate::Result<()> {
    println!("create {:?}", self.name());
    Ok(())
  }

  fn render_before(&mut self, ctx: &ContextPtr) -> crate::Result<()> {
    println!("render_before {:?}", self.name());
    Ok(())
  }

  fn render_after(&mut self, ctx: &ContextPtr) -> crate::Result<()> {
    println!("render_after {:?}", self.name());
    Ok(())
  }

  fn destroy(&mut self, ctx: &ContextPtr) -> crate::Result<()> {
    println!("destroy {:?}", self.name());
    Ok(())
  }
}
