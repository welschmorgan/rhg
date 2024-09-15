use std::{cell::RefCell, rc::Rc};

use glow::HasContext as _;

use super::{ContextPtr, Renderable, VertexBuffer};

pub type RenderablePtr = Rc<RefCell<dyn Renderable>>;

pub struct GLRenderer {
  context: ContextPtr,
  renderables: Vec<RenderablePtr>,
}

impl GLRenderer {
  pub fn new(context: ContextPtr) -> Self {
    Self {
      context,
      renderables: vec![],
    }
  }

  pub fn create_vertex_buffer(&mut self) -> crate::Result<Rc<RefCell<VertexBuffer>>> {
    let mut buf = VertexBuffer::default();
    buf.create(&self.context)?;
    self.add_renderable(buf)
  }

  pub fn add_renderable<R: Renderable + 'static>(
    &mut self,
    mut r: R,
  ) -> crate::Result<Rc<RefCell<R>>> {
    r.create(&self.context)?;
    let ptr = Rc::new(RefCell::new(r));
    self.renderables.push(ptr.clone());
    Ok(ptr)
  }

  pub fn render_before(&mut self) -> crate::Result<()> {
    unsafe {
      let ctx = self.context.borrow();
      ctx.clear_color(0.2f32, 0.2f32, 0.2f32, 1.0f32);
      ctx.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
    }

    for renderable in &self.renderables {
      renderable.borrow_mut().render_before(&self.context)?;
    }
    Ok(())
  }

  pub fn render_after(&mut self) -> crate::Result<()> {
    for renderable in &self.renderables {
      renderable.borrow_mut().render_after(&self.context)?;
    }
    Ok(())
  }

  pub fn destroy(&mut self) -> crate::Result<()> {
    for buf in &mut self.renderables {
      buf.borrow_mut().destroy(&self.context)?;
    }
    Ok(())
  }
}

impl Drop for GLRenderer {
  fn drop(&mut self) {
    self.destroy().unwrap();
  }
}
