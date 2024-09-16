use std::{any::Any, cell::RefCell, rc::Rc};

use crate::{err, here, Error, ErrorKind};
use as_any::AsAny;

use super::{ContextRef, Renderable, RenderableRef, ShaderPtr, VertexBuffer, VertexBufferRef, Window, WindowPtr};

pub trait Renderer: AsAny {
  fn create(&self, gfx_api: &GraphicsAPI<'_>, window: WindowPtr) -> crate::Result<()>;
  fn destroy(&self) -> crate::Result<()>;

  fn window(&self) -> Option<WindowPtr>;

  fn render_before(&self) -> crate::Result<()>;
  fn render_after(&self) -> crate::Result<()>;

  fn create_vertex_buffer(&self) -> crate::Result<VertexBufferRef>;
  // fn create_shader(&mut self) -> crate::Result<ShaderPtr>;
}

/// Taken from slint's API
#[derive(Clone)]
#[non_exhaustive]
pub enum GraphicsAPI<'a> {
  /// The rendering is done using OpenGL.
  NativeOpenGL {
    /// Use this function pointer to obtain access to the OpenGL implementation - similar to `eglGetProcAddress`.
    get_proc_address: &'a dyn Fn(&core::ffi::CStr) -> *const core::ffi::c_void,
  },
  /// The rendering is done on a HTML Canvas element using WebGL.
  WebGL {
    /// The DOM element id of the HTML Canvas element used for rendering.
    canvas_element_id: &'a str,
    /// The drawing context type used on the HTML Canvas element for rendering. This is the argument to the
    /// `getContext` function on the HTML Canvas element.
    context_type: &'a str,
  },
}

impl<'a> core::fmt::Debug for GraphicsAPI<'a> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      GraphicsAPI::NativeOpenGL { .. } => write!(f, "GraphicsAPI::NativeOpenGL"),
      GraphicsAPI::WebGL { context_type, .. } => {
        write!(f, "GraphicsAPI::WebGL(context_type = {})", context_type)
      }
    }
  }
}

pub type RendererPtr = Rc<RefCell<dyn Renderer>>;
