use std::{cell::RefCell, rc::Rc};

use raw_window_handle::{HasWindowHandle, WindowHandle};
use rhg_core::{here, Error, ErrorKind, Window};

pub struct GLWindow {
  handle: Rc<RefCell<i_slint_core::api::Window>>,
}

impl GLWindow {
  pub fn new(handle: Rc<RefCell<i_slint_core::api::Window>>) -> Self {
    Self { handle }
  }
}
