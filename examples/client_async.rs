#[macro_use]
extern crate log;

use simple_logger::SimpleLogger;

use rtdlib::types::*;
use telegram_client::api::*;
use telegram_client::client::Client;

use crate::helpers::config::{Config, LogType};
use crate::helpers::{tgfn, thelp};

mod helpers;

#[tokio::main]
async fn main() {
  SimpleLogger::new()
    .with_level(log::LevelFilter::Debug)
    .init()
    .unwrap();

  let api_id = std::env::var("API_ID").expect("set API_ID to run this example");
  let api_hash = std::env::var("API_HASH").expect("set API_HASH to run this example");

  let config = Config::default();
  let api = Api::rasync();

  let mut client = Client::new(api.api().clone());
  let listener = client.listener().rasync_listener_mut();

  if let Some(proxy) = config.proxy() {
    futures::executor::block_on(async {
      api.clone().add_proxy(proxy).await.unwrap();
    });
  }

  if let Some(v) = config.log() {
    Client::set_log_verbosity_level(v.level as i32).unwrap();
    if v.type_ == LogType::File {
      if let Some(path) = v.path {
        Client::set_log_file_path(Some(&path));
      }
    }
  }

  listener.on_receive(|(_api, object)| {
    Box::pin(async move {
      println!("{}", object);
      Ok(())
    })
  });

  listener.on_update_option(|(_api, option)| {
    Box::pin(async move {
      let value = option.value();
      if value.is_empty() {
        debug!("Receive an option {} but it's empty", option.name())
      }
      if value.is_string() {
        debug!(
          "Receive an option {}: String => {}",
          option.name(),
          value
            .as_string()
            .map_or("None".to_string(), |v| v.value().clone())
        )
      }
      if value.is_integer() {
        debug!(
          "Receive an option {}: i32 => {}",
          option.name(),
          value.as_integer().map_or(-1, |v| v.value())
        )
      }
      if value.is_boolean() {
        debug!(
          "Receive an option {}: bool => {}",
          option.name(),
          value.as_boolean().map(|v| v.value()).unwrap_or(false)
        )
      }

      if option.name() == "version" {
        if let Some(v) = value.as_string() {
          debug!("VERSION IS {}", v.value());
        }
      }
      Ok(())
    })
  });

  listener.on_update_authorization_state(move |(api, update)| {
    let api_id = api_id.clone();
    let api_hash = api_hash.clone();
    Box::pin(async move {
      let rasync_api = api.rasync_api();
      debug!("-----> {:?}", update);
      let state = update.authorization_state();

      match state {
        AuthorizationState::WaitTdlibParameters(_) => {
          rasync_api
            .set_tdlib_parameters(
              SetTdlibParameters::builder()
                .database_directory("tdlib")
                .files_directory("tdlib")
                .database_encryption_key("")
                .use_message_database(true)
                .use_secret_chats(true)
                .api_id(toolkit::number::as_i64(api_id.as_str()).unwrap())
                .api_hash(api_hash.as_str())
                .system_language_code("ru")
                .device_model("Desktop")
                .system_version("Unknown")
                .application_version(env!("CARGO_PKG_VERSION"))
                .build(),
            )
            .await?;
          debug!("Set tdlib parameters");
        }
        AuthorizationState::WaitPhoneNumber(_) => {
          thelp::tip(format!(
            "{} {}",
            "Please type your telegram phone number:",
            "(If you copy log to anywhere, don't forget hide your phone number)"
          ));
          tgfn::type_phone_number(&api);
        }
        AuthorizationState::WaitCode(_) => {
          thelp::tip("Please type authentication code:");
          tgfn::type_authentication_code(&api);
        }
        _ => {}
      }
      Ok(())
    })
  });

  listener.on_update_new_message(|(api, update)| {
    Box::pin(async move {
      let rasync_api = api.rasync_api();
      let message = update.message();
      let chat = rasync_api
        .get_chat(GetChat::builder().chat_id(message.chat_id()).build())
        .await?;
      debug!("Receive new chat: {}", chat.to_json()?);
      Ok(())
    })
  });

  client
    .daemon("telegram-rs")
    .expect("failed to create telegram daemon");
}
