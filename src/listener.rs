use std::sync::Arc;

use rtdlib::types::*;
use crate::errors::*;
use crate::api::Api;


/// Telegram client event listener
#[derive(Clone, Default)]
pub struct Listener {
  exception: Option<Arc<dyn Fn((&Api, &TGError)) + Send + Sync + 'static>>,
  receive: Option<Arc<dyn Fn((&Api, &String)) -> TGResult<()> + Send + Sync + 'static>>,



  test_use_update: Option<Arc<dyn Fn((&Api, &TestUseUpdate)) -> TGResult<()> + Send + Sync + 'static>>,
  update_authorization_state: Option<Arc<dyn Fn((&Api, &UpdateAuthorizationState)) -> TGResult<()> + Send + Sync + 'static>>,
  update_basic_group: Option<Arc<dyn Fn((&Api, &UpdateBasicGroup)) -> TGResult<()> + Send + Sync + 'static>>,
  update_basic_group_full_info: Option<Arc<dyn Fn((&Api, &UpdateBasicGroupFullInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  update_call: Option<Arc<dyn Fn((&Api, &UpdateCall)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_default_disable_notification: Option<Arc<dyn Fn((&Api, &UpdateChatDefaultDisableNotification)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_draft_message: Option<Arc<dyn Fn((&Api, &UpdateChatDraftMessage)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_is_marked_as_unread: Option<Arc<dyn Fn((&Api, &UpdateChatIsMarkedAsUnread)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_is_pinned: Option<Arc<dyn Fn((&Api, &UpdateChatIsPinned)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_is_sponsored: Option<Arc<dyn Fn((&Api, &UpdateChatIsSponsored)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_last_message: Option<Arc<dyn Fn((&Api, &UpdateChatLastMessage)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_notification_settings: Option<Arc<dyn Fn((&Api, &UpdateChatNotificationSettings)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_order: Option<Arc<dyn Fn((&Api, &UpdateChatOrder)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_photo: Option<Arc<dyn Fn((&Api, &UpdateChatPhoto)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_read_inbox: Option<Arc<dyn Fn((&Api, &UpdateChatReadInbox)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_read_outbox: Option<Arc<dyn Fn((&Api, &UpdateChatReadOutbox)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_reply_markup: Option<Arc<dyn Fn((&Api, &UpdateChatReplyMarkup)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_title: Option<Arc<dyn Fn((&Api, &UpdateChatTitle)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_unread_mention_count: Option<Arc<dyn Fn((&Api, &UpdateChatUnreadMentionCount)) -> TGResult<()> + Send + Sync + 'static>>,
  update_connection_state: Option<Arc<dyn Fn((&Api, &UpdateConnectionState)) -> TGResult<()> + Send + Sync + 'static>>,
  update_delete_messages: Option<Arc<dyn Fn((&Api, &UpdateDeleteMessages)) -> TGResult<()> + Send + Sync + 'static>>,
  update_favorite_stickers: Option<Arc<dyn Fn((&Api, &UpdateFavoriteStickers)) -> TGResult<()> + Send + Sync + 'static>>,
  update_file: Option<Arc<dyn Fn((&Api, &UpdateFile)) -> TGResult<()> + Send + Sync + 'static>>,
  update_file_generation_start: Option<Arc<dyn Fn((&Api, &UpdateFileGenerationStart)) -> TGResult<()> + Send + Sync + 'static>>,
  update_file_generation_stop: Option<Arc<dyn Fn((&Api, &UpdateFileGenerationStop)) -> TGResult<()> + Send + Sync + 'static>>,
  update_installed_sticker_sets: Option<Arc<dyn Fn((&Api, &UpdateInstalledStickerSets)) -> TGResult<()> + Send + Sync + 'static>>,
  update_language_pack_strings: Option<Arc<dyn Fn((&Api, &UpdateLanguagePackStrings)) -> TGResult<()> + Send + Sync + 'static>>,
  update_message_content: Option<Arc<dyn Fn((&Api, &UpdateMessageContent)) -> TGResult<()> + Send + Sync + 'static>>,
  update_message_content_opened: Option<Arc<dyn Fn((&Api, &UpdateMessageContentOpened)) -> TGResult<()> + Send + Sync + 'static>>,
  update_message_edited: Option<Arc<dyn Fn((&Api, &UpdateMessageEdited)) -> TGResult<()> + Send + Sync + 'static>>,
  update_message_mention_read: Option<Arc<dyn Fn((&Api, &UpdateMessageMentionRead)) -> TGResult<()> + Send + Sync + 'static>>,
  update_message_send_acknowledged: Option<Arc<dyn Fn((&Api, &UpdateMessageSendAcknowledged)) -> TGResult<()> + Send + Sync + 'static>>,
  update_message_send_failed: Option<Arc<dyn Fn((&Api, &UpdateMessageSendFailed)) -> TGResult<()> + Send + Sync + 'static>>,
  update_message_send_succeeded: Option<Arc<dyn Fn((&Api, &UpdateMessageSendSucceeded)) -> TGResult<()> + Send + Sync + 'static>>,
  update_message_views: Option<Arc<dyn Fn((&Api, &UpdateMessageViews)) -> TGResult<()> + Send + Sync + 'static>>,
  update_new_callback_query: Option<Arc<dyn Fn((&Api, &UpdateNewCallbackQuery)) -> TGResult<()> + Send + Sync + 'static>>,
  update_new_chat: Option<Arc<dyn Fn((&Api, &UpdateNewChat)) -> TGResult<()> + Send + Sync + 'static>>,
  update_new_chosen_inline_result: Option<Arc<dyn Fn((&Api, &UpdateNewChosenInlineResult)) -> TGResult<()> + Send + Sync + 'static>>,
  update_new_custom_event: Option<Arc<dyn Fn((&Api, &UpdateNewCustomEvent)) -> TGResult<()> + Send + Sync + 'static>>,
  update_new_custom_query: Option<Arc<dyn Fn((&Api, &UpdateNewCustomQuery)) -> TGResult<()> + Send + Sync + 'static>>,
  update_new_inline_callback_query: Option<Arc<dyn Fn((&Api, &UpdateNewInlineCallbackQuery)) -> TGResult<()> + Send + Sync + 'static>>,
  update_new_inline_query: Option<Arc<dyn Fn((&Api, &UpdateNewInlineQuery)) -> TGResult<()> + Send + Sync + 'static>>,
  update_new_message: Option<Arc<dyn Fn((&Api, &UpdateNewMessage)) -> TGResult<()> + Send + Sync + 'static>>,
  update_new_pre_checkout_query: Option<Arc<dyn Fn((&Api, &UpdateNewPreCheckoutQuery)) -> TGResult<()> + Send + Sync + 'static>>,
  update_new_shipping_query: Option<Arc<dyn Fn((&Api, &UpdateNewShippingQuery)) -> TGResult<()> + Send + Sync + 'static>>,
  update_option: Option<Arc<dyn Fn((&Api, &UpdateOption)) -> TGResult<()> + Send + Sync + 'static>>,
  update_recent_stickers: Option<Arc<dyn Fn((&Api, &UpdateRecentStickers)) -> TGResult<()> + Send + Sync + 'static>>,
  update_saved_animations: Option<Arc<dyn Fn((&Api, &UpdateSavedAnimations)) -> TGResult<()> + Send + Sync + 'static>>,
  update_scope_notification_settings: Option<Arc<dyn Fn((&Api, &UpdateScopeNotificationSettings)) -> TGResult<()> + Send + Sync + 'static>>,
  update_secret_chat: Option<Arc<dyn Fn((&Api, &UpdateSecretChat)) -> TGResult<()> + Send + Sync + 'static>>,
  update_service_notification: Option<Arc<dyn Fn((&Api, &UpdateServiceNotification)) -> TGResult<()> + Send + Sync + 'static>>,
  update_supergroup: Option<Arc<dyn Fn((&Api, &UpdateSupergroup)) -> TGResult<()> + Send + Sync + 'static>>,
  update_supergroup_full_info: Option<Arc<dyn Fn((&Api, &UpdateSupergroupFullInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  update_terms_of_service: Option<Arc<dyn Fn((&Api, &UpdateTermsOfService)) -> TGResult<()> + Send + Sync + 'static>>,
  update_trending_sticker_sets: Option<Arc<dyn Fn((&Api, &UpdateTrendingStickerSets)) -> TGResult<()> + Send + Sync + 'static>>,
  update_unread_chat_count: Option<Arc<dyn Fn((&Api, &UpdateUnreadChatCount)) -> TGResult<()> + Send + Sync + 'static>>,
  update_unread_message_count: Option<Arc<dyn Fn((&Api, &UpdateUnreadMessageCount)) -> TGResult<()> + Send + Sync + 'static>>,
  update_user: Option<Arc<dyn Fn((&Api, &UpdateUser)) -> TGResult<()> + Send + Sync + 'static>>,
  update_user_chat_action: Option<Arc<dyn Fn((&Api, &UpdateUserChatAction)) -> TGResult<()> + Send + Sync + 'static>>,
  update_user_full_info: Option<Arc<dyn Fn((&Api, &UpdateUserFullInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  update_user_privacy_setting_rules: Option<Arc<dyn Fn((&Api, &UpdateUserPrivacySettingRules)) -> TGResult<()> + Send + Sync + 'static>>,
  update_user_status: Option<Arc<dyn Fn((&Api, &UpdateUserStatus)) -> TGResult<()> + Send + Sync + 'static>>,


  authorization_state: Option<Arc<dyn Fn((&Api, &AuthorizationState)) -> TGResult<()> + Send + Sync + 'static>>,
  check_chat_username_result: Option<Arc<dyn Fn((&Api, &CheckChatUsernameResult)) -> TGResult<()> + Send + Sync + 'static>>,
  language_pack_string_value: Option<Arc<dyn Fn((&Api, &LanguagePackStringValue)) -> TGResult<()> + Send + Sync + 'static>>,
  option_value: Option<Arc<dyn Fn((&Api, &OptionValue)) -> TGResult<()> + Send + Sync + 'static>>,
  passport_element: Option<Arc<dyn Fn((&Api, &PassportElement)) -> TGResult<()> + Send + Sync + 'static>>,
  update: Option<Arc<dyn Fn((&Api, &Update)) -> TGResult<()> + Send + Sync + 'static>>,
  account_ttl: Option<Arc<dyn Fn((&Api, &AccountTtl)) -> TGResult<()> + Send + Sync + 'static>>,
  animations: Option<Arc<dyn Fn((&Api, &Animations)) -> TGResult<()> + Send + Sync + 'static>>,
  authentication_code_info: Option<Arc<dyn Fn((&Api, &AuthenticationCodeInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  basic_group: Option<Arc<dyn Fn((&Api, &BasicGroup)) -> TGResult<()> + Send + Sync + 'static>>,
  basic_group_full_info: Option<Arc<dyn Fn((&Api, &BasicGroupFullInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  call_id: Option<Arc<dyn Fn((&Api, &CallId)) -> TGResult<()> + Send + Sync + 'static>>,
  callback_query_answer: Option<Arc<dyn Fn((&Api, &CallbackQueryAnswer)) -> TGResult<()> + Send + Sync + 'static>>,
  chat: Option<Arc<dyn Fn((&Api, &Chat)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_events: Option<Arc<dyn Fn((&Api, &ChatEvents)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_invite_link: Option<Arc<dyn Fn((&Api, &ChatInviteLink)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_invite_link_info: Option<Arc<dyn Fn((&Api, &ChatInviteLinkInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_member: Option<Arc<dyn Fn((&Api, &ChatMember)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_members: Option<Arc<dyn Fn((&Api, &ChatMembers)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_report_spam_state: Option<Arc<dyn Fn((&Api, &ChatReportSpamState)) -> TGResult<()> + Send + Sync + 'static>>,
  chats: Option<Arc<dyn Fn((&Api, &Chats)) -> TGResult<()> + Send + Sync + 'static>>,
  connected_websites: Option<Arc<dyn Fn((&Api, &ConnectedWebsites)) -> TGResult<()> + Send + Sync + 'static>>,
  count: Option<Arc<dyn Fn((&Api, &Count)) -> TGResult<()> + Send + Sync + 'static>>,
  custom_request_result: Option<Arc<dyn Fn((&Api, &CustomRequestResult)) -> TGResult<()> + Send + Sync + 'static>>,
  deep_link_info: Option<Arc<dyn Fn((&Api, &DeepLinkInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  email_address_authentication_code_info: Option<Arc<dyn Fn((&Api, &EmailAddressAuthenticationCodeInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  error: Option<Arc<dyn Fn((&Api, &Error)) -> TGResult<()> + Send + Sync + 'static>>,
  file: Option<Arc<dyn Fn((&Api, &File)) -> TGResult<()> + Send + Sync + 'static>>,
  formatted_text: Option<Arc<dyn Fn((&Api, &FormattedText)) -> TGResult<()> + Send + Sync + 'static>>,
  found_messages: Option<Arc<dyn Fn((&Api, &FoundMessages)) -> TGResult<()> + Send + Sync + 'static>>,
  game_high_scores: Option<Arc<dyn Fn((&Api, &GameHighScores)) -> TGResult<()> + Send + Sync + 'static>>,
  hashtags: Option<Arc<dyn Fn((&Api, &Hashtags)) -> TGResult<()> + Send + Sync + 'static>>,
  imported_contacts: Option<Arc<dyn Fn((&Api, &ImportedContacts)) -> TGResult<()> + Send + Sync + 'static>>,
  inline_query_results: Option<Arc<dyn Fn((&Api, &InlineQueryResults)) -> TGResult<()> + Send + Sync + 'static>>,
  language_pack_strings: Option<Arc<dyn Fn((&Api, &LanguagePackStrings)) -> TGResult<()> + Send + Sync + 'static>>,
  localization_target_info: Option<Arc<dyn Fn((&Api, &LocalizationTargetInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  message: Option<Arc<dyn Fn((&Api, &Message)) -> TGResult<()> + Send + Sync + 'static>>,
  messages: Option<Arc<dyn Fn((&Api, &Messages)) -> TGResult<()> + Send + Sync + 'static>>,
  network_statistics: Option<Arc<dyn Fn((&Api, &NetworkStatistics)) -> TGResult<()> + Send + Sync + 'static>>,
  ok: Option<Arc<dyn Fn((&Api, &Ok)) -> TGResult<()> + Send + Sync + 'static>>,
  order_info: Option<Arc<dyn Fn((&Api, &OrderInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  passport_authorization_form: Option<Arc<dyn Fn((&Api, &PassportAuthorizationForm)) -> TGResult<()> + Send + Sync + 'static>>,
  passport_elements: Option<Arc<dyn Fn((&Api, &PassportElements)) -> TGResult<()> + Send + Sync + 'static>>,
  password_state: Option<Arc<dyn Fn((&Api, &PasswordState)) -> TGResult<()> + Send + Sync + 'static>>,
  payment_form: Option<Arc<dyn Fn((&Api, &PaymentForm)) -> TGResult<()> + Send + Sync + 'static>>,
  payment_receipt: Option<Arc<dyn Fn((&Api, &PaymentReceipt)) -> TGResult<()> + Send + Sync + 'static>>,
  payment_result: Option<Arc<dyn Fn((&Api, &PaymentResult)) -> TGResult<()> + Send + Sync + 'static>>,
  proxies: Option<Arc<dyn Fn((&Api, &Proxies)) -> TGResult<()> + Send + Sync + 'static>>,
  proxy: Option<Arc<dyn Fn((&Api, &Proxy)) -> TGResult<()> + Send + Sync + 'static>>,
  public_message_link: Option<Arc<dyn Fn((&Api, &PublicMessageLink)) -> TGResult<()> + Send + Sync + 'static>>,
  recovery_email_address: Option<Arc<dyn Fn((&Api, &RecoveryEmailAddress)) -> TGResult<()> + Send + Sync + 'static>>,
  scope_notification_settings: Option<Arc<dyn Fn((&Api, &ScopeNotificationSettings)) -> TGResult<()> + Send + Sync + 'static>>,
  seconds: Option<Arc<dyn Fn((&Api, &Seconds)) -> TGResult<()> + Send + Sync + 'static>>,
  secret_chat: Option<Arc<dyn Fn((&Api, &SecretChat)) -> TGResult<()> + Send + Sync + 'static>>,
  sessions: Option<Arc<dyn Fn((&Api, &Sessions)) -> TGResult<()> + Send + Sync + 'static>>,
  sticker_emojis: Option<Arc<dyn Fn((&Api, &StickerEmojis)) -> TGResult<()> + Send + Sync + 'static>>,
  sticker_set: Option<Arc<dyn Fn((&Api, &StickerSet)) -> TGResult<()> + Send + Sync + 'static>>,
  sticker_sets: Option<Arc<dyn Fn((&Api, &StickerSets)) -> TGResult<()> + Send + Sync + 'static>>,
  stickers: Option<Arc<dyn Fn((&Api, &Stickers)) -> TGResult<()> + Send + Sync + 'static>>,
  storage_statistics: Option<Arc<dyn Fn((&Api, &StorageStatistics)) -> TGResult<()> + Send + Sync + 'static>>,
  storage_statistics_fast: Option<Arc<dyn Fn((&Api, &StorageStatisticsFast)) -> TGResult<()> + Send + Sync + 'static>>,
  supergroup: Option<Arc<dyn Fn((&Api, &Supergroup)) -> TGResult<()> + Send + Sync + 'static>>,
  supergroup_full_info: Option<Arc<dyn Fn((&Api, &SupergroupFullInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  t_me_urls: Option<Arc<dyn Fn((&Api, &TMeUrls)) -> TGResult<()> + Send + Sync + 'static>>,
  temporary_password_state: Option<Arc<dyn Fn((&Api, &TemporaryPasswordState)) -> TGResult<()> + Send + Sync + 'static>>,
  test_bytes: Option<Arc<dyn Fn((&Api, &TestBytes)) -> TGResult<()> + Send + Sync + 'static>>,
  test_int: Option<Arc<dyn Fn((&Api, &TestInt)) -> TGResult<()> + Send + Sync + 'static>>,
  test_string: Option<Arc<dyn Fn((&Api, &TestString)) -> TGResult<()> + Send + Sync + 'static>>,
  test_vector_int: Option<Arc<dyn Fn((&Api, &TestVectorInt)) -> TGResult<()> + Send + Sync + 'static>>,
  test_vector_int_object: Option<Arc<dyn Fn((&Api, &TestVectorIntObject)) -> TGResult<()> + Send + Sync + 'static>>,
  test_vector_string: Option<Arc<dyn Fn((&Api, &TestVectorString)) -> TGResult<()> + Send + Sync + 'static>>,
  test_vector_string_object: Option<Arc<dyn Fn((&Api, &TestVectorStringObject)) -> TGResult<()> + Send + Sync + 'static>>,
  text: Option<Arc<dyn Fn((&Api, &Text)) -> TGResult<()> + Send + Sync + 'static>>,
  text_entities: Option<Arc<dyn Fn((&Api, &TextEntities)) -> TGResult<()> + Send + Sync + 'static>>,
  user: Option<Arc<dyn Fn((&Api, &User)) -> TGResult<()> + Send + Sync + 'static>>,
  user_full_info: Option<Arc<dyn Fn((&Api, &UserFullInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  user_privacy_setting_rules: Option<Arc<dyn Fn((&Api, &UserPrivacySettingRules)) -> TGResult<()> + Send + Sync + 'static>>,
  user_profile_photos: Option<Arc<dyn Fn((&Api, &UserProfilePhotos)) -> TGResult<()> + Send + Sync + 'static>>,
  users: Option<Arc<dyn Fn((&Api, &Users)) -> TGResult<()> + Send + Sync + 'static>>,
  validated_order_info: Option<Arc<dyn Fn((&Api, &ValidatedOrderInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  wallpapers: Option<Arc<dyn Fn((&Api, &Wallpapers)) -> TGResult<()> + Send + Sync + 'static>>,
  web_page: Option<Arc<dyn Fn((&Api, &WebPage)) -> TGResult<()> + Send + Sync + 'static>>,
  web_page_instant_view: Option<Arc<dyn Fn((&Api, &WebPageInstantView)) -> TGResult<()> + Send + Sync + 'static>>,

}


impl Listener {
  pub fn new() -> Self { Listener::default() }

  pub(crate) fn has_receive_listen(&self) -> bool { self.receive.is_some() }

  pub(crate) fn lout(&self) -> Lout { Lout::new(self.clone()) }


  /// when receive data from tdlib
  pub fn on_receive<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&Api, &String)) -> TGResult<()> + Send + Sync + 'static {
    self.receive = Some(Arc::new(fnc));
    self
  }

  /// when telegram client throw exception
  pub fn on_exception<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&Api, &TGError)) + Send + Sync + 'static {
    self.exception = Some(Arc::new(fnc));
    self
  }






  /// Does nothing and ensures that the Update object is used; for testing only
  pub fn on_test_use_update<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &TestUseUpdate)) -> TGResult<()> + Send + Sync + 'static {
    self.test_use_update = Some(Arc::new(fnc));
    self
  }

  /// The user authorization state has changed
  pub fn on_update_authorization_state<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateAuthorizationState)) -> TGResult<()> + Send + Sync + 'static {
    self.update_authorization_state = Some(Arc::new(fnc));
    self
  }

  /// Some data of a basic group has changed. This update is guaranteed to come before the basic group identifier is returned to the client
  pub fn on_update_basic_group<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateBasicGroup)) -> TGResult<()> + Send + Sync + 'static {
    self.update_basic_group = Some(Arc::new(fnc));
    self
  }

  /// Some data from basicGroupFullInfo has been changed
  pub fn on_update_basic_group_full_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateBasicGroupFullInfo)) -> TGResult<()> + Send + Sync + 'static {
    self.update_basic_group_full_info = Some(Arc::new(fnc));
    self
  }

  /// New call was created or information about a call was updated
  pub fn on_update_call<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateCall)) -> TGResult<()> + Send + Sync + 'static {
    self.update_call = Some(Arc::new(fnc));
    self
  }

  /// The value of the default disable_notification parameter, used when a message is sent to the chat, was changed
  pub fn on_update_chat_default_disable_notification<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatDefaultDisableNotification)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_default_disable_notification = Some(Arc::new(fnc));
    self
  }

  /// A chat draft has changed. Be aware that the update may come in the currently opened chat but with old content of the draft. If the user has changed the content of the draft, this update shouldn't be applied
  pub fn on_update_chat_draft_message<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatDraftMessage)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_draft_message = Some(Arc::new(fnc));
    self
  }

  /// A chat was marked as unread or was read
  pub fn on_update_chat_is_marked_as_unread<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatIsMarkedAsUnread)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_is_marked_as_unread = Some(Arc::new(fnc));
    self
  }

  /// A chat was pinned or unpinned
  pub fn on_update_chat_is_pinned<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatIsPinned)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_is_pinned = Some(Arc::new(fnc));
    self
  }

  /// A chat's is_sponsored field has changed
  pub fn on_update_chat_is_sponsored<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatIsSponsored)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_is_sponsored = Some(Arc::new(fnc));
    self
  }

  /// The last message of a chat was changed. If last_message is null then the last message in the chat became unknown. Some new unknown messages might be added to the chat in this case
  pub fn on_update_chat_last_message<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatLastMessage)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_last_message = Some(Arc::new(fnc));
    self
  }

  /// Notification settings for a chat were changed
  pub fn on_update_chat_notification_settings<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatNotificationSettings)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_notification_settings = Some(Arc::new(fnc));
    self
  }

  /// The order of the chat in the chats list has changed. Instead of this update updateChatLastMessage, updateChatIsPinned or updateChatDraftMessage might be sent
  pub fn on_update_chat_order<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatOrder)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_order = Some(Arc::new(fnc));
    self
  }

  /// A chat photo was changed
  pub fn on_update_chat_photo<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatPhoto)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_photo = Some(Arc::new(fnc));
    self
  }

  /// Incoming messages were read or number of unread messages has been changed
  pub fn on_update_chat_read_inbox<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatReadInbox)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_read_inbox = Some(Arc::new(fnc));
    self
  }

  /// Outgoing messages were read
  pub fn on_update_chat_read_outbox<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatReadOutbox)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_read_outbox = Some(Arc::new(fnc));
    self
  }

  /// The default chat reply markup was changed. Can occur because new messages with reply markup were received or because an old reply markup was hidden by the user
  pub fn on_update_chat_reply_markup<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatReplyMarkup)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_reply_markup = Some(Arc::new(fnc));
    self
  }

