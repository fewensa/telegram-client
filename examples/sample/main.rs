#[macro_use]
extern crate log;

use std::fs::File;
use std::path::Path;
use std::sync::{Arc, Mutex};

use rtdlib::types::*;
use telegram_client::api::Api;
use telegram_client::client::Client;

use crate::proxy::TProxy;

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

  // IMPORTANT: THIS WILL USE THE PROXY 127.0.0.1:1080,
  // IF YOU DON'T WANT THIS, PLEASE REMOVE THIS CODE.
  let tproxy = TProxy::new(&api);
  tproxy.add();


  let listener = client.listener();

  let have_authorization: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

//  listener.on_receive(|(api, object)| {
//    println!("{:?}", object);
//  });

  listener.on_update_option(|(api, option)| {
    let value = option.value();
    if value.is_empty() { debug!("Receive an option {} but it's empty", option.name()) }
    if value.is_string() { debug!("Receive an option {}: String => {}", option.name(), value.as_string().map_or("None".to_string(), |v| v.value().clone())) }
    if value.is_integer() { debug!("Receive an option {}: i32 => {}", option.name(), value.as_integer().map_or(-1, |v| v.value())) }
    if value.is_boolean() { debug!("Receive an option {}: bool => {}", option.name(), value.as_boolean().map_or(false, |v| v.value())) }


    match &option.name()[..] {
      "version" => { value.as_string().map(|v| debug!("VERSION IS {}", v.value())); }
      _ => {}
    };
    Ok(())
  });

  listener.on_update_authorization_state(move |(api, update)| {
    let state = update.authorization_state();
    state.on_wait_tdlib_parameters(|_| {
      let paras = SetTdlibParameters::builder()
        .parameters(TdlibParameters::builder()
          .database_directory("tdlib")
          .use_message_database(true)
          .use_secret_chats(true)
          .api_id(toolkit::number::as_i64(api_id).unwrap())
          .api_hash(api_hash)
          .system_language_code("en")
          .device_model("Desktop")
          .system_version("Unknown")
          .application_version(env!("CARGO_PKG_VERSION"))
          .enable_storage_optimizer(true)
          .build())
        .build();
      api.send(&paras);
      debug!("Set tdlib parameters");
    });
    state.on_wait_encryption_key(|_| {
      api.send(CheckDatabaseEncryptionKey::builder().build());
      debug!("Set encryption key");
    });
    state.on_wait_phone_number(|_| {
      api.send(SetAuthenticationPhoneNumber::builder().phone_number(env!("TG_PHONE")).build());
      debug!("Set phone number");
    });
    state.on_wait_password(|_| {
      api.send(CheckAuthenticationPassword::builder().password(env!("TG_PASSWORD")).build());
      debug!("Set password");
    });

    state.on_ready(|_| {
      let mut have_authorization = have_authorization.lock().unwrap();
      *have_authorization = true;
      debug!("Authorization ready");
    });
    state.on_logging_out(|_| {
      let mut have_authorization = have_authorization.lock().unwrap();
      *have_authorization = false;
      debug!("Logging out");
    });
    state.on_closing(|_| {
      let mut have_authorization = have_authorization.lock().unwrap();
      *have_authorization = false;
      debug!("Closing");
    });
    state.on_closed(|_| {
      debug!("Closed");
    });
    Ok(())
  });

  listener.on_update_connection_state(|(api, update)| {
    let state = update.state();
    state
      .on_waiting_for_network(|_| { debug!("waiting for network"); })
      .on_connecting_to_proxy(|_| { debug!("connecting to proxy"); })
      .on_connecting(|_| { debug!("connecting"); })
      .on_updating(|_| { debug!("updating..."); })
      .on_ready(|_| { debug!("connection ready") });
    Ok(())
  });

  listener.on_error(|(api, error)| {
    let code = error.code();
    let message = error.message();
    error!("ERROR [{}] {}", code, message);
    Ok(())
  });

  listener.on_ok(|api| {
    debug!("OK");
    Ok(())
  });

  listener.on_proxy(|(api, pxy)| {
    debug!("Proxy info => {:?}", pxy);
    Ok(())
  });


  client.daemon("telegram-rs");
}

