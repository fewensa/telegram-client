use rtdlib::types as rtd_types;

use crate::api::aevent::EventApi;
use crate::listener::Lout;
use crate::errors::TGError;
use crate::observer;
use crate::tip;
use rtdlib::types::*;

pub struct Handler<'a> {
  api: &'a EventApi,
  lout: &'a Lout,
  warn_unregister_listener: &'a bool,
}

// macro_rules! event_handler {
//   ($event_name:ident, $td_type:ident) => {
//     |api: &EventApi, lout: &Lout, json: &String, extra: &String| {
//       if let Some(ev) = lout.$event_name() {
//         match rtd_types::from_json::<rtd_types::$td_type>(json) {
//           Ok(t) => {
//             if let Err(_e) = ev((api, &t)) {
//               if let Some(ev) = lout.exception() { ev((api, &TGError::new("EVENT_HANDLER_ERROR"))); }
//             }
//             observer::notify(extra.to_string(), rtd_types::TdType::$td_type(t));
//           }
//           Err(e) => {
//             error!("{}", tip::data_fail_with_json(json));
//             eprintln!("{:?}", e);
//             if let Some(ev) = lout.exception() { ev((api, &TGError::new("DESERIALIZE_JSON_FAIL"))); }
//           }
//         }
//         return;
//       }
//       warn!("{}", tip::un_register_listener(stringify!($event_name)));
//     }
//   };
// }
//
// macro_rules! update_handler {
//   ($event_name:ident, $td_type:ident, $return_type:ident) => {
//     |api: &EventApi, lout: &Lout, json: &String, extra: &String| {
//       if let Some(ev) = lout.$event_name() {
//         let td_type = rtd_types::from_json::<$td_type>(json).unwrap();
//         match rtd_types::from_json::<$return_type>(json) {
//           Ok(t) => {
//             if let Err(_e) = ev((api, &td_type)) {
//               if let Some(ev) = lout.exception() { ev((api, &TGError::new("EVENT_HANDLER_ERROR"))); }
//             }
//             observer::notify(extra.to_string(), TdType::$return_type($return_type::$td_type(td_type)));
//           }
//           Err(e) => {
//             error!("{}", tip::data_fail_with_json(json));
//             eprintln!("{:?}", e);
//             if let Some(ev) = lout.exception() { ev((api, &TGError::new("DESERIALIZE_JSON_FAIL"))); }
//           }
//         }
//         return;
//       }
//       warn!("{}", tip::un_register_listener(stringify!($event_name)));
//     }
//   };
// }

impl<'a> Handler<'a> {
  pub(crate) fn new(api: &'a EventApi, lout: &'a Lout, warn_unregister_listener: &'a bool) -> Self {
    Self {
      api,
      lout,
      warn_unregister_listener,
    }
  }

