use std::sync::{Arc, mpsc, Mutex};
use std::thread;

use rtdlib::tdjson;

use crate::listener::Listener;
use crate::tdrecv::TdRecv;

pub struct Client {
  stop_flag: Arc<Mutex<bool>>,
  listener: Listener,
}

impl Client {
  pub fn set_log_verbosity_level<'a>(level: i32) -> Result<(), &'a str> {
    tdjson::Tdlib::set_log_verbosity_level(level)
  }

  pub fn set_log_max_file_size(size: i64) {
    tdjson::Tdlib::set_log_max_file_size(size)
  }

  pub fn set_log_file_path(path: Option<&str>) -> bool {
    tdjson::Tdlib::set_log_file_path(path)
  }

  pub fn new() -> Self {
    let stop_flag = Arc::new(Mutex::new(false));
    Self {
      stop_flag,
      listener: Listener::new(),
    }
  }

  pub fn start(&self) {
//    let lout = self.listener.lout();
    let tdrecv = TdRecv::new();
    tdrecv.start(self.stop_flag.clone());
  }

  pub fn listener(&mut self) -> &mut Listener {
    &mut self.listener
  }

//  pub fn on_update(&self) -> &Self {
//
//  }
}



