#[macro_use]
extern crate log;

use std::sync::{Arc, Mutex};

use rtdlib::types::*;

use telegram_client::api::*;
use telegram_client::client::Client;

use crate::config::{Config, LogType};

mod thelp;
mod tgfn;
mod config;

fn main() {
  simple_logger::init().unwrap();
  log::set_max_level(log::LevelFilter::Debug);


  let api_id = env!("API_ID");
  let api_hash = env!("API_HASH");

  let config = Config::default();
  let api = Api::default();
  let mut client = Client::new(api.clone());

  config.proxy().map(|v| { api.add_proxy(v) });

  config.log().map(|v| {
    Client::set_log_verbosity_level(v.level.clone() as i32).unwrap();
    if v.type_ == LogType::File {
      v.path.clone().map(|v| {
        Client::set_log_file_path(Some(&v[..]));
      });
    }
  });


  let listener = client.listener();

  let have_authorization: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

//  listener.on_receive(|(api, object)| {
//    println!("{:?}", object);
//  });

  listener.on_option(|(api, option)| {
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

  listener.on_authorization_state(move |(api, update)| {
    let state = update.authorization_state();
    state.on_wait_tdlib_parameters(|_| {
      api.set_tdlib_parameters(SetTdlibParameters::builder().parameters(
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
      ).build());
      debug!("Set tdlib parameters");
    });
    state.on_wait_encryption_key(|_| {
      api.check_database_encryption_key(CheckDatabaseEncryptionKey::builder().build());
      debug!("Set encryption key");
    });
    state.on_wait_phone_number(|_| {
      thelp::tip("Please type your telegram phone number:");
      tgfn::type_phone_number(api);
    });
    state.on_wait_password(|_| {
      api.check_authentication_password(CheckAuthenticationPassword::builder()
        .password(thelp::typed_with_message("Please type your telegram password:"))
        .build());
      debug!("Set password *****");
    });
    state.on_wait_code(|astat| {
      if astat.is_registered() {
        thelp::tip("Please type authentication code:");
        tgfn::type_authentication_code(api);
      } else {
        thelp::tip("Welcome to use telegram");
        thelp::tip("Your phone number is not registed to telegramm, please type your name. and authentication code.");
        tgfn::type_authentication_code_register(api);
      }
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

  listener.on_connection_state(|(api, update)| {
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
            thelp::tip("Phone number invalid, please type a right phone number for telegram");
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

  listener.on_ok(|api| {
    debug!("OK");
    Ok(())
  });

  listener.on_proxy(|(api, update)| {
    debug!("Proxy info => {:?}", update);
    Ok(())
  });

  listener.on_user(|(api, update)| {
    debug!("Update user => {:?}", update);
    Ok(())
  });

  listener.on_have_pending_notifications(|(api, update)| {
    debug!("have pending notifications {:?}", update);
    Ok(())
  });

  listener.on_scope_notification_settings(|(api, update)| {
    debug!("scope notification settings {:?}", update);
    Ok(())
  });

  listener.on_user_status(|(api, update)| {
    debug!("User [{}] status is {:?}", update.user_id(), update.status());
    Ok(())
  });

  listener.on_new_chat(|(api, update)| {
    let chat = update.chat();
    debug!("Receive new chat, title: '{}', data: {}", chat.title(), chat.to_json().expect("Can't serialize json"));
    Ok(())
  });

  listener.on_new_message(|(api, update)| {
    let message = update.message();
    if message.is_outgoing() {
      return Ok(());
    }
    let content = message.content();
    content.on_message_text(|m| {
      if message.chat_id() != 102993 {
        return;
      }
      // Send origin message
      api.send_message(SendMessage::builder()
        .chat_id(message.chat_id())
        .input_message_content(InputMessageContent::input_message_text(InputMessageText::builder()
          .text(FormattedText::builder().text(m.text().text()).build())
          .clear_draft(true)
          .build()))
        .build());
      debug!("Receive text message => {} <= entities => {:?}", m.text().text(), m.text().entities());
    });
    content.on_message_video(|m| {
      debug!("Receive video message");
      let v = m.video();
      let f = v.video();
      let remote_file = f.remote();

      let size = remote_file.uploaded_size();

      api.get_remote_file(GetRemoteFile::builder()
        .remote_file_id(remote_file.id())
        .file_type(FileType::video(FileTypeVideo::builder()))
        .build());
      debug!("video remote id => {:?}", remote_file.id());

      debug!("video local path => {:?}", f.local().path());
    });
    debug!("Receive new message, from: '{}', data: {}", message.sender_user_id(), message.to_json().expect("Can't serialize json"));
    Ok(())
  });

  listener.on_chat_read_inbox(|(api, update)| {
    debug!("Read inbox unread_count: {}, chat_id: {}, last_read_inbox_message_id: {}",
           update.unread_count(),
           update.chat_id(),
           update.last_read_inbox_message_id(),
    );
    Ok(())
  });

  listener.on_chat_last_message(|(api, update)| {
    debug!("Chat last message: {}, data: {}",
           update.chat_id(),
           update.last_message().clone().map_or("None".to_string(), |v| v.to_json().expect("Can't serialize json"))
    );
    Ok(())
  });

  listener.on_chat_read_outbox(|(api, update)| {
    debug!("Read outbox chat_id: {}, last_read_outbox_message_id: {}",
           update.chat_id(),
           update.last_read_outbox_message_id(),
    );
    Ok(())
  });

  listener.on_user_full_info(|(api, update)| {
    debug!("Receive user full info, user_id: {}, full_info: {}",
           update.user_id(),
           update.user_full_info().to_json().expect("Can't serialize json")
    );
    Ok(())
  });

  listener.on_delete_messages(|(api, update)| {
    debug!("Receive delete messages, chat_id: {}, message_ids: {:?}, data: {}",
           update.chat_id(),
           update.message_ids(),
           update.to_json().expect("Can't serialize json")
    );
    Ok(())
  });

  listener.on_file(|(api, update)| {
    debug!("Receive a file => {}", update.to_json().expect("Can't serialize json"));
    let file = update.file();
    let size = file.size();
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
      .build());
    Ok(())
  });

//  listener.on_update_file(|(api, update)| {
////    debug!("Update file => {}", update.file().to_json());
//  });

//  listener.on_message(|(api, update)| {
//    debug!("Message => {}", update.to_json());
//  });

  listener.on_supergroup_full_info(|(api, update)| {
    debug!("Supergroup full info => {}", update.to_json().expect("Can't serialize json"));
    Ok(())
  });

  listener.on_user_chat_action(|(api, update)| {
    debug!("User chat action => {}", update.to_json().expect("Can't serialize json"));
    Ok(())
  });

  listener.on_terms_of_service(|(api, update)| {
    debug!("Terms of serivce => {}", update.to_json().expect("Can't serialize json"));
    Ok(())
  });

  client.daemon("telegram-rs");
}

