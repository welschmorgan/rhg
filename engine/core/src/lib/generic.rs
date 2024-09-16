use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use crate::{Context, Renderer};

pub fn borrow_downcast<T: Any>(cell: &RefCell<dyn Any>) -> Option<Ref<T>> {
  let r = cell.borrow();
  if (*r).type_id() == TypeId::of::<T>() {
    Some(Ref::map(r, |x| x.downcast_ref::<T>().unwrap()))
  } else {
    None
  }
}

pub fn borrow_downcast_mut<T: Any>(cell: &RefCell<dyn Any>) -> Option<RefMut<T>> {
  let r = cell.borrow_mut();
  if (*r).type_id() == TypeId::of::<T>() {
    Some(RefMut::map(r, |x| x.downcast_mut::<T>().unwrap()))
  } else {
    None
  }
}

#[cfg(feature = "trait_upcasting")]
pub trait BorrowUpcast<Trait: ?Sized> {
  fn borrow_upcast(it: Rc<RefCell<Self>>) -> Rc<RefCell<Trait>>;
}

#[cfg(test)]
mod tests {
  use std::{
    any::Any,
    cell::{Ref, RefCell},
    ops::Deref,
    rc::Rc,
  };

  use crate::{borrow_downcast, Context};

  #[cfg(feature = "trait_upcasting")]
  use super::BorrowUpcast;

  trait Base: Any {
    fn say(&self);
  }

  struct Concrete {}

  impl Base for Concrete {
    fn say(&self) {
      println!("hello");
    }
  }

  #[cfg(feature = "trait_upcasting")]
  impl BorrowUpcast<dyn Base> for Concrete {
    fn borrow_upcast(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn Base>> {
      it
    }
  }

  #[cfg(feature = "trait_upcasting")]
  #[test]
  fn upcast() {
    let concrete = Rc::new(RefCell::new(Concrete {}));
    let base: Rc<RefCell<dyn Base>> = BorrowUpcast::borrow_upcast(concrete);
    base.borrow().say();
  }

  #[test]
  fn downcast() {
    let base: Rc<RefCell<dyn Base>> = Rc::new(RefCell::new(Concrete {}));
    let concrete: Ref<'_, Concrete> = borrow_downcast::<Concrete>(base.deref()).unwrap();
  }
}
