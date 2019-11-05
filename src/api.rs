use core::borrow::Borrow;
use std::sync::Arc;

use regex::Regex;
use rtdlib::errors::*;
use rtdlib::tdjson;
use rtdlib::types::*;

use crate::tip;

#[derive(Debug, Clone)]
pub struct ApiBuilder {
  inner: Api
}

impl ApiBuilder {
  pub fn new() -> Self {
    Self {
      inner: Api {
        tdlib: Arc::new(tdjson::Tdlib::new()),
        log: true,
        unsafe_log: false
      }
    }
  }

  pub fn build(&self) -> Api {
    self.inner.clone()
  }

  fn tdlib(&mut self, tdlib: tdjson::Tdlib) -> &mut Self{
    self.inner.tdlib = Arc::new(tdlib);
    self
  }

  pub fn log(&mut self, open: bool) -> &mut Self {
    self.inner.log = open;
    self
  }

  pub fn unsafe_log(&mut self, unsafe_log: bool) -> &mut Self {
    self.inner.unsafe_log = unsafe_log;
    self
  }
}


#[derive(Debug, Clone)]
pub struct Api {
  tdlib: Arc<tdjson::Tdlib>,
  log: bool,
  unsafe_log: bool,
}

impl Default for Api {
  fn default() -> Self {
    ApiBuilder::new().build()
  }
}

impl Api {

  pub fn builder() -> ApiBuilder {
    ApiBuilder::new()
  }

  pub fn new(tdlib: tdjson::Tdlib) -> Self {
    ApiBuilder::new().tdlib(tdlib).build()
  }

  #[doc(hidden)]
  pub fn tdlib(&self) -> &tdjson::Tdlib {
    self.tdlib.borrow()
  }