  /// The title of a chat was changed
  pub fn on_update_chat_title<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatTitle)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_title = Some(Arc::new(fnc));
    self
  }

  /// The chat unread_mention_count has changed
  pub fn on_update_chat_unread_mention_count<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatUnreadMentionCount)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_unread_mention_count = Some(Arc::new(fnc));
    self
  }

  /// The connection state has changed
  pub fn on_update_connection_state<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateConnectionState)) -> TGResult<()> + Send + Sync + 'static {
    self.update_connection_state = Some(Arc::new(fnc));
    self
  }

  /// Some messages were deleted
  pub fn on_update_delete_messages<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateDeleteMessages)) -> TGResult<()> + Send + Sync + 'static {
    self.update_delete_messages = Some(Arc::new(fnc));
    self
  }

  /// The list of favorite stickers was updated
  pub fn on_update_favorite_stickers<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateFavoriteStickers)) -> TGResult<()> + Send + Sync + 'static {
    self.update_favorite_stickers = Some(Arc::new(fnc));
    self
  }

  /// Information about a file was updated
  pub fn on_update_file<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateFile)) -> TGResult<()> + Send + Sync + 'static {
    self.update_file = Some(Arc::new(fnc));
    self
  }

  /// The file generation process needs to be started by the client
  pub fn on_update_file_generation_start<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateFileGenerationStart)) -> TGResult<()> + Send + Sync + 'static {
    self.update_file_generation_start = Some(Arc::new(fnc));
    self
  }

  /// File generation is no longer needed
  pub fn on_update_file_generation_stop<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateFileGenerationStop)) -> TGResult<()> + Send + Sync + 'static {
    self.update_file_generation_stop = Some(Arc::new(fnc));
    self
  }

  /// The list of installed sticker sets was updated
  pub fn on_update_installed_sticker_sets<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateInstalledStickerSets)) -> TGResult<()> + Send + Sync + 'static {
    self.update_installed_sticker_sets = Some(Arc::new(fnc));
    self
  }

  /// Some language pack strings have been updated
  pub fn on_update_language_pack_strings<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateLanguagePackStrings)) -> TGResult<()> + Send + Sync + 'static {
    self.update_language_pack_strings = Some(Arc::new(fnc));
    self
  }

  /// The message content has changed
  pub fn on_update_message_content<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateMessageContent)) -> TGResult<()> + Send + Sync + 'static {
    self.update_message_content = Some(Arc::new(fnc));
    self
  }

  /// The message content was opened. Updates voice note messages to "listened", video note messages to "viewed" and starts the TTL timer for self-destructing messages
  pub fn on_update_message_content_opened<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateMessageContentOpened)) -> TGResult<()> + Send + Sync + 'static {
    self.update_message_content_opened = Some(Arc::new(fnc));
    self
  }

  /// A message was edited. Changes in the message content will come in a separate updateMessageContent
  pub fn on_update_message_edited<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateMessageEdited)) -> TGResult<()> + Send + Sync + 'static {
    self.update_message_edited = Some(Arc::new(fnc));
    self
  }

  /// A message with an unread mention was read
  pub fn on_update_message_mention_read<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateMessageMentionRead)) -> TGResult<()> + Send + Sync + 'static {
    self.update_message_mention_read = Some(Arc::new(fnc));
    self
  }

  /// A request to send a message has reached the Telegram server. This doesn't mean that the message will be sent successfully or even that the send message request will be processed. This update will be sent only if the option "use_quick_ack" is set to true. This update may be sent multiple times for the same message
  pub fn on_update_message_send_acknowledged<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateMessageSendAcknowledged)) -> TGResult<()> + Send + Sync + 'static {
    self.update_message_send_acknowledged = Some(Arc::new(fnc));
    self
  }

  /// A message failed to send. Be aware that some messages being sent can be irrecoverably deleted, in which case updateDeleteMessages will be received instead of this update
  pub fn on_update_message_send_failed<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateMessageSendFailed)) -> TGResult<()> + Send + Sync + 'static {
    self.update_message_send_failed = Some(Arc::new(fnc));
    self
  }

  /// A message has been successfully sent
  pub fn on_update_message_send_succeeded<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateMessageSendSucceeded)) -> TGResult<()> + Send + Sync + 'static {
    self.update_message_send_succeeded = Some(Arc::new(fnc));
    self
  }

  /// The view count of the message has changed
  pub fn on_update_message_views<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateMessageViews)) -> TGResult<()> + Send + Sync + 'static {
    self.update_message_views = Some(Arc::new(fnc));
    self
  }

  /// A new incoming callback query; for bots only
  pub fn on_update_new_callback_query<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateNewCallbackQuery)) -> TGResult<()> + Send + Sync + 'static {
    self.update_new_callback_query = Some(Arc::new(fnc));
    self
  }

  /// A new chat has been loaded/created. This update is guaranteed to come before the chat identifier is returned to the client. The chat field changes will be reported through separate updates
  pub fn on_update_new_chat<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateNewChat)) -> TGResult<()> + Send + Sync + 'static {
    self.update_new_chat = Some(Arc::new(fnc));
    self
  }

  /// The user has chosen a result of an inline query; for bots only
  pub fn on_update_new_chosen_inline_result<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateNewChosenInlineResult)) -> TGResult<()> + Send + Sync + 'static {
    self.update_new_chosen_inline_result = Some(Arc::new(fnc));
    self
  }

  /// A new incoming event; for bots only
  pub fn on_update_new_custom_event<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateNewCustomEvent)) -> TGResult<()> + Send + Sync + 'static {
    self.update_new_custom_event = Some(Arc::new(fnc));
    self
  }

  /// A new incoming query; for bots only
  pub fn on_update_new_custom_query<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateNewCustomQuery)) -> TGResult<()> + Send + Sync + 'static {
    self.update_new_custom_query = Some(Arc::new(fnc));
    self
  }

  /// A new incoming callback query from a message sent via a bot; for bots only
  pub fn on_update_new_inline_callback_query<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateNewInlineCallbackQuery)) -> TGResult<()> + Send + Sync + 'static {
    self.update_new_inline_callback_query = Some(Arc::new(fnc));
    self
  }

  /// A new incoming inline query; for bots only
  pub fn on_update_new_inline_query<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateNewInlineQuery)) -> TGResult<()> + Send + Sync + 'static {
    self.update_new_inline_query = Some(Arc::new(fnc));
    self
  }

  /// A new message was received; can also be an outgoing message
  pub fn on_update_new_message<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateNewMessage)) -> TGResult<()> + Send + Sync + 'static {
    self.update_new_message = Some(Arc::new(fnc));
    self
  }

  /// A new incoming pre-checkout query; for bots only. Contains full information about a checkout
  pub fn on_update_new_pre_checkout_query<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateNewPreCheckoutQuery)) -> TGResult<()> + Send + Sync + 'static {
    self.update_new_pre_checkout_query = Some(Arc::new(fnc));
    self
  }

  /// A new incoming shipping query; for bots only. Only for invoices with flexible price
  pub fn on_update_new_shipping_query<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateNewShippingQuery)) -> TGResult<()> + Send + Sync + 'static {
    self.update_new_shipping_query = Some(Arc::new(fnc));
    self
  }

  /// An option changed its value
  pub fn on_update_option<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateOption)) -> TGResult<()> + Send + Sync + 'static {
    self.update_option = Some(Arc::new(fnc));
    self
  }

  /// The list of recently used stickers was updated
  pub fn on_update_recent_stickers<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateRecentStickers)) -> TGResult<()> + Send + Sync + 'static {
    self.update_recent_stickers = Some(Arc::new(fnc));
    self
  }

  /// The list of saved animations was updated
  pub fn on_update_saved_animations<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateSavedAnimations)) -> TGResult<()> + Send + Sync + 'static {
    self.update_saved_animations = Some(Arc::new(fnc));
    self
  }

  /// Notification settings for some type of chats were updated
  pub fn on_update_scope_notification_settings<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateScopeNotificationSettings)) -> TGResult<()> + Send + Sync + 'static {
    self.update_scope_notification_settings = Some(Arc::new(fnc));
    self
  }

  /// Some data of a secret chat has changed. This update is guaranteed to come before the secret chat identifier is returned to the client
  pub fn on_update_secret_chat<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateSecretChat)) -> TGResult<()> + Send + Sync + 'static {
    self.update_secret_chat = Some(Arc::new(fnc));
    self
  }

  /// Service notification from the server. Upon receiving this the client must show a popup with the content of the notification
  pub fn on_update_service_notification<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateServiceNotification)) -> TGResult<()> + Send + Sync + 'static {
    self.update_service_notification = Some(Arc::new(fnc));
    self
  }

  /// Some data of a supergroup or a channel has changed. This update is guaranteed to come before the supergroup identifier is returned to the client
  pub fn on_update_supergroup<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateSupergroup)) -> TGResult<()> + Send + Sync + 'static {
    self.update_supergroup = Some(Arc::new(fnc));
    self
  }

  /// Some data from supergroupFullInfo has been changed
  pub fn on_update_supergroup_full_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateSupergroupFullInfo)) -> TGResult<()> + Send + Sync + 'static {
    self.update_supergroup_full_info = Some(Arc::new(fnc));
    self
  }

  /// New terms of service must be accepted by the user. If the terms of service are declined, then the deleteAccount method should be called with the reason "Decline ToS update"
  pub fn on_update_terms_of_service<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateTermsOfService)) -> TGResult<()> + Send + Sync + 'static {
    self.update_terms_of_service = Some(Arc::new(fnc));
    self
  }

  /// The list of trending sticker sets was updated or some of them were viewed
  pub fn on_update_trending_sticker_sets<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateTrendingStickerSets)) -> TGResult<()> + Send + Sync + 'static {
    self.update_trending_sticker_sets = Some(Arc::new(fnc));
    self
  }

  /// Number of unread chats, i.e. with unread messages or marked as unread, has changed. This update is sent only if a message database is used
  pub fn on_update_unread_chat_count<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateUnreadChatCount)) -> TGResult<()> + Send + Sync + 'static {
    self.update_unread_chat_count = Some(Arc::new(fnc));
    self
  }

  /// Number of unread messages has changed. This update is sent only if a message database is used
  pub fn on_update_unread_message_count<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateUnreadMessageCount)) -> TGResult<()> + Send + Sync + 'static {
    self.update_unread_message_count = Some(Arc::new(fnc));
    self
  }

  /// Some data of a user has changed. This update is guaranteed to come before the user identifier is returned to the client
  pub fn on_update_user<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateUser)) -> TGResult<()> + Send + Sync + 'static {
    self.update_user = Some(Arc::new(fnc));
    self
  }

  /// User activity in the chat has changed
  pub fn on_update_user_chat_action<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateUserChatAction)) -> TGResult<()> + Send + Sync + 'static {
    self.update_user_chat_action = Some(Arc::new(fnc));
    self
  }

  /// Some data from userFullInfo has been changed
  pub fn on_update_user_full_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateUserFullInfo)) -> TGResult<()> + Send + Sync + 'static {
    self.update_user_full_info = Some(Arc::new(fnc));
    self
  }

  /// Some privacy setting rules have been changed
  pub fn on_update_user_privacy_setting_rules<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateUserPrivacySettingRules)) -> TGResult<()> + Send + Sync + 'static {
    self.update_user_privacy_setting_rules = Some(Arc::new(fnc));
    self
  }

  /// The user went online or offline
  pub fn on_update_user_status<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateUserStatus)) -> TGResult<()> + Send + Sync + 'static {
    self.update_user_status = Some(Arc::new(fnc));
    self
  }



  /// Represents the current authorization state of the client
  pub fn on_authorization_state<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &AuthorizationState)) -> TGResult<()> + Send + Sync + 'static {
    self.authorization_state = Some(Arc::new(fnc));
    self
  }

  /// Represents result of checking whether a username can be set for a chat
  pub fn on_check_chat_username_result<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &CheckChatUsernameResult)) -> TGResult<()> + Send + Sync + 'static {
    self.check_chat_username_result = Some(Arc::new(fnc));
    self
  }

  /// Represents the value of a string in a language pack
  pub fn on_language_pack_string_value<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &LanguagePackStringValue)) -> TGResult<()> + Send + Sync + 'static {
    self.language_pack_string_value = Some(Arc::new(fnc));
    self
  }

  /// Represents the value of an option
  pub fn on_option_value<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &OptionValue)) -> TGResult<()> + Send + Sync + 'static {
    self.option_value = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a Telegram Passport element
  pub fn on_passport_element<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &PassportElement)) -> TGResult<()> + Send + Sync + 'static {
    self.passport_element = Some(Arc::new(fnc));
    self
  }

  /// Contains notifications about data changes
  pub fn on_update<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Update)) -> TGResult<()> + Send + Sync + 'static {
    self.update = Some(Arc::new(fnc));
    self
  }

  /// Contains information about the period of inactivity after which the current user's account will automatically be deleted
  pub fn on_account_ttl<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &AccountTtl)) -> TGResult<()> + Send + Sync + 'static {
    self.account_ttl = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of animations
  pub fn on_animations<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Animations)) -> TGResult<()> + Send + Sync + 'static {
    self.animations = Some(Arc::new(fnc));
    self
  }

  /// Information about the authentication code that was sent
  pub fn on_authentication_code_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &AuthenticationCodeInfo)) -> TGResult<()> + Send + Sync + 'static {
    self.authentication_code_info = Some(Arc::new(fnc));
    self
  }

  /// Represents a basic group of 0-200 users (must be upgraded to a supergroup to accommodate more than 200 users)
  pub fn on_basic_group<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &BasicGroup)) -> TGResult<()> + Send + Sync + 'static {
    self.basic_group = Some(Arc::new(fnc));
    self
  }

  /// Contains full information about a basic group
  pub fn on_basic_group_full_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &BasicGroupFullInfo)) -> TGResult<()> + Send + Sync + 'static {
    self.basic_group_full_info = Some(Arc::new(fnc));
    self
  }

  /// Contains the call identifier
  pub fn on_call_id<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &CallId)) -> TGResult<()> + Send + Sync + 'static {
    self.call_id = Some(Arc::new(fnc));
    self
  }

  /// Contains a bot's answer to a callback query
  pub fn on_callback_query_answer<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &CallbackQueryAnswer)) -> TGResult<()> + Send + Sync + 'static {
    self.callback_query_answer = Some(Arc::new(fnc));
    self
  }

  /// A chat. (Can be a private chat, basic group, supergroup, or secret chat)
  pub fn on_chat<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Chat)) -> TGResult<()> + Send + Sync + 'static {
    self.chat = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of chat events
  pub fn on_chat_events<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &ChatEvents)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_events = Some(Arc::new(fnc));
    self
  }

  /// Contains a chat invite link
  pub fn on_chat_invite_link<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &ChatInviteLink)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_invite_link = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a chat invite link
  pub fn on_chat_invite_link_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &ChatInviteLinkInfo)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_invite_link_info = Some(Arc::new(fnc));
    self
  }

  /// A user with information about joining/leaving a chat
  pub fn on_chat_member<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &ChatMember)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_member = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of chat members
  pub fn on_chat_members<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &ChatMembers)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_members = Some(Arc::new(fnc));
    self
  }

  /// Contains information about the availability of the "Report spam" action for a chat
  pub fn on_chat_report_spam_state<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &ChatReportSpamState)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_report_spam_state = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of chats
  pub fn on_chats<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Chats)) -> TGResult<()> + Send + Sync + 'static {
    self.chats = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of websites the current user is logged in with Telegram
  pub fn on_connected_websites<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &ConnectedWebsites)) -> TGResult<()> + Send + Sync + 'static {
    self.connected_websites = Some(Arc::new(fnc));
    self
  }

  /// Contains a counter
  pub fn on_count<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Count)) -> TGResult<()> + Send + Sync + 'static {
    self.count = Some(Arc::new(fnc));
    self
  }

  /// Contains the result of a custom request
  pub fn on_custom_request_result<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &CustomRequestResult)) -> TGResult<()> + Send + Sync + 'static {
    self.custom_request_result = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a tg:// deep link
  pub fn on_deep_link_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &DeepLinkInfo)) -> TGResult<()> + Send + Sync + 'static {
    self.deep_link_info = Some(Arc::new(fnc));
    self
  }

  /// Information about the email address authentication code that was sent
  pub fn on_email_address_authentication_code_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &EmailAddressAuthenticationCodeInfo)) -> TGResult<()> + Send + Sync + 'static {
    self.email_address_authentication_code_info = Some(Arc::new(fnc));
    self
  }

  /// An object of this type can be returned on every function call, in case of an error
  pub fn on_error<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Error)) -> TGResult<()> + Send + Sync + 'static {
    self.error = Some(Arc::new(fnc));
    self
  }

  /// Represents a file
  pub fn on_file<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &File)) -> TGResult<()> + Send + Sync + 'static {
    self.file = Some(Arc::new(fnc));
    self
  }

  /// A text with some entities
  pub fn on_formatted_text<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &FormattedText)) -> TGResult<()> + Send + Sync + 'static {
    self.formatted_text = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of messages found by a search
  pub fn on_found_messages<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &FoundMessages)) -> TGResult<()> + Send + Sync + 'static {
    self.found_messages = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of game high scores
  pub fn on_game_high_scores<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &GameHighScores)) -> TGResult<()> + Send + Sync + 'static {
    self.game_high_scores = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of hashtags
  pub fn on_hashtags<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Hashtags)) -> TGResult<()> + Send + Sync + 'static {
    self.hashtags = Some(Arc::new(fnc));
    self
  }

  /// Represents the result of an ImportContacts request
  pub fn on_imported_contacts<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &ImportedContacts)) -> TGResult<()> + Send + Sync + 'static {
    self.imported_contacts = Some(Arc::new(fnc));
    self
  }

  /// Represents the results of the inline query. Use sendInlineQueryResultMessage to send the result of the query
  pub fn on_inline_query_results<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &InlineQueryResults)) -> TGResult<()> + Send + Sync + 'static {
    self.inline_query_results = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of language pack strings
  pub fn on_language_pack_strings<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &LanguagePackStrings)) -> TGResult<()> + Send + Sync + 'static {
    self.language_pack_strings = Some(Arc::new(fnc));
    self
  }

  /// Contains information about the current localization target
  pub fn on_localization_target_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &LocalizationTargetInfo)) -> TGResult<()> + Send + Sync + 'static {
    self.localization_target_info = Some(Arc::new(fnc));
    self
  }

  /// Describes a message
  pub fn on_message<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Message)) -> TGResult<()> + Send + Sync + 'static {
    self.message = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of messages
  pub fn on_messages<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Messages)) -> TGResult<()> + Send + Sync + 'static {
    self.messages = Some(Arc::new(fnc));
    self
  }

  /// A full list of available network statistic entries
  pub fn on_network_statistics<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &NetworkStatistics)) -> TGResult<()> + Send + Sync + 'static {
    self.network_statistics = Some(Arc::new(fnc));
    self
  }

  /// An object of this type is returned on a successful function call for certain functions
  pub fn on_ok<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Ok)) -> TGResult<()> + Send + Sync + 'static {
    self.ok = Some(Arc::new(fnc));
    self
  }

  /// Order information
  pub fn on_order_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &OrderInfo)) -> TGResult<()> + Send + Sync + 'static {
    self.order_info = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a Telegram Passport authorization form that was requested
  pub fn on_passport_authorization_form<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &PassportAuthorizationForm)) -> TGResult<()> + Send + Sync + 'static {
    self.passport_authorization_form = Some(Arc::new(fnc));
    self
  }

  /// Contains information about saved Telegram Passport elements
  pub fn on_passport_elements<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &PassportElements)) -> TGResult<()> + Send + Sync + 'static {
    self.passport_elements = Some(Arc::new(fnc));
    self
  }

  /// Represents the current state of 2-step verification
  pub fn on_password_state<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &PasswordState)) -> TGResult<()> + Send + Sync + 'static {
    self.password_state = Some(Arc::new(fnc));
    self
  }

  /// Contains information about an invoice payment form
  pub fn on_payment_form<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &PaymentForm)) -> TGResult<()> + Send + Sync + 'static {
    self.payment_form = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a successful payment
  pub fn on_payment_receipt<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &PaymentReceipt)) -> TGResult<()> + Send + Sync + 'static {
    self.payment_receipt = Some(Arc::new(fnc));
    self
  }

  /// Contains the result of a payment request
  pub fn on_payment_result<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &PaymentResult)) -> TGResult<()> + Send + Sync + 'static {
    self.payment_result = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of proxy servers
  pub fn on_proxies<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Proxies)) -> TGResult<()> + Send + Sync + 'static {
    self.proxies = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a proxy server
  pub fn on_proxy<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Proxy)) -> TGResult<()> + Send + Sync + 'static {
    self.proxy = Some(Arc::new(fnc));
    self
  }

  /// Contains a public HTTPS link to a message in a public supergroup or channel
  pub fn on_public_message_link<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &PublicMessageLink)) -> TGResult<()> + Send + Sync + 'static {
    self.public_message_link = Some(Arc::new(fnc));
    self
  }

  /// Contains information about the current recovery email address
  pub fn on_recovery_email_address<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &RecoveryEmailAddress)) -> TGResult<()> + Send + Sync + 'static {
    self.recovery_email_address = Some(Arc::new(fnc));
    self
  }

  /// Contains information about notification settings for several chats
  pub fn on_scope_notification_settings<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &ScopeNotificationSettings)) -> TGResult<()> + Send + Sync + 'static {
    self.scope_notification_settings = Some(Arc::new(fnc));
    self
  }

  /// Contains a value representing a number of seconds
  pub fn on_seconds<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Seconds)) -> TGResult<()> + Send + Sync + 'static {
    self.seconds = Some(Arc::new(fnc));
    self
  }

  /// Represents a secret chat
  pub fn on_secret_chat<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &SecretChat)) -> TGResult<()> + Send + Sync + 'static {
    self.secret_chat = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of sessions
  pub fn on_sessions<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Sessions)) -> TGResult<()> + Send + Sync + 'static {
    self.sessions = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of all emoji corresponding to a sticker in a sticker set. The list is only for informational purposes, because a sticker is always sent with a fixed emoji from the corresponding Sticker object
  pub fn on_sticker_emojis<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &StickerEmojis)) -> TGResult<()> + Send + Sync + 'static {
    self.sticker_emojis = Some(Arc::new(fnc));
    self
  }

  /// Represents a sticker set
  pub fn on_sticker_set<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &StickerSet)) -> TGResult<()> + Send + Sync + 'static {
    self.sticker_set = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of sticker sets
  pub fn on_sticker_sets<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &StickerSets)) -> TGResult<()> + Send + Sync + 'static {
    self.sticker_sets = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of stickers
  pub fn on_stickers<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Stickers)) -> TGResult<()> + Send + Sync + 'static {
    self.stickers = Some(Arc::new(fnc));
    self
  }

  /// Contains the exact storage usage statistics split by chats and file type
  pub fn on_storage_statistics<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &StorageStatistics)) -> TGResult<()> + Send + Sync + 'static {
    self.storage_statistics = Some(Arc::new(fnc));
    self
  }

  /// Contains approximate storage usage statistics, excluding files of unknown file type
  pub fn on_storage_statistics_fast<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &StorageStatisticsFast)) -> TGResult<()> + Send + Sync + 'static {
    self.storage_statistics_fast = Some(Arc::new(fnc));
    self
  }

  /// Represents a supergroup or channel with zero or more members (subscribers in the case of channels). From the point of view of the system, a channel is a special kind of a supergroup: only administrators can post and see the list of members, and posts from all administrators use the name and photo of the channel instead of individual names and profile photos. Unlike supergroups, channels can have an unlimited number of subscribers
  pub fn on_supergroup<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Supergroup)) -> TGResult<()> + Send + Sync + 'static {
    self.supergroup = Some(Arc::new(fnc));
    self
  }

  /// Contains full information about a supergroup or channel
  pub fn on_supergroup_full_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &SupergroupFullInfo)) -> TGResult<()> + Send + Sync + 'static {
    self.supergroup_full_info = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of t.me URLs
  pub fn on_t_me_urls<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &TMeUrls)) -> TGResult<()> + Send + Sync + 'static {
    self.t_me_urls = Some(Arc::new(fnc));
    self
  }

  /// Returns information about the availability of a temporary password, which can be used for payments
  pub fn on_temporary_password_state<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &TemporaryPasswordState)) -> TGResult<()> + Send + Sync + 'static {
    self.temporary_password_state = Some(Arc::new(fnc));
    self
  }

  /// A simple object containing a sequence of bytes; for testing only
  pub fn on_test_bytes<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &TestBytes)) -> TGResult<()> + Send + Sync + 'static {
    self.test_bytes = Some(Arc::new(fnc));
    self
  }

  /// A simple object containing a number; for testing only
  pub fn on_test_int<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &TestInt)) -> TGResult<()> + Send + Sync + 'static {
    self.test_int = Some(Arc::new(fnc));
    self
  }

  /// A simple object containing a string; for testing only
  pub fn on_test_string<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &TestString)) -> TGResult<()> + Send + Sync + 'static {
    self.test_string = Some(Arc::new(fnc));
    self
  }

  /// A simple object containing a vector of numbers; for testing only
  pub fn on_test_vector_int<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &TestVectorInt)) -> TGResult<()> + Send + Sync + 'static {
    self.test_vector_int = Some(Arc::new(fnc));
    self
  }

  /// A simple object containing a vector of objects that hold a number; for testing only
  pub fn on_test_vector_int_object<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &TestVectorIntObject)) -> TGResult<()> + Send + Sync + 'static {
    self.test_vector_int_object = Some(Arc::new(fnc));
    self
  }

  /// A simple object containing a vector of strings; for testing only
  pub fn on_test_vector_string<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &TestVectorString)) -> TGResult<()> + Send + Sync + 'static {
    self.test_vector_string = Some(Arc::new(fnc));
    self
  }

  /// A simple object containing a vector of objects that hold a string; for testing only
  pub fn on_test_vector_string_object<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &TestVectorStringObject)) -> TGResult<()> + Send + Sync + 'static {
    self.test_vector_string_object = Some(Arc::new(fnc));
    self
  }

  /// Contains some text
  pub fn on_text<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Text)) -> TGResult<()> + Send + Sync + 'static {
    self.text = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of text entities
  pub fn on_text_entities<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &TextEntities)) -> TGResult<()> + Send + Sync + 'static {
    self.text_entities = Some(Arc::new(fnc));
    self
  }

  /// Represents a user
  pub fn on_user<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &User)) -> TGResult<()> + Send + Sync + 'static {
    self.user = Some(Arc::new(fnc));
    self
  }

  /// Contains full information about a user (except the full list of profile photos)
  pub fn on_user_full_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UserFullInfo)) -> TGResult<()> + Send + Sync + 'static {
    self.user_full_info = Some(Arc::new(fnc));
    self
  }

  /// A list of privacy rules. Rules are matched in the specified order. The first matched rule defines the privacy setting for a given user. If no rule matches, the action is not allowed
  pub fn on_user_privacy_setting_rules<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UserPrivacySettingRules)) -> TGResult<()> + Send + Sync + 'static {
    self.user_privacy_setting_rules = Some(Arc::new(fnc));
    self
  }

  /// Contains part of the list of user photos
  pub fn on_user_profile_photos<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UserProfilePhotos)) -> TGResult<()> + Send + Sync + 'static {
    self.user_profile_photos = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of users
  pub fn on_users<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Users)) -> TGResult<()> + Send + Sync + 'static {
    self.users = Some(Arc::new(fnc));
    self
  }

  /// Contains a temporary identifier of validated order information, which is stored for one hour. Also contains the available shipping options
  pub fn on_validated_order_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &ValidatedOrderInfo)) -> TGResult<()> + Send + Sync + 'static {
    self.validated_order_info = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of wallpapers
  pub fn on_wallpapers<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Wallpapers)) -> TGResult<()> + Send + Sync + 'static {
    self.wallpapers = Some(Arc::new(fnc));
    self
  }

  /// Describes a web page preview
  pub fn on_web_page<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &WebPage)) -> TGResult<()> + Send + Sync + 'static {
    self.web_page = Some(Arc::new(fnc));
    self
  }

  /// Describes an instant view page for a web page
  pub fn on_web_page_instant_view<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &WebPageInstantView)) -> TGResult<()> + Send + Sync + 'static {
    self.web_page_instant_view = Some(Arc::new(fnc));
    self
  }

}


