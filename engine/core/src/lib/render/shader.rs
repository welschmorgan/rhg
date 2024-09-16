use std::{cell::RefCell, rc::Rc};

use super::ContextRef;

pub enum ShaderKind {
  Vertex,
  Shader,
  Geometry,
  Compute,
}

pub type ShaderPtr = Rc<RefCell<dyn Shader>>;

pub trait Shader {
  fn kind(&self) -> ShaderKind;
  fn source(&self) -> String;

  fn create(&mut self, ctx: &ContextRef) -> crate::Result<()>;
  fn destroy(&mut self, ctx: &ContextRef) -> crate::Result<()>;
}
