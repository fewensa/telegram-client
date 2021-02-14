use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use rtdlib::Tdlib;

use crate::api::Api;
use crate::listener::Listener;
use crate::rtd::TdRecv;

#[derive(Clone)]
pub struct Client {
  stop_flag: Arc<Mutex<bool>>,
  listener: Listener,
  api: Api,
  warn_unregister_listener: bool,
}

impl Default for Client {
  /// Creates a default Client
  ///
  /// # Examples
  ///
  /// ```
  /// use telegram_client::client::Client;
  /// use telegram_client::api::Api;
  /// let client = Client::new(Api::default());
  /// ```
  fn default() -> Self {
    Client::new(Api::default())
  }
}

impl Client {
  /// Sets the verbosity level of the internal logging of TDLib.
  ///
  /// By default the TDLib uses a log verbosity level of 5.
  ///
  /// # Parameters
  ///
  /// `level` New value of logging verbosity level. Value 0 corresponds to fatal errors,
  /// value 1 corresponds to errors, value 2 corresponds to warnings and debug warnings,
  /// value 3 corresponds to informational, value 4 corresponds to debug, value 5 corresponds
  /// to verbose debug, value greater than 5 and up to 1024 can be used to enable even more logging.
  ///
  /// # Examples
  ///
  /// ```
  /// use telegram_client::client::Client;
  /// Client::set_log_verbosity_level(3);
  /// ```
  pub fn set_log_verbosity_level<'a>(level: i32) -> Result<(), &'a str> {
    Tdlib::set_log_verbosity_level(level)
  }

  /// Sets maximum size of the file to where the internal TDLib log is written before the file will be auto-rotated.
  ///
  /// Unused if log is not written to a file. Defaults to 10 MB.
  ///
  /// # Parameters
  ///
  /// `size` Maximum size of the file to where the internal TDLib log is written before the file will be auto-rotated. Should be positive.
  ///
  /// # Examples
  ///
  /// ```
  /// use telegram_client::client::Client;
  /// Client::set_log_max_file_size(1024 * 1024);
  /// ```
  pub fn set_log_max_file_size(size: i64) {
    Tdlib::set_log_max_file_size(size)
  }

  /// Sets the path to the file where the internal TDLib log will be written.
  ///
  /// By default TDLib writes logs to stderr or an OS specific log. Use this method to write the log to a file instead.
  ///
  /// # Parameters
  ///
  /// `path` Maybe path to a file where the internal TDLib log will be written. Use `None` to switch back to the default logging behaviour.
  ///
  /// # Examples
  ///
  /// ```
  /// use telegram_client::client::Client;
  /// Client::set_log_file_path(Some("/var/log/tdlib/tdlib.log"));
  /// ```
  pub fn set_log_file_path(path: Option<&str>) -> bool {
    Tdlib::set_log_file_path(path)
  }

  /// Creates a new Client with api
  ///
  /// # Examples
  ///
  /// ```
  /// use telegram_client::client::Client;
  /// use telegram_client::api::Api;
  /// let client = Client::new(Api::default());
  /// ```
  pub fn new(api: Api) -> Self {
    let stop_flag = Arc::new(Mutex::new(false));
    Self {
      stop_flag,
      api,
      listener: Listener::new(),
      warn_unregister_listener: true,
    }
  }

  /// Start a Client.
  ///
  /// # Examples
  ///
  /// ```
  /// use telegram_client::client::Client;
  /// let client = Client::default();
  /// client.start();
  /// ```
  pub fn start(&self) -> JoinHandle<()> {
    let lout = self.listener.lout();
    let tdrecv = TdRecv::new();
    tdrecv.start(Arc::new(self.api.clone()), self.stop_flag.clone(), Arc::new(lout), Arc::new(self.warn_unregister_listener))
  }

  /// Start a daemon Client.
  ///
  /// # Examples
  ///
  /// ```
  /// use telegram_client::client::Client;
  /// let client = Client::default();
  /// client.daemon("tgclient");
  /// ```
  pub fn daemon<S: AsRef<str>>(self, name: S) -> std::thread::Result<()> {
    self.start().join()
//    debug!("Telegram client started.");
//    let daemon = Daemon {
//      name: name.as_ref().to_string(),
//    };
//    daemon.run(move |rx: Receiver<State>| {
//      debug!("Worker started.");
//      for signal in rx.iter() {
//        match signal {
//          State::Start => debug!("Worker: Start"),
//          State::Reload => debug!("Worker: Reload"),
//          State::Stop => debug!("Worker: Stop"),
//        };
//      }
//      debug!("Worker finished.");
//    }).expect("Can not start daemon");
//    debug!("{} finished.", name.as_ref());
  }

  pub fn listener(&mut self) -> &mut Listener {
    &mut self.listener
  }

  pub fn warn_unregister_listener(&mut self, value: bool) {
    self.warn_unregister_listener = value
  }
}
