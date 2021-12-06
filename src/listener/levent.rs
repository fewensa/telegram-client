use rtdlib::types::*;
use std::sync::Arc;

use crate::api::Api;
use crate::errors::*;

/// Telegram client event listener
#[derive(Clone, Default)]
pub struct EventListener {
  exception: Option<Arc<dyn Fn((&Api, &TGError)) + Send + Sync + 'static>>,
  receive: Option<Arc<dyn Fn((&Api, &String)) -> TGResult<()> + Send + Sync + 'static>>,



  test_use_update: Option<Arc<dyn Fn((&Api, &TestUseUpdate)) -> TGResult<()> + Send + Sync + 'static>>,
  update_active_notifications: Option<Arc<dyn Fn((&Api, &UpdateActiveNotifications)) -> TGResult<()> + Send + Sync + 'static>>,
  update_animated_emoji_message_clicked: Option<Arc<dyn Fn((&Api, &UpdateAnimatedEmojiMessageClicked)) -> TGResult<()> + Send + Sync + 'static>>,
  update_animation_search_parameters: Option<Arc<dyn Fn((&Api, &UpdateAnimationSearchParameters)) -> TGResult<()> + Send + Sync + 'static>>,
  update_authorization_state: Option<Arc<dyn Fn((&Api, &UpdateAuthorizationState)) -> TGResult<()> + Send + Sync + 'static>>,
  update_basic_group: Option<Arc<dyn Fn((&Api, &UpdateBasicGroup)) -> TGResult<()> + Send + Sync + 'static>>,
  update_basic_group_full_info: Option<Arc<dyn Fn((&Api, &UpdateBasicGroupFullInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  update_call: Option<Arc<dyn Fn((&Api, &UpdateCall)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_action_bar: Option<Arc<dyn Fn((&Api, &UpdateChatActionBar)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_default_disable_notification: Option<Arc<dyn Fn((&Api, &UpdateChatDefaultDisableNotification)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_draft_message: Option<Arc<dyn Fn((&Api, &UpdateChatDraftMessage)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_filters: Option<Arc<dyn Fn((&Api, &UpdateChatFilters)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_has_scheduled_messages: Option<Arc<dyn Fn((&Api, &UpdateChatHasScheduledMessages)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_is_blocked: Option<Arc<dyn Fn((&Api, &UpdateChatIsBlocked)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_is_marked_as_unread: Option<Arc<dyn Fn((&Api, &UpdateChatIsMarkedAsUnread)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_last_message: Option<Arc<dyn Fn((&Api, &UpdateChatLastMessage)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_member: Option<Arc<dyn Fn((&Api, &UpdateChatMember)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_message_ttl_setting: Option<Arc<dyn Fn((&Api, &UpdateChatMessageTtlSetting)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_notification_settings: Option<Arc<dyn Fn((&Api, &UpdateChatNotificationSettings)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_online_member_count: Option<Arc<dyn Fn((&Api, &UpdateChatOnlineMemberCount)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_pending_join_requests: Option<Arc<dyn Fn((&Api, &UpdateChatPendingJoinRequests)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_permissions: Option<Arc<dyn Fn((&Api, &UpdateChatPermissions)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_photo: Option<Arc<dyn Fn((&Api, &UpdateChatPhoto)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_position: Option<Arc<dyn Fn((&Api, &UpdateChatPosition)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_read_inbox: Option<Arc<dyn Fn((&Api, &UpdateChatReadInbox)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_read_outbox: Option<Arc<dyn Fn((&Api, &UpdateChatReadOutbox)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_reply_markup: Option<Arc<dyn Fn((&Api, &UpdateChatReplyMarkup)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_theme: Option<Arc<dyn Fn((&Api, &UpdateChatTheme)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_themes: Option<Arc<dyn Fn((&Api, &UpdateChatThemes)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_title: Option<Arc<dyn Fn((&Api, &UpdateChatTitle)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_unread_mention_count: Option<Arc<dyn Fn((&Api, &UpdateChatUnreadMentionCount)) -> TGResult<()> + Send + Sync + 'static>>,
  update_chat_video_chat: Option<Arc<dyn Fn((&Api, &UpdateChatVideoChat)) -> TGResult<()> + Send + Sync + 'static>>,
  update_connection_state: Option<Arc<dyn Fn((&Api, &UpdateConnectionState)) -> TGResult<()> + Send + Sync + 'static>>,
  update_delete_messages: Option<Arc<dyn Fn((&Api, &UpdateDeleteMessages)) -> TGResult<()> + Send + Sync + 'static>>,
  update_dice_emojis: Option<Arc<dyn Fn((&Api, &UpdateDiceEmojis)) -> TGResult<()> + Send + Sync + 'static>>,
  update_favorite_stickers: Option<Arc<dyn Fn((&Api, &UpdateFavoriteStickers)) -> TGResult<()> + Send + Sync + 'static>>,
  update_file: Option<Arc<dyn Fn((&Api, &UpdateFile)) -> TGResult<()> + Send + Sync + 'static>>,
  update_file_generation_start: Option<Arc<dyn Fn((&Api, &UpdateFileGenerationStart)) -> TGResult<()> + Send + Sync + 'static>>,
  update_file_generation_stop: Option<Arc<dyn Fn((&Api, &UpdateFileGenerationStop)) -> TGResult<()> + Send + Sync + 'static>>,
  update_group_call: Option<Arc<dyn Fn((&Api, &UpdateGroupCall)) -> TGResult<()> + Send + Sync + 'static>>,
  update_group_call_participant: Option<Arc<dyn Fn((&Api, &UpdateGroupCallParticipant)) -> TGResult<()> + Send + Sync + 'static>>,
  update_have_pending_notifications: Option<Arc<dyn Fn((&Api, &UpdateHavePendingNotifications)) -> TGResult<()> + Send + Sync + 'static>>,
  update_installed_sticker_sets: Option<Arc<dyn Fn((&Api, &UpdateInstalledStickerSets)) -> TGResult<()> + Send + Sync + 'static>>,
  update_language_pack_strings: Option<Arc<dyn Fn((&Api, &UpdateLanguagePackStrings)) -> TGResult<()> + Send + Sync + 'static>>,
  update_message_content: Option<Arc<dyn Fn((&Api, &UpdateMessageContent)) -> TGResult<()> + Send + Sync + 'static>>,
  update_message_content_opened: Option<Arc<dyn Fn((&Api, &UpdateMessageContentOpened)) -> TGResult<()> + Send + Sync + 'static>>,
  update_message_edited: Option<Arc<dyn Fn((&Api, &UpdateMessageEdited)) -> TGResult<()> + Send + Sync + 'static>>,
  update_message_interaction_info: Option<Arc<dyn Fn((&Api, &UpdateMessageInteractionInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  update_message_is_pinned: Option<Arc<dyn Fn((&Api, &UpdateMessageIsPinned)) -> TGResult<()> + Send + Sync + 'static>>,
  update_message_live_location_viewed: Option<Arc<dyn Fn((&Api, &UpdateMessageLiveLocationViewed)) -> TGResult<()> + Send + Sync + 'static>>,
  update_message_mention_read: Option<Arc<dyn Fn((&Api, &UpdateMessageMentionRead)) -> TGResult<()> + Send + Sync + 'static>>,
  update_message_send_acknowledged: Option<Arc<dyn Fn((&Api, &UpdateMessageSendAcknowledged)) -> TGResult<()> + Send + Sync + 'static>>,
  update_message_send_failed: Option<Arc<dyn Fn((&Api, &UpdateMessageSendFailed)) -> TGResult<()> + Send + Sync + 'static>>,
  update_message_send_succeeded: Option<Arc<dyn Fn((&Api, &UpdateMessageSendSucceeded)) -> TGResult<()> + Send + Sync + 'static>>,
  update_new_call_signaling_data: Option<Arc<dyn Fn((&Api, &UpdateNewCallSignalingData)) -> TGResult<()> + Send + Sync + 'static>>,
  update_new_callback_query: Option<Arc<dyn Fn((&Api, &UpdateNewCallbackQuery)) -> TGResult<()> + Send + Sync + 'static>>,
  update_new_chat: Option<Arc<dyn Fn((&Api, &UpdateNewChat)) -> TGResult<()> + Send + Sync + 'static>>,
  update_new_chat_join_request: Option<Arc<dyn Fn((&Api, &UpdateNewChatJoinRequest)) -> TGResult<()> + Send + Sync + 'static>>,
  update_new_chosen_inline_result: Option<Arc<dyn Fn((&Api, &UpdateNewChosenInlineResult)) -> TGResult<()> + Send + Sync + 'static>>,
  update_new_custom_event: Option<Arc<dyn Fn((&Api, &UpdateNewCustomEvent)) -> TGResult<()> + Send + Sync + 'static>>,
  update_new_custom_query: Option<Arc<dyn Fn((&Api, &UpdateNewCustomQuery)) -> TGResult<()> + Send + Sync + 'static>>,
  update_new_inline_callback_query: Option<Arc<dyn Fn((&Api, &UpdateNewInlineCallbackQuery)) -> TGResult<()> + Send + Sync + 'static>>,
  update_new_inline_query: Option<Arc<dyn Fn((&Api, &UpdateNewInlineQuery)) -> TGResult<()> + Send + Sync + 'static>>,
  update_new_message: Option<Arc<dyn Fn((&Api, &UpdateNewMessage)) -> TGResult<()> + Send + Sync + 'static>>,
  update_new_pre_checkout_query: Option<Arc<dyn Fn((&Api, &UpdateNewPreCheckoutQuery)) -> TGResult<()> + Send + Sync + 'static>>,
  update_new_shipping_query: Option<Arc<dyn Fn((&Api, &UpdateNewShippingQuery)) -> TGResult<()> + Send + Sync + 'static>>,
  update_notification: Option<Arc<dyn Fn((&Api, &UpdateNotification)) -> TGResult<()> + Send + Sync + 'static>>,
  update_notification_group: Option<Arc<dyn Fn((&Api, &UpdateNotificationGroup)) -> TGResult<()> + Send + Sync + 'static>>,
  update_option: Option<Arc<dyn Fn((&Api, &UpdateOption)) -> TGResult<()> + Send + Sync + 'static>>,
  update_poll: Option<Arc<dyn Fn((&Api, &UpdatePoll)) -> TGResult<()> + Send + Sync + 'static>>,
  update_poll_answer: Option<Arc<dyn Fn((&Api, &UpdatePollAnswer)) -> TGResult<()> + Send + Sync + 'static>>,
  update_recent_stickers: Option<Arc<dyn Fn((&Api, &UpdateRecentStickers)) -> TGResult<()> + Send + Sync + 'static>>,
  update_saved_animations: Option<Arc<dyn Fn((&Api, &UpdateSavedAnimations)) -> TGResult<()> + Send + Sync + 'static>>,
  update_scope_notification_settings: Option<Arc<dyn Fn((&Api, &UpdateScopeNotificationSettings)) -> TGResult<()> + Send + Sync + 'static>>,
  update_secret_chat: Option<Arc<dyn Fn((&Api, &UpdateSecretChat)) -> TGResult<()> + Send + Sync + 'static>>,
  update_selected_background: Option<Arc<dyn Fn((&Api, &UpdateSelectedBackground)) -> TGResult<()> + Send + Sync + 'static>>,
  update_service_notification: Option<Arc<dyn Fn((&Api, &UpdateServiceNotification)) -> TGResult<()> + Send + Sync + 'static>>,
  update_sticker_set: Option<Arc<dyn Fn((&Api, &UpdateStickerSet)) -> TGResult<()> + Send + Sync + 'static>>,
  update_suggested_actions: Option<Arc<dyn Fn((&Api, &UpdateSuggestedActions)) -> TGResult<()> + Send + Sync + 'static>>,
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
  update_users_nearby: Option<Arc<dyn Fn((&Api, &UpdateUsersNearby)) -> TGResult<()> + Send + Sync + 'static>>,


  authorization_state: Option<Arc<dyn Fn((&Api, &AuthorizationState)) -> TGResult<()> + Send + Sync + 'static>>,
  can_transfer_ownership_result: Option<Arc<dyn Fn((&Api, &CanTransferOwnershipResult)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_statistics: Option<Arc<dyn Fn((&Api, &ChatStatistics)) -> TGResult<()> + Send + Sync + 'static>>,
  check_chat_username_result: Option<Arc<dyn Fn((&Api, &CheckChatUsernameResult)) -> TGResult<()> + Send + Sync + 'static>>,
  check_sticker_set_name_result: Option<Arc<dyn Fn((&Api, &CheckStickerSetNameResult)) -> TGResult<()> + Send + Sync + 'static>>,
  internal_link_type: Option<Arc<dyn Fn((&Api, &InternalLinkType)) -> TGResult<()> + Send + Sync + 'static>>,
  json_value: Option<Arc<dyn Fn((&Api, &JsonValue)) -> TGResult<()> + Send + Sync + 'static>>,
  language_pack_string_value: Option<Arc<dyn Fn((&Api, &LanguagePackStringValue)) -> TGResult<()> + Send + Sync + 'static>>,
  log_stream: Option<Arc<dyn Fn((&Api, &LogStream)) -> TGResult<()> + Send + Sync + 'static>>,
  login_url_info: Option<Arc<dyn Fn((&Api, &LoginUrlInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  message_file_type: Option<Arc<dyn Fn((&Api, &MessageFileType)) -> TGResult<()> + Send + Sync + 'static>>,
  option_value: Option<Arc<dyn Fn((&Api, &OptionValue)) -> TGResult<()> + Send + Sync + 'static>>,
  passport_element: Option<Arc<dyn Fn((&Api, &PassportElement)) -> TGResult<()> + Send + Sync + 'static>>,
  reset_password_result: Option<Arc<dyn Fn((&Api, &ResetPasswordResult)) -> TGResult<()> + Send + Sync + 'static>>,
  statistical_graph: Option<Arc<dyn Fn((&Api, &StatisticalGraph)) -> TGResult<()> + Send + Sync + 'static>>,
  update: Option<Arc<dyn Fn((&Api, &Update)) -> TGResult<()> + Send + Sync + 'static>>,
  account_ttl: Option<Arc<dyn Fn((&Api, &AccountTtl)) -> TGResult<()> + Send + Sync + 'static>>,
  animated_emoji: Option<Arc<dyn Fn((&Api, &AnimatedEmoji)) -> TGResult<()> + Send + Sync + 'static>>,
  animations: Option<Arc<dyn Fn((&Api, &Animations)) -> TGResult<()> + Send + Sync + 'static>>,
  authentication_code_info: Option<Arc<dyn Fn((&Api, &AuthenticationCodeInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  auto_download_settings_presets: Option<Arc<dyn Fn((&Api, &AutoDownloadSettingsPresets)) -> TGResult<()> + Send + Sync + 'static>>,
  background: Option<Arc<dyn Fn((&Api, &Background)) -> TGResult<()> + Send + Sync + 'static>>,
  backgrounds: Option<Arc<dyn Fn((&Api, &Backgrounds)) -> TGResult<()> + Send + Sync + 'static>>,
  bank_card_info: Option<Arc<dyn Fn((&Api, &BankCardInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  basic_group: Option<Arc<dyn Fn((&Api, &BasicGroup)) -> TGResult<()> + Send + Sync + 'static>>,
  basic_group_full_info: Option<Arc<dyn Fn((&Api, &BasicGroupFullInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  bot_commands: Option<Arc<dyn Fn((&Api, &BotCommands)) -> TGResult<()> + Send + Sync + 'static>>,
  call_id: Option<Arc<dyn Fn((&Api, &CallId)) -> TGResult<()> + Send + Sync + 'static>>,
  callback_query_answer: Option<Arc<dyn Fn((&Api, &CallbackQueryAnswer)) -> TGResult<()> + Send + Sync + 'static>>,
  chat: Option<Arc<dyn Fn((&Api, &Chat)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_administrators: Option<Arc<dyn Fn((&Api, &ChatAdministrators)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_events: Option<Arc<dyn Fn((&Api, &ChatEvents)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_filter: Option<Arc<dyn Fn((&Api, &ChatFilter)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_filter_info: Option<Arc<dyn Fn((&Api, &ChatFilterInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_invite_link: Option<Arc<dyn Fn((&Api, &ChatInviteLink)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_invite_link_counts: Option<Arc<dyn Fn((&Api, &ChatInviteLinkCounts)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_invite_link_info: Option<Arc<dyn Fn((&Api, &ChatInviteLinkInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_invite_link_members: Option<Arc<dyn Fn((&Api, &ChatInviteLinkMembers)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_invite_links: Option<Arc<dyn Fn((&Api, &ChatInviteLinks)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_join_requests: Option<Arc<dyn Fn((&Api, &ChatJoinRequests)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_lists: Option<Arc<dyn Fn((&Api, &ChatLists)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_member: Option<Arc<dyn Fn((&Api, &ChatMember)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_members: Option<Arc<dyn Fn((&Api, &ChatMembers)) -> TGResult<()> + Send + Sync + 'static>>,
  chat_photos: Option<Arc<dyn Fn((&Api, &ChatPhotos)) -> TGResult<()> + Send + Sync + 'static>>,
  chats: Option<Arc<dyn Fn((&Api, &Chats)) -> TGResult<()> + Send + Sync + 'static>>,
  chats_nearby: Option<Arc<dyn Fn((&Api, &ChatsNearby)) -> TGResult<()> + Send + Sync + 'static>>,
  connected_websites: Option<Arc<dyn Fn((&Api, &ConnectedWebsites)) -> TGResult<()> + Send + Sync + 'static>>,
  count: Option<Arc<dyn Fn((&Api, &Count)) -> TGResult<()> + Send + Sync + 'static>>,
  countries: Option<Arc<dyn Fn((&Api, &Countries)) -> TGResult<()> + Send + Sync + 'static>>,
  custom_request_result: Option<Arc<dyn Fn((&Api, &CustomRequestResult)) -> TGResult<()> + Send + Sync + 'static>>,
  database_statistics: Option<Arc<dyn Fn((&Api, &DatabaseStatistics)) -> TGResult<()> + Send + Sync + 'static>>,
  deep_link_info: Option<Arc<dyn Fn((&Api, &DeepLinkInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  email_address_authentication_code_info: Option<Arc<dyn Fn((&Api, &EmailAddressAuthenticationCodeInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  emojis: Option<Arc<dyn Fn((&Api, &Emojis)) -> TGResult<()> + Send + Sync + 'static>>,
  error: Option<Arc<dyn Fn((&Api, &Error)) -> TGResult<()> + Send + Sync + 'static>>,
  file: Option<Arc<dyn Fn((&Api, &File)) -> TGResult<()> + Send + Sync + 'static>>,
  file_part: Option<Arc<dyn Fn((&Api, &FilePart)) -> TGResult<()> + Send + Sync + 'static>>,
  formatted_text: Option<Arc<dyn Fn((&Api, &FormattedText)) -> TGResult<()> + Send + Sync + 'static>>,
  found_messages: Option<Arc<dyn Fn((&Api, &FoundMessages)) -> TGResult<()> + Send + Sync + 'static>>,
  game_high_scores: Option<Arc<dyn Fn((&Api, &GameHighScores)) -> TGResult<()> + Send + Sync + 'static>>,
  group_call: Option<Arc<dyn Fn((&Api, &GroupCall)) -> TGResult<()> + Send + Sync + 'static>>,
  group_call_id: Option<Arc<dyn Fn((&Api, &GroupCallId)) -> TGResult<()> + Send + Sync + 'static>>,
  hashtags: Option<Arc<dyn Fn((&Api, &Hashtags)) -> TGResult<()> + Send + Sync + 'static>>,
  http_url: Option<Arc<dyn Fn((&Api, &HttpUrl)) -> TGResult<()> + Send + Sync + 'static>>,
  imported_contacts: Option<Arc<dyn Fn((&Api, &ImportedContacts)) -> TGResult<()> + Send + Sync + 'static>>,
  inline_query_results: Option<Arc<dyn Fn((&Api, &InlineQueryResults)) -> TGResult<()> + Send + Sync + 'static>>,
  language_pack_info: Option<Arc<dyn Fn((&Api, &LanguagePackInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  language_pack_strings: Option<Arc<dyn Fn((&Api, &LanguagePackStrings)) -> TGResult<()> + Send + Sync + 'static>>,
  localization_target_info: Option<Arc<dyn Fn((&Api, &LocalizationTargetInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  log_tags: Option<Arc<dyn Fn((&Api, &LogTags)) -> TGResult<()> + Send + Sync + 'static>>,
  log_verbosity_level: Option<Arc<dyn Fn((&Api, &LogVerbosityLevel)) -> TGResult<()> + Send + Sync + 'static>>,
  message: Option<Arc<dyn Fn((&Api, &Message)) -> TGResult<()> + Send + Sync + 'static>>,
  message_calendar: Option<Arc<dyn Fn((&Api, &MessageCalendar)) -> TGResult<()> + Send + Sync + 'static>>,
  message_link: Option<Arc<dyn Fn((&Api, &MessageLink)) -> TGResult<()> + Send + Sync + 'static>>,
  message_link_info: Option<Arc<dyn Fn((&Api, &MessageLinkInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  message_positions: Option<Arc<dyn Fn((&Api, &MessagePositions)) -> TGResult<()> + Send + Sync + 'static>>,
  message_senders: Option<Arc<dyn Fn((&Api, &MessageSenders)) -> TGResult<()> + Send + Sync + 'static>>,
  message_statistics: Option<Arc<dyn Fn((&Api, &MessageStatistics)) -> TGResult<()> + Send + Sync + 'static>>,
  message_thread_info: Option<Arc<dyn Fn((&Api, &MessageThreadInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  messages: Option<Arc<dyn Fn((&Api, &Messages)) -> TGResult<()> + Send + Sync + 'static>>,
  network_statistics: Option<Arc<dyn Fn((&Api, &NetworkStatistics)) -> TGResult<()> + Send + Sync + 'static>>,
  ok: Option<Arc<dyn Fn((&Api, &Ok)) -> TGResult<()> + Send + Sync + 'static>>,
  order_info: Option<Arc<dyn Fn((&Api, &OrderInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  passport_authorization_form: Option<Arc<dyn Fn((&Api, &PassportAuthorizationForm)) -> TGResult<()> + Send + Sync + 'static>>,
  passport_elements: Option<Arc<dyn Fn((&Api, &PassportElements)) -> TGResult<()> + Send + Sync + 'static>>,
  passport_elements_with_errors: Option<Arc<dyn Fn((&Api, &PassportElementsWithErrors)) -> TGResult<()> + Send + Sync + 'static>>,
  password_state: Option<Arc<dyn Fn((&Api, &PasswordState)) -> TGResult<()> + Send + Sync + 'static>>,
  payment_form: Option<Arc<dyn Fn((&Api, &PaymentForm)) -> TGResult<()> + Send + Sync + 'static>>,
  payment_receipt: Option<Arc<dyn Fn((&Api, &PaymentReceipt)) -> TGResult<()> + Send + Sync + 'static>>,
  payment_result: Option<Arc<dyn Fn((&Api, &PaymentResult)) -> TGResult<()> + Send + Sync + 'static>>,
  phone_number_info: Option<Arc<dyn Fn((&Api, &PhoneNumberInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  proxies: Option<Arc<dyn Fn((&Api, &Proxies)) -> TGResult<()> + Send + Sync + 'static>>,
  proxy: Option<Arc<dyn Fn((&Api, &Proxy)) -> TGResult<()> + Send + Sync + 'static>>,
  push_receiver_id: Option<Arc<dyn Fn((&Api, &PushReceiverId)) -> TGResult<()> + Send + Sync + 'static>>,
  recommended_chat_filters: Option<Arc<dyn Fn((&Api, &RecommendedChatFilters)) -> TGResult<()> + Send + Sync + 'static>>,
  recovery_email_address: Option<Arc<dyn Fn((&Api, &RecoveryEmailAddress)) -> TGResult<()> + Send + Sync + 'static>>,
  scope_notification_settings: Option<Arc<dyn Fn((&Api, &ScopeNotificationSettings)) -> TGResult<()> + Send + Sync + 'static>>,
  seconds: Option<Arc<dyn Fn((&Api, &Seconds)) -> TGResult<()> + Send + Sync + 'static>>,
  secret_chat: Option<Arc<dyn Fn((&Api, &SecretChat)) -> TGResult<()> + Send + Sync + 'static>>,
  session: Option<Arc<dyn Fn((&Api, &Session)) -> TGResult<()> + Send + Sync + 'static>>,
  sessions: Option<Arc<dyn Fn((&Api, &Sessions)) -> TGResult<()> + Send + Sync + 'static>>,
  sponsored_messages: Option<Arc<dyn Fn((&Api, &SponsoredMessages)) -> TGResult<()> + Send + Sync + 'static>>,
  sticker: Option<Arc<dyn Fn((&Api, &Sticker)) -> TGResult<()> + Send + Sync + 'static>>,
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
  updates: Option<Arc<dyn Fn((&Api, &Updates)) -> TGResult<()> + Send + Sync + 'static>>,
  user: Option<Arc<dyn Fn((&Api, &User)) -> TGResult<()> + Send + Sync + 'static>>,
  user_full_info: Option<Arc<dyn Fn((&Api, &UserFullInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  user_privacy_setting_rules: Option<Arc<dyn Fn((&Api, &UserPrivacySettingRules)) -> TGResult<()> + Send + Sync + 'static>>,
  users: Option<Arc<dyn Fn((&Api, &Users)) -> TGResult<()> + Send + Sync + 'static>>,
  validated_order_info: Option<Arc<dyn Fn((&Api, &ValidatedOrderInfo)) -> TGResult<()> + Send + Sync + 'static>>,
  web_page: Option<Arc<dyn Fn((&Api, &WebPage)) -> TGResult<()> + Send + Sync + 'static>>,
  web_page_instant_view: Option<Arc<dyn Fn((&Api, &WebPageInstantView)) -> TGResult<()> + Send + Sync + 'static>>,

}


impl EventListener {
  pub fn new() -> Self { EventListener::default() }

  #[allow(dead_code)]
  pub(crate) fn has_receive_listen(&self) -> bool { self.receive.is_some() }

  pub(crate) fn lout(&self) -> EventLout { EventLout::new(self.clone()) }


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






  /// Does nothing and ensures that the Update object is used; for testing only. This is an offline method. Can be called before authorization
  pub fn on_test_use_update<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &TestUseUpdate)) -> TGResult<()> + Send + Sync + 'static {
    self.test_use_update = Some(Arc::new(fnc));
    self
  }

  /// Contains active notifications that was shown on previous application launches. This update is sent only if the message database is used. In that case it comes once before any updateNotification and updateNotificationGroup update
  pub fn on_update_active_notifications<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateActiveNotifications)) -> TGResult<()> + Send + Sync + 'static {
    self.update_active_notifications = Some(Arc::new(fnc));
    self
  }

  /// Some animated emoji message was clicked and a big animated sticker must be played if the message is visible on the screen. chatActionWatchingAnimations with the text of the message needs to be sent if the sticker is played
  pub fn on_update_animated_emoji_message_clicked<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateAnimatedEmojiMessageClicked)) -> TGResult<()> + Send + Sync + 'static {
    self.update_animated_emoji_message_clicked = Some(Arc::new(fnc));
    self
  }

  /// The parameters of animation search through GetOption("animation_search_bot_username") bot has changed
  pub fn on_update_animation_search_parameters<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateAnimationSearchParameters)) -> TGResult<()> + Send + Sync + 'static {
    self.update_animation_search_parameters = Some(Arc::new(fnc));
    self
  }

  /// The user authorization state has changed
  pub fn on_update_authorization_state<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateAuthorizationState)) -> TGResult<()> + Send + Sync + 'static {
    self.update_authorization_state = Some(Arc::new(fnc));
    self
  }

  /// Some data of a basic group has changed. This update is guaranteed to come before the basic group identifier is returned to the application
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

  /// The chat action bar was changed
  pub fn on_update_chat_action_bar<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatActionBar)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_action_bar = Some(Arc::new(fnc));
    self
  }

  /// The value of the default disable_notification parameter, used when a message is sent to the chat, was changed
  pub fn on_update_chat_default_disable_notification<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatDefaultDisableNotification)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_default_disable_notification = Some(Arc::new(fnc));
    self
  }

  /// A chat draft has changed. Be aware that the update may come in the currently opened chat but with old content of the draft. If the user has changed the content of the draft, this update mustn't be applied
  pub fn on_update_chat_draft_message<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatDraftMessage)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_draft_message = Some(Arc::new(fnc));
    self
  }

  /// The list of chat filters or a chat filter has changed
  pub fn on_update_chat_filters<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatFilters)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_filters = Some(Arc::new(fnc));
    self
  }

  /// A chat's has_scheduled_messages field has changed
  pub fn on_update_chat_has_scheduled_messages<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatHasScheduledMessages)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_has_scheduled_messages = Some(Arc::new(fnc));
    self
  }

  /// A chat was blocked or unblocked
  pub fn on_update_chat_is_blocked<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatIsBlocked)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_is_blocked = Some(Arc::new(fnc));
    self
  }

  /// A chat was marked as unread or was read
  pub fn on_update_chat_is_marked_as_unread<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatIsMarkedAsUnread)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_is_marked_as_unread = Some(Arc::new(fnc));
    self
  }

  /// The last message of a chat was changed. If last_message is null, then the last message in the chat became unknown. Some new unknown messages might be added to the chat in this case
  pub fn on_update_chat_last_message<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatLastMessage)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_last_message = Some(Arc::new(fnc));
    self
  }

  /// User rights changed in a chat; for bots only
  pub fn on_update_chat_member<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatMember)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_member = Some(Arc::new(fnc));
    self
  }

  /// The message Time To Live setting for a chat was changed
  pub fn on_update_chat_message_ttl_setting<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatMessageTtlSetting)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_message_ttl_setting = Some(Arc::new(fnc));
    self
  }

  /// Notification settings for a chat were changed
  pub fn on_update_chat_notification_settings<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatNotificationSettings)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_notification_settings = Some(Arc::new(fnc));
    self
  }

  /// The number of online group members has changed. This update with non-zero count is sent only for currently opened chats. There is no guarantee that it will be sent just after the count has changed
  pub fn on_update_chat_online_member_count<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatOnlineMemberCount)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_online_member_count = Some(Arc::new(fnc));
    self
  }

  /// The chat pending join requests were changed
  pub fn on_update_chat_pending_join_requests<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatPendingJoinRequests)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_pending_join_requests = Some(Arc::new(fnc));
    self
  }

  /// Chat permissions was changed
  pub fn on_update_chat_permissions<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatPermissions)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_permissions = Some(Arc::new(fnc));
    self
  }

  /// A chat photo was changed
  pub fn on_update_chat_photo<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatPhoto)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_photo = Some(Arc::new(fnc));
    self
  }

  /// The position of a chat in a chat list has changed. Instead of this update updateChatLastMessage or updateChatDraftMessage might be sent
  pub fn on_update_chat_position<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatPosition)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_position = Some(Arc::new(fnc));
    self
  }

  /// Incoming messages were read or the number of unread messages has been changed
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

  /// The chat theme was changed
  pub fn on_update_chat_theme<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatTheme)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_theme = Some(Arc::new(fnc));
    self
  }

  /// The list of available chat themes has changed
  pub fn on_update_chat_themes<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatThemes)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_themes = Some(Arc::new(fnc));
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

  /// A chat video chat state has changed
  pub fn on_update_chat_video_chat<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateChatVideoChat)) -> TGResult<()> + Send + Sync + 'static {
    self.update_chat_video_chat = Some(Arc::new(fnc));
    self
  }

  /// The connection state has changed. This update must be used only to show a human-readable description of the connection state
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

  /// The list of supported dice emojis has changed
  pub fn on_update_dice_emojis<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateDiceEmojis)) -> TGResult<()> + Send + Sync + 'static {
    self.update_dice_emojis = Some(Arc::new(fnc));
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

  /// The file generation process needs to be started by the application
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

  /// Information about a group call was updated
  pub fn on_update_group_call<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateGroupCall)) -> TGResult<()> + Send + Sync + 'static {
    self.update_group_call = Some(Arc::new(fnc));
    self
  }

  /// Information about a group call participant was changed. The updates are sent only after the group call is received through getGroupCall and only if the call is joined or being joined
  pub fn on_update_group_call_participant<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateGroupCallParticipant)) -> TGResult<()> + Send + Sync + 'static {
    self.update_group_call_participant = Some(Arc::new(fnc));
    self
  }

  /// Describes whether there are some pending notification updates. Can be used to prevent application from killing, while there are some pending notifications
  pub fn on_update_have_pending_notifications<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateHavePendingNotifications)) -> TGResult<()> + Send + Sync + 'static {
    self.update_have_pending_notifications = Some(Arc::new(fnc));
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

  /// The information about interactions with a message has changed
  pub fn on_update_message_interaction_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateMessageInteractionInfo)) -> TGResult<()> + Send + Sync + 'static {
    self.update_message_interaction_info = Some(Arc::new(fnc));
    self
  }

  /// The message pinned state was changed
  pub fn on_update_message_is_pinned<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateMessageIsPinned)) -> TGResult<()> + Send + Sync + 'static {
    self.update_message_is_pinned = Some(Arc::new(fnc));
    self
  }

