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

use crate::proxy::TProxy;

mod exmlog;
mod proxy;

fn main() {
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


//  listener.on_receive(|(api, object)| {
//    println!("{:?}", object);
//  });

  listener.on_option(|(api, option)| {
    option.on_name("version", |value| {
      println!("OVALUE STRING: {:?}", value.as_string());
      println!("OVALUE INTEGER: {:?}", value.as_integer());
      println!("OVALUE BOOL: {:?}", value.as_bool());
    });
  });

  listener.on_authorization_state(|(api, state)| {
    state.on_wait_tdlibparameters(|| {
      let paras = SetTdlibParameters::builder()
        .parameters(TdlibParameters::builder()
          .database_directory("tdlib")
          .use_message_database(true)
          .use_secret_chats(true)
          .api_id(toolkit::number::as_i32(env!("API_ID")).unwrap())
          .api_hash(env!("API_HASH"))
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

    let have_authorization_o: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

    state.on_state_ready(|| {
      let mut have_authorization = have_authorization_o.lock().unwrap();
      *have_authorization = true;
      debug!(exmlog::examples(), "Authorization ready");
    });
    state.on_state_logging_out(|| {
      let mut have_authorization = have_authorization_o.lock().unwrap();
      *have_authorization = false;
      debug!(exmlog::examples(), "Logging out");
    });
    state.on_state_closing(|| {
      let mut have_authorization = have_authorization_o.lock().unwrap();
      *have_authorization = false;
      debug!(exmlog::examples(), "Closing");
    });
    state.on_state_closed(|| {
      debug!(exmlog::examples(), "Closed");
    });
  });


  client.daemon("telegram-rs");
}

