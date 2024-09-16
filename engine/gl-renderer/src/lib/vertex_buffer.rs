use std::{cell::{Ref, RefCell, RefMut}, rc::Rc};

use as_any::Downcast;
use glow::{Buffer, HasContext as _, VertexArray};

use rhg_core::{err, here, ContextRef, Error, ErrorKind, Renderable, Vertex, VertexBuffer};

use crate::GLContext;

pub struct GLVertexBuffer(RefCell<GLVertexBufferInner>);

impl Default for GLVertexBuffer {
  fn default() -> Self {
    Self(RefCell::new(Default::default()))
  }
}

#[derive(Default)]
pub struct GLVertexBufferInner {
  ctx: Option<ContextRef>,
  name: Option<String>,
  vbo: Option<Buffer>,
  vao: Option<VertexArray>,
  vertices: Vec<Vertex<f32>>,
}

impl GLVertexBuffer {
  pub fn new() -> Self {
    Self(RefCell::new(GLVertexBufferInner {
      ctx: None,
      name: None,
      vao: None,
      vbo: None,
      vertices: Vec::new(),
    }))
  }

  pub fn named<N: AsRef<str>>(name: N) -> Self {
    Self::new().with_name(name)
  }

  pub fn with_name<N: AsRef<str>>(self, name: N) -> Self {
    self.0.borrow_mut().name = Some(name.as_ref().to_string());
    self
  }

  pub fn with_vertex(self, v: Vertex) -> Self {
    self.0.borrow_mut().vertices.push(v);
    self
  }

  pub fn with_vertices<const N: usize>(self, v: [Vertex; N]) -> Self {
    self.0.borrow_mut().vertices.extend(v);
    self
  }

  pub fn inner(&self) -> Ref<'_, GLVertexBufferInner> {
    self.0.borrow()
  }

  pub fn inner_mut(&self) -> RefMut<'_, GLVertexBufferInner> {
    self.0.borrow_mut()
  }

  pub fn context(&self) -> Option<ContextRef> {
    self.0.borrow().ctx.clone()
  }

  pub fn gl_context(&self) -> rhg_core::Result<Rc<GLContext>> {
    let ctx = self.0.borrow().ctx
      .as_ref()
      .and_then(|ctx| Rc::downcast::<GLContext>(ctx.clone()).ok());
    ctx.ok_or_else(|| {
      Error::new(
        ErrorKind::Rendering,
        format!("invalid GLContext"),
        None,
        here!(),
      )
    })
  }

  pub fn name(&self) -> Ref<'_, Option<String>> {
    Ref::map(self.0.borrow(), |inner| &inner.name)
  }

  pub fn vao(&self) -> Ref<'_, Option<VertexArray>> {
    Ref::map(self.0.borrow(), |inner| &inner.vao)
  }

  pub fn vbo(&self) -> Ref<'_, Option<Buffer>> {
    Ref::map(self.0.borrow(), |inner| &inner.vbo)
  }

}

impl Drop for GLVertexBuffer {
  fn drop(&mut self) {
    if let Some(ctx) = self.0.borrow().ctx.as_ref() {
      self
        .destroy(&ctx.clone())
        .map_err(|e| format!("failed to destroy {:?}, {}", self.name(), e))
        .unwrap();
    }
  }
}

impl VertexBuffer for GLVertexBuffer {
  fn vertices(&self) -> Ref<'_, Vec<Vertex<f32>>> {
    Ref::map(self.0.borrow(), |inner| &inner.vertices)
  }

  fn vertices_mut(&mut self) -> RefMut<'_, Vec<Vertex<f32>>> {
    RefMut::map(self.0.borrow_mut(), |inner| &mut inner.vertices)
  }
}

impl Renderable for GLVertexBuffer {
  fn name(&self) -> Ref<'_, Option<String>> {
    Ref::map(self.0.borrow(), |inner| &inner.name)
  }

  fn was_created(&self) -> bool {
    let inner = self.0.borrow();
    inner.vbo.is_some() && inner.vao.is_some()
  }

  fn create(&self, ctx: &ContextRef) -> rhg_core::Result<()> {
    self.0.borrow_mut().ctx = Some(ctx.clone());
    let gl_ctx = self.gl_context()?;
    unsafe {
      let gl = self.gl_context().as_ref().unwrap().clone();
      self.inner_mut().vbo = Some(gl.create_buffer().map_err(|e| {
        Error::new(
          ErrorKind::Rendering,
          format!("failed to create VBO, {}", e),
          None,
          here!(),
        )
      })?);
      self.inner_mut().vao = Some(gl.create_vertex_array().map_err(|e| {
        Error::new(
          ErrorKind::Rendering,
          format!("failed to create VAO, {}", e),
          None,
          here!(),
        )
      })?);

      gl.bind_buffer(glow::ARRAY_BUFFER, self.inner().vbo);

      gl.buffer_data_u8_slice(
        glow::ARRAY_BUFFER,
        self.inner().vertices.align_to().1,
        glow::STATIC_DRAW,
      );

      gl.bind_vertex_array(self.inner().vao);
      gl.enable_vertex_attrib_array(0);
      gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 8, 0);

      gl.bind_buffer(glow::ARRAY_BUFFER, None);
      gl.bind_vertex_array(None);
    }
    Ok(())
  }

  fn render_before(&self, ctx: &ContextRef) -> rhg_core::Result<()> {
    let gl = self.gl_context().as_ref().unwrap().clone();
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

      gl.bind_buffer(glow::ARRAY_BUFFER, self.inner().vbo);

      #[cfg(not(target_arch = "wasm32"))]
      let old_vao =
        std::num::NonZeroU32::new(gl.get_parameter_i32(glow::VERTEX_ARRAY_BINDING) as u32)
          .map(glow::NativeVertexArray);
      #[cfg(target_arch = "wasm32")]
      let old_vao = None;

      gl.bind_vertex_array(self.inner().vao);

      gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);

      gl.bind_buffer(glow::ARRAY_BUFFER, old_buffer);
      gl.bind_vertex_array(old_vao);
    }

    Ok(())
  }

  fn render_after(&self, ctx: &ContextRef) -> rhg_core::Result<()> {
    Ok(())
  }

  fn destroy(&self, ctx: &ContextRef) -> rhg_core::Result<()> {
    if !self.was_created() {
      return err!(
        ErrorKind::Rendering,
        format!(
          "cannot destroy buffer{}, was_created = false",
          self.name().as_ref().map(|n| format!("'{}'", n)).unwrap_or_default()
        )
      );
    }
    unsafe {
      let gl = self.gl_context().as_ref().unwrap().clone();
      if let Some(vbo) = self.inner().vbo {
        gl.delete_buffer(vbo);
      }
      if let Some(vao) = self.inner().vao {
        gl.delete_vertex_array(vao);
      }
    }
    Ok(())
  }
}
