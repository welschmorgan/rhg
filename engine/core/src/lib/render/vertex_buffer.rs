use std::{cell::{Ref, RefCell, RefMut}, rc::Rc};

use crate::{Renderable, Vertex, VertexList};

pub type VertexBufferRef = Rc<dyn VertexBuffer>;

pub trait VertexBuffer: Renderable {
  fn vertices(&self) -> Ref<'_, Vec<Vertex<f32>>>;
  fn vertices_mut(&mut self) -> RefMut<'_, Vec<Vertex<f32>>>;
}
