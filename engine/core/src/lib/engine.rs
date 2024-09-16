use std::{cell::RefCell, rc::Rc};

use crate::{ContextRef, EventBusPtr, Renderer, RendererPtr, StdEventBus};

pub type EnginePtr = Rc<RefCell<Engine>>;

#[derive(Default)]
pub struct Engine {
  event_bus: Option<EventBusPtr>,
  renderer: Option<RendererPtr>,
}

impl Engine {
  pub fn with_event_bus(mut self, evt: EventBusPtr) -> Self {
    self.event_bus = Some(evt);
    self
  }

  pub fn event_bus(&self) -> Option<&EventBusPtr> {
    self.event_bus.as_ref()
  }

  pub fn set_event_bus(&mut self, evt: EventBusPtr) {
    self.event_bus = Some(evt);
  }

  pub fn with_renderer<R: Renderer + 'static>(mut self, rdr: R) -> Self {
    self.renderer = Some(Rc::new(RefCell::new(rdr)));
    self
  }

  pub fn renderer(&self) -> Option<&RendererPtr> {
    self.renderer.as_ref()
  }

  pub fn set_renderer(&mut self, rdr: RendererPtr) {
    self.renderer = Some(rdr);
  }
}
