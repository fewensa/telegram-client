#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate slog;
extern crate slog_term;

use rtdlib::types::*;

use telegram_client::api::Api;
use telegram_client::client::Client;
use std::sync::{Arc, Mutex};
use std::rc::Rc;

mod exmlog;

fn main() {
  let apix = Api::default();
  let mut client = Client::new(apix.clone());
  let listener = client.listener();



//  listener.on_update(|(api, update)| {
//    println!("{:?}", update);
//    api.clone().test();
//  });

  // todo support option name
  listener.on_option_string(|(api, option)| {
    debug!(exmlog::examples(), "option value : {:?}", option.value())
  });

  listener.on_authorization_state_wait_tdlibparameters(|(api, _)| {
    let paras = SetTdlibParameters::builder()
      .parameters(TdlibParameters::builder()
        .database_directory("tdlib".to_string())
        .use_message_database(true)
        .use_secret_chats(true)
        .api_id(self::api_id())
        .api_hash(self::api_hash())
        .system_language_code("en".to_string())
        .device_model("Desktop".to_string())
        .system_version("Unknown".to_string())
        .application_version(env!("CARGO_PKG_VERSION").to_string())
        .enable_storage_optimizer(true)
        .build())
      .build();
    api.send(&paras);
    debug!(exmlog::examples(), "Set tdlib parameters");
  });
  listener.on_authorization_state_wait_encryption_key(|(api, _)| {
    api.send(CheckDatabaseEncryptionKey::builder().build());
    debug!(exmlog::examples(), "Set encryption key");
  });
  listener.on_authorization_state_wait_phone_number(|(api, _)| {
    api.send(SetAuthenticationPhoneNumber::builder().phone_number(env!("TG_PHONE").to_string()).build());
    debug!(exmlog::examples(), "Set phone number");
  });
  listener.on_authorization_state_wait_password(|(api, _)| {
    api.send(CheckAuthenticationPassword::builder().password(env!("TG_PASSWORD").to_string()).build());
    debug!(exmlog::examples(), "Set password");
  });


  let have_authorization_o: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
  let have_authorization_out = Arc::clone(&have_authorization_o);
  let have_authorization_close = Arc::clone(&have_authorization_o);
  listener.on_authorization_state_ready(move |(api, _)| {
    let mut have_authorization = have_authorization_o.lock().unwrap();
    *have_authorization = true;
    debug!(exmlog::examples(), "Authorization ready");
  });
  listener.on_authorization_state_logging_out(move |(api, _)| {
    let mut have_authorization = have_authorization_out.lock().unwrap();
    *have_authorization = true;
    debug!(exmlog::examples(), "Logging out");
  });
  listener.on_authorization_state_closing(move |(api, _)| {
    let mut have_authorization = have_authorization_close.lock().unwrap();
    *have_authorization = true;
    debug!(exmlog::examples(), "Closing");
  });
  listener.on_authorization_state_closed(|(api, _)| {
    debug!(exmlog::examples(), "Closed");
  });


  client.daemon("telegram-rs");
}


fn api_id() -> i32 {
  let v = std::env::var("API_ID").unwrap();
  toolkit::number::as_i32(v).unwrap()
}

fn api_hash() -> String {
  std::env::var("API_HASH").unwrap()
}

