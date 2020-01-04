use std::sync::Arc;

use rtdlib::types::*;
use crate::errors::*;
use crate::api::Api;


/// Telegram client event listener
#[derive(Clone, Default)]
pub struct Listener {
  exception: Option<Arc<dyn Fn((&Api, &TGError)) + Send + Sync + 'static>>,
  receive: Option<Arc<dyn Fn((&Api, &String)) -> TGResult<()> + Send + Sync + 'static>>,

  error: Option<Arc<dyn Fn((&Api, &Error)) -> TGResult<()> + Send + Sync + 'static>>,
  ok: Option<Arc<dyn Fn((&Api, &Ok)) -> TGResult<()> + Send + Sync + 'static>>,
  proxy: Option<Arc<dyn Fn((&Api, &Proxy)) -> TGResult<()> + Send + Sync + 'static>>,


  authorization_state: Option<Arc<dyn Fn((&Api, &UpdateAuthorizationState)) -> TGResult<()> + Send + Sync + 'static>>,
  new_message: Option<Arc<dyn Fn((&Api, &UpdateNewMessage)) -> TGResult<()> + Send + Sync + 'static>>,
  message_send_acknowledged: Option<Arc<dyn Fn((&Api, &UpdateMessageSendAcknowledged)) -> TGResult<()> + Send + Sync + 'static>>,
  message_send_succeeded: Option<Arc<dyn Fn((&Api, &UpdateMessageSendSucceeded)) -> TGResult<()> + Send + Sync + 'static>>,
  message_send_failed: Option<Arc<dyn Fn((&Api, &UpdateMessageSendFailed)) -> TGResult<()> + Send + Sync + 'static>>,
  message_content: Option<Arc<dyn Fn((&Api, &UpdateMessageContent)) -> TGResult<()> + Send + Sync + 'static>>,
  message_edited: Option<Arc<dyn Fn((&Api, &UpdateMessageEdited)) -> TGResult<()> + Send + Sync + 'static>>,
  message_views: Option<Arc<dyn Fn((&Api, &UpdateMessageViews)) -> TGResult<()> + Send + Sync + 'static>>,
  message_content_opened: Option<Arc<dyn Fn((&Api, &UpdateMessageContentOpened)) -> TGResult<()> + Send + Sync + 'static>>,
  message_mention_read: Option<Arc<dyn Fn((&Api, &UpdateMessageMentionRead)) -> TGResult<()> + Send + Sync + 'static>>,
  message_live_location_viewed: Option<Arc<dyn Fn((&Api, &UpdateMessageLiveLocationViewed)) -> TGResult<()> + Send + Sync + 'static>>,
  new_chat: Option<Arc<dyn Fn((&Api, &UpdateNewChat)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_chat_list: Option<Arc<dyn Fn((&Api, &UpdateChatChatList)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_title: Option<Arc<dyn Fn((&Api, &UpdateChatTitle)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_photo: Option<Arc<dyn Fn((&Api, &UpdateChatPhoto)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_permissions: Option<Arc<dyn Fn((&Api, &UpdateChatPermissions)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_last_message: Option<Arc<dyn Fn((&Api, &UpdateChatLastMessage)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_order: Option<Arc<dyn Fn((&Api, &UpdateChatOrder)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_is_pinned: Option<Arc<dyn Fn((&Api, &UpdateChatIsPinned)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_is_marked_as_unread: Option<Arc<dyn Fn((&Api, &UpdateChatIsMarkedAsUnread)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_is_sponsored: Option<Arc<dyn Fn((&Api, &UpdateChatIsSponsored)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_has_scheduled_messages: Option<Arc<dyn Fn((&Api, &UpdateChatHasScheduledMessages)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_default_disable_notification: Option<Arc<dyn Fn((&Api, &UpdateChatDefaultDisableNotification)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_read_inbox: Option<Arc<dyn Fn((&Api, &UpdateChatReadInbox)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_read_outbox: Option<Arc<dyn Fn((&Api, &UpdateChatReadOutbox)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_unread_mention_count: Option<Arc<dyn Fn((&Api, &UpdateChatUnreadMentionCount)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_notification_settings: Option<Arc<dyn Fn((&Api, &UpdateChatNotificationSettings)) -> TGResult<()> + Send + Sync + 'static>>,
  scope_notification_settings: Option<Arc<dyn Fn((&Api, &UpdateScopeNotificationSettings)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_action_bar: Option<Arc<dyn Fn((&Api, &UpdateChatActionBar)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_pinned_message: Option<Arc<dyn Fn((&Api, &UpdateChatPinnedMessage)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_reply_markup: Option<Arc<dyn Fn((&Api, &UpdateChatReplyMarkup)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_draft_message: Option<Arc<dyn Fn((&Api, &UpdateChatDraftMessage)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_online_member_count: Option<Arc<dyn Fn((&Api, &UpdateChatOnlineMemberCount)) -> TGResult<()> + Send + Sync + 'static>>,
  notification: Option<Arc<dyn Fn((&Api, &UpdateNotification)) -> TGResult<()> + Send + Sync + 'static>>,
  notification_group: Option<Arc<dyn Fn((&Api, &UpdateNotificationGroup)) -> TGResult<()> + Send + Sync + 'static>>,
  active_notifications: Option<Arc<dyn Fn((&Api, &UpdateActiveNotifications)) -> TGResult<()> + Send + Sync + 'static>>,
  have_pending_notifications: Option<Arc<dyn Fn((&Api, &UpdateHavePendingNotifications)) -> TGResult<()> + Send + Sync + 'static>>,
  delete_messages: Option<Arc<dyn Fn((&Api, &UpdateDeleteMessages)) -> TGResult<()> + Send + Sync + 'static>>,
  user_chat_action: Option<Arc<dyn Fn((&Api, &UpdateUserChatAction)) -> TGResult<()> + Send + Sync + 'static>>,
  user_status: Option<Arc<dyn Fn((&Api, &UpdateUserStatus)) -> TGResult<()> + Send + Sync + 'static>>,
  user: Option<Arc<dyn Fn((&Api, &UpdateUser)) -> TGResult<()> + Send + Sync + 'static>>,
  basic_group: Option<Arc<dyn Fn((&Api, &UpdateBasicGroup)) -> TGResult<()> + Send + Sync + 'static>>,
  supergroup: Option<Arc<dyn Fn((&Api, &UpdateSupergroup)) -> TGResult<()> + Send + Sync + 'static>>,
  secret_chat: Option<Arc<dyn Fn((&Api, &UpdateSecretChat)) -> TGResult<()> + Send + Sync + 'static>>,
  user_full_info: Option<Arc<dyn Fn((&Api, &UpdateUserFullInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  basic_group_full_info: Option<Arc<dyn Fn((&Api, &UpdateBasicGroupFullInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  supergroup_full_info: Option<Arc<dyn Fn((&Api, &UpdateSupergroupFullInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  service_notification: Option<Arc<dyn Fn((&Api, &UpdateServiceNotification)) -> TGResult<()> + Send + Sync + 'static>>,
  file: Option<Arc<dyn Fn((&Api, &UpdateFile)) -> TGResult<()> + Send + Sync + 'static>>,
  file_generation_start: Option<Arc<dyn Fn((&Api, &UpdateFileGenerationStart)) -> TGResult<()> + Send + Sync + 'static>>,
  file_generation_stop: Option<Arc<dyn Fn((&Api, &UpdateFileGenerationStop)) -> TGResult<()> + Send + Sync + 'static>>,
  call: Option<Arc<dyn Fn((&Api, &UpdateCall)) -> TGResult<()> + Send + Sync + 'static>>,
  user_privacy_setting_rules: Option<Arc<dyn Fn((&Api, &UpdateUserPrivacySettingRules)) -> TGResult<()> + Send + Sync + 'static>>,
  unread_message_count: Option<Arc<dyn Fn((&Api, &UpdateUnreadMessageCount)) -> TGResult<()> + Send + Sync + 'static>>,
  unread_chat_count: Option<Arc<dyn Fn((&Api, &UpdateUnreadChatCount)) -> TGResult<()> + Send + Sync + 'static>>,
  option: Option<Arc<dyn Fn((&Api, &UpdateOption)) -> TGResult<()> + Send + Sync + 'static>>,
  installed_sticker_sets: Option<Arc<dyn Fn((&Api, &UpdateInstalledStickerSets)) -> TGResult<()> + Send + Sync + 'static>>,
  trending_sticker_sets: Option<Arc<dyn Fn((&Api, &UpdateTrendingStickerSets)) -> TGResult<()> + Send + Sync + 'static>>,
  recent_stickers: Option<Arc<dyn Fn((&Api, &UpdateRecentStickers)) -> TGResult<()> + Send + Sync + 'static>>,
  favorite_stickers: Option<Arc<dyn Fn((&Api, &UpdateFavoriteStickers)) -> TGResult<()> + Send + Sync + 'static>>,
  saved_animations: Option<Arc<dyn Fn((&Api, &UpdateSavedAnimations)) -> TGResult<()> + Send + Sync + 'static>>,
  selected_background: Option<Arc<dyn Fn((&Api, &UpdateSelectedBackground)) -> TGResult<()> + Send + Sync + 'static>>,
  language_pack_strings: Option<Arc<dyn Fn((&Api, &UpdateLanguagePackStrings)) -> TGResult<()> + Send + Sync + 'static>>,
  connection_state: Option<Arc<dyn Fn((&Api, &UpdateConnectionState)) -> TGResult<()> + Send + Sync + 'static>>,
  terms_of_service: Option<Arc<dyn Fn((&Api, &UpdateTermsOfService)) -> TGResult<()> + Send + Sync + 'static>>,
  users_nearby: Option<Arc<dyn Fn((&Api, &UpdateUsersNearby)) -> TGResult<()> + Send + Sync + 'static>>,
  new_inline_query: Option<Arc<dyn Fn((&Api, &UpdateNewInlineQuery)) -> TGResult<()> + Send + Sync + 'static>>,
  new_chosen_inline_result: Option<Arc<dyn Fn((&Api, &UpdateNewChosenInlineResult)) -> TGResult<()> + Send + Sync + 'static>>,
  new_callback_query: Option<Arc<dyn Fn((&Api, &UpdateNewCallbackQuery)) -> TGResult<()> + Send + Sync + 'static>>,
  new_inline_callback_query: Option<Arc<dyn Fn((&Api, &UpdateNewInlineCallbackQuery)) -> TGResult<()> + Send + Sync + 'static>>,
  new_shipping_query: Option<Arc<dyn Fn((&Api, &UpdateNewShippingQuery)) -> TGResult<()> + Send + Sync + 'static>>,
  new_pre_checkout_query: Option<Arc<dyn Fn((&Api, &UpdateNewPreCheckoutQuery)) -> TGResult<()> + Send + Sync + 'static>>,
  new_custom_event: Option<Arc<dyn Fn((&Api, &UpdateNewCustomEvent)) -> TGResult<()> + Send + Sync + 'static>>,
  new_custom_query: Option<Arc<dyn Fn((&Api, &UpdateNewCustomQuery)) -> TGResult<()> + Send + Sync + 'static>>,
  poll: Option<Arc<dyn Fn((&Api, &UpdatePoll)) -> TGResult<()> + Send + Sync + 'static>>,
  test_use_update: Option<Arc<dyn Fn((&Api, &TestUseUpdate)) -> TGResult<()> + Send + Sync + 'static>>,


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


  /// An object of this type can be returned on every function call, in case of an error
  pub fn on_error<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&Api, &Error)) -> TGResult<()> + Send + Sync + 'static {
    self.error = Some(Arc::new(fnc));
    self
  }

  /// An object of this type is returned on a successful function call for certain functions
  pub fn on_ok<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&Api, &Ok)) -> TGResult<()> + Send + Sync + 'static {
    self.ok = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a proxy server
  pub fn on_proxy<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&Api, &Proxy)) -> TGResult<()> + Send + Sync + 'static {
    self.proxy = Some(Arc::new(fnc));
    self
  }





  /// The user authorization state has changed
  pub fn on_authorization_state<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateAuthorizationState)) -> TGResult<()> + Send + Sync + 'static {
    self.authorization_state = Some(Arc::new(fnc));
    self
  }

  /// A new message was received; can also be an outgoing message
  pub fn on_new_message<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateNewMessage)) -> TGResult<()> + Send + Sync + 'static {
    self.new_message = Some(Arc::new(fnc));
    self
  }

  /// A request to send a message has reached the Telegram server. This doesn't mean that the message will be sent successfully or even that the send message request will be processed. This update will be sent only if the option "use_quick_ack" is set to true. This update may be sent multiple times for the same message
  pub fn on_message_send_acknowledged<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateMessageSendAcknowledged)) -> TGResult<()> + Send + Sync + 'static {
    self.message_send_acknowledged = Some(Arc::new(fnc));
    self
  }

  /// A message has been successfully sent
  pub fn on_message_send_succeeded<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateMessageSendSucceeded)) -> TGResult<()> + Send + Sync + 'static {
    self.message_send_succeeded = Some(Arc::new(fnc));
    self
  }

  /// A message failed to send. Be aware that some messages being sent can be irrecoverably deleted, in which case updateDeleteMessages will be received instead of this update
  pub fn on_message_send_failed<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateMessageSendFailed)) -> TGResult<()> + Send + Sync + 'static {
    self.message_send_failed = Some(Arc::new(fnc));
    self
  }

  /// The message content has changed
  pub fn on_message_content<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateMessageContent)) -> TGResult<()> + Send + Sync + 'static {
    self.message_content = Some(Arc::new(fnc));
    self
  }

