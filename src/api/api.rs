use core::borrow::Borrow;
use std::sync::Arc;

use rtdlib::tdjson;
use rtdlib::types::Function;

use crate::api::*;
use crate::tglog;

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

  pub fn send<Fnc: Function>(&self, fnc: Fnc) -> &Self {
    let json = fnc.to_json();
    info!(tglog::telegram(), "===> {}", json);
    self.tdlib.send(&json[..]);
    self
  }

  pub fn receive(&self, timeout: f64) -> Option<String> {
    let receive = self.tdlib.receive(timeout)
      .map(|v| rtdlib::tdkit::fill_json_struct(v));
    if receive.is_some() {
      info!(tglog::telegram(), "<=== {}", receive.clone().unwrap());
    }
    receive
  }

  pub fn execute<Fnc: Function>(&self, fnc: Fnc) -> Option<String> {
    let json = fnc.to_json();
    info!(tglog::telegram(), "===>>> {}", json);
    self.tdlib.execute(&json[..])
      .map(|v| rtdlib::tdkit::fill_json_struct(v))
  }

  
  pub fn accept_call<C: AsRef<TGAcceptCall>>(&self, accept_call: C) -> &Self {
    self.send(accept_call.as_ref().build())
  }
  
  pub fn accept_terms_of_service<C: AsRef<TGAcceptTermsOfService>>(&self, accept_terms_of_service: C) -> &Self {
    self.send(accept_terms_of_service.as_ref().build())
  }
  
  pub fn add_chat_member<C: AsRef<TGAddChatMember>>(&self, add_chat_member: C) -> &Self {
    self.send(add_chat_member.as_ref().build())
  }
  
  pub fn add_chat_members<C: AsRef<TGAddChatMembers>>(&self, add_chat_members: C) -> &Self {
    self.send(add_chat_members.as_ref().build())
  }
  
  pub fn add_custom_server_language_pack<C: AsRef<TGAddCustomServerLanguagePack>>(&self, add_custom_server_language_pack: C) -> &Self {
    self.send(add_custom_server_language_pack.as_ref().build())
  }
  
  pub fn add_favorite_sticker<C: AsRef<TGAddFavoriteSticker>>(&self, add_favorite_sticker: C) -> &Self {
    self.send(add_favorite_sticker.as_ref().build())
  }
  
  pub fn add_local_message<C: AsRef<TGAddLocalMessage>>(&self, add_local_message: C) -> &Self {
    self.send(add_local_message.as_ref().build())
  }
  
  pub fn add_log_message<C: AsRef<TGAddLogMessage>>(&self, add_log_message: C) -> &Self {
    self.send(add_log_message.as_ref().build())
  }
  
  pub fn add_network_statistics<C: AsRef<TGAddNetworkStatistics>>(&self, add_network_statistics: C) -> &Self {
    self.send(add_network_statistics.as_ref().build())
  }
  
  pub fn add_proxy<C: AsRef<TGAddProxy>>(&self, add_proxy: C) -> &Self {
    self.send(add_proxy.as_ref().build())
  }
  
  pub fn add_recently_found_chat<C: AsRef<TGAddRecentlyFoundChat>>(&self, add_recently_found_chat: C) -> &Self {
    self.send(add_recently_found_chat.as_ref().build())
  }
  
  pub fn add_recent_sticker<C: AsRef<TGAddRecentSticker>>(&self, add_recent_sticker: C) -> &Self {
    self.send(add_recent_sticker.as_ref().build())
  }
  
  pub fn add_saved_animation<C: AsRef<TGAddSavedAnimation>>(&self, add_saved_animation: C) -> &Self {
    self.send(add_saved_animation.as_ref().build())
  }
  
  pub fn add_sticker_to_set<C: AsRef<TGAddStickerToSet>>(&self, add_sticker_to_set: C) -> &Self {
    self.send(add_sticker_to_set.as_ref().build())
  }
  
  pub fn answer_callback_query<C: AsRef<TGAnswerCallbackQuery>>(&self, answer_callback_query: C) -> &Self {
    self.send(answer_callback_query.as_ref().build())
  }
  
  pub fn answer_custom_query<C: AsRef<TGAnswerCustomQuery>>(&self, answer_custom_query: C) -> &Self {
    self.send(answer_custom_query.as_ref().build())
  }
  
  pub fn answer_inline_query<C: AsRef<TGAnswerInlineQuery>>(&self, answer_inline_query: C) -> &Self {
    self.send(answer_inline_query.as_ref().build())
  }
  
  pub fn answer_pre_checkout_query<C: AsRef<TGAnswerPreCheckoutQuery>>(&self, answer_pre_checkout_query: C) -> &Self {
    self.send(answer_pre_checkout_query.as_ref().build())
  }
  
  pub fn answer_shipping_query<C: AsRef<TGAnswerShippingQuery>>(&self, answer_shipping_query: C) -> &Self {
    self.send(answer_shipping_query.as_ref().build())
  }
  
  pub fn block_user<C: AsRef<TGBlockUser>>(&self, block_user: C) -> &Self {
    self.send(block_user.as_ref().build())
  }
  
  pub fn cancel_download_file<C: AsRef<TGCancelDownloadFile>>(&self, cancel_download_file: C) -> &Self {
    self.send(cancel_download_file.as_ref().build())
  }
  
  pub fn cancel_upload_file<C: AsRef<TGCancelUploadFile>>(&self, cancel_upload_file: C) -> &Self {
    self.send(cancel_upload_file.as_ref().build())
  }
  
  pub fn change_chat_report_spam_state<C: AsRef<TGChangeChatReportSpamState>>(&self, change_chat_report_spam_state: C) -> &Self {
    self.send(change_chat_report_spam_state.as_ref().build())
  }
  
  pub fn change_imported_contacts<C: AsRef<TGChangeImportedContacts>>(&self, change_imported_contacts: C) -> &Self {
    self.send(change_imported_contacts.as_ref().build())
  }
  
  pub fn change_phone_number<C: AsRef<TGChangePhoneNumber>>(&self, change_phone_number: C) -> &Self {
    self.send(change_phone_number.as_ref().build())
  }
  
  pub fn change_sticker_set<C: AsRef<TGChangeStickerSet>>(&self, change_sticker_set: C) -> &Self {
    self.send(change_sticker_set.as_ref().build())
  }
  
  pub fn check_authentication_bot_token<C: AsRef<TGCheckAuthenticationBotToken>>(&self, check_authentication_bot_token: C) -> &Self {
    self.send(check_authentication_bot_token.as_ref().build())
  }
  
  pub fn check_authentication_code<C: AsRef<TGCheckAuthenticationCode>>(&self, check_authentication_code: C) -> &Self {
    self.send(check_authentication_code.as_ref().build())
  }
  
  pub fn check_authentication_password<C: AsRef<TGCheckAuthenticationPassword>>(&self, check_authentication_password: C) -> &Self {
    self.send(check_authentication_password.as_ref().build())
  }
  
  pub fn check_change_phone_number_code<C: AsRef<TGCheckChangePhoneNumberCode>>(&self, check_change_phone_number_code: C) -> &Self {
    self.send(check_change_phone_number_code.as_ref().build())
  }
  
  pub fn check_chat_invite_link<C: AsRef<TGCheckChatInviteLink>>(&self, check_chat_invite_link: C) -> &Self {
    self.send(check_chat_invite_link.as_ref().build())
  }
  
  pub fn check_chat_username<C: AsRef<TGCheckChatUsername>>(&self, check_chat_username: C) -> &Self {
    self.send(check_chat_username.as_ref().build())
  }
  
  pub fn check_database_encryption_key<C: AsRef<TGCheckDatabaseEncryptionKey>>(&self, check_database_encryption_key: C) -> &Self {
    self.send(check_database_encryption_key.as_ref().build())
  }
  
  pub fn check_email_address_verification_code<C: AsRef<TGCheckEmailAddressVerificationCode>>(&self, check_email_address_verification_code: C) -> &Self {
    self.send(check_email_address_verification_code.as_ref().build())
  }
  
  pub fn check_phone_number_confirmation_code<C: AsRef<TGCheckPhoneNumberConfirmationCode>>(&self, check_phone_number_confirmation_code: C) -> &Self {
    self.send(check_phone_number_confirmation_code.as_ref().build())
  }
  
  pub fn check_phone_number_verification_code<C: AsRef<TGCheckPhoneNumberVerificationCode>>(&self, check_phone_number_verification_code: C) -> &Self {
    self.send(check_phone_number_verification_code.as_ref().build())
  }
  
  pub fn check_recovery_email_address_code<C: AsRef<TGCheckRecoveryEmailAddressCode>>(&self, check_recovery_email_address_code: C) -> &Self {
    self.send(check_recovery_email_address_code.as_ref().build())
  }
  
  pub fn clean_file_name<C: AsRef<TGCleanFileName>>(&self, clean_file_name: C) -> &Self {
    self.send(clean_file_name.as_ref().build())
  }
  
  pub fn clear_all_draft_messages<C: AsRef<TGClearAllDraftMessages>>(&self, clear_all_draft_messages: C) -> &Self {
    self.send(clear_all_draft_messages.as_ref().build())
  }
  
  pub fn clear_imported_contacts<C: AsRef<TGClearImportedContacts>>(&self, clear_imported_contacts: C) -> &Self {
    self.send(clear_imported_contacts.as_ref().build())
  }
  
  pub fn clear_recently_found_chats<C: AsRef<TGClearRecentlyFoundChats>>(&self, clear_recently_found_chats: C) -> &Self {
    self.send(clear_recently_found_chats.as_ref().build())
  }
  
  pub fn clear_recent_stickers<C: AsRef<TGClearRecentStickers>>(&self, clear_recent_stickers: C) -> &Self {
    self.send(clear_recent_stickers.as_ref().build())
  }
  
  pub fn close<C: AsRef<TGClose>>(&self, close: C) -> &Self {
    self.send(close.as_ref().build())
  }
  
  pub fn close_chat<C: AsRef<TGCloseChat>>(&self, close_chat: C) -> &Self {
    self.send(close_chat.as_ref().build())
  }
  
  pub fn close_secret_chat<C: AsRef<TGCloseSecretChat>>(&self, close_secret_chat: C) -> &Self {
    self.send(close_secret_chat.as_ref().build())
  }
  
  pub fn create_basic_group_chat<C: AsRef<TGCreateBasicGroupChat>>(&self, create_basic_group_chat: C) -> &Self {
    self.send(create_basic_group_chat.as_ref().build())
  }
  
  pub fn create_call<C: AsRef<TGCreateCall>>(&self, create_call: C) -> &Self {
    self.send(create_call.as_ref().build())
  }
  
  pub fn create_new_basic_group_chat<C: AsRef<TGCreateNewBasicGroupChat>>(&self, create_new_basic_group_chat: C) -> &Self {
    self.send(create_new_basic_group_chat.as_ref().build())
  }
  
  pub fn create_new_secret_chat<C: AsRef<TGCreateNewSecretChat>>(&self, create_new_secret_chat: C) -> &Self {
    self.send(create_new_secret_chat.as_ref().build())
  }
  
  pub fn create_new_sticker_set<C: AsRef<TGCreateNewStickerSet>>(&self, create_new_sticker_set: C) -> &Self {
    self.send(create_new_sticker_set.as_ref().build())
  }
  
  pub fn create_new_supergroup_chat<C: AsRef<TGCreateNewSupergroupChat>>(&self, create_new_supergroup_chat: C) -> &Self {
    self.send(create_new_supergroup_chat.as_ref().build())
  }
  
  pub fn create_private_chat<C: AsRef<TGCreatePrivateChat>>(&self, create_private_chat: C) -> &Self {
    self.send(create_private_chat.as_ref().build())
  }
  
  pub fn create_secret_chat<C: AsRef<TGCreateSecretChat>>(&self, create_secret_chat: C) -> &Self {
    self.send(create_secret_chat.as_ref().build())
  }
  
  pub fn create_supergroup_chat<C: AsRef<TGCreateSupergroupChat>>(&self, create_supergroup_chat: C) -> &Self {
    self.send(create_supergroup_chat.as_ref().build())
  }
  
  pub fn create_temporary_password<C: AsRef<TGCreateTemporaryPassword>>(&self, create_temporary_password: C) -> &Self {
    self.send(create_temporary_password.as_ref().build())
  }
  
  pub fn delete_account<C: AsRef<TGDeleteAccount>>(&self, delete_account: C) -> &Self {
    self.send(delete_account.as_ref().build())
  }
  
  pub fn delete_chat_history<C: AsRef<TGDeleteChatHistory>>(&self, delete_chat_history: C) -> &Self {
    self.send(delete_chat_history.as_ref().build())
  }
  
  pub fn delete_chat_messages_from_user<C: AsRef<TGDeleteChatMessagesFromUser>>(&self, delete_chat_messages_from_user: C) -> &Self {
    self.send(delete_chat_messages_from_user.as_ref().build())
  }
  
  pub fn delete_chat_reply_markup<C: AsRef<TGDeleteChatReplyMarkup>>(&self, delete_chat_reply_markup: C) -> &Self {
    self.send(delete_chat_reply_markup.as_ref().build())
  }
  
  pub fn delete_file<C: AsRef<TGDeleteFile>>(&self, delete_file: C) -> &Self {
    self.send(delete_file.as_ref().build())
  }
  
  pub fn delete_language_pack<C: AsRef<TGDeleteLanguagePack>>(&self, delete_language_pack: C) -> &Self {
    self.send(delete_language_pack.as_ref().build())
  }
  
  pub fn delete_messages<C: AsRef<TGDeleteMessages>>(&self, delete_messages: C) -> &Self {
    self.send(delete_messages.as_ref().build())
  }
  
  pub fn delete_passport_element<C: AsRef<TGDeletePassportElement>>(&self, delete_passport_element: C) -> &Self {
    self.send(delete_passport_element.as_ref().build())
  }
  
  pub fn delete_profile_photo<C: AsRef<TGDeleteProfilePhoto>>(&self, delete_profile_photo: C) -> &Self {
    self.send(delete_profile_photo.as_ref().build())
  }
  
  pub fn delete_saved_credentials<C: AsRef<TGDeleteSavedCredentials>>(&self, delete_saved_credentials: C) -> &Self {
    self.send(delete_saved_credentials.as_ref().build())
  }
  
  pub fn delete_saved_order_info<C: AsRef<TGDeleteSavedOrderInfo>>(&self, delete_saved_order_info: C) -> &Self {
    self.send(delete_saved_order_info.as_ref().build())
  }
  
  pub fn delete_supergroup<C: AsRef<TGDeleteSupergroup>>(&self, delete_supergroup: C) -> &Self {
    self.send(delete_supergroup.as_ref().build())
  }
  
  pub fn destroy<C: AsRef<TGDestroy>>(&self, destroy: C) -> &Self {
    self.send(destroy.as_ref().build())
  }
  
  pub fn disable_proxy<C: AsRef<TGDisableProxy>>(&self, disable_proxy: C) -> &Self {
    self.send(disable_proxy.as_ref().build())
  }
  
  pub fn discard_call<C: AsRef<TGDiscardCall>>(&self, discard_call: C) -> &Self {
    self.send(discard_call.as_ref().build())
  }
  
  pub fn disconnect_all_websites<C: AsRef<TGDisconnectAllWebsites>>(&self, disconnect_all_websites: C) -> &Self {
    self.send(disconnect_all_websites.as_ref().build())
  }
  
  pub fn disconnect_website<C: AsRef<TGDisconnectWebsite>>(&self, disconnect_website: C) -> &Self {
    self.send(disconnect_website.as_ref().build())
  }
  
  pub fn download_file<C: AsRef<TGDownloadFile>>(&self, download_file: C) -> &Self {
    self.send(download_file.as_ref().build())
  }
  
  pub fn edit_custom_language_pack_info<C: AsRef<TGEditCustomLanguagePackInfo>>(&self, edit_custom_language_pack_info: C) -> &Self {
    self.send(edit_custom_language_pack_info.as_ref().build())
  }
  
  pub fn edit_inline_message_caption<C: AsRef<TGEditInlineMessageCaption>>(&self, edit_inline_message_caption: C) -> &Self {
    self.send(edit_inline_message_caption.as_ref().build())
  }
  
  pub fn edit_inline_message_live_location<C: AsRef<TGEditInlineMessageLiveLocation>>(&self, edit_inline_message_live_location: C) -> &Self {
    self.send(edit_inline_message_live_location.as_ref().build())
  }
  
  pub fn edit_inline_message_media<C: AsRef<TGEditInlineMessageMedia>>(&self, edit_inline_message_media: C) -> &Self {
    self.send(edit_inline_message_media.as_ref().build())
  }
  
  pub fn edit_inline_message_reply_markup<C: AsRef<TGEditInlineMessageReplyMarkup>>(&self, edit_inline_message_reply_markup: C) -> &Self {
    self.send(edit_inline_message_reply_markup.as_ref().build())
  }
  
  pub fn edit_inline_message_text<C: AsRef<TGEditInlineMessageText>>(&self, edit_inline_message_text: C) -> &Self {
    self.send(edit_inline_message_text.as_ref().build())
  }
  
  pub fn edit_message_caption<C: AsRef<TGEditMessageCaption>>(&self, edit_message_caption: C) -> &Self {
    self.send(edit_message_caption.as_ref().build())
  }
  
  pub fn edit_message_live_location<C: AsRef<TGEditMessageLiveLocation>>(&self, edit_message_live_location: C) -> &Self {
    self.send(edit_message_live_location.as_ref().build())
  }
  
  pub fn edit_message_media<C: AsRef<TGEditMessageMedia>>(&self, edit_message_media: C) -> &Self {
    self.send(edit_message_media.as_ref().build())
  }
  
  pub fn edit_message_reply_markup<C: AsRef<TGEditMessageReplyMarkup>>(&self, edit_message_reply_markup: C) -> &Self {
    self.send(edit_message_reply_markup.as_ref().build())
  }
  
  pub fn edit_message_text<C: AsRef<TGEditMessageText>>(&self, edit_message_text: C) -> &Self {
    self.send(edit_message_text.as_ref().build())
  }
  
  pub fn edit_proxy<C: AsRef<TGEditProxy>>(&self, edit_proxy: C) -> &Self {
    self.send(edit_proxy.as_ref().build())
  }
  
  pub fn enable_proxy<C: AsRef<TGEnableProxy>>(&self, enable_proxy: C) -> &Self {
    self.send(enable_proxy.as_ref().build())
  }
  
  pub fn finish_file_generation<C: AsRef<TGFinishFileGeneration>>(&self, finish_file_generation: C) -> &Self {
    self.send(finish_file_generation.as_ref().build())
  }
  
  pub fn forward_messages<C: AsRef<TGForwardMessages>>(&self, forward_messages: C) -> &Self {
    self.send(forward_messages.as_ref().build())
  }
  
  pub fn generate_chat_invite_link<C: AsRef<TGGenerateChatInviteLink>>(&self, generate_chat_invite_link: C) -> &Self {
    self.send(generate_chat_invite_link.as_ref().build())
  }
  
  pub fn get_account_ttl<C: AsRef<TGGetAccountTtl>>(&self, get_account_ttl: C) -> &Self {
    self.send(get_account_ttl.as_ref().build())
  }
  
  pub fn get_active_live_location_messages<C: AsRef<TGGetActiveLiveLocationMessages>>(&self, get_active_live_location_messages: C) -> &Self {
    self.send(get_active_live_location_messages.as_ref().build())
  }
  
  pub fn get_active_sessions<C: AsRef<TGGetActiveSessions>>(&self, get_active_sessions: C) -> &Self {
    self.send(get_active_sessions.as_ref().build())
  }
  
  pub fn get_all_passport_elements<C: AsRef<TGGetAllPassportElements>>(&self, get_all_passport_elements: C) -> &Self {
    self.send(get_all_passport_elements.as_ref().build())
  }
  
  pub fn get_application_config<C: AsRef<TGGetApplicationConfig>>(&self, get_application_config: C) -> &Self {
    self.send(get_application_config.as_ref().build())
  }
  
  pub fn get_archived_sticker_sets<C: AsRef<TGGetArchivedStickerSets>>(&self, get_archived_sticker_sets: C) -> &Self {
    self.send(get_archived_sticker_sets.as_ref().build())
  }
  
  pub fn get_attached_sticker_sets<C: AsRef<TGGetAttachedStickerSets>>(&self, get_attached_sticker_sets: C) -> &Self {
    self.send(get_attached_sticker_sets.as_ref().build())
  }
  
  pub fn get_authorization_state<C: AsRef<TGGetAuthorizationState>>(&self, get_authorization_state: C) -> &Self {
    self.send(get_authorization_state.as_ref().build())
  }
  
  pub fn get_basic_group<C: AsRef<TGGetBasicGroup>>(&self, get_basic_group: C) -> &Self {
    self.send(get_basic_group.as_ref().build())
  }
  
  pub fn get_basic_group_full_info<C: AsRef<TGGetBasicGroupFullInfo>>(&self, get_basic_group_full_info: C) -> &Self {
    self.send(get_basic_group_full_info.as_ref().build())
  }
  
  pub fn get_blocked_users<C: AsRef<TGGetBlockedUsers>>(&self, get_blocked_users: C) -> &Self {
    self.send(get_blocked_users.as_ref().build())
  }
  
  pub fn get_callback_query_answer<C: AsRef<TGGetCallbackQueryAnswer>>(&self, get_callback_query_answer: C) -> &Self {
    self.send(get_callback_query_answer.as_ref().build())
  }
  
  pub fn get_chat<C: AsRef<TGGetChat>>(&self, get_chat: C) -> &Self {
    self.send(get_chat.as_ref().build())
  }
  
  pub fn get_chat_administrators<C: AsRef<TGGetChatAdministrators>>(&self, get_chat_administrators: C) -> &Self {
    self.send(get_chat_administrators.as_ref().build())
  }
  
  pub fn get_chat_event_log<C: AsRef<TGGetChatEventLog>>(&self, get_chat_event_log: C) -> &Self {
    self.send(get_chat_event_log.as_ref().build())
  }
  
  pub fn get_chat_history<C: AsRef<TGGetChatHistory>>(&self, get_chat_history: C) -> &Self {
    self.send(get_chat_history.as_ref().build())
  }
  
  pub fn get_chat_member<C: AsRef<TGGetChatMember>>(&self, get_chat_member: C) -> &Self {
    self.send(get_chat_member.as_ref().build())
  }
  
  pub fn get_chat_message_by_date<C: AsRef<TGGetChatMessageByDate>>(&self, get_chat_message_by_date: C) -> &Self {
    self.send(get_chat_message_by_date.as_ref().build())
  }
  
  pub fn get_chat_message_count<C: AsRef<TGGetChatMessageCount>>(&self, get_chat_message_count: C) -> &Self {
    self.send(get_chat_message_count.as_ref().build())
  }
  
  pub fn get_chat_notification_settings_exceptions<C: AsRef<TGGetChatNotificationSettingsExceptions>>(&self, get_chat_notification_settings_exceptions: C) -> &Self {
    self.send(get_chat_notification_settings_exceptions.as_ref().build())
  }
  
  pub fn get_chat_pinned_message<C: AsRef<TGGetChatPinnedMessage>>(&self, get_chat_pinned_message: C) -> &Self {
    self.send(get_chat_pinned_message.as_ref().build())
  }
  
  pub fn get_chat_report_spam_state<C: AsRef<TGGetChatReportSpamState>>(&self, get_chat_report_spam_state: C) -> &Self {
    self.send(get_chat_report_spam_state.as_ref().build())
  }
  
  pub fn get_chats<C: AsRef<TGGetChats>>(&self, get_chats: C) -> &Self {
    self.send(get_chats.as_ref().build())
  }
  
  pub fn get_chat_statistics_url<C: AsRef<TGGetChatStatisticsUrl>>(&self, get_chat_statistics_url: C) -> &Self {
    self.send(get_chat_statistics_url.as_ref().build())
  }
  
  pub fn get_connected_websites<C: AsRef<TGGetConnectedWebsites>>(&self, get_connected_websites: C) -> &Self {
    self.send(get_connected_websites.as_ref().build())
  }
  
  pub fn get_contacts<C: AsRef<TGGetContacts>>(&self, get_contacts: C) -> &Self {
    self.send(get_contacts.as_ref().build())
  }
  
  pub fn get_country_code<C: AsRef<TGGetCountryCode>>(&self, get_country_code: C) -> &Self {
    self.send(get_country_code.as_ref().build())
  }
  
  pub fn get_created_public_chats<C: AsRef<TGGetCreatedPublicChats>>(&self, get_created_public_chats: C) -> &Self {
    self.send(get_created_public_chats.as_ref().build())
  }
  
  pub fn get_current_state<C: AsRef<TGGetCurrentState>>(&self, get_current_state: C) -> &Self {
    self.send(get_current_state.as_ref().build())
  }
  
  pub fn get_database_statistics<C: AsRef<TGGetDatabaseStatistics>>(&self, get_database_statistics: C) -> &Self {
    self.send(get_database_statistics.as_ref().build())
  }
  
  pub fn get_deep_link_info<C: AsRef<TGGetDeepLinkInfo>>(&self, get_deep_link_info: C) -> &Self {
    self.send(get_deep_link_info.as_ref().build())
  }
  
  pub fn get_favorite_stickers<C: AsRef<TGGetFavoriteStickers>>(&self, get_favorite_stickers: C) -> &Self {
    self.send(get_favorite_stickers.as_ref().build())
  }
  
  pub fn get_file<C: AsRef<TGGetFile>>(&self, get_file: C) -> &Self {
    self.send(get_file.as_ref().build())
  }
  
  pub fn get_file_downloaded_prefix_size<C: AsRef<TGGetFileDownloadedPrefixSize>>(&self, get_file_downloaded_prefix_size: C) -> &Self {
    self.send(get_file_downloaded_prefix_size.as_ref().build())
  }
  
  pub fn get_file_extension<C: AsRef<TGGetFileExtension>>(&self, get_file_extension: C) -> &Self {
    self.send(get_file_extension.as_ref().build())
  }
  
  pub fn get_file_mime_type<C: AsRef<TGGetFileMimeType>>(&self, get_file_mime_type: C) -> &Self {
    self.send(get_file_mime_type.as_ref().build())
  }
  
  pub fn get_game_high_scores<C: AsRef<TGGetGameHighScores>>(&self, get_game_high_scores: C) -> &Self {
    self.send(get_game_high_scores.as_ref().build())
  }
  
  pub fn get_groups_in_common<C: AsRef<TGGetGroupsInCommon>>(&self, get_groups_in_common: C) -> &Self {
    self.send(get_groups_in_common.as_ref().build())
  }
  
  pub fn get_imported_contact_count<C: AsRef<TGGetImportedContactCount>>(&self, get_imported_contact_count: C) -> &Self {
    self.send(get_imported_contact_count.as_ref().build())
  }
  
  pub fn get_inline_game_high_scores<C: AsRef<TGGetInlineGameHighScores>>(&self, get_inline_game_high_scores: C) -> &Self {
    self.send(get_inline_game_high_scores.as_ref().build())
  }
  
  pub fn get_inline_query_results<C: AsRef<TGGetInlineQueryResults>>(&self, get_inline_query_results: C) -> &Self {
    self.send(get_inline_query_results.as_ref().build())
  }
  
  pub fn get_installed_sticker_sets<C: AsRef<TGGetInstalledStickerSets>>(&self, get_installed_sticker_sets: C) -> &Self {
    self.send(get_installed_sticker_sets.as_ref().build())
  }
  
  pub fn get_invite_text<C: AsRef<TGGetInviteText>>(&self, get_invite_text: C) -> &Self {
    self.send(get_invite_text.as_ref().build())
  }
  
  pub fn get_language_pack_info<C: AsRef<TGGetLanguagePackInfo>>(&self, get_language_pack_info: C) -> &Self {
    self.send(get_language_pack_info.as_ref().build())
  }
  
  pub fn get_language_pack_string<C: AsRef<TGGetLanguagePackString>>(&self, get_language_pack_string: C) -> &Self {
    self.send(get_language_pack_string.as_ref().build())
  }
  
  pub fn get_language_pack_strings<C: AsRef<TGGetLanguagePackStrings>>(&self, get_language_pack_strings: C) -> &Self {
    self.send(get_language_pack_strings.as_ref().build())
  }
  
  pub fn get_localization_target_info<C: AsRef<TGGetLocalizationTargetInfo>>(&self, get_localization_target_info: C) -> &Self {
    self.send(get_localization_target_info.as_ref().build())
  }
  
  pub fn get_log_stream<C: AsRef<TGGetLogStream>>(&self, get_log_stream: C) -> &Self {
    self.send(get_log_stream.as_ref().build())
  }
  
  pub fn get_log_tags<C: AsRef<TGGetLogTags>>(&self, get_log_tags: C) -> &Self {
    self.send(get_log_tags.as_ref().build())
  }
  
  pub fn get_log_tag_verbosity_level<C: AsRef<TGGetLogTagVerbosityLevel>>(&self, get_log_tag_verbosity_level: C) -> &Self {
    self.send(get_log_tag_verbosity_level.as_ref().build())
  }
  
  pub fn get_log_verbosity_level<C: AsRef<TGGetLogVerbosityLevel>>(&self, get_log_verbosity_level: C) -> &Self {
    self.send(get_log_verbosity_level.as_ref().build())
  }
  
  pub fn get_map_thumbnail_file<C: AsRef<TGGetMapThumbnailFile>>(&self, get_map_thumbnail_file: C) -> &Self {
    self.send(get_map_thumbnail_file.as_ref().build())
  }
  
  pub fn get_me<C: AsRef<TGGetMe>>(&self, get_me: C) -> &Self {
    self.send(get_me.as_ref().build())
  }
  
  pub fn get_message<C: AsRef<TGGetMessage>>(&self, get_message: C) -> &Self {
    self.send(get_message.as_ref().build())
  }
  
  pub fn get_message_link<C: AsRef<TGGetMessageLink>>(&self, get_message_link: C) -> &Self {
    self.send(get_message_link.as_ref().build())
  }
  
  pub fn get_message_locally<C: AsRef<TGGetMessageLocally>>(&self, get_message_locally: C) -> &Self {
    self.send(get_message_locally.as_ref().build())
  }
  
  pub fn get_messages<C: AsRef<TGGetMessages>>(&self, get_messages: C) -> &Self {
    self.send(get_messages.as_ref().build())
  }
  
  pub fn get_network_statistics<C: AsRef<TGGetNetworkStatistics>>(&self, get_network_statistics: C) -> &Self {
    self.send(get_network_statistics.as_ref().build())
  }
  
  pub fn get_option<C: AsRef<TGGetOption>>(&self, get_option: C) -> &Self {
    self.send(get_option.as_ref().build())
  }
  
  pub fn get_passport_authorization_form<C: AsRef<TGGetPassportAuthorizationForm>>(&self, get_passport_authorization_form: C) -> &Self {
    self.send(get_passport_authorization_form.as_ref().build())
  }
  
  pub fn get_passport_authorization_form_available_elements<C: AsRef<TGGetPassportAuthorizationFormAvailableElements>>(&self, get_passport_authorization_form_available_elements: C) -> &Self {
    self.send(get_passport_authorization_form_available_elements.as_ref().build())
  }
  
  pub fn get_passport_element<C: AsRef<TGGetPassportElement>>(&self, get_passport_element: C) -> &Self {
    self.send(get_passport_element.as_ref().build())
  }
  
  pub fn get_password_state<C: AsRef<TGGetPasswordState>>(&self, get_password_state: C) -> &Self {
    self.send(get_password_state.as_ref().build())
  }
  
  pub fn get_payment_form<C: AsRef<TGGetPaymentForm>>(&self, get_payment_form: C) -> &Self {
    self.send(get_payment_form.as_ref().build())
  }
  
  pub fn get_payment_receipt<C: AsRef<TGGetPaymentReceipt>>(&self, get_payment_receipt: C) -> &Self {
    self.send(get_payment_receipt.as_ref().build())
  }
  
  pub fn get_preferred_country_language<C: AsRef<TGGetPreferredCountryLanguage>>(&self, get_preferred_country_language: C) -> &Self {
    self.send(get_preferred_country_language.as_ref().build())
  }
  
  pub fn get_proxies<C: AsRef<TGGetProxies>>(&self, get_proxies: C) -> &Self {
    self.send(get_proxies.as_ref().build())
  }
  
  pub fn get_proxy_link<C: AsRef<TGGetProxyLink>>(&self, get_proxy_link: C) -> &Self {
    self.send(get_proxy_link.as_ref().build())
  }
  
  pub fn get_public_message_link<C: AsRef<TGGetPublicMessageLink>>(&self, get_public_message_link: C) -> &Self {
    self.send(get_public_message_link.as_ref().build())
  }
  
  pub fn get_push_receiver_id<C: AsRef<TGGetPushReceiverId>>(&self, get_push_receiver_id: C) -> &Self {
    self.send(get_push_receiver_id.as_ref().build())
  }
  
  pub fn get_recent_inline_bots<C: AsRef<TGGetRecentInlineBots>>(&self, get_recent_inline_bots: C) -> &Self {
    self.send(get_recent_inline_bots.as_ref().build())
  }
  
  pub fn get_recently_visited_t_me_urls<C: AsRef<TGGetRecentlyVisitedTMeUrls>>(&self, get_recently_visited_t_me_urls: C) -> &Self {
    self.send(get_recently_visited_t_me_urls.as_ref().build())
  }
  
  pub fn get_recent_stickers<C: AsRef<TGGetRecentStickers>>(&self, get_recent_stickers: C) -> &Self {
    self.send(get_recent_stickers.as_ref().build())
  }
  
  pub fn get_recovery_email_address<C: AsRef<TGGetRecoveryEmailAddress>>(&self, get_recovery_email_address: C) -> &Self {
    self.send(get_recovery_email_address.as_ref().build())
  }
  
  pub fn get_remote_file<C: AsRef<TGGetRemoteFile>>(&self, get_remote_file: C) -> &Self {
    self.send(get_remote_file.as_ref().build())
  }
  
  pub fn get_replied_message<C: AsRef<TGGetRepliedMessage>>(&self, get_replied_message: C) -> &Self {
    self.send(get_replied_message.as_ref().build())
  }
  
  pub fn get_saved_animations<C: AsRef<TGGetSavedAnimations>>(&self, get_saved_animations: C) -> &Self {
    self.send(get_saved_animations.as_ref().build())
  }
  
  pub fn get_saved_order_info<C: AsRef<TGGetSavedOrderInfo>>(&self, get_saved_order_info: C) -> &Self {
    self.send(get_saved_order_info.as_ref().build())
  }
  
  pub fn get_scope_notification_settings<C: AsRef<TGGetScopeNotificationSettings>>(&self, get_scope_notification_settings: C) -> &Self {
    self.send(get_scope_notification_settings.as_ref().build())
  }
  
  pub fn get_secret_chat<C: AsRef<TGGetSecretChat>>(&self, get_secret_chat: C) -> &Self {
    self.send(get_secret_chat.as_ref().build())
  }
  
  pub fn get_sticker_emojis<C: AsRef<TGGetStickerEmojis>>(&self, get_sticker_emojis: C) -> &Self {
    self.send(get_sticker_emojis.as_ref().build())
  }
  
  pub fn get_stickers<C: AsRef<TGGetStickers>>(&self, get_stickers: C) -> &Self {
    self.send(get_stickers.as_ref().build())
  }
  
  pub fn get_sticker_set<C: AsRef<TGGetStickerSet>>(&self, get_sticker_set: C) -> &Self {
    self.send(get_sticker_set.as_ref().build())
  }
  
  pub fn get_storage_statistics<C: AsRef<TGGetStorageStatistics>>(&self, get_storage_statistics: C) -> &Self {
    self.send(get_storage_statistics.as_ref().build())
  }
  
  pub fn get_storage_statistics_fast<C: AsRef<TGGetStorageStatisticsFast>>(&self, get_storage_statistics_fast: C) -> &Self {
    self.send(get_storage_statistics_fast.as_ref().build())
  }
  
  pub fn get_supergroup<C: AsRef<TGGetSupergroup>>(&self, get_supergroup: C) -> &Self {
    self.send(get_supergroup.as_ref().build())
  }
  
  pub fn get_supergroup_full_info<C: AsRef<TGGetSupergroupFullInfo>>(&self, get_supergroup_full_info: C) -> &Self {
    self.send(get_supergroup_full_info.as_ref().build())
  }
  
  pub fn get_supergroup_members<C: AsRef<TGGetSupergroupMembers>>(&self, get_supergroup_members: C) -> &Self {
    self.send(get_supergroup_members.as_ref().build())
  }
  
  pub fn get_support_user<C: AsRef<TGGetSupportUser>>(&self, get_support_user: C) -> &Self {
    self.send(get_support_user.as_ref().build())
  }
  
  pub fn get_temporary_password_state<C: AsRef<TGGetTemporaryPasswordState>>(&self, get_temporary_password_state: C) -> &Self {
    self.send(get_temporary_password_state.as_ref().build())
  }
  
  pub fn get_text_entities<C: AsRef<TGGetTextEntities>>(&self, get_text_entities: C) -> &Self {
    self.send(get_text_entities.as_ref().build())
  }
  
  pub fn get_top_chats<C: AsRef<TGGetTopChats>>(&self, get_top_chats: C) -> &Self {
    self.send(get_top_chats.as_ref().build())
  }
  
  pub fn get_trending_sticker_sets<C: AsRef<TGGetTrendingStickerSets>>(&self, get_trending_sticker_sets: C) -> &Self {
    self.send(get_trending_sticker_sets.as_ref().build())
  }
  
  pub fn get_user<C: AsRef<TGGetUser>>(&self, get_user: C) -> &Self {
    self.send(get_user.as_ref().build())
  }
  
  pub fn get_user_full_info<C: AsRef<TGGetUserFullInfo>>(&self, get_user_full_info: C) -> &Self {
    self.send(get_user_full_info.as_ref().build())
  }
  
  pub fn get_user_privacy_setting_rules<C: AsRef<TGGetUserPrivacySettingRules>>(&self, get_user_privacy_setting_rules: C) -> &Self {
    self.send(get_user_privacy_setting_rules.as_ref().build())
  }
  
  pub fn get_user_profile_photos<C: AsRef<TGGetUserProfilePhotos>>(&self, get_user_profile_photos: C) -> &Self {
    self.send(get_user_profile_photos.as_ref().build())
  }
  
  pub fn get_wallpapers<C: AsRef<TGGetWallpapers>>(&self, get_wallpapers: C) -> &Self {
    self.send(get_wallpapers.as_ref().build())
  }
  
  pub fn get_web_page_instant_view<C: AsRef<TGGetWebPageInstantView>>(&self, get_web_page_instant_view: C) -> &Self {
    self.send(get_web_page_instant_view.as_ref().build())
  }
  
  pub fn get_web_page_preview<C: AsRef<TGGetWebPagePreview>>(&self, get_web_page_preview: C) -> &Self {
    self.send(get_web_page_preview.as_ref().build())
  }
  
  pub fn import_contacts<C: AsRef<TGImportContacts>>(&self, import_contacts: C) -> &Self {
    self.send(import_contacts.as_ref().build())
  }
  
  pub fn join_chat<C: AsRef<TGJoinChat>>(&self, join_chat: C) -> &Self {
    self.send(join_chat.as_ref().build())
  }
  
  pub fn join_chat_by_invite_link<C: AsRef<TGJoinChatByInviteLink>>(&self, join_chat_by_invite_link: C) -> &Self {
    self.send(join_chat_by_invite_link.as_ref().build())
  }
  
  pub fn leave_chat<C: AsRef<TGLeaveChat>>(&self, leave_chat: C) -> &Self {
    self.send(leave_chat.as_ref().build())
  }
  
  pub fn log_out<C: AsRef<TGLogOut>>(&self, log_out: C) -> &Self {
    self.send(log_out.as_ref().build())
  }
  
  pub fn open_chat<C: AsRef<TGOpenChat>>(&self, open_chat: C) -> &Self {
    self.send(open_chat.as_ref().build())
  }
  
  pub fn open_message_content<C: AsRef<TGOpenMessageContent>>(&self, open_message_content: C) -> &Self {
    self.send(open_message_content.as_ref().build())
  }
  
  pub fn optimize_storage<C: AsRef<TGOptimizeStorage>>(&self, optimize_storage: C) -> &Self {
    self.send(optimize_storage.as_ref().build())
  }
  
  pub fn parse_text_entities<C: AsRef<TGParseTextEntities>>(&self, parse_text_entities: C) -> &Self {
    self.send(parse_text_entities.as_ref().build())
  }
  
  pub fn pin_chat_message<C: AsRef<TGPinChatMessage>>(&self, pin_chat_message: C) -> &Self {
    self.send(pin_chat_message.as_ref().build())
  }
  
  pub fn ping_proxy<C: AsRef<TGPingProxy>>(&self, ping_proxy: C) -> &Self {
    self.send(ping_proxy.as_ref().build())
  }
  
  pub fn process_push_notification<C: AsRef<TGProcessPushNotification>>(&self, process_push_notification: C) -> &Self {
    self.send(process_push_notification.as_ref().build())
  }
  
  pub fn read_all_chat_mentions<C: AsRef<TGReadAllChatMentions>>(&self, read_all_chat_mentions: C) -> &Self {
    self.send(read_all_chat_mentions.as_ref().build())
  }
  
  pub fn read_file_part<C: AsRef<TGReadFilePart>>(&self, read_file_part: C) -> &Self {
    self.send(read_file_part.as_ref().build())
  }
  
  pub fn recover_authentication_password<C: AsRef<TGRecoverAuthenticationPassword>>(&self, recover_authentication_password: C) -> &Self {
    self.send(recover_authentication_password.as_ref().build())
  }
  
  pub fn recover_password<C: AsRef<TGRecoverPassword>>(&self, recover_password: C) -> &Self {
    self.send(recover_password.as_ref().build())
  }
  
  pub fn register_device<C: AsRef<TGRegisterDevice>>(&self, register_device: C) -> &Self {
    self.send(register_device.as_ref().build())
  }
  
  pub fn remove_contacts<C: AsRef<TGRemoveContacts>>(&self, remove_contacts: C) -> &Self {
    self.send(remove_contacts.as_ref().build())
  }
  
  pub fn remove_favorite_sticker<C: AsRef<TGRemoveFavoriteSticker>>(&self, remove_favorite_sticker: C) -> &Self {
    self.send(remove_favorite_sticker.as_ref().build())
  }
  
  pub fn remove_notification<C: AsRef<TGRemoveNotification>>(&self, remove_notification: C) -> &Self {
    self.send(remove_notification.as_ref().build())
  }
  
  pub fn remove_notification_group<C: AsRef<TGRemoveNotificationGroup>>(&self, remove_notification_group: C) -> &Self {
    self.send(remove_notification_group.as_ref().build())
  }
  
  pub fn remove_proxy<C: AsRef<TGRemoveProxy>>(&self, remove_proxy: C) -> &Self {
    self.send(remove_proxy.as_ref().build())
  }
  
  pub fn remove_recent_hashtag<C: AsRef<TGRemoveRecentHashtag>>(&self, remove_recent_hashtag: C) -> &Self {
    self.send(remove_recent_hashtag.as_ref().build())
  }
  
  pub fn remove_recently_found_chat<C: AsRef<TGRemoveRecentlyFoundChat>>(&self, remove_recently_found_chat: C) -> &Self {
    self.send(remove_recently_found_chat.as_ref().build())
  }
  
  pub fn remove_recent_sticker<C: AsRef<TGRemoveRecentSticker>>(&self, remove_recent_sticker: C) -> &Self {
    self.send(remove_recent_sticker.as_ref().build())
  }
  
  pub fn remove_saved_animation<C: AsRef<TGRemoveSavedAnimation>>(&self, remove_saved_animation: C) -> &Self {
    self.send(remove_saved_animation.as_ref().build())
  }
  
  pub fn remove_sticker_from_set<C: AsRef<TGRemoveStickerFromSet>>(&self, remove_sticker_from_set: C) -> &Self {
    self.send(remove_sticker_from_set.as_ref().build())
  }
  
  pub fn remove_top_chat<C: AsRef<TGRemoveTopChat>>(&self, remove_top_chat: C) -> &Self {
    self.send(remove_top_chat.as_ref().build())
  }
  
  pub fn reorder_installed_sticker_sets<C: AsRef<TGReorderInstalledStickerSets>>(&self, reorder_installed_sticker_sets: C) -> &Self {
    self.send(reorder_installed_sticker_sets.as_ref().build())
  }
  
  pub fn report_chat<C: AsRef<TGReportChat>>(&self, report_chat: C) -> &Self {
    self.send(report_chat.as_ref().build())
  }
  
  pub fn report_supergroup_spam<C: AsRef<TGReportSupergroupSpam>>(&self, report_supergroup_spam: C) -> &Self {
    self.send(report_supergroup_spam.as_ref().build())
  }
  
  pub fn request_authentication_password_recovery<C: AsRef<TGRequestAuthenticationPasswordRecovery>>(&self, request_authentication_password_recovery: C) -> &Self {
    self.send(request_authentication_password_recovery.as_ref().build())
  }
  
  pub fn request_password_recovery<C: AsRef<TGRequestPasswordRecovery>>(&self, request_password_recovery: C) -> &Self {
    self.send(request_password_recovery.as_ref().build())
  }
  
  pub fn resend_authentication_code<C: AsRef<TGResendAuthenticationCode>>(&self, resend_authentication_code: C) -> &Self {
    self.send(resend_authentication_code.as_ref().build())
  }
  
  pub fn resend_change_phone_number_code<C: AsRef<TGResendChangePhoneNumberCode>>(&self, resend_change_phone_number_code: C) -> &Self {
    self.send(resend_change_phone_number_code.as_ref().build())
  }
  
  pub fn resend_email_address_verification_code<C: AsRef<TGResendEmailAddressVerificationCode>>(&self, resend_email_address_verification_code: C) -> &Self {
    self.send(resend_email_address_verification_code.as_ref().build())
  }
  
  pub fn resend_phone_number_confirmation_code<C: AsRef<TGResendPhoneNumberConfirmationCode>>(&self, resend_phone_number_confirmation_code: C) -> &Self {
    self.send(resend_phone_number_confirmation_code.as_ref().build())
  }
  
  pub fn resend_phone_number_verification_code<C: AsRef<TGResendPhoneNumberVerificationCode>>(&self, resend_phone_number_verification_code: C) -> &Self {
    self.send(resend_phone_number_verification_code.as_ref().build())
  }
  
  pub fn resend_recovery_email_address_code<C: AsRef<TGResendRecoveryEmailAddressCode>>(&self, resend_recovery_email_address_code: C) -> &Self {
    self.send(resend_recovery_email_address_code.as_ref().build())
  }
  
  pub fn reset_all_notification_settings<C: AsRef<TGResetAllNotificationSettings>>(&self, reset_all_notification_settings: C) -> &Self {
    self.send(reset_all_notification_settings.as_ref().build())
  }
  
  pub fn reset_network_statistics<C: AsRef<TGResetNetworkStatistics>>(&self, reset_network_statistics: C) -> &Self {
    self.send(reset_network_statistics.as_ref().build())
  }
  
  pub fn search_call_messages<C: AsRef<TGSearchCallMessages>>(&self, search_call_messages: C) -> &Self {
    self.send(search_call_messages.as_ref().build())
  }
  
  pub fn search_chat_members<C: AsRef<TGSearchChatMembers>>(&self, search_chat_members: C) -> &Self {
    self.send(search_chat_members.as_ref().build())
  }
  
  pub fn search_chat_messages<C: AsRef<TGSearchChatMessages>>(&self, search_chat_messages: C) -> &Self {
    self.send(search_chat_messages.as_ref().build())
  }
  
  pub fn search_chat_recent_location_messages<C: AsRef<TGSearchChatRecentLocationMessages>>(&self, search_chat_recent_location_messages: C) -> &Self {
    self.send(search_chat_recent_location_messages.as_ref().build())
  }
  
  pub fn search_chats<C: AsRef<TGSearchChats>>(&self, search_chats: C) -> &Self {
    self.send(search_chats.as_ref().build())
  }
  
  pub fn search_chats_on_server<C: AsRef<TGSearchChatsOnServer>>(&self, search_chats_on_server: C) -> &Self {
    self.send(search_chats_on_server.as_ref().build())
  }
  
  pub fn search_contacts<C: AsRef<TGSearchContacts>>(&self, search_contacts: C) -> &Self {
    self.send(search_contacts.as_ref().build())
  }
  
  pub fn search_hashtags<C: AsRef<TGSearchHashtags>>(&self, search_hashtags: C) -> &Self {
    self.send(search_hashtags.as_ref().build())
  }
  
  pub fn search_installed_sticker_sets<C: AsRef<TGSearchInstalledStickerSets>>(&self, search_installed_sticker_sets: C) -> &Self {
    self.send(search_installed_sticker_sets.as_ref().build())
  }
  
  pub fn search_messages<C: AsRef<TGSearchMessages>>(&self, search_messages: C) -> &Self {
    self.send(search_messages.as_ref().build())
  }
  
  pub fn search_public_chat<C: AsRef<TGSearchPublicChat>>(&self, search_public_chat: C) -> &Self {
    self.send(search_public_chat.as_ref().build())
  }
  
  pub fn search_public_chats<C: AsRef<TGSearchPublicChats>>(&self, search_public_chats: C) -> &Self {
    self.send(search_public_chats.as_ref().build())
  }
  
  pub fn search_secret_messages<C: AsRef<TGSearchSecretMessages>>(&self, search_secret_messages: C) -> &Self {
    self.send(search_secret_messages.as_ref().build())
  }
  
  pub fn search_stickers<C: AsRef<TGSearchStickers>>(&self, search_stickers: C) -> &Self {
    self.send(search_stickers.as_ref().build())
  }
  
  pub fn search_sticker_set<C: AsRef<TGSearchStickerSet>>(&self, search_sticker_set: C) -> &Self {
    self.send(search_sticker_set.as_ref().build())
  }
  
  pub fn search_sticker_sets<C: AsRef<TGSearchStickerSets>>(&self, search_sticker_sets: C) -> &Self {
    self.send(search_sticker_sets.as_ref().build())
  }
  
  pub fn send_bot_start_message<C: AsRef<TGSendBotStartMessage>>(&self, send_bot_start_message: C) -> &Self {
    self.send(send_bot_start_message.as_ref().build())
  }
  
  pub fn send_call_debug_information<C: AsRef<TGSendCallDebugInformation>>(&self, send_call_debug_information: C) -> &Self {
    self.send(send_call_debug_information.as_ref().build())
  }
  
  pub fn send_call_rating<C: AsRef<TGSendCallRating>>(&self, send_call_rating: C) -> &Self {
    self.send(send_call_rating.as_ref().build())
  }
  
  pub fn send_chat_action<C: AsRef<TGSendChatAction>>(&self, send_chat_action: C) -> &Self {
    self.send(send_chat_action.as_ref().build())
  }
  
  pub fn send_chat_screenshot_taken_notification<C: AsRef<TGSendChatScreenshotTakenNotification>>(&self, send_chat_screenshot_taken_notification: C) -> &Self {
    self.send(send_chat_screenshot_taken_notification.as_ref().build())
  }
  
  pub fn send_chat_set_ttl_message<C: AsRef<TGSendChatSetTtlMessage>>(&self, send_chat_set_ttl_message: C) -> &Self {
    self.send(send_chat_set_ttl_message.as_ref().build())
  }
  
  pub fn send_custom_request<C: AsRef<TGSendCustomRequest>>(&self, send_custom_request: C) -> &Self {
    self.send(send_custom_request.as_ref().build())
  }
  
  pub fn send_email_address_verification_code<C: AsRef<TGSendEmailAddressVerificationCode>>(&self, send_email_address_verification_code: C) -> &Self {
    self.send(send_email_address_verification_code.as_ref().build())
  }
  
  pub fn send_inline_query_result_message<C: AsRef<TGSendInlineQueryResultMessage>>(&self, send_inline_query_result_message: C) -> &Self {
    self.send(send_inline_query_result_message.as_ref().build())
  }
  
  pub fn send_message<C: AsRef<TGSendMessage>>(&self, send_message: C) -> &Self {
    self.send(send_message.as_ref().build())
  }
  
  pub fn send_message_album<C: AsRef<TGSendMessageAlbum>>(&self, send_message_album: C) -> &Self {
    self.send(send_message_album.as_ref().build())
  }
  
  pub fn send_passport_authorization_form<C: AsRef<TGSendPassportAuthorizationForm>>(&self, send_passport_authorization_form: C) -> &Self {
    self.send(send_passport_authorization_form.as_ref().build())
  }
  
  pub fn send_payment_form<C: AsRef<TGSendPaymentForm>>(&self, send_payment_form: C) -> &Self {
    self.send(send_payment_form.as_ref().build())
  }
  
  pub fn send_phone_number_confirmation_code<C: AsRef<TGSendPhoneNumberConfirmationCode>>(&self, send_phone_number_confirmation_code: C) -> &Self {
    self.send(send_phone_number_confirmation_code.as_ref().build())
  }
  
  pub fn send_phone_number_verification_code<C: AsRef<TGSendPhoneNumberVerificationCode>>(&self, send_phone_number_verification_code: C) -> &Self {
    self.send(send_phone_number_verification_code.as_ref().build())
  }
  
  pub fn set_account_ttl<C: AsRef<TGSetAccountTtl>>(&self, set_account_ttl: C) -> &Self {
    self.send(set_account_ttl.as_ref().build())
  }
  
  pub fn set_alarm<C: AsRef<TGSetAlarm>>(&self, set_alarm: C) -> &Self {
    self.send(set_alarm.as_ref().build())
  }
  
  pub fn set_authentication_phone_number<C: AsRef<TGSetAuthenticationPhoneNumber>>(&self, set_authentication_phone_number: C) -> &Self {
    self.send(set_authentication_phone_number.as_ref().build())
  }
  
  pub fn set_bio<C: AsRef<TGSetBio>>(&self, set_bio: C) -> &Self {
    self.send(set_bio.as_ref().build())
  }
  
  pub fn set_bot_updates_status<C: AsRef<TGSetBotUpdatesStatus>>(&self, set_bot_updates_status: C) -> &Self {
    self.send(set_bot_updates_status.as_ref().build())
  }
  
  pub fn set_chat_client_data<C: AsRef<TGSetChatClientData>>(&self, set_chat_client_data: C) -> &Self {
    self.send(set_chat_client_data.as_ref().build())
  }
  
  pub fn set_chat_draft_message<C: AsRef<TGSetChatDraftMessage>>(&self, set_chat_draft_message: C) -> &Self {
    self.send(set_chat_draft_message.as_ref().build())
  }
  
  pub fn set_chat_member_status<C: AsRef<TGSetChatMemberStatus>>(&self, set_chat_member_status: C) -> &Self {
    self.send(set_chat_member_status.as_ref().build())
  }
  
  pub fn set_chat_notification_settings<C: AsRef<TGSetChatNotificationSettings>>(&self, set_chat_notification_settings: C) -> &Self {
    self.send(set_chat_notification_settings.as_ref().build())
  }
  
  pub fn set_chat_photo<C: AsRef<TGSetChatPhoto>>(&self, set_chat_photo: C) -> &Self {
    self.send(set_chat_photo.as_ref().build())
  }
  
  pub fn set_chat_title<C: AsRef<TGSetChatTitle>>(&self, set_chat_title: C) -> &Self {
    self.send(set_chat_title.as_ref().build())
  }
  
  pub fn set_custom_language_pack<C: AsRef<TGSetCustomLanguagePack>>(&self, set_custom_language_pack: C) -> &Self {
    self.send(set_custom_language_pack.as_ref().build())
  }
  
  pub fn set_custom_language_pack_string<C: AsRef<TGSetCustomLanguagePackString>>(&self, set_custom_language_pack_string: C) -> &Self {
    self.send(set_custom_language_pack_string.as_ref().build())
  }
  
  pub fn set_database_encryption_key<C: AsRef<TGSetDatabaseEncryptionKey>>(&self, set_database_encryption_key: C) -> &Self {
    self.send(set_database_encryption_key.as_ref().build())
  }
  
  pub fn set_file_generation_progress<C: AsRef<TGSetFileGenerationProgress>>(&self, set_file_generation_progress: C) -> &Self {
    self.send(set_file_generation_progress.as_ref().build())
  }
  
  pub fn set_game_score<C: AsRef<TGSetGameScore>>(&self, set_game_score: C) -> &Self {
    self.send(set_game_score.as_ref().build())
  }
  
  pub fn set_inline_game_score<C: AsRef<TGSetInlineGameScore>>(&self, set_inline_game_score: C) -> &Self {
    self.send(set_inline_game_score.as_ref().build())
  }
  
  pub fn set_log_stream<C: AsRef<TGSetLogStream>>(&self, set_log_stream: C) -> &Self {
    self.send(set_log_stream.as_ref().build())
  }
  
  pub fn set_log_tag_verbosity_level<C: AsRef<TGSetLogTagVerbosityLevel>>(&self, set_log_tag_verbosity_level: C) -> &Self {
    self.send(set_log_tag_verbosity_level.as_ref().build())
  }
  
  pub fn set_log_verbosity_level<C: AsRef<TGSetLogVerbosityLevel>>(&self, set_log_verbosity_level: C) -> &Self {
    self.send(set_log_verbosity_level.as_ref().build())
  }
  
  pub fn set_name<C: AsRef<TGSetName>>(&self, set_name: C) -> &Self {
    self.send(set_name.as_ref().build())
  }
  
  pub fn set_network_type<C: AsRef<TGSetNetworkType>>(&self, set_network_type: C) -> &Self {
    self.send(set_network_type.as_ref().build())
  }
  
  pub fn set_option<C: AsRef<TGSetOption>>(&self, set_option: C) -> &Self {
    self.send(set_option.as_ref().build())
  }
  
  pub fn set_passport_element<C: AsRef<TGSetPassportElement>>(&self, set_passport_element: C) -> &Self {
    self.send(set_passport_element.as_ref().build())
  }
  
  pub fn set_passport_element_errors<C: AsRef<TGSetPassportElementErrors>>(&self, set_passport_element_errors: C) -> &Self {
    self.send(set_passport_element_errors.as_ref().build())
  }
  
  pub fn set_password<C: AsRef<TGSetPassword>>(&self, set_password: C) -> &Self {
    self.send(set_password.as_ref().build())
  }
  
  pub fn set_pinned_chats<C: AsRef<TGSetPinnedChats>>(&self, set_pinned_chats: C) -> &Self {
    self.send(set_pinned_chats.as_ref().build())
  }
  
  pub fn set_poll_answer<C: AsRef<TGSetPollAnswer>>(&self, set_poll_answer: C) -> &Self {
    self.send(set_poll_answer.as_ref().build())
  }
  
  pub fn set_profile_photo<C: AsRef<TGSetProfilePhoto>>(&self, set_profile_photo: C) -> &Self {
    self.send(set_profile_photo.as_ref().build())
  }
  
  pub fn set_recovery_email_address<C: AsRef<TGSetRecoveryEmailAddress>>(&self, set_recovery_email_address: C) -> &Self {
    self.send(set_recovery_email_address.as_ref().build())
  }
  
  pub fn set_scope_notification_settings<C: AsRef<TGSetScopeNotificationSettings>>(&self, set_scope_notification_settings: C) -> &Self {
    self.send(set_scope_notification_settings.as_ref().build())
  }
  
  pub fn set_sticker_position_in_set<C: AsRef<TGSetStickerPositionInSet>>(&self, set_sticker_position_in_set: C) -> &Self {
    self.send(set_sticker_position_in_set.as_ref().build())
  }
  
  pub fn set_supergroup_description<C: AsRef<TGSetSupergroupDescription>>(&self, set_supergroup_description: C) -> &Self {
    self.send(set_supergroup_description.as_ref().build())
  }
  
  pub fn set_supergroup_sticker_set<C: AsRef<TGSetSupergroupStickerSet>>(&self, set_supergroup_sticker_set: C) -> &Self {
    self.send(set_supergroup_sticker_set.as_ref().build())
  }
  
  pub fn set_supergroup_username<C: AsRef<TGSetSupergroupUsername>>(&self, set_supergroup_username: C) -> &Self {
    self.send(set_supergroup_username.as_ref().build())
  }
  
  pub fn set_tdlib_parameters<C: AsRef<TGSetTdlibParameters>>(&self, set_tdlib_parameters: C) -> &Self {
    self.send(set_tdlib_parameters.as_ref().build())
  }
  
  pub fn set_username<C: AsRef<TGSetUsername>>(&self, set_username: C) -> &Self {
    self.send(set_username.as_ref().build())
  }
  
  pub fn set_user_privacy_setting_rules<C: AsRef<TGSetUserPrivacySettingRules>>(&self, set_user_privacy_setting_rules: C) -> &Self {
    self.send(set_user_privacy_setting_rules.as_ref().build())
  }
  
  pub fn stop_poll<C: AsRef<TGStopPoll>>(&self, stop_poll: C) -> &Self {
    self.send(stop_poll.as_ref().build())
  }
  
  pub fn synchronize_language_pack<C: AsRef<TGSynchronizeLanguagePack>>(&self, synchronize_language_pack: C) -> &Self {
    self.send(synchronize_language_pack.as_ref().build())
  }
  
  pub fn terminate_all_other_sessions<C: AsRef<TGTerminateAllOtherSessions>>(&self, terminate_all_other_sessions: C) -> &Self {
    self.send(terminate_all_other_sessions.as_ref().build())
  }
  
  pub fn terminate_session<C: AsRef<TGTerminateSession>>(&self, terminate_session: C) -> &Self {
    self.send(terminate_session.as_ref().build())
  }
  
  pub fn test_call_bytes<C: AsRef<TGTestCallBytes>>(&self, test_call_bytes: C) -> &Self {
    self.send(test_call_bytes.as_ref().build())
  }
  
  pub fn test_call_empty<C: AsRef<TGTestCallEmpty>>(&self, test_call_empty: C) -> &Self {
    self.send(test_call_empty.as_ref().build())
  }
  
  pub fn test_call_string<C: AsRef<TGTestCallString>>(&self, test_call_string: C) -> &Self {
    self.send(test_call_string.as_ref().build())
  }
  
  pub fn test_call_vector_int<C: AsRef<TGTestCallVectorInt>>(&self, test_call_vector_int: C) -> &Self {
    self.send(test_call_vector_int.as_ref().build())
  }
  
  pub fn test_call_vector_int_object<C: AsRef<TGTestCallVectorIntObject>>(&self, test_call_vector_int_object: C) -> &Self {
    self.send(test_call_vector_int_object.as_ref().build())
  }
  
  pub fn test_call_vector_string<C: AsRef<TGTestCallVectorString>>(&self, test_call_vector_string: C) -> &Self {
    self.send(test_call_vector_string.as_ref().build())
  }
  
  pub fn test_call_vector_string_object<C: AsRef<TGTestCallVectorStringObject>>(&self, test_call_vector_string_object: C) -> &Self {
    self.send(test_call_vector_string_object.as_ref().build())
  }
  
  pub fn test_get_difference<C: AsRef<TGTestGetDifference>>(&self, test_get_difference: C) -> &Self {
    self.send(test_get_difference.as_ref().build())
  }
  
  pub fn test_network<C: AsRef<TGTestNetwork>>(&self, test_network: C) -> &Self {
    self.send(test_network.as_ref().build())
  }
  
  pub fn test_square_int<C: AsRef<TGTestSquareInt>>(&self, test_square_int: C) -> &Self {
    self.send(test_square_int.as_ref().build())
  }
  
  pub fn test_use_error<C: AsRef<TGTestUseError>>(&self, test_use_error: C) -> &Self {
    self.send(test_use_error.as_ref().build())
  }
  
  pub fn test_use_update<C: AsRef<TGTestUseUpdate>>(&self, test_use_update: C) -> &Self {
    self.send(test_use_update.as_ref().build())
  }
  
  pub fn toggle_basic_group_administrators<C: AsRef<TGToggleBasicGroupAdministrators>>(&self, toggle_basic_group_administrators: C) -> &Self {
    self.send(toggle_basic_group_administrators.as_ref().build())
  }
  
  pub fn toggle_chat_default_disable_notification<C: AsRef<TGToggleChatDefaultDisableNotification>>(&self, toggle_chat_default_disable_notification: C) -> &Self {
    self.send(toggle_chat_default_disable_notification.as_ref().build())
  }
  
  pub fn toggle_chat_is_marked_as_unread<C: AsRef<TGToggleChatIsMarkedAsUnread>>(&self, toggle_chat_is_marked_as_unread: C) -> &Self {
    self.send(toggle_chat_is_marked_as_unread.as_ref().build())
  }
  
  pub fn toggle_chat_is_pinned<C: AsRef<TGToggleChatIsPinned>>(&self, toggle_chat_is_pinned: C) -> &Self {
    self.send(toggle_chat_is_pinned.as_ref().build())
  }
  
  pub fn toggle_supergroup_invites<C: AsRef<TGToggleSupergroupInvites>>(&self, toggle_supergroup_invites: C) -> &Self {
    self.send(toggle_supergroup_invites.as_ref().build())
  }
  
  pub fn toggle_supergroup_is_all_history_available<C: AsRef<TGToggleSupergroupIsAllHistoryAvailable>>(&self, toggle_supergroup_is_all_history_available: C) -> &Self {
    self.send(toggle_supergroup_is_all_history_available.as_ref().build())
  }
  
  pub fn toggle_supergroup_sign_messages<C: AsRef<TGToggleSupergroupSignMessages>>(&self, toggle_supergroup_sign_messages: C) -> &Self {
    self.send(toggle_supergroup_sign_messages.as_ref().build())
  }
  
  pub fn unblock_user<C: AsRef<TGUnblockUser>>(&self, unblock_user: C) -> &Self {
    self.send(unblock_user.as_ref().build())
  }
  
  pub fn unpin_chat_message<C: AsRef<TGUnpinChatMessage>>(&self, unpin_chat_message: C) -> &Self {
    self.send(unpin_chat_message.as_ref().build())
  }
  
  pub fn upgrade_basic_group_chat_to_supergroup_chat<C: AsRef<TGUpgradeBasicGroupChatToSupergroupChat>>(&self, upgrade_basic_group_chat_to_supergroup_chat: C) -> &Self {
    self.send(upgrade_basic_group_chat_to_supergroup_chat.as_ref().build())
  }
  
  pub fn upload_file<C: AsRef<TGUploadFile>>(&self, upload_file: C) -> &Self {
    self.send(upload_file.as_ref().build())
  }
  
  pub fn upload_sticker_file<C: AsRef<TGUploadStickerFile>>(&self, upload_sticker_file: C) -> &Self {
    self.send(upload_sticker_file.as_ref().build())
  }
  
  pub fn validate_order_info<C: AsRef<TGValidateOrderInfo>>(&self, validate_order_info: C) -> &Self {
    self.send(validate_order_info.as_ref().build())
  }
  
  pub fn view_messages<C: AsRef<TGViewMessages>>(&self, view_messages: C) -> &Self {
    self.send(view_messages.as_ref().build())
  }
  
  pub fn view_trending_sticker_sets<C: AsRef<TGViewTrendingStickerSets>>(&self, view_trending_sticker_sets: C) -> &Self {
    self.send(view_trending_sticker_sets.as_ref().build())
  }
  
  pub fn write_generated_file_part<C: AsRef<TGWriteGeneratedFilePart>>(&self, write_generated_file_part: C) -> &Self {
    self.send(write_generated_file_part.as_ref().build())
  }
  

}