  pub fn handle(&self, json: &'a String) {
    let (td_type, extra) = match rtd_types::detect_td_type_and_extra(json) {
      (Some(t), Some(e)) => (t, Some(e)),
      (Some(t), None) => (t, None),
      (None, _) => {
        warn!("{}", tip::data_fail_with_json(json));
        return;
      }
    };

    if let Some(ev) = self.lout.receive() {
      if let Err(e) = ev((self.api, json)) {
        if let Some(ev) = self.lout.exception() { ev((self.api, &e)); }
      }
    }


    macro_rules! event_handler {
      ($event_name:ident, $td_type:ident) => {

        match rtd_types::from_json::<rtd_types::$td_type>(json) {
          Ok(t) => {

            // event handler
            if self.lout.is_support(&td_type) {
              if let Some(ev) = self.lout.$event_name() {
                if let Err(_e) = ev((self.api, &t)) {
                  if let Some(ev) = self.lout.exception() {
                    ev((self.api, &TGError::new("EVENT_HANDLER_ERROR")));
                  }
                }
                return;
              }
              if *self.warn_unregister_listener {
                warn!("{}", tip::un_register_listener(stringify!($event_name)));
              }
            } else {
              warn!("{}", tip::not_have_listener(td_type));
            }

            // observer handler
            // todox: not have rtdtype, need add to rtdlib project, see api.rs#TdType
            if let Some(ext) = extra {
              observer::notify(ext, TdType::$td_type(t));
            }
          }
          Err(e) => {
            error!("{}", tip::data_fail_with_json(json));
            // eprintln!("{:?}", e);
            error!("{:?}", e);
            if let Some(ev) = self.lout.exception() { ev((self.api, &TGError::new("DESERIALIZE_JSON_FAIL"))); }
          }
        }

      };
    }

    match &td_type[..] {
      "testUseUpdate" => event_handler!(test_use_update, TestUseUpdate),
      "updateActiveNotifications" => event_handler!(update_active_notifications, UpdateActiveNotifications),
      "updateAnimationSearchParameters" => event_handler!(update_animation_search_parameters, UpdateAnimationSearchParameters),
      "updateAuthorizationState" => event_handler!(update_authorization_state, UpdateAuthorizationState),
      "updateBasicGroup" => event_handler!(update_basic_group, UpdateBasicGroup),
      "updateBasicGroupFullInfo" => event_handler!(update_basic_group_full_info, UpdateBasicGroupFullInfo),
      "updateCall" => event_handler!(update_call, UpdateCall),
      "updateChatActionBar" => event_handler!(update_chat_action_bar, UpdateChatActionBar),
      "updateChatDefaultDisableNotification" => event_handler!(update_chat_default_disable_notification, UpdateChatDefaultDisableNotification),
      "updateChatDraftMessage" => event_handler!(update_chat_draft_message, UpdateChatDraftMessage),
      "updateChatFilters" => event_handler!(update_chat_filters, UpdateChatFilters),
      "updateChatHasScheduledMessages" => event_handler!(update_chat_has_scheduled_messages, UpdateChatHasScheduledMessages),
      "updateChatIsMarkedAsUnread" => event_handler!(update_chat_is_marked_as_unread, UpdateChatIsMarkedAsUnread),
      "updateChatLastMessage" => event_handler!(update_chat_last_message, UpdateChatLastMessage),
      "updateChatNotificationSettings" => event_handler!(update_chat_notification_settings, UpdateChatNotificationSettings),
      "updateChatOnlineMemberCount" => event_handler!(update_chat_online_member_count, UpdateChatOnlineMemberCount),
      "updateChatPermissions" => event_handler!(update_chat_permissions, UpdateChatPermissions),
      "updateChatPhoto" => event_handler!(update_chat_photo, UpdateChatPhoto),
      "updateChatPinnedMessage" => event_handler!(update_chat_pinned_message, UpdateChatPinnedMessage),
      "updateChatPosition" => event_handler!(update_chat_position, UpdateChatPosition),
      "updateChatReadInbox" => event_handler!(update_chat_read_inbox, UpdateChatReadInbox),
      "updateChatReadOutbox" => event_handler!(update_chat_read_outbox, UpdateChatReadOutbox),
      "updateChatReplyMarkup" => event_handler!(update_chat_reply_markup, UpdateChatReplyMarkup),
      "updateChatTitle" => event_handler!(update_chat_title, UpdateChatTitle),
      "updateChatUnreadMentionCount" => event_handler!(update_chat_unread_mention_count, UpdateChatUnreadMentionCount),
      "updateConnectionState" => event_handler!(update_connection_state, UpdateConnectionState),
      "updateDeleteMessages" => event_handler!(update_delete_messages, UpdateDeleteMessages),
      "updateDiceEmojis" => event_handler!(update_dice_emojis, UpdateDiceEmojis),
      "updateFavoriteStickers" => event_handler!(update_favorite_stickers, UpdateFavoriteStickers),
      "updateFile" => event_handler!(update_file, UpdateFile),
      "updateFileGenerationStart" => event_handler!(update_file_generation_start, UpdateFileGenerationStart),
      "updateFileGenerationStop" => event_handler!(update_file_generation_stop, UpdateFileGenerationStop),
      "updateHavePendingNotifications" => event_handler!(update_have_pending_notifications, UpdateHavePendingNotifications),
      "updateInstalledStickerSets" => event_handler!(update_installed_sticker_sets, UpdateInstalledStickerSets),
      "updateLanguagePackStrings" => event_handler!(update_language_pack_strings, UpdateLanguagePackStrings),
      "updateMessageContent" => event_handler!(update_message_content, UpdateMessageContent),
      "updateMessageContentOpened" => event_handler!(update_message_content_opened, UpdateMessageContentOpened),
      "updateMessageEdited" => event_handler!(update_message_edited, UpdateMessageEdited),
      "updateMessageLiveLocationViewed" => event_handler!(update_message_live_location_viewed, UpdateMessageLiveLocationViewed),
      "updateMessageMentionRead" => event_handler!(update_message_mention_read, UpdateMessageMentionRead),
      "updateMessageSendAcknowledged" => event_handler!(update_message_send_acknowledged, UpdateMessageSendAcknowledged),
      "updateMessageSendFailed" => event_handler!(update_message_send_failed, UpdateMessageSendFailed),
      "updateMessageSendSucceeded" => event_handler!(update_message_send_succeeded, UpdateMessageSendSucceeded),
      "updateMessageViews" => event_handler!(update_message_views, UpdateMessageViews),
      "updateNewCallSignalingData" => event_handler!(update_new_call_signaling_data, UpdateNewCallSignalingData),
      "updateNewCallbackQuery" => event_handler!(update_new_callback_query, UpdateNewCallbackQuery),
      "updateNewChat" => event_handler!(update_new_chat, UpdateNewChat),
      "updateNewChosenInlineResult" => event_handler!(update_new_chosen_inline_result, UpdateNewChosenInlineResult),
      "updateNewCustomEvent" => event_handler!(update_new_custom_event, UpdateNewCustomEvent),
      "updateNewCustomQuery" => event_handler!(update_new_custom_query, UpdateNewCustomQuery),
      "updateNewInlineCallbackQuery" => event_handler!(update_new_inline_callback_query, UpdateNewInlineCallbackQuery),
      "updateNewInlineQuery" => event_handler!(update_new_inline_query, UpdateNewInlineQuery),
      "updateNewMessage" => event_handler!(update_new_message, UpdateNewMessage),
      "updateNewPreCheckoutQuery" => event_handler!(update_new_pre_checkout_query, UpdateNewPreCheckoutQuery),
      "updateNewShippingQuery" => event_handler!(update_new_shipping_query, UpdateNewShippingQuery),
      "updateNotification" => event_handler!(update_notification, UpdateNotification),
      "updateNotificationGroup" => event_handler!(update_notification_group, UpdateNotificationGroup),
      "updateOption" => event_handler!(update_option, UpdateOption),
      "updatePoll" => event_handler!(update_poll, UpdatePoll),
      "updatePollAnswer" => event_handler!(update_poll_answer, UpdatePollAnswer),
      "updateRecentStickers" => event_handler!(update_recent_stickers, UpdateRecentStickers),
      "updateSavedAnimations" => event_handler!(update_saved_animations, UpdateSavedAnimations),
      "updateScopeNotificationSettings" => event_handler!(update_scope_notification_settings, UpdateScopeNotificationSettings),
      "updateSecretChat" => event_handler!(update_secret_chat, UpdateSecretChat),
      "updateSelectedBackground" => event_handler!(update_selected_background, UpdateSelectedBackground),
      "updateServiceNotification" => event_handler!(update_service_notification, UpdateServiceNotification),
      "updateStickerSet" => event_handler!(update_sticker_set, UpdateStickerSet),
      "updateSuggestedActions" => event_handler!(update_suggested_actions, UpdateSuggestedActions),
      "updateSupergroup" => event_handler!(update_supergroup, UpdateSupergroup),
      "updateSupergroupFullInfo" => event_handler!(update_supergroup_full_info, UpdateSupergroupFullInfo),
      "updateTermsOfService" => event_handler!(update_terms_of_service, UpdateTermsOfService),
      "updateTrendingStickerSets" => event_handler!(update_trending_sticker_sets, UpdateTrendingStickerSets),
      "updateUnreadChatCount" => event_handler!(update_unread_chat_count, UpdateUnreadChatCount),
      "updateUnreadMessageCount" => event_handler!(update_unread_message_count, UpdateUnreadMessageCount),
      "updateUser" => event_handler!(update_user, UpdateUser),
      "updateUserChatAction" => event_handler!(update_user_chat_action, UpdateUserChatAction),
      "updateUserFullInfo" => event_handler!(update_user_full_info, UpdateUserFullInfo),
      "updateUserPrivacySettingRules" => event_handler!(update_user_privacy_setting_rules, UpdateUserPrivacySettingRules),
      "updateUserStatus" => event_handler!(update_user_status, UpdateUserStatus),
      "updateUsersNearby" => event_handler!(update_users_nearby, UpdateUsersNearby),

      "AuthorizationState" => event_handler!(authorization_state, AuthorizationState),
      "CanTransferOwnershipResult" => event_handler!(can_transfer_ownership_result, CanTransferOwnershipResult),
      "ChatStatistics" => event_handler!(chat_statistics, ChatStatistics),
      "CheckChatUsernameResult" => event_handler!(check_chat_username_result, CheckChatUsernameResult),
      "JsonValue" => event_handler!(json_value, JsonValue),
      "LanguagePackStringValue" => event_handler!(language_pack_string_value, LanguagePackStringValue),
      "LogStream" => event_handler!(log_stream, LogStream),
      "LoginUrlInfo" => event_handler!(login_url_info, LoginUrlInfo),
      "OptionValue" => event_handler!(option_value, OptionValue),
      "PassportElement" => event_handler!(passport_element, PassportElement),
      "StatisticsGraph" => event_handler!(statistics_graph, StatisticsGraph),
      "Update" => event_handler!(update, Update),
      "accountTtl" => event_handler!(account_ttl, AccountTtl),
      "animations" => event_handler!(animations, Animations),
      "authenticationCodeInfo" => event_handler!(authentication_code_info, AuthenticationCodeInfo),
      "autoDownloadSettingsPresets" => event_handler!(auto_download_settings_presets, AutoDownloadSettingsPresets),
      "background" => event_handler!(background, Background),
      "backgrounds" => event_handler!(backgrounds, Backgrounds),
      "bankCardInfo" => event_handler!(bank_card_info, BankCardInfo),
      "basicGroup" => event_handler!(basic_group, BasicGroup),
      "basicGroupFullInfo" => event_handler!(basic_group_full_info, BasicGroupFullInfo),
      "callId" => event_handler!(call_id, CallId),
      "callbackQueryAnswer" => event_handler!(callback_query_answer, CallbackQueryAnswer),
      "chat" => event_handler!(chat, Chat),
      "chatAdministrators" => event_handler!(chat_administrators, ChatAdministrators),
      "chatEvents" => event_handler!(chat_events, ChatEvents),
      "chatFilter" => event_handler!(chat_filter, ChatFilter),
      "chatFilterInfo" => event_handler!(chat_filter_info, ChatFilterInfo),
      "chatInviteLink" => event_handler!(chat_invite_link, ChatInviteLink),
      "chatInviteLinkInfo" => event_handler!(chat_invite_link_info, ChatInviteLinkInfo),
      "chatLists" => event_handler!(chat_lists, ChatLists),
      "chatMember" => event_handler!(chat_member, ChatMember),
      "chatMembers" => event_handler!(chat_members, ChatMembers),
      "chatPhotos" => event_handler!(chat_photos, ChatPhotos),
      "chats" => event_handler!(chats, Chats),
      "chatsNearby" => event_handler!(chats_nearby, ChatsNearby),
      "connectedWebsites" => event_handler!(connected_websites, ConnectedWebsites),
      "count" => event_handler!(count, Count),
      "customRequestResult" => event_handler!(custom_request_result, CustomRequestResult),
      "databaseStatistics" => event_handler!(database_statistics, DatabaseStatistics),
      "deepLinkInfo" => event_handler!(deep_link_info, DeepLinkInfo),
      "emailAddressAuthenticationCodeInfo" => event_handler!(email_address_authentication_code_info, EmailAddressAuthenticationCodeInfo),
      "emojis" => event_handler!(emojis, Emojis),
      "error" => event_handler!(error, Error),
      "file" => event_handler!(file, File),
      "filePart" => event_handler!(file_part, FilePart),
      "formattedText" => event_handler!(formatted_text, FormattedText),
      "foundMessages" => event_handler!(found_messages, FoundMessages),
      "gameHighScores" => event_handler!(game_high_scores, GameHighScores),
      "hashtags" => event_handler!(hashtags, Hashtags),
      "httpUrl" => event_handler!(http_url, HttpUrl),
      "importedContacts" => event_handler!(imported_contacts, ImportedContacts),
      "inlineQueryResults" => event_handler!(inline_query_results, InlineQueryResults),
      "languagePackInfo" => event_handler!(language_pack_info, LanguagePackInfo),
      "languagePackStrings" => event_handler!(language_pack_strings, LanguagePackStrings),
      "localizationTargetInfo" => event_handler!(localization_target_info, LocalizationTargetInfo),
      "logTags" => event_handler!(log_tags, LogTags),
      "logVerbosityLevel" => event_handler!(log_verbosity_level, LogVerbosityLevel),
      "message" => event_handler!(message, Message),
      "messageLinkInfo" => event_handler!(message_link_info, MessageLinkInfo),
      "messages" => event_handler!(messages, Messages),
      "networkStatistics" => event_handler!(network_statistics, NetworkStatistics),
      "ok" => event_handler!(ok, Ok),
      "orderInfo" => event_handler!(order_info, OrderInfo),
      "passportAuthorizationForm" => event_handler!(passport_authorization_form, PassportAuthorizationForm),
      "passportElements" => event_handler!(passport_elements, PassportElements),
      "passportElementsWithErrors" => event_handler!(passport_elements_with_errors, PassportElementsWithErrors),
      "passwordState" => event_handler!(password_state, PasswordState),
      "paymentForm" => event_handler!(payment_form, PaymentForm),
      "paymentReceipt" => event_handler!(payment_receipt, PaymentReceipt),
      "paymentResult" => event_handler!(payment_result, PaymentResult),
      "proxies" => event_handler!(proxies, Proxies),
      "proxy" => event_handler!(proxy, Proxy),
      "publicMessageLink" => event_handler!(public_message_link, PublicMessageLink),
      "pushReceiverId" => event_handler!(push_receiver_id, PushReceiverId),
      "recommendedChatFilters" => event_handler!(recommended_chat_filters, RecommendedChatFilters),
      "recoveryEmailAddress" => event_handler!(recovery_email_address, RecoveryEmailAddress),
      "scopeNotificationSettings" => event_handler!(scope_notification_settings, ScopeNotificationSettings),
      "seconds" => event_handler!(seconds, Seconds),
      "secretChat" => event_handler!(secret_chat, SecretChat),
      "session" => event_handler!(session, Session),
      "sessions" => event_handler!(sessions, Sessions),
      "stickerSet" => event_handler!(sticker_set, StickerSet),
      "stickerSets" => event_handler!(sticker_sets, StickerSets),
      "stickers" => event_handler!(stickers, Stickers),
      "storageStatistics" => event_handler!(storage_statistics, StorageStatistics),
      "storageStatisticsFast" => event_handler!(storage_statistics_fast, StorageStatisticsFast),
      "supergroup" => event_handler!(supergroup, Supergroup),
      "supergroupFullInfo" => event_handler!(supergroup_full_info, SupergroupFullInfo),
      "tMeUrls" => event_handler!(t_me_urls, TMeUrls),
      "temporaryPasswordState" => event_handler!(temporary_password_state, TemporaryPasswordState),
      "testBytes" => event_handler!(test_bytes, TestBytes),
      "testInt" => event_handler!(test_int, TestInt),
      "testString" => event_handler!(test_string, TestString),
      "testVectorInt" => event_handler!(test_vector_int, TestVectorInt),
      "testVectorIntObject" => event_handler!(test_vector_int_object, TestVectorIntObject),
      "testVectorString" => event_handler!(test_vector_string, TestVectorString),
      "testVectorStringObject" => event_handler!(test_vector_string_object, TestVectorStringObject),
      "text" => event_handler!(text, Text),
      "textEntities" => event_handler!(text_entities, TextEntities),
      "updates" => event_handler!(updates, Updates),
      "user" => event_handler!(user, User),
      "userFullInfo" => event_handler!(user_full_info, UserFullInfo),
      "userPrivacySettingRules" => event_handler!(user_privacy_setting_rules, UserPrivacySettingRules),
      "users" => event_handler!(users, Users),
      "validatedOrderInfo" => event_handler!(validated_order_info, ValidatedOrderInfo),
      "webPage" => event_handler!(web_page, WebPage),
      "webPageInstantView" => event_handler!(web_page_instant_view, WebPageInstantView),



      _ => {
        warn!("{}", tip::data_fail_with_json(json))
      }
    }
  }

}


