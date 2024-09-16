use std::{cell::RefCell, rc::Rc};

use raw_window_handle::WindowHandle;

pub type WindowPtr = Rc<RefCell<dyn Window>>;

pub trait Window {
  fn create(&mut self);
  fn destroy(&mut self);

  // fn handle(&self) -> &WindowHandle<'_>;
}
