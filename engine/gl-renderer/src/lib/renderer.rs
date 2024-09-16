use std::{any::Any, cell::RefCell, ops::Deref, rc::Rc};

use as_any::AsAny;
use glow::HasContext as _;

use rhg_core::{
  borrow_downcast, err, here, AsGenericContext, ContextPtr, Error, ErrorKind, GraphicsAPI,
  Renderable, RenderablePtr, Renderer, VertexBuffer, WindowPtr,
};

use crate::GLContext;

use super::GLVertexBuffer;

pub struct GLRenderer {
  context: Option<Rc<RefCell<GLContext>>>,
  renderables: Vec<RenderablePtr>,
  window: Option<WindowPtr>,
}

impl GLRenderer {
  pub fn new() -> Self {
    Self {
      context: None,
      renderables: vec![],
      window: None,
    }
  }

  pub fn gl_context(&self) -> Rc<RefCell<GLContext>> {
    self.context.as_ref().unwrap().clone()
  }

  pub fn context(&self) -> ContextPtr {
    let gl_ctx = self.context.as_ref().unwrap().clone();
    let ctx = AsGenericContext::as_generic_context(gl_ctx);
    ctx
  }
}

impl Drop for GLRenderer {
  fn drop(&mut self) {
    self.destroy().unwrap();
  }
}

impl Renderer for GLRenderer {
  fn window(&self) -> Option<&WindowPtr> {
    self.window.as_ref()
  }

  fn create(&mut self, gfx_api: &GraphicsAPI<'_>, window: WindowPtr) -> rhg_core::Result<()> {
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
    self.window = Some(window);
    self.context = Some(Rc::new(RefCell::new(GLContext::new(context))));
    Ok(())
  }

  fn create_vertex_buffer(&mut self) -> rhg_core::Result<Rc<RefCell<dyn VertexBuffer>>> {
    let ctx = self.context();
    let mut buf = GLVertexBuffer::default();
    buf.create(&ctx)?;
    Ok(
      self
        .add_renderable(buf)
        .as_any_mut()
        .downcast_ref::<Rc<RefCell<dyn VertexBuffer>>>()
        .unwrap()
        .clone(),
    )
  }

  fn add_renderable<R: Renderable>(
    &mut self,
    mut r: R,
  ) -> rhg_core::Result<Rc<RefCell<dyn Renderable>>>
  where
    Self: Sized,
  {
    r.create(&self.context())?;
    let ptr = Rc::new(RefCell::new(r));
    self.renderables.push(ptr.clone());
    Ok(ptr)
  }

  fn render_before(&mut self) -> rhg_core::Result<()> {
    unsafe {
      let gl = self.gl_context();
      gl.borrow().clear_color(0.2f32, 0.2f32, 0.2f32, 1.0f32);
      gl.borrow()
        .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
    }
    let ctx = self.context().clone();
    for renderable in &self.renderables {
      let mut renderable = renderable.borrow_mut();
      if !renderable.was_created() {
        renderable.create(&ctx)?;
      }
      renderable.render_before(&ctx)?;
    }
    Ok(())
  }

  fn render_after(&mut self) -> rhg_core::Result<()> {
    let ctx = self.context();
    for renderable in &self.renderables {
      renderable.borrow_mut().render_after(&ctx)?;
    }
    Ok(())
  }

  fn destroy(&mut self) -> rhg_core::Result<()> {
    let ctx = self.context();
    for buf in &self.renderables {
      buf.borrow_mut().destroy(&ctx)?;
    }
    Ok(())
  }
}
