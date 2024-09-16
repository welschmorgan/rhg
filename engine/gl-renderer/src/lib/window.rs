use raw_window_handle::{HasWindowHandle, WindowHandle};
use rhg_core::{here, Error, ErrorKind, Window};

pub struct GLWindow {
  // handle: WindowHandle<'a>,
}

impl GLWindow {
  pub fn new() -> Self {
    Self {}
  }
}

impl Window for GLWindow {
  fn create(&mut self) {}

  fn destroy(&mut self) {}
}
