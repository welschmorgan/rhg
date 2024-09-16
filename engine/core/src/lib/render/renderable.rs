use std::{any::Any, cell::{Ref, RefCell}, rc::Rc};

use as_any::AsAny;

use crate::Context;

pub type ContextRef = Rc<dyn Any>;
pub type RenderableRef = Rc<dyn Renderable>;

pub trait Renderable: AsAny {
  fn was_created(&self) -> bool;

  fn name(&self) -> Ref<'_, Option<String>>;

  fn create(&self, ctx: &ContextRef) -> crate::Result<()> {
    dbg!("create {:?}", self.name());
    Ok(())
  }

  fn render_before(&self, ctx: &ContextRef) -> crate::Result<()> {
    dbg!("render_before {:?}", self.name());
    Ok(())
  }

  fn render_after(&self, ctx: &ContextRef) -> crate::Result<()> {
    dbg!("render_after {:?}", self.name());
    Ok(())
  }

  fn destroy(&self, ctx: &ContextRef) -> crate::Result<()> {
    dbg!("destroy {:?}", self.name());
    Ok(())
  }
}
