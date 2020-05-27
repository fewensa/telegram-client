use rtdlib::types as rtd_types;

use crate::api::Api;
use crate::listener::Lout;
use crate::errors::TGError;
use crate::tip;

pub struct Handler<'a> {
  api: &'a Api,
  lout: &'a Lout,
}

macro_rules! event_handler {
  ($event_name:ident, $td_type:ident) => {
    |api: &Api, lout: &Lout, json: &String| {
      if let Some(ev) = lout.$event_name() {
        match rtd_types::from_json::<rtd_types::$td_type>(json) {
          Ok(t) => {
            if let Err(_e) = ev((api, &t)) {
              if let Some(ev) = lout.exception() { ev((api, &TGError::new("EVENT_HANDLER_ERROR"))); }
            }
          }
          Err(e) => {
            error!("{}", tip::data_fail_with_json(json));
            eprintln!("{:?}", e);
            if let Some(ev) = lout.exception() { ev((api, &TGError::new("DESERIALIZE_JSON_FAIL"))); }
          }
        }
        return;
      }
      warn!("{}", tip::un_register_listener(stringify!($event_name)));
    }
  };
}

impl<'a> Handler<'a> {
  pub(crate) fn new(api: &'a Api, lout: &'a Lout) -> Self {
    Self {
      api,
      lout,
    }
  }

