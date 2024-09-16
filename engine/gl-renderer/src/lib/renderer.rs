use std::{
  any::Any,
  cell::{Ref, RefCell},
  ops::Deref,
  rc::{Rc, Weak},
};

use as_any::{AsAny, Downcast};
use glow::HasContext as _;

use rhg_core::{
  borrow_downcast, err, here, Context, ContextRef, Error, ErrorKind, GraphicsAPI, Renderable,
  RenderableRef, Renderer, VertexBuffer, VertexBufferRef, WindowPtr,
};

use crate::GLContext;

use super::GLVertexBuffer;

pub struct GLRendererInner {
  context: Option<Rc<dyn Any>>,
  renderables: Vec<RenderableRef>,
  window: Option<WindowPtr>,
}

pub struct GLRenderer(RefCell<GLRendererInner>);

impl GLRenderer {
  pub fn new() -> Self {
    Self(RefCell::new(GLRendererInner {
      context: None,
      renderables: vec![],
      window: None,
    }))
  }

  pub fn context(&self) -> Option<Rc<dyn Any>> {
    self.0.borrow().context.as_ref().cloned()
  }

  pub fn gl_context(&self) -> Option<Rc<GLContext>> {
    self.0.borrow().context.as_ref().and_then(|ctx| {
      let dup = ctx.clone();
      dup.downcast::<GLContext>().ok()
    })
  }
}

impl Drop for GLRenderer {
  fn drop(&mut self) {
    self.destroy().unwrap();
  }
}

impl Renderer for GLRenderer {
  fn window(&self) -> Option<WindowPtr> {
    self.0.borrow().window.as_ref().cloned()
  }

  fn create(&self, gfx_api: &GraphicsAPI<'_>, window: WindowPtr) -> rhg_core::Result<()> {
    let context = match gfx_api {
      #[cfg(not(target_arch = "wasm32"))]
      GraphicsAPI::NativeOpenGL { get_proc_address } => unsafe {
        glow::Context::from_loader_function_cstr(|s| get_proc_address(s))
      },
      #[cfg(target_arch = "wasm32")]
      slint::GraphicsAPI::WebGL {
        canvas_element_id,
        context_type,
      } => {
        use wasm_bindgen::JsCast;

        let canvas = web_sys::window()
          .unwrap()
          .document()
          .unwrap()
          .get_element_by_id(canvas_element_id)
          .unwrap()
          .dyn_into::<web_sys::HtmlCanvasElement>()
          .unwrap();

        let webgl1_context = canvas
          .get_context(context_type)
          .unwrap()
          .unwrap()
          .dyn_into::<web_sys::WebGlRenderingContext>()
          .unwrap();

        glow::Context::from_webgl1_context(webgl1_context)
      }
      _ => {
        return err!(
          ErrorKind::Rendering,
          format!("failed to initialize GLRenderer, unsupported GraphicsAPI",)
        )
      }
    };
    println!("Renderer initialized: {:?}!", context.version());
    self.0.borrow_mut().window = Some(window);
    self.0.borrow_mut().context = Some(Rc::new(GLContext::new(context)));
    Ok(())
  }

  fn create_vertex_buffer(&self) -> rhg_core::Result<VertexBufferRef> {
    let ctx = self
      .context()
      .ok_or_else(|| {
        Error::new(
          ErrorKind::Rendering,
          format!("invalid GLContext"),
          None,
          here!(),
        )
      })?
      .clone();
    let buf: Rc<dyn VertexBuffer> = Rc::new(GLVertexBuffer::default());
    buf.create(&ctx)?;
    Ok(buf)
  }

  fn render_before(&self) -> rhg_core::Result<()> {
    unsafe {
      let gl = self.gl_context().ok_or_else(|| {
        Error::new(
          ErrorKind::Rendering,
          format!("invalid GLContext"),
          None,
          here!(),
        )
      })?;
      gl.clear_color(0.2f32, 0.2f32, 0.2f32, 1.0f32);
      gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
    }
    let ctx = self
      .context()
      .ok_or_else(|| {
        Error::new(
          ErrorKind::Rendering,
          format!("invalid GLContext"),
          None,
          here!(),
        )
      })?
      .clone();
    for renderable in &self.0.borrow().renderables {
      if !renderable.was_created() {
        renderable.create(&ctx)?;
      }
      renderable.render_before(&ctx)?;
    }
    Ok(())
  }

  fn render_after(&self) -> rhg_core::Result<()> {
    let ctx = self.context().expect("invalid GLContext");
    for renderable in &self.0.borrow().renderables {
      renderable.render_after(&ctx)?;
    }
    Ok(())
  }

  fn destroy(&self) -> rhg_core::Result<()> {
    let ctx = self.context().expect("invalid GLContext");
    for buf in &self.0.borrow().renderables {
      buf.destroy(&ctx)?;
    }
    Ok(())
  }
}
