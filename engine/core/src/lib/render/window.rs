use std::{cell::RefCell, rc::Rc};

use raw_window_handle::WindowHandle;

pub type WindowPtr = Rc<RefCell<dyn Window>>;

pub trait Window {
  fn create(&mut self);
  fn destroy(&mut self);

  fn size(&self) -> (u32, u32);
  fn set_size(&mut self, w: u32, h: u32);

  fn position(&self) -> (i32, i32);
  fn set_position(&mut self, x: i32, y: i32);
}
