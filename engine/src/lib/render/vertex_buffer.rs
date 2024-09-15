use glow::{Buffer, HasContext, VertexArray};

use crate::{err, here, Error, ErrorKind, Vector};

use super::{ContextPtr, Renderable, Vertex};

#[derive(Default)]
pub struct VertexBuffer<T = f32, P = Vector<T, 3>, C = Vector<T, 4>, TC = Vector<T, 3>> {
  ctx: Option<ContextPtr>,
  name: Option<String>,
  vbo: Option<glow::Buffer>,
  vao: Option<glow::VertexArray>,
  vertices: Vec<Vertex<T, P, C, TC>>,
}

impl<T, P, C, TC> VertexBuffer<T, P, C, TC> {
  pub fn new() -> Self {
    Self {
      ctx: None,
      name: None,
      vao: None,
      vbo: None,
      vertices: vec![],
    }
  }

  pub fn named<N: AsRef<str>>(name: N) -> Self {
    Self::new().with_name(name)
  }

  pub fn with_name<N: AsRef<str>>(mut self, name: N) -> Self {
    self.name = Some(name.as_ref().to_string());
    self
  }

  pub fn with_vertex(mut self, v: Vertex<T, P, C, TC>) -> Self {
    self.vertices.push(v);
    self
  }

  pub fn with_vertices<const N: usize>(mut self, v: [Vertex<T, P, C, TC>;N]) -> Self {
    self.vertices.extend(v);
    self
  }

  pub fn context(&self) -> Option<&ContextPtr> {
    self.ctx.as_ref()
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn vao(&self) -> Option<&VertexArray> {
    self.vao.as_ref()
  }

  pub fn vbo(&self) -> Option<&Buffer> {
    self.vbo.as_ref()
  }
}

impl<T, P, C, TC> Drop for VertexBuffer<T, P, C, TC> {
  fn drop(&mut self) {
    if let Some(ctx) = self.ctx.as_ref() {
      self
        .destroy(&ctx.clone())
        .map_err(|e| format!("failed to destroy {:?}, {}", self.name(), e))
        .unwrap();
    }
  }
}

impl<T, P, C, TC> Renderable for VertexBuffer<T, P, C, TC> {
  fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  fn was_created(&self) -> bool {
    self.vbo.is_some() && self.vao.is_some()
  }

  fn create(&mut self, ctx: &ContextPtr) -> crate::Result<()> {
    self.ctx = Some(ctx.clone());
    unsafe {
      self.vbo = Some(ctx.borrow().create_buffer().map_err(|e| {
        Error::new(
          ErrorKind::Rendering,
          format!("failed to create VBO, {}", e),
          None,
          here!(),
        )
      })?);
      self.vao = Some(ctx.borrow().create_vertex_array().map_err(|e| {
        Error::new(
          ErrorKind::Rendering,
          format!("failed to create VAO, {}", e),
          None,
          here!(),
        )
      })?);
    }
    Ok(())
  }

  fn render_before(&mut self, ctx: &ContextPtr) -> crate::Result<()> {
    Ok(())
  }

  fn render_after(&mut self, ctx: &ContextPtr) -> crate::Result<()> {
    Ok(())
  }

  fn destroy(&mut self, ctx: &ContextPtr) -> crate::Result<()> {
    if !self.was_created() {
      return err!(
        ErrorKind::Rendering,
        format!(
          "cannot destroy buffer{}, was_created = false",
          match self.name() {
            Some(name) => format!("'{}'", name),
            None => String::new(),
          }
        )
      );
    }
    unsafe {
      if let Some(vbo) = self.vbo {
        self.ctx.as_ref().unwrap().borrow().delete_buffer(vbo);
      }
      if let Some(vao) = self.vao {
        self.ctx.as_ref().unwrap().borrow().delete_vertex_array(vao);
      }
    }
    Ok(())
  }
}