  /// A message with a live location was viewed. When the update is received, the application is supposed to update the live location
  pub fn on_update_message_live_location_viewed<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateMessageLiveLocationViewed)) -> TGResult<()> + Send + Sync + 'static {
    self.update_message_live_location_viewed = Some(Arc::new(fnc));
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

  /// New call signaling data arrived
  pub fn on_update_new_call_signaling_data<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateNewCallSignalingData)) -> TGResult<()> + Send + Sync + 'static {
    self.update_new_call_signaling_data = Some(Arc::new(fnc));
    self
  }

  /// A new incoming callback query; for bots only
  pub fn on_update_new_callback_query<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateNewCallbackQuery)) -> TGResult<()> + Send + Sync + 'static {
    self.update_new_callback_query = Some(Arc::new(fnc));
    self
  }

  /// A new chat has been loaded/created. This update is guaranteed to come before the chat identifier is returned to the application. The chat field changes will be reported through separate updates
  pub fn on_update_new_chat<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateNewChat)) -> TGResult<()> + Send + Sync + 'static {
    self.update_new_chat = Some(Arc::new(fnc));
    self
  }

  /// A user sent a join request to a chat; for bots only
  pub fn on_update_new_chat_join_request<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateNewChatJoinRequest)) -> TGResult<()> + Send + Sync + 'static {
    self.update_new_chat_join_request = Some(Arc::new(fnc));
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

  /// A notification was changed
  pub fn on_update_notification<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateNotification)) -> TGResult<()> + Send + Sync + 'static {
    self.update_notification = Some(Arc::new(fnc));
    self
  }

  /// A list of active notifications in a notification group has changed
  pub fn on_update_notification_group<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateNotificationGroup)) -> TGResult<()> + Send + Sync + 'static {
    self.update_notification_group = Some(Arc::new(fnc));
    self
  }

  /// An option changed its value
  pub fn on_update_option<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateOption)) -> TGResult<()> + Send + Sync + 'static {
    self.update_option = Some(Arc::new(fnc));
    self
  }

  /// A poll was updated; for bots only
  pub fn on_update_poll<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdatePoll)) -> TGResult<()> + Send + Sync + 'static {
    self.update_poll = Some(Arc::new(fnc));
    self
  }

  /// A user changed the answer to a poll; for bots only
  pub fn on_update_poll_answer<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdatePollAnswer)) -> TGResult<()> + Send + Sync + 'static {
    self.update_poll_answer = Some(Arc::new(fnc));
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

  /// Some data of a secret chat has changed. This update is guaranteed to come before the secret chat identifier is returned to the application
  pub fn on_update_secret_chat<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateSecretChat)) -> TGResult<()> + Send + Sync + 'static {
    self.update_secret_chat = Some(Arc::new(fnc));
    self
  }

  /// The selected background has changed
  pub fn on_update_selected_background<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateSelectedBackground)) -> TGResult<()> + Send + Sync + 'static {
    self.update_selected_background = Some(Arc::new(fnc));
    self
  }

  /// Service notification from the server. Upon receiving this the application must show a popup with the content of the notification
  pub fn on_update_service_notification<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateServiceNotification)) -> TGResult<()> + Send + Sync + 'static {
    self.update_service_notification = Some(Arc::new(fnc));
    self
  }

  /// A sticker set has changed
  pub fn on_update_sticker_set<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateStickerSet)) -> TGResult<()> + Send + Sync + 'static {
    self.update_sticker_set = Some(Arc::new(fnc));
    self
  }

  /// The list of suggested to the user actions has changed
  pub fn on_update_suggested_actions<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateSuggestedActions)) -> TGResult<()> + Send + Sync + 'static {
    self.update_suggested_actions = Some(Arc::new(fnc));
    self
  }

  /// Some data of a supergroup or a channel has changed. This update is guaranteed to come before the supergroup identifier is returned to the application
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

  /// New terms of service must be accepted by the user. If the terms of service are declined, then the deleteAccount method must be called with the reason "Decline ToS update"
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

  /// Number of unread chats, i.e. with unread messages or marked as unread, has changed. This update is sent only if the message database is used
  pub fn on_update_unread_chat_count<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateUnreadChatCount)) -> TGResult<()> + Send + Sync + 'static {
    self.update_unread_chat_count = Some(Arc::new(fnc));
    self
  }

  /// Number of unread messages in a chat list has changed. This update is sent only if the message database is used
  pub fn on_update_unread_message_count<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateUnreadMessageCount)) -> TGResult<()> + Send + Sync + 'static {
    self.update_unread_message_count = Some(Arc::new(fnc));
    self
  }

  /// Some data of a user has changed. This update is guaranteed to come before the user identifier is returned to the application
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

  /// The list of users nearby has changed. The update is guaranteed to be sent only 60 seconds after a successful searchChatsNearby request
  pub fn on_update_users_nearby<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &UpdateUsersNearby)) -> TGResult<()> + Send + Sync + 'static {
    self.update_users_nearby = Some(Arc::new(fnc));
    self
  }



  /// Represents the current authorization state of the TDLib client
  pub fn on_authorization_state<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &AuthorizationState)) -> TGResult<()> + Send + Sync + 'static {
    self.authorization_state = Some(Arc::new(fnc));
    self
  }

  /// Represents result of checking whether the current session can be used to transfer a chat ownership to another user
  pub fn on_can_transfer_ownership_result<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &CanTransferOwnershipResult)) -> TGResult<()> + Send + Sync + 'static {
    self.can_transfer_ownership_result = Some(Arc::new(fnc));
    self
  }

  /// Contains a detailed statistics about a chat
  pub fn on_chat_statistics<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &ChatStatistics)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_statistics = Some(Arc::new(fnc));
    self
  }

  /// Represents result of checking whether a username can be set for a chat
  pub fn on_check_chat_username_result<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &CheckChatUsernameResult)) -> TGResult<()> + Send + Sync + 'static {
    self.check_chat_username_result = Some(Arc::new(fnc));
    self
  }

  /// Represents result of checking whether a name can be used for a new sticker set
  pub fn on_check_sticker_set_name_result<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &CheckStickerSetNameResult)) -> TGResult<()> + Send + Sync + 'static {
    self.check_sticker_set_name_result = Some(Arc::new(fnc));
    self
  }

  /// Describes an internal https://t.me or tg: link, which must be processed by the app in a special way
  pub fn on_internal_link_type<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &InternalLinkType)) -> TGResult<()> + Send + Sync + 'static {
    self.internal_link_type = Some(Arc::new(fnc));
    self
  }

  /// Represents a JSON value
  pub fn on_json_value<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &JsonValue)) -> TGResult<()> + Send + Sync + 'static {
    self.json_value = Some(Arc::new(fnc));
    self
  }

  /// Represents the value of a string in a language pack
  pub fn on_language_pack_string_value<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &LanguagePackStringValue)) -> TGResult<()> + Send + Sync + 'static {
    self.language_pack_string_value = Some(Arc::new(fnc));
    self
  }

  /// Describes a stream to which TDLib internal log is written
  pub fn on_log_stream<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &LogStream)) -> TGResult<()> + Send + Sync + 'static {
    self.log_stream = Some(Arc::new(fnc));
    self
  }

  /// Contains information about an inline button of type inlineKeyboardButtonTypeLoginUrl
  pub fn on_login_url_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &LoginUrlInfo)) -> TGResult<()> + Send + Sync + 'static {
    self.login_url_info = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a file with messages exported from another app
  pub fn on_message_file_type<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &MessageFileType)) -> TGResult<()> + Send + Sync + 'static {
    self.message_file_type = Some(Arc::new(fnc));
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

  /// Represents result of 2-step verification password reset
  pub fn on_reset_password_result<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &ResetPasswordResult)) -> TGResult<()> + Send + Sync + 'static {
    self.reset_password_result = Some(Arc::new(fnc));
    self
  }

  /// Describes a statistical graph
  pub fn on_statistical_graph<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &StatisticalGraph)) -> TGResult<()> + Send + Sync + 'static {
    self.statistical_graph = Some(Arc::new(fnc));
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

  /// Describes an animated representation of an emoji
  pub fn on_animated_emoji<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &AnimatedEmoji)) -> TGResult<()> + Send + Sync + 'static {
    self.animated_emoji = Some(Arc::new(fnc));
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

  /// Contains auto-download settings presets for the current user
  pub fn on_auto_download_settings_presets<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &AutoDownloadSettingsPresets)) -> TGResult<()> + Send + Sync + 'static {
    self.auto_download_settings_presets = Some(Arc::new(fnc));
    self
  }

  /// Describes a chat background
  pub fn on_background<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Background)) -> TGResult<()> + Send + Sync + 'static {
    self.background = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of backgrounds
  pub fn on_backgrounds<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Backgrounds)) -> TGResult<()> + Send + Sync + 'static {
    self.backgrounds = Some(Arc::new(fnc));
    self
  }

  /// Information about a bank card
  pub fn on_bank_card_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &BankCardInfo)) -> TGResult<()> + Send + Sync + 'static {
    self.bank_card_info = Some(Arc::new(fnc));
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

  /// Contains a list of bot commands
  pub fn on_bot_commands<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &BotCommands)) -> TGResult<()> + Send + Sync + 'static {
    self.bot_commands = Some(Arc::new(fnc));
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

  /// Represents a list of chat administrators
  pub fn on_chat_administrators<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &ChatAdministrators)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_administrators = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of chat events
  pub fn on_chat_events<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &ChatEvents)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_events = Some(Arc::new(fnc));
    self
  }

  /// Represents a filter of user chats
  pub fn on_chat_filter<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &ChatFilter)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_filter = Some(Arc::new(fnc));
    self
  }

  /// Contains basic information about a chat filter
  pub fn on_chat_filter_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &ChatFilterInfo)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_filter_info = Some(Arc::new(fnc));
    self
  }

  /// Contains a chat invite link
  pub fn on_chat_invite_link<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &ChatInviteLink)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_invite_link = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of chat invite link counts
  pub fn on_chat_invite_link_counts<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &ChatInviteLinkCounts)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_invite_link_counts = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a chat invite link
  pub fn on_chat_invite_link_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &ChatInviteLinkInfo)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_invite_link_info = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of chat members joined a chat by an invite link
  pub fn on_chat_invite_link_members<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &ChatInviteLinkMembers)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_invite_link_members = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of chat invite links
  pub fn on_chat_invite_links<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &ChatInviteLinks)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_invite_links = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of chat join requests
  pub fn on_chat_join_requests<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &ChatJoinRequests)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_join_requests = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of chat lists
  pub fn on_chat_lists<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &ChatLists)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_lists = Some(Arc::new(fnc));
    self
  }

  /// Describes a user or a chat as a member of another chat
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

  /// Contains a list of chat or user profile photos
  pub fn on_chat_photos<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &ChatPhotos)) -> TGResult<()> + Send + Sync + 'static {
    self.chat_photos = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of chats
  pub fn on_chats<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Chats)) -> TGResult<()> + Send + Sync + 'static {
    self.chats = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of chats located nearby
  pub fn on_chats_nearby<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &ChatsNearby)) -> TGResult<()> + Send + Sync + 'static {
    self.chats_nearby = Some(Arc::new(fnc));
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

  /// Contains information about countries
  pub fn on_countries<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Countries)) -> TGResult<()> + Send + Sync + 'static {
    self.countries = Some(Arc::new(fnc));
    self
  }

  /// Contains the result of a custom request
  pub fn on_custom_request_result<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &CustomRequestResult)) -> TGResult<()> + Send + Sync + 'static {
    self.custom_request_result = Some(Arc::new(fnc));
    self
  }

  /// Contains database statistics
  pub fn on_database_statistics<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &DatabaseStatistics)) -> TGResult<()> + Send + Sync + 'static {
    self.database_statistics = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a tg: deep link
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

  /// Represents a list of emoji
  pub fn on_emojis<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Emojis)) -> TGResult<()> + Send + Sync + 'static {
    self.emojis = Some(Arc::new(fnc));
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

  /// Contains a part of a file
  pub fn on_file_part<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &FilePart)) -> TGResult<()> + Send + Sync + 'static {
    self.file_part = Some(Arc::new(fnc));
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

  /// Describes a group call
  pub fn on_group_call<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &GroupCall)) -> TGResult<()> + Send + Sync + 'static {
    self.group_call = Some(Arc::new(fnc));
    self
  }

  /// Contains the group call identifier
  pub fn on_group_call_id<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &GroupCallId)) -> TGResult<()> + Send + Sync + 'static {
    self.group_call_id = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of hashtags
  pub fn on_hashtags<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Hashtags)) -> TGResult<()> + Send + Sync + 'static {
    self.hashtags = Some(Arc::new(fnc));
    self
  }

  /// Contains an HTTP URL
  pub fn on_http_url<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &HttpUrl)) -> TGResult<()> + Send + Sync + 'static {
    self.http_url = Some(Arc::new(fnc));
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

  /// Contains information about a language pack
  pub fn on_language_pack_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &LanguagePackInfo)) -> TGResult<()> + Send + Sync + 'static {
    self.language_pack_info = Some(Arc::new(fnc));
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

  /// Contains a list of available TDLib internal log tags
  pub fn on_log_tags<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &LogTags)) -> TGResult<()> + Send + Sync + 'static {
    self.log_tags = Some(Arc::new(fnc));
    self
  }

  /// Contains a TDLib internal log verbosity level
  pub fn on_log_verbosity_level<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &LogVerbosityLevel)) -> TGResult<()> + Send + Sync + 'static {
    self.log_verbosity_level = Some(Arc::new(fnc));
    self
  }

  /// Describes a message
  pub fn on_message<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Message)) -> TGResult<()> + Send + Sync + 'static {
    self.message = Some(Arc::new(fnc));
    self
  }

  /// Contains information about found messages, splitted by days according to the option "utc_time_offset"
  pub fn on_message_calendar<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &MessageCalendar)) -> TGResult<()> + Send + Sync + 'static {
    self.message_calendar = Some(Arc::new(fnc));
    self
  }

  /// Contains an HTTPS link to a message in a supergroup or channel
  pub fn on_message_link<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &MessageLink)) -> TGResult<()> + Send + Sync + 'static {
    self.message_link = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a link to a message in a chat
  pub fn on_message_link_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &MessageLinkInfo)) -> TGResult<()> + Send + Sync + 'static {
    self.message_link_info = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of message positions
  pub fn on_message_positions<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &MessagePositions)) -> TGResult<()> + Send + Sync + 'static {
    self.message_positions = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of message senders
  pub fn on_message_senders<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &MessageSenders)) -> TGResult<()> + Send + Sync + 'static {
    self.message_senders = Some(Arc::new(fnc));
    self
  }

  /// A detailed statistics about a message
  pub fn on_message_statistics<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &MessageStatistics)) -> TGResult<()> + Send + Sync + 'static {
    self.message_statistics = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a message thread
  pub fn on_message_thread_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &MessageThreadInfo)) -> TGResult<()> + Send + Sync + 'static {
    self.message_thread_info = Some(Arc::new(fnc));
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

  /// Contains information about a Telegram Passport elements and corresponding errors
  pub fn on_passport_elements_with_errors<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &PassportElementsWithErrors)) -> TGResult<()> + Send + Sync + 'static {
    self.passport_elements_with_errors = Some(Arc::new(fnc));
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

  /// Contains information about a phone number
  pub fn on_phone_number_info<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &PhoneNumberInfo)) -> TGResult<()> + Send + Sync + 'static {
    self.phone_number_info = Some(Arc::new(fnc));
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

  /// Contains a globally unique push receiver identifier, which can be used to identify which account has received a push notification
  pub fn on_push_receiver_id<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &PushReceiverId)) -> TGResult<()> + Send + Sync + 'static {
    self.push_receiver_id = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of recommended chat filters
  pub fn on_recommended_chat_filters<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &RecommendedChatFilters)) -> TGResult<()> + Send + Sync + 'static {
    self.recommended_chat_filters = Some(Arc::new(fnc));
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

  /// Contains information about one session in a Telegram application used by the current user. Sessions must be shown to the user in the returned order
  pub fn on_session<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Session)) -> TGResult<()> + Send + Sync + 'static {
    self.session = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of sessions
  pub fn on_sessions<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Sessions)) -> TGResult<()> + Send + Sync + 'static {
    self.sessions = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of sponsored messages
  pub fn on_sponsored_messages<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &SponsoredMessages)) -> TGResult<()> + Send + Sync + 'static {
    self.sponsored_messages = Some(Arc::new(fnc));
    self
  }

  /// Describes a sticker
  pub fn on_sticker<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Sticker)) -> TGResult<()> + Send + Sync + 'static {
    self.sticker = Some(Arc::new(fnc));
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

  /// Contains a list of updates
  pub fn on_updates<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &Updates)) -> TGResult<()> + Send + Sync + 'static {
    self.updates = Some(Arc::new(fnc));
    self
  }

  /// Represents a user
  pub fn on_user<F>(&mut self, fnc: F) -> &mut Self
    where F: Fn((&Api, &User)) -> TGResult<()> + Send + Sync + 'static {
    self.user = Some(Arc::new(fnc));
    self
  }

  /// Contains full information about a user
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
#[derive(Clone)]
pub struct EventLout {
  listener: EventListener,
  supports: Vec<&'static str>
}

