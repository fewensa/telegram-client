use crate::listener::lasync::RasyncListener;
use crate::listener::levent::EventListener;

#[derive(Clone)]
pub struct Listener {
  event_listener: EventListener,
  rasync_listener: RasyncListener,
}


impl Listener {
  pub fn new() -> Self {
    Self {
      event_listener: EventListener::default(),
      rasync_listener: RasyncListener::default(),
    }
  }
}

impl Listener {
  pub fn event_listener(&self) -> &EventListener {
    &self.event_listener
  }
  pub fn rasync_listener(&self) -> &RasyncListener {
    &self.rasync_listener
  }
  pub fn event_listener_mut(&mut self) -> &mut EventListener {
    &mut self.event_listener
  }
  pub fn rasync_listener_mut(&mut self) -> &mut RasyncListener {
    &mut self.rasync_listener
  }
}
