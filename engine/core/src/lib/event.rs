use std::{any::Any, cell::RefCell, collections::VecDeque, fmt::Debug, rc::Rc, sync::Arc};

#[derive(Debug, Clone)]
pub enum Event {
  EngineInitStarted,
  EngineInitStopped,
  Custom(Arc<dyn Any>),
}

impl PartialEq for Event {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Custom(l0), Self::Custom(r0)) => true,
      _ => core::mem::discriminant(self) == core::mem::discriminant(other),
    }
  }
}

unsafe impl Send for Event {}
unsafe impl Sync for Event {}

pub trait EventListener {
  fn on_event(&mut self, value: &Event) -> crate::Result<()>;
}

pub type EventListenerPtr = Rc<RefCell<dyn EventListener>>;

pub trait EventBus {
  fn listeners(&self) -> &VecDeque<EventListenerPtr>;
  fn listeners_mut(&mut self) -> &mut VecDeque<EventListenerPtr>;

  fn add_listener(&mut self, l: EventListenerPtr) {
    self.listeners_mut().push_back(l);
  }

  fn queued_events(&self) -> &VecDeque<Event>;
  fn queued_events_mut(&mut self) -> &mut VecDeque<Event>;

  fn queue_event(&mut self, e: Event) {
    self.queued_events_mut().push_back(e);
  }

  fn propagate(&mut self) -> crate::Result<()> {
    while let Some(evt) = self.queued_events_mut().pop_front() {
      for lstn in self.listeners_mut() {
        lstn.borrow_mut().on_event(&evt)?;
      }
    }
    Ok(())
  }
}

pub type EventBusPtr = Rc<RefCell<dyn EventBus>>;

pub struct StdEventBus {
  listeners: VecDeque<EventListenerPtr>,
  event_queue: VecDeque<Event>,
}

impl EventBus for StdEventBus {
  fn listeners(&self) -> &VecDeque<EventListenerPtr> {
    &self.listeners
  }

  fn listeners_mut(&mut self) -> &mut VecDeque<EventListenerPtr> {
    &mut self.listeners
  }

  fn queued_events(&self) -> &VecDeque<Event> {
    &self.event_queue
  }

  fn queued_events_mut(&mut self) -> &mut VecDeque<Event> {
    &mut self.event_queue
  }
}

impl Default for StdEventBus {
  fn default() -> Self {
    Self {
      listeners: Default::default(),
      event_queue: Default::default(),
    }
  }
}

#[cfg(test)]
mod tests {
  use std::{
    cell::RefCell,
    ops::{Deref, Index},
    rc::Rc,
  };

  use super::{Event, EventBus, EventListener, StdEventBus};

  #[derive(Debug)]
  struct TestAccu(Vec<Event>);

  impl Deref for TestAccu {
    type Target = Vec<Event>;

    fn deref(&self) -> &Self::Target {
      &self.0
    }
  }

  impl Index<usize> for TestAccu {
    type Output = Event;

    fn index(&self, index: usize) -> &Self::Output {
      &self.0[index]
    }
  }

  impl EventListener for TestAccu {
    fn on_event(&mut self, value: &Event) -> crate::Result<()> {
      self.0.push(value.clone());
      Ok(())
    }
  }

  #[test]
  fn test() {
    let accu = Rc::new(RefCell::new(TestAccu(vec![])));
    let mut bus = StdEventBus::default();
    bus.queue_event(Event::EngineInitStarted);
    bus.add_listener(accu.clone());
    bus.propagate().unwrap();
    assert_eq!(accu.borrow().len(), 1);
    assert_eq!(accu.borrow().get(0), Some(&Event::EngineInitStarted));
  }
}
