use core::borrow::Borrow;
use std::sync::Arc;

use rtdlib::tdjson;
use rtdlib::errors::*;
use rtdlib::types::*;

use crate::tip;


#[derive(Debug, Clone)]
pub struct Api {
  tdlib: Arc<tdjson::Tdlib>
}

impl Default for Api {
  fn default() -> Self {
    Api::new(tdjson::Tdlib::new())
  }
}

impl Api {
  pub fn new(tdlib: tdjson::Tdlib) -> Self {
    Self { tdlib: Arc::new(tdlib) }
  }

  #[doc(hidden)]
  pub fn tdlib(&self) -> &tdjson::Tdlib {
    self.tdlib.borrow()
  }

  pub fn send<Fnc: RFunction>(&self, fnc: Fnc) -> RTDResult<()> {
    let json = fnc.to_json()?;
    info!("===> {}", json);
    self.tdlib.send(&json[..]);
    Ok(())
  }

  pub fn receive(&self, timeout: f64) -> Option<String> {
    let receive = self.tdlib.receive(timeout);
    if receive.is_some() {
      info!("<=== {}", receive.clone().map_or("NONE".to_string(), |v| v));
    }
    receive
  }

  pub fn execute<Fnc: RFunction>(&self, fnc: Fnc) -> RTDResult<Option<String>> {
    let json = fnc.to_json()?;
    info!("===>>> {}", json);
    Ok(self.tdlib.execute(&json[..]))
  }


