#[macro_use]
extern crate log;

use rtdlib::types::*;
use telegram_client::api::Api;
use telegram_client::client::Client;
use std::sync::{Arc, Mutex, Condvar};
use std::io;


#[tokio::main]
async fn main() {
  simple_logger::init().unwrap();
  log::set_max_level(log::LevelFilter::Debug);
  let api = Api::rasync();
  Client::set_log_verbosity_level(2);
  let mut client = Client::new(api.api().clone());
  client.warn_unregister_listener(false);
  let listener = client.listener();
  let have_authorization = Arc::new((Mutex::new(false), Condvar::new()));
  let wait_auth = Arc::clone(&have_authorization);
  listener.on_update_authorization_state(move |(api, update)| {
    let state = update.authorization_state();
    state.on_wait_tdlib_parameters(|_| {
      let paras = SetTdlibParameters::builder()
        .parameters(TdlibParameters::builder()
          .database_directory("tdlib")
          .use_test_dc(false)
          .api_id(toolkit::number::as_i64(env!("API_ID")).unwrap())
          .api_hash(env!("API_HASH"))
          .system_language_code("en")
          .device_model("Desktop")
          .system_version("Unknown")
          .application_version(env!("CARGO_PKG_VERSION"))
          .enable_storage_optimizer(true)
          .build())
        .build();
      api.set_tdlib_parameters(&paras).unwrap();
      debug!("Set tdlib parameters");
    });
    state.on_wait_encryption_key(|_| {
      let params = SetDatabaseEncryptionKey::builder().build();
      api.set_database_encryption_key(&params).unwrap();
      debug!("Set encryption key");
    });
    state.on_wait_phone_number(|_| {
      let params = SetAuthenticationPhoneNumber::builder().phone_number(env!("TG_PHONE")).build();
      api.set_authentication_phone_number(&params).unwrap();
      debug!("Set phone number");
    });
    state.on_wait_code(|_| {
      info!("wait for auth code");
      let code = type_in();
      let code = CheckAuthenticationCode::builder().code(&code).build();
      api.check_authentication_code(&code).unwrap();
      debug!("Set auth code");
    });

    state.on_ready(|_| {
      let (lock, cvar) = &*have_authorization;
      let mut authorized = lock.lock().unwrap();
      *authorized = true;
      cvar.notify_one();
    });
    state.on_logging_out(|_| {
      let (lock, _) = &*have_authorization;
      let mut authorized = lock.lock().unwrap();
      *authorized = false;
      debug!("Logging out");
    });
    state.on_closing(|_| {
      let (lock, _) = &*have_authorization;
      let mut authorized = lock.lock().unwrap();
      *authorized = false;
      debug!("Closing");
    });
    state.on_closed(|_| {
      debug!("Closed");
    });
    Ok(())
  });

  client.start();
  let (lock, cvar) = &*wait_auth;
  let mut started = lock.lock().unwrap();
  while !*started {
    started = cvar.wait(started).unwrap();
  }
  let chats = api.search_public_chats(SearchPublicChats::builder().query("rust async").build()).await.unwrap();
  info!("found chats: {}", chats.chat_ids().len());
  for chat_id in chats.chat_ids() {
    let chat = api.get_chat(GetChat::builder().chat_id(*chat_id)).await.unwrap();
    info!("{:?}", chat)

  }
}


fn type_in() -> String {
  let mut input = String::new();
  match io::stdin().read_line(&mut input) {
    Ok(_) => input.trim().to_string(),
    Err(e) => panic!("Can not get input value: {:?}", e)
  }
}
