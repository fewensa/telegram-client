#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate slog;
extern crate slog_term;

use std::fs::File;
use std::path::Path;
use std::rc::Rc;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::time::Duration;

use rtdlib::types::RObject;

use telegram_client::api::*;
use telegram_client::client::Client;
use telegram_client::errors;
use telegram_client::types::*;

use crate::config::{Config, LogType};

mod exmlog;
mod thelp;
mod tgfn;
mod config;

fn main() {
  let api_id = env!("API_ID");
  let api_hash = env!("API_HASH");


  let log_file = toolkit::path::root_dir().join("tdlib.log");
  if log_file.exists() {
    std::fs::remove_file(&log_file);
  }
  File::create(&log_file).unwrap();

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
      api.set_tdlib_parameters(TGSetTdlibParameters::builder().parameters(
        TGTdlibParameters::builder()
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
          .build()
      ).build());
      debug!(exmlog::examples(), "Set tdlib parameters");
    });
    state.on_wait_encryption_key(|enck| {
      api.check_database_encryption_key(TGCheckDatabaseEncryptionKey::builder().build());
      debug!(exmlog::examples(), "Set encryption key");
    });
    state.on_wait_phone_number(|| {
      thelp::tip("Please type your telegram phone number:");
      tgfn::type_phone_number(api);
    });
    state.on_wait_password(|aswp| {
      api.check_authentication_password(TGCheckAuthenticationPassword::builder()
        .password(thelp::typed_with_message("Please type your telegram password:"))
        .build());
      debug!(exmlog::examples(), "Set password *****");
    });
    state.on_wait_code(|awc| {
      if awc.is_registered().clone().map_or(false, |v| v) {
        thelp::tip("Please type authentication code:");
        tgfn::type_authentication_code(api);
      } else {
        thelp::tip("Welcome to use telegram");
        thelp::tip("Your phone number is not registed to telegramm, please type your name. and authentication code.");
        tgfn::type_authentication_code_register(api);
      }
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
        }
        TGConnectionState::ConnectingToProxy => {
          debug!(exmlog::examples(), "connecting to proxy")
        }
        TGConnectionState::Connecting => {
          debug!(exmlog::examples(), "connecting")
        }
        TGConnectionState::Updating => {
          debug!(exmlog::examples(), "updating...")
        }
        TGConnectionState::Ready => {
          debug!(exmlog::examples(), "connection ready")
        }
      }
    });
  });

  listener.on_error(|(api, update)| {
    let code = update.code().clone().map_or(-1, |v| v);
    let message = update.message().clone().map_or("None".to_string(), |v| v);
    error!(exmlog::examples(), "ERROR [{}] {}", code, message);
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
    }
  });

  listener.on_ok(|api| {
    debug!(exmlog::examples(), "OK");
  });

  listener.on_proxy(|(api, update)| {
    debug!(exmlog::examples(), "Proxy info => {:?}", update);
  });

  listener.on_user(|(api, update)| {
    debug!(exmlog::examples(), "Update user => {:?}", update);
  });

  listener.on_have_pending_notifications(|(api, update)| {
    debug!(exmlog::examples(), "have pending notifications {:?}", update);
  });

  listener.on_scope_notification_settings(|(api, update)| {
    debug!(exmlog::examples(), "scope notification settings {:?}", update);
  });

  listener.on_user_status(|(api, update)| {
    debug!(exmlog::examples(), "User [{}] status is {:?}", update.user_id(), update.status());
  });

  listener.on_new_chat(|(api, update)| {
    let chat = update.chat();
    debug!(exmlog::examples(), "Receive new chat, title: '{}', data: {}", chat.title(), chat.to_json());
  });

  listener.on_new_message(|(api, update)| {
    let message = update.message();
    if message.is_outgoing() {
      return;
    }
    let content = message.content();
    content.on_text(|m| {
      // Send origin message
      api.send_message(TGSendMessage::builder()
        .chat_id(message.chat_id())
        .input_message_content(TGInputMessageContent::text(TGInputMessageText::builder()
          .text(TGFormattedText::builder().text(m.text().text()).build())
          .clear_draft(true)
          .build()))
        .build());
      debug!(exmlog::examples(), "Receive text message => {} <= entities => {:?}", m.text().text(), m.text().entities());
    });
    content.on_video(|m| {
      debug!(exmlog::examples(), "Receive video message");
      let v = m.video();
      let f = v.video();
      f.remote().map(|f| {

        let size = f.uploaded_size();
//        api.download_file(TGDownloadFile::new()
//          .file_id(f.id()))
        f.id().map(|v| {
          api.get_remote_file(TGGetRemoteFile::builder()
            .remote_file_id(v)
            .file_type(TGFileType::Video)
            .build())
        });
        debug!(exmlog::examples(), "video remote id => {:?}", f.id());
      });
      f.local().map(|f| {
        debug!(exmlog::examples(), "video local path => {:?}", f.path());
      });
    });
    debug!(exmlog::examples(), "Receive new message, from: '{}', data: {}", message.sender_user_id(), message.to_json());
  });

  listener.on_chat_read_inbox(|(api, update)| {
    debug!(exmlog::examples(), "Read inbox unread_count: {}, chat_id: {}, last_read_inbox_message_id: {}",
           update.unread_count().map_or(0, |v| v),
           update.chat_id().map_or(0, |v| v),
           update.last_read_inbox_message_id().map_or(0, |v| v),
    );
  });

  listener.on_chat_last_message(|(api, update)| {
    debug!(exmlog::examples(), "Chat last message: {}, data: {}", update.chat_id(), update.last_message().to_json())
  });

  listener.on_chat_read_outbox(|(api, update)| {
    debug!(exmlog::examples(), "Read outbox chat_id: {}, last_read_outbox_message_id: {}",
           update.chat_id().map_or(0, |v| v),
           update.last_read_outbox_message_id().map_or(0, |v| v),
    );
  });

  listener.on_user_full_info(|(api, update)| {
    debug!(exmlog::examples(), "Receive user full info, user_id: {}, full_info: {}",
           update.user_id().map_or(0, |v| v),
           update.user_full_info().expect(errors::TELEGRAM_DATA_FAIL).to_json()
    );
  });

  listener.on_delete_messages(|(api, update)| {
    debug!(exmlog::examples(), "Receive delete messages, chat_id: {}, message_ids: {:?}, data: {}",
           update.chat_id(),
           update.message_ids(),
           update.to_json()
    );
  });

  listener.on_file(|(api, update)| {
    debug!(exmlog::examples(), "Receive a file => {}", update.to_json());
    let size = update.size();
    if update.local().map_or(false, |v| v.is_downloading_completed()) {
      debug!(exmlog::examples(), "File {} download complete => {:?}", update.id(), update.local().map(|v| v.path()).expect("Can not get download path"));
      return;
    }
    api.download_file(TGDownloadFile::builder()
      .file_id(update.id())
      .offset(0)
      .limit(0)
      .priority(1)
      .synchronous(false)
      .build());
  });

  listener.on_update_file(|(api, update)| {
//    debug!(exmlog::examples(), "Update file => {}", update.file().to_json());
  });

  listener.on_message(|(api, update)| {
    debug!(exmlog::examples(), "Message => {}", update.to_json());
  });

  client.daemon("telegram-rs");
}