  pub fn get_authorization_state<C: AsRef<GetAuthorizationState>>(&self, get_authorization_state: C) -> RTDResult<AuthorizationState> {
  
    match self.execute(get_authorization_state.as_ref())? {
      Some(json) => Ok(AuthorizationState::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn set_tdlib_parameters<C: AsRef<SetTdlibParameters>>(&self, set_tdlib_parameters: C) -> RTDResult<()> {
    self.send(set_tdlib_parameters.as_ref()) 
  }

  pub fn check_database_encryption_key<C: AsRef<CheckDatabaseEncryptionKey>>(&self, check_database_encryption_key: C) -> RTDResult<()> {
    self.send(check_database_encryption_key.as_ref()) 
  }

  pub fn set_authentication_phone_number<C: AsRef<SetAuthenticationPhoneNumber>>(&self, set_authentication_phone_number: C) -> RTDResult<()> {
    self.send(set_authentication_phone_number.as_ref()) 
  }

  pub fn resend_authentication_code<C: AsRef<ResendAuthenticationCode>>(&self, resend_authentication_code: C) -> RTDResult<()> {
    self.send(resend_authentication_code.as_ref()) 
  }

  pub fn check_authentication_code<C: AsRef<CheckAuthenticationCode>>(&self, check_authentication_code: C) -> RTDResult<()> {
    self.send(check_authentication_code.as_ref()) 
  }

  pub fn check_authentication_password<C: AsRef<CheckAuthenticationPassword>>(&self, check_authentication_password: C) -> RTDResult<()> {
    self.send(check_authentication_password.as_ref()) 
  }

  pub fn request_authentication_password_recovery<C: AsRef<RequestAuthenticationPasswordRecovery>>(&self, request_authentication_password_recovery: C) -> RTDResult<()> {
    self.send(request_authentication_password_recovery.as_ref()) 
  }

  pub fn recover_authentication_password<C: AsRef<RecoverAuthenticationPassword>>(&self, recover_authentication_password: C) -> RTDResult<()> {
    self.send(recover_authentication_password.as_ref()) 
  }

  pub fn check_authentication_bot_token<C: AsRef<CheckAuthenticationBotToken>>(&self, check_authentication_bot_token: C) -> RTDResult<()> {
    self.send(check_authentication_bot_token.as_ref()) 
  }

  pub fn log_out<C: AsRef<LogOut>>(&self, log_out: C) -> RTDResult<()> {
    self.send(log_out.as_ref()) 
  }

  pub fn close<C: AsRef<Close>>(&self, close: C) -> RTDResult<()> {
    self.send(close.as_ref()) 
  }

  pub fn destroy<C: AsRef<Destroy>>(&self, destroy: C) -> RTDResult<()> {
    self.send(destroy.as_ref()) 
  }

  pub fn set_database_encryption_key<C: AsRef<SetDatabaseEncryptionKey>>(&self, set_database_encryption_key: C) -> RTDResult<()> {
    self.send(set_database_encryption_key.as_ref()) 
  }

  pub fn get_password_state<C: AsRef<GetPasswordState>>(&self, get_password_state: C) -> RTDResult<PasswordState> {
  
    match self.execute(get_password_state.as_ref())? {
      Some(json) => Ok(PasswordState::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn set_password<C: AsRef<SetPassword>>(&self, set_password: C) -> RTDResult<PasswordState> {
  
    match self.execute(set_password.as_ref())? {
      Some(json) => Ok(PasswordState::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_recovery_email_address<C: AsRef<GetRecoveryEmailAddress>>(&self, get_recovery_email_address: C) -> RTDResult<RecoveryEmailAddress> {
  
    match self.execute(get_recovery_email_address.as_ref())? {
      Some(json) => Ok(RecoveryEmailAddress::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn set_recovery_email_address<C: AsRef<SetRecoveryEmailAddress>>(&self, set_recovery_email_address: C) -> RTDResult<PasswordState> {
  
    match self.execute(set_recovery_email_address.as_ref())? {
      Some(json) => Ok(PasswordState::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn request_password_recovery<C: AsRef<RequestPasswordRecovery>>(&self, request_password_recovery: C) -> RTDResult<EmailAddressAuthenticationCodeInfo> {
  
    match self.execute(request_password_recovery.as_ref())? {
      Some(json) => Ok(EmailAddressAuthenticationCodeInfo::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn recover_password<C: AsRef<RecoverPassword>>(&self, recover_password: C) -> RTDResult<PasswordState> {
  
    match self.execute(recover_password.as_ref())? {
      Some(json) => Ok(PasswordState::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn create_temporary_password<C: AsRef<CreateTemporaryPassword>>(&self, create_temporary_password: C) -> RTDResult<TemporaryPasswordState> {
  
    match self.execute(create_temporary_password.as_ref())? {
      Some(json) => Ok(TemporaryPasswordState::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_temporary_password_state<C: AsRef<GetTemporaryPasswordState>>(&self, get_temporary_password_state: C) -> RTDResult<TemporaryPasswordState> {
  
    match self.execute(get_temporary_password_state.as_ref())? {
      Some(json) => Ok(TemporaryPasswordState::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn process_dc_update<C: AsRef<ProcessDcUpdate>>(&self, process_dc_update: C) -> RTDResult<()> {
    self.send(process_dc_update.as_ref()) 
  }

  pub fn get_me<C: AsRef<GetMe>>(&self, get_me: C) -> RTDResult<User> {
  
    match self.execute(get_me.as_ref())? {
      Some(json) => Ok(User::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_user<C: AsRef<GetUser>>(&self, get_user: C) -> RTDResult<User> {
  
    match self.execute(get_user.as_ref())? {
      Some(json) => Ok(User::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_user_full_info<C: AsRef<GetUserFullInfo>>(&self, get_user_full_info: C) -> RTDResult<UserFullInfo> {
  
    match self.execute(get_user_full_info.as_ref())? {
      Some(json) => Ok(UserFullInfo::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_basic_group<C: AsRef<GetBasicGroup>>(&self, get_basic_group: C) -> RTDResult<BasicGroup> {
  
    match self.execute(get_basic_group.as_ref())? {
      Some(json) => Ok(BasicGroup::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_basic_group_full_info<C: AsRef<GetBasicGroupFullInfo>>(&self, get_basic_group_full_info: C) -> RTDResult<BasicGroupFullInfo> {
  
    match self.execute(get_basic_group_full_info.as_ref())? {
      Some(json) => Ok(BasicGroupFullInfo::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_supergroup<C: AsRef<GetSupergroup>>(&self, get_supergroup: C) -> RTDResult<Supergroup> {
  
    match self.execute(get_supergroup.as_ref())? {
      Some(json) => Ok(Supergroup::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_supergroup_full_info<C: AsRef<GetSupergroupFullInfo>>(&self, get_supergroup_full_info: C) -> RTDResult<SupergroupFullInfo> {
  
    match self.execute(get_supergroup_full_info.as_ref())? {
      Some(json) => Ok(SupergroupFullInfo::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_secret_chat<C: AsRef<GetSecretChat>>(&self, get_secret_chat: C) -> RTDResult<SecretChat> {
  
    match self.execute(get_secret_chat.as_ref())? {
      Some(json) => Ok(SecretChat::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_chat<C: AsRef<GetChat>>(&self, get_chat: C) -> RTDResult<Chat> {
  
    match self.execute(get_chat.as_ref())? {
      Some(json) => Ok(Chat::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_message<C: AsRef<GetMessage>>(&self, get_message: C) -> RTDResult<Message> {
  
    match self.execute(get_message.as_ref())? {
      Some(json) => Ok(Message::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_replied_message<C: AsRef<GetRepliedMessage>>(&self, get_replied_message: C) -> RTDResult<Message> {
  
    match self.execute(get_replied_message.as_ref())? {
      Some(json) => Ok(Message::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_chat_pinned_message<C: AsRef<GetChatPinnedMessage>>(&self, get_chat_pinned_message: C) -> RTDResult<Message> {
  
    match self.execute(get_chat_pinned_message.as_ref())? {
      Some(json) => Ok(Message::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_messages<C: AsRef<GetMessages>>(&self, get_messages: C) -> RTDResult<Messages> {
  
    match self.execute(get_messages.as_ref())? {
      Some(json) => Ok(Messages::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_file<C: AsRef<GetFile>>(&self, get_file: C) -> RTDResult<File> {
  
    match self.execute(get_file.as_ref())? {
      Some(json) => Ok(File::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_remote_file<C: AsRef<GetRemoteFile>>(&self, get_remote_file: C) -> RTDResult<File> {
  
    match self.execute(get_remote_file.as_ref())? {
      Some(json) => Ok(File::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_chats<C: AsRef<GetChats>>(&self, get_chats: C) -> RTDResult<Chats> {
  
    match self.execute(get_chats.as_ref())? {
      Some(json) => Ok(Chats::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn search_public_chat<C: AsRef<SearchPublicChat>>(&self, search_public_chat: C) -> RTDResult<Chat> {
  
    match self.execute(search_public_chat.as_ref())? {
      Some(json) => Ok(Chat::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn search_public_chats<C: AsRef<SearchPublicChats>>(&self, search_public_chats: C) -> RTDResult<Chats> {
  
    match self.execute(search_public_chats.as_ref())? {
      Some(json) => Ok(Chats::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn search_chats<C: AsRef<SearchChats>>(&self, search_chats: C) -> RTDResult<Chats> {
  
    match self.execute(search_chats.as_ref())? {
      Some(json) => Ok(Chats::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn search_chats_on_server<C: AsRef<SearchChatsOnServer>>(&self, search_chats_on_server: C) -> RTDResult<Chats> {
  
    match self.execute(search_chats_on_server.as_ref())? {
      Some(json) => Ok(Chats::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_top_chats<C: AsRef<GetTopChats>>(&self, get_top_chats: C) -> RTDResult<Chats> {
  
    match self.execute(get_top_chats.as_ref())? {
      Some(json) => Ok(Chats::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn remove_top_chat<C: AsRef<RemoveTopChat>>(&self, remove_top_chat: C) -> RTDResult<()> {
    self.send(remove_top_chat.as_ref()) 
  }

  pub fn add_recently_found_chat<C: AsRef<AddRecentlyFoundChat>>(&self, add_recently_found_chat: C) -> RTDResult<()> {
    self.send(add_recently_found_chat.as_ref()) 
  }

  pub fn remove_recently_found_chat<C: AsRef<RemoveRecentlyFoundChat>>(&self, remove_recently_found_chat: C) -> RTDResult<()> {
    self.send(remove_recently_found_chat.as_ref()) 
  }

  pub fn clear_recently_found_chats<C: AsRef<ClearRecentlyFoundChats>>(&self, clear_recently_found_chats: C) -> RTDResult<()> {
    self.send(clear_recently_found_chats.as_ref()) 
  }

  pub fn check_chat_username<C: AsRef<CheckChatUsername>>(&self, check_chat_username: C) -> RTDResult<CheckChatUsernameResult> {
  
    match self.execute(check_chat_username.as_ref())? {
      Some(json) => Ok(CheckChatUsernameResult::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_created_public_chats<C: AsRef<GetCreatedPublicChats>>(&self, get_created_public_chats: C) -> RTDResult<Chats> {
  
    match self.execute(get_created_public_chats.as_ref())? {
      Some(json) => Ok(Chats::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_groups_in_common<C: AsRef<GetGroupsInCommon>>(&self, get_groups_in_common: C) -> RTDResult<Chats> {
  
    match self.execute(get_groups_in_common.as_ref())? {
      Some(json) => Ok(Chats::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_chat_history<C: AsRef<GetChatHistory>>(&self, get_chat_history: C) -> RTDResult<Messages> {
  
    match self.execute(get_chat_history.as_ref())? {
      Some(json) => Ok(Messages::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn delete_chat_history<C: AsRef<DeleteChatHistory>>(&self, delete_chat_history: C) -> RTDResult<()> {
    self.send(delete_chat_history.as_ref()) 
  }

  pub fn search_chat_messages<C: AsRef<SearchChatMessages>>(&self, search_chat_messages: C) -> RTDResult<Messages> {
  
    match self.execute(search_chat_messages.as_ref())? {
      Some(json) => Ok(Messages::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn search_messages<C: AsRef<SearchMessages>>(&self, search_messages: C) -> RTDResult<Messages> {
  
    match self.execute(search_messages.as_ref())? {
      Some(json) => Ok(Messages::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn search_secret_messages<C: AsRef<SearchSecretMessages>>(&self, search_secret_messages: C) -> RTDResult<FoundMessages> {
  
    match self.execute(search_secret_messages.as_ref())? {
      Some(json) => Ok(FoundMessages::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn search_call_messages<C: AsRef<SearchCallMessages>>(&self, search_call_messages: C) -> RTDResult<Messages> {
  
    match self.execute(search_call_messages.as_ref())? {
      Some(json) => Ok(Messages::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn search_chat_recent_location_messages<C: AsRef<SearchChatRecentLocationMessages>>(&self, search_chat_recent_location_messages: C) -> RTDResult<Messages> {
  
    match self.execute(search_chat_recent_location_messages.as_ref())? {
      Some(json) => Ok(Messages::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_active_live_location_messages<C: AsRef<GetActiveLiveLocationMessages>>(&self, get_active_live_location_messages: C) -> RTDResult<Messages> {
  
    match self.execute(get_active_live_location_messages.as_ref())? {
      Some(json) => Ok(Messages::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_chat_message_by_date<C: AsRef<GetChatMessageByDate>>(&self, get_chat_message_by_date: C) -> RTDResult<Message> {
  
    match self.execute(get_chat_message_by_date.as_ref())? {
      Some(json) => Ok(Message::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_chat_message_count<C: AsRef<GetChatMessageCount>>(&self, get_chat_message_count: C) -> RTDResult<Count> {
  
    match self.execute(get_chat_message_count.as_ref())? {
      Some(json) => Ok(Count::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_public_message_link<C: AsRef<GetPublicMessageLink>>(&self, get_public_message_link: C) -> RTDResult<PublicMessageLink> {
  
    match self.execute(get_public_message_link.as_ref())? {
      Some(json) => Ok(PublicMessageLink::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn send_message<C: AsRef<SendMessage>>(&self, send_message: C) -> RTDResult<Message> {
  
    match self.execute(send_message.as_ref())? {
      Some(json) => Ok(Message::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn send_message_album<C: AsRef<SendMessageAlbum>>(&self, send_message_album: C) -> RTDResult<Messages> {
  
    match self.execute(send_message_album.as_ref())? {
      Some(json) => Ok(Messages::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn send_bot_start_message<C: AsRef<SendBotStartMessage>>(&self, send_bot_start_message: C) -> RTDResult<Message> {
  
    match self.execute(send_bot_start_message.as_ref())? {
      Some(json) => Ok(Message::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn send_inline_query_result_message<C: AsRef<SendInlineQueryResultMessage>>(&self, send_inline_query_result_message: C) -> RTDResult<Message> {
  
    match self.execute(send_inline_query_result_message.as_ref())? {
      Some(json) => Ok(Message::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn forward_messages<C: AsRef<ForwardMessages>>(&self, forward_messages: C) -> RTDResult<Messages> {
  
    match self.execute(forward_messages.as_ref())? {
      Some(json) => Ok(Messages::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn send_chat_set_ttl_message<C: AsRef<SendChatSetTtlMessage>>(&self, send_chat_set_ttl_message: C) -> RTDResult<Message> {
  
    match self.execute(send_chat_set_ttl_message.as_ref())? {
      Some(json) => Ok(Message::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn send_chat_screenshot_taken_notification<C: AsRef<SendChatScreenshotTakenNotification>>(&self, send_chat_screenshot_taken_notification: C) -> RTDResult<()> {
    self.send(send_chat_screenshot_taken_notification.as_ref()) 
  }

  pub fn add_local_message<C: AsRef<AddLocalMessage>>(&self, add_local_message: C) -> RTDResult<Message> {
  
    match self.execute(add_local_message.as_ref())? {
      Some(json) => Ok(Message::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn delete_messages<C: AsRef<DeleteMessages>>(&self, delete_messages: C) -> RTDResult<()> {
    self.send(delete_messages.as_ref()) 
  }

  pub fn delete_chat_messages_from_user<C: AsRef<DeleteChatMessagesFromUser>>(&self, delete_chat_messages_from_user: C) -> RTDResult<()> {
    self.send(delete_chat_messages_from_user.as_ref()) 
  }

  pub fn edit_message_text<C: AsRef<EditMessageText>>(&self, edit_message_text: C) -> RTDResult<Message> {
  
    match self.execute(edit_message_text.as_ref())? {
      Some(json) => Ok(Message::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn edit_message_live_location<C: AsRef<EditMessageLiveLocation>>(&self, edit_message_live_location: C) -> RTDResult<Message> {
  
    match self.execute(edit_message_live_location.as_ref())? {
      Some(json) => Ok(Message::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn edit_message_media<C: AsRef<EditMessageMedia>>(&self, edit_message_media: C) -> RTDResult<Message> {
  
    match self.execute(edit_message_media.as_ref())? {
      Some(json) => Ok(Message::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn edit_message_caption<C: AsRef<EditMessageCaption>>(&self, edit_message_caption: C) -> RTDResult<Message> {
  
    match self.execute(edit_message_caption.as_ref())? {
      Some(json) => Ok(Message::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn edit_message_reply_markup<C: AsRef<EditMessageReplyMarkup>>(&self, edit_message_reply_markup: C) -> RTDResult<Message> {
  
    match self.execute(edit_message_reply_markup.as_ref())? {
      Some(json) => Ok(Message::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn edit_inline_message_text<C: AsRef<EditInlineMessageText>>(&self, edit_inline_message_text: C) -> RTDResult<()> {
    self.send(edit_inline_message_text.as_ref()) 
  }

  pub fn edit_inline_message_live_location<C: AsRef<EditInlineMessageLiveLocation>>(&self, edit_inline_message_live_location: C) -> RTDResult<()> {
    self.send(edit_inline_message_live_location.as_ref()) 
  }

  pub fn edit_inline_message_media<C: AsRef<EditInlineMessageMedia>>(&self, edit_inline_message_media: C) -> RTDResult<()> {
    self.send(edit_inline_message_media.as_ref()) 
  }

  pub fn edit_inline_message_caption<C: AsRef<EditInlineMessageCaption>>(&self, edit_inline_message_caption: C) -> RTDResult<()> {
    self.send(edit_inline_message_caption.as_ref()) 
  }

  pub fn edit_inline_message_reply_markup<C: AsRef<EditInlineMessageReplyMarkup>>(&self, edit_inline_message_reply_markup: C) -> RTDResult<()> {
    self.send(edit_inline_message_reply_markup.as_ref()) 
  }

  pub fn get_text_entities<C: AsRef<GetTextEntities>>(&self, get_text_entities: C) -> RTDResult<TextEntities> {
  
    match self.execute(get_text_entities.as_ref())? {
      Some(json) => Ok(TextEntities::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn parse_text_entities<C: AsRef<ParseTextEntities>>(&self, parse_text_entities: C) -> RTDResult<FormattedText> {
  
    match self.execute(parse_text_entities.as_ref())? {
      Some(json) => Ok(FormattedText::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_file_mime_type<C: AsRef<GetFileMimeType>>(&self, get_file_mime_type: C) -> RTDResult<Text> {
  
    match self.execute(get_file_mime_type.as_ref())? {
      Some(json) => Ok(Text::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_file_extension<C: AsRef<GetFileExtension>>(&self, get_file_extension: C) -> RTDResult<Text> {
  
    match self.execute(get_file_extension.as_ref())? {
      Some(json) => Ok(Text::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn clean_file_name<C: AsRef<CleanFileName>>(&self, clean_file_name: C) -> RTDResult<Text> {
  
    match self.execute(clean_file_name.as_ref())? {
      Some(json) => Ok(Text::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_language_pack_string<C: AsRef<GetLanguagePackString>>(&self, get_language_pack_string: C) -> RTDResult<LanguagePackStringValue> {
  
    match self.execute(get_language_pack_string.as_ref())? {
      Some(json) => Ok(LanguagePackStringValue::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_inline_query_results<C: AsRef<GetInlineQueryResults>>(&self, get_inline_query_results: C) -> RTDResult<InlineQueryResults> {
  
    match self.execute(get_inline_query_results.as_ref())? {
      Some(json) => Ok(InlineQueryResults::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn answer_inline_query<C: AsRef<AnswerInlineQuery>>(&self, answer_inline_query: C) -> RTDResult<()> {
    self.send(answer_inline_query.as_ref()) 
  }

  pub fn get_callback_query_answer<C: AsRef<GetCallbackQueryAnswer>>(&self, get_callback_query_answer: C) -> RTDResult<CallbackQueryAnswer> {
  
    match self.execute(get_callback_query_answer.as_ref())? {
      Some(json) => Ok(CallbackQueryAnswer::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn answer_callback_query<C: AsRef<AnswerCallbackQuery>>(&self, answer_callback_query: C) -> RTDResult<()> {
    self.send(answer_callback_query.as_ref()) 
  }

  pub fn answer_shipping_query<C: AsRef<AnswerShippingQuery>>(&self, answer_shipping_query: C) -> RTDResult<()> {
    self.send(answer_shipping_query.as_ref()) 
  }

  pub fn answer_pre_checkout_query<C: AsRef<AnswerPreCheckoutQuery>>(&self, answer_pre_checkout_query: C) -> RTDResult<()> {
    self.send(answer_pre_checkout_query.as_ref()) 
  }

  pub fn set_game_score<C: AsRef<SetGameScore>>(&self, set_game_score: C) -> RTDResult<Message> {
  
    match self.execute(set_game_score.as_ref())? {
      Some(json) => Ok(Message::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn set_inline_game_score<C: AsRef<SetInlineGameScore>>(&self, set_inline_game_score: C) -> RTDResult<()> {
    self.send(set_inline_game_score.as_ref()) 
  }

  pub fn get_game_high_scores<C: AsRef<GetGameHighScores>>(&self, get_game_high_scores: C) -> RTDResult<GameHighScores> {
  
    match self.execute(get_game_high_scores.as_ref())? {
      Some(json) => Ok(GameHighScores::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_inline_game_high_scores<C: AsRef<GetInlineGameHighScores>>(&self, get_inline_game_high_scores: C) -> RTDResult<GameHighScores> {
  
    match self.execute(get_inline_game_high_scores.as_ref())? {
      Some(json) => Ok(GameHighScores::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn delete_chat_reply_markup<C: AsRef<DeleteChatReplyMarkup>>(&self, delete_chat_reply_markup: C) -> RTDResult<()> {
    self.send(delete_chat_reply_markup.as_ref()) 
  }

  pub fn send_chat_action<C: AsRef<SendChatAction>>(&self, send_chat_action: C) -> RTDResult<()> {
    self.send(send_chat_action.as_ref()) 
  }

  pub fn open_chat<C: AsRef<OpenChat>>(&self, open_chat: C) -> RTDResult<()> {
    self.send(open_chat.as_ref()) 
  }

  pub fn close_chat<C: AsRef<CloseChat>>(&self, close_chat: C) -> RTDResult<()> {
    self.send(close_chat.as_ref()) 
  }

  pub fn view_messages<C: AsRef<ViewMessages>>(&self, view_messages: C) -> RTDResult<()> {
    self.send(view_messages.as_ref()) 
  }

  pub fn open_message_content<C: AsRef<OpenMessageContent>>(&self, open_message_content: C) -> RTDResult<()> {
    self.send(open_message_content.as_ref()) 
  }

  pub fn read_all_chat_mentions<C: AsRef<ReadAllChatMentions>>(&self, read_all_chat_mentions: C) -> RTDResult<()> {
    self.send(read_all_chat_mentions.as_ref()) 
  }

  pub fn create_private_chat<C: AsRef<CreatePrivateChat>>(&self, create_private_chat: C) -> RTDResult<Chat> {
  
    match self.execute(create_private_chat.as_ref())? {
      Some(json) => Ok(Chat::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn create_basic_group_chat<C: AsRef<CreateBasicGroupChat>>(&self, create_basic_group_chat: C) -> RTDResult<Chat> {
  
    match self.execute(create_basic_group_chat.as_ref())? {
      Some(json) => Ok(Chat::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn create_supergroup_chat<C: AsRef<CreateSupergroupChat>>(&self, create_supergroup_chat: C) -> RTDResult<Chat> {
  
    match self.execute(create_supergroup_chat.as_ref())? {
      Some(json) => Ok(Chat::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn create_secret_chat<C: AsRef<CreateSecretChat>>(&self, create_secret_chat: C) -> RTDResult<Chat> {
  
    match self.execute(create_secret_chat.as_ref())? {
      Some(json) => Ok(Chat::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn create_new_basic_group_chat<C: AsRef<CreateNewBasicGroupChat>>(&self, create_new_basic_group_chat: C) -> RTDResult<Chat> {
  
    match self.execute(create_new_basic_group_chat.as_ref())? {
      Some(json) => Ok(Chat::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn create_new_supergroup_chat<C: AsRef<CreateNewSupergroupChat>>(&self, create_new_supergroup_chat: C) -> RTDResult<Chat> {
  
    match self.execute(create_new_supergroup_chat.as_ref())? {
      Some(json) => Ok(Chat::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn create_new_secret_chat<C: AsRef<CreateNewSecretChat>>(&self, create_new_secret_chat: C) -> RTDResult<Chat> {
  
    match self.execute(create_new_secret_chat.as_ref())? {
      Some(json) => Ok(Chat::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn upgrade_basic_group_chat_to_supergroup_chat<C: AsRef<UpgradeBasicGroupChatToSupergroupChat>>(&self, upgrade_basic_group_chat_to_supergroup_chat: C) -> RTDResult<Chat> {
  
    match self.execute(upgrade_basic_group_chat_to_supergroup_chat.as_ref())? {
      Some(json) => Ok(Chat::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn set_chat_title<C: AsRef<SetChatTitle>>(&self, set_chat_title: C) -> RTDResult<()> {
    self.send(set_chat_title.as_ref()) 
  }

  pub fn set_chat_photo<C: AsRef<SetChatPhoto>>(&self, set_chat_photo: C) -> RTDResult<()> {
    self.send(set_chat_photo.as_ref()) 
  }

  pub fn set_chat_draft_message<C: AsRef<SetChatDraftMessage>>(&self, set_chat_draft_message: C) -> RTDResult<()> {
    self.send(set_chat_draft_message.as_ref()) 
  }

  pub fn set_chat_notification_settings<C: AsRef<SetChatNotificationSettings>>(&self, set_chat_notification_settings: C) -> RTDResult<()> {
    self.send(set_chat_notification_settings.as_ref()) 
  }

  pub fn toggle_chat_is_pinned<C: AsRef<ToggleChatIsPinned>>(&self, toggle_chat_is_pinned: C) -> RTDResult<()> {
    self.send(toggle_chat_is_pinned.as_ref()) 
  }

  pub fn toggle_chat_is_marked_as_unread<C: AsRef<ToggleChatIsMarkedAsUnread>>(&self, toggle_chat_is_marked_as_unread: C) -> RTDResult<()> {
    self.send(toggle_chat_is_marked_as_unread.as_ref()) 
  }

  pub fn toggle_chat_default_disable_notification<C: AsRef<ToggleChatDefaultDisableNotification>>(&self, toggle_chat_default_disable_notification: C) -> RTDResult<()> {
    self.send(toggle_chat_default_disable_notification.as_ref()) 
  }

  pub fn set_chat_client_data<C: AsRef<SetChatClientData>>(&self, set_chat_client_data: C) -> RTDResult<()> {
    self.send(set_chat_client_data.as_ref()) 
  }

  pub fn join_chat<C: AsRef<JoinChat>>(&self, join_chat: C) -> RTDResult<()> {
    self.send(join_chat.as_ref()) 
  }

  pub fn leave_chat<C: AsRef<LeaveChat>>(&self, leave_chat: C) -> RTDResult<()> {
    self.send(leave_chat.as_ref()) 
  }

  pub fn add_chat_member<C: AsRef<AddChatMember>>(&self, add_chat_member: C) -> RTDResult<()> {
    self.send(add_chat_member.as_ref()) 
  }

  pub fn add_chat_members<C: AsRef<AddChatMembers>>(&self, add_chat_members: C) -> RTDResult<()> {
    self.send(add_chat_members.as_ref()) 
  }

  pub fn set_chat_member_status<C: AsRef<SetChatMemberStatus>>(&self, set_chat_member_status: C) -> RTDResult<()> {
    self.send(set_chat_member_status.as_ref()) 
  }

  pub fn get_chat_member<C: AsRef<GetChatMember>>(&self, get_chat_member: C) -> RTDResult<ChatMember> {
  
    match self.execute(get_chat_member.as_ref())? {
      Some(json) => Ok(ChatMember::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn search_chat_members<C: AsRef<SearchChatMembers>>(&self, search_chat_members: C) -> RTDResult<ChatMembers> {
  
    match self.execute(search_chat_members.as_ref())? {
      Some(json) => Ok(ChatMembers::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_chat_administrators<C: AsRef<GetChatAdministrators>>(&self, get_chat_administrators: C) -> RTDResult<Users> {
  
    match self.execute(get_chat_administrators.as_ref())? {
      Some(json) => Ok(Users::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn clear_all_draft_messages<C: AsRef<ClearAllDraftMessages>>(&self, clear_all_draft_messages: C) -> RTDResult<()> {
    self.send(clear_all_draft_messages.as_ref()) 
  }

  pub fn get_scope_notification_settings<C: AsRef<GetScopeNotificationSettings>>(&self, get_scope_notification_settings: C) -> RTDResult<ScopeNotificationSettings> {
  
    match self.execute(get_scope_notification_settings.as_ref())? {
      Some(json) => Ok(ScopeNotificationSettings::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn set_scope_notification_settings<C: AsRef<SetScopeNotificationSettings>>(&self, set_scope_notification_settings: C) -> RTDResult<()> {
    self.send(set_scope_notification_settings.as_ref()) 
  }

  pub fn reset_all_notification_settings<C: AsRef<ResetAllNotificationSettings>>(&self, reset_all_notification_settings: C) -> RTDResult<()> {
    self.send(reset_all_notification_settings.as_ref()) 
  }

  pub fn set_pinned_chats<C: AsRef<SetPinnedChats>>(&self, set_pinned_chats: C) -> RTDResult<()> {
    self.send(set_pinned_chats.as_ref()) 
  }

  pub fn download_file<C: AsRef<DownloadFile>>(&self, download_file: C) -> RTDResult<File> {
  
    match self.execute(download_file.as_ref())? {
      Some(json) => Ok(File::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn cancel_download_file<C: AsRef<CancelDownloadFile>>(&self, cancel_download_file: C) -> RTDResult<()> {
    self.send(cancel_download_file.as_ref()) 
  }

  pub fn upload_file<C: AsRef<UploadFile>>(&self, upload_file: C) -> RTDResult<File> {
  
    match self.execute(upload_file.as_ref())? {
      Some(json) => Ok(File::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn cancel_upload_file<C: AsRef<CancelUploadFile>>(&self, cancel_upload_file: C) -> RTDResult<()> {
    self.send(cancel_upload_file.as_ref()) 
  }

  pub fn set_file_generation_progress<C: AsRef<SetFileGenerationProgress>>(&self, set_file_generation_progress: C) -> RTDResult<()> {
    self.send(set_file_generation_progress.as_ref()) 
  }

  pub fn finish_file_generation<C: AsRef<FinishFileGeneration>>(&self, finish_file_generation: C) -> RTDResult<()> {
    self.send(finish_file_generation.as_ref()) 
  }

  pub fn delete_file<C: AsRef<DeleteFile>>(&self, delete_file: C) -> RTDResult<()> {
    self.send(delete_file.as_ref()) 
  }

  pub fn generate_chat_invite_link<C: AsRef<GenerateChatInviteLink>>(&self, generate_chat_invite_link: C) -> RTDResult<ChatInviteLink> {
  
    match self.execute(generate_chat_invite_link.as_ref())? {
      Some(json) => Ok(ChatInviteLink::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn check_chat_invite_link<C: AsRef<CheckChatInviteLink>>(&self, check_chat_invite_link: C) -> RTDResult<ChatInviteLinkInfo> {
  
    match self.execute(check_chat_invite_link.as_ref())? {
      Some(json) => Ok(ChatInviteLinkInfo::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn join_chat_by_invite_link<C: AsRef<JoinChatByInviteLink>>(&self, join_chat_by_invite_link: C) -> RTDResult<Chat> {
  
    match self.execute(join_chat_by_invite_link.as_ref())? {
      Some(json) => Ok(Chat::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn create_call<C: AsRef<CreateCall>>(&self, create_call: C) -> RTDResult<CallId> {
  
    match self.execute(create_call.as_ref())? {
      Some(json) => Ok(CallId::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn accept_call<C: AsRef<AcceptCall>>(&self, accept_call: C) -> RTDResult<()> {
    self.send(accept_call.as_ref()) 
  }

  pub fn discard_call<C: AsRef<DiscardCall>>(&self, discard_call: C) -> RTDResult<()> {
    self.send(discard_call.as_ref()) 
  }

  pub fn send_call_rating<C: AsRef<SendCallRating>>(&self, send_call_rating: C) -> RTDResult<()> {
    self.send(send_call_rating.as_ref()) 
  }

  pub fn send_call_debug_information<C: AsRef<SendCallDebugInformation>>(&self, send_call_debug_information: C) -> RTDResult<()> {
    self.send(send_call_debug_information.as_ref()) 
  }

  pub fn block_user<C: AsRef<BlockUser>>(&self, block_user: C) -> RTDResult<()> {
    self.send(block_user.as_ref()) 
  }

  pub fn unblock_user<C: AsRef<UnblockUser>>(&self, unblock_user: C) -> RTDResult<()> {
    self.send(unblock_user.as_ref()) 
  }

  pub fn get_blocked_users<C: AsRef<GetBlockedUsers>>(&self, get_blocked_users: C) -> RTDResult<Users> {
  
    match self.execute(get_blocked_users.as_ref())? {
      Some(json) => Ok(Users::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn import_contacts<C: AsRef<ImportContacts>>(&self, import_contacts: C) -> RTDResult<ImportedContacts> {
  
    match self.execute(import_contacts.as_ref())? {
      Some(json) => Ok(ImportedContacts::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_contacts<C: AsRef<GetContacts>>(&self, get_contacts: C) -> RTDResult<Users> {
  
    match self.execute(get_contacts.as_ref())? {
      Some(json) => Ok(Users::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn search_contacts<C: AsRef<SearchContacts>>(&self, search_contacts: C) -> RTDResult<Users> {
  
    match self.execute(search_contacts.as_ref())? {
      Some(json) => Ok(Users::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn remove_contacts<C: AsRef<RemoveContacts>>(&self, remove_contacts: C) -> RTDResult<()> {
    self.send(remove_contacts.as_ref()) 
  }

  pub fn get_imported_contact_count<C: AsRef<GetImportedContactCount>>(&self, get_imported_contact_count: C) -> RTDResult<Count> {
  
    match self.execute(get_imported_contact_count.as_ref())? {
      Some(json) => Ok(Count::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn change_imported_contacts<C: AsRef<ChangeImportedContacts>>(&self, change_imported_contacts: C) -> RTDResult<ImportedContacts> {
  
    match self.execute(change_imported_contacts.as_ref())? {
      Some(json) => Ok(ImportedContacts::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn clear_imported_contacts<C: AsRef<ClearImportedContacts>>(&self, clear_imported_contacts: C) -> RTDResult<()> {
    self.send(clear_imported_contacts.as_ref()) 
  }

  pub fn get_user_profile_photos<C: AsRef<GetUserProfilePhotos>>(&self, get_user_profile_photos: C) -> RTDResult<UserProfilePhotos> {
  
    match self.execute(get_user_profile_photos.as_ref())? {
      Some(json) => Ok(UserProfilePhotos::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_stickers<C: AsRef<GetStickers>>(&self, get_stickers: C) -> RTDResult<Stickers> {
  
    match self.execute(get_stickers.as_ref())? {
      Some(json) => Ok(Stickers::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn search_stickers<C: AsRef<SearchStickers>>(&self, search_stickers: C) -> RTDResult<Stickers> {
  
    match self.execute(search_stickers.as_ref())? {
      Some(json) => Ok(Stickers::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_installed_sticker_sets<C: AsRef<GetInstalledStickerSets>>(&self, get_installed_sticker_sets: C) -> RTDResult<StickerSets> {
  
    match self.execute(get_installed_sticker_sets.as_ref())? {
      Some(json) => Ok(StickerSets::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_archived_sticker_sets<C: AsRef<GetArchivedStickerSets>>(&self, get_archived_sticker_sets: C) -> RTDResult<StickerSets> {
  
    match self.execute(get_archived_sticker_sets.as_ref())? {
      Some(json) => Ok(StickerSets::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_trending_sticker_sets<C: AsRef<GetTrendingStickerSets>>(&self, get_trending_sticker_sets: C) -> RTDResult<StickerSets> {
  
    match self.execute(get_trending_sticker_sets.as_ref())? {
      Some(json) => Ok(StickerSets::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_attached_sticker_sets<C: AsRef<GetAttachedStickerSets>>(&self, get_attached_sticker_sets: C) -> RTDResult<StickerSets> {
  
    match self.execute(get_attached_sticker_sets.as_ref())? {
      Some(json) => Ok(StickerSets::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_sticker_set<C: AsRef<GetStickerSet>>(&self, get_sticker_set: C) -> RTDResult<StickerSet> {
  
    match self.execute(get_sticker_set.as_ref())? {
      Some(json) => Ok(StickerSet::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn search_sticker_set<C: AsRef<SearchStickerSet>>(&self, search_sticker_set: C) -> RTDResult<StickerSet> {
  
    match self.execute(search_sticker_set.as_ref())? {
      Some(json) => Ok(StickerSet::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn search_installed_sticker_sets<C: AsRef<SearchInstalledStickerSets>>(&self, search_installed_sticker_sets: C) -> RTDResult<StickerSets> {
  
    match self.execute(search_installed_sticker_sets.as_ref())? {
      Some(json) => Ok(StickerSets::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn search_sticker_sets<C: AsRef<SearchStickerSets>>(&self, search_sticker_sets: C) -> RTDResult<StickerSets> {
  
    match self.execute(search_sticker_sets.as_ref())? {
      Some(json) => Ok(StickerSets::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn change_sticker_set<C: AsRef<ChangeStickerSet>>(&self, change_sticker_set: C) -> RTDResult<()> {
    self.send(change_sticker_set.as_ref()) 
  }

  pub fn view_trending_sticker_sets<C: AsRef<ViewTrendingStickerSets>>(&self, view_trending_sticker_sets: C) -> RTDResult<()> {
    self.send(view_trending_sticker_sets.as_ref()) 
  }

  pub fn reorder_installed_sticker_sets<C: AsRef<ReorderInstalledStickerSets>>(&self, reorder_installed_sticker_sets: C) -> RTDResult<()> {
    self.send(reorder_installed_sticker_sets.as_ref()) 
  }

  pub fn get_recent_stickers<C: AsRef<GetRecentStickers>>(&self, get_recent_stickers: C) -> RTDResult<Stickers> {
  
    match self.execute(get_recent_stickers.as_ref())? {
      Some(json) => Ok(Stickers::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn add_recent_sticker<C: AsRef<AddRecentSticker>>(&self, add_recent_sticker: C) -> RTDResult<Stickers> {
  
    match self.execute(add_recent_sticker.as_ref())? {
      Some(json) => Ok(Stickers::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn remove_recent_sticker<C: AsRef<RemoveRecentSticker>>(&self, remove_recent_sticker: C) -> RTDResult<()> {
    self.send(remove_recent_sticker.as_ref()) 
  }

  pub fn clear_recent_stickers<C: AsRef<ClearRecentStickers>>(&self, clear_recent_stickers: C) -> RTDResult<()> {
    self.send(clear_recent_stickers.as_ref()) 
  }

  pub fn get_favorite_stickers<C: AsRef<GetFavoriteStickers>>(&self, get_favorite_stickers: C) -> RTDResult<Stickers> {
  
    match self.execute(get_favorite_stickers.as_ref())? {
      Some(json) => Ok(Stickers::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn add_favorite_sticker<C: AsRef<AddFavoriteSticker>>(&self, add_favorite_sticker: C) -> RTDResult<()> {
    self.send(add_favorite_sticker.as_ref()) 
  }

  pub fn remove_favorite_sticker<C: AsRef<RemoveFavoriteSticker>>(&self, remove_favorite_sticker: C) -> RTDResult<()> {
    self.send(remove_favorite_sticker.as_ref()) 
  }

  pub fn get_sticker_emojis<C: AsRef<GetStickerEmojis>>(&self, get_sticker_emojis: C) -> RTDResult<StickerEmojis> {
  
    match self.execute(get_sticker_emojis.as_ref())? {
      Some(json) => Ok(StickerEmojis::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_saved_animations<C: AsRef<GetSavedAnimations>>(&self, get_saved_animations: C) -> RTDResult<Animations> {
  
    match self.execute(get_saved_animations.as_ref())? {
      Some(json) => Ok(Animations::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn add_saved_animation<C: AsRef<AddSavedAnimation>>(&self, add_saved_animation: C) -> RTDResult<()> {
    self.send(add_saved_animation.as_ref()) 
  }

  pub fn remove_saved_animation<C: AsRef<RemoveSavedAnimation>>(&self, remove_saved_animation: C) -> RTDResult<()> {
    self.send(remove_saved_animation.as_ref()) 
  }

  pub fn get_recent_inline_bots<C: AsRef<GetRecentInlineBots>>(&self, get_recent_inline_bots: C) -> RTDResult<Users> {
  
    match self.execute(get_recent_inline_bots.as_ref())? {
      Some(json) => Ok(Users::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn search_hashtags<C: AsRef<SearchHashtags>>(&self, search_hashtags: C) -> RTDResult<Hashtags> {
  
    match self.execute(search_hashtags.as_ref())? {
      Some(json) => Ok(Hashtags::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn remove_recent_hashtag<C: AsRef<RemoveRecentHashtag>>(&self, remove_recent_hashtag: C) -> RTDResult<()> {
    self.send(remove_recent_hashtag.as_ref()) 
  }

  pub fn get_web_page_preview<C: AsRef<GetWebPagePreview>>(&self, get_web_page_preview: C) -> RTDResult<WebPage> {
  
    match self.execute(get_web_page_preview.as_ref())? {
      Some(json) => Ok(WebPage::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_web_page_instant_view<C: AsRef<GetWebPageInstantView>>(&self, get_web_page_instant_view: C) -> RTDResult<WebPageInstantView> {
  
    match self.execute(get_web_page_instant_view.as_ref())? {
      Some(json) => Ok(WebPageInstantView::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn set_profile_photo<C: AsRef<SetProfilePhoto>>(&self, set_profile_photo: C) -> RTDResult<()> {
    self.send(set_profile_photo.as_ref()) 
  }

  pub fn delete_profile_photo<C: AsRef<DeleteProfilePhoto>>(&self, delete_profile_photo: C) -> RTDResult<()> {
    self.send(delete_profile_photo.as_ref()) 
  }

  pub fn set_name<C: AsRef<SetName>>(&self, set_name: C) -> RTDResult<()> {
    self.send(set_name.as_ref()) 
  }

  pub fn set_bio<C: AsRef<SetBio>>(&self, set_bio: C) -> RTDResult<()> {
    self.send(set_bio.as_ref()) 
  }

  pub fn set_username<C: AsRef<SetUsername>>(&self, set_username: C) -> RTDResult<()> {
    self.send(set_username.as_ref()) 
  }

  pub fn change_phone_number<C: AsRef<ChangePhoneNumber>>(&self, change_phone_number: C) -> RTDResult<AuthenticationCodeInfo> {
  
    match self.execute(change_phone_number.as_ref())? {
      Some(json) => Ok(AuthenticationCodeInfo::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn resend_change_phone_number_code<C: AsRef<ResendChangePhoneNumberCode>>(&self, resend_change_phone_number_code: C) -> RTDResult<AuthenticationCodeInfo> {
  
    match self.execute(resend_change_phone_number_code.as_ref())? {
      Some(json) => Ok(AuthenticationCodeInfo::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn check_change_phone_number_code<C: AsRef<CheckChangePhoneNumberCode>>(&self, check_change_phone_number_code: C) -> RTDResult<()> {
    self.send(check_change_phone_number_code.as_ref()) 
  }

  pub fn get_active_sessions<C: AsRef<GetActiveSessions>>(&self, get_active_sessions: C) -> RTDResult<Sessions> {
  
    match self.execute(get_active_sessions.as_ref())? {
      Some(json) => Ok(Sessions::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn terminate_session<C: AsRef<TerminateSession>>(&self, terminate_session: C) -> RTDResult<()> {
    self.send(terminate_session.as_ref()) 
  }

  pub fn terminate_all_other_sessions<C: AsRef<TerminateAllOtherSessions>>(&self, terminate_all_other_sessions: C) -> RTDResult<()> {
    self.send(terminate_all_other_sessions.as_ref()) 
  }

  pub fn get_connected_websites<C: AsRef<GetConnectedWebsites>>(&self, get_connected_websites: C) -> RTDResult<ConnectedWebsites> {
  
    match self.execute(get_connected_websites.as_ref())? {
      Some(json) => Ok(ConnectedWebsites::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn disconnect_website<C: AsRef<DisconnectWebsite>>(&self, disconnect_website: C) -> RTDResult<()> {
    self.send(disconnect_website.as_ref()) 
  }

  pub fn disconnect_all_websites<C: AsRef<DisconnectAllWebsites>>(&self, disconnect_all_websites: C) -> RTDResult<()> {
    self.send(disconnect_all_websites.as_ref()) 
  }

  pub fn toggle_basic_group_administrators<C: AsRef<ToggleBasicGroupAdministrators>>(&self, toggle_basic_group_administrators: C) -> RTDResult<()> {
    self.send(toggle_basic_group_administrators.as_ref()) 
  }

  pub fn set_supergroup_username<C: AsRef<SetSupergroupUsername>>(&self, set_supergroup_username: C) -> RTDResult<()> {
    self.send(set_supergroup_username.as_ref()) 
  }

  pub fn set_supergroup_sticker_set<C: AsRef<SetSupergroupStickerSet>>(&self, set_supergroup_sticker_set: C) -> RTDResult<()> {
    self.send(set_supergroup_sticker_set.as_ref()) 
  }

  pub fn toggle_supergroup_invites<C: AsRef<ToggleSupergroupInvites>>(&self, toggle_supergroup_invites: C) -> RTDResult<()> {
    self.send(toggle_supergroup_invites.as_ref()) 
  }

  pub fn toggle_supergroup_sign_messages<C: AsRef<ToggleSupergroupSignMessages>>(&self, toggle_supergroup_sign_messages: C) -> RTDResult<()> {
    self.send(toggle_supergroup_sign_messages.as_ref()) 
  }

  pub fn toggle_supergroup_is_all_history_available<C: AsRef<ToggleSupergroupIsAllHistoryAvailable>>(&self, toggle_supergroup_is_all_history_available: C) -> RTDResult<()> {
    self.send(toggle_supergroup_is_all_history_available.as_ref()) 
  }

  pub fn set_supergroup_description<C: AsRef<SetSupergroupDescription>>(&self, set_supergroup_description: C) -> RTDResult<()> {
    self.send(set_supergroup_description.as_ref()) 
  }

  pub fn pin_supergroup_message<C: AsRef<PinSupergroupMessage>>(&self, pin_supergroup_message: C) -> RTDResult<()> {
    self.send(pin_supergroup_message.as_ref()) 
  }

  pub fn unpin_supergroup_message<C: AsRef<UnpinSupergroupMessage>>(&self, unpin_supergroup_message: C) -> RTDResult<()> {
    self.send(unpin_supergroup_message.as_ref()) 
  }

  pub fn report_supergroup_spam<C: AsRef<ReportSupergroupSpam>>(&self, report_supergroup_spam: C) -> RTDResult<()> {
    self.send(report_supergroup_spam.as_ref()) 
  }

  pub fn get_supergroup_members<C: AsRef<GetSupergroupMembers>>(&self, get_supergroup_members: C) -> RTDResult<ChatMembers> {
  
    match self.execute(get_supergroup_members.as_ref())? {
      Some(json) => Ok(ChatMembers::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn delete_supergroup<C: AsRef<DeleteSupergroup>>(&self, delete_supergroup: C) -> RTDResult<()> {
    self.send(delete_supergroup.as_ref()) 
  }

  pub fn close_secret_chat<C: AsRef<CloseSecretChat>>(&self, close_secret_chat: C) -> RTDResult<()> {
    self.send(close_secret_chat.as_ref()) 
  }

  pub fn get_chat_event_log<C: AsRef<GetChatEventLog>>(&self, get_chat_event_log: C) -> RTDResult<ChatEvents> {
  
    match self.execute(get_chat_event_log.as_ref())? {
      Some(json) => Ok(ChatEvents::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_payment_form<C: AsRef<GetPaymentForm>>(&self, get_payment_form: C) -> RTDResult<PaymentForm> {
  
    match self.execute(get_payment_form.as_ref())? {
      Some(json) => Ok(PaymentForm::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn validate_order_info<C: AsRef<ValidateOrderInfo>>(&self, validate_order_info: C) -> RTDResult<ValidatedOrderInfo> {
  
    match self.execute(validate_order_info.as_ref())? {
      Some(json) => Ok(ValidatedOrderInfo::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn send_payment_form<C: AsRef<SendPaymentForm>>(&self, send_payment_form: C) -> RTDResult<PaymentResult> {
  
    match self.execute(send_payment_form.as_ref())? {
      Some(json) => Ok(PaymentResult::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_payment_receipt<C: AsRef<GetPaymentReceipt>>(&self, get_payment_receipt: C) -> RTDResult<PaymentReceipt> {
  
    match self.execute(get_payment_receipt.as_ref())? {
      Some(json) => Ok(PaymentReceipt::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_saved_order_info<C: AsRef<GetSavedOrderInfo>>(&self, get_saved_order_info: C) -> RTDResult<OrderInfo> {
  
    match self.execute(get_saved_order_info.as_ref())? {
      Some(json) => Ok(OrderInfo::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn delete_saved_order_info<C: AsRef<DeleteSavedOrderInfo>>(&self, delete_saved_order_info: C) -> RTDResult<()> {
    self.send(delete_saved_order_info.as_ref()) 
  }

  pub fn delete_saved_credentials<C: AsRef<DeleteSavedCredentials>>(&self, delete_saved_credentials: C) -> RTDResult<()> {
    self.send(delete_saved_credentials.as_ref()) 
  }

  pub fn get_support_user<C: AsRef<GetSupportUser>>(&self, get_support_user: C) -> RTDResult<User> {
  
    match self.execute(get_support_user.as_ref())? {
      Some(json) => Ok(User::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_wallpapers<C: AsRef<GetWallpapers>>(&self, get_wallpapers: C) -> RTDResult<Wallpapers> {
  
    match self.execute(get_wallpapers.as_ref())? {
      Some(json) => Ok(Wallpapers::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_localization_target_info<C: AsRef<GetLocalizationTargetInfo>>(&self, get_localization_target_info: C) -> RTDResult<LocalizationTargetInfo> {
  
    match self.execute(get_localization_target_info.as_ref())? {
      Some(json) => Ok(LocalizationTargetInfo::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_language_pack_strings<C: AsRef<GetLanguagePackStrings>>(&self, get_language_pack_strings: C) -> RTDResult<LanguagePackStrings> {
  
    match self.execute(get_language_pack_strings.as_ref())? {
      Some(json) => Ok(LanguagePackStrings::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn set_custom_language_pack<C: AsRef<SetCustomLanguagePack>>(&self, set_custom_language_pack: C) -> RTDResult<()> {
    self.send(set_custom_language_pack.as_ref()) 
  }

  pub fn edit_custom_language_pack_info<C: AsRef<EditCustomLanguagePackInfo>>(&self, edit_custom_language_pack_info: C) -> RTDResult<()> {
    self.send(edit_custom_language_pack_info.as_ref()) 
  }

  pub fn set_custom_language_pack_string<C: AsRef<SetCustomLanguagePackString>>(&self, set_custom_language_pack_string: C) -> RTDResult<()> {
    self.send(set_custom_language_pack_string.as_ref()) 
  }

  pub fn delete_language_pack<C: AsRef<DeleteLanguagePack>>(&self, delete_language_pack: C) -> RTDResult<()> {
    self.send(delete_language_pack.as_ref()) 
  }

  pub fn register_device<C: AsRef<RegisterDevice>>(&self, register_device: C) -> RTDResult<()> {
    self.send(register_device.as_ref()) 
  }

  pub fn get_recently_visited_t_me_urls<C: AsRef<GetRecentlyVisitedTMeUrls>>(&self, get_recently_visited_t_me_urls: C) -> RTDResult<TMeUrls> {
  
    match self.execute(get_recently_visited_t_me_urls.as_ref())? {
      Some(json) => Ok(TMeUrls::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn set_user_privacy_setting_rules<C: AsRef<SetUserPrivacySettingRules>>(&self, set_user_privacy_setting_rules: C) -> RTDResult<()> {
    self.send(set_user_privacy_setting_rules.as_ref()) 
  }

  pub fn get_user_privacy_setting_rules<C: AsRef<GetUserPrivacySettingRules>>(&self, get_user_privacy_setting_rules: C) -> RTDResult<UserPrivacySettingRules> {
  
    match self.execute(get_user_privacy_setting_rules.as_ref())? {
      Some(json) => Ok(UserPrivacySettingRules::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_option<C: AsRef<GetOption>>(&self, get_option: C) -> RTDResult<OptionValue> {
  
    match self.execute(get_option.as_ref())? {
      Some(json) => Ok(OptionValue::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn set_option<C: AsRef<SetOption>>(&self, set_option: C) -> RTDResult<()> {
    self.send(set_option.as_ref()) 
  }

  pub fn set_account_ttl<C: AsRef<SetAccountTtl>>(&self, set_account_ttl: C) -> RTDResult<()> {
    self.send(set_account_ttl.as_ref()) 
  }

  pub fn get_account_ttl<C: AsRef<GetAccountTtl>>(&self, get_account_ttl: C) -> RTDResult<AccountTtl> {
  
    match self.execute(get_account_ttl.as_ref())? {
      Some(json) => Ok(AccountTtl::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn delete_account<C: AsRef<DeleteAccount>>(&self, delete_account: C) -> RTDResult<()> {
    self.send(delete_account.as_ref()) 
  }

  pub fn get_chat_report_spam_state<C: AsRef<GetChatReportSpamState>>(&self, get_chat_report_spam_state: C) -> RTDResult<ChatReportSpamState> {
  
    match self.execute(get_chat_report_spam_state.as_ref())? {
      Some(json) => Ok(ChatReportSpamState::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn change_chat_report_spam_state<C: AsRef<ChangeChatReportSpamState>>(&self, change_chat_report_spam_state: C) -> RTDResult<()> {
    self.send(change_chat_report_spam_state.as_ref()) 
  }

  pub fn report_chat<C: AsRef<ReportChat>>(&self, report_chat: C) -> RTDResult<()> {
    self.send(report_chat.as_ref()) 
  }

  pub fn get_storage_statistics<C: AsRef<GetStorageStatistics>>(&self, get_storage_statistics: C) -> RTDResult<StorageStatistics> {
  
    match self.execute(get_storage_statistics.as_ref())? {
      Some(json) => Ok(StorageStatistics::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_storage_statistics_fast<C: AsRef<GetStorageStatisticsFast>>(&self, get_storage_statistics_fast: C) -> RTDResult<StorageStatisticsFast> {
  
    match self.execute(get_storage_statistics_fast.as_ref())? {
      Some(json) => Ok(StorageStatisticsFast::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn optimize_storage<C: AsRef<OptimizeStorage>>(&self, optimize_storage: C) -> RTDResult<StorageStatistics> {
  
    match self.execute(optimize_storage.as_ref())? {
      Some(json) => Ok(StorageStatistics::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn set_network_type<C: AsRef<SetNetworkType>>(&self, set_network_type: C) -> RTDResult<()> {
    self.send(set_network_type.as_ref()) 
  }

  pub fn get_network_statistics<C: AsRef<GetNetworkStatistics>>(&self, get_network_statistics: C) -> RTDResult<NetworkStatistics> {
  
    match self.execute(get_network_statistics.as_ref())? {
      Some(json) => Ok(NetworkStatistics::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn add_network_statistics<C: AsRef<AddNetworkStatistics>>(&self, add_network_statistics: C) -> RTDResult<()> {
    self.send(add_network_statistics.as_ref()) 
  }

  pub fn reset_network_statistics<C: AsRef<ResetNetworkStatistics>>(&self, reset_network_statistics: C) -> RTDResult<()> {
    self.send(reset_network_statistics.as_ref()) 
  }

  pub fn get_passport_element<C: AsRef<GetPassportElement>>(&self, get_passport_element: C) -> RTDResult<PassportElement> {
  
    match self.execute(get_passport_element.as_ref())? {
      Some(json) => Ok(PassportElement::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_all_passport_elements<C: AsRef<GetAllPassportElements>>(&self, get_all_passport_elements: C) -> RTDResult<PassportElements> {
  
    match self.execute(get_all_passport_elements.as_ref())? {
      Some(json) => Ok(PassportElements::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn set_passport_element<C: AsRef<SetPassportElement>>(&self, set_passport_element: C) -> RTDResult<PassportElement> {
  
    match self.execute(set_passport_element.as_ref())? {
      Some(json) => Ok(PassportElement::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn delete_passport_element<C: AsRef<DeletePassportElement>>(&self, delete_passport_element: C) -> RTDResult<()> {
    self.send(delete_passport_element.as_ref()) 
  }

  pub fn set_passport_element_errors<C: AsRef<SetPassportElementErrors>>(&self, set_passport_element_errors: C) -> RTDResult<()> {
    self.send(set_passport_element_errors.as_ref()) 
  }

  pub fn get_preferred_country_language<C: AsRef<GetPreferredCountryLanguage>>(&self, get_preferred_country_language: C) -> RTDResult<Text> {
  
    match self.execute(get_preferred_country_language.as_ref())? {
      Some(json) => Ok(Text::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn send_phone_number_verification_code<C: AsRef<SendPhoneNumberVerificationCode>>(&self, send_phone_number_verification_code: C) -> RTDResult<AuthenticationCodeInfo> {
  
    match self.execute(send_phone_number_verification_code.as_ref())? {
      Some(json) => Ok(AuthenticationCodeInfo::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn resend_phone_number_verification_code<C: AsRef<ResendPhoneNumberVerificationCode>>(&self, resend_phone_number_verification_code: C) -> RTDResult<AuthenticationCodeInfo> {
  
    match self.execute(resend_phone_number_verification_code.as_ref())? {
      Some(json) => Ok(AuthenticationCodeInfo::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn check_phone_number_verification_code<C: AsRef<CheckPhoneNumberVerificationCode>>(&self, check_phone_number_verification_code: C) -> RTDResult<()> {
    self.send(check_phone_number_verification_code.as_ref()) 
  }

  pub fn send_email_address_verification_code<C: AsRef<SendEmailAddressVerificationCode>>(&self, send_email_address_verification_code: C) -> RTDResult<EmailAddressAuthenticationCodeInfo> {
  
    match self.execute(send_email_address_verification_code.as_ref())? {
      Some(json) => Ok(EmailAddressAuthenticationCodeInfo::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn resend_email_address_verification_code<C: AsRef<ResendEmailAddressVerificationCode>>(&self, resend_email_address_verification_code: C) -> RTDResult<EmailAddressAuthenticationCodeInfo> {
  
    match self.execute(resend_email_address_verification_code.as_ref())? {
      Some(json) => Ok(EmailAddressAuthenticationCodeInfo::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn check_email_address_verification_code<C: AsRef<CheckEmailAddressVerificationCode>>(&self, check_email_address_verification_code: C) -> RTDResult<()> {
    self.send(check_email_address_verification_code.as_ref()) 
  }

  pub fn get_passport_authorization_form<C: AsRef<GetPassportAuthorizationForm>>(&self, get_passport_authorization_form: C) -> RTDResult<PassportAuthorizationForm> {
  
    match self.execute(get_passport_authorization_form.as_ref())? {
      Some(json) => Ok(PassportAuthorizationForm::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn send_passport_authorization_form<C: AsRef<SendPassportAuthorizationForm>>(&self, send_passport_authorization_form: C) -> RTDResult<()> {
    self.send(send_passport_authorization_form.as_ref()) 
  }

  pub fn send_phone_number_confirmation_code<C: AsRef<SendPhoneNumberConfirmationCode>>(&self, send_phone_number_confirmation_code: C) -> RTDResult<AuthenticationCodeInfo> {
  
    match self.execute(send_phone_number_confirmation_code.as_ref())? {
      Some(json) => Ok(AuthenticationCodeInfo::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn resend_phone_number_confirmation_code<C: AsRef<ResendPhoneNumberConfirmationCode>>(&self, resend_phone_number_confirmation_code: C) -> RTDResult<AuthenticationCodeInfo> {
  
    match self.execute(resend_phone_number_confirmation_code.as_ref())? {
      Some(json) => Ok(AuthenticationCodeInfo::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn check_phone_number_confirmation_code<C: AsRef<CheckPhoneNumberConfirmationCode>>(&self, check_phone_number_confirmation_code: C) -> RTDResult<()> {
    self.send(check_phone_number_confirmation_code.as_ref()) 
  }

  pub fn set_bot_updates_status<C: AsRef<SetBotUpdatesStatus>>(&self, set_bot_updates_status: C) -> RTDResult<()> {
    self.send(set_bot_updates_status.as_ref()) 
  }

  pub fn upload_sticker_file<C: AsRef<UploadStickerFile>>(&self, upload_sticker_file: C) -> RTDResult<File> {
  
    match self.execute(upload_sticker_file.as_ref())? {
      Some(json) => Ok(File::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn create_new_sticker_set<C: AsRef<CreateNewStickerSet>>(&self, create_new_sticker_set: C) -> RTDResult<StickerSet> {
  
    match self.execute(create_new_sticker_set.as_ref())? {
      Some(json) => Ok(StickerSet::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn add_sticker_to_set<C: AsRef<AddStickerToSet>>(&self, add_sticker_to_set: C) -> RTDResult<StickerSet> {
  
    match self.execute(add_sticker_to_set.as_ref())? {
      Some(json) => Ok(StickerSet::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn set_sticker_position_in_set<C: AsRef<SetStickerPositionInSet>>(&self, set_sticker_position_in_set: C) -> RTDResult<()> {
    self.send(set_sticker_position_in_set.as_ref()) 
  }

  pub fn remove_sticker_from_set<C: AsRef<RemoveStickerFromSet>>(&self, remove_sticker_from_set: C) -> RTDResult<()> {
    self.send(remove_sticker_from_set.as_ref()) 
  }

  pub fn get_map_thumbnail_file<C: AsRef<GetMapThumbnailFile>>(&self, get_map_thumbnail_file: C) -> RTDResult<File> {
  
    match self.execute(get_map_thumbnail_file.as_ref())? {
      Some(json) => Ok(File::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn accept_terms_of_service<C: AsRef<AcceptTermsOfService>>(&self, accept_terms_of_service: C) -> RTDResult<()> {
    self.send(accept_terms_of_service.as_ref()) 
  }

  pub fn send_custom_request<C: AsRef<SendCustomRequest>>(&self, send_custom_request: C) -> RTDResult<CustomRequestResult> {
  
    match self.execute(send_custom_request.as_ref())? {
      Some(json) => Ok(CustomRequestResult::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn answer_custom_query<C: AsRef<AnswerCustomQuery>>(&self, answer_custom_query: C) -> RTDResult<()> {
    self.send(answer_custom_query.as_ref()) 
  }

  pub fn set_alarm<C: AsRef<SetAlarm>>(&self, set_alarm: C) -> RTDResult<()> {
    self.send(set_alarm.as_ref()) 
  }

  pub fn get_country_code<C: AsRef<GetCountryCode>>(&self, get_country_code: C) -> RTDResult<Text> {
  
    match self.execute(get_country_code.as_ref())? {
      Some(json) => Ok(Text::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_invite_text<C: AsRef<GetInviteText>>(&self, get_invite_text: C) -> RTDResult<Text> {
  
    match self.execute(get_invite_text.as_ref())? {
      Some(json) => Ok(Text::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_deep_link_info<C: AsRef<GetDeepLinkInfo>>(&self, get_deep_link_info: C) -> RTDResult<DeepLinkInfo> {
  
    match self.execute(get_deep_link_info.as_ref())? {
      Some(json) => Ok(DeepLinkInfo::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn add_proxy<C: AsRef<AddProxy>>(&self, add_proxy: C) -> RTDResult<Proxy> {
  
    match self.execute(add_proxy.as_ref())? {
      Some(json) => Ok(Proxy::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn edit_proxy<C: AsRef<EditProxy>>(&self, edit_proxy: C) -> RTDResult<Proxy> {
  
    match self.execute(edit_proxy.as_ref())? {
      Some(json) => Ok(Proxy::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn enable_proxy<C: AsRef<EnableProxy>>(&self, enable_proxy: C) -> RTDResult<()> {
    self.send(enable_proxy.as_ref()) 
  }

  pub fn disable_proxy<C: AsRef<DisableProxy>>(&self, disable_proxy: C) -> RTDResult<()> {
    self.send(disable_proxy.as_ref()) 
  }

  pub fn remove_proxy<C: AsRef<RemoveProxy>>(&self, remove_proxy: C) -> RTDResult<()> {
    self.send(remove_proxy.as_ref()) 
  }

  pub fn get_proxies<C: AsRef<GetProxies>>(&self, get_proxies: C) -> RTDResult<Proxies> {
  
    match self.execute(get_proxies.as_ref())? {
      Some(json) => Ok(Proxies::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn get_proxy_link<C: AsRef<GetProxyLink>>(&self, get_proxy_link: C) -> RTDResult<Text> {
  
    match self.execute(get_proxy_link.as_ref())? {
      Some(json) => Ok(Text::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn ping_proxy<C: AsRef<PingProxy>>(&self, ping_proxy: C) -> RTDResult<Seconds> {
  
    match self.execute(ping_proxy.as_ref())? {
      Some(json) => Ok(Seconds::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn test_call_empty<C: AsRef<TestCallEmpty>>(&self, test_call_empty: C) -> RTDResult<()> {
    self.send(test_call_empty.as_ref()) 
  }

  pub fn test_call_string<C: AsRef<TestCallString>>(&self, test_call_string: C) -> RTDResult<TestString> {
  
    match self.execute(test_call_string.as_ref())? {
      Some(json) => Ok(TestString::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn test_call_bytes<C: AsRef<TestCallBytes>>(&self, test_call_bytes: C) -> RTDResult<TestBytes> {
  
    match self.execute(test_call_bytes.as_ref())? {
      Some(json) => Ok(TestBytes::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn test_call_vector_int<C: AsRef<TestCallVectorInt>>(&self, test_call_vector_int: C) -> RTDResult<TestVectorInt> {
  
    match self.execute(test_call_vector_int.as_ref())? {
      Some(json) => Ok(TestVectorInt::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn test_call_vector_int_object<C: AsRef<TestCallVectorIntObject>>(&self, test_call_vector_int_object: C) -> RTDResult<TestVectorIntObject> {
  
    match self.execute(test_call_vector_int_object.as_ref())? {
      Some(json) => Ok(TestVectorIntObject::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn test_call_vector_string<C: AsRef<TestCallVectorString>>(&self, test_call_vector_string: C) -> RTDResult<TestVectorString> {
  
    match self.execute(test_call_vector_string.as_ref())? {
      Some(json) => Ok(TestVectorString::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn test_call_vector_string_object<C: AsRef<TestCallVectorStringObject>>(&self, test_call_vector_string_object: C) -> RTDResult<TestVectorStringObject> {
  
    match self.execute(test_call_vector_string_object.as_ref())? {
      Some(json) => Ok(TestVectorStringObject::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn test_square_int<C: AsRef<TestSquareInt>>(&self, test_square_int: C) -> RTDResult<TestInt> {
  
    match self.execute(test_square_int.as_ref())? {
      Some(json) => Ok(TestInt::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn test_network<C: AsRef<TestNetwork>>(&self, test_network: C) -> RTDResult<()> {
    self.send(test_network.as_ref()) 
  }

  pub fn test_get_difference<C: AsRef<TestGetDifference>>(&self, test_get_difference: C) -> RTDResult<()> {
    self.send(test_get_difference.as_ref()) 
  }

  pub fn test_use_update<C: AsRef<TestUseUpdate>>(&self, test_use_update: C) -> RTDResult<Update> {
  
    match self.execute(test_use_update.as_ref())? {
      Some(json) => Ok(Update::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }

  pub fn test_use_error<C: AsRef<TestUseError>>(&self, test_use_error: C) -> RTDResult<Error> {
  
    match self.execute(test_use_error.as_ref())? {
      Some(json) => Ok(Error::from_json(json)?),
      None => Err(rtdlib::errors::RTDError::custom(tip::no_data_returned_from_tdlib())),
    }
  
  }


}

