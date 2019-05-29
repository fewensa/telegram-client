use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;

use daemon::Daemon;
use daemon::DaemonRunner;
use daemon::State;
use rtdlib::tdjson;

use crate::api::Api;
use crate::listener::Listener;
use crate::rtd::tdrecv::TdRecv;
use crate::tglog;

pub struct Client {
  stop_flag: Arc<Mutex<bool>>,
  listener: Listener,
  api: Api,
}

impl Default for Client {
  fn default() -> Self {
    Client::new(Api::default())
  }
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

  pub fn new(api: Api) -> Self {
    let stop_flag = Arc::new(Mutex::new(false));
    Self {
      stop_flag,
      api,
      listener: Listener::new(),
    }
  }

  pub fn start(self) {
    let lout = self.listener.lout();
    let tdrecv = TdRecv::new();
    tdrecv.start(Arc::new(self.api), self.stop_flag.clone(), Arc::new(lout));
  }

  pub fn daemon<S: AsRef<str>>(self, name: S) {
    self.start();
    debug!(tglog::telegram(), "Telegram client started.");
    let daemon = Daemon {
      name: name.as_ref().to_string(),
    };
    daemon.run(move |rx: Receiver<State>| {
      debug!(tglog::telegram(), "Worker started.");
      for signal in rx.iter() {
        match signal {
          State::Start => debug!(tglog::telegram(), "Worker: Start"),
          State::Reload => debug!(tglog::telegram(), "Worker: Reload"),
          State::Stop => debug!(tglog::telegram(), "Worker: Stop"),
        };
      }
      debug!(tglog::telegram(), "Worker finished.");
    }).expect("Can not start daemon");
    debug!(tglog::telegram(), "{} finished.", name.as_ref());
  }

  pub fn listener(&mut self) -> &mut Listener {
    &mut self.listener
  }
}