  /// A message was edited. Changes in the message content will come in a separate updateMessageContent
  pub fn on_message_edited<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateMessageEdited)) -> TGResult<()> + Send + Sync + 'static {
    self.message_edited = Some(Arc::new(fnc));
    self
  }

  /// The view count of the message has changed
  pub fn on_message_views<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateMessageViews)) -> TGResult<()> + Send + Sync + 'static {
    self.message_views = Some(Arc::new(fnc));
    self
  }

  /// The message content was opened. Updates voice note messages to "listened", video note messages to "viewed" and starts the TTL timer for self-destructing messages
  pub fn on_message_content_opened<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateMessageContentOpened)) -> TGResult<()> + Send + Sync + 'static {
    self.message_content_opened = Some(Arc::new(fnc));
    self
  }

  /// A message with an unread mention was read
  pub fn on_message_mention_read<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateMessageMentionRead)) -> TGResult<()> + Send + Sync + 'static {
    self.message_mention_read = Some(Arc::new(fnc));
    self
  }

  /// A message with a live location was viewed. When the update is received, the client is supposed to update the live location
  pub fn on_message_live_location_viewed<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateMessageLiveLocationViewed)) -> TGResult<()> + Send + Sync + 'static {
    self.message_live_location_viewed = Some(Arc::new(fnc));
    self
  }

  /// A new chat has been loaded/created. This update is guaranteed to come before the chat identifier is returned to the client. The chat field changes will be reported through separate updates
  pub fn on_new_chat<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateNewChat)) -> TGResult<()> + Send + Sync + 'static {
    self.new_chat = Some(Arc::new(fnc));
    self
  }

  /// The list to which the chat belongs was changed. This update is guaranteed to be sent only when chat.order == 0 and the current or the new chat list is null
  pub fn on_chat_chat_list<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatChatList)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_chat_list = Some(Arc::new(fnc));
    self
  }

  /// The title of a chat was changed
  pub fn on_chat_title<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatTitle)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_title = Some(Arc::new(fnc));
    self
  }

  /// A chat photo was changed
  pub fn on_chat_photo<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatPhoto)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_photo = Some(Arc::new(fnc));
    self
  }

  /// Chat permissions was changed
  pub fn on_chat_permissions<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatPermissions)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_permissions = Some(Arc::new(fnc));
    self
  }

  /// The last message of a chat was changed. If last_message is null, then the last message in the chat became unknown. Some new unknown messages might be added to the chat in this case
  pub fn on_chat_last_message<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatLastMessage)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_last_message = Some(Arc::new(fnc));
    self
  }

  /// The order of the chat in the chat list has changed. Instead of this update updateChatLastMessage, updateChatIsPinned, updateChatDraftMessage, or updateChatIsSponsored might be sent
  pub fn on_chat_order<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatOrder)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_order = Some(Arc::new(fnc));
    self
  }

  /// A chat was pinned or unpinned
  pub fn on_chat_is_pinned<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatIsPinned)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_is_pinned = Some(Arc::new(fnc));
    self
  }

  /// A chat was marked as unread or was read
  pub fn on_chat_is_marked_as_unread<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatIsMarkedAsUnread)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_is_marked_as_unread = Some(Arc::new(fnc));
    self
  }

  /// A chat's is_sponsored field has changed
  pub fn on_chat_is_sponsored<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatIsSponsored)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_is_sponsored = Some(Arc::new(fnc));
    self
  }

  /// A chat's has_scheduled_messages field has changed
  pub fn on_chat_has_scheduled_messages<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatHasScheduledMessages)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_has_scheduled_messages = Some(Arc::new(fnc));
    self
  }

  /// The value of the default disable_notification parameter, used when a message is sent to the chat, was changed
  pub fn on_chat_default_disable_notification<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatDefaultDisableNotification)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_default_disable_notification = Some(Arc::new(fnc));
    self
  }

  /// Incoming messages were read or number of unread messages has been changed
  pub fn on_chat_read_inbox<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatReadInbox)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_read_inbox = Some(Arc::new(fnc));
    self
  }

  /// Outgoing messages were read
  pub fn on_chat_read_outbox<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatReadOutbox)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_read_outbox = Some(Arc::new(fnc));
    self
  }

  /// The chat unread_mention_count has changed
  pub fn on_chat_unread_mention_count<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatUnreadMentionCount)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_unread_mention_count = Some(Arc::new(fnc));
    self
  }

  /// Notification settings for a chat were changed
  pub fn on_chat_notification_settings<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatNotificationSettings)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_notification_settings = Some(Arc::new(fnc));
    self
  }

  /// Notification settings for some type of chats were updated
  pub fn on_scope_notification_settings<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateScopeNotificationSettings)) -> TGResult<()> + Send + Sync + 'static {
    self.scope_notification_settings = Some(Arc::new(fnc));
    self
  }

  /// The chat action bar was changed
  pub fn on_chat_action_bar<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatActionBar)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_action_bar = Some(Arc::new(fnc));
    self
  }

  /// The chat pinned message was changed
  pub fn on_chat_pinned_message<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatPinnedMessage)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_pinned_message = Some(Arc::new(fnc));
    self
  }

  /// The default chat reply markup was changed. Can occur because new messages with reply markup were received or because an old reply markup was hidden by the user
  pub fn on_chat_reply_markup<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatReplyMarkup)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_reply_markup = Some(Arc::new(fnc));
    self
  }

  /// A chat draft has changed. Be aware that the update may come in the currently opened chat but with old content of the draft. If the user has changed the content of the draft, this update shouldn't be applied
  pub fn on_chat_draft_message<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatDraftMessage)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_draft_message = Some(Arc::new(fnc));
    self
  }

  /// The number of online group members has changed. This update with non-zero count is sent only for currently opened chats. There is no guarantee that it will be sent just after the count has changed
  pub fn on_chat_online_member_count<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatOnlineMemberCount)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_online_member_count = Some(Arc::new(fnc));
    self
  }

  /// A notification was changed
  pub fn on_notification<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateNotification)) -> TGResult<()> + Send + Sync + 'static {
    self.notification = Some(Arc::new(fnc));
    self
  }

  /// A list of active notifications in a notification group has changed
  pub fn on_notification_group<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateNotificationGroup)) -> TGResult<()> + Send + Sync + 'static {
    self.notification_group = Some(Arc::new(fnc));
    self
  }

  /// Contains active notifications that was shown on previous application launches. This update is sent only if a message database is used. In that case it comes once before any updateNotification and updateNotificationGroup update
  pub fn on_active_notifications<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateActiveNotifications)) -> TGResult<()> + Send + Sync + 'static {
    self.active_notifications = Some(Arc::new(fnc));
    self
  }

  /// Describes whether there are some pending notification updates. Can be used to prevent application from killing, while there are some pending notifications
  pub fn on_have_pending_notifications<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateHavePendingNotifications)) -> TGResult<()> + Send + Sync + 'static {
    self.have_pending_notifications = Some(Arc::new(fnc));
    self
  }

  /// Some messages were deleted
  pub fn on_delete_messages<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateDeleteMessages)) -> TGResult<()> + Send + Sync + 'static {
    self.delete_messages = Some(Arc::new(fnc));
    self
  }

  /// User activity in the chat has changed
  pub fn on_user_chat_action<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateUserChatAction)) -> TGResult<()> + Send + Sync + 'static {
    self.user_chat_action = Some(Arc::new(fnc));
    self
  }

  /// The user went online or offline
  pub fn on_user_status<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateUserStatus)) -> TGResult<()> + Send + Sync + 'static {
    self.user_status = Some(Arc::new(fnc));
    self
  }

  /// Some data of a user has changed. This update is guaranteed to come before the user identifier is returned to the client
  pub fn on_user<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateUser)) -> TGResult<()> + Send + Sync + 'static {
    self.user = Some(Arc::new(fnc));
    self
  }

  /// Some data of a basic group has changed. This update is guaranteed to come before the basic group identifier is returned to the client
  pub fn on_basic_group<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateBasicGroup)) -> TGResult<()> + Send + Sync + 'static {
    self.basic_group = Some(Arc::new(fnc));
    self
  }

  /// Some data of a supergroup or a channel has changed. This update is guaranteed to come before the supergroup identifier is returned to the client
  pub fn on_supergroup<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateSupergroup)) -> TGResult<()> + Send + Sync + 'static {
    self.supergroup = Some(Arc::new(fnc));
    self
  }

  /// Some data of a secret chat has changed. This update is guaranteed to come before the secret chat identifier is returned to the client
  pub fn on_secret_chat<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateSecretChat)) -> TGResult<()> + Send + Sync + 'static {
    self.secret_chat = Some(Arc::new(fnc));
    self
  }

  /// Some data from userFullInfo has been changed
  pub fn on_user_full_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateUserFullInfo)) -> TGResult<()> + Send + Sync + 'static {
    self.user_full_info = Some(Arc::new(fnc));
    self
  }

  /// Some data from basicGroupFullInfo has been changed
  pub fn on_basic_group_full_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateBasicGroupFullInfo)) -> TGResult<()> + Send + Sync + 'static {
    self.basic_group_full_info = Some(Arc::new(fnc));
    self
  }

  /// Some data from supergroupFullInfo has been changed
  pub fn on_supergroup_full_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateSupergroupFullInfo)) -> TGResult<()> + Send + Sync + 'static {
    self.supergroup_full_info = Some(Arc::new(fnc));
    self
  }

  /// Service notification from the server. Upon receiving this the client must show a popup with the content of the notification
  pub fn on_service_notification<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateServiceNotification)) -> TGResult<()> + Send + Sync + 'static {
    self.service_notification = Some(Arc::new(fnc));
    self
  }

  /// Information about a file was updated
  pub fn on_file<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateFile)) -> TGResult<()> + Send + Sync + 'static {
    self.file = Some(Arc::new(fnc));
    self
  }

  /// The file generation process needs to be started by the client
  pub fn on_file_generation_start<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateFileGenerationStart)) -> TGResult<()> + Send + Sync + 'static {
    self.file_generation_start = Some(Arc::new(fnc));
    self
  }

  /// File generation is no longer needed
  pub fn on_file_generation_stop<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateFileGenerationStop)) -> TGResult<()> + Send + Sync + 'static {
    self.file_generation_stop = Some(Arc::new(fnc));
    self
  }

  /// New call was created or information about a call was updated
  pub fn on_call<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateCall)) -> TGResult<()> + Send + Sync + 'static {
    self.call = Some(Arc::new(fnc));
    self
  }

  /// Some privacy setting rules have been changed
  pub fn on_user_privacy_setting_rules<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateUserPrivacySettingRules)) -> TGResult<()> + Send + Sync + 'static {
    self.user_privacy_setting_rules = Some(Arc::new(fnc));
    self
  }

  /// Number of unread messages in a chat list has changed. This update is sent only if a message database is used
  pub fn on_unread_message_count<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateUnreadMessageCount)) -> TGResult<()> + Send + Sync + 'static {
    self.unread_message_count = Some(Arc::new(fnc));
    self
  }

  /// Number of unread chats, i.e. with unread messages or marked as unread, has changed. This update is sent only if a message database is used
  pub fn on_unread_chat_count<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateUnreadChatCount)) -> TGResult<()> + Send + Sync + 'static {
    self.unread_chat_count = Some(Arc::new(fnc));
    self
  }

  /// An option changed its value
  pub fn on_option<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateOption)) -> TGResult<()> + Send + Sync + 'static {
    self.option = Some(Arc::new(fnc));
    self
  }

  /// The list of installed sticker sets was updated
  pub fn on_installed_sticker_sets<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateInstalledStickerSets)) -> TGResult<()> + Send + Sync + 'static {
    self.installed_sticker_sets = Some(Arc::new(fnc));
    self
  }

  /// The list of trending sticker sets was updated or some of them were viewed
  pub fn on_trending_sticker_sets<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateTrendingStickerSets)) -> TGResult<()> + Send + Sync + 'static {
    self.trending_sticker_sets = Some(Arc::new(fnc));
    self
  }

  /// The list of recently used stickers was updated
  pub fn on_recent_stickers<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateRecentStickers)) -> TGResult<()> + Send + Sync + 'static {
    self.recent_stickers = Some(Arc::new(fnc));
    self
  }

  /// The list of favorite stickers was updated
  pub fn on_favorite_stickers<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateFavoriteStickers)) -> TGResult<()> + Send + Sync + 'static {
    self.favorite_stickers = Some(Arc::new(fnc));
    self
  }

  /// The list of saved animations was updated
  pub fn on_saved_animations<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateSavedAnimations)) -> TGResult<()> + Send + Sync + 'static {
    self.saved_animations = Some(Arc::new(fnc));
    self
  }

  /// The selected background has changed
  pub fn on_selected_background<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateSelectedBackground)) -> TGResult<()> + Send + Sync + 'static {
    self.selected_background = Some(Arc::new(fnc));
    self
  }

  /// Some language pack strings have been updated
  pub fn on_language_pack_strings<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateLanguagePackStrings)) -> TGResult<()> + Send + Sync + 'static {
    self.language_pack_strings = Some(Arc::new(fnc));
    self
  }

  /// The connection state has changed
  pub fn on_connection_state<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateConnectionState)) -> TGResult<()> + Send + Sync + 'static {
    self.connection_state = Some(Arc::new(fnc));
    self
  }

  /// New terms of service must be accepted by the user. If the terms of service are declined, then the deleteAccount method should be called with the reason "Decline ToS update"
  pub fn on_terms_of_service<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateTermsOfService)) -> TGResult<()> + Send + Sync + 'static {
    self.terms_of_service = Some(Arc::new(fnc));
    self
  }

  /// List of users nearby has changed. The update is sent only 60 seconds after a successful searchChatsNearby request
  pub fn on_users_nearby<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateUsersNearby)) -> TGResult<()> + Send + Sync + 'static {
    self.users_nearby = Some(Arc::new(fnc));
    self
  }

  /// A new incoming inline query; for bots only
  pub fn on_new_inline_query<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateNewInlineQuery)) -> TGResult<()> + Send + Sync + 'static {
    self.new_inline_query = Some(Arc::new(fnc));
    self
  }

  /// The user has chosen a result of an inline query; for bots only
  pub fn on_new_chosen_inline_result<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateNewChosenInlineResult)) -> TGResult<()> + Send + Sync + 'static {
    self.new_chosen_inline_result = Some(Arc::new(fnc));
    self
  }

  /// A new incoming callback query; for bots only
  pub fn on_new_callback_query<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateNewCallbackQuery)) -> TGResult<()> + Send + Sync + 'static {
    self.new_callback_query = Some(Arc::new(fnc));
    self
  }

  /// A new incoming callback query from a message sent via a bot; for bots only
  pub fn on_new_inline_callback_query<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateNewInlineCallbackQuery)) -> TGResult<()> + Send + Sync + 'static {
    self.new_inline_callback_query = Some(Arc::new(fnc));
    self
  }

  /// A new incoming shipping query; for bots only. Only for invoices with flexible price
  pub fn on_new_shipping_query<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateNewShippingQuery)) -> TGResult<()> + Send + Sync + 'static {
    self.new_shipping_query = Some(Arc::new(fnc));
    self
  }

  /// A new incoming pre-checkout query; for bots only. Contains full information about a checkout
  pub fn on_new_pre_checkout_query<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateNewPreCheckoutQuery)) -> TGResult<()> + Send + Sync + 'static {
    self.new_pre_checkout_query = Some(Arc::new(fnc));
    self
  }

  /// A new incoming event; for bots only
  pub fn on_new_custom_event<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateNewCustomEvent)) -> TGResult<()> + Send + Sync + 'static {
    self.new_custom_event = Some(Arc::new(fnc));
    self
  }

  /// A new incoming query; for bots only
  pub fn on_new_custom_query<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateNewCustomQuery)) -> TGResult<()> + Send + Sync + 'static {
    self.new_custom_query = Some(Arc::new(fnc));
    self
  }

  /// Information about a poll was updated; for bots only
  pub fn on_poll<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdatePoll)) -> TGResult<()> + Send + Sync + 'static {
    self.poll = Some(Arc::new(fnc));
    self
  }

  /// Does nothing and ensures that the Update object is used; for testing only. This is an offline method. Can be called before authorization
  pub fn on_test_use_update<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &TestUseUpdate)) -> TGResult<()> + Send + Sync + 'static {
    self.test_use_update = Some(Arc::new(fnc));
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
      "error",
      "ok",
      "proxy",


      "updateAuthorizationState",
      "updateNewMessage",
      "updateMessageSendAcknowledged",
      "updateMessageSendSucceeded",
      "updateMessageSendFailed",
      "updateMessageContent",
      "updateMessageEdited",
      "updateMessageViews",
      "updateMessageContentOpened",
      "updateMessageMentionRead",
      "updateMessageLiveLocationViewed",
      "updateNewChat",
      "updateChatChatList",
      "updateChatTitle",
      "updateChatPhoto",
      "updateChatPermissions",
      "updateChatLastMessage",
      "updateChatOrder",
      "updateChatIsPinned",
      "updateChatIsMarkedAsUnread",
      "updateChatIsSponsored",
      "updateChatHasScheduledMessages",
      "updateChatDefaultDisableNotification",
      "updateChatReadInbox",
      "updateChatReadOutbox",
      "updateChatUnreadMentionCount",
      "updateChatNotificationSettings",
      "updateScopeNotificationSettings",
      "updateChatActionBar",
      "updateChatPinnedMessage",
      "updateChatReplyMarkup",
      "updateChatDraftMessage",
      "updateChatOnlineMemberCount",
      "updateNotification",
      "updateNotificationGroup",
      "updateActiveNotifications",
      "updateHavePendingNotifications",
      "updateDeleteMessages",
      "updateUserChatAction",
      "updateUserStatus",
      "updateUser",
      "updateBasicGroup",
      "updateSupergroup",
      "updateSecretChat",
      "updateUserFullInfo",
      "updateBasicGroupFullInfo",
      "updateSupergroupFullInfo",
      "updateServiceNotification",
      "updateFile",
      "updateFileGenerationStart",
      "updateFileGenerationStop",
      "updateCall",
      "updateUserPrivacySettingRules",
      "updateUnreadMessageCount",
      "updateUnreadChatCount",
      "updateOption",
      "updateInstalledStickerSets",
      "updateTrendingStickerSets",
      "updateRecentStickers",
      "updateFavoriteStickers",
      "updateSavedAnimations",
      "updateSelectedBackground",
      "updateLanguagePackStrings",
      "updateConnectionState",
      "updateTermsOfService",
      "updateUsersNearby",
      "updateNewInlineQuery",
      "updateNewChosenInlineResult",
      "updateNewCallbackQuery",
      "updateNewInlineCallbackQuery",
      "updateNewShippingQuery",
      "updateNewPreCheckoutQuery",
      "updateNewCustomEvent",
      "updateNewCustomQuery",
      "updatePoll",
      "testUseUpdate",

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


  /// An object of this type can be returned on every function call, in case of an error
  pub fn error(&self) -> &Option<Arc<dyn Fn((&Api, &Error)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.error
  }

  /// An object of this type is returned on a successful function call for certain functions
  pub fn ok(&self) -> &Option<Arc<dyn Fn((&Api, &Ok)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.ok
  }

  /// Contains information about a proxy server
  pub fn proxy(&self) -> &Option<Arc<dyn Fn((&Api, &Proxy)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.proxy
  }




  /// The user authorization state has changed
  pub fn authorization_state(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateAuthorizationState)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.authorization_state
  }

  /// A new message was received; can also be an outgoing message
  pub fn new_message(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateNewMessage)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.new_message
  }

  /// A request to send a message has reached the Telegram server. This doesn't mean that the message will be sent successfully or even that the send message request will be processed. This update will be sent only if the option "use_quick_ack" is set to true. This update may be sent multiple times for the same message
  pub fn message_send_acknowledged(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateMessageSendAcknowledged)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.message_send_acknowledged
  }

  /// A message has been successfully sent
  pub fn message_send_succeeded(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateMessageSendSucceeded)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.message_send_succeeded
  }

  /// A message failed to send. Be aware that some messages being sent can be irrecoverably deleted, in which case updateDeleteMessages will be received instead of this update
  pub fn message_send_failed(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateMessageSendFailed)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.message_send_failed
  }

  /// The message content has changed
  pub fn message_content(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateMessageContent)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.message_content
  }

  /// A message was edited. Changes in the message content will come in a separate updateMessageContent
  pub fn message_edited(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateMessageEdited)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.message_edited
  }

  /// The view count of the message has changed
  pub fn message_views(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateMessageViews)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.message_views
  }

  /// The message content was opened. Updates voice note messages to "listened", video note messages to "viewed" and starts the TTL timer for self-destructing messages
  pub fn message_content_opened(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateMessageContentOpened)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.message_content_opened
  }

  /// A message with an unread mention was read
  pub fn message_mention_read(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateMessageMentionRead)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.message_mention_read
  }

  /// A message with a live location was viewed. When the update is received, the client is supposed to update the live location
  pub fn message_live_location_viewed(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateMessageLiveLocationViewed)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.message_live_location_viewed
  }

  /// A new chat has been loaded/created. This update is guaranteed to come before the chat identifier is returned to the client. The chat field changes will be reported through separate updates
  pub fn new_chat(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateNewChat)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.new_chat
  }

  /// The list to which the chat belongs was changed. This update is guaranteed to be sent only when chat.order == 0 and the current or the new chat list is null
  pub fn chat_chat_list(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatChatList)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_chat_list
  }

  /// The title of a chat was changed
  pub fn chat_title(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatTitle)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_title
  }

  /// A chat photo was changed
  pub fn chat_photo(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatPhoto)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_photo
  }

  /// Chat permissions was changed
  pub fn chat_permissions(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatPermissions)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_permissions
  }

  /// The last message of a chat was changed. If last_message is null, then the last message in the chat became unknown. Some new unknown messages might be added to the chat in this case
  pub fn chat_last_message(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatLastMessage)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_last_message
  }

  /// The order of the chat in the chat list has changed. Instead of this update updateChatLastMessage, updateChatIsPinned, updateChatDraftMessage, or updateChatIsSponsored might be sent
  pub fn chat_order(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatOrder)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_order
  }

  /// A chat was pinned or unpinned
  pub fn chat_is_pinned(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatIsPinned)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_is_pinned
  }

  /// A chat was marked as unread or was read
  pub fn chat_is_marked_as_unread(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatIsMarkedAsUnread)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_is_marked_as_unread
  }

  /// A chat's is_sponsored field has changed
  pub fn chat_is_sponsored(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatIsSponsored)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_is_sponsored
  }

  /// A chat's has_scheduled_messages field has changed
  pub fn chat_has_scheduled_messages(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatHasScheduledMessages)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_has_scheduled_messages
  }

  /// The value of the default disable_notification parameter, used when a message is sent to the chat, was changed
  pub fn chat_default_disable_notification(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatDefaultDisableNotification)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_default_disable_notification
  }

  /// Incoming messages were read or number of unread messages has been changed
  pub fn chat_read_inbox(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatReadInbox)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_read_inbox
  }

  /// Outgoing messages were read
  pub fn chat_read_outbox(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatReadOutbox)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_read_outbox
  }

  /// The chat unread_mention_count has changed
  pub fn chat_unread_mention_count(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatUnreadMentionCount)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_unread_mention_count
  }

  /// Notification settings for a chat were changed
  pub fn chat_notification_settings(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatNotificationSettings)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_notification_settings
  }

  /// Notification settings for some type of chats were updated
  pub fn scope_notification_settings(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateScopeNotificationSettings)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.scope_notification_settings
  }

  /// The chat action bar was changed
  pub fn chat_action_bar(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatActionBar)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_action_bar
  }

  /// The chat pinned message was changed
  pub fn chat_pinned_message(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatPinnedMessage)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_pinned_message
  }

  /// The default chat reply markup was changed. Can occur because new messages with reply markup were received or because an old reply markup was hidden by the user
  pub fn chat_reply_markup(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatReplyMarkup)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_reply_markup
  }

  /// A chat draft has changed. Be aware that the update may come in the currently opened chat but with old content of the draft. If the user has changed the content of the draft, this update shouldn't be applied
  pub fn chat_draft_message(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatDraftMessage)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_draft_message
  }

  /// The number of online group members has changed. This update with non-zero count is sent only for currently opened chats. There is no guarantee that it will be sent just after the count has changed
  pub fn chat_online_member_count(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatOnlineMemberCount)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_online_member_count
  }

  /// A notification was changed
  pub fn notification(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateNotification)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.notification
  }

  /// A list of active notifications in a notification group has changed
  pub fn notification_group(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateNotificationGroup)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.notification_group
  }

  /// Contains active notifications that was shown on previous application launches. This update is sent only if a message database is used. In that case it comes once before any updateNotification and updateNotificationGroup update
  pub fn active_notifications(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateActiveNotifications)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.active_notifications
  }

  /// Describes whether there are some pending notification updates. Can be used to prevent application from killing, while there are some pending notifications
  pub fn have_pending_notifications(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateHavePendingNotifications)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.have_pending_notifications
  }

  /// Some messages were deleted
  pub fn delete_messages(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateDeleteMessages)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.delete_messages
  }

  /// User activity in the chat has changed
  pub fn user_chat_action(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateUserChatAction)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.user_chat_action
  }

  /// The user went online or offline
  pub fn user_status(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateUserStatus)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.user_status
  }

  /// Some data of a user has changed. This update is guaranteed to come before the user identifier is returned to the client
  pub fn user(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateUser)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.user
  }

  /// Some data of a basic group has changed. This update is guaranteed to come before the basic group identifier is returned to the client
  pub fn basic_group(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateBasicGroup)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.basic_group
  }

  /// Some data of a supergroup or a channel has changed. This update is guaranteed to come before the supergroup identifier is returned to the client
  pub fn supergroup(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateSupergroup)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.supergroup
  }

  /// Some data of a secret chat has changed. This update is guaranteed to come before the secret chat identifier is returned to the client
  pub fn secret_chat(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateSecretChat)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.secret_chat
  }

  /// Some data from userFullInfo has been changed
  pub fn user_full_info(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateUserFullInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.user_full_info
  }

  /// Some data from basicGroupFullInfo has been changed
  pub fn basic_group_full_info(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateBasicGroupFullInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.basic_group_full_info
  }

  /// Some data from supergroupFullInfo has been changed
  pub fn supergroup_full_info(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateSupergroupFullInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.supergroup_full_info
  }

  /// Service notification from the server. Upon receiving this the client must show a popup with the content of the notification
  pub fn service_notification(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateServiceNotification)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.service_notification
  }

  /// Information about a file was updated
  pub fn file(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateFile)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.file
  }

  /// The file generation process needs to be started by the client
  pub fn file_generation_start(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateFileGenerationStart)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.file_generation_start
  }

  /// File generation is no longer needed
  pub fn file_generation_stop(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateFileGenerationStop)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.file_generation_stop
  }

  /// New call was created or information about a call was updated
  pub fn call(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateCall)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.call
  }

  /// Some privacy setting rules have been changed
  pub fn user_privacy_setting_rules(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateUserPrivacySettingRules)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.user_privacy_setting_rules
  }

  /// Number of unread messages in a chat list has changed. This update is sent only if a message database is used
  pub fn unread_message_count(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateUnreadMessageCount)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.unread_message_count
  }

  /// Number of unread chats, i.e. with unread messages or marked as unread, has changed. This update is sent only if a message database is used
  pub fn unread_chat_count(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateUnreadChatCount)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.unread_chat_count
  }

  /// An option changed its value
  pub fn option(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateOption)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.option
  }

  /// The list of installed sticker sets was updated
  pub fn installed_sticker_sets(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateInstalledStickerSets)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.installed_sticker_sets
  }

  /// The list of trending sticker sets was updated or some of them were viewed
  pub fn trending_sticker_sets(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateTrendingStickerSets)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.trending_sticker_sets
  }

  /// The list of recently used stickers was updated
  pub fn recent_stickers(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateRecentStickers)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.recent_stickers
  }

  /// The list of favorite stickers was updated
  pub fn favorite_stickers(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateFavoriteStickers)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.favorite_stickers
  }

  /// The list of saved animations was updated
  pub fn saved_animations(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateSavedAnimations)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.saved_animations
  }

  /// The selected background has changed
  pub fn selected_background(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateSelectedBackground)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.selected_background
  }

  /// Some language pack strings have been updated
  pub fn language_pack_strings(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateLanguagePackStrings)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.language_pack_strings
  }

  /// The connection state has changed
  pub fn connection_state(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateConnectionState)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.connection_state
  }

  /// New terms of service must be accepted by the user. If the terms of service are declined, then the deleteAccount method should be called with the reason "Decline ToS update"
  pub fn terms_of_service(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateTermsOfService)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.terms_of_service
  }

  /// List of users nearby has changed. The update is sent only 60 seconds after a successful searchChatsNearby request
  pub fn users_nearby(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateUsersNearby)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.users_nearby
  }

  /// A new incoming inline query; for bots only
  pub fn new_inline_query(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateNewInlineQuery)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.new_inline_query
  }

  /// The user has chosen a result of an inline query; for bots only
  pub fn new_chosen_inline_result(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateNewChosenInlineResult)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.new_chosen_inline_result
  }

  /// A new incoming callback query; for bots only
  pub fn new_callback_query(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateNewCallbackQuery)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.new_callback_query
  }

  /// A new incoming callback query from a message sent via a bot; for bots only
  pub fn new_inline_callback_query(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateNewInlineCallbackQuery)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.new_inline_callback_query
  }

  /// A new incoming shipping query; for bots only. Only for invoices with flexible price
  pub fn new_shipping_query(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateNewShippingQuery)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.new_shipping_query
  }

  /// A new incoming pre-checkout query; for bots only. Contains full information about a checkout
  pub fn new_pre_checkout_query(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateNewPreCheckoutQuery)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.new_pre_checkout_query
  }

  /// A new incoming event; for bots only
  pub fn new_custom_event(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateNewCustomEvent)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.new_custom_event
  }

  /// A new incoming query; for bots only
  pub fn new_custom_query(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateNewCustomQuery)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.new_custom_query
  }

  /// Information about a poll was updated; for bots only
  pub fn poll(&self) -> &Option<Arc<dyn Fn((&Api, &UpdatePoll)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.poll
  }

  /// Does nothing and ensures that the Update object is used; for testing only. This is an offline method. Can be called before authorization
  pub fn test_use_update(&self) -> &Option<Arc<dyn Fn((&Api, &TestUseUpdate)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.test_use_update
  }

}



