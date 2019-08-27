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
      
      "updateAuthorizationState" => event_handler!(authorization_state, UpdateAuthorizationState)(self.api, self.lout, json),
      
      "updateNewMessage" => event_handler!(new_message, UpdateNewMessage)(self.api, self.lout, json),
      
      "updateMessageSendAcknowledged" => event_handler!(message_send_acknowledged, UpdateMessageSendAcknowledged)(self.api, self.lout, json),
      
      "updateMessageSendSucceeded" => event_handler!(message_send_succeeded, UpdateMessageSendSucceeded)(self.api, self.lout, json),
      
      "updateMessageSendFailed" => event_handler!(message_send_failed, UpdateMessageSendFailed)(self.api, self.lout, json),
      
      "updateMessageContent" => event_handler!(message_content, UpdateMessageContent)(self.api, self.lout, json),
      
      "updateMessageEdited" => event_handler!(message_edited, UpdateMessageEdited)(self.api, self.lout, json),
      
      "updateMessageViews" => event_handler!(message_views, UpdateMessageViews)(self.api, self.lout, json),
      
      "updateMessageContentOpened" => event_handler!(message_content_opened, UpdateMessageContentOpened)(self.api, self.lout, json),
      
      "updateMessageMentionRead" => event_handler!(message_mention_read, UpdateMessageMentionRead)(self.api, self.lout, json),
      
      "updateNewChat" => event_handler!(new_chat, UpdateNewChat)(self.api, self.lout, json),
      
      "updateChatTitle" => event_handler!(chat_title, UpdateChatTitle)(self.api, self.lout, json),
      
      "updateChatPhoto" => event_handler!(chat_photo, UpdateChatPhoto)(self.api, self.lout, json),
      
      "updateChatLastMessage" => event_handler!(chat_last_message, UpdateChatLastMessage)(self.api, self.lout, json),
      
      "updateChatOrder" => event_handler!(chat_order, UpdateChatOrder)(self.api, self.lout, json),
      
      "updateChatIsPinned" => event_handler!(chat_is_pinned, UpdateChatIsPinned)(self.api, self.lout, json),
      
      "updateChatIsMarkedAsUnread" => event_handler!(chat_is_marked_as_unread, UpdateChatIsMarkedAsUnread)(self.api, self.lout, json),
      
      "updateChatIsSponsored" => event_handler!(chat_is_sponsored, UpdateChatIsSponsored)(self.api, self.lout, json),
      
      "updateChatDefaultDisableNotification" => event_handler!(chat_default_disable_notification, UpdateChatDefaultDisableNotification)(self.api, self.lout, json),
      
      "updateChatReadInbox" => event_handler!(chat_read_inbox, UpdateChatReadInbox)(self.api, self.lout, json),
      
      "updateChatReadOutbox" => event_handler!(chat_read_outbox, UpdateChatReadOutbox)(self.api, self.lout, json),
      
      "updateChatUnreadMentionCount" => event_handler!(chat_unread_mention_count, UpdateChatUnreadMentionCount)(self.api, self.lout, json),
      
      "updateChatNotificationSettings" => event_handler!(chat_notification_settings, UpdateChatNotificationSettings)(self.api, self.lout, json),
      
      "updateScopeNotificationSettings" => event_handler!(scope_notification_settings, UpdateScopeNotificationSettings)(self.api, self.lout, json),
      
      "updateChatPinnedMessage" => event_handler!(chat_pinned_message, UpdateChatPinnedMessage)(self.api, self.lout, json),
      
      "updateChatReplyMarkup" => event_handler!(chat_reply_markup, UpdateChatReplyMarkup)(self.api, self.lout, json),
      
      "updateChatDraftMessage" => event_handler!(chat_draft_message, UpdateChatDraftMessage)(self.api, self.lout, json),
      
      "updateChatOnlineMemberCount" => event_handler!(chat_online_member_count, UpdateChatOnlineMemberCount)(self.api, self.lout, json),
      
      "updateNotification" => event_handler!(notification, UpdateNotification)(self.api, self.lout, json),
      
      "updateNotificationGroup" => event_handler!(notification_group, UpdateNotificationGroup)(self.api, self.lout, json),
      
      "updateActiveNotifications" => event_handler!(active_notifications, UpdateActiveNotifications)(self.api, self.lout, json),
      
      "updateHavePendingNotifications" => event_handler!(have_pending_notifications, UpdateHavePendingNotifications)(self.api, self.lout, json),
      
      "updateDeleteMessages" => event_handler!(delete_messages, UpdateDeleteMessages)(self.api, self.lout, json),
      
      "updateUserChatAction" => event_handler!(user_chat_action, UpdateUserChatAction)(self.api, self.lout, json),
      
      "updateUserStatus" => event_handler!(user_status, UpdateUserStatus)(self.api, self.lout, json),
      
      "updateUser" => event_handler!(user, UpdateUser)(self.api, self.lout, json),
      
      "updateBasicGroup" => event_handler!(basic_group, UpdateBasicGroup)(self.api, self.lout, json),
      
      "updateSupergroup" => event_handler!(supergroup, UpdateSupergroup)(self.api, self.lout, json),
      
      "updateSecretChat" => event_handler!(secret_chat, UpdateSecretChat)(self.api, self.lout, json),
      
      "updateUserFullInfo" => event_handler!(user_full_info, UpdateUserFullInfo)(self.api, self.lout, json),
      
      "updateBasicGroupFullInfo" => event_handler!(basic_group_full_info, UpdateBasicGroupFullInfo)(self.api, self.lout, json),
      
      "updateSupergroupFullInfo" => event_handler!(supergroup_full_info, UpdateSupergroupFullInfo)(self.api, self.lout, json),
      
      "updateServiceNotification" => event_handler!(service_notification, UpdateServiceNotification)(self.api, self.lout, json),
      
      "updateFile" => event_handler!(file, UpdateFile)(self.api, self.lout, json),
      
      "updateFileGenerationStart" => event_handler!(file_generation_start, UpdateFileGenerationStart)(self.api, self.lout, json),
      
      "updateFileGenerationStop" => event_handler!(file_generation_stop, UpdateFileGenerationStop)(self.api, self.lout, json),
      
      "updateCall" => event_handler!(call, UpdateCall)(self.api, self.lout, json),
      
      "updateUserPrivacySettingRules" => event_handler!(user_privacy_setting_rules, UpdateUserPrivacySettingRules)(self.api, self.lout, json),
      
      "updateUnreadMessageCount" => event_handler!(unread_message_count, UpdateUnreadMessageCount)(self.api, self.lout, json),
      
      "updateUnreadChatCount" => event_handler!(unread_chat_count, UpdateUnreadChatCount)(self.api, self.lout, json),
      
      "updateOption" => event_handler!(option, UpdateOption)(self.api, self.lout, json),
      
      "updateInstalledStickerSets" => event_handler!(installed_sticker_sets, UpdateInstalledStickerSets)(self.api, self.lout, json),
      
      "updateTrendingStickerSets" => event_handler!(trending_sticker_sets, UpdateTrendingStickerSets)(self.api, self.lout, json),
      
      "updateRecentStickers" => event_handler!(recent_stickers, UpdateRecentStickers)(self.api, self.lout, json),
      
      "updateFavoriteStickers" => event_handler!(favorite_stickers, UpdateFavoriteStickers)(self.api, self.lout, json),
      
      "updateSavedAnimations" => event_handler!(saved_animations, UpdateSavedAnimations)(self.api, self.lout, json),
      
      "updateLanguagePackStrings" => event_handler!(language_pack_strings, UpdateLanguagePackStrings)(self.api, self.lout, json),
      
      "updateConnectionState" => event_handler!(connection_state, UpdateConnectionState)(self.api, self.lout, json),
      
      "updateTermsOfService" => event_handler!(terms_of_service, UpdateTermsOfService)(self.api, self.lout, json),
      
      "updateNewInlineQuery" => event_handler!(new_inline_query, UpdateNewInlineQuery)(self.api, self.lout, json),
      
      "updateNewChosenInlineResult" => event_handler!(new_chosen_inline_result, UpdateNewChosenInlineResult)(self.api, self.lout, json),
      
      "updateNewCallbackQuery" => event_handler!(new_callback_query, UpdateNewCallbackQuery)(self.api, self.lout, json),
      
      "updateNewInlineCallbackQuery" => event_handler!(new_inline_callback_query, UpdateNewInlineCallbackQuery)(self.api, self.lout, json),
      
      "updateNewShippingQuery" => event_handler!(new_shipping_query, UpdateNewShippingQuery)(self.api, self.lout, json),
      
      "updateNewPreCheckoutQuery" => event_handler!(new_pre_checkout_query, UpdateNewPreCheckoutQuery)(self.api, self.lout, json),
      
      "updateNewCustomEvent" => event_handler!(new_custom_event, UpdateNewCustomEvent)(self.api, self.lout, json),
      
      "updateNewCustomQuery" => event_handler!(new_custom_query, UpdateNewCustomQuery)(self.api, self.lout, json),
      
      "updatePoll" => event_handler!(poll, UpdatePoll)(self.api, self.lout, json),
      
      "testUseUpdate" => event_handler!(test_use_update, TestUseUpdate)(self.api, self.lout, json),
      
      
      "error" => event_handler!(error, Error)(self.api, self.lout, json),
      
      "ok" => event_handler!(ok, Ok)(self.api, self.lout, json),
      
      "proxy" => event_handler!(proxy, Proxy)(self.api, self.lout, json),
      
      _ => {
        warn!("{}", tip::data_fail_with_json(json))
      }
    }
  }

}


