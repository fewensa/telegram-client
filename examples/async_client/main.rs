#[macro_use]
extern crate log;

use std::io::{self, BufRead};
use std::sync::{Arc, Mutex};

use simple_logger::SimpleLogger;
use tokio::prelude::*;

use rtdlib::Tdlib;
use rtdlib::types::*;
use telegram_client::api::Api;
use telegram_client::client::Client;

use crate::config::{Config, LogType};
use tokio::runtime::Runtime;

mod thelp;
mod tgfn;
mod config;

#[tokio::main]
async fn main() {
  SimpleLogger::new()
    .with_level(log::LevelFilter::Debug)
    .init()
    .unwrap();


  let api_id = env!("API_ID");
  let api_hash = env!("API_HASH");


  let config = Config::default();
  let async_api = Api::rasync();

  let mut client = Client::new(async_api.api().clone());
  let listener = client.listener();



  config.proxy().map(|v| {
    async_api.add_proxy(v);
  });

  config.log().map(|v| {
    Client::set_log_verbosity_level(v.level.clone() as i32).unwrap();
    if v.type_ == LogType::File {
      v.path.clone().map(|v| {
        Client::set_log_file_path(Some(&v[..]));
      });
    }
  });




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
    debug!("-----> {:?}", update);
    let state = update.authorization_state();

    state.on_wait_tdlib_parameters(|_| {
      api.set_tdlib_parameters(SetTdlibParameters::builder().parameters(
        TdlibParameters::builder()
          .database_directory("tdlib")
          .use_message_database(true)
          .use_secret_chats(true)
          .api_id(toolkit::number::as_i64(api_id).unwrap())
          .api_hash(api_hash)
          .system_language_code("ru")
          .device_model("Desktop")
          .system_version("Unknown")
          .application_version(env!("CARGO_PKG_VERSION"))
          .enable_storage_optimizer(true)
          .build()
      ).build()).unwrap();
      debug!("Set tdlib parameters");
    });
    state.on_wait_phone_number(|_| {
      thelp::tip(format!("{} {}", "Please type your telegram phone number:", "(If you copy log to anywhere, don't forget hide your phone number)"));
      tgfn::type_phone_number(api);
    });
    state.on_wait_code(|_| {
      thelp::tip("Please type authentication code:");
      tgfn::type_authentication_code(api);
    });
    state.on_wait_encryption_key(|_| {
      api.check_database_encryption_key(CheckDatabaseEncryptionKey::builder().build()).unwrap();
      debug!("Set encryption key");
    });

    Ok(())
  });


  // let move_api = async_api.clone();
  // listener.on_update_new_message(move |(api, update)| {
  //   let move_api = move_api.clone();
  //   let runtime = Runtime::new().unwrap();
  //   let chat = runtime.block_on(async move {
  //     tokio::time::sleep(std::time::Duration::from_secs(5)).await;
  //     move_api.get_chat(GetChat::builder().chat_id(1)).await
  //   });
  //   debug!("{:?}", chat);
  //   Ok(())
  // });


  client.start();

  // let move_api = async_api.clone();
  // let runtime = Runtime::new().unwrap();
  // let _ = runtime.spawn(async move {
  //   tokio::time::sleep(std::time::Duration::from_secs(5)).await;
  //   let chat = move_api.get_chat(GetChat::builder().chat_id(1)).await;
  //   println!("{:?}", chat);
  // });

  let move_api = async_api.clone();
  tokio::time::sleep(std::time::Duration::from_secs(5)).await;
  let chat = move_api.get_chat(GetChat::builder().chat_id(1)).await;
  println!("{:?}", chat);


  tokio::time::sleep(std::time::Duration::from_secs(10)).await;
}
