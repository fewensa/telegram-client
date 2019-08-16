#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate slog;
extern crate slog_term;

use std::fs::File;
use std::path::Path;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use rtdlib::types::*;

use telegram_client::api::Api;
use telegram_client::client::Client;
use telegram_client::types::*;

use crate::proxy::TProxy;

#[path = "../../examples_shared/exmlog.rs"]
mod exmlog;
mod proxy;

fn main() {

  let api_id = env!("API_ID");
  let api_hash = env!("API_HASH");

  let log_file = toolkit::path::root_dir().join("tdlib.log");
  if log_file.exists() {
    std::fs::remove_file(&log_file);
  }
  File::create(&log_file).unwrap();

  Client::set_log_verbosity_level(1);
//  Client::set_log_file_path(Some(&toolkit::path::canonicalize_path(log_file).unwrap()[..]));

  let api = Api::default();
  let mut client = Client::new(api.clone());

  let tproxy = TProxy::new(&api);
  tproxy.add();


  let listener = client.listener();

  let have_authorization: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

//  listener.on_receive(|(api, object)| {
//    println!("{:?}", object);
//  });

  listener.on_option(|(api, option)| {
    let value = option.value();
    if value.is_empty() { debug!(exmlog::examples(), "Receive an option {} but it's empty", option.name()) }
    if value.is_string() { debug!(exmlog::examples(), "Receive an option {}: String => {}", option.name(), value.as_string().map_or("None".to_string(), |v| v)) }
    if value.is_integer() { debug!(exmlog::examples(), "Receive an option {}: i32 => {}", option.name(), value.as_integer().map_or(-1, |v| v)) }
    if value.is_bool() { debug!(exmlog::examples(), "Receive an option {}: bool => {}", option.name(), value.as_bool().map_or(false, |v| v)) }

    option.on_name("version", |value| {
      value.as_string().map(|v| { debug!(exmlog::examples(), "VERSION IS {}", v); });
    });
  });

  listener.on_authorization_state(move |(api, state)| {
    state.on_wait_tdlibparameters(|| {
      let paras = SetTdlibParameters::builder()
        .parameters(TdlibParameters::builder()
          .database_directory("tdlib")
          .use_message_database(true)
          .use_secret_chats(true)
          .api_id(toolkit::number::as_i32(api_id).unwrap())
          .api_hash(api_hash)
          .system_language_code("en")
          .device_model("Desktop")
          .system_version("Unknown")
          .application_version(env!("CARGO_PKG_VERSION"))
          .enable_storage_optimizer(true)
          .build())
        .build();
      api.send(&paras);
      debug!(exmlog::examples(), "Set tdlib parameters");
    });
    state.on_wait_encryption_key(|enck| {
      api.send(CheckDatabaseEncryptionKey::builder().build());
      debug!(exmlog::examples(), "Set encryption key");
    });
    state.on_wait_phone_number(|| {
      api.send(SetAuthenticationPhoneNumber::builder().phone_number(env!("TG_PHONE")).build());
      debug!(exmlog::examples(), "Set phone number");
    });
    state.on_wait_password(|a| {
      api.send(CheckAuthenticationPassword::builder().password(env!("TG_PASSWORD")).build());
      debug!(exmlog::examples(), "Set password");
    });

    state.on_ready(|| {
      let mut have_authorization = have_authorization.lock().unwrap();
      *have_authorization = true;
      debug!(exmlog::examples(), "Authorization ready");
    });
    state.on_logging_out(|| {
      let mut have_authorization = have_authorization.lock().unwrap();
      *have_authorization = false;
      debug!(exmlog::examples(), "Logging out");
    });
    state.on_closing(|| {
      let mut have_authorization = have_authorization.lock().unwrap();
      *have_authorization = false;
      debug!(exmlog::examples(), "Closing");
    });
    state.on_closed(|| {
      debug!(exmlog::examples(), "Closed");
    });
  });

  listener.on_connection_state(|(api, update)| {
    update.on_state(|state| {
      match state {
        TGConnectionState::WaitingForNetwork => {
          debug!(exmlog::examples(), "waiting for network")
        },
        TGConnectionState::ConnectingToProxy => {
          debug!(exmlog::examples(), "connection to proxy")
        },
        TGConnectionState::Connecting => {
          debug!(exmlog::examples(), "connecting")
        },
        TGConnectionState::Updating => {
          debug!(exmlog::examples(), "updating...")
        },
        TGConnectionState::Ready => {
          debug!(exmlog::examples(), "connection ready")
        },
      }
    });
  });

  listener.on_error(|(api, error)| {
    let code = error.code().clone().map_or(-1, |v| v);
    let message = error.message().clone().map_or("None".to_string(), |v| v);
    error!(exmlog::examples(), "ERROR [{}] {}", code, message);
  });

  listener.on_ok(|api| {
    debug!(exmlog::examples(), "OK");
  });

  listener.on_proxy(|(api, pxy)| {
    debug!(exmlog::examples(), "Proxy info => {:?}", pxy);
  });


  client.daemon("telegram-rs");
}

