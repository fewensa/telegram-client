#[macro_use]
extern crate log;

use simple_logger::SimpleLogger;

use rtdlib::types::*;
use telegram_client::api::*;
use telegram_client::client::Client;

use crate::helpers::{tgfn, thelp};
use crate::helpers::config::{Config, LogType};

mod helpers;


#[tokio::main]
async fn main() {
  SimpleLogger::new()
    .with_level(log::LevelFilter::Debug)
    .init()
    .unwrap();


  let api_id = env!("API_ID");
  let api_hash = env!("API_HASH");


  let config = Config::default();
  let api = Api::rasync();

  let mut client = Client::new(api.api().clone());
  let listener = client.listener().rasync_listener_mut();


  config.proxy().map(|v| {
    futures::executor::block_on(async move {
      api.clone().add_proxy(v).await.unwrap();
    });
  });

  config.log().map(|v| {
    Client::set_log_verbosity_level(v.level.clone() as i32).unwrap();
    if v.type_ == LogType::File {
      v.path.clone().map(|v| {
        Client::set_log_file_path(Some(&v[..]));
      });
    }
  });

  listener.on_receive(|(_api, object)| Box::pin(async move {
    println!("{}", object);
    Ok(())
  }));


  listener.on_update_option(|(_api, option)| Box::pin(async move {
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
  }));

  listener.on_update_authorization_state(move |(api, update)| Box::pin(async move {
    let rasync_api = api.rasync_api();
    debug!("-----> {:?}", update);
    let state = update.authorization_state();

    match state {
      AuthorizationState::WaitTdlibParameters(_) => {
        rasync_api.set_tdlib_parameters(SetTdlibParameters::builder().parameters(
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
        ).build()).await?;
        debug!("Set tdlib parameters");
      }
      AuthorizationState::WaitPhoneNumber(_) => {
        thelp::tip(format!("{} {}", "Please type your telegram phone number:", "(If you copy log to anywhere, don't forget hide your phone number)"));
        tgfn::type_phone_number(&api);
      }
      AuthorizationState::WaitCode(_) => {
        thelp::tip("Please type authentication code:");
        tgfn::type_authentication_code(&api);
      }
      AuthorizationState::WaitEncryptionKey(_) => {
        rasync_api.check_database_encryption_key(CheckDatabaseEncryptionKey::builder().build()).await?;
        debug!("Set encryption key");
      }
      _ => {}
    }
    Ok(())
  }));

  listener.on_update_new_message(|(api, update)| Box::pin(async move {
    let rasync_api = api.rasync_api();
    let message = update.message();
    let chat = rasync_api.get_chat(GetChat::builder().chat_id(message.chat_id()).build()).await?;
    debug!("Receive new chat: {}", chat.to_json()?);
    Ok(())
  }));

  client.daemon("telegram-rs").expect("failed to create telegram daemon");
}