/// Get listener
pub struct Lout {
  listener: Listener,
  supports: Vec<&'static str>
}

impl Lout {
  fn new(listener: Listener) -> Self {
    let supports = vec![


      "testUseUpdate",
      "updateAuthorizationState",
      "updateBasicGroup",
      "updateBasicGroupFullInfo",
      "updateCall",
      "updateChatDefaultDisableNotification",
      "updateChatDraftMessage",
      "updateChatIsMarkedAsUnread",
      "updateChatIsPinned",
      "updateChatIsSponsored",
      "updateChatLastMessage",
      "updateChatNotificationSettings",
      "updateChatOrder",
      "updateChatPhoto",
      "updateChatReadInbox",
      "updateChatReadOutbox",
      "updateChatReplyMarkup",
      "updateChatTitle",
      "updateChatUnreadMentionCount",
      "updateConnectionState",
      "updateDeleteMessages",
      "updateFavoriteStickers",
      "updateFile",
      "updateFileGenerationStart",
      "updateFileGenerationStop",
      "updateInstalledStickerSets",
      "updateLanguagePackStrings",
      "updateMessageContent",
      "updateMessageContentOpened",
      "updateMessageEdited",
      "updateMessageMentionRead",
      "updateMessageSendAcknowledged",
      "updateMessageSendFailed",
      "updateMessageSendSucceeded",
      "updateMessageViews",
      "updateNewCallbackQuery",
      "updateNewChat",
      "updateNewChosenInlineResult",
      "updateNewCustomEvent",
      "updateNewCustomQuery",
      "updateNewInlineCallbackQuery",
      "updateNewInlineQuery",
      "updateNewMessage",
      "updateNewPreCheckoutQuery",
      "updateNewShippingQuery",
      "updateOption",
      "updateRecentStickers",
      "updateSavedAnimations",
      "updateScopeNotificationSettings",
      "updateSecretChat",
      "updateServiceNotification",
      "updateSupergroup",
      "updateSupergroupFullInfo",
      "updateTermsOfService",
      "updateTrendingStickerSets",
      "updateUnreadChatCount",
      "updateUnreadMessageCount",
      "updateUser",
      "updateUserChatAction",
      "updateUserFullInfo",
      "updateUserPrivacySettingRules",
      "updateUserStatus",


      "AuthorizationState",
      "CheckChatUsernameResult",
      "LanguagePackStringValue",
      "OptionValue",
      "PassportElement",
      "Update",
      "accountTtl",
      "animations",
      "authenticationCodeInfo",
      "basicGroup",
      "basicGroupFullInfo",
      "callId",
      "callbackQueryAnswer",
      "chat",
      "chatEvents",
      "chatInviteLink",
      "chatInviteLinkInfo",
      "chatMember",
      "chatMembers",
      "chatReportSpamState",
      "chats",
      "connectedWebsites",
      "count",
      "customRequestResult",
      "deepLinkInfo",
      "emailAddressAuthenticationCodeInfo",
      "error",
      "file",
      "formattedText",
      "foundMessages",
      "gameHighScores",
      "hashtags",
      "importedContacts",
      "inlineQueryResults",
      "languagePackStrings",
      "localizationTargetInfo",
      "message",
      "messages",
      "networkStatistics",
      "ok",
      "orderInfo",
      "passportAuthorizationForm",
      "passportElements",
      "passwordState",
      "paymentForm",
      "paymentReceipt",
      "paymentResult",
      "proxies",
      "proxy",
      "publicMessageLink",
      "recoveryEmailAddress",
      "scopeNotificationSettings",
      "seconds",
      "secretChat",
      "sessions",
      "stickerEmojis",
      "stickerSet",
      "stickerSets",
      "stickers",
      "storageStatistics",
      "storageStatisticsFast",
      "supergroup",
      "supergroupFullInfo",
      "tMeUrls",
      "temporaryPasswordState",
      "testBytes",
      "testInt",
      "testString",
      "testVectorInt",
      "testVectorIntObject",
      "testVectorString",
      "testVectorStringObject",
      "text",
      "textEntities",
      "user",
      "userFullInfo",
      "userPrivacySettingRules",
      "userProfilePhotos",
      "users",
      "validatedOrderInfo",
      "wallpapers",
      "webPage",
      "webPageInstantView",


    ];
    Self { listener, supports }
  }

