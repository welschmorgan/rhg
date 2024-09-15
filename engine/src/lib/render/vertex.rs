use std::{
  collections::VecDeque,
  marker::PhantomData,
  ops::{Deref, DerefMut},
};

use crate::Vector;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Vertex<T = f32, P = Vector<T, 3>, C = Vector<T, 4>, TC = Vector<T, 3>> {
  _phantom: PhantomData<T>,
  position: P,
  color: C,
  tex_coord: TC,
}

impl<T, P, C, TC> Vertex<T, P, C, TC> {
  pub fn new(position: P, color: C, tex_coord: TC) -> Self {
    Self {
      _phantom: PhantomData {},
      position,
      color,
      tex_coord,
    }
  }

  pub fn position(&self) -> &P {
    &self.position
  }

  pub fn color(&self) -> &C {
    &self.color
  }

  pub fn tex_coord(&self) -> &TC {
    &self.tex_coord
  }
}

pub struct VertexList<T, P = Vector<T, 3>, C = Vector<T, 4>, TC = Vector<T, 3>>(
  VecDeque<Vertex<T, P, C, TC>>,
);

impl<T, P, C, TC> VertexList<T, P, C, TC> {
  pub fn new() -> Self {
    Self(VecDeque::new())
  }
}

impl<T, P, C, TC> Deref for VertexList<T, P, C, TC> {
  type Target = VecDeque<Vertex<T, P, C, TC>>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T, P, C, TC> DerefMut for VertexList<T, P, C, TC> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

#[cfg(test)]
mod tests {
  use crate::{Vec2u8, Vec3u8, Vec4u8};

  use super::{Vertex, VertexList};

  #[test]
  fn vertex_new() {
    let vert = Vertex::<u8>::new(
      Vec3u8::new(2, 3, 0),
      Vec4u8::new(255, 0, 0, 255),
      Vec3u8::new(0, 0, 0),
    );
  }

  #[test]
  fn vertex_list() {
    let mut list = VertexList::<u8>::new();
  }
}
