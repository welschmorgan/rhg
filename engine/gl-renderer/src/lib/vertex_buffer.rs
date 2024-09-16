use std::{cell::RefCell, rc::Rc};

use as_any::Downcast;
use glow::{Buffer, HasContext as _, VertexArray};

use rhg_core::{err, here, ContextPtr, Error, ErrorKind, Renderable, Vertex};

use crate::GLContext;

#[derive(Default)]
pub struct GLVertexBuffer {
  ctx: Option<ContextPtr>,
  name: Option<String>,
  vbo: Option<Buffer>,
  vao: Option<VertexArray>,
  vertices: Vec<Vertex<f32>>,
}

impl GLVertexBuffer {
  pub fn new() -> Self {
    Self {
      ctx: None,
      name: None,
      vao: None,
      vbo: None,
      vertices: Vec::new(),
    }
  }

  pub fn named<N: AsRef<str>>(name: N) -> Self {
    Self::new().with_name(name)
  }

  pub fn with_name<N: AsRef<str>>(mut self, name: N) -> Self {
    self.name = Some(name.as_ref().to_string());
    self
  }

  pub fn with_vertex(mut self, v: Vertex) -> Self {
    self.vertices.push(v);
    self
  }

  pub fn with_vertices<const N: usize>(mut self, v: [Vertex; N]) -> Self {
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

  pub fn gl_context(&self) -> Option<Rc<RefCell<GLContext>>> {
    self.context().map(|ctx| {
      ctx
        .borrow()
        .internal()
        .downcast_ref::<Rc<RefCell<GLContext>>>()
        .unwrap()
        .clone()
    })
  }
}

impl Drop for GLVertexBuffer {
  fn drop(&mut self) {
    if let Some(ctx) = self.ctx.as_ref() {
      self
        .destroy(&ctx.clone())
        .map_err(|e| format!("failed to destroy {:?}, {}", self.name(), e))
        .unwrap();
    }
  }
}

impl Renderable for GLVertexBuffer {
  fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  fn was_created(&self) -> bool {
    self.vbo.is_some() && self.vao.is_some()
  }

  fn create(&mut self, ctx: &ContextPtr) -> rhg_core::Result<()> {
    self.ctx = Some(ctx.clone());
    unsafe {
      let gl = self.gl_context().as_ref().unwrap().clone();
      let gl = gl.borrow();
      self.vbo = Some(gl.create_buffer().map_err(|e| {
        Error::new(
          ErrorKind::Rendering,
          format!("failed to create VBO, {}", e),
          None,
          here!(),
        )
      })?);
      self.vao = Some(gl.create_vertex_array().map_err(|e| {
        Error::new(
          ErrorKind::Rendering,
          format!("failed to create VAO, {}", e),
          None,
          here!(),
        )
      })?);

      gl.bind_buffer(glow::ARRAY_BUFFER, self.vbo);

      gl.buffer_data_u8_slice(
        glow::ARRAY_BUFFER,
        self.vertices.align_to().1,
        glow::STATIC_DRAW,
      );

      gl.bind_vertex_array(self.vao);
      gl.enable_vertex_attrib_array(0);
      gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 8, 0);

      gl.bind_buffer(glow::ARRAY_BUFFER, None);
      gl.bind_vertex_array(None);
    }
    Ok(())
  }

  fn render_before(&mut self, ctx: &ContextPtr) -> rhg_core::Result<()> {
    let gl = self.gl_context().as_ref().unwrap().clone();
    let gl = gl.borrow();
    unsafe {
      // Retrieving the buffer with glow only works with native builds right now. For WASM this requires https://github.com/grovesNL/glow/pull/190
      // That means we can't properly restore the vao/vbo, but this is okay for now as this only works with femtovg, which doesn't rely on
      // these bindings to persist across frames.
      #[cfg(not(target_arch = "wasm32"))]
      let old_buffer =
        std::num::NonZeroU32::new(gl.get_parameter_i32(glow::ARRAY_BUFFER_BINDING) as u32)
          .map(glow::NativeBuffer);
      #[cfg(target_arch = "wasm32")]
      let old_buffer = None;

      gl.bind_buffer(glow::ARRAY_BUFFER, self.vbo);

      #[cfg(not(target_arch = "wasm32"))]
      let old_vao =
        std::num::NonZeroU32::new(gl.get_parameter_i32(glow::VERTEX_ARRAY_BINDING) as u32)
          .map(glow::NativeVertexArray);
      #[cfg(target_arch = "wasm32")]
      let old_vao = None;

      gl.bind_vertex_array(self.vao);

      gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);

      gl.bind_buffer(glow::ARRAY_BUFFER, old_buffer);
      gl.bind_vertex_array(old_vao);
    }

    Ok(())
  }

  fn render_after(&mut self, ctx: &ContextPtr) -> rhg_core::Result<()> {
    Ok(())
  }

  fn destroy(&mut self, ctx: &ContextPtr) -> rhg_core::Result<()> {
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
      let gl = self.gl_context().as_ref().unwrap().clone();
      let gl = gl.borrow();
      if let Some(vbo) = self.vbo {
        gl.delete_buffer(vbo);
      }
      if let Some(vao) = self.vao {
        gl.delete_vertex_array(vao);
      }
    }
    Ok(())
  }
}