  pub fn is_support<S: AsRef<str>>(&self, name: S) -> bool {
    self.supports.iter()
      .find(|&&item| item == name.as_ref())
      .is_some()
  }

  /// when telegram client throw exception
  pub fn exception(&self) -> &Option<Arc<dyn Fn((&Api, &TGError)) + Send + Sync + 'static>> {
    &self.listener.exception
  }

  /// when receive data from tdlib
  pub fn receive(&self) -> &Option<Arc<dyn Fn((&Api, &String)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.receive
  }





  /// Does nothing and ensures that the Update object is used; for testing only
  pub fn test_use_update(&self) -> &Option<Arc<dyn Fn((&Api, &TestUseUpdate)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.test_use_update
  }

  /// The user authorization state has changed
  pub fn update_authorization_state(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateAuthorizationState)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_authorization_state
  }

  /// Some data of a basic group has changed. This update is guaranteed to come before the basic group identifier is returned to the client
  pub fn update_basic_group(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateBasicGroup)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_basic_group
  }

  /// Some data from basicGroupFullInfo has been changed
  pub fn update_basic_group_full_info(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateBasicGroupFullInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_basic_group_full_info
  }

  /// New call was created or information about a call was updated
  pub fn update_call(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateCall)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_call
  }

  /// The value of the default disable_notification parameter, used when a message is sent to the chat, was changed
  pub fn update_chat_default_disable_notification(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatDefaultDisableNotification)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_default_disable_notification
  }

  /// A chat draft has changed. Be aware that the update may come in the currently opened chat but with old content of the draft. If the user has changed the content of the draft, this update shouldn't be applied
  pub fn update_chat_draft_message(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatDraftMessage)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_draft_message
  }

  /// A chat was marked as unread or was read
  pub fn update_chat_is_marked_as_unread(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatIsMarkedAsUnread)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_is_marked_as_unread
  }

  /// A chat was pinned or unpinned
  pub fn update_chat_is_pinned(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatIsPinned)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_is_pinned
  }

  /// A chat's is_sponsored field has changed
  pub fn update_chat_is_sponsored(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatIsSponsored)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_is_sponsored
  }

  /// The last message of a chat was changed. If last_message is null then the last message in the chat became unknown. Some new unknown messages might be added to the chat in this case
  pub fn update_chat_last_message(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatLastMessage)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_last_message
  }

  /// Notification settings for a chat were changed
  pub fn update_chat_notification_settings(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatNotificationSettings)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_notification_settings
  }

  /// The order of the chat in the chats list has changed. Instead of this update updateChatLastMessage, updateChatIsPinned or updateChatDraftMessage might be sent
  pub fn update_chat_order(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatOrder)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_order
  }

  /// A chat photo was changed
  pub fn update_chat_photo(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatPhoto)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_photo
  }

  /// Incoming messages were read or number of unread messages has been changed
  pub fn update_chat_read_inbox(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatReadInbox)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_read_inbox
  }

  /// Outgoing messages were read
  pub fn update_chat_read_outbox(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatReadOutbox)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_read_outbox
  }

  /// The default chat reply markup was changed. Can occur because new messages with reply markup were received or because an old reply markup was hidden by the user
  pub fn update_chat_reply_markup(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatReplyMarkup)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_reply_markup
  }

  /// The title of a chat was changed
  pub fn update_chat_title(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatTitle)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_title
  }

  /// The chat unread_mention_count has changed
  pub fn update_chat_unread_mention_count(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatUnreadMentionCount)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_unread_mention_count
  }

  /// The connection state has changed
  pub fn update_connection_state(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateConnectionState)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_connection_state
  }

  /// Some messages were deleted
  pub fn update_delete_messages(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateDeleteMessages)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_delete_messages
  }

  /// The list of favorite stickers was updated
  pub fn update_favorite_stickers(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateFavoriteStickers)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_favorite_stickers
  }

  /// Information about a file was updated
  pub fn update_file(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateFile)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_file
  }

  /// The file generation process needs to be started by the client
  pub fn update_file_generation_start(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateFileGenerationStart)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_file_generation_start
  }

  /// File generation is no longer needed
  pub fn update_file_generation_stop(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateFileGenerationStop)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_file_generation_stop
  }

  /// The list of installed sticker sets was updated
  pub fn update_installed_sticker_sets(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateInstalledStickerSets)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_installed_sticker_sets
  }

  /// Some language pack strings have been updated
  pub fn update_language_pack_strings(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateLanguagePackStrings)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_language_pack_strings
  }

  /// The message content has changed
  pub fn update_message_content(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateMessageContent)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_message_content
  }

  /// The message content was opened. Updates voice note messages to "listened", video note messages to "viewed" and starts the TTL timer for self-destructing messages
  pub fn update_message_content_opened(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateMessageContentOpened)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_message_content_opened
  }

  /// A message was edited. Changes in the message content will come in a separate updateMessageContent
  pub fn update_message_edited(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateMessageEdited)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_message_edited
  }

  /// A message with an unread mention was read
  pub fn update_message_mention_read(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateMessageMentionRead)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_message_mention_read
  }

  /// A request to send a message has reached the Telegram server. This doesn't mean that the message will be sent successfully or even that the send message request will be processed. This update will be sent only if the option "use_quick_ack" is set to true. This update may be sent multiple times for the same message
  pub fn update_message_send_acknowledged(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateMessageSendAcknowledged)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_message_send_acknowledged
  }

  /// A message failed to send. Be aware that some messages being sent can be irrecoverably deleted, in which case updateDeleteMessages will be received instead of this update
  pub fn update_message_send_failed(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateMessageSendFailed)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_message_send_failed
  }

  /// A message has been successfully sent
  pub fn update_message_send_succeeded(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateMessageSendSucceeded)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_message_send_succeeded
  }

  /// The view count of the message has changed
  pub fn update_message_views(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateMessageViews)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_message_views
  }

  /// A new incoming callback query; for bots only
  pub fn update_new_callback_query(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateNewCallbackQuery)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_new_callback_query
  }

  /// A new chat has been loaded/created. This update is guaranteed to come before the chat identifier is returned to the client. The chat field changes will be reported through separate updates
  pub fn update_new_chat(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateNewChat)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_new_chat
  }

  /// The user has chosen a result of an inline query; for bots only
  pub fn update_new_chosen_inline_result(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateNewChosenInlineResult)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_new_chosen_inline_result
  }

  /// A new incoming event; for bots only
  pub fn update_new_custom_event(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateNewCustomEvent)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_new_custom_event
  }

  /// A new incoming query; for bots only
  pub fn update_new_custom_query(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateNewCustomQuery)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_new_custom_query
  }

  /// A new incoming callback query from a message sent via a bot; for bots only
  pub fn update_new_inline_callback_query(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateNewInlineCallbackQuery)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_new_inline_callback_query
  }

  /// A new incoming inline query; for bots only
  pub fn update_new_inline_query(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateNewInlineQuery)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_new_inline_query
  }

  /// A new message was received; can also be an outgoing message
  pub fn update_new_message(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateNewMessage)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_new_message
  }

  /// A new incoming pre-checkout query; for bots only. Contains full information about a checkout
  pub fn update_new_pre_checkout_query(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateNewPreCheckoutQuery)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_new_pre_checkout_query
  }

  /// A new incoming shipping query; for bots only. Only for invoices with flexible price
  pub fn update_new_shipping_query(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateNewShippingQuery)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_new_shipping_query
  }

  /// An option changed its value
  pub fn update_option(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateOption)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_option
  }

  /// The list of recently used stickers was updated
  pub fn update_recent_stickers(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateRecentStickers)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_recent_stickers
  }

  /// The list of saved animations was updated
  pub fn update_saved_animations(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateSavedAnimations)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_saved_animations
  }

  /// Notification settings for some type of chats were updated
  pub fn update_scope_notification_settings(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateScopeNotificationSettings)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_scope_notification_settings
  }

  /// Some data of a secret chat has changed. This update is guaranteed to come before the secret chat identifier is returned to the client
  pub fn update_secret_chat(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateSecretChat)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_secret_chat
  }

  /// Service notification from the server. Upon receiving this the client must show a popup with the content of the notification
  pub fn update_service_notification(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateServiceNotification)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_service_notification
  }

  /// Some data of a supergroup or a channel has changed. This update is guaranteed to come before the supergroup identifier is returned to the client
  pub fn update_supergroup(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateSupergroup)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_supergroup
  }

  /// Some data from supergroupFullInfo has been changed
  pub fn update_supergroup_full_info(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateSupergroupFullInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_supergroup_full_info
  }

  /// New terms of service must be accepted by the user. If the terms of service are declined, then the deleteAccount method should be called with the reason "Decline ToS update"
  pub fn update_terms_of_service(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateTermsOfService)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_terms_of_service
  }

  /// The list of trending sticker sets was updated or some of them were viewed
  pub fn update_trending_sticker_sets(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateTrendingStickerSets)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_trending_sticker_sets
  }

  /// Number of unread chats, i.e. with unread messages or marked as unread, has changed. This update is sent only if a message database is used
  pub fn update_unread_chat_count(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateUnreadChatCount)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_unread_chat_count
  }

  /// Number of unread messages has changed. This update is sent only if a message database is used
  pub fn update_unread_message_count(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateUnreadMessageCount)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_unread_message_count
  }

  /// Some data of a user has changed. This update is guaranteed to come before the user identifier is returned to the client
  pub fn update_user(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateUser)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_user
  }

  /// User activity in the chat has changed
  pub fn update_user_chat_action(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateUserChatAction)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_user_chat_action
  }

  /// Some data from userFullInfo has been changed
  pub fn update_user_full_info(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateUserFullInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_user_full_info
  }

  /// Some privacy setting rules have been changed
  pub fn update_user_privacy_setting_rules(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateUserPrivacySettingRules)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_user_privacy_setting_rules
  }

  /// The user went online or offline
  pub fn update_user_status(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateUserStatus)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_user_status
  }



  /// Represents the current authorization state of the client
  pub fn authorization_state(&self) -> &Option<Arc<dyn Fn((&Api, &AuthorizationState)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.authorization_state
  }

  /// Represents result of checking whether a username can be set for a chat
  pub fn check_chat_username_result(&self) -> &Option<Arc<dyn Fn((&Api, &CheckChatUsernameResult)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.check_chat_username_result
  }

  /// Represents the value of a string in a language pack
  pub fn language_pack_string_value(&self) -> &Option<Arc<dyn Fn((&Api, &LanguagePackStringValue)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.language_pack_string_value
  }

  /// Represents the value of an option
  pub fn option_value(&self) -> &Option<Arc<dyn Fn((&Api, &OptionValue)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.option_value
  }

  /// Contains information about a Telegram Passport element
  pub fn passport_element(&self) -> &Option<Arc<dyn Fn((&Api, &PassportElement)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.passport_element
  }

  /// Contains notifications about data changes
  pub fn update(&self) -> &Option<Arc<dyn Fn((&Api, &Update)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update
  }

  /// Contains information about the period of inactivity after which the current user's account will automatically be deleted
  pub fn account_ttl(&self) -> &Option<Arc<dyn Fn((&Api, &AccountTtl)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.account_ttl
  }

  /// Represents a list of animations
  pub fn animations(&self) -> &Option<Arc<dyn Fn((&Api, &Animations)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.animations
  }

  /// Information about the authentication code that was sent
  pub fn authentication_code_info(&self) -> &Option<Arc<dyn Fn((&Api, &AuthenticationCodeInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.authentication_code_info
  }

  /// Represents a basic group of 0-200 users (must be upgraded to a supergroup to accommodate more than 200 users)
  pub fn basic_group(&self) -> &Option<Arc<dyn Fn((&Api, &BasicGroup)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.basic_group
  }

  /// Contains full information about a basic group
  pub fn basic_group_full_info(&self) -> &Option<Arc<dyn Fn((&Api, &BasicGroupFullInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.basic_group_full_info
  }

  /// Contains the call identifier
  pub fn call_id(&self) -> &Option<Arc<dyn Fn((&Api, &CallId)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.call_id
  }

  /// Contains a bot's answer to a callback query
  pub fn callback_query_answer(&self) -> &Option<Arc<dyn Fn((&Api, &CallbackQueryAnswer)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.callback_query_answer
  }

  /// A chat. (Can be a private chat, basic group, supergroup, or secret chat)
  pub fn chat(&self) -> &Option<Arc<dyn Fn((&Api, &Chat)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat
  }

  /// Contains a list of chat events
  pub fn chat_events(&self) -> &Option<Arc<dyn Fn((&Api, &ChatEvents)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_events
  }

  /// Contains a chat invite link
  pub fn chat_invite_link(&self) -> &Option<Arc<dyn Fn((&Api, &ChatInviteLink)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_invite_link
  }

  /// Contains information about a chat invite link
  pub fn chat_invite_link_info(&self) -> &Option<Arc<dyn Fn((&Api, &ChatInviteLinkInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_invite_link_info
  }

  /// A user with information about joining/leaving a chat
  pub fn chat_member(&self) -> &Option<Arc<dyn Fn((&Api, &ChatMember)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_member
  }

  /// Contains a list of chat members
  pub fn chat_members(&self) -> &Option<Arc<dyn Fn((&Api, &ChatMembers)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_members
  }

  /// Contains information about the availability of the "Report spam" action for a chat
  pub fn chat_report_spam_state(&self) -> &Option<Arc<dyn Fn((&Api, &ChatReportSpamState)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_report_spam_state
  }

  /// Represents a list of chats
  pub fn chats(&self) -> &Option<Arc<dyn Fn((&Api, &Chats)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chats
  }

  /// Contains a list of websites the current user is logged in with Telegram
  pub fn connected_websites(&self) -> &Option<Arc<dyn Fn((&Api, &ConnectedWebsites)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.connected_websites
  }

  /// Contains a counter
  pub fn count(&self) -> &Option<Arc<dyn Fn((&Api, &Count)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.count
  }

  /// Contains the result of a custom request
  pub fn custom_request_result(&self) -> &Option<Arc<dyn Fn((&Api, &CustomRequestResult)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.custom_request_result
  }

  /// Contains information about a tg:// deep link
  pub fn deep_link_info(&self) -> &Option<Arc<dyn Fn((&Api, &DeepLinkInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.deep_link_info
  }

  /// Information about the email address authentication code that was sent
  pub fn email_address_authentication_code_info(&self) -> &Option<Arc<dyn Fn((&Api, &EmailAddressAuthenticationCodeInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.email_address_authentication_code_info
  }

  /// An object of this type can be returned on every function call, in case of an error
  pub fn error(&self) -> &Option<Arc<dyn Fn((&Api, &Error)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.error
  }

  /// Represents a file
  pub fn file(&self) -> &Option<Arc<dyn Fn((&Api, &File)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.file
  }

  /// A text with some entities
  pub fn formatted_text(&self) -> &Option<Arc<dyn Fn((&Api, &FormattedText)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.formatted_text
  }

  /// Contains a list of messages found by a search
  pub fn found_messages(&self) -> &Option<Arc<dyn Fn((&Api, &FoundMessages)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.found_messages
  }

  /// Contains a list of game high scores
  pub fn game_high_scores(&self) -> &Option<Arc<dyn Fn((&Api, &GameHighScores)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.game_high_scores
  }

  /// Contains a list of hashtags
  pub fn hashtags(&self) -> &Option<Arc<dyn Fn((&Api, &Hashtags)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.hashtags
  }

  /// Represents the result of an ImportContacts request
  pub fn imported_contacts(&self) -> &Option<Arc<dyn Fn((&Api, &ImportedContacts)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.imported_contacts
  }

  /// Represents the results of the inline query. Use sendInlineQueryResultMessage to send the result of the query
  pub fn inline_query_results(&self) -> &Option<Arc<dyn Fn((&Api, &InlineQueryResults)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.inline_query_results
  }

  /// Contains a list of language pack strings
  pub fn language_pack_strings(&self) -> &Option<Arc<dyn Fn((&Api, &LanguagePackStrings)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.language_pack_strings
  }

  /// Contains information about the current localization target
  pub fn localization_target_info(&self) -> &Option<Arc<dyn Fn((&Api, &LocalizationTargetInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.localization_target_info
  }

  /// Describes a message
  pub fn message(&self) -> &Option<Arc<dyn Fn((&Api, &Message)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.message
  }

  /// Contains a list of messages
  pub fn messages(&self) -> &Option<Arc<dyn Fn((&Api, &Messages)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.messages
  }

  /// A full list of available network statistic entries
  pub fn network_statistics(&self) -> &Option<Arc<dyn Fn((&Api, &NetworkStatistics)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.network_statistics
  }

  /// An object of this type is returned on a successful function call for certain functions
  pub fn ok(&self) -> &Option<Arc<dyn Fn((&Api, &Ok)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.ok
  }

  /// Order information
  pub fn order_info(&self) -> &Option<Arc<dyn Fn((&Api, &OrderInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.order_info
  }

  /// Contains information about a Telegram Passport authorization form that was requested
  pub fn passport_authorization_form(&self) -> &Option<Arc<dyn Fn((&Api, &PassportAuthorizationForm)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.passport_authorization_form
  }

  /// Contains information about saved Telegram Passport elements
  pub fn passport_elements(&self) -> &Option<Arc<dyn Fn((&Api, &PassportElements)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.passport_elements
  }

  /// Represents the current state of 2-step verification
  pub fn password_state(&self) -> &Option<Arc<dyn Fn((&Api, &PasswordState)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.password_state
  }

  /// Contains information about an invoice payment form
  pub fn payment_form(&self) -> &Option<Arc<dyn Fn((&Api, &PaymentForm)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.payment_form
  }

  /// Contains information about a successful payment
  pub fn payment_receipt(&self) -> &Option<Arc<dyn Fn((&Api, &PaymentReceipt)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.payment_receipt
  }

  /// Contains the result of a payment request
  pub fn payment_result(&self) -> &Option<Arc<dyn Fn((&Api, &PaymentResult)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.payment_result
  }

  /// Represents a list of proxy servers
  pub fn proxies(&self) -> &Option<Arc<dyn Fn((&Api, &Proxies)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.proxies
  }

  /// Contains information about a proxy server
  pub fn proxy(&self) -> &Option<Arc<dyn Fn((&Api, &Proxy)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.proxy
  }

  /// Contains a public HTTPS link to a message in a public supergroup or channel
  pub fn public_message_link(&self) -> &Option<Arc<dyn Fn((&Api, &PublicMessageLink)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.public_message_link
  }

  /// Contains information about the current recovery email address
  pub fn recovery_email_address(&self) -> &Option<Arc<dyn Fn((&Api, &RecoveryEmailAddress)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.recovery_email_address
  }

  /// Contains information about notification settings for several chats
  pub fn scope_notification_settings(&self) -> &Option<Arc<dyn Fn((&Api, &ScopeNotificationSettings)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.scope_notification_settings
  }

  /// Contains a value representing a number of seconds
  pub fn seconds(&self) -> &Option<Arc<dyn Fn((&Api, &Seconds)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.seconds
  }

  /// Represents a secret chat
  pub fn secret_chat(&self) -> &Option<Arc<dyn Fn((&Api, &SecretChat)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.secret_chat
  }

  /// Contains a list of sessions
  pub fn sessions(&self) -> &Option<Arc<dyn Fn((&Api, &Sessions)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.sessions
  }

  /// Represents a list of all emoji corresponding to a sticker in a sticker set. The list is only for informational purposes, because a sticker is always sent with a fixed emoji from the corresponding Sticker object
  pub fn sticker_emojis(&self) -> &Option<Arc<dyn Fn((&Api, &StickerEmojis)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.sticker_emojis
  }

  /// Represents a sticker set
  pub fn sticker_set(&self) -> &Option<Arc<dyn Fn((&Api, &StickerSet)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.sticker_set
  }

  /// Represents a list of sticker sets
  pub fn sticker_sets(&self) -> &Option<Arc<dyn Fn((&Api, &StickerSets)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.sticker_sets
  }

  /// Represents a list of stickers
  pub fn stickers(&self) -> &Option<Arc<dyn Fn((&Api, &Stickers)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.stickers
  }

  /// Contains the exact storage usage statistics split by chats and file type
  pub fn storage_statistics(&self) -> &Option<Arc<dyn Fn((&Api, &StorageStatistics)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.storage_statistics
  }

  /// Contains approximate storage usage statistics, excluding files of unknown file type
  pub fn storage_statistics_fast(&self) -> &Option<Arc<dyn Fn((&Api, &StorageStatisticsFast)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.storage_statistics_fast
  }

  /// Represents a supergroup or channel with zero or more members (subscribers in the case of channels). From the point of view of the system, a channel is a special kind of a supergroup: only administrators can post and see the list of members, and posts from all administrators use the name and photo of the channel instead of individual names and profile photos. Unlike supergroups, channels can have an unlimited number of subscribers
  pub fn supergroup(&self) -> &Option<Arc<dyn Fn((&Api, &Supergroup)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.supergroup
  }

  /// Contains full information about a supergroup or channel
  pub fn supergroup_full_info(&self) -> &Option<Arc<dyn Fn((&Api, &SupergroupFullInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.supergroup_full_info
  }

  /// Contains a list of t.me URLs
  pub fn t_me_urls(&self) -> &Option<Arc<dyn Fn((&Api, &TMeUrls)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.t_me_urls
  }

  /// Returns information about the availability of a temporary password, which can be used for payments
  pub fn temporary_password_state(&self) -> &Option<Arc<dyn Fn((&Api, &TemporaryPasswordState)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.temporary_password_state
  }

  /// A simple object containing a sequence of bytes; for testing only
  pub fn test_bytes(&self) -> &Option<Arc<dyn Fn((&Api, &TestBytes)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.test_bytes
  }

  /// A simple object containing a number; for testing only
  pub fn test_int(&self) -> &Option<Arc<dyn Fn((&Api, &TestInt)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.test_int
  }

  /// A simple object containing a string; for testing only
  pub fn test_string(&self) -> &Option<Arc<dyn Fn((&Api, &TestString)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.test_string
  }

  /// A simple object containing a vector of numbers; for testing only
  pub fn test_vector_int(&self) -> &Option<Arc<dyn Fn((&Api, &TestVectorInt)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.test_vector_int
  }

  /// A simple object containing a vector of objects that hold a number; for testing only
  pub fn test_vector_int_object(&self) -> &Option<Arc<dyn Fn((&Api, &TestVectorIntObject)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.test_vector_int_object
  }

  /// A simple object containing a vector of strings; for testing only
  pub fn test_vector_string(&self) -> &Option<Arc<dyn Fn((&Api, &TestVectorString)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.test_vector_string
  }

  /// A simple object containing a vector of objects that hold a string; for testing only
  pub fn test_vector_string_object(&self) -> &Option<Arc<dyn Fn((&Api, &TestVectorStringObject)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.test_vector_string_object
  }

  /// Contains some text
  pub fn text(&self) -> &Option<Arc<dyn Fn((&Api, &Text)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.text
  }

  /// Contains a list of text entities
  pub fn text_entities(&self) -> &Option<Arc<dyn Fn((&Api, &TextEntities)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.text_entities
  }

  /// Represents a user
  pub fn user(&self) -> &Option<Arc<dyn Fn((&Api, &User)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.user
  }

  /// Contains full information about a user (except the full list of profile photos)
  pub fn user_full_info(&self) -> &Option<Arc<dyn Fn((&Api, &UserFullInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.user_full_info
  }

  /// A list of privacy rules. Rules are matched in the specified order. The first matched rule defines the privacy setting for a given user. If no rule matches, the action is not allowed
  pub fn user_privacy_setting_rules(&self) -> &Option<Arc<dyn Fn((&Api, &UserPrivacySettingRules)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.user_privacy_setting_rules
  }

  /// Contains part of the list of user photos
  pub fn user_profile_photos(&self) -> &Option<Arc<dyn Fn((&Api, &UserProfilePhotos)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.user_profile_photos
  }

  /// Represents a list of users
  pub fn users(&self) -> &Option<Arc<dyn Fn((&Api, &Users)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.users
  }

  /// Contains a temporary identifier of validated order information, which is stored for one hour. Also contains the available shipping options
  pub fn validated_order_info(&self) -> &Option<Arc<dyn Fn((&Api, &ValidatedOrderInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.validated_order_info
  }

  /// Contains a list of wallpapers
  pub fn wallpapers(&self) -> &Option<Arc<dyn Fn((&Api, &Wallpapers)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.wallpapers
  }

  /// Describes a web page preview
  pub fn web_page(&self) -> &Option<Arc<dyn Fn((&Api, &WebPage)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.web_page
  }

  /// Describes an instant view page for a web page
  pub fn web_page_instant_view(&self) -> &Option<Arc<dyn Fn((&Api, &WebPageInstantView)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.web_page_instant_view
  }


}



