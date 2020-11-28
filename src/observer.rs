use std::sync::{RwLock};
use std::collections::HashMap;
use futures::channel::mpsc;
use rtdlib::types::TdType;

lazy_static! {
  static ref OBSERVER: Observer = {
    Observer::new()
  };
}

struct Observer {
  channels: RwLock<HashMap<String, mpsc::Sender<TdType>>>,
}

impl Observer {
  fn new() -> Self {
    Self {
      channels: RwLock::new(HashMap::new())
    }
  }

  fn notify(&self, extra: String, payload: TdType) {
    let mut map = self.channels.write().unwrap();
    if let Some(sender) = map.get_mut(&extra) {
      sender.try_send(payload).unwrap();
    }
  }

  fn subscribe(&self, extra: String) -> mpsc::Receiver<TdType> {
    let (sender, receiver) = mpsc::channel::<TdType>(1);
    match self.channels.write() {
      Ok(mut map) => {
        map.insert(extra, sender);
      }
      _ => {}
    };
    receiver
  }

  fn unsubscribe(&self, extra: &str) {
    match self.channels.write() {
      Ok(mut map) => {
        map.remove(extra);
      }
      _ => {}
    };
  }
}


pub fn notify<T: AsRef<str>>(extra: T, payload: TdType) {
  OBSERVER.notify(extra.as_ref().to_string(), payload)
}

pub fn subscribe<T: AsRef<str>>(extra: T) -> mpsc::Receiver<TdType>{
  OBSERVER.subscribe(extra.as_ref().to_string())
}

pub fn unsubscribe<T: AsRef<str>>(extra: T) {
  OBSERVER.unsubscribe(extra.as_ref())
}
