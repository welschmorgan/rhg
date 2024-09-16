use std::{cell::RefCell, rc::Rc};

use crate::{Renderable, Vertex, VertexList};

pub type VertexBufferPtr = Rc<RefCell<dyn VertexBuffer>>;

pub trait VertexBuffer: Renderable + Drop {
  fn vertices(&self) -> &VertexList<Vertex<f32>>;
  fn vertices_mut(&mut self) -> &mut VertexList<Vertex<f32>>;
}