  fn safe_log(&self, text: &String) -> String {
    if self.unsafe_log {
      return text.clone();
    }
    if text.contains("api_id") || text.contains("api_hash") {
      let regex_api_id = Regex::new(r#"api_id":\d*"#).expect("Regex fail");
      let hide_api_id = regex_api_id.replace_all(text, r#"api_id":"****""#);
      let regex_api_hash = Regex::new(r#"api_hash":"[0-9|a-f]*""#).expect("Regex fail");
      let hide_api_hash = regex_api_hash.replace_all(&hide_api_id, r#"api_hash":"**********""#);
      hide_api_hash.into_owned()
    } else {
      text.clone()
    }
  }

  pub fn send<Fnc: RFunction>(&self, fnc: Fnc) -> RTDResult<()> {
    let json = fnc.to_json()?;
    if self.log {
      info!("===> {}", self.safe_log(&json));
    }
    self.tdlib.send(&json[..]);
    Ok(())
  }

  pub fn receive(&self, timeout: f64) -> Option<String> {
    let receive = self.tdlib.receive(timeout);
    if self.log {
      if receive.is_some() {
        info!("<=== {}", receive.clone().map_or("<NONE>".to_string(), |v| self.safe_log(&v)));
      }
    }
    receive
  }

  pub fn execute<Fnc: RFunction>(&self, fnc: Fnc) -> RTDResult<Option<String>> {
    let json = fnc.to_json()?;
    if self.log {
      info!("===>>> {}", self.safe_log(&json));
    }
    Ok(self.tdlib.execute(&json[..]))
  }




  pub fn get_authorization_state<C: AsRef<GetAuthorizationState>>(&self, get_authorization_state: C) -> RTDResult<()> {
    self.send(get_authorization_state.as_ref())
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

  pub fn register_user<C: AsRef<RegisterUser>>(&self, register_user: C) -> RTDResult<()> {
    self.send(register_user.as_ref())
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

  pub fn get_current_state<C: AsRef<GetCurrentState>>(&self, get_current_state: C) -> RTDResult<()> {
    self.send(get_current_state.as_ref())
  }

  pub fn set_database_encryption_key<C: AsRef<SetDatabaseEncryptionKey>>(&self, set_database_encryption_key: C) -> RTDResult<()> {
    self.send(set_database_encryption_key.as_ref())
  }

  pub fn get_password_state<C: AsRef<GetPasswordState>>(&self, get_password_state: C) -> RTDResult<()> {
    self.send(get_password_state.as_ref())
  }

  pub fn set_password<C: AsRef<SetPassword>>(&self, set_password: C) -> RTDResult<()> {
    self.send(set_password.as_ref())
  }

  pub fn get_recovery_email_address<C: AsRef<GetRecoveryEmailAddress>>(&self, get_recovery_email_address: C) -> RTDResult<()> {
    self.send(get_recovery_email_address.as_ref())
  }

  pub fn set_recovery_email_address<C: AsRef<SetRecoveryEmailAddress>>(&self, set_recovery_email_address: C) -> RTDResult<()> {
    self.send(set_recovery_email_address.as_ref())
  }

  pub fn check_recovery_email_address_code<C: AsRef<CheckRecoveryEmailAddressCode>>(&self, check_recovery_email_address_code: C) -> RTDResult<()> {
    self.send(check_recovery_email_address_code.as_ref())
  }

  pub fn resend_recovery_email_address_code<C: AsRef<ResendRecoveryEmailAddressCode>>(&self, resend_recovery_email_address_code: C) -> RTDResult<()> {
    self.send(resend_recovery_email_address_code.as_ref())
  }

  pub fn request_password_recovery<C: AsRef<RequestPasswordRecovery>>(&self, request_password_recovery: C) -> RTDResult<()> {
    self.send(request_password_recovery.as_ref())
  }

  pub fn recover_password<C: AsRef<RecoverPassword>>(&self, recover_password: C) -> RTDResult<()> {
    self.send(recover_password.as_ref())
  }

  pub fn create_temporary_password<C: AsRef<CreateTemporaryPassword>>(&self, create_temporary_password: C) -> RTDResult<()> {
    self.send(create_temporary_password.as_ref())
  }

  pub fn get_temporary_password_state<C: AsRef<GetTemporaryPasswordState>>(&self, get_temporary_password_state: C) -> RTDResult<()> {
    self.send(get_temporary_password_state.as_ref())
  }

  pub fn get_me<C: AsRef<GetMe>>(&self, get_me: C) -> RTDResult<()> {
    self.send(get_me.as_ref())
  }

  pub fn get_user<C: AsRef<GetUser>>(&self, get_user: C) -> RTDResult<()> {
    self.send(get_user.as_ref())
  }

  pub fn get_user_full_info<C: AsRef<GetUserFullInfo>>(&self, get_user_full_info: C) -> RTDResult<()> {
    self.send(get_user_full_info.as_ref())
  }

  pub fn get_basic_group<C: AsRef<GetBasicGroup>>(&self, get_basic_group: C) -> RTDResult<()> {
    self.send(get_basic_group.as_ref())
  }

  pub fn get_basic_group_full_info<C: AsRef<GetBasicGroupFullInfo>>(&self, get_basic_group_full_info: C) -> RTDResult<()> {
    self.send(get_basic_group_full_info.as_ref())
  }

  pub fn get_supergroup<C: AsRef<GetSupergroup>>(&self, get_supergroup: C) -> RTDResult<()> {
    self.send(get_supergroup.as_ref())
  }

  pub fn get_supergroup_full_info<C: AsRef<GetSupergroupFullInfo>>(&self, get_supergroup_full_info: C) -> RTDResult<()> {
    self.send(get_supergroup_full_info.as_ref())
  }

  pub fn get_secret_chat<C: AsRef<GetSecretChat>>(&self, get_secret_chat: C) -> RTDResult<()> {
    self.send(get_secret_chat.as_ref())
  }

  pub fn get_chat<C: AsRef<GetChat>>(&self, get_chat: C) -> RTDResult<()> {
    self.send(get_chat.as_ref())
  }

  pub fn get_message<C: AsRef<GetMessage>>(&self, get_message: C) -> RTDResult<()> {
    self.send(get_message.as_ref())
  }

  pub fn get_message_locally<C: AsRef<GetMessageLocally>>(&self, get_message_locally: C) -> RTDResult<()> {
    self.send(get_message_locally.as_ref())
  }

  pub fn get_replied_message<C: AsRef<GetRepliedMessage>>(&self, get_replied_message: C) -> RTDResult<()> {
    self.send(get_replied_message.as_ref())
  }

  pub fn get_chat_pinned_message<C: AsRef<GetChatPinnedMessage>>(&self, get_chat_pinned_message: C) -> RTDResult<()> {
    self.send(get_chat_pinned_message.as_ref())
  }

  pub fn get_messages<C: AsRef<GetMessages>>(&self, get_messages: C) -> RTDResult<()> {
    self.send(get_messages.as_ref())
  }

  pub fn get_file<C: AsRef<GetFile>>(&self, get_file: C) -> RTDResult<()> {
    self.send(get_file.as_ref())
  }

  pub fn get_remote_file<C: AsRef<GetRemoteFile>>(&self, get_remote_file: C) -> RTDResult<()> {
    self.send(get_remote_file.as_ref())
  }

  pub fn get_chats<C: AsRef<GetChats>>(&self, get_chats: C) -> RTDResult<()> {
    self.send(get_chats.as_ref())
  }

  pub fn search_public_chat<C: AsRef<SearchPublicChat>>(&self, search_public_chat: C) -> RTDResult<()> {
    self.send(search_public_chat.as_ref())
  }

  pub fn search_public_chats<C: AsRef<SearchPublicChats>>(&self, search_public_chats: C) -> RTDResult<()> {
    self.send(search_public_chats.as_ref())
  }

  pub fn search_chats<C: AsRef<SearchChats>>(&self, search_chats: C) -> RTDResult<()> {
    self.send(search_chats.as_ref())
  }

  pub fn search_chats_on_server<C: AsRef<SearchChatsOnServer>>(&self, search_chats_on_server: C) -> RTDResult<()> {
    self.send(search_chats_on_server.as_ref())
  }

  pub fn get_top_chats<C: AsRef<GetTopChats>>(&self, get_top_chats: C) -> RTDResult<()> {
    self.send(get_top_chats.as_ref())
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

  pub fn check_chat_username<C: AsRef<CheckChatUsername>>(&self, check_chat_username: C) -> RTDResult<()> {
    self.send(check_chat_username.as_ref())
  }

  pub fn get_created_public_chats<C: AsRef<GetCreatedPublicChats>>(&self, get_created_public_chats: C) -> RTDResult<()> {
    self.send(get_created_public_chats.as_ref())
  }

  pub fn get_groups_in_common<C: AsRef<GetGroupsInCommon>>(&self, get_groups_in_common: C) -> RTDResult<()> {
    self.send(get_groups_in_common.as_ref())
  }

  pub fn get_chat_history<C: AsRef<GetChatHistory>>(&self, get_chat_history: C) -> RTDResult<()> {
    self.send(get_chat_history.as_ref())
  }

  pub fn delete_chat_history<C: AsRef<DeleteChatHistory>>(&self, delete_chat_history: C) -> RTDResult<()> {
    self.send(delete_chat_history.as_ref())
  }

  pub fn search_chat_messages<C: AsRef<SearchChatMessages>>(&self, search_chat_messages: C) -> RTDResult<()> {
    self.send(search_chat_messages.as_ref())
  }

  pub fn search_messages<C: AsRef<SearchMessages>>(&self, search_messages: C) -> RTDResult<()> {
    self.send(search_messages.as_ref())
  }

  pub fn search_secret_messages<C: AsRef<SearchSecretMessages>>(&self, search_secret_messages: C) -> RTDResult<()> {
    self.send(search_secret_messages.as_ref())
  }

  pub fn search_call_messages<C: AsRef<SearchCallMessages>>(&self, search_call_messages: C) -> RTDResult<()> {
    self.send(search_call_messages.as_ref())
  }

  pub fn search_chat_recent_location_messages<C: AsRef<SearchChatRecentLocationMessages>>(&self, search_chat_recent_location_messages: C) -> RTDResult<()> {
    self.send(search_chat_recent_location_messages.as_ref())
  }

  pub fn get_active_live_location_messages<C: AsRef<GetActiveLiveLocationMessages>>(&self, get_active_live_location_messages: C) -> RTDResult<()> {
    self.send(get_active_live_location_messages.as_ref())
  }

  pub fn get_chat_message_by_date<C: AsRef<GetChatMessageByDate>>(&self, get_chat_message_by_date: C) -> RTDResult<()> {
    self.send(get_chat_message_by_date.as_ref())
  }

  pub fn get_chat_message_count<C: AsRef<GetChatMessageCount>>(&self, get_chat_message_count: C) -> RTDResult<()> {
    self.send(get_chat_message_count.as_ref())
  }

  pub fn remove_notification<C: AsRef<RemoveNotification>>(&self, remove_notification: C) -> RTDResult<()> {
    self.send(remove_notification.as_ref())
  }

  pub fn remove_notification_group<C: AsRef<RemoveNotificationGroup>>(&self, remove_notification_group: C) -> RTDResult<()> {
    self.send(remove_notification_group.as_ref())
  }

  pub fn get_public_message_link<C: AsRef<GetPublicMessageLink>>(&self, get_public_message_link: C) -> RTDResult<()> {
    self.send(get_public_message_link.as_ref())
  }

  pub fn get_message_link<C: AsRef<GetMessageLink>>(&self, get_message_link: C) -> RTDResult<()> {
    self.send(get_message_link.as_ref())
  }

  pub fn get_message_link_info<C: AsRef<GetMessageLinkInfo>>(&self, get_message_link_info: C) -> RTDResult<()> {
    self.send(get_message_link_info.as_ref())
  }

  pub fn send_message<C: AsRef<SendMessage>>(&self, send_message: C) -> RTDResult<()> {
    self.send(send_message.as_ref())
  }

  pub fn send_message_album<C: AsRef<SendMessageAlbum>>(&self, send_message_album: C) -> RTDResult<()> {
    self.send(send_message_album.as_ref())
  }

  pub fn send_bot_start_message<C: AsRef<SendBotStartMessage>>(&self, send_bot_start_message: C) -> RTDResult<()> {
    self.send(send_bot_start_message.as_ref())
  }

  pub fn send_inline_query_result_message<C: AsRef<SendInlineQueryResultMessage>>(&self, send_inline_query_result_message: C) -> RTDResult<()> {
    self.send(send_inline_query_result_message.as_ref())
  }

  pub fn forward_messages<C: AsRef<ForwardMessages>>(&self, forward_messages: C) -> RTDResult<()> {
    self.send(forward_messages.as_ref())
  }

  pub fn resend_messages<C: AsRef<ResendMessages>>(&self, resend_messages: C) -> RTDResult<()> {
    self.send(resend_messages.as_ref())
  }

  pub fn send_chat_set_ttl_message<C: AsRef<SendChatSetTtlMessage>>(&self, send_chat_set_ttl_message: C) -> RTDResult<()> {
    self.send(send_chat_set_ttl_message.as_ref())
  }

  pub fn send_chat_screenshot_taken_notification<C: AsRef<SendChatScreenshotTakenNotification>>(&self, send_chat_screenshot_taken_notification: C) -> RTDResult<()> {
    self.send(send_chat_screenshot_taken_notification.as_ref())
  }

  pub fn add_local_message<C: AsRef<AddLocalMessage>>(&self, add_local_message: C) -> RTDResult<()> {
    self.send(add_local_message.as_ref())
  }

  pub fn delete_messages<C: AsRef<DeleteMessages>>(&self, delete_messages: C) -> RTDResult<()> {
    self.send(delete_messages.as_ref())
  }

  pub fn delete_chat_messages_from_user<C: AsRef<DeleteChatMessagesFromUser>>(&self, delete_chat_messages_from_user: C) -> RTDResult<()> {
    self.send(delete_chat_messages_from_user.as_ref())
  }

  pub fn edit_message_text<C: AsRef<EditMessageText>>(&self, edit_message_text: C) -> RTDResult<()> {
    self.send(edit_message_text.as_ref())
  }

  pub fn edit_message_live_location<C: AsRef<EditMessageLiveLocation>>(&self, edit_message_live_location: C) -> RTDResult<()> {
    self.send(edit_message_live_location.as_ref())
  }

  pub fn edit_message_media<C: AsRef<EditMessageMedia>>(&self, edit_message_media: C) -> RTDResult<()> {
    self.send(edit_message_media.as_ref())
  }

  pub fn edit_message_caption<C: AsRef<EditMessageCaption>>(&self, edit_message_caption: C) -> RTDResult<()> {
    self.send(edit_message_caption.as_ref())
  }

  pub fn edit_message_reply_markup<C: AsRef<EditMessageReplyMarkup>>(&self, edit_message_reply_markup: C) -> RTDResult<()> {
    self.send(edit_message_reply_markup.as_ref())
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

  pub fn get_text_entities<C: AsRef<GetTextEntities>>(&self, get_text_entities: C) -> RTDResult<()> {
    self.send(get_text_entities.as_ref())
  }

  pub fn parse_text_entities<C: AsRef<ParseTextEntities>>(&self, parse_text_entities: C) -> RTDResult<()> {
    self.send(parse_text_entities.as_ref())
  }

  pub fn get_file_mime_type<C: AsRef<GetFileMimeType>>(&self, get_file_mime_type: C) -> RTDResult<()> {
    self.send(get_file_mime_type.as_ref())
  }

  pub fn get_file_extension<C: AsRef<GetFileExtension>>(&self, get_file_extension: C) -> RTDResult<()> {
    self.send(get_file_extension.as_ref())
  }

  pub fn clean_file_name<C: AsRef<CleanFileName>>(&self, clean_file_name: C) -> RTDResult<()> {
    self.send(clean_file_name.as_ref())
  }

  pub fn get_language_pack_string<C: AsRef<GetLanguagePackString>>(&self, get_language_pack_string: C) -> RTDResult<()> {
    self.send(get_language_pack_string.as_ref())
  }

  pub fn get_json_value<C: AsRef<GetJsonValue>>(&self, get_json_value: C) -> RTDResult<()> {
    self.send(get_json_value.as_ref())
  }

  pub fn get_json_string<C: AsRef<GetJsonString>>(&self, get_json_string: C) -> RTDResult<()> {
    self.send(get_json_string.as_ref())
  }

  pub fn set_poll_answer<C: AsRef<SetPollAnswer>>(&self, set_poll_answer: C) -> RTDResult<()> {
    self.send(set_poll_answer.as_ref())
  }

  pub fn stop_poll<C: AsRef<StopPoll>>(&self, stop_poll: C) -> RTDResult<()> {
    self.send(stop_poll.as_ref())
  }

  pub fn get_inline_query_results<C: AsRef<GetInlineQueryResults>>(&self, get_inline_query_results: C) -> RTDResult<()> {
    self.send(get_inline_query_results.as_ref())
  }

  pub fn answer_inline_query<C: AsRef<AnswerInlineQuery>>(&self, answer_inline_query: C) -> RTDResult<()> {
    self.send(answer_inline_query.as_ref())
  }

  pub fn get_callback_query_answer<C: AsRef<GetCallbackQueryAnswer>>(&self, get_callback_query_answer: C) -> RTDResult<()> {
    self.send(get_callback_query_answer.as_ref())
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

  pub fn set_game_score<C: AsRef<SetGameScore>>(&self, set_game_score: C) -> RTDResult<()> {
    self.send(set_game_score.as_ref())
  }

  pub fn set_inline_game_score<C: AsRef<SetInlineGameScore>>(&self, set_inline_game_score: C) -> RTDResult<()> {
    self.send(set_inline_game_score.as_ref())
  }

  pub fn get_game_high_scores<C: AsRef<GetGameHighScores>>(&self, get_game_high_scores: C) -> RTDResult<()> {
    self.send(get_game_high_scores.as_ref())
  }

  pub fn get_inline_game_high_scores<C: AsRef<GetInlineGameHighScores>>(&self, get_inline_game_high_scores: C) -> RTDResult<()> {
    self.send(get_inline_game_high_scores.as_ref())
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

  pub fn create_private_chat<C: AsRef<CreatePrivateChat>>(&self, create_private_chat: C) -> RTDResult<()> {
    self.send(create_private_chat.as_ref())
  }

  pub fn create_basic_group_chat<C: AsRef<CreateBasicGroupChat>>(&self, create_basic_group_chat: C) -> RTDResult<()> {
    self.send(create_basic_group_chat.as_ref())
  }

  pub fn create_supergroup_chat<C: AsRef<CreateSupergroupChat>>(&self, create_supergroup_chat: C) -> RTDResult<()> {
    self.send(create_supergroup_chat.as_ref())
  }

  pub fn create_secret_chat<C: AsRef<CreateSecretChat>>(&self, create_secret_chat: C) -> RTDResult<()> {
    self.send(create_secret_chat.as_ref())
  }

  pub fn create_new_basic_group_chat<C: AsRef<CreateNewBasicGroupChat>>(&self, create_new_basic_group_chat: C) -> RTDResult<()> {
    self.send(create_new_basic_group_chat.as_ref())
  }

  pub fn create_new_supergroup_chat<C: AsRef<CreateNewSupergroupChat>>(&self, create_new_supergroup_chat: C) -> RTDResult<()> {
    self.send(create_new_supergroup_chat.as_ref())
  }

  pub fn create_new_secret_chat<C: AsRef<CreateNewSecretChat>>(&self, create_new_secret_chat: C) -> RTDResult<()> {
    self.send(create_new_secret_chat.as_ref())
  }

  pub fn upgrade_basic_group_chat_to_supergroup_chat<C: AsRef<UpgradeBasicGroupChatToSupergroupChat>>(&self, upgrade_basic_group_chat_to_supergroup_chat: C) -> RTDResult<()> {
    self.send(upgrade_basic_group_chat_to_supergroup_chat.as_ref())
  }

  pub fn set_chat_title<C: AsRef<SetChatTitle>>(&self, set_chat_title: C) -> RTDResult<()> {
    self.send(set_chat_title.as_ref())
  }

  pub fn set_chat_photo<C: AsRef<SetChatPhoto>>(&self, set_chat_photo: C) -> RTDResult<()> {
    self.send(set_chat_photo.as_ref())
  }

  pub fn set_chat_permissions<C: AsRef<SetChatPermissions>>(&self, set_chat_permissions: C) -> RTDResult<()> {
    self.send(set_chat_permissions.as_ref())
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

  pub fn set_chat_description<C: AsRef<SetChatDescription>>(&self, set_chat_description: C) -> RTDResult<()> {
    self.send(set_chat_description.as_ref())
  }

  pub fn pin_chat_message<C: AsRef<PinChatMessage>>(&self, pin_chat_message: C) -> RTDResult<()> {
    self.send(pin_chat_message.as_ref())
  }

  pub fn unpin_chat_message<C: AsRef<UnpinChatMessage>>(&self, unpin_chat_message: C) -> RTDResult<()> {
    self.send(unpin_chat_message.as_ref())
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

  pub fn get_chat_member<C: AsRef<GetChatMember>>(&self, get_chat_member: C) -> RTDResult<()> {
    self.send(get_chat_member.as_ref())
  }

  pub fn search_chat_members<C: AsRef<SearchChatMembers>>(&self, search_chat_members: C) -> RTDResult<()> {
    self.send(search_chat_members.as_ref())
  }

  pub fn get_chat_administrators<C: AsRef<GetChatAdministrators>>(&self, get_chat_administrators: C) -> RTDResult<()> {
    self.send(get_chat_administrators.as_ref())
  }

  pub fn clear_all_draft_messages<C: AsRef<ClearAllDraftMessages>>(&self, clear_all_draft_messages: C) -> RTDResult<()> {
    self.send(clear_all_draft_messages.as_ref())
  }

  pub fn get_chat_notification_settings_exceptions<C: AsRef<GetChatNotificationSettingsExceptions>>(&self, get_chat_notification_settings_exceptions: C) -> RTDResult<()> {
    self.send(get_chat_notification_settings_exceptions.as_ref())
  }

  pub fn get_scope_notification_settings<C: AsRef<GetScopeNotificationSettings>>(&self, get_scope_notification_settings: C) -> RTDResult<()> {
    self.send(get_scope_notification_settings.as_ref())
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

  pub fn download_file<C: AsRef<DownloadFile>>(&self, download_file: C) -> RTDResult<()> {
    self.send(download_file.as_ref())
  }

  pub fn get_file_downloaded_prefix_size<C: AsRef<GetFileDownloadedPrefixSize>>(&self, get_file_downloaded_prefix_size: C) -> RTDResult<()> {
    self.send(get_file_downloaded_prefix_size.as_ref())
  }

  pub fn cancel_download_file<C: AsRef<CancelDownloadFile>>(&self, cancel_download_file: C) -> RTDResult<()> {
    self.send(cancel_download_file.as_ref())
  }

  pub fn upload_file<C: AsRef<UploadFile>>(&self, upload_file: C) -> RTDResult<()> {
    self.send(upload_file.as_ref())
  }

  pub fn cancel_upload_file<C: AsRef<CancelUploadFile>>(&self, cancel_upload_file: C) -> RTDResult<()> {
    self.send(cancel_upload_file.as_ref())
  }

  pub fn write_generated_file_part<C: AsRef<WriteGeneratedFilePart>>(&self, write_generated_file_part: C) -> RTDResult<()> {
    self.send(write_generated_file_part.as_ref())
  }

  pub fn set_file_generation_progress<C: AsRef<SetFileGenerationProgress>>(&self, set_file_generation_progress: C) -> RTDResult<()> {
    self.send(set_file_generation_progress.as_ref())
  }

  pub fn finish_file_generation<C: AsRef<FinishFileGeneration>>(&self, finish_file_generation: C) -> RTDResult<()> {
    self.send(finish_file_generation.as_ref())
  }

  pub fn read_file_part<C: AsRef<ReadFilePart>>(&self, read_file_part: C) -> RTDResult<()> {
    self.send(read_file_part.as_ref())
  }

  pub fn delete_file<C: AsRef<DeleteFile>>(&self, delete_file: C) -> RTDResult<()> {
    self.send(delete_file.as_ref())
  }

  pub fn generate_chat_invite_link<C: AsRef<GenerateChatInviteLink>>(&self, generate_chat_invite_link: C) -> RTDResult<()> {
    self.send(generate_chat_invite_link.as_ref())
  }

  pub fn check_chat_invite_link<C: AsRef<CheckChatInviteLink>>(&self, check_chat_invite_link: C) -> RTDResult<()> {
    self.send(check_chat_invite_link.as_ref())
  }

  pub fn join_chat_by_invite_link<C: AsRef<JoinChatByInviteLink>>(&self, join_chat_by_invite_link: C) -> RTDResult<()> {
    self.send(join_chat_by_invite_link.as_ref())
  }

  pub fn create_call<C: AsRef<CreateCall>>(&self, create_call: C) -> RTDResult<()> {
    self.send(create_call.as_ref())
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

  pub fn get_blocked_users<C: AsRef<GetBlockedUsers>>(&self, get_blocked_users: C) -> RTDResult<()> {
    self.send(get_blocked_users.as_ref())
  }

  pub fn import_contacts<C: AsRef<ImportContacts>>(&self, import_contacts: C) -> RTDResult<()> {
    self.send(import_contacts.as_ref())
  }

  pub fn get_contacts<C: AsRef<GetContacts>>(&self, get_contacts: C) -> RTDResult<()> {
    self.send(get_contacts.as_ref())
  }

  pub fn search_contacts<C: AsRef<SearchContacts>>(&self, search_contacts: C) -> RTDResult<()> {
    self.send(search_contacts.as_ref())
  }

  pub fn remove_contacts<C: AsRef<RemoveContacts>>(&self, remove_contacts: C) -> RTDResult<()> {
    self.send(remove_contacts.as_ref())
  }

  pub fn get_imported_contact_count<C: AsRef<GetImportedContactCount>>(&self, get_imported_contact_count: C) -> RTDResult<()> {
    self.send(get_imported_contact_count.as_ref())
  }

  pub fn change_imported_contacts<C: AsRef<ChangeImportedContacts>>(&self, change_imported_contacts: C) -> RTDResult<()> {
    self.send(change_imported_contacts.as_ref())
  }

  pub fn clear_imported_contacts<C: AsRef<ClearImportedContacts>>(&self, clear_imported_contacts: C) -> RTDResult<()> {
    self.send(clear_imported_contacts.as_ref())
  }

  pub fn get_user_profile_photos<C: AsRef<GetUserProfilePhotos>>(&self, get_user_profile_photos: C) -> RTDResult<()> {
    self.send(get_user_profile_photos.as_ref())
  }

  pub fn get_stickers<C: AsRef<GetStickers>>(&self, get_stickers: C) -> RTDResult<()> {
    self.send(get_stickers.as_ref())
  }

  pub fn search_stickers<C: AsRef<SearchStickers>>(&self, search_stickers: C) -> RTDResult<()> {
    self.send(search_stickers.as_ref())
  }

  pub fn get_installed_sticker_sets<C: AsRef<GetInstalledStickerSets>>(&self, get_installed_sticker_sets: C) -> RTDResult<()> {
    self.send(get_installed_sticker_sets.as_ref())
  }

  pub fn get_archived_sticker_sets<C: AsRef<GetArchivedStickerSets>>(&self, get_archived_sticker_sets: C) -> RTDResult<()> {
    self.send(get_archived_sticker_sets.as_ref())
  }

  pub fn get_trending_sticker_sets<C: AsRef<GetTrendingStickerSets>>(&self, get_trending_sticker_sets: C) -> RTDResult<()> {
    self.send(get_trending_sticker_sets.as_ref())
  }

  pub fn get_attached_sticker_sets<C: AsRef<GetAttachedStickerSets>>(&self, get_attached_sticker_sets: C) -> RTDResult<()> {
    self.send(get_attached_sticker_sets.as_ref())
  }

  pub fn get_sticker_set<C: AsRef<GetStickerSet>>(&self, get_sticker_set: C) -> RTDResult<()> {
    self.send(get_sticker_set.as_ref())
  }

  pub fn search_sticker_set<C: AsRef<SearchStickerSet>>(&self, search_sticker_set: C) -> RTDResult<()> {
    self.send(search_sticker_set.as_ref())
  }

  pub fn search_installed_sticker_sets<C: AsRef<SearchInstalledStickerSets>>(&self, search_installed_sticker_sets: C) -> RTDResult<()> {
    self.send(search_installed_sticker_sets.as_ref())
  }

  pub fn search_sticker_sets<C: AsRef<SearchStickerSets>>(&self, search_sticker_sets: C) -> RTDResult<()> {
    self.send(search_sticker_sets.as_ref())
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

  pub fn get_recent_stickers<C: AsRef<GetRecentStickers>>(&self, get_recent_stickers: C) -> RTDResult<()> {
    self.send(get_recent_stickers.as_ref())
  }

  pub fn add_recent_sticker<C: AsRef<AddRecentSticker>>(&self, add_recent_sticker: C) -> RTDResult<()> {
    self.send(add_recent_sticker.as_ref())
  }

  pub fn remove_recent_sticker<C: AsRef<RemoveRecentSticker>>(&self, remove_recent_sticker: C) -> RTDResult<()> {
    self.send(remove_recent_sticker.as_ref())
  }

  pub fn clear_recent_stickers<C: AsRef<ClearRecentStickers>>(&self, clear_recent_stickers: C) -> RTDResult<()> {
    self.send(clear_recent_stickers.as_ref())
  }

  pub fn get_favorite_stickers<C: AsRef<GetFavoriteStickers>>(&self, get_favorite_stickers: C) -> RTDResult<()> {
    self.send(get_favorite_stickers.as_ref())
  }

  pub fn add_favorite_sticker<C: AsRef<AddFavoriteSticker>>(&self, add_favorite_sticker: C) -> RTDResult<()> {
    self.send(add_favorite_sticker.as_ref())
  }

  pub fn remove_favorite_sticker<C: AsRef<RemoveFavoriteSticker>>(&self, remove_favorite_sticker: C) -> RTDResult<()> {
    self.send(remove_favorite_sticker.as_ref())
  }

  pub fn get_sticker_emojis<C: AsRef<GetStickerEmojis>>(&self, get_sticker_emojis: C) -> RTDResult<()> {
    self.send(get_sticker_emojis.as_ref())
  }

  pub fn search_emojis<C: AsRef<SearchEmojis>>(&self, search_emojis: C) -> RTDResult<()> {
    self.send(search_emojis.as_ref())
  }

  pub fn get_emoji_suggestions_url<C: AsRef<GetEmojiSuggestionsUrl>>(&self, get_emoji_suggestions_url: C) -> RTDResult<()> {
    self.send(get_emoji_suggestions_url.as_ref())
  }

  pub fn get_saved_animations<C: AsRef<GetSavedAnimations>>(&self, get_saved_animations: C) -> RTDResult<()> {
    self.send(get_saved_animations.as_ref())
  }

  pub fn add_saved_animation<C: AsRef<AddSavedAnimation>>(&self, add_saved_animation: C) -> RTDResult<()> {
    self.send(add_saved_animation.as_ref())
  }

  pub fn remove_saved_animation<C: AsRef<RemoveSavedAnimation>>(&self, remove_saved_animation: C) -> RTDResult<()> {
    self.send(remove_saved_animation.as_ref())
  }

  pub fn get_recent_inline_bots<C: AsRef<GetRecentInlineBots>>(&self, get_recent_inline_bots: C) -> RTDResult<()> {
    self.send(get_recent_inline_bots.as_ref())
  }

  pub fn search_hashtags<C: AsRef<SearchHashtags>>(&self, search_hashtags: C) -> RTDResult<()> {
    self.send(search_hashtags.as_ref())
  }

  pub fn remove_recent_hashtag<C: AsRef<RemoveRecentHashtag>>(&self, remove_recent_hashtag: C) -> RTDResult<()> {
    self.send(remove_recent_hashtag.as_ref())
  }

  pub fn get_web_page_preview<C: AsRef<GetWebPagePreview>>(&self, get_web_page_preview: C) -> RTDResult<()> {
    self.send(get_web_page_preview.as_ref())
  }

  pub fn get_web_page_instant_view<C: AsRef<GetWebPageInstantView>>(&self, get_web_page_instant_view: C) -> RTDResult<()> {
    self.send(get_web_page_instant_view.as_ref())
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

  pub fn change_phone_number<C: AsRef<ChangePhoneNumber>>(&self, change_phone_number: C) -> RTDResult<()> {
    self.send(change_phone_number.as_ref())
  }

  pub fn resend_change_phone_number_code<C: AsRef<ResendChangePhoneNumberCode>>(&self, resend_change_phone_number_code: C) -> RTDResult<()> {
    self.send(resend_change_phone_number_code.as_ref())
  }

  pub fn check_change_phone_number_code<C: AsRef<CheckChangePhoneNumberCode>>(&self, check_change_phone_number_code: C) -> RTDResult<()> {
    self.send(check_change_phone_number_code.as_ref())
  }

  pub fn get_active_sessions<C: AsRef<GetActiveSessions>>(&self, get_active_sessions: C) -> RTDResult<()> {
    self.send(get_active_sessions.as_ref())
  }

  pub fn terminate_session<C: AsRef<TerminateSession>>(&self, terminate_session: C) -> RTDResult<()> {
    self.send(terminate_session.as_ref())
  }

  pub fn terminate_all_other_sessions<C: AsRef<TerminateAllOtherSessions>>(&self, terminate_all_other_sessions: C) -> RTDResult<()> {
    self.send(terminate_all_other_sessions.as_ref())
  }

  pub fn get_connected_websites<C: AsRef<GetConnectedWebsites>>(&self, get_connected_websites: C) -> RTDResult<()> {
    self.send(get_connected_websites.as_ref())
  }

  pub fn disconnect_website<C: AsRef<DisconnectWebsite>>(&self, disconnect_website: C) -> RTDResult<()> {
    self.send(disconnect_website.as_ref())
  }

  pub fn disconnect_all_websites<C: AsRef<DisconnectAllWebsites>>(&self, disconnect_all_websites: C) -> RTDResult<()> {
    self.send(disconnect_all_websites.as_ref())
  }

  pub fn set_supergroup_username<C: AsRef<SetSupergroupUsername>>(&self, set_supergroup_username: C) -> RTDResult<()> {
    self.send(set_supergroup_username.as_ref())
  }

  pub fn set_supergroup_sticker_set<C: AsRef<SetSupergroupStickerSet>>(&self, set_supergroup_sticker_set: C) -> RTDResult<()> {
    self.send(set_supergroup_sticker_set.as_ref())
  }

  pub fn toggle_supergroup_sign_messages<C: AsRef<ToggleSupergroupSignMessages>>(&self, toggle_supergroup_sign_messages: C) -> RTDResult<()> {
    self.send(toggle_supergroup_sign_messages.as_ref())
  }

  pub fn toggle_supergroup_is_all_history_available<C: AsRef<ToggleSupergroupIsAllHistoryAvailable>>(&self, toggle_supergroup_is_all_history_available: C) -> RTDResult<()> {
    self.send(toggle_supergroup_is_all_history_available.as_ref())
  }

  pub fn report_supergroup_spam<C: AsRef<ReportSupergroupSpam>>(&self, report_supergroup_spam: C) -> RTDResult<()> {
    self.send(report_supergroup_spam.as_ref())
  }

  pub fn get_supergroup_members<C: AsRef<GetSupergroupMembers>>(&self, get_supergroup_members: C) -> RTDResult<()> {
    self.send(get_supergroup_members.as_ref())
  }

  pub fn delete_supergroup<C: AsRef<DeleteSupergroup>>(&self, delete_supergroup: C) -> RTDResult<()> {
    self.send(delete_supergroup.as_ref())
  }

  pub fn close_secret_chat<C: AsRef<CloseSecretChat>>(&self, close_secret_chat: C) -> RTDResult<()> {
    self.send(close_secret_chat.as_ref())
  }

  pub fn get_chat_event_log<C: AsRef<GetChatEventLog>>(&self, get_chat_event_log: C) -> RTDResult<()> {
    self.send(get_chat_event_log.as_ref())
  }

  pub fn get_payment_form<C: AsRef<GetPaymentForm>>(&self, get_payment_form: C) -> RTDResult<()> {
    self.send(get_payment_form.as_ref())
  }

  pub fn validate_order_info<C: AsRef<ValidateOrderInfo>>(&self, validate_order_info: C) -> RTDResult<()> {
    self.send(validate_order_info.as_ref())
  }

  pub fn send_payment_form<C: AsRef<SendPaymentForm>>(&self, send_payment_form: C) -> RTDResult<()> {
    self.send(send_payment_form.as_ref())
  }

  pub fn get_payment_receipt<C: AsRef<GetPaymentReceipt>>(&self, get_payment_receipt: C) -> RTDResult<()> {
    self.send(get_payment_receipt.as_ref())
  }

  pub fn get_saved_order_info<C: AsRef<GetSavedOrderInfo>>(&self, get_saved_order_info: C) -> RTDResult<()> {
    self.send(get_saved_order_info.as_ref())
  }

  pub fn delete_saved_order_info<C: AsRef<DeleteSavedOrderInfo>>(&self, delete_saved_order_info: C) -> RTDResult<()> {
    self.send(delete_saved_order_info.as_ref())
  }

  pub fn delete_saved_credentials<C: AsRef<DeleteSavedCredentials>>(&self, delete_saved_credentials: C) -> RTDResult<()> {
    self.send(delete_saved_credentials.as_ref())
  }

  pub fn get_support_user<C: AsRef<GetSupportUser>>(&self, get_support_user: C) -> RTDResult<()> {
    self.send(get_support_user.as_ref())
  }

  pub fn get_backgrounds<C: AsRef<GetBackgrounds>>(&self, get_backgrounds: C) -> RTDResult<()> {
    self.send(get_backgrounds.as_ref())
  }

  pub fn get_background_url<C: AsRef<GetBackgroundUrl>>(&self, get_background_url: C) -> RTDResult<()> {
    self.send(get_background_url.as_ref())
  }

  pub fn search_background<C: AsRef<SearchBackground>>(&self, search_background: C) -> RTDResult<()> {
    self.send(search_background.as_ref())
  }

  pub fn set_background<C: AsRef<SetBackground>>(&self, set_background: C) -> RTDResult<()> {
    self.send(set_background.as_ref())
  }

  pub fn remove_background<C: AsRef<RemoveBackground>>(&self, remove_background: C) -> RTDResult<()> {
    self.send(remove_background.as_ref())
  }

  pub fn reset_backgrounds<C: AsRef<ResetBackgrounds>>(&self, reset_backgrounds: C) -> RTDResult<()> {
    self.send(reset_backgrounds.as_ref())
  }

  pub fn get_localization_target_info<C: AsRef<GetLocalizationTargetInfo>>(&self, get_localization_target_info: C) -> RTDResult<()> {
    self.send(get_localization_target_info.as_ref())
  }

  pub fn get_language_pack_info<C: AsRef<GetLanguagePackInfo>>(&self, get_language_pack_info: C) -> RTDResult<()> {
    self.send(get_language_pack_info.as_ref())
  }

  pub fn get_language_pack_strings<C: AsRef<GetLanguagePackStrings>>(&self, get_language_pack_strings: C) -> RTDResult<()> {
    self.send(get_language_pack_strings.as_ref())
  }

  pub fn synchronize_language_pack<C: AsRef<SynchronizeLanguagePack>>(&self, synchronize_language_pack: C) -> RTDResult<()> {
    self.send(synchronize_language_pack.as_ref())
  }

  pub fn add_custom_server_language_pack<C: AsRef<AddCustomServerLanguagePack>>(&self, add_custom_server_language_pack: C) -> RTDResult<()> {
    self.send(add_custom_server_language_pack.as_ref())
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

  pub fn process_push_notification<C: AsRef<ProcessPushNotification>>(&self, process_push_notification: C) -> RTDResult<()> {
    self.send(process_push_notification.as_ref())
  }

  pub fn get_push_receiver_id<C: AsRef<GetPushReceiverId>>(&self, get_push_receiver_id: C) -> RTDResult<()> {
    self.send(get_push_receiver_id.as_ref())
  }

  pub fn get_recently_visited_t_me_urls<C: AsRef<GetRecentlyVisitedTMeUrls>>(&self, get_recently_visited_t_me_urls: C) -> RTDResult<()> {
    self.send(get_recently_visited_t_me_urls.as_ref())
  }

  pub fn set_user_privacy_setting_rules<C: AsRef<SetUserPrivacySettingRules>>(&self, set_user_privacy_setting_rules: C) -> RTDResult<()> {
    self.send(set_user_privacy_setting_rules.as_ref())
  }

  pub fn get_user_privacy_setting_rules<C: AsRef<GetUserPrivacySettingRules>>(&self, get_user_privacy_setting_rules: C) -> RTDResult<()> {
    self.send(get_user_privacy_setting_rules.as_ref())
  }

  pub fn get_option<C: AsRef<GetOption>>(&self, get_option: C) -> RTDResult<()> {
    self.send(get_option.as_ref())
  }

  pub fn set_option<C: AsRef<SetOption>>(&self, set_option: C) -> RTDResult<()> {
    self.send(set_option.as_ref())
  }

  pub fn set_account_ttl<C: AsRef<SetAccountTtl>>(&self, set_account_ttl: C) -> RTDResult<()> {
    self.send(set_account_ttl.as_ref())
  }

  pub fn get_account_ttl<C: AsRef<GetAccountTtl>>(&self, get_account_ttl: C) -> RTDResult<()> {
    self.send(get_account_ttl.as_ref())
  }

  pub fn delete_account<C: AsRef<DeleteAccount>>(&self, delete_account: C) -> RTDResult<()> {
    self.send(delete_account.as_ref())
  }

  pub fn get_chat_report_spam_state<C: AsRef<GetChatReportSpamState>>(&self, get_chat_report_spam_state: C) -> RTDResult<()> {
    self.send(get_chat_report_spam_state.as_ref())
  }

  pub fn change_chat_report_spam_state<C: AsRef<ChangeChatReportSpamState>>(&self, change_chat_report_spam_state: C) -> RTDResult<()> {
    self.send(change_chat_report_spam_state.as_ref())
  }

  pub fn report_chat<C: AsRef<ReportChat>>(&self, report_chat: C) -> RTDResult<()> {
    self.send(report_chat.as_ref())
  }

  pub fn get_chat_statistics_url<C: AsRef<GetChatStatisticsUrl>>(&self, get_chat_statistics_url: C) -> RTDResult<()> {
    self.send(get_chat_statistics_url.as_ref())
  }

  pub fn get_storage_statistics<C: AsRef<GetStorageStatistics>>(&self, get_storage_statistics: C) -> RTDResult<()> {
    self.send(get_storage_statistics.as_ref())
  }

  pub fn get_storage_statistics_fast<C: AsRef<GetStorageStatisticsFast>>(&self, get_storage_statistics_fast: C) -> RTDResult<()> {
    self.send(get_storage_statistics_fast.as_ref())
  }

  pub fn get_database_statistics<C: AsRef<GetDatabaseStatistics>>(&self, get_database_statistics: C) -> RTDResult<()> {
    self.send(get_database_statistics.as_ref())
  }

  pub fn optimize_storage<C: AsRef<OptimizeStorage>>(&self, optimize_storage: C) -> RTDResult<()> {
    self.send(optimize_storage.as_ref())
  }

  pub fn set_network_type<C: AsRef<SetNetworkType>>(&self, set_network_type: C) -> RTDResult<()> {
    self.send(set_network_type.as_ref())
  }

  pub fn get_network_statistics<C: AsRef<GetNetworkStatistics>>(&self, get_network_statistics: C) -> RTDResult<()> {
    self.send(get_network_statistics.as_ref())
  }

  pub fn add_network_statistics<C: AsRef<AddNetworkStatistics>>(&self, add_network_statistics: C) -> RTDResult<()> {
    self.send(add_network_statistics.as_ref())
  }

  pub fn reset_network_statistics<C: AsRef<ResetNetworkStatistics>>(&self, reset_network_statistics: C) -> RTDResult<()> {
    self.send(reset_network_statistics.as_ref())
  }

  pub fn get_auto_download_settings_presets<C: AsRef<GetAutoDownloadSettingsPresets>>(&self, get_auto_download_settings_presets: C) -> RTDResult<()> {
    self.send(get_auto_download_settings_presets.as_ref())
  }

  pub fn set_auto_download_settings<C: AsRef<SetAutoDownloadSettings>>(&self, set_auto_download_settings: C) -> RTDResult<()> {
    self.send(set_auto_download_settings.as_ref())
  }

  pub fn get_passport_element<C: AsRef<GetPassportElement>>(&self, get_passport_element: C) -> RTDResult<()> {
    self.send(get_passport_element.as_ref())
  }

  pub fn get_all_passport_elements<C: AsRef<GetAllPassportElements>>(&self, get_all_passport_elements: C) -> RTDResult<()> {
    self.send(get_all_passport_elements.as_ref())
  }

  pub fn set_passport_element<C: AsRef<SetPassportElement>>(&self, set_passport_element: C) -> RTDResult<()> {
    self.send(set_passport_element.as_ref())
  }

  pub fn delete_passport_element<C: AsRef<DeletePassportElement>>(&self, delete_passport_element: C) -> RTDResult<()> {
    self.send(delete_passport_element.as_ref())
  }

  pub fn set_passport_element_errors<C: AsRef<SetPassportElementErrors>>(&self, set_passport_element_errors: C) -> RTDResult<()> {
    self.send(set_passport_element_errors.as_ref())
  }

  pub fn get_preferred_country_language<C: AsRef<GetPreferredCountryLanguage>>(&self, get_preferred_country_language: C) -> RTDResult<()> {
    self.send(get_preferred_country_language.as_ref())
  }

  pub fn send_phone_number_verification_code<C: AsRef<SendPhoneNumberVerificationCode>>(&self, send_phone_number_verification_code: C) -> RTDResult<()> {
    self.send(send_phone_number_verification_code.as_ref())
  }

  pub fn resend_phone_number_verification_code<C: AsRef<ResendPhoneNumberVerificationCode>>(&self, resend_phone_number_verification_code: C) -> RTDResult<()> {
    self.send(resend_phone_number_verification_code.as_ref())
  }

  pub fn check_phone_number_verification_code<C: AsRef<CheckPhoneNumberVerificationCode>>(&self, check_phone_number_verification_code: C) -> RTDResult<()> {
    self.send(check_phone_number_verification_code.as_ref())
  }

  pub fn send_email_address_verification_code<C: AsRef<SendEmailAddressVerificationCode>>(&self, send_email_address_verification_code: C) -> RTDResult<()> {
    self.send(send_email_address_verification_code.as_ref())
  }

  pub fn resend_email_address_verification_code<C: AsRef<ResendEmailAddressVerificationCode>>(&self, resend_email_address_verification_code: C) -> RTDResult<()> {
    self.send(resend_email_address_verification_code.as_ref())
  }

  pub fn check_email_address_verification_code<C: AsRef<CheckEmailAddressVerificationCode>>(&self, check_email_address_verification_code: C) -> RTDResult<()> {
    self.send(check_email_address_verification_code.as_ref())
  }

  pub fn get_passport_authorization_form<C: AsRef<GetPassportAuthorizationForm>>(&self, get_passport_authorization_form: C) -> RTDResult<()> {
    self.send(get_passport_authorization_form.as_ref())
  }

  pub fn get_passport_authorization_form_available_elements<C: AsRef<GetPassportAuthorizationFormAvailableElements>>(&self, get_passport_authorization_form_available_elements: C) -> RTDResult<()> {
    self.send(get_passport_authorization_form_available_elements.as_ref())
  }

  pub fn send_passport_authorization_form<C: AsRef<SendPassportAuthorizationForm>>(&self, send_passport_authorization_form: C) -> RTDResult<()> {
    self.send(send_passport_authorization_form.as_ref())
  }

  pub fn send_phone_number_confirmation_code<C: AsRef<SendPhoneNumberConfirmationCode>>(&self, send_phone_number_confirmation_code: C) -> RTDResult<()> {
    self.send(send_phone_number_confirmation_code.as_ref())
  }

  pub fn resend_phone_number_confirmation_code<C: AsRef<ResendPhoneNumberConfirmationCode>>(&self, resend_phone_number_confirmation_code: C) -> RTDResult<()> {
    self.send(resend_phone_number_confirmation_code.as_ref())
  }

  pub fn check_phone_number_confirmation_code<C: AsRef<CheckPhoneNumberConfirmationCode>>(&self, check_phone_number_confirmation_code: C) -> RTDResult<()> {
    self.send(check_phone_number_confirmation_code.as_ref())
  }

  pub fn set_bot_updates_status<C: AsRef<SetBotUpdatesStatus>>(&self, set_bot_updates_status: C) -> RTDResult<()> {
    self.send(set_bot_updates_status.as_ref())
  }

  pub fn upload_sticker_file<C: AsRef<UploadStickerFile>>(&self, upload_sticker_file: C) -> RTDResult<()> {
    self.send(upload_sticker_file.as_ref())
  }

  pub fn create_new_sticker_set<C: AsRef<CreateNewStickerSet>>(&self, create_new_sticker_set: C) -> RTDResult<()> {
    self.send(create_new_sticker_set.as_ref())
  }

  pub fn add_sticker_to_set<C: AsRef<AddStickerToSet>>(&self, add_sticker_to_set: C) -> RTDResult<()> {
    self.send(add_sticker_to_set.as_ref())
  }

  pub fn set_sticker_position_in_set<C: AsRef<SetStickerPositionInSet>>(&self, set_sticker_position_in_set: C) -> RTDResult<()> {
    self.send(set_sticker_position_in_set.as_ref())
  }

  pub fn remove_sticker_from_set<C: AsRef<RemoveStickerFromSet>>(&self, remove_sticker_from_set: C) -> RTDResult<()> {
    self.send(remove_sticker_from_set.as_ref())
  }

  pub fn get_map_thumbnail_file<C: AsRef<GetMapThumbnailFile>>(&self, get_map_thumbnail_file: C) -> RTDResult<()> {
    self.send(get_map_thumbnail_file.as_ref())
  }

  pub fn accept_terms_of_service<C: AsRef<AcceptTermsOfService>>(&self, accept_terms_of_service: C) -> RTDResult<()> {
    self.send(accept_terms_of_service.as_ref())
  }

  pub fn send_custom_request<C: AsRef<SendCustomRequest>>(&self, send_custom_request: C) -> RTDResult<()> {
    self.send(send_custom_request.as_ref())
  }

  pub fn answer_custom_query<C: AsRef<AnswerCustomQuery>>(&self, answer_custom_query: C) -> RTDResult<()> {
    self.send(answer_custom_query.as_ref())
  }

  pub fn send_ton_lite_server_request<C: AsRef<SendTonLiteServerRequest>>(&self, send_ton_lite_server_request: C) -> RTDResult<()> {
    self.send(send_ton_lite_server_request.as_ref())
  }

  pub fn get_ton_wallet_password_salt<C: AsRef<GetTonWalletPasswordSalt>>(&self, get_ton_wallet_password_salt: C) -> RTDResult<()> {
    self.send(get_ton_wallet_password_salt.as_ref())
  }

  pub fn set_alarm<C: AsRef<SetAlarm>>(&self, set_alarm: C) -> RTDResult<()> {
    self.send(set_alarm.as_ref())
  }

  pub fn get_country_code<C: AsRef<GetCountryCode>>(&self, get_country_code: C) -> RTDResult<()> {
    self.send(get_country_code.as_ref())
  }

  pub fn get_invite_text<C: AsRef<GetInviteText>>(&self, get_invite_text: C) -> RTDResult<()> {
    self.send(get_invite_text.as_ref())
  }

  pub fn get_deep_link_info<C: AsRef<GetDeepLinkInfo>>(&self, get_deep_link_info: C) -> RTDResult<()> {
    self.send(get_deep_link_info.as_ref())
  }

  pub fn get_application_config<C: AsRef<GetApplicationConfig>>(&self, get_application_config: C) -> RTDResult<()> {
    self.send(get_application_config.as_ref())
  }

  pub fn save_application_log_event<C: AsRef<SaveApplicationLogEvent>>(&self, save_application_log_event: C) -> RTDResult<()> {
    self.send(save_application_log_event.as_ref())
  }

  pub fn add_proxy<C: AsRef<AddProxy>>(&self, add_proxy: C) -> RTDResult<()> {
    self.send(add_proxy.as_ref())
  }

  pub fn edit_proxy<C: AsRef<EditProxy>>(&self, edit_proxy: C) -> RTDResult<()> {
    self.send(edit_proxy.as_ref())
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

  pub fn get_proxies<C: AsRef<GetProxies>>(&self, get_proxies: C) -> RTDResult<()> {
    self.send(get_proxies.as_ref())
  }

  pub fn get_proxy_link<C: AsRef<GetProxyLink>>(&self, get_proxy_link: C) -> RTDResult<()> {
    self.send(get_proxy_link.as_ref())
  }

  pub fn ping_proxy<C: AsRef<PingProxy>>(&self, ping_proxy: C) -> RTDResult<()> {
    self.send(ping_proxy.as_ref())
  }

  pub fn set_log_stream<C: AsRef<SetLogStream>>(&self, set_log_stream: C) -> RTDResult<()> {
    self.send(set_log_stream.as_ref())
  }

  pub fn get_log_stream<C: AsRef<GetLogStream>>(&self, get_log_stream: C) -> RTDResult<()> {
    self.send(get_log_stream.as_ref())
  }

  pub fn set_log_verbosity_level<C: AsRef<SetLogVerbosityLevel>>(&self, set_log_verbosity_level: C) -> RTDResult<()> {
    self.send(set_log_verbosity_level.as_ref())
  }

  pub fn get_log_verbosity_level<C: AsRef<GetLogVerbosityLevel>>(&self, get_log_verbosity_level: C) -> RTDResult<()> {
    self.send(get_log_verbosity_level.as_ref())
  }

  pub fn get_log_tags<C: AsRef<GetLogTags>>(&self, get_log_tags: C) -> RTDResult<()> {
    self.send(get_log_tags.as_ref())
  }

  pub fn set_log_tag_verbosity_level<C: AsRef<SetLogTagVerbosityLevel>>(&self, set_log_tag_verbosity_level: C) -> RTDResult<()> {
    self.send(set_log_tag_verbosity_level.as_ref())
  }

  pub fn get_log_tag_verbosity_level<C: AsRef<GetLogTagVerbosityLevel>>(&self, get_log_tag_verbosity_level: C) -> RTDResult<()> {
    self.send(get_log_tag_verbosity_level.as_ref())
  }

  pub fn add_log_message<C: AsRef<AddLogMessage>>(&self, add_log_message: C) -> RTDResult<()> {
    self.send(add_log_message.as_ref())
  }

  pub fn test_call_empty<C: AsRef<TestCallEmpty>>(&self, test_call_empty: C) -> RTDResult<()> {
    self.send(test_call_empty.as_ref())
  }

  pub fn test_call_string<C: AsRef<TestCallString>>(&self, test_call_string: C) -> RTDResult<()> {
    self.send(test_call_string.as_ref())
  }

  pub fn test_call_bytes<C: AsRef<TestCallBytes>>(&self, test_call_bytes: C) -> RTDResult<()> {
    self.send(test_call_bytes.as_ref())
  }

  pub fn test_call_vector_int<C: AsRef<TestCallVectorInt>>(&self, test_call_vector_int: C) -> RTDResult<()> {
    self.send(test_call_vector_int.as_ref())
  }

  pub fn test_call_vector_int_object<C: AsRef<TestCallVectorIntObject>>(&self, test_call_vector_int_object: C) -> RTDResult<()> {
    self.send(test_call_vector_int_object.as_ref())
  }

  pub fn test_call_vector_string<C: AsRef<TestCallVectorString>>(&self, test_call_vector_string: C) -> RTDResult<()> {
    self.send(test_call_vector_string.as_ref())
  }

  pub fn test_call_vector_string_object<C: AsRef<TestCallVectorStringObject>>(&self, test_call_vector_string_object: C) -> RTDResult<()> {
    self.send(test_call_vector_string_object.as_ref())
  }

  pub fn test_square_int<C: AsRef<TestSquareInt>>(&self, test_square_int: C) -> RTDResult<()> {
    self.send(test_square_int.as_ref())
  }

  pub fn test_network<C: AsRef<TestNetwork>>(&self, test_network: C) -> RTDResult<()> {
    self.send(test_network.as_ref())
  }

  pub fn test_proxy<C: AsRef<TestProxy>>(&self, test_proxy: C) -> RTDResult<()> {
    self.send(test_proxy.as_ref())
  }

  pub fn test_get_difference<C: AsRef<TestGetDifference>>(&self, test_get_difference: C) -> RTDResult<()> {
    self.send(test_get_difference.as_ref())
  }

  pub fn test_use_update<C: AsRef<TestUseUpdate>>(&self, test_use_update: C) -> RTDResult<()> {
    self.send(test_use_update.as_ref())
  }

  pub fn test_return_error<C: AsRef<TestReturnError>>(&self, test_return_error: C) -> RTDResult<()> {
    self.send(test_return_error.as_ref())
  }



}

