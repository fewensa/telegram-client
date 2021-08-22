#[macro_use]
extern crate log;

use std::sync::{Arc, Mutex};

use colored::Colorize;
use simple_logger::SimpleLogger;

use rtdlib::types::*;
use telegram_client::api::*;
use telegram_client::client::Client;

use crate::helpers::{tgfn, thelp};
use crate::helpers::config::{Config, LogType};

mod helpers;


fn main() {
  SimpleLogger::new()
    .with_level(log::LevelFilter::Debug)
    .init()
    .unwrap();


  let api_id = env!("API_ID");
  let api_hash = env!("API_HASH");

  let config = Config::default();
  debug!("{:#?}", config);
  let api = Api::event();
  let mut client = Client::new(api.api().clone());

  config.proxy().map(|v| { api.add_proxy(v) });

  config.log().map(|v| {
    Client::set_log_verbosity_level(v.level.clone() as i32).unwrap();
    if v.type_ == LogType::File {
      v.path.clone().map(|v| {
        Client::set_log_file_path(Some(&v[..]));
      });
    }
  });


  let listener = client.listener().event_listener_mut();

  let have_authorization: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

  listener.on_receive(|(_api, object)| {
    println!("{}", object);
    Ok(())
  });

  listener.on_update_option(|(_api, option)| {
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
    let event_api = api.event_api();
    let state = update.authorization_state();
    state.on_wait_tdlib_parameters(|_| {
      event_api.set_tdlib_parameters(SetTdlibParameters::builder().parameters(
        TdlibParameters::builder()
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
          .build()
      ).build())
        .unwrap();
      debug!("Set tdlib parameters");
    });
    state.on_wait_encryption_key(|_| {
      event_api.check_database_encryption_key(CheckDatabaseEncryptionKey::builder().build()).unwrap();
      debug!("Set encryption key");
    });
    state.on_wait_phone_number(|_| {
      thelp::tip(format!("{} {}", "Please type your telegram phone number:", "(If you copy log to anywhere, don't forget hide your phone number)".red()));
      tgfn::type_phone_number(&api);
    });
    state.on_wait_password(|_| {
      event_api.check_authentication_password(CheckAuthenticationPassword::builder()
        .password(thelp::typed_with_message(format!("{} {}", "Please type your telegram password:", "(If you copy log to anywhere, don't forget hide your password)".red())))
        .build())
        .unwrap();
      debug!("Set password *****");
    });
    state.on_wait_registration(|_| {
      thelp::tip("Welcome to use telegram");
      thelp::tip("Your phone number is not registered to telegram, please type your name. and register.");
      tgfn::type_and_register(&api);
    });
    state.on_wait_code(|_astat| {
      thelp::tip("Please type authentication code:");
      tgfn::type_authentication_code(&api);
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

  listener.on_update_connection_state(|(_api, update)| {
    let state = update.state();
    state.on_waiting_for_network(|_| { debug!("waiting for network"); });
    state.on_connecting_to_proxy(|_| { debug!("connecting to proxy"); });
    state.on_connecting(|_| { debug!("connecting"); });
    state.on_updating(|_| { debug!("updating..."); });
    state.on_ready(|_| { debug!("connection ready") });
    Ok(())
  });

  listener.on_error(|(api, update)| {
    let code = update.code();
    let message = update.message();
    error!("ERROR [{}] {}", code, message);
    match code {
      8 => {
        thelp::tip(&message);
        thelp::tip("Please type telegram phone number");
        tgfn::type_phone_number(api);
      }
      400 => {
        match &message[..] {
          "PHONE_NUMBER_INVALID" => {
            thelp::tip(format!("{} {}", "Phone number invalid, please type a right phone number for telegram", "(If you copy log to anywhere, don't forget hide your phone number)".red()));
            tgfn::type_phone_number(api);
          }
          "PHONE_CODE_INVALID" | "PHONE_CODE_EMPTY" => {
            thelp::tip("Phone code invalid, please type an authentication code");
            tgfn::type_authentication_code(api);
          }
          _ => {}
        }
      }
      429 => thelp::wait_too_many_requests(api, &message),
      _ => thelp::unknown(code, &message)
    };
    Ok(())
  });

  listener.on_ok(|_api| {
    debug!("OK");
    Ok(())
  });

  listener.on_proxy(|(_api, update)| {
    debug!("Proxy info => {:?}", update);
    Ok(())
  });

  listener.on_update_user(|(_api, update)| {
    debug!("Update user => {:?}", update);
    Ok(())
  });

  listener.on_update_have_pending_notifications(|(_api, update)| {
    debug!("have pending notifications {:?}", update);
    Ok(())
  });

  listener.on_update_scope_notification_settings(|(_api, update)| {
    debug!("scope notification settings {:?}", update);
    Ok(())
  });

  listener.on_update_user_status(|(_api, update)| {
    debug!("User [{}] status is {:?}", update.user_id(), update.status());
    Ok(())
  });

  listener.on_update_new_chat(|(_api, update)| {
    let chat = update.chat();
    debug!("Receive new chat, title: '{}', data: {}", chat.title(), chat.to_json().expect("Can't serialize json"));
    Ok(())
  });

  listener.on_update_new_message(|(api, update)| {
    let message = update.message();
    if message.is_outgoing() {
      return Ok(());
    }
    let api = api.event_api();
    let chat_id = message.chat_id();
    api.get_chat(GetChat::builder().chat_id(chat_id).build())?;
    debug!("Receive new message, from: '{:?}', data: {}", message.sender_user_id(), message.to_json().expect("Can't serialize json"));
    Ok(())
  });

  listener.on_update_user_full_info(|(_api, update)| {
    debug!("Receive user full info, user_id: {}, full_info: {}",
      update.user_id(),
      update.user_full_info().to_json().expect("Can't serialize json")
    );
    Ok(())
  });

  listener.on_update_delete_messages(|(_api, update)| {
    debug!("Receive delete messages, chat_id: {}, message_ids: {:?}, data: {}",
      update.chat_id(),
      update.message_ids(),
      update.to_json().expect("Can't serialize json")
    );
    Ok(())
  });

  listener.on_update_file(|(api, update)| {
    let api = api.event_api();
    debug!("Receive a file => {}", update.to_json().expect("Can't serialize json"));
    let file = update.file();
    let _size = file.size();
    let local_file = file.local();
    if local_file.is_downloading_completed() {
      debug!("File {} download complete => {:?}", file.id(), local_file.path());
      return Ok(());
    }
    api.download_file(DownloadFile::builder()
      .file_id(file.id())
      .offset(0)
      .limit(0)
      .priority(1)
      .synchronous(false)
      .build())
      .unwrap();
    Ok(())
  });

  listener.on_update_supergroup_full_info(|(_api, update)| {
    debug!("Supergroup full info => {}", update.to_json().unwrap());
    Ok(())
  });

  listener.on_update_user_chat_action(|(_api, update)| {
    debug!("User chat action => {}", update.to_json().unwrap());
    Ok(())
  });

  listener.on_update_terms_of_service(|(_api, update)| {
    debug!("Terms of serivce => {}", update.to_json().unwrap());
    Ok(())
  });

  client.daemon("telegram-rs").expect("failed to create telegram daemon");
}