impl EventLout {
  fn new(listener: EventListener) -> Self {
    let supports = vec![


      "testUseUpdate",
      "updateActiveNotifications",
      "updateAnimatedEmojiMessageClicked",
      "updateAnimationSearchParameters",
      "updateAuthorizationState",
      "updateBasicGroup",
      "updateBasicGroupFullInfo",
      "updateCall",
      "updateChatActionBar",
      "updateChatDefaultDisableNotification",
      "updateChatDraftMessage",
      "updateChatFilters",
      "updateChatHasScheduledMessages",
      "updateChatIsBlocked",
      "updateChatIsMarkedAsUnread",
      "updateChatLastMessage",
      "updateChatMember",
      "updateChatMessageTtlSetting",
      "updateChatNotificationSettings",
      "updateChatOnlineMemberCount",
      "updateChatPendingJoinRequests",
      "updateChatPermissions",
      "updateChatPhoto",
      "updateChatPosition",
      "updateChatReadInbox",
      "updateChatReadOutbox",
      "updateChatReplyMarkup",
      "updateChatTheme",
      "updateChatThemes",
      "updateChatTitle",
      "updateChatUnreadMentionCount",
      "updateChatVideoChat",
      "updateConnectionState",
      "updateDeleteMessages",
      "updateDiceEmojis",
      "updateFavoriteStickers",
      "updateFile",
      "updateFileGenerationStart",
      "updateFileGenerationStop",
      "updateGroupCall",
      "updateGroupCallParticipant",
      "updateHavePendingNotifications",
      "updateInstalledStickerSets",
      "updateLanguagePackStrings",
      "updateMessageContent",
      "updateMessageContentOpened",
      "updateMessageEdited",
      "updateMessageInteractionInfo",
      "updateMessageIsPinned",
      "updateMessageLiveLocationViewed",
      "updateMessageMentionRead",
      "updateMessageSendAcknowledged",
      "updateMessageSendFailed",
      "updateMessageSendSucceeded",
      "updateNewCallSignalingData",
      "updateNewCallbackQuery",
      "updateNewChat",
      "updateNewChatJoinRequest",
      "updateNewChosenInlineResult",
      "updateNewCustomEvent",
      "updateNewCustomQuery",
      "updateNewInlineCallbackQuery",
      "updateNewInlineQuery",
      "updateNewMessage",
      "updateNewPreCheckoutQuery",
      "updateNewShippingQuery",
      "updateNotification",
      "updateNotificationGroup",
      "updateOption",
      "updatePoll",
      "updatePollAnswer",
      "updateRecentStickers",
      "updateSavedAnimations",
      "updateScopeNotificationSettings",
      "updateSecretChat",
      "updateSelectedBackground",
      "updateServiceNotification",
      "updateStickerSet",
      "updateSuggestedActions",
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
      "updateUsersNearby",


      "AuthorizationState",
      "CanTransferOwnershipResult",
      "ChatStatistics",
      "CheckChatUsernameResult",
      "CheckStickerSetNameResult",
      "InternalLinkType",
      "JsonValue",
      "LanguagePackStringValue",
      "LogStream",
      "LoginUrlInfo",
      "MessageFileType",
      "OptionValue",
      "PassportElement",
      "ResetPasswordResult",
      "StatisticalGraph",
      "Update",
      "accountTtl",
      "animatedEmoji",
      "animations",
      "authenticationCodeInfo",
      "autoDownloadSettingsPresets",
      "background",
      "backgrounds",
      "bankCardInfo",
      "basicGroup",
      "basicGroupFullInfo",
      "botCommands",
      "callId",
      "callbackQueryAnswer",
      "chat",
      "chatAdministrators",
      "chatEvents",
      "chatFilter",
      "chatFilterInfo",
      "chatInviteLink",
      "chatInviteLinkCounts",
      "chatInviteLinkInfo",
      "chatInviteLinkMembers",
      "chatInviteLinks",
      "chatJoinRequests",
      "chatLists",
      "chatMember",
      "chatMembers",
      "chatPhotos",
      "chats",
      "chatsNearby",
      "connectedWebsites",
      "count",
      "countries",
      "customRequestResult",
      "databaseStatistics",
      "deepLinkInfo",
      "emailAddressAuthenticationCodeInfo",
      "emojis",
      "error",
      "file",
      "filePart",
      "formattedText",
      "foundMessages",
      "gameHighScores",
      "groupCall",
      "groupCallId",
      "hashtags",
      "httpUrl",
      "importedContacts",
      "inlineQueryResults",
      "languagePackInfo",
      "languagePackStrings",
      "localizationTargetInfo",
      "logTags",
      "logVerbosityLevel",
      "message",
      "messageCalendar",
      "messageLink",
      "messageLinkInfo",
      "messagePositions",
      "messageSenders",
      "messageStatistics",
      "messageThreadInfo",
      "messages",
      "networkStatistics",
      "ok",
      "orderInfo",
      "passportAuthorizationForm",
      "passportElements",
      "passportElementsWithErrors",
      "passwordState",
      "paymentForm",
      "paymentReceipt",
      "paymentResult",
      "phoneNumberInfo",
      "proxies",
      "proxy",
      "pushReceiverId",
      "recommendedChatFilters",
      "recoveryEmailAddress",
      "scopeNotificationSettings",
      "seconds",
      "secretChat",
      "session",
      "sessions",
      "sponsoredMessages",
      "sticker",
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
      "updates",
      "user",
      "userFullInfo",
      "userPrivacySettingRules",
      "users",
      "validatedOrderInfo",
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

  pub fn handle_type(&self, api: &Api, td_type: &TdType) -> TGResult<bool>  {
    match td_type {


    TdType::TestUseUpdate(value) => match &self.listener.test_use_update {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateActiveNotifications(value) => match &self.listener.update_active_notifications {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateAnimatedEmojiMessageClicked(value) => match &self.listener.update_animated_emoji_message_clicked {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateAnimationSearchParameters(value) => match &self.listener.update_animation_search_parameters {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateAuthorizationState(value) => match &self.listener.update_authorization_state {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateBasicGroup(value) => match &self.listener.update_basic_group {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateBasicGroupFullInfo(value) => match &self.listener.update_basic_group_full_info {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateCall(value) => match &self.listener.update_call {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateChatActionBar(value) => match &self.listener.update_chat_action_bar {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateChatDefaultDisableNotification(value) => match &self.listener.update_chat_default_disable_notification {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateChatDraftMessage(value) => match &self.listener.update_chat_draft_message {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateChatFilters(value) => match &self.listener.update_chat_filters {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateChatHasScheduledMessages(value) => match &self.listener.update_chat_has_scheduled_messages {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateChatIsBlocked(value) => match &self.listener.update_chat_is_blocked {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateChatIsMarkedAsUnread(value) => match &self.listener.update_chat_is_marked_as_unread {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateChatLastMessage(value) => match &self.listener.update_chat_last_message {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateChatMember(value) => match &self.listener.update_chat_member {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateChatMessageTtlSetting(value) => match &self.listener.update_chat_message_ttl_setting {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateChatNotificationSettings(value) => match &self.listener.update_chat_notification_settings {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateChatOnlineMemberCount(value) => match &self.listener.update_chat_online_member_count {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateChatPendingJoinRequests(value) => match &self.listener.update_chat_pending_join_requests {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateChatPermissions(value) => match &self.listener.update_chat_permissions {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateChatPhoto(value) => match &self.listener.update_chat_photo {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateChatPosition(value) => match &self.listener.update_chat_position {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateChatReadInbox(value) => match &self.listener.update_chat_read_inbox {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateChatReadOutbox(value) => match &self.listener.update_chat_read_outbox {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateChatReplyMarkup(value) => match &self.listener.update_chat_reply_markup {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateChatTheme(value) => match &self.listener.update_chat_theme {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateChatThemes(value) => match &self.listener.update_chat_themes {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateChatTitle(value) => match &self.listener.update_chat_title {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateChatUnreadMentionCount(value) => match &self.listener.update_chat_unread_mention_count {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateChatVideoChat(value) => match &self.listener.update_chat_video_chat {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateConnectionState(value) => match &self.listener.update_connection_state {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateDeleteMessages(value) => match &self.listener.update_delete_messages {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateDiceEmojis(value) => match &self.listener.update_dice_emojis {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateFavoriteStickers(value) => match &self.listener.update_favorite_stickers {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateFile(value) => match &self.listener.update_file {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateFileGenerationStart(value) => match &self.listener.update_file_generation_start {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateFileGenerationStop(value) => match &self.listener.update_file_generation_stop {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateGroupCall(value) => match &self.listener.update_group_call {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateGroupCallParticipant(value) => match &self.listener.update_group_call_participant {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateHavePendingNotifications(value) => match &self.listener.update_have_pending_notifications {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateInstalledStickerSets(value) => match &self.listener.update_installed_sticker_sets {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateLanguagePackStrings(value) => match &self.listener.update_language_pack_strings {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateMessageContent(value) => match &self.listener.update_message_content {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateMessageContentOpened(value) => match &self.listener.update_message_content_opened {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateMessageEdited(value) => match &self.listener.update_message_edited {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateMessageInteractionInfo(value) => match &self.listener.update_message_interaction_info {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateMessageIsPinned(value) => match &self.listener.update_message_is_pinned {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateMessageLiveLocationViewed(value) => match &self.listener.update_message_live_location_viewed {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateMessageMentionRead(value) => match &self.listener.update_message_mention_read {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateMessageSendAcknowledged(value) => match &self.listener.update_message_send_acknowledged {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateMessageSendFailed(value) => match &self.listener.update_message_send_failed {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateMessageSendSucceeded(value) => match &self.listener.update_message_send_succeeded {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateNewCallSignalingData(value) => match &self.listener.update_new_call_signaling_data {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateNewCallbackQuery(value) => match &self.listener.update_new_callback_query {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateNewChat(value) => match &self.listener.update_new_chat {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateNewChatJoinRequest(value) => match &self.listener.update_new_chat_join_request {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateNewChosenInlineResult(value) => match &self.listener.update_new_chosen_inline_result {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateNewCustomEvent(value) => match &self.listener.update_new_custom_event {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateNewCustomQuery(value) => match &self.listener.update_new_custom_query {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateNewInlineCallbackQuery(value) => match &self.listener.update_new_inline_callback_query {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateNewInlineQuery(value) => match &self.listener.update_new_inline_query {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateNewMessage(value) => match &self.listener.update_new_message {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateNewPreCheckoutQuery(value) => match &self.listener.update_new_pre_checkout_query {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateNewShippingQuery(value) => match &self.listener.update_new_shipping_query {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateNotification(value) => match &self.listener.update_notification {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateNotificationGroup(value) => match &self.listener.update_notification_group {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateOption(value) => match &self.listener.update_option {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdatePoll(value) => match &self.listener.update_poll {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdatePollAnswer(value) => match &self.listener.update_poll_answer {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateRecentStickers(value) => match &self.listener.update_recent_stickers {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateSavedAnimations(value) => match &self.listener.update_saved_animations {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateScopeNotificationSettings(value) => match &self.listener.update_scope_notification_settings {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateSecretChat(value) => match &self.listener.update_secret_chat {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateSelectedBackground(value) => match &self.listener.update_selected_background {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateServiceNotification(value) => match &self.listener.update_service_notification {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateStickerSet(value) => match &self.listener.update_sticker_set {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateSuggestedActions(value) => match &self.listener.update_suggested_actions {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateSupergroup(value) => match &self.listener.update_supergroup {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateSupergroupFullInfo(value) => match &self.listener.update_supergroup_full_info {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateTermsOfService(value) => match &self.listener.update_terms_of_service {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateTrendingStickerSets(value) => match &self.listener.update_trending_sticker_sets {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateUnreadChatCount(value) => match &self.listener.update_unread_chat_count {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateUnreadMessageCount(value) => match &self.listener.update_unread_message_count {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateUser(value) => match &self.listener.update_user {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateUserChatAction(value) => match &self.listener.update_user_chat_action {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateUserFullInfo(value) => match &self.listener.update_user_full_info {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateUserPrivacySettingRules(value) => match &self.listener.update_user_privacy_setting_rules {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateUserStatus(value) => match &self.listener.update_user_status {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UpdateUsersNearby(value) => match &self.listener.update_users_nearby {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },


    TdType::AuthorizationState(value) => match &self.listener.authorization_state {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::CanTransferOwnershipResult(value) => match &self.listener.can_transfer_ownership_result {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::ChatStatistics(value) => match &self.listener.chat_statistics {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::CheckChatUsernameResult(value) => match &self.listener.check_chat_username_result {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::CheckStickerSetNameResult(value) => match &self.listener.check_sticker_set_name_result {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::InternalLinkType(value) => match &self.listener.internal_link_type {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::JsonValue(value) => match &self.listener.json_value {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::LanguagePackStringValue(value) => match &self.listener.language_pack_string_value {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::LogStream(value) => match &self.listener.log_stream {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::LoginUrlInfo(value) => match &self.listener.login_url_info {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::MessageFileType(value) => match &self.listener.message_file_type {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::OptionValue(value) => match &self.listener.option_value {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::PassportElement(value) => match &self.listener.passport_element {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::ResetPasswordResult(value) => match &self.listener.reset_password_result {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::StatisticalGraph(value) => match &self.listener.statistical_graph {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::Update(value) => match &self.listener.update {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::AccountTtl(value) => match &self.listener.account_ttl {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::AnimatedEmoji(value) => match &self.listener.animated_emoji {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::Animations(value) => match &self.listener.animations {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::AuthenticationCodeInfo(value) => match &self.listener.authentication_code_info {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::AutoDownloadSettingsPresets(value) => match &self.listener.auto_download_settings_presets {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::Background(value) => match &self.listener.background {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::Backgrounds(value) => match &self.listener.backgrounds {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::BankCardInfo(value) => match &self.listener.bank_card_info {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::BasicGroup(value) => match &self.listener.basic_group {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::BasicGroupFullInfo(value) => match &self.listener.basic_group_full_info {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::BotCommands(value) => match &self.listener.bot_commands {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::CallId(value) => match &self.listener.call_id {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::CallbackQueryAnswer(value) => match &self.listener.callback_query_answer {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::Chat(value) => match &self.listener.chat {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::ChatAdministrators(value) => match &self.listener.chat_administrators {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::ChatEvents(value) => match &self.listener.chat_events {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::ChatFilter(value) => match &self.listener.chat_filter {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::ChatFilterInfo(value) => match &self.listener.chat_filter_info {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::ChatInviteLink(value) => match &self.listener.chat_invite_link {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::ChatInviteLinkCounts(value) => match &self.listener.chat_invite_link_counts {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::ChatInviteLinkInfo(value) => match &self.listener.chat_invite_link_info {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::ChatInviteLinkMembers(value) => match &self.listener.chat_invite_link_members {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::ChatInviteLinks(value) => match &self.listener.chat_invite_links {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::ChatJoinRequests(value) => match &self.listener.chat_join_requests {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::ChatLists(value) => match &self.listener.chat_lists {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::ChatMember(value) => match &self.listener.chat_member {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::ChatMembers(value) => match &self.listener.chat_members {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::ChatPhotos(value) => match &self.listener.chat_photos {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::Chats(value) => match &self.listener.chats {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::ChatsNearby(value) => match &self.listener.chats_nearby {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::ConnectedWebsites(value) => match &self.listener.connected_websites {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::Count(value) => match &self.listener.count {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::Countries(value) => match &self.listener.countries {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::CustomRequestResult(value) => match &self.listener.custom_request_result {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::DatabaseStatistics(value) => match &self.listener.database_statistics {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::DeepLinkInfo(value) => match &self.listener.deep_link_info {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::EmailAddressAuthenticationCodeInfo(value) => match &self.listener.email_address_authentication_code_info {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::Emojis(value) => match &self.listener.emojis {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::Error(value) => match &self.listener.error {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::File(value) => match &self.listener.file {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::FilePart(value) => match &self.listener.file_part {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::FormattedText(value) => match &self.listener.formatted_text {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::FoundMessages(value) => match &self.listener.found_messages {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::GameHighScores(value) => match &self.listener.game_high_scores {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::GroupCall(value) => match &self.listener.group_call {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::GroupCallId(value) => match &self.listener.group_call_id {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::Hashtags(value) => match &self.listener.hashtags {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::HttpUrl(value) => match &self.listener.http_url {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::ImportedContacts(value) => match &self.listener.imported_contacts {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::InlineQueryResults(value) => match &self.listener.inline_query_results {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::LanguagePackInfo(value) => match &self.listener.language_pack_info {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::LanguagePackStrings(value) => match &self.listener.language_pack_strings {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::LocalizationTargetInfo(value) => match &self.listener.localization_target_info {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::LogTags(value) => match &self.listener.log_tags {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::LogVerbosityLevel(value) => match &self.listener.log_verbosity_level {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::Message(value) => match &self.listener.message {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::MessageCalendar(value) => match &self.listener.message_calendar {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::MessageLink(value) => match &self.listener.message_link {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::MessageLinkInfo(value) => match &self.listener.message_link_info {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::MessagePositions(value) => match &self.listener.message_positions {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::MessageSenders(value) => match &self.listener.message_senders {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::MessageStatistics(value) => match &self.listener.message_statistics {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::MessageThreadInfo(value) => match &self.listener.message_thread_info {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::Messages(value) => match &self.listener.messages {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::NetworkStatistics(value) => match &self.listener.network_statistics {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::Ok(value) => match &self.listener.ok {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::OrderInfo(value) => match &self.listener.order_info {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::PassportAuthorizationForm(value) => match &self.listener.passport_authorization_form {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::PassportElements(value) => match &self.listener.passport_elements {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::PassportElementsWithErrors(value) => match &self.listener.passport_elements_with_errors {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::PasswordState(value) => match &self.listener.password_state {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::PaymentForm(value) => match &self.listener.payment_form {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::PaymentReceipt(value) => match &self.listener.payment_receipt {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::PaymentResult(value) => match &self.listener.payment_result {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::PhoneNumberInfo(value) => match &self.listener.phone_number_info {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::Proxies(value) => match &self.listener.proxies {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::Proxy(value) => match &self.listener.proxy {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::PushReceiverId(value) => match &self.listener.push_receiver_id {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::RecommendedChatFilters(value) => match &self.listener.recommended_chat_filters {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::RecoveryEmailAddress(value) => match &self.listener.recovery_email_address {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::ScopeNotificationSettings(value) => match &self.listener.scope_notification_settings {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::Seconds(value) => match &self.listener.seconds {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::SecretChat(value) => match &self.listener.secret_chat {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::Session(value) => match &self.listener.session {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::Sessions(value) => match &self.listener.sessions {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::SponsoredMessages(value) => match &self.listener.sponsored_messages {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::Sticker(value) => match &self.listener.sticker {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::StickerSet(value) => match &self.listener.sticker_set {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::StickerSets(value) => match &self.listener.sticker_sets {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::Stickers(value) => match &self.listener.stickers {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::StorageStatistics(value) => match &self.listener.storage_statistics {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::StorageStatisticsFast(value) => match &self.listener.storage_statistics_fast {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::Supergroup(value) => match &self.listener.supergroup {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::SupergroupFullInfo(value) => match &self.listener.supergroup_full_info {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::TMeUrls(value) => match &self.listener.t_me_urls {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::TemporaryPasswordState(value) => match &self.listener.temporary_password_state {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::TestBytes(value) => match &self.listener.test_bytes {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::TestInt(value) => match &self.listener.test_int {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::TestString(value) => match &self.listener.test_string {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::TestVectorInt(value) => match &self.listener.test_vector_int {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::TestVectorIntObject(value) => match &self.listener.test_vector_int_object {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::TestVectorString(value) => match &self.listener.test_vector_string {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::TestVectorStringObject(value) => match &self.listener.test_vector_string_object {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::Text(value) => match &self.listener.text {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::TextEntities(value) => match &self.listener.text_entities {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::Updates(value) => match &self.listener.updates {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::User(value) => match &self.listener.user {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UserFullInfo(value) => match &self.listener.user_full_info {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::UserPrivacySettingRules(value) => match &self.listener.user_privacy_setting_rules {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::Users(value) => match &self.listener.users {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::ValidatedOrderInfo(value) => match &self.listener.validated_order_info {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::WebPage(value) => match &self.listener.web_page {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },

    TdType::WebPageInstantView(value) => match &self.listener.web_page_instant_view {
      None => Ok(false),
      Some(f) => f((api, value)).map(|_|true),
    },


  }
  }

  /// when telegram client throw exception
  pub fn exception(&self) -> &Option<Arc<dyn Fn((&Api, &TGError)) + Send + Sync + 'static>> {
    &self.listener.exception
  }

  /// when receive data from tdlib
  pub fn receive(&self) -> &Option<Arc<dyn Fn((&Api, &String)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.receive
  }





  /// Does nothing and ensures that the Update object is used; for testing only. This is an offline method. Can be called before authorization
  pub fn test_use_update(&self) -> &Option<Arc<dyn Fn((&Api, &TestUseUpdate)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.test_use_update
  }

  /// Contains active notifications that was shown on previous application launches. This update is sent only if the message database is used. In that case it comes once before any updateNotification and updateNotificationGroup update
  pub fn update_active_notifications(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateActiveNotifications)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_active_notifications
  }

  /// Some animated emoji message was clicked and a big animated sticker must be played if the message is visible on the screen. chatActionWatchingAnimations with the text of the message needs to be sent if the sticker is played
  pub fn update_animated_emoji_message_clicked(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateAnimatedEmojiMessageClicked)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_animated_emoji_message_clicked
  }

  /// The parameters of animation search through GetOption("animation_search_bot_username") bot has changed
  pub fn update_animation_search_parameters(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateAnimationSearchParameters)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_animation_search_parameters
  }

  /// The user authorization state has changed
  pub fn update_authorization_state(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateAuthorizationState)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_authorization_state
  }

  /// Some data of a basic group has changed. This update is guaranteed to come before the basic group identifier is returned to the application
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

  /// The chat action bar was changed
  pub fn update_chat_action_bar(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatActionBar)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_action_bar
  }

  /// The value of the default disable_notification parameter, used when a message is sent to the chat, was changed
  pub fn update_chat_default_disable_notification(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatDefaultDisableNotification)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_default_disable_notification
  }

  /// A chat draft has changed. Be aware that the update may come in the currently opened chat but with old content of the draft. If the user has changed the content of the draft, this update mustn't be applied
  pub fn update_chat_draft_message(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatDraftMessage)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_draft_message
  }

  /// The list of chat filters or a chat filter has changed
  pub fn update_chat_filters(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatFilters)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_filters
  }

  /// A chat's has_scheduled_messages field has changed
  pub fn update_chat_has_scheduled_messages(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatHasScheduledMessages)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_has_scheduled_messages
  }

  /// A chat was blocked or unblocked
  pub fn update_chat_is_blocked(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatIsBlocked)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_is_blocked
  }

  /// A chat was marked as unread or was read
  pub fn update_chat_is_marked_as_unread(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatIsMarkedAsUnread)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_is_marked_as_unread
  }

  /// The last message of a chat was changed. If last_message is null, then the last message in the chat became unknown. Some new unknown messages might be added to the chat in this case
  pub fn update_chat_last_message(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatLastMessage)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_last_message
  }

  /// User rights changed in a chat; for bots only
  pub fn update_chat_member(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatMember)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_member
  }

  /// The message Time To Live setting for a chat was changed
  pub fn update_chat_message_ttl_setting(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatMessageTtlSetting)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_message_ttl_setting
  }

  /// Notification settings for a chat were changed
  pub fn update_chat_notification_settings(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatNotificationSettings)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_notification_settings
  }

  /// The number of online group members has changed. This update with non-zero count is sent only for currently opened chats. There is no guarantee that it will be sent just after the count has changed
  pub fn update_chat_online_member_count(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatOnlineMemberCount)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_online_member_count
  }

  /// The chat pending join requests were changed
  pub fn update_chat_pending_join_requests(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatPendingJoinRequests)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_pending_join_requests
  }

  /// Chat permissions was changed
  pub fn update_chat_permissions(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatPermissions)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_permissions
  }

  /// A chat photo was changed
  pub fn update_chat_photo(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatPhoto)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_photo
  }

  /// The position of a chat in a chat list has changed. Instead of this update updateChatLastMessage or updateChatDraftMessage might be sent
  pub fn update_chat_position(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatPosition)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_position
  }

  /// Incoming messages were read or the number of unread messages has been changed
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

  /// The chat theme was changed
  pub fn update_chat_theme(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatTheme)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_theme
  }

  /// The list of available chat themes has changed
  pub fn update_chat_themes(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatThemes)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_themes
  }

  /// The title of a chat was changed
  pub fn update_chat_title(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatTitle)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_title
  }

  /// The chat unread_mention_count has changed
  pub fn update_chat_unread_mention_count(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatUnreadMentionCount)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_unread_mention_count
  }

  /// A chat video chat state has changed
  pub fn update_chat_video_chat(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateChatVideoChat)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_chat_video_chat
  }

  /// The connection state has changed. This update must be used only to show a human-readable description of the connection state
  pub fn update_connection_state(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateConnectionState)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_connection_state
  }

  /// Some messages were deleted
  pub fn update_delete_messages(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateDeleteMessages)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_delete_messages
  }

  /// The list of supported dice emojis has changed
  pub fn update_dice_emojis(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateDiceEmojis)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_dice_emojis
  }

  /// The list of favorite stickers was updated
  pub fn update_favorite_stickers(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateFavoriteStickers)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_favorite_stickers
  }

  /// Information about a file was updated
  pub fn update_file(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateFile)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_file
  }

  /// The file generation process needs to be started by the application
  pub fn update_file_generation_start(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateFileGenerationStart)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_file_generation_start
  }

  /// File generation is no longer needed
  pub fn update_file_generation_stop(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateFileGenerationStop)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_file_generation_stop
  }

  /// Information about a group call was updated
  pub fn update_group_call(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateGroupCall)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_group_call
  }

  /// Information about a group call participant was changed. The updates are sent only after the group call is received through getGroupCall and only if the call is joined or being joined
  pub fn update_group_call_participant(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateGroupCallParticipant)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_group_call_participant
  }

  /// Describes whether there are some pending notification updates. Can be used to prevent application from killing, while there are some pending notifications
  pub fn update_have_pending_notifications(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateHavePendingNotifications)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_have_pending_notifications
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

  /// The information about interactions with a message has changed
  pub fn update_message_interaction_info(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateMessageInteractionInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_message_interaction_info
  }

  /// The message pinned state was changed
  pub fn update_message_is_pinned(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateMessageIsPinned)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_message_is_pinned
  }

  /// A message with a live location was viewed. When the update is received, the application is supposed to update the live location
  pub fn update_message_live_location_viewed(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateMessageLiveLocationViewed)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_message_live_location_viewed
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

  /// New call signaling data arrived
  pub fn update_new_call_signaling_data(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateNewCallSignalingData)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_new_call_signaling_data
  }

  /// A new incoming callback query; for bots only
  pub fn update_new_callback_query(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateNewCallbackQuery)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_new_callback_query
  }

  /// A new chat has been loaded/created. This update is guaranteed to come before the chat identifier is returned to the application. The chat field changes will be reported through separate updates
  pub fn update_new_chat(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateNewChat)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_new_chat
  }

  /// A user sent a join request to a chat; for bots only
  pub fn update_new_chat_join_request(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateNewChatJoinRequest)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_new_chat_join_request
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

  /// A notification was changed
  pub fn update_notification(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateNotification)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_notification
  }

  /// A list of active notifications in a notification group has changed
  pub fn update_notification_group(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateNotificationGroup)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_notification_group
  }

  /// An option changed its value
  pub fn update_option(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateOption)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_option
  }

  /// A poll was updated; for bots only
  pub fn update_poll(&self) -> &Option<Arc<dyn Fn((&Api, &UpdatePoll)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_poll
  }

  /// A user changed the answer to a poll; for bots only
  pub fn update_poll_answer(&self) -> &Option<Arc<dyn Fn((&Api, &UpdatePollAnswer)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_poll_answer
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

  /// Some data of a secret chat has changed. This update is guaranteed to come before the secret chat identifier is returned to the application
  pub fn update_secret_chat(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateSecretChat)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_secret_chat
  }

  /// The selected background has changed
  pub fn update_selected_background(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateSelectedBackground)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_selected_background
  }

  /// Service notification from the server. Upon receiving this the application must show a popup with the content of the notification
  pub fn update_service_notification(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateServiceNotification)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_service_notification
  }

  /// A sticker set has changed
  pub fn update_sticker_set(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateStickerSet)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_sticker_set
  }

  /// The list of suggested to the user actions has changed
  pub fn update_suggested_actions(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateSuggestedActions)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_suggested_actions
  }

  /// Some data of a supergroup or a channel has changed. This update is guaranteed to come before the supergroup identifier is returned to the application
  pub fn update_supergroup(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateSupergroup)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_supergroup
  }

  /// Some data from supergroupFullInfo has been changed
  pub fn update_supergroup_full_info(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateSupergroupFullInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_supergroup_full_info
  }

  /// New terms of service must be accepted by the user. If the terms of service are declined, then the deleteAccount method must be called with the reason "Decline ToS update"
  pub fn update_terms_of_service(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateTermsOfService)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_terms_of_service
  }

  /// The list of trending sticker sets was updated or some of them were viewed
  pub fn update_trending_sticker_sets(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateTrendingStickerSets)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_trending_sticker_sets
  }

  /// Number of unread chats, i.e. with unread messages or marked as unread, has changed. This update is sent only if the message database is used
  pub fn update_unread_chat_count(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateUnreadChatCount)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_unread_chat_count
  }

  /// Number of unread messages in a chat list has changed. This update is sent only if the message database is used
  pub fn update_unread_message_count(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateUnreadMessageCount)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_unread_message_count
  }

  /// Some data of a user has changed. This update is guaranteed to come before the user identifier is returned to the application
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

  /// The list of users nearby has changed. The update is guaranteed to be sent only 60 seconds after a successful searchChatsNearby request
  pub fn update_users_nearby(&self) -> &Option<Arc<dyn Fn((&Api, &UpdateUsersNearby)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update_users_nearby
  }



  /// Represents the current authorization state of the TDLib client
  pub fn authorization_state(&self) -> &Option<Arc<dyn Fn((&Api, &AuthorizationState)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.authorization_state
  }

  /// Represents result of checking whether the current session can be used to transfer a chat ownership to another user
  pub fn can_transfer_ownership_result(&self) -> &Option<Arc<dyn Fn((&Api, &CanTransferOwnershipResult)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.can_transfer_ownership_result
  }

  /// Contains a detailed statistics about a chat
  pub fn chat_statistics(&self) -> &Option<Arc<dyn Fn((&Api, &ChatStatistics)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_statistics
  }

  /// Represents result of checking whether a username can be set for a chat
  pub fn check_chat_username_result(&self) -> &Option<Arc<dyn Fn((&Api, &CheckChatUsernameResult)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.check_chat_username_result
  }

  /// Represents result of checking whether a name can be used for a new sticker set
  pub fn check_sticker_set_name_result(&self) -> &Option<Arc<dyn Fn((&Api, &CheckStickerSetNameResult)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.check_sticker_set_name_result
  }

  /// Describes an internal https://t.me or tg: link, which must be processed by the app in a special way
  pub fn internal_link_type(&self) -> &Option<Arc<dyn Fn((&Api, &InternalLinkType)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.internal_link_type
  }

  /// Represents a JSON value
  pub fn json_value(&self) -> &Option<Arc<dyn Fn((&Api, &JsonValue)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.json_value
  }

  /// Represents the value of a string in a language pack
  pub fn language_pack_string_value(&self) -> &Option<Arc<dyn Fn((&Api, &LanguagePackStringValue)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.language_pack_string_value
  }

  /// Describes a stream to which TDLib internal log is written
  pub fn log_stream(&self) -> &Option<Arc<dyn Fn((&Api, &LogStream)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.log_stream
  }

  /// Contains information about an inline button of type inlineKeyboardButtonTypeLoginUrl
  pub fn login_url_info(&self) -> &Option<Arc<dyn Fn((&Api, &LoginUrlInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.login_url_info
  }

  /// Contains information about a file with messages exported from another app
  pub fn message_file_type(&self) -> &Option<Arc<dyn Fn((&Api, &MessageFileType)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.message_file_type
  }

  /// Represents the value of an option
  pub fn option_value(&self) -> &Option<Arc<dyn Fn((&Api, &OptionValue)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.option_value
  }

  /// Contains information about a Telegram Passport element
  pub fn passport_element(&self) -> &Option<Arc<dyn Fn((&Api, &PassportElement)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.passport_element
  }

  /// Represents result of 2-step verification password reset
  pub fn reset_password_result(&self) -> &Option<Arc<dyn Fn((&Api, &ResetPasswordResult)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.reset_password_result
  }

  /// Describes a statistical graph
  pub fn statistical_graph(&self) -> &Option<Arc<dyn Fn((&Api, &StatisticalGraph)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.statistical_graph
  }

  /// Contains notifications about data changes
  pub fn update(&self) -> &Option<Arc<dyn Fn((&Api, &Update)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.update
  }

  /// Contains information about the period of inactivity after which the current user's account will automatically be deleted
  pub fn account_ttl(&self) -> &Option<Arc<dyn Fn((&Api, &AccountTtl)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.account_ttl
  }

  /// Describes an animated representation of an emoji
  pub fn animated_emoji(&self) -> &Option<Arc<dyn Fn((&Api, &AnimatedEmoji)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.animated_emoji
  }

  /// Represents a list of animations
  pub fn animations(&self) -> &Option<Arc<dyn Fn((&Api, &Animations)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.animations
  }

  /// Information about the authentication code that was sent
  pub fn authentication_code_info(&self) -> &Option<Arc<dyn Fn((&Api, &AuthenticationCodeInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.authentication_code_info
  }

  /// Contains auto-download settings presets for the current user
  pub fn auto_download_settings_presets(&self) -> &Option<Arc<dyn Fn((&Api, &AutoDownloadSettingsPresets)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.auto_download_settings_presets
  }

  /// Describes a chat background
  pub fn background(&self) -> &Option<Arc<dyn Fn((&Api, &Background)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.background
  }

  /// Contains a list of backgrounds
  pub fn backgrounds(&self) -> &Option<Arc<dyn Fn((&Api, &Backgrounds)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.backgrounds
  }

  /// Information about a bank card
  pub fn bank_card_info(&self) -> &Option<Arc<dyn Fn((&Api, &BankCardInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.bank_card_info
  }

  /// Represents a basic group of 0-200 users (must be upgraded to a supergroup to accommodate more than 200 users)
  pub fn basic_group(&self) -> &Option<Arc<dyn Fn((&Api, &BasicGroup)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.basic_group
  }

  /// Contains full information about a basic group
  pub fn basic_group_full_info(&self) -> &Option<Arc<dyn Fn((&Api, &BasicGroupFullInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.basic_group_full_info
  }

  /// Contains a list of bot commands
  pub fn bot_commands(&self) -> &Option<Arc<dyn Fn((&Api, &BotCommands)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.bot_commands
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

  /// Represents a list of chat administrators
  pub fn chat_administrators(&self) -> &Option<Arc<dyn Fn((&Api, &ChatAdministrators)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_administrators
  }

  /// Contains a list of chat events
  pub fn chat_events(&self) -> &Option<Arc<dyn Fn((&Api, &ChatEvents)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_events
  }

  /// Represents a filter of user chats
  pub fn chat_filter(&self) -> &Option<Arc<dyn Fn((&Api, &ChatFilter)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_filter
  }

  /// Contains basic information about a chat filter
  pub fn chat_filter_info(&self) -> &Option<Arc<dyn Fn((&Api, &ChatFilterInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_filter_info
  }

  /// Contains a chat invite link
  pub fn chat_invite_link(&self) -> &Option<Arc<dyn Fn((&Api, &ChatInviteLink)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_invite_link
  }

  /// Contains a list of chat invite link counts
  pub fn chat_invite_link_counts(&self) -> &Option<Arc<dyn Fn((&Api, &ChatInviteLinkCounts)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_invite_link_counts
  }

  /// Contains information about a chat invite link
  pub fn chat_invite_link_info(&self) -> &Option<Arc<dyn Fn((&Api, &ChatInviteLinkInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_invite_link_info
  }

  /// Contains a list of chat members joined a chat by an invite link
  pub fn chat_invite_link_members(&self) -> &Option<Arc<dyn Fn((&Api, &ChatInviteLinkMembers)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_invite_link_members
  }

  /// Contains a list of chat invite links
  pub fn chat_invite_links(&self) -> &Option<Arc<dyn Fn((&Api, &ChatInviteLinks)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_invite_links
  }

  /// Contains a list of chat join requests
  pub fn chat_join_requests(&self) -> &Option<Arc<dyn Fn((&Api, &ChatJoinRequests)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_join_requests
  }

  /// Contains a list of chat lists
  pub fn chat_lists(&self) -> &Option<Arc<dyn Fn((&Api, &ChatLists)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_lists
  }

  /// Describes a user or a chat as a member of another chat
  pub fn chat_member(&self) -> &Option<Arc<dyn Fn((&Api, &ChatMember)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_member
  }

  /// Contains a list of chat members
  pub fn chat_members(&self) -> &Option<Arc<dyn Fn((&Api, &ChatMembers)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_members
  }

  /// Contains a list of chat or user profile photos
  pub fn chat_photos(&self) -> &Option<Arc<dyn Fn((&Api, &ChatPhotos)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chat_photos
  }

  /// Represents a list of chats
  pub fn chats(&self) -> &Option<Arc<dyn Fn((&Api, &Chats)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chats
  }

  /// Represents a list of chats located nearby
  pub fn chats_nearby(&self) -> &Option<Arc<dyn Fn((&Api, &ChatsNearby)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.chats_nearby
  }

  /// Contains a list of websites the current user is logged in with Telegram
  pub fn connected_websites(&self) -> &Option<Arc<dyn Fn((&Api, &ConnectedWebsites)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.connected_websites
  }

  /// Contains a counter
  pub fn count(&self) -> &Option<Arc<dyn Fn((&Api, &Count)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.count
  }

  /// Contains information about countries
  pub fn countries(&self) -> &Option<Arc<dyn Fn((&Api, &Countries)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.countries
  }

  /// Contains the result of a custom request
  pub fn custom_request_result(&self) -> &Option<Arc<dyn Fn((&Api, &CustomRequestResult)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.custom_request_result
  }

  /// Contains database statistics
  pub fn database_statistics(&self) -> &Option<Arc<dyn Fn((&Api, &DatabaseStatistics)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.database_statistics
  }

  /// Contains information about a tg: deep link
  pub fn deep_link_info(&self) -> &Option<Arc<dyn Fn((&Api, &DeepLinkInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.deep_link_info
  }

  /// Information about the email address authentication code that was sent
  pub fn email_address_authentication_code_info(&self) -> &Option<Arc<dyn Fn((&Api, &EmailAddressAuthenticationCodeInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.email_address_authentication_code_info
  }

  /// Represents a list of emoji
  pub fn emojis(&self) -> &Option<Arc<dyn Fn((&Api, &Emojis)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.emojis
  }

  /// An object of this type can be returned on every function call, in case of an error
  pub fn error(&self) -> &Option<Arc<dyn Fn((&Api, &Error)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.error
  }

  /// Represents a file
  pub fn file(&self) -> &Option<Arc<dyn Fn((&Api, &File)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.file
  }

  /// Contains a part of a file
  pub fn file_part(&self) -> &Option<Arc<dyn Fn((&Api, &FilePart)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.file_part
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

  /// Describes a group call
  pub fn group_call(&self) -> &Option<Arc<dyn Fn((&Api, &GroupCall)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.group_call
  }

  /// Contains the group call identifier
  pub fn group_call_id(&self) -> &Option<Arc<dyn Fn((&Api, &GroupCallId)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.group_call_id
  }

  /// Contains a list of hashtags
  pub fn hashtags(&self) -> &Option<Arc<dyn Fn((&Api, &Hashtags)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.hashtags
  }

  /// Contains an HTTP URL
  pub fn http_url(&self) -> &Option<Arc<dyn Fn((&Api, &HttpUrl)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.http_url
  }

  /// Represents the result of an ImportContacts request
  pub fn imported_contacts(&self) -> &Option<Arc<dyn Fn((&Api, &ImportedContacts)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.imported_contacts
  }

  /// Represents the results of the inline query. Use sendInlineQueryResultMessage to send the result of the query
  pub fn inline_query_results(&self) -> &Option<Arc<dyn Fn((&Api, &InlineQueryResults)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.inline_query_results
  }

  /// Contains information about a language pack
  pub fn language_pack_info(&self) -> &Option<Arc<dyn Fn((&Api, &LanguagePackInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.language_pack_info
  }

  /// Contains a list of language pack strings
  pub fn language_pack_strings(&self) -> &Option<Arc<dyn Fn((&Api, &LanguagePackStrings)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.language_pack_strings
  }

  /// Contains information about the current localization target
  pub fn localization_target_info(&self) -> &Option<Arc<dyn Fn((&Api, &LocalizationTargetInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.localization_target_info
  }

  /// Contains a list of available TDLib internal log tags
  pub fn log_tags(&self) -> &Option<Arc<dyn Fn((&Api, &LogTags)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.log_tags
  }

  /// Contains a TDLib internal log verbosity level
  pub fn log_verbosity_level(&self) -> &Option<Arc<dyn Fn((&Api, &LogVerbosityLevel)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.log_verbosity_level
  }

  /// Describes a message
  pub fn message(&self) -> &Option<Arc<dyn Fn((&Api, &Message)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.message
  }

  /// Contains information about found messages, splitted by days according to the option "utc_time_offset"
  pub fn message_calendar(&self) -> &Option<Arc<dyn Fn((&Api, &MessageCalendar)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.message_calendar
  }

  /// Contains an HTTPS link to a message in a supergroup or channel
  pub fn message_link(&self) -> &Option<Arc<dyn Fn((&Api, &MessageLink)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.message_link
  }

  /// Contains information about a link to a message in a chat
  pub fn message_link_info(&self) -> &Option<Arc<dyn Fn((&Api, &MessageLinkInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.message_link_info
  }

  /// Contains a list of message positions
  pub fn message_positions(&self) -> &Option<Arc<dyn Fn((&Api, &MessagePositions)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.message_positions
  }

  /// Represents a list of message senders
  pub fn message_senders(&self) -> &Option<Arc<dyn Fn((&Api, &MessageSenders)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.message_senders
  }

  /// A detailed statistics about a message
  pub fn message_statistics(&self) -> &Option<Arc<dyn Fn((&Api, &MessageStatistics)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.message_statistics
  }

  /// Contains information about a message thread
  pub fn message_thread_info(&self) -> &Option<Arc<dyn Fn((&Api, &MessageThreadInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.message_thread_info
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

  /// Contains information about a Telegram Passport elements and corresponding errors
  pub fn passport_elements_with_errors(&self) -> &Option<Arc<dyn Fn((&Api, &PassportElementsWithErrors)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.passport_elements_with_errors
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

  /// Contains information about a phone number
  pub fn phone_number_info(&self) -> &Option<Arc<dyn Fn((&Api, &PhoneNumberInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.phone_number_info
  }

  /// Represents a list of proxy servers
  pub fn proxies(&self) -> &Option<Arc<dyn Fn((&Api, &Proxies)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.proxies
  }

  /// Contains information about a proxy server
  pub fn proxy(&self) -> &Option<Arc<dyn Fn((&Api, &Proxy)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.proxy
  }

  /// Contains a globally unique push receiver identifier, which can be used to identify which account has received a push notification
  pub fn push_receiver_id(&self) -> &Option<Arc<dyn Fn((&Api, &PushReceiverId)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.push_receiver_id
  }

  /// Contains a list of recommended chat filters
  pub fn recommended_chat_filters(&self) -> &Option<Arc<dyn Fn((&Api, &RecommendedChatFilters)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.recommended_chat_filters
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

  /// Contains information about one session in a Telegram application used by the current user. Sessions must be shown to the user in the returned order
  pub fn session(&self) -> &Option<Arc<dyn Fn((&Api, &Session)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.session
  }

  /// Contains a list of sessions
  pub fn sessions(&self) -> &Option<Arc<dyn Fn((&Api, &Sessions)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.sessions
  }

  /// Contains a list of sponsored messages
  pub fn sponsored_messages(&self) -> &Option<Arc<dyn Fn((&Api, &SponsoredMessages)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.sponsored_messages
  }

  /// Describes a sticker
  pub fn sticker(&self) -> &Option<Arc<dyn Fn((&Api, &Sticker)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.sticker
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

  /// Contains a list of updates
  pub fn updates(&self) -> &Option<Arc<dyn Fn((&Api, &Updates)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.updates
  }

  /// Represents a user
  pub fn user(&self) -> &Option<Arc<dyn Fn((&Api, &User)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.user
  }

  /// Contains full information about a user
  pub fn user_full_info(&self) -> &Option<Arc<dyn Fn((&Api, &UserFullInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.user_full_info
  }

  /// A list of privacy rules. Rules are matched in the specified order. The first matched rule defines the privacy setting for a given user. If no rule matches, the action is not allowed
  pub fn user_privacy_setting_rules(&self) -> &Option<Arc<dyn Fn((&Api, &UserPrivacySettingRules)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.user_privacy_setting_rules
  }

  /// Represents a list of users
  pub fn users(&self) -> &Option<Arc<dyn Fn((&Api, &Users)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.users
  }

  /// Contains a temporary identifier of validated order information, which is stored for one hour. Also contains the available shipping options
  pub fn validated_order_info(&self) -> &Option<Arc<dyn Fn((&Api, &ValidatedOrderInfo)) -> TGResult<()> + Send + Sync + 'static>> {
    &self.listener.validated_order_info
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



