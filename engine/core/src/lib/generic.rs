use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use crate::Context;

pub trait AsGenericContext {
  fn as_generic_context(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn Context>>;
}
