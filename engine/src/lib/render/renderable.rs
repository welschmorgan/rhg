use std::{cell::RefCell, rc::Rc};

pub type ContextPtr = Rc<RefCell<glow::Context>>;

pub trait Renderable {
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