  pub fn handle(&self, json: &'a String) {
    let td_type = match rtd_types::detect_td_type(json) {
      Some(t) => t,
      None => {
        warn!("{}", tip::data_fail_with_json(json));
        return;
      }
    };
    if !self.lout.is_support(&td_type) {
      warn!("{}", tip::not_have_listener(td_type));
      return;
    }

    if let Some(ev) = self.lout.receive() {
      if let Err(e) = ev((self.api, json)) {
        if let Some(ev) = self.lout.exception() { ev((self.api, &e)); }
      }
    }

    match &td_type[..] {
      "testUseUpdate" => event_handler!(test_use_update, TestUseUpdate)(self.api, self.lout, json),
      "updateActiveNotifications" => event_handler!(update_active_notifications, UpdateActiveNotifications)(self.api, self.lout, json),
      "updateAuthorizationState" => event_handler!(update_authorization_state, UpdateAuthorizationState)(self.api, self.lout, json),
      "updateBasicGroup" => event_handler!(update_basic_group, UpdateBasicGroup)(self.api, self.lout, json),
      "updateBasicGroupFullInfo" => event_handler!(update_basic_group_full_info, UpdateBasicGroupFullInfo)(self.api, self.lout, json),
      "updateCall" => event_handler!(update_call, UpdateCall)(self.api, self.lout, json),
      "updateChatDefaultDisableNotification" => event_handler!(update_chat_default_disable_notification, UpdateChatDefaultDisableNotification)(self.api, self.lout, json),
      "updateChatDraftMessage" => event_handler!(update_chat_draft_message, UpdateChatDraftMessage)(self.api, self.lout, json),
      "updateChatIsMarkedAsUnread" => event_handler!(update_chat_is_marked_as_unread, UpdateChatIsMarkedAsUnread)(self.api, self.lout, json),
      "updateChatIsPinned" => event_handler!(update_chat_is_pinned, UpdateChatIsPinned)(self.api, self.lout, json),
      "updateChatIsSponsored" => event_handler!(update_chat_is_sponsored, UpdateChatIsSponsored)(self.api, self.lout, json),
      "updateChatLastMessage" => event_handler!(update_chat_last_message, UpdateChatLastMessage)(self.api, self.lout, json),
      "updateChatNotificationSettings" => event_handler!(update_chat_notification_settings, UpdateChatNotificationSettings)(self.api, self.lout, json),
      "updateChatOnlineMemberCount" => event_handler!(update_chat_online_member_count, UpdateChatOnlineMemberCount)(self.api, self.lout, json),
      "updateChatOrder" => event_handler!(update_chat_order, UpdateChatOrder)(self.api, self.lout, json),
      "updateChatPhoto" => event_handler!(update_chat_photo, UpdateChatPhoto)(self.api, self.lout, json),
      "updateChatPinnedMessage" => event_handler!(update_chat_pinned_message, UpdateChatPinnedMessage)(self.api, self.lout, json),
      "updateChatReadInbox" => event_handler!(update_chat_read_inbox, UpdateChatReadInbox)(self.api, self.lout, json),
      "updateChatReadOutbox" => event_handler!(update_chat_read_outbox, UpdateChatReadOutbox)(self.api, self.lout, json),
      "updateChatReplyMarkup" => event_handler!(update_chat_reply_markup, UpdateChatReplyMarkup)(self.api, self.lout, json),
      "updateChatTitle" => event_handler!(update_chat_title, UpdateChatTitle)(self.api, self.lout, json),
      "updateChatUnreadMentionCount" => event_handler!(update_chat_unread_mention_count, UpdateChatUnreadMentionCount)(self.api, self.lout, json),
      "updateConnectionState" => event_handler!(update_connection_state, UpdateConnectionState)(self.api, self.lout, json),
      "updateDeleteMessages" => event_handler!(update_delete_messages, UpdateDeleteMessages)(self.api, self.lout, json),
      "updateFavoriteStickers" => event_handler!(update_favorite_stickers, UpdateFavoriteStickers)(self.api, self.lout, json),
      "updateFile" => event_handler!(update_file, UpdateFile)(self.api, self.lout, json),
      "updateFileGenerationStart" => event_handler!(update_file_generation_start, UpdateFileGenerationStart)(self.api, self.lout, json),
      "updateFileGenerationStop" => event_handler!(update_file_generation_stop, UpdateFileGenerationStop)(self.api, self.lout, json),
      "updateHavePendingNotifications" => event_handler!(update_have_pending_notifications, UpdateHavePendingNotifications)(self.api, self.lout, json),
      "updateInstalledStickerSets" => event_handler!(update_installed_sticker_sets, UpdateInstalledStickerSets)(self.api, self.lout, json),
      "updateLanguagePackStrings" => event_handler!(update_language_pack_strings, UpdateLanguagePackStrings)(self.api, self.lout, json),
      "updateMessageContent" => event_handler!(update_message_content, UpdateMessageContent)(self.api, self.lout, json),
      "updateMessageContentOpened" => event_handler!(update_message_content_opened, UpdateMessageContentOpened)(self.api, self.lout, json),
      "updateMessageEdited" => event_handler!(update_message_edited, UpdateMessageEdited)(self.api, self.lout, json),
      "updateMessageMentionRead" => event_handler!(update_message_mention_read, UpdateMessageMentionRead)(self.api, self.lout, json),
      "updateMessageSendAcknowledged" => event_handler!(update_message_send_acknowledged, UpdateMessageSendAcknowledged)(self.api, self.lout, json),
      "updateMessageSendFailed" => event_handler!(update_message_send_failed, UpdateMessageSendFailed)(self.api, self.lout, json),
      "updateMessageSendSucceeded" => event_handler!(update_message_send_succeeded, UpdateMessageSendSucceeded)(self.api, self.lout, json),
      "updateMessageViews" => event_handler!(update_message_views, UpdateMessageViews)(self.api, self.lout, json),
      "updateNewCallbackQuery" => event_handler!(update_new_callback_query, UpdateNewCallbackQuery)(self.api, self.lout, json),
      "updateNewChat" => event_handler!(update_new_chat, UpdateNewChat)(self.api, self.lout, json),
      "updateNewChosenInlineResult" => event_handler!(update_new_chosen_inline_result, UpdateNewChosenInlineResult)(self.api, self.lout, json),
      "updateNewCustomEvent" => event_handler!(update_new_custom_event, UpdateNewCustomEvent)(self.api, self.lout, json),
      "updateNewCustomQuery" => event_handler!(update_new_custom_query, UpdateNewCustomQuery)(self.api, self.lout, json),
      "updateNewInlineCallbackQuery" => event_handler!(update_new_inline_callback_query, UpdateNewInlineCallbackQuery)(self.api, self.lout, json),
      "updateNewInlineQuery" => event_handler!(update_new_inline_query, UpdateNewInlineQuery)(self.api, self.lout, json),
      "updateNewMessage" => event_handler!(update_new_message, UpdateNewMessage)(self.api, self.lout, json),
      "updateNewPreCheckoutQuery" => event_handler!(update_new_pre_checkout_query, UpdateNewPreCheckoutQuery)(self.api, self.lout, json),
      "updateNewShippingQuery" => event_handler!(update_new_shipping_query, UpdateNewShippingQuery)(self.api, self.lout, json),
      "updateNotification" => event_handler!(update_notification, UpdateNotification)(self.api, self.lout, json),
      "updateNotificationGroup" => event_handler!(update_notification_group, UpdateNotificationGroup)(self.api, self.lout, json),
      "updateOption" => event_handler!(update_option, UpdateOption)(self.api, self.lout, json),
      "updatePoll" => event_handler!(update_poll, UpdatePoll)(self.api, self.lout, json),
      "updateRecentStickers" => event_handler!(update_recent_stickers, UpdateRecentStickers)(self.api, self.lout, json),
      "updateSavedAnimations" => event_handler!(update_saved_animations, UpdateSavedAnimations)(self.api, self.lout, json),
      "updateScopeNotificationSettings" => event_handler!(update_scope_notification_settings, UpdateScopeNotificationSettings)(self.api, self.lout, json),
      "updateSecretChat" => event_handler!(update_secret_chat, UpdateSecretChat)(self.api, self.lout, json),
      "updateServiceNotification" => event_handler!(update_service_notification, UpdateServiceNotification)(self.api, self.lout, json),
      "updateSupergroup" => event_handler!(update_supergroup, UpdateSupergroup)(self.api, self.lout, json),
      "updateSupergroupFullInfo" => event_handler!(update_supergroup_full_info, UpdateSupergroupFullInfo)(self.api, self.lout, json),
      "updateTermsOfService" => event_handler!(update_terms_of_service, UpdateTermsOfService)(self.api, self.lout, json),
      "updateTrendingStickerSets" => event_handler!(update_trending_sticker_sets, UpdateTrendingStickerSets)(self.api, self.lout, json),
      "updateUnreadChatCount" => event_handler!(update_unread_chat_count, UpdateUnreadChatCount)(self.api, self.lout, json),
      "updateUnreadMessageCount" => event_handler!(update_unread_message_count, UpdateUnreadMessageCount)(self.api, self.lout, json),
      "updateUser" => event_handler!(update_user, UpdateUser)(self.api, self.lout, json),
      "updateUserChatAction" => event_handler!(update_user_chat_action, UpdateUserChatAction)(self.api, self.lout, json),
      "updateUserFullInfo" => event_handler!(update_user_full_info, UpdateUserFullInfo)(self.api, self.lout, json),
      "updateUserPrivacySettingRules" => event_handler!(update_user_privacy_setting_rules, UpdateUserPrivacySettingRules)(self.api, self.lout, json),
      "updateUserStatus" => event_handler!(update_user_status, UpdateUserStatus)(self.api, self.lout, json),

      "AuthorizationState" => event_handler!(authorization_state, AuthorizationState)(self.api, self.lout, json),
      "CheckChatUsernameResult" => event_handler!(check_chat_username_result, CheckChatUsernameResult)(self.api, self.lout, json),
      "JsonValue" => event_handler!(json_value, JsonValue)(self.api, self.lout, json),
      "LanguagePackStringValue" => event_handler!(language_pack_string_value, LanguagePackStringValue)(self.api, self.lout, json),
      "LogStream" => event_handler!(log_stream, LogStream)(self.api, self.lout, json),
      "OptionValue" => event_handler!(option_value, OptionValue)(self.api, self.lout, json),
      "PassportElement" => event_handler!(passport_element, PassportElement)(self.api, self.lout, json),
      "Update" => event_handler!(update, Update)(self.api, self.lout, json),
      "accountTtl" => event_handler!(account_ttl, AccountTtl)(self.api, self.lout, json),
      "animations" => event_handler!(animations, Animations)(self.api, self.lout, json),
      "authenticationCodeInfo" => event_handler!(authentication_code_info, AuthenticationCodeInfo)(self.api, self.lout, json),
      "basicGroup" => event_handler!(basic_group, BasicGroup)(self.api, self.lout, json),
      "basicGroupFullInfo" => event_handler!(basic_group_full_info, BasicGroupFullInfo)(self.api, self.lout, json),
      "callId" => event_handler!(call_id, CallId)(self.api, self.lout, json),
      "callbackQueryAnswer" => event_handler!(callback_query_answer, CallbackQueryAnswer)(self.api, self.lout, json),
      "chat" => event_handler!(chat, Chat)(self.api, self.lout, json),
      "chatEvents" => event_handler!(chat_events, ChatEvents)(self.api, self.lout, json),
      "chatInviteLink" => event_handler!(chat_invite_link, ChatInviteLink)(self.api, self.lout, json),
      "chatInviteLinkInfo" => event_handler!(chat_invite_link_info, ChatInviteLinkInfo)(self.api, self.lout, json),
      "chatMember" => event_handler!(chat_member, ChatMember)(self.api, self.lout, json),
      "chatMembers" => event_handler!(chat_members, ChatMembers)(self.api, self.lout, json),
      "chatReportSpamState" => event_handler!(chat_report_spam_state, ChatReportSpamState)(self.api, self.lout, json),
      "chats" => event_handler!(chats, Chats)(self.api, self.lout, json),
      "connectedWebsites" => event_handler!(connected_websites, ConnectedWebsites)(self.api, self.lout, json),
      "count" => event_handler!(count, Count)(self.api, self.lout, json),
      "customRequestResult" => event_handler!(custom_request_result, CustomRequestResult)(self.api, self.lout, json),
      "databaseStatistics" => event_handler!(database_statistics, DatabaseStatistics)(self.api, self.lout, json),
      "deepLinkInfo" => event_handler!(deep_link_info, DeepLinkInfo)(self.api, self.lout, json),
      "emailAddressAuthenticationCodeInfo" => event_handler!(email_address_authentication_code_info, EmailAddressAuthenticationCodeInfo)(self.api, self.lout, json),
      "error" => event_handler!(error, Error)(self.api, self.lout, json),
      "file" => event_handler!(file, File)(self.api, self.lout, json),
      "filePart" => event_handler!(file_part, FilePart)(self.api, self.lout, json),
      "formattedText" => event_handler!(formatted_text, FormattedText)(self.api, self.lout, json),
      "foundMessages" => event_handler!(found_messages, FoundMessages)(self.api, self.lout, json),
      "gameHighScores" => event_handler!(game_high_scores, GameHighScores)(self.api, self.lout, json),
      "hashtags" => event_handler!(hashtags, Hashtags)(self.api, self.lout, json),
      "httpUrl" => event_handler!(http_url, HttpUrl)(self.api, self.lout, json),
      "importedContacts" => event_handler!(imported_contacts, ImportedContacts)(self.api, self.lout, json),
      "inlineQueryResults" => event_handler!(inline_query_results, InlineQueryResults)(self.api, self.lout, json),
      "languagePackInfo" => event_handler!(language_pack_info, LanguagePackInfo)(self.api, self.lout, json),
      "languagePackStrings" => event_handler!(language_pack_strings, LanguagePackStrings)(self.api, self.lout, json),
      "localizationTargetInfo" => event_handler!(localization_target_info, LocalizationTargetInfo)(self.api, self.lout, json),
      "logTags" => event_handler!(log_tags, LogTags)(self.api, self.lout, json),
      "logVerbosityLevel" => event_handler!(log_verbosity_level, LogVerbosityLevel)(self.api, self.lout, json),
      "message" => event_handler!(message, Message)(self.api, self.lout, json),
      "messages" => event_handler!(messages, Messages)(self.api, self.lout, json),
      "networkStatistics" => event_handler!(network_statistics, NetworkStatistics)(self.api, self.lout, json),
      "ok" => event_handler!(ok, Ok)(self.api, self.lout, json),
      "orderInfo" => event_handler!(order_info, OrderInfo)(self.api, self.lout, json),
      "passportAuthorizationForm" => event_handler!(passport_authorization_form, PassportAuthorizationForm)(self.api, self.lout, json),
      "passportElements" => event_handler!(passport_elements, PassportElements)(self.api, self.lout, json),
      "passportElementsWithErrors" => event_handler!(passport_elements_with_errors, PassportElementsWithErrors)(self.api, self.lout, json),
      "passwordState" => event_handler!(password_state, PasswordState)(self.api, self.lout, json),
      "paymentForm" => event_handler!(payment_form, PaymentForm)(self.api, self.lout, json),
      "paymentReceipt" => event_handler!(payment_receipt, PaymentReceipt)(self.api, self.lout, json),
      "paymentResult" => event_handler!(payment_result, PaymentResult)(self.api, self.lout, json),
      "proxies" => event_handler!(proxies, Proxies)(self.api, self.lout, json),
      "proxy" => event_handler!(proxy, Proxy)(self.api, self.lout, json),
      "publicMessageLink" => event_handler!(public_message_link, PublicMessageLink)(self.api, self.lout, json),
      "pushReceiverId" => event_handler!(push_receiver_id, PushReceiverId)(self.api, self.lout, json),
      "recoveryEmailAddress" => event_handler!(recovery_email_address, RecoveryEmailAddress)(self.api, self.lout, json),
      "scopeNotificationSettings" => event_handler!(scope_notification_settings, ScopeNotificationSettings)(self.api, self.lout, json),
      "seconds" => event_handler!(seconds, Seconds)(self.api, self.lout, json),
      "secretChat" => event_handler!(secret_chat, SecretChat)(self.api, self.lout, json),
      "sessions" => event_handler!(sessions, Sessions)(self.api, self.lout, json),
      "stickerEmojis" => event_handler!(sticker_emojis, StickerEmojis)(self.api, self.lout, json),
      "stickerSet" => event_handler!(sticker_set, StickerSet)(self.api, self.lout, json),
      "stickerSets" => event_handler!(sticker_sets, StickerSets)(self.api, self.lout, json),
      "stickers" => event_handler!(stickers, Stickers)(self.api, self.lout, json),
      "storageStatistics" => event_handler!(storage_statistics, StorageStatistics)(self.api, self.lout, json),
      "storageStatisticsFast" => event_handler!(storage_statistics_fast, StorageStatisticsFast)(self.api, self.lout, json),
      "supergroup" => event_handler!(supergroup, Supergroup)(self.api, self.lout, json),
      "supergroupFullInfo" => event_handler!(supergroup_full_info, SupergroupFullInfo)(self.api, self.lout, json),
      "tMeUrls" => event_handler!(t_me_urls, TMeUrls)(self.api, self.lout, json),
      "temporaryPasswordState" => event_handler!(temporary_password_state, TemporaryPasswordState)(self.api, self.lout, json),
      "testBytes" => event_handler!(test_bytes, TestBytes)(self.api, self.lout, json),
      "testInt" => event_handler!(test_int, TestInt)(self.api, self.lout, json),
      "testString" => event_handler!(test_string, TestString)(self.api, self.lout, json),
      "testVectorInt" => event_handler!(test_vector_int, TestVectorInt)(self.api, self.lout, json),
      "testVectorIntObject" => event_handler!(test_vector_int_object, TestVectorIntObject)(self.api, self.lout, json),
      "testVectorString" => event_handler!(test_vector_string, TestVectorString)(self.api, self.lout, json),
      "testVectorStringObject" => event_handler!(test_vector_string_object, TestVectorStringObject)(self.api, self.lout, json),
      "text" => event_handler!(text, Text)(self.api, self.lout, json),
      "textEntities" => event_handler!(text_entities, TextEntities)(self.api, self.lout, json),
      "updates" => event_handler!(updates, Updates)(self.api, self.lout, json),
      "user" => event_handler!(user, User)(self.api, self.lout, json),
      "userFullInfo" => event_handler!(user_full_info, UserFullInfo)(self.api, self.lout, json),
      "userPrivacySettingRules" => event_handler!(user_privacy_setting_rules, UserPrivacySettingRules)(self.api, self.lout, json),
      "userProfilePhotos" => event_handler!(user_profile_photos, UserProfilePhotos)(self.api, self.lout, json),
      "users" => event_handler!(users, Users)(self.api, self.lout, json),
      "validatedOrderInfo" => event_handler!(validated_order_info, ValidatedOrderInfo)(self.api, self.lout, json),
      "wallpapers" => event_handler!(wallpapers, Wallpapers)(self.api, self.lout, json),
      "webPage" => event_handler!(web_page, WebPage)(self.api, self.lout, json),
      "webPageInstantView" => event_handler!(web_page_instant_view, WebPageInstantView)(self.api, self.lout, json),


      _ => {
        warn!("{}", tip::data_fail_with_json(json))
      }
    }
  }

}


