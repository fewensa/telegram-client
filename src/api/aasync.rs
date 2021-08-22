use futures::StreamExt;
use rtdlib::errors::{RTDError, RTDResult};
use rtdlib::types::*;

use crate::api::Api;
use crate::observer;

macro_rules! async_caller {
  ($td_type:ident) => {
    async fn async_caller<Fnc: RFunction>(api: &Api, fnc: Fnc) -> RTDResult<$td_type> {
      let extra = fnc.extra()
        .ok_or(RTDError::Custom("invalid tdjson response type, not have `extra` field".to_string()))?;
      let mut rec = observer::subscribe(&extra);
      api.send(&fnc)?;
      let val = rec.next().await;
      observer::unsubscribe(&extra);
      if let Some(TdType::Error(v)) = val {
        return Err(RTDError::custom(format!("[{}] {}", v.code(), v.message())));
      }
      match val {
        Some(TdType::$td_type(v)) => { Ok(v) }
        _ => { Err(RTDError::custom("invalid tdjson response type, unexpected `extra` field".to_string())) }
      }
    }
  }
}

#[derive(Clone)]
pub struct AsyncApi {
  api: Api,
}

impl AsyncApi {
  pub fn new(api: Api) -> Self {
    Self { api}
  }

  #[doc(hidden)]
  pub fn api(&self) -> &Api {
    &self.api
  }


  pub async fn accept_call<C: AsRef<AcceptCall>>(&self, accept_call: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, accept_call.as_ref()).await
  }

  pub async fn accept_terms_of_service<C: AsRef<AcceptTermsOfService>>(&self, accept_terms_of_service: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, accept_terms_of_service.as_ref()).await
  }

  pub async fn add_chat_member<C: AsRef<AddChatMember>>(&self, add_chat_member: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, add_chat_member.as_ref()).await
  }

  pub async fn add_chat_members<C: AsRef<AddChatMembers>>(&self, add_chat_members: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, add_chat_members.as_ref()).await
  }

  pub async fn add_custom_server_language_pack<C: AsRef<AddCustomServerLanguagePack>>(&self, add_custom_server_language_pack: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, add_custom_server_language_pack.as_ref()).await
  }

  pub async fn add_favorite_sticker<C: AsRef<AddFavoriteSticker>>(&self, add_favorite_sticker: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, add_favorite_sticker.as_ref()).await
  }

  pub async fn add_local_message<C: AsRef<AddLocalMessage>>(&self, add_local_message: C) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, add_local_message.as_ref()).await
  }

  pub async fn add_log_message<C: AsRef<AddLogMessage>>(&self, add_log_message: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, add_log_message.as_ref()).await
  }

  pub async fn add_network_statistics<C: AsRef<AddNetworkStatistics>>(&self, add_network_statistics: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, add_network_statistics.as_ref()).await
  }

  pub async fn add_proxy<C: AsRef<AddProxy>>(&self, add_proxy: C) -> RTDResult<Proxy> {
    async_caller!(Proxy);
    async_caller(&self.api, add_proxy.as_ref()).await
  }

  pub async fn add_recent_sticker<C: AsRef<AddRecentSticker>>(&self, add_recent_sticker: C) -> RTDResult<Stickers> {
    async_caller!(Stickers);
    async_caller(&self.api, add_recent_sticker.as_ref()).await
  }

  pub async fn add_recently_found_chat<C: AsRef<AddRecentlyFoundChat>>(&self, add_recently_found_chat: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, add_recently_found_chat.as_ref()).await
  }

  pub async fn add_saved_animation<C: AsRef<AddSavedAnimation>>(&self, add_saved_animation: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, add_saved_animation.as_ref()).await
  }

  pub async fn add_sticker_to_set<C: AsRef<AddStickerToSet>>(&self, add_sticker_to_set: C) -> RTDResult<StickerSet> {
    async_caller!(StickerSet);
    async_caller(&self.api, add_sticker_to_set.as_ref()).await
  }

  pub async fn answer_callback_query<C: AsRef<AnswerCallbackQuery>>(&self, answer_callback_query: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, answer_callback_query.as_ref()).await
  }

  pub async fn answer_custom_query<C: AsRef<AnswerCustomQuery>>(&self, answer_custom_query: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, answer_custom_query.as_ref()).await
  }

  pub async fn answer_inline_query<C: AsRef<AnswerInlineQuery>>(&self, answer_inline_query: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, answer_inline_query.as_ref()).await
  }

  pub async fn answer_pre_checkout_query<C: AsRef<AnswerPreCheckoutQuery>>(&self, answer_pre_checkout_query: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, answer_pre_checkout_query.as_ref()).await
  }

  pub async fn answer_shipping_query<C: AsRef<AnswerShippingQuery>>(&self, answer_shipping_query: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, answer_shipping_query.as_ref()).await
  }

  pub async fn block_user<C: AsRef<BlockUser>>(&self, block_user: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, block_user.as_ref()).await
  }

  pub async fn cancel_download_file<C: AsRef<CancelDownloadFile>>(&self, cancel_download_file: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, cancel_download_file.as_ref()).await
  }

  pub async fn cancel_upload_file<C: AsRef<CancelUploadFile>>(&self, cancel_upload_file: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, cancel_upload_file.as_ref()).await
  }

  pub async fn change_chat_report_spam_state<C: AsRef<ChangeChatReportSpamState>>(&self, change_chat_report_spam_state: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, change_chat_report_spam_state.as_ref()).await
  }

  pub async fn change_imported_contacts<C: AsRef<ChangeImportedContacts>>(&self, change_imported_contacts: C) -> RTDResult<ImportedContacts> {
    async_caller!(ImportedContacts);
    async_caller(&self.api, change_imported_contacts.as_ref()).await
  }

  pub async fn change_phone_number<C: AsRef<ChangePhoneNumber>>(&self, change_phone_number: C) -> RTDResult<AuthenticationCodeInfo> {
    async_caller!(AuthenticationCodeInfo);
    async_caller(&self.api, change_phone_number.as_ref()).await
  }

  pub async fn change_sticker_set<C: AsRef<ChangeStickerSet>>(&self, change_sticker_set: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, change_sticker_set.as_ref()).await
  }

  pub async fn check_authentication_bot_token<C: AsRef<CheckAuthenticationBotToken>>(&self, check_authentication_bot_token: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, check_authentication_bot_token.as_ref()).await
  }

  pub async fn check_authentication_code<C: AsRef<CheckAuthenticationCode>>(&self, check_authentication_code: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, check_authentication_code.as_ref()).await
  }

  pub async fn check_authentication_password<C: AsRef<CheckAuthenticationPassword>>(&self, check_authentication_password: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, check_authentication_password.as_ref()).await
  }

  pub async fn check_change_phone_number_code<C: AsRef<CheckChangePhoneNumberCode>>(&self, check_change_phone_number_code: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, check_change_phone_number_code.as_ref()).await
  }

  pub async fn check_chat_invite_link<C: AsRef<CheckChatInviteLink>>(&self, check_chat_invite_link: C) -> RTDResult<ChatInviteLinkInfo> {
    async_caller!(ChatInviteLinkInfo);
    async_caller(&self.api, check_chat_invite_link.as_ref()).await
  }

  pub async fn check_chat_username<C: AsRef<CheckChatUsername>>(&self, check_chat_username: C) -> RTDResult<CheckChatUsernameResult> {
    async_caller!(CheckChatUsernameResult);
    async_caller(&self.api, check_chat_username.as_ref()).await
  }

  pub async fn check_database_encryption_key<C: AsRef<CheckDatabaseEncryptionKey>>(&self, check_database_encryption_key: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, check_database_encryption_key.as_ref()).await
  }

  pub async fn check_email_address_verification_code<C: AsRef<CheckEmailAddressVerificationCode>>(&self, check_email_address_verification_code: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, check_email_address_verification_code.as_ref()).await
  }

  pub async fn check_phone_number_confirmation_code<C: AsRef<CheckPhoneNumberConfirmationCode>>(&self, check_phone_number_confirmation_code: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, check_phone_number_confirmation_code.as_ref()).await
  }

  pub async fn check_phone_number_verification_code<C: AsRef<CheckPhoneNumberVerificationCode>>(&self, check_phone_number_verification_code: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, check_phone_number_verification_code.as_ref()).await
  }

  pub async fn check_recovery_email_address_code<C: AsRef<CheckRecoveryEmailAddressCode>>(&self, check_recovery_email_address_code: C) -> RTDResult<PasswordState> {
    async_caller!(PasswordState);
    async_caller(&self.api, check_recovery_email_address_code.as_ref()).await
  }

  pub async fn clean_file_name<C: AsRef<CleanFileName>>(&self, clean_file_name: C) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, clean_file_name.as_ref()).await
  }

  pub async fn clear_all_draft_messages<C: AsRef<ClearAllDraftMessages>>(&self, clear_all_draft_messages: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, clear_all_draft_messages.as_ref()).await
  }

  pub async fn clear_imported_contacts<C: AsRef<ClearImportedContacts>>(&self, clear_imported_contacts: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, clear_imported_contacts.as_ref()).await
  }

  pub async fn clear_recent_stickers<C: AsRef<ClearRecentStickers>>(&self, clear_recent_stickers: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, clear_recent_stickers.as_ref()).await
  }

  pub async fn clear_recently_found_chats<C: AsRef<ClearRecentlyFoundChats>>(&self, clear_recently_found_chats: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, clear_recently_found_chats.as_ref()).await
  }

  pub async fn close<C: AsRef<Close>>(&self, close: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, close.as_ref()).await
  }

  pub async fn close_chat<C: AsRef<CloseChat>>(&self, close_chat: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, close_chat.as_ref()).await
  }

  pub async fn close_secret_chat<C: AsRef<CloseSecretChat>>(&self, close_secret_chat: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, close_secret_chat.as_ref()).await
  }

  pub async fn create_basic_group_chat<C: AsRef<CreateBasicGroupChat>>(&self, create_basic_group_chat: C) -> RTDResult<Chat> {
    async_caller!(Chat);
    async_caller(&self.api, create_basic_group_chat.as_ref()).await
  }

  pub async fn create_call<C: AsRef<CreateCall>>(&self, create_call: C) -> RTDResult<CallId> {
    async_caller!(CallId);
    async_caller(&self.api, create_call.as_ref()).await
  }

  pub async fn create_new_basic_group_chat<C: AsRef<CreateNewBasicGroupChat>>(&self, create_new_basic_group_chat: C) -> RTDResult<Chat> {
    async_caller!(Chat);
    async_caller(&self.api, create_new_basic_group_chat.as_ref()).await
  }

  pub async fn create_new_secret_chat<C: AsRef<CreateNewSecretChat>>(&self, create_new_secret_chat: C) -> RTDResult<Chat> {
    async_caller!(Chat);
    async_caller(&self.api, create_new_secret_chat.as_ref()).await
  }

  pub async fn create_new_sticker_set<C: AsRef<CreateNewStickerSet>>(&self, create_new_sticker_set: C) -> RTDResult<StickerSet> {
    async_caller!(StickerSet);
    async_caller(&self.api, create_new_sticker_set.as_ref()).await
  }

  pub async fn create_new_supergroup_chat<C: AsRef<CreateNewSupergroupChat>>(&self, create_new_supergroup_chat: C) -> RTDResult<Chat> {
    async_caller!(Chat);
    async_caller(&self.api, create_new_supergroup_chat.as_ref()).await
  }

  pub async fn create_private_chat<C: AsRef<CreatePrivateChat>>(&self, create_private_chat: C) -> RTDResult<Chat> {
    async_caller!(Chat);
    async_caller(&self.api, create_private_chat.as_ref()).await
  }

  pub async fn create_secret_chat<C: AsRef<CreateSecretChat>>(&self, create_secret_chat: C) -> RTDResult<Chat> {
    async_caller!(Chat);
    async_caller(&self.api, create_secret_chat.as_ref()).await
  }

  pub async fn create_supergroup_chat<C: AsRef<CreateSupergroupChat>>(&self, create_supergroup_chat: C) -> RTDResult<Chat> {
    async_caller!(Chat);
    async_caller(&self.api, create_supergroup_chat.as_ref()).await
  }

  pub async fn create_temporary_password<C: AsRef<CreateTemporaryPassword>>(&self, create_temporary_password: C) -> RTDResult<TemporaryPasswordState> {
    async_caller!(TemporaryPasswordState);
    async_caller(&self.api, create_temporary_password.as_ref()).await
  }

  pub async fn delete_account<C: AsRef<DeleteAccount>>(&self, delete_account: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_account.as_ref()).await
  }

  pub async fn delete_chat_history<C: AsRef<DeleteChatHistory>>(&self, delete_chat_history: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_chat_history.as_ref()).await
  }

  pub async fn delete_chat_messages_from_user<C: AsRef<DeleteChatMessagesFromUser>>(&self, delete_chat_messages_from_user: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_chat_messages_from_user.as_ref()).await
  }

  pub async fn delete_chat_reply_markup<C: AsRef<DeleteChatReplyMarkup>>(&self, delete_chat_reply_markup: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_chat_reply_markup.as_ref()).await
  }

  pub async fn delete_file<C: AsRef<DeleteFile>>(&self, delete_file: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_file.as_ref()).await
  }

  pub async fn delete_language_pack<C: AsRef<DeleteLanguagePack>>(&self, delete_language_pack: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_language_pack.as_ref()).await
  }

  pub async fn delete_messages<C: AsRef<DeleteMessages>>(&self, delete_messages: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_messages.as_ref()).await
  }

  pub async fn delete_passport_element<C: AsRef<DeletePassportElement>>(&self, delete_passport_element: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_passport_element.as_ref()).await
  }

  pub async fn delete_profile_photo<C: AsRef<DeleteProfilePhoto>>(&self, delete_profile_photo: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_profile_photo.as_ref()).await
  }

  pub async fn delete_saved_credentials<C: AsRef<DeleteSavedCredentials>>(&self, delete_saved_credentials: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_saved_credentials.as_ref()).await
  }

  pub async fn delete_saved_order_info<C: AsRef<DeleteSavedOrderInfo>>(&self, delete_saved_order_info: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_saved_order_info.as_ref()).await
  }

  pub async fn delete_supergroup<C: AsRef<DeleteSupergroup>>(&self, delete_supergroup: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_supergroup.as_ref()).await
  }

  pub async fn destroy<C: AsRef<Destroy>>(&self, destroy: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, destroy.as_ref()).await
  }

  pub async fn disable_proxy<C: AsRef<DisableProxy>>(&self, disable_proxy: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, disable_proxy.as_ref()).await
  }

  pub async fn discard_call<C: AsRef<DiscardCall>>(&self, discard_call: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, discard_call.as_ref()).await
  }

  pub async fn disconnect_all_websites<C: AsRef<DisconnectAllWebsites>>(&self, disconnect_all_websites: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, disconnect_all_websites.as_ref()).await
  }

  pub async fn disconnect_website<C: AsRef<DisconnectWebsite>>(&self, disconnect_website: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, disconnect_website.as_ref()).await
  }

  pub async fn download_file<C: AsRef<DownloadFile>>(&self, download_file: C) -> RTDResult<File> {
    async_caller!(File);
    async_caller(&self.api, download_file.as_ref()).await
  }

  pub async fn edit_custom_language_pack_info<C: AsRef<EditCustomLanguagePackInfo>>(&self, edit_custom_language_pack_info: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, edit_custom_language_pack_info.as_ref()).await
  }

  pub async fn edit_inline_message_caption<C: AsRef<EditInlineMessageCaption>>(&self, edit_inline_message_caption: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, edit_inline_message_caption.as_ref()).await
  }

  pub async fn edit_inline_message_live_location<C: AsRef<EditInlineMessageLiveLocation>>(&self, edit_inline_message_live_location: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, edit_inline_message_live_location.as_ref()).await
  }

  pub async fn edit_inline_message_media<C: AsRef<EditInlineMessageMedia>>(&self, edit_inline_message_media: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, edit_inline_message_media.as_ref()).await
  }

  pub async fn edit_inline_message_reply_markup<C: AsRef<EditInlineMessageReplyMarkup>>(&self, edit_inline_message_reply_markup: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, edit_inline_message_reply_markup.as_ref()).await
  }

  pub async fn edit_inline_message_text<C: AsRef<EditInlineMessageText>>(&self, edit_inline_message_text: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, edit_inline_message_text.as_ref()).await
  }

  pub async fn edit_message_caption<C: AsRef<EditMessageCaption>>(&self, edit_message_caption: C) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, edit_message_caption.as_ref()).await
  }

  pub async fn edit_message_live_location<C: AsRef<EditMessageLiveLocation>>(&self, edit_message_live_location: C) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, edit_message_live_location.as_ref()).await
  }

  pub async fn edit_message_media<C: AsRef<EditMessageMedia>>(&self, edit_message_media: C) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, edit_message_media.as_ref()).await
  }

  pub async fn edit_message_reply_markup<C: AsRef<EditMessageReplyMarkup>>(&self, edit_message_reply_markup: C) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, edit_message_reply_markup.as_ref()).await
  }

  pub async fn edit_message_text<C: AsRef<EditMessageText>>(&self, edit_message_text: C) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, edit_message_text.as_ref()).await
  }

  pub async fn edit_proxy<C: AsRef<EditProxy>>(&self, edit_proxy: C) -> RTDResult<Proxy> {
    async_caller!(Proxy);
    async_caller(&self.api, edit_proxy.as_ref()).await
  }

  pub async fn enable_proxy<C: AsRef<EnableProxy>>(&self, enable_proxy: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, enable_proxy.as_ref()).await
  }

  pub async fn finish_file_generation<C: AsRef<FinishFileGeneration>>(&self, finish_file_generation: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, finish_file_generation.as_ref()).await
  }

  pub async fn forward_messages<C: AsRef<ForwardMessages>>(&self, forward_messages: C) -> RTDResult<Messages> {
    async_caller!(Messages);
    async_caller(&self.api, forward_messages.as_ref()).await
  }

  pub async fn generate_chat_invite_link<C: AsRef<GenerateChatInviteLink>>(&self, generate_chat_invite_link: C) -> RTDResult<ChatInviteLink> {
    async_caller!(ChatInviteLink);
    async_caller(&self.api, generate_chat_invite_link.as_ref()).await
  }

  pub async fn get_account_ttl<C: AsRef<GetAccountTtl>>(&self, get_account_ttl: C) -> RTDResult<AccountTtl> {
    async_caller!(AccountTtl);
    async_caller(&self.api, get_account_ttl.as_ref()).await
  }

  pub async fn get_active_live_location_messages<C: AsRef<GetActiveLiveLocationMessages>>(&self, get_active_live_location_messages: C) -> RTDResult<Messages> {
    async_caller!(Messages);
    async_caller(&self.api, get_active_live_location_messages.as_ref()).await
  }

  pub async fn get_active_sessions<C: AsRef<GetActiveSessions>>(&self, get_active_sessions: C) -> RTDResult<Sessions> {
    async_caller!(Sessions);
    async_caller(&self.api, get_active_sessions.as_ref()).await
  }

  pub async fn get_all_passport_elements<C: AsRef<GetAllPassportElements>>(&self, get_all_passport_elements: C) -> RTDResult<PassportElements> {
    async_caller!(PassportElements);
    async_caller(&self.api, get_all_passport_elements.as_ref()).await
  }

  pub async fn get_application_config<C: AsRef<GetApplicationConfig>>(&self, get_application_config: C) -> RTDResult<JsonValue> {
    async_caller!(JsonValue);
    async_caller(&self.api, get_application_config.as_ref()).await
  }

  pub async fn get_archived_sticker_sets<C: AsRef<GetArchivedStickerSets>>(&self, get_archived_sticker_sets: C) -> RTDResult<StickerSets> {
    async_caller!(StickerSets);
    async_caller(&self.api, get_archived_sticker_sets.as_ref()).await
  }

  pub async fn get_attached_sticker_sets<C: AsRef<GetAttachedStickerSets>>(&self, get_attached_sticker_sets: C) -> RTDResult<StickerSets> {
    async_caller!(StickerSets);
    async_caller(&self.api, get_attached_sticker_sets.as_ref()).await
  }

  pub async fn get_authorization_state<C: AsRef<GetAuthorizationState>>(&self, get_authorization_state: C) -> RTDResult<AuthorizationState> {
    async_caller!(AuthorizationState);
    async_caller(&self.api, get_authorization_state.as_ref()).await
  }

  pub async fn get_auto_download_settings_presets<C: AsRef<GetAutoDownloadSettingsPresets>>(&self, get_auto_download_settings_presets: C) -> RTDResult<AutoDownloadSettingsPresets> {
    async_caller!(AutoDownloadSettingsPresets);
    async_caller(&self.api, get_auto_download_settings_presets.as_ref()).await
  }

  pub async fn get_background_url<C: AsRef<GetBackgroundUrl>>(&self, get_background_url: C) -> RTDResult<HttpUrl> {
    async_caller!(HttpUrl);
    async_caller(&self.api, get_background_url.as_ref()).await
  }

  pub async fn get_backgrounds<C: AsRef<GetBackgrounds>>(&self, get_backgrounds: C) -> RTDResult<Backgrounds> {
    async_caller!(Backgrounds);
    async_caller(&self.api, get_backgrounds.as_ref()).await
  }

  pub async fn get_basic_group<C: AsRef<GetBasicGroup>>(&self, get_basic_group: C) -> RTDResult<BasicGroup> {
    async_caller!(BasicGroup);
    async_caller(&self.api, get_basic_group.as_ref()).await
  }

  pub async fn get_basic_group_full_info<C: AsRef<GetBasicGroupFullInfo>>(&self, get_basic_group_full_info: C) -> RTDResult<BasicGroupFullInfo> {
    async_caller!(BasicGroupFullInfo);
    async_caller(&self.api, get_basic_group_full_info.as_ref()).await
  }

  pub async fn get_blocked_users<C: AsRef<GetBlockedUsers>>(&self, get_blocked_users: C) -> RTDResult<Users> {
    async_caller!(Users);
    async_caller(&self.api, get_blocked_users.as_ref()).await
  }

  pub async fn get_callback_query_answer<C: AsRef<GetCallbackQueryAnswer>>(&self, get_callback_query_answer: C) -> RTDResult<CallbackQueryAnswer> {
    async_caller!(CallbackQueryAnswer);
    async_caller(&self.api, get_callback_query_answer.as_ref()).await
  }

  pub async fn get_chat<C: AsRef<GetChat>>(&self, get_chat: C) -> RTDResult<Chat> {
    async_caller!(Chat);
    async_caller(&self.api, get_chat.as_ref()).await
  }

  pub async fn get_chat_administrators<C: AsRef<GetChatAdministrators>>(&self, get_chat_administrators: C) -> RTDResult<Users> {
    async_caller!(Users);
    async_caller(&self.api, get_chat_administrators.as_ref()).await
  }

  pub async fn get_chat_event_log<C: AsRef<GetChatEventLog>>(&self, get_chat_event_log: C) -> RTDResult<ChatEvents> {
    async_caller!(ChatEvents);
    async_caller(&self.api, get_chat_event_log.as_ref()).await
  }

  pub async fn get_chat_history<C: AsRef<GetChatHistory>>(&self, get_chat_history: C) -> RTDResult<Messages> {
    async_caller!(Messages);
    async_caller(&self.api, get_chat_history.as_ref()).await
  }

  pub async fn get_chat_member<C: AsRef<GetChatMember>>(&self, get_chat_member: C) -> RTDResult<ChatMember> {
    async_caller!(ChatMember);
    async_caller(&self.api, get_chat_member.as_ref()).await
  }

  pub async fn get_chat_message_by_date<C: AsRef<GetChatMessageByDate>>(&self, get_chat_message_by_date: C) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, get_chat_message_by_date.as_ref()).await
  }

  pub async fn get_chat_message_count<C: AsRef<GetChatMessageCount>>(&self, get_chat_message_count: C) -> RTDResult<Count> {
    async_caller!(Count);
    async_caller(&self.api, get_chat_message_count.as_ref()).await
  }

  pub async fn get_chat_notification_settings_exceptions<C: AsRef<GetChatNotificationSettingsExceptions>>(&self, get_chat_notification_settings_exceptions: C) -> RTDResult<Chats> {
    async_caller!(Chats);
    async_caller(&self.api, get_chat_notification_settings_exceptions.as_ref()).await
  }

  pub async fn get_chat_pinned_message<C: AsRef<GetChatPinnedMessage>>(&self, get_chat_pinned_message: C) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, get_chat_pinned_message.as_ref()).await
  }

  pub async fn get_chat_report_spam_state<C: AsRef<GetChatReportSpamState>>(&self, get_chat_report_spam_state: C) -> RTDResult<ChatReportSpamState> {
    async_caller!(ChatReportSpamState);
    async_caller(&self.api, get_chat_report_spam_state.as_ref()).await
  }

  pub async fn get_chat_statistics_url<C: AsRef<GetChatStatisticsUrl>>(&self, get_chat_statistics_url: C) -> RTDResult<HttpUrl> {
    async_caller!(HttpUrl);
    async_caller(&self.api, get_chat_statistics_url.as_ref()).await
  }

  pub async fn get_chats<C: AsRef<GetChats>>(&self, get_chats: C) -> RTDResult<Chats> {
    async_caller!(Chats);
    async_caller(&self.api, get_chats.as_ref()).await
  }

  pub async fn get_connected_websites<C: AsRef<GetConnectedWebsites>>(&self, get_connected_websites: C) -> RTDResult<ConnectedWebsites> {
    async_caller!(ConnectedWebsites);
    async_caller(&self.api, get_connected_websites.as_ref()).await
  }

  pub async fn get_contacts<C: AsRef<GetContacts>>(&self, get_contacts: C) -> RTDResult<Users> {
    async_caller!(Users);
    async_caller(&self.api, get_contacts.as_ref()).await
  }

  pub async fn get_country_code<C: AsRef<GetCountryCode>>(&self, get_country_code: C) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, get_country_code.as_ref()).await
  }

  pub async fn get_created_public_chats<C: AsRef<GetCreatedPublicChats>>(&self, get_created_public_chats: C) -> RTDResult<Chats> {
    async_caller!(Chats);
    async_caller(&self.api, get_created_public_chats.as_ref()).await
  }

  pub async fn get_current_state<C: AsRef<GetCurrentState>>(&self, get_current_state: C) -> RTDResult<Updates> {
    async_caller!(Updates);
    async_caller(&self.api, get_current_state.as_ref()).await
  }

  pub async fn get_database_statistics<C: AsRef<GetDatabaseStatistics>>(&self, get_database_statistics: C) -> RTDResult<DatabaseStatistics> {
    async_caller!(DatabaseStatistics);
    async_caller(&self.api, get_database_statistics.as_ref()).await
  }

  pub async fn get_deep_link_info<C: AsRef<GetDeepLinkInfo>>(&self, get_deep_link_info: C) -> RTDResult<DeepLinkInfo> {
    async_caller!(DeepLinkInfo);
    async_caller(&self.api, get_deep_link_info.as_ref()).await
  }

  pub async fn get_emoji_suggestions_url<C: AsRef<GetEmojiSuggestionsUrl>>(&self, get_emoji_suggestions_url: C) -> RTDResult<HttpUrl> {
    async_caller!(HttpUrl);
    async_caller(&self.api, get_emoji_suggestions_url.as_ref()).await
  }

  pub async fn get_favorite_stickers<C: AsRef<GetFavoriteStickers>>(&self, get_favorite_stickers: C) -> RTDResult<Stickers> {
    async_caller!(Stickers);
    async_caller(&self.api, get_favorite_stickers.as_ref()).await
  }

  pub async fn get_file<C: AsRef<GetFile>>(&self, get_file: C) -> RTDResult<File> {
    async_caller!(File);
    async_caller(&self.api, get_file.as_ref()).await
  }

  pub async fn get_file_downloaded_prefix_size<C: AsRef<GetFileDownloadedPrefixSize>>(&self, get_file_downloaded_prefix_size: C) -> RTDResult<Count> {
    async_caller!(Count);
    async_caller(&self.api, get_file_downloaded_prefix_size.as_ref()).await
  }

  pub async fn get_file_extension<C: AsRef<GetFileExtension>>(&self, get_file_extension: C) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, get_file_extension.as_ref()).await
  }

  pub async fn get_file_mime_type<C: AsRef<GetFileMimeType>>(&self, get_file_mime_type: C) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, get_file_mime_type.as_ref()).await
  }

  pub async fn get_game_high_scores<C: AsRef<GetGameHighScores>>(&self, get_game_high_scores: C) -> RTDResult<GameHighScores> {
    async_caller!(GameHighScores);
    async_caller(&self.api, get_game_high_scores.as_ref()).await
  }

  pub async fn get_groups_in_common<C: AsRef<GetGroupsInCommon>>(&self, get_groups_in_common: C) -> RTDResult<Chats> {
    async_caller!(Chats);
    async_caller(&self.api, get_groups_in_common.as_ref()).await
  }

  pub async fn get_imported_contact_count<C: AsRef<GetImportedContactCount>>(&self, get_imported_contact_count: C) -> RTDResult<Count> {
    async_caller!(Count);
    async_caller(&self.api, get_imported_contact_count.as_ref()).await
  }

  pub async fn get_inline_game_high_scores<C: AsRef<GetInlineGameHighScores>>(&self, get_inline_game_high_scores: C) -> RTDResult<GameHighScores> {
    async_caller!(GameHighScores);
    async_caller(&self.api, get_inline_game_high_scores.as_ref()).await
  }

  pub async fn get_inline_query_results<C: AsRef<GetInlineQueryResults>>(&self, get_inline_query_results: C) -> RTDResult<InlineQueryResults> {
    async_caller!(InlineQueryResults);
    async_caller(&self.api, get_inline_query_results.as_ref()).await
  }

  pub async fn get_installed_sticker_sets<C: AsRef<GetInstalledStickerSets>>(&self, get_installed_sticker_sets: C) -> RTDResult<StickerSets> {
    async_caller!(StickerSets);
    async_caller(&self.api, get_installed_sticker_sets.as_ref()).await
  }

  pub async fn get_invite_text<C: AsRef<GetInviteText>>(&self, get_invite_text: C) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, get_invite_text.as_ref()).await
  }

  pub async fn get_json_string<C: AsRef<GetJsonString>>(&self, get_json_string: C) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, get_json_string.as_ref()).await
  }

  pub async fn get_json_value<C: AsRef<GetJsonValue>>(&self, get_json_value: C) -> RTDResult<JsonValue> {
    async_caller!(JsonValue);
    async_caller(&self.api, get_json_value.as_ref()).await
  }

  pub async fn get_language_pack_info<C: AsRef<GetLanguagePackInfo>>(&self, get_language_pack_info: C) -> RTDResult<LanguagePackInfo> {
    async_caller!(LanguagePackInfo);
    async_caller(&self.api, get_language_pack_info.as_ref()).await
  }

  pub async fn get_language_pack_string<C: AsRef<GetLanguagePackString>>(&self, get_language_pack_string: C) -> RTDResult<LanguagePackStringValue> {
    async_caller!(LanguagePackStringValue);
    async_caller(&self.api, get_language_pack_string.as_ref()).await
  }

  pub async fn get_language_pack_strings<C: AsRef<GetLanguagePackStrings>>(&self, get_language_pack_strings: C) -> RTDResult<LanguagePackStrings> {
    async_caller!(LanguagePackStrings);
    async_caller(&self.api, get_language_pack_strings.as_ref()).await
  }

  pub async fn get_localization_target_info<C: AsRef<GetLocalizationTargetInfo>>(&self, get_localization_target_info: C) -> RTDResult<LocalizationTargetInfo> {
    async_caller!(LocalizationTargetInfo);
    async_caller(&self.api, get_localization_target_info.as_ref()).await
  }

  pub async fn get_log_stream<C: AsRef<GetLogStream>>(&self, get_log_stream: C) -> RTDResult<LogStream> {
    async_caller!(LogStream);
    async_caller(&self.api, get_log_stream.as_ref()).await
  }

  pub async fn get_log_tag_verbosity_level<C: AsRef<GetLogTagVerbosityLevel>>(&self, get_log_tag_verbosity_level: C) -> RTDResult<LogVerbosityLevel> {
    async_caller!(LogVerbosityLevel);
    async_caller(&self.api, get_log_tag_verbosity_level.as_ref()).await
  }

  pub async fn get_log_tags<C: AsRef<GetLogTags>>(&self, get_log_tags: C) -> RTDResult<LogTags> {
    async_caller!(LogTags);
    async_caller(&self.api, get_log_tags.as_ref()).await
  }

  pub async fn get_log_verbosity_level<C: AsRef<GetLogVerbosityLevel>>(&self, get_log_verbosity_level: C) -> RTDResult<LogVerbosityLevel> {
    async_caller!(LogVerbosityLevel);
    async_caller(&self.api, get_log_verbosity_level.as_ref()).await
  }

  pub async fn get_map_thumbnail_file<C: AsRef<GetMapThumbnailFile>>(&self, get_map_thumbnail_file: C) -> RTDResult<File> {
    async_caller!(File);
    async_caller(&self.api, get_map_thumbnail_file.as_ref()).await
  }

  pub async fn get_me<C: AsRef<GetMe>>(&self, get_me: C) -> RTDResult<User> {
    async_caller!(User);
    async_caller(&self.api, get_me.as_ref()).await
  }

  pub async fn get_message<C: AsRef<GetMessage>>(&self, get_message: C) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, get_message.as_ref()).await
  }

  pub async fn get_message_link<C: AsRef<GetMessageLink>>(&self, get_message_link: C) -> RTDResult<HttpUrl> {
    async_caller!(HttpUrl);
    async_caller(&self.api, get_message_link.as_ref()).await
  }

  pub async fn get_message_link_info<C: AsRef<GetMessageLinkInfo>>(&self, get_message_link_info: C) -> RTDResult<MessageLinkInfo> {
    async_caller!(MessageLinkInfo);
    async_caller(&self.api, get_message_link_info.as_ref()).await
  }

  pub async fn get_message_locally<C: AsRef<GetMessageLocally>>(&self, get_message_locally: C) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, get_message_locally.as_ref()).await
  }

  pub async fn get_messages<C: AsRef<GetMessages>>(&self, get_messages: C) -> RTDResult<Messages> {
    async_caller!(Messages);
    async_caller(&self.api, get_messages.as_ref()).await
  }

  pub async fn get_network_statistics<C: AsRef<GetNetworkStatistics>>(&self, get_network_statistics: C) -> RTDResult<NetworkStatistics> {
    async_caller!(NetworkStatistics);
    async_caller(&self.api, get_network_statistics.as_ref()).await
  }

  pub async fn get_option<C: AsRef<GetOption>>(&self, get_option: C) -> RTDResult<OptionValue> {
    async_caller!(OptionValue);
    async_caller(&self.api, get_option.as_ref()).await
  }

  pub async fn get_passport_authorization_form<C: AsRef<GetPassportAuthorizationForm>>(&self, get_passport_authorization_form: C) -> RTDResult<PassportAuthorizationForm> {
    async_caller!(PassportAuthorizationForm);
    async_caller(&self.api, get_passport_authorization_form.as_ref()).await
  }

  pub async fn get_passport_authorization_form_available_elements<C: AsRef<GetPassportAuthorizationFormAvailableElements>>(&self, get_passport_authorization_form_available_elements: C) -> RTDResult<PassportElementsWithErrors> {
    async_caller!(PassportElementsWithErrors);
    async_caller(&self.api, get_passport_authorization_form_available_elements.as_ref()).await
  }

  pub async fn get_passport_element<C: AsRef<GetPassportElement>>(&self, get_passport_element: C) -> RTDResult<PassportElement> {
    async_caller!(PassportElement);
    async_caller(&self.api, get_passport_element.as_ref()).await
  }

  pub async fn get_password_state<C: AsRef<GetPasswordState>>(&self, get_password_state: C) -> RTDResult<PasswordState> {
    async_caller!(PasswordState);
    async_caller(&self.api, get_password_state.as_ref()).await
  }

  pub async fn get_payment_form<C: AsRef<GetPaymentForm>>(&self, get_payment_form: C) -> RTDResult<PaymentForm> {
    async_caller!(PaymentForm);
    async_caller(&self.api, get_payment_form.as_ref()).await
  }

  pub async fn get_payment_receipt<C: AsRef<GetPaymentReceipt>>(&self, get_payment_receipt: C) -> RTDResult<PaymentReceipt> {
    async_caller!(PaymentReceipt);
    async_caller(&self.api, get_payment_receipt.as_ref()).await
  }

  pub async fn get_preferred_country_language<C: AsRef<GetPreferredCountryLanguage>>(&self, get_preferred_country_language: C) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, get_preferred_country_language.as_ref()).await
  }

  pub async fn get_proxies<C: AsRef<GetProxies>>(&self, get_proxies: C) -> RTDResult<Proxies> {
    async_caller!(Proxies);
    async_caller(&self.api, get_proxies.as_ref()).await
  }

  pub async fn get_proxy_link<C: AsRef<GetProxyLink>>(&self, get_proxy_link: C) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, get_proxy_link.as_ref()).await
  }

  pub async fn get_public_message_link<C: AsRef<GetPublicMessageLink>>(&self, get_public_message_link: C) -> RTDResult<PublicMessageLink> {
    async_caller!(PublicMessageLink);
    async_caller(&self.api, get_public_message_link.as_ref()).await
  }

  pub async fn get_push_receiver_id<C: AsRef<GetPushReceiverId>>(&self, get_push_receiver_id: C) -> RTDResult<PushReceiverId> {
    async_caller!(PushReceiverId);
    async_caller(&self.api, get_push_receiver_id.as_ref()).await
  }

  pub async fn get_recent_inline_bots<C: AsRef<GetRecentInlineBots>>(&self, get_recent_inline_bots: C) -> RTDResult<Users> {
    async_caller!(Users);
    async_caller(&self.api, get_recent_inline_bots.as_ref()).await
  }

  pub async fn get_recent_stickers<C: AsRef<GetRecentStickers>>(&self, get_recent_stickers: C) -> RTDResult<Stickers> {
    async_caller!(Stickers);
    async_caller(&self.api, get_recent_stickers.as_ref()).await
  }

  pub async fn get_recently_visited_t_me_urls<C: AsRef<GetRecentlyVisitedTMeUrls>>(&self, get_recently_visited_t_me_urls: C) -> RTDResult<TMeUrls> {
    async_caller!(TMeUrls);
    async_caller(&self.api, get_recently_visited_t_me_urls.as_ref()).await
  }

  pub async fn get_recovery_email_address<C: AsRef<GetRecoveryEmailAddress>>(&self, get_recovery_email_address: C) -> RTDResult<RecoveryEmailAddress> {
    async_caller!(RecoveryEmailAddress);
    async_caller(&self.api, get_recovery_email_address.as_ref()).await
  }

  pub async fn get_remote_file<C: AsRef<GetRemoteFile>>(&self, get_remote_file: C) -> RTDResult<File> {
    async_caller!(File);
    async_caller(&self.api, get_remote_file.as_ref()).await
  }

  pub async fn get_replied_message<C: AsRef<GetRepliedMessage>>(&self, get_replied_message: C) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, get_replied_message.as_ref()).await
  }

  pub async fn get_saved_animations<C: AsRef<GetSavedAnimations>>(&self, get_saved_animations: C) -> RTDResult<Animations> {
    async_caller!(Animations);
    async_caller(&self.api, get_saved_animations.as_ref()).await
  }

  pub async fn get_saved_order_info<C: AsRef<GetSavedOrderInfo>>(&self, get_saved_order_info: C) -> RTDResult<OrderInfo> {
    async_caller!(OrderInfo);
    async_caller(&self.api, get_saved_order_info.as_ref()).await
  }

  pub async fn get_scope_notification_settings<C: AsRef<GetScopeNotificationSettings>>(&self, get_scope_notification_settings: C) -> RTDResult<ScopeNotificationSettings> {
    async_caller!(ScopeNotificationSettings);
    async_caller(&self.api, get_scope_notification_settings.as_ref()).await
  }

  pub async fn get_secret_chat<C: AsRef<GetSecretChat>>(&self, get_secret_chat: C) -> RTDResult<SecretChat> {
    async_caller!(SecretChat);
    async_caller(&self.api, get_secret_chat.as_ref()).await
  }

  pub async fn get_sticker_emojis<C: AsRef<GetStickerEmojis>>(&self, get_sticker_emojis: C) -> RTDResult<Emojis> {
    async_caller!(Emojis);
    async_caller(&self.api, get_sticker_emojis.as_ref()).await
  }

  pub async fn get_sticker_set<C: AsRef<GetStickerSet>>(&self, get_sticker_set: C) -> RTDResult<StickerSet> {
    async_caller!(StickerSet);
    async_caller(&self.api, get_sticker_set.as_ref()).await
  }

  pub async fn get_stickers<C: AsRef<GetStickers>>(&self, get_stickers: C) -> RTDResult<Stickers> {
    async_caller!(Stickers);
    async_caller(&self.api, get_stickers.as_ref()).await
  }

  pub async fn get_storage_statistics<C: AsRef<GetStorageStatistics>>(&self, get_storage_statistics: C) -> RTDResult<StorageStatistics> {
    async_caller!(StorageStatistics);
    async_caller(&self.api, get_storage_statistics.as_ref()).await
  }

  pub async fn get_storage_statistics_fast<C: AsRef<GetStorageStatisticsFast>>(&self, get_storage_statistics_fast: C) -> RTDResult<StorageStatisticsFast> {
    async_caller!(StorageStatisticsFast);
    async_caller(&self.api, get_storage_statistics_fast.as_ref()).await
  }

  pub async fn get_supergroup<C: AsRef<GetSupergroup>>(&self, get_supergroup: C) -> RTDResult<Supergroup> {
    async_caller!(Supergroup);
    async_caller(&self.api, get_supergroup.as_ref()).await
  }

  pub async fn get_supergroup_full_info<C: AsRef<GetSupergroupFullInfo>>(&self, get_supergroup_full_info: C) -> RTDResult<SupergroupFullInfo> {
    async_caller!(SupergroupFullInfo);
    async_caller(&self.api, get_supergroup_full_info.as_ref()).await
  }

  pub async fn get_supergroup_members<C: AsRef<GetSupergroupMembers>>(&self, get_supergroup_members: C) -> RTDResult<ChatMembers> {
    async_caller!(ChatMembers);
    async_caller(&self.api, get_supergroup_members.as_ref()).await
  }

  pub async fn get_support_user<C: AsRef<GetSupportUser>>(&self, get_support_user: C) -> RTDResult<User> {
    async_caller!(User);
    async_caller(&self.api, get_support_user.as_ref()).await
  }

  pub async fn get_temporary_password_state<C: AsRef<GetTemporaryPasswordState>>(&self, get_temporary_password_state: C) -> RTDResult<TemporaryPasswordState> {
    async_caller!(TemporaryPasswordState);
    async_caller(&self.api, get_temporary_password_state.as_ref()).await
  }

  pub async fn get_text_entities<C: AsRef<GetTextEntities>>(&self, get_text_entities: C) -> RTDResult<TextEntities> {
    async_caller!(TextEntities);
    async_caller(&self.api, get_text_entities.as_ref()).await
  }

  pub async fn get_top_chats<C: AsRef<GetTopChats>>(&self, get_top_chats: C) -> RTDResult<Chats> {
    async_caller!(Chats);
    async_caller(&self.api, get_top_chats.as_ref()).await
  }

  pub async fn get_trending_sticker_sets<C: AsRef<GetTrendingStickerSets>>(&self, get_trending_sticker_sets: C) -> RTDResult<StickerSets> {
    async_caller!(StickerSets);
    async_caller(&self.api, get_trending_sticker_sets.as_ref()).await
  }

  pub async fn get_user<C: AsRef<GetUser>>(&self, get_user: C) -> RTDResult<User> {
    async_caller!(User);
    async_caller(&self.api, get_user.as_ref()).await
  }

  pub async fn get_user_full_info<C: AsRef<GetUserFullInfo>>(&self, get_user_full_info: C) -> RTDResult<UserFullInfo> {
    async_caller!(UserFullInfo);
    async_caller(&self.api, get_user_full_info.as_ref()).await
  }

  pub async fn get_user_privacy_setting_rules<C: AsRef<GetUserPrivacySettingRules>>(&self, get_user_privacy_setting_rules: C) -> RTDResult<UserPrivacySettingRules> {
    async_caller!(UserPrivacySettingRules);
    async_caller(&self.api, get_user_privacy_setting_rules.as_ref()).await
  }

  pub async fn get_user_profile_photos<C: AsRef<GetUserProfilePhotos>>(&self, get_user_profile_photos: C) -> RTDResult<UserProfilePhotos> {
    async_caller!(UserProfilePhotos);
    async_caller(&self.api, get_user_profile_photos.as_ref()).await
  }

  pub async fn get_web_page_instant_view<C: AsRef<GetWebPageInstantView>>(&self, get_web_page_instant_view: C) -> RTDResult<WebPageInstantView> {
    async_caller!(WebPageInstantView);
    async_caller(&self.api, get_web_page_instant_view.as_ref()).await
  }

  pub async fn get_web_page_preview<C: AsRef<GetWebPagePreview>>(&self, get_web_page_preview: C) -> RTDResult<WebPage> {
    async_caller!(WebPage);
    async_caller(&self.api, get_web_page_preview.as_ref()).await
  }

  pub async fn import_contacts<C: AsRef<ImportContacts>>(&self, import_contacts: C) -> RTDResult<ImportedContacts> {
    async_caller!(ImportedContacts);
    async_caller(&self.api, import_contacts.as_ref()).await
  }

  pub async fn join_chat<C: AsRef<JoinChat>>(&self, join_chat: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, join_chat.as_ref()).await
  }

  pub async fn join_chat_by_invite_link<C: AsRef<JoinChatByInviteLink>>(&self, join_chat_by_invite_link: C) -> RTDResult<Chat> {
    async_caller!(Chat);
    async_caller(&self.api, join_chat_by_invite_link.as_ref()).await
  }

  pub async fn leave_chat<C: AsRef<LeaveChat>>(&self, leave_chat: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, leave_chat.as_ref()).await
  }

  pub async fn log_out<C: AsRef<LogOut>>(&self, log_out: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, log_out.as_ref()).await
  }

  pub async fn open_chat<C: AsRef<OpenChat>>(&self, open_chat: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, open_chat.as_ref()).await
  }

  pub async fn open_message_content<C: AsRef<OpenMessageContent>>(&self, open_message_content: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, open_message_content.as_ref()).await
  }

  pub async fn optimize_storage<C: AsRef<OptimizeStorage>>(&self, optimize_storage: C) -> RTDResult<StorageStatistics> {
    async_caller!(StorageStatistics);
    async_caller(&self.api, optimize_storage.as_ref()).await
  }

  pub async fn parse_text_entities<C: AsRef<ParseTextEntities>>(&self, parse_text_entities: C) -> RTDResult<FormattedText> {
    async_caller!(FormattedText);
    async_caller(&self.api, parse_text_entities.as_ref()).await
  }

  pub async fn pin_chat_message<C: AsRef<PinChatMessage>>(&self, pin_chat_message: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, pin_chat_message.as_ref()).await
  }

  pub async fn ping_proxy<C: AsRef<PingProxy>>(&self, ping_proxy: C) -> RTDResult<Seconds> {
    async_caller!(Seconds);
    async_caller(&self.api, ping_proxy.as_ref()).await
  }

  pub async fn process_push_notification<C: AsRef<ProcessPushNotification>>(&self, process_push_notification: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, process_push_notification.as_ref()).await
  }

  pub async fn read_all_chat_mentions<C: AsRef<ReadAllChatMentions>>(&self, read_all_chat_mentions: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, read_all_chat_mentions.as_ref()).await
  }

  pub async fn read_file_part<C: AsRef<ReadFilePart>>(&self, read_file_part: C) -> RTDResult<FilePart> {
    async_caller!(FilePart);
    async_caller(&self.api, read_file_part.as_ref()).await
  }

  pub async fn recover_authentication_password<C: AsRef<RecoverAuthenticationPassword>>(&self, recover_authentication_password: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, recover_authentication_password.as_ref()).await
  }

  pub async fn recover_password<C: AsRef<RecoverPassword>>(&self, recover_password: C) -> RTDResult<PasswordState> {
    async_caller!(PasswordState);
    async_caller(&self.api, recover_password.as_ref()).await
  }

  pub async fn register_device<C: AsRef<RegisterDevice>>(&self, register_device: C) -> RTDResult<PushReceiverId> {
    async_caller!(PushReceiverId);
    async_caller(&self.api, register_device.as_ref()).await
  }

  pub async fn register_user<C: AsRef<RegisterUser>>(&self, register_user: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, register_user.as_ref()).await
  }

  pub async fn remove_background<C: AsRef<RemoveBackground>>(&self, remove_background: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_background.as_ref()).await
  }

  pub async fn remove_contacts<C: AsRef<RemoveContacts>>(&self, remove_contacts: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_contacts.as_ref()).await
  }

  pub async fn remove_favorite_sticker<C: AsRef<RemoveFavoriteSticker>>(&self, remove_favorite_sticker: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_favorite_sticker.as_ref()).await
  }

  pub async fn remove_notification<C: AsRef<RemoveNotification>>(&self, remove_notification: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_notification.as_ref()).await
  }

  pub async fn remove_notification_group<C: AsRef<RemoveNotificationGroup>>(&self, remove_notification_group: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_notification_group.as_ref()).await
  }

  pub async fn remove_proxy<C: AsRef<RemoveProxy>>(&self, remove_proxy: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_proxy.as_ref()).await
  }

  pub async fn remove_recent_hashtag<C: AsRef<RemoveRecentHashtag>>(&self, remove_recent_hashtag: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_recent_hashtag.as_ref()).await
  }

  pub async fn remove_recent_sticker<C: AsRef<RemoveRecentSticker>>(&self, remove_recent_sticker: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_recent_sticker.as_ref()).await
  }

  pub async fn remove_recently_found_chat<C: AsRef<RemoveRecentlyFoundChat>>(&self, remove_recently_found_chat: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_recently_found_chat.as_ref()).await
  }

  pub async fn remove_saved_animation<C: AsRef<RemoveSavedAnimation>>(&self, remove_saved_animation: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_saved_animation.as_ref()).await
  }

  pub async fn remove_sticker_from_set<C: AsRef<RemoveStickerFromSet>>(&self, remove_sticker_from_set: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_sticker_from_set.as_ref()).await
  }

  pub async fn remove_top_chat<C: AsRef<RemoveTopChat>>(&self, remove_top_chat: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_top_chat.as_ref()).await
  }

  pub async fn reorder_installed_sticker_sets<C: AsRef<ReorderInstalledStickerSets>>(&self, reorder_installed_sticker_sets: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, reorder_installed_sticker_sets.as_ref()).await
  }

  pub async fn report_chat<C: AsRef<ReportChat>>(&self, report_chat: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, report_chat.as_ref()).await
  }

  pub async fn report_supergroup_spam<C: AsRef<ReportSupergroupSpam>>(&self, report_supergroup_spam: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, report_supergroup_spam.as_ref()).await
  }

  pub async fn request_authentication_password_recovery<C: AsRef<RequestAuthenticationPasswordRecovery>>(&self, request_authentication_password_recovery: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, request_authentication_password_recovery.as_ref()).await
  }

  pub async fn request_password_recovery<C: AsRef<RequestPasswordRecovery>>(&self, request_password_recovery: C) -> RTDResult<EmailAddressAuthenticationCodeInfo> {
    async_caller!(EmailAddressAuthenticationCodeInfo);
    async_caller(&self.api, request_password_recovery.as_ref()).await
  }

  pub async fn resend_authentication_code<C: AsRef<ResendAuthenticationCode>>(&self, resend_authentication_code: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, resend_authentication_code.as_ref()).await
  }

  pub async fn resend_change_phone_number_code<C: AsRef<ResendChangePhoneNumberCode>>(&self, resend_change_phone_number_code: C) -> RTDResult<AuthenticationCodeInfo> {
    async_caller!(AuthenticationCodeInfo);
    async_caller(&self.api, resend_change_phone_number_code.as_ref()).await
  }

  pub async fn resend_email_address_verification_code<C: AsRef<ResendEmailAddressVerificationCode>>(&self, resend_email_address_verification_code: C) -> RTDResult<EmailAddressAuthenticationCodeInfo> {
    async_caller!(EmailAddressAuthenticationCodeInfo);
    async_caller(&self.api, resend_email_address_verification_code.as_ref()).await
  }

  pub async fn resend_messages<C: AsRef<ResendMessages>>(&self, resend_messages: C) -> RTDResult<Messages> {
    async_caller!(Messages);
    async_caller(&self.api, resend_messages.as_ref()).await
  }

  pub async fn resend_phone_number_confirmation_code<C: AsRef<ResendPhoneNumberConfirmationCode>>(&self, resend_phone_number_confirmation_code: C) -> RTDResult<AuthenticationCodeInfo> {
    async_caller!(AuthenticationCodeInfo);
    async_caller(&self.api, resend_phone_number_confirmation_code.as_ref()).await
  }

  pub async fn resend_phone_number_verification_code<C: AsRef<ResendPhoneNumberVerificationCode>>(&self, resend_phone_number_verification_code: C) -> RTDResult<AuthenticationCodeInfo> {
    async_caller!(AuthenticationCodeInfo);
    async_caller(&self.api, resend_phone_number_verification_code.as_ref()).await
  }

  pub async fn resend_recovery_email_address_code<C: AsRef<ResendRecoveryEmailAddressCode>>(&self, resend_recovery_email_address_code: C) -> RTDResult<PasswordState> {
    async_caller!(PasswordState);
    async_caller(&self.api, resend_recovery_email_address_code.as_ref()).await
  }

  pub async fn reset_all_notification_settings<C: AsRef<ResetAllNotificationSettings>>(&self, reset_all_notification_settings: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, reset_all_notification_settings.as_ref()).await
  }

  pub async fn reset_backgrounds<C: AsRef<ResetBackgrounds>>(&self, reset_backgrounds: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, reset_backgrounds.as_ref()).await
  }

  pub async fn reset_network_statistics<C: AsRef<ResetNetworkStatistics>>(&self, reset_network_statistics: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, reset_network_statistics.as_ref()).await
  }

  pub async fn save_application_log_event<C: AsRef<SaveApplicationLogEvent>>(&self, save_application_log_event: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, save_application_log_event.as_ref()).await
  }

  pub async fn search_background<C: AsRef<SearchBackground>>(&self, search_background: C) -> RTDResult<Background> {
    async_caller!(Background);
    async_caller(&self.api, search_background.as_ref()).await
  }

  pub async fn search_call_messages<C: AsRef<SearchCallMessages>>(&self, search_call_messages: C) -> RTDResult<Messages> {
    async_caller!(Messages);
    async_caller(&self.api, search_call_messages.as_ref()).await
  }

  pub async fn search_chat_members<C: AsRef<SearchChatMembers>>(&self, search_chat_members: C) -> RTDResult<ChatMembers> {
    async_caller!(ChatMembers);
    async_caller(&self.api, search_chat_members.as_ref()).await
  }

  pub async fn search_chat_messages<C: AsRef<SearchChatMessages>>(&self, search_chat_messages: C) -> RTDResult<Messages> {
    async_caller!(Messages);
    async_caller(&self.api, search_chat_messages.as_ref()).await
  }

  pub async fn search_chat_recent_location_messages<C: AsRef<SearchChatRecentLocationMessages>>(&self, search_chat_recent_location_messages: C) -> RTDResult<Messages> {
    async_caller!(Messages);
    async_caller(&self.api, search_chat_recent_location_messages.as_ref()).await
  }

  pub async fn search_chats<C: AsRef<SearchChats>>(&self, search_chats: C) -> RTDResult<Chats> {
    async_caller!(Chats);
    async_caller(&self.api, search_chats.as_ref()).await
  }

  pub async fn search_chats_on_server<C: AsRef<SearchChatsOnServer>>(&self, search_chats_on_server: C) -> RTDResult<Chats> {
    async_caller!(Chats);
    async_caller(&self.api, search_chats_on_server.as_ref()).await
  }

  pub async fn search_contacts<C: AsRef<SearchContacts>>(&self, search_contacts: C) -> RTDResult<Users> {
    async_caller!(Users);
    async_caller(&self.api, search_contacts.as_ref()).await
  }

  pub async fn search_emojis<C: AsRef<SearchEmojis>>(&self, search_emojis: C) -> RTDResult<Emojis> {
    async_caller!(Emojis);
    async_caller(&self.api, search_emojis.as_ref()).await
  }

  pub async fn search_hashtags<C: AsRef<SearchHashtags>>(&self, search_hashtags: C) -> RTDResult<Hashtags> {
    async_caller!(Hashtags);
    async_caller(&self.api, search_hashtags.as_ref()).await
  }

  pub async fn search_installed_sticker_sets<C: AsRef<SearchInstalledStickerSets>>(&self, search_installed_sticker_sets: C) -> RTDResult<StickerSets> {
    async_caller!(StickerSets);
    async_caller(&self.api, search_installed_sticker_sets.as_ref()).await
  }

  pub async fn search_messages<C: AsRef<SearchMessages>>(&self, search_messages: C) -> RTDResult<Messages> {
    async_caller!(Messages);
    async_caller(&self.api, search_messages.as_ref()).await
  }

  pub async fn search_public_chat<C: AsRef<SearchPublicChat>>(&self, search_public_chat: C) -> RTDResult<Chat> {
    async_caller!(Chat);
    async_caller(&self.api, search_public_chat.as_ref()).await
  }

  pub async fn search_public_chats<C: AsRef<SearchPublicChats>>(&self, search_public_chats: C) -> RTDResult<Chats> {
    async_caller!(Chats);
    async_caller(&self.api, search_public_chats.as_ref()).await
  }

  pub async fn search_secret_messages<C: AsRef<SearchSecretMessages>>(&self, search_secret_messages: C) -> RTDResult<FoundMessages> {
    async_caller!(FoundMessages);
    async_caller(&self.api, search_secret_messages.as_ref()).await
  }

  pub async fn search_sticker_set<C: AsRef<SearchStickerSet>>(&self, search_sticker_set: C) -> RTDResult<StickerSet> {
    async_caller!(StickerSet);
    async_caller(&self.api, search_sticker_set.as_ref()).await
  }

  pub async fn search_sticker_sets<C: AsRef<SearchStickerSets>>(&self, search_sticker_sets: C) -> RTDResult<StickerSets> {
    async_caller!(StickerSets);
    async_caller(&self.api, search_sticker_sets.as_ref()).await
  }

  pub async fn search_stickers<C: AsRef<SearchStickers>>(&self, search_stickers: C) -> RTDResult<Stickers> {
    async_caller!(Stickers);
    async_caller(&self.api, search_stickers.as_ref()).await
  }

  pub async fn send_bot_start_message<C: AsRef<SendBotStartMessage>>(&self, send_bot_start_message: C) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, send_bot_start_message.as_ref()).await
  }

  pub async fn send_call_debug_information<C: AsRef<SendCallDebugInformation>>(&self, send_call_debug_information: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, send_call_debug_information.as_ref()).await
  }

  pub async fn send_call_rating<C: AsRef<SendCallRating>>(&self, send_call_rating: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, send_call_rating.as_ref()).await
  }

  pub async fn send_chat_action<C: AsRef<SendChatAction>>(&self, send_chat_action: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, send_chat_action.as_ref()).await
  }

  pub async fn send_chat_screenshot_taken_notification<C: AsRef<SendChatScreenshotTakenNotification>>(&self, send_chat_screenshot_taken_notification: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, send_chat_screenshot_taken_notification.as_ref()).await
  }

  pub async fn send_chat_set_ttl_message<C: AsRef<SendChatSetTtlMessage>>(&self, send_chat_set_ttl_message: C) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, send_chat_set_ttl_message.as_ref()).await
  }

  pub async fn send_custom_request<C: AsRef<SendCustomRequest>>(&self, send_custom_request: C) -> RTDResult<CustomRequestResult> {
    async_caller!(CustomRequestResult);
    async_caller(&self.api, send_custom_request.as_ref()).await
  }

  pub async fn send_email_address_verification_code<C: AsRef<SendEmailAddressVerificationCode>>(&self, send_email_address_verification_code: C) -> RTDResult<EmailAddressAuthenticationCodeInfo> {
    async_caller!(EmailAddressAuthenticationCodeInfo);
    async_caller(&self.api, send_email_address_verification_code.as_ref()).await
  }

  pub async fn send_inline_query_result_message<C: AsRef<SendInlineQueryResultMessage>>(&self, send_inline_query_result_message: C) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, send_inline_query_result_message.as_ref()).await
  }

  pub async fn send_message<C: AsRef<SendMessage>>(&self, send_message: C) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, send_message.as_ref()).await
  }

  pub async fn send_message_album<C: AsRef<SendMessageAlbum>>(&self, send_message_album: C) -> RTDResult<Messages> {
    async_caller!(Messages);
    async_caller(&self.api, send_message_album.as_ref()).await
  }

  pub async fn send_passport_authorization_form<C: AsRef<SendPassportAuthorizationForm>>(&self, send_passport_authorization_form: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, send_passport_authorization_form.as_ref()).await
  }

  pub async fn send_payment_form<C: AsRef<SendPaymentForm>>(&self, send_payment_form: C) -> RTDResult<PaymentResult> {
    async_caller!(PaymentResult);
    async_caller(&self.api, send_payment_form.as_ref()).await
  }

  pub async fn send_phone_number_confirmation_code<C: AsRef<SendPhoneNumberConfirmationCode>>(&self, send_phone_number_confirmation_code: C) -> RTDResult<AuthenticationCodeInfo> {
    async_caller!(AuthenticationCodeInfo);
    async_caller(&self.api, send_phone_number_confirmation_code.as_ref()).await
  }

  pub async fn send_phone_number_verification_code<C: AsRef<SendPhoneNumberVerificationCode>>(&self, send_phone_number_verification_code: C) -> RTDResult<AuthenticationCodeInfo> {
    async_caller!(AuthenticationCodeInfo);
    async_caller(&self.api, send_phone_number_verification_code.as_ref()).await
  }

  pub async fn set_account_ttl<C: AsRef<SetAccountTtl>>(&self, set_account_ttl: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_account_ttl.as_ref()).await
  }

  pub async fn set_alarm<C: AsRef<SetAlarm>>(&self, set_alarm: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_alarm.as_ref()).await
  }

  pub async fn set_authentication_phone_number<C: AsRef<SetAuthenticationPhoneNumber>>(&self, set_authentication_phone_number: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_authentication_phone_number.as_ref()).await
  }

  pub async fn set_auto_download_settings<C: AsRef<SetAutoDownloadSettings>>(&self, set_auto_download_settings: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_auto_download_settings.as_ref()).await
  }

  pub async fn set_background<C: AsRef<SetBackground>>(&self, set_background: C) -> RTDResult<Background> {
    async_caller!(Background);
    async_caller(&self.api, set_background.as_ref()).await
  }

  pub async fn set_bio<C: AsRef<SetBio>>(&self, set_bio: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_bio.as_ref()).await
  }

  pub async fn set_bot_updates_status<C: AsRef<SetBotUpdatesStatus>>(&self, set_bot_updates_status: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_bot_updates_status.as_ref()).await
  }

  pub async fn set_chat_client_data<C: AsRef<SetChatClientData>>(&self, set_chat_client_data: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_client_data.as_ref()).await
  }

  pub async fn set_chat_description<C: AsRef<SetChatDescription>>(&self, set_chat_description: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_description.as_ref()).await
  }

  pub async fn set_chat_draft_message<C: AsRef<SetChatDraftMessage>>(&self, set_chat_draft_message: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_draft_message.as_ref()).await
  }

  pub async fn set_chat_member_status<C: AsRef<SetChatMemberStatus>>(&self, set_chat_member_status: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_member_status.as_ref()).await
  }

  pub async fn set_chat_notification_settings<C: AsRef<SetChatNotificationSettings>>(&self, set_chat_notification_settings: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_notification_settings.as_ref()).await
  }

  pub async fn set_chat_permissions<C: AsRef<SetChatPermissions>>(&self, set_chat_permissions: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_permissions.as_ref()).await
  }

  pub async fn set_chat_photo<C: AsRef<SetChatPhoto>>(&self, set_chat_photo: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_photo.as_ref()).await
  }

  pub async fn set_chat_title<C: AsRef<SetChatTitle>>(&self, set_chat_title: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_title.as_ref()).await
  }

  pub async fn set_custom_language_pack<C: AsRef<SetCustomLanguagePack>>(&self, set_custom_language_pack: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_custom_language_pack.as_ref()).await
  }

  pub async fn set_custom_language_pack_string<C: AsRef<SetCustomLanguagePackString>>(&self, set_custom_language_pack_string: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_custom_language_pack_string.as_ref()).await
  }

  pub async fn set_database_encryption_key<C: AsRef<SetDatabaseEncryptionKey>>(&self, set_database_encryption_key: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_database_encryption_key.as_ref()).await
  }

  pub async fn set_file_generation_progress<C: AsRef<SetFileGenerationProgress>>(&self, set_file_generation_progress: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_file_generation_progress.as_ref()).await
  }

  pub async fn set_game_score<C: AsRef<SetGameScore>>(&self, set_game_score: C) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, set_game_score.as_ref()).await
  }

  pub async fn set_inline_game_score<C: AsRef<SetInlineGameScore>>(&self, set_inline_game_score: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_inline_game_score.as_ref()).await
  }

  pub async fn set_log_stream<C: AsRef<SetLogStream>>(&self, set_log_stream: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_log_stream.as_ref()).await
  }

  pub async fn set_log_tag_verbosity_level<C: AsRef<SetLogTagVerbosityLevel>>(&self, set_log_tag_verbosity_level: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_log_tag_verbosity_level.as_ref()).await
  }

  pub async fn set_log_verbosity_level<C: AsRef<SetLogVerbosityLevel>>(&self, set_log_verbosity_level: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_log_verbosity_level.as_ref()).await
  }

  pub async fn set_name<C: AsRef<SetName>>(&self, set_name: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_name.as_ref()).await
  }

  pub async fn set_network_type<C: AsRef<SetNetworkType>>(&self, set_network_type: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_network_type.as_ref()).await
  }

  pub async fn set_option<C: AsRef<SetOption>>(&self, set_option: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_option.as_ref()).await
  }

  pub async fn set_passport_element<C: AsRef<SetPassportElement>>(&self, set_passport_element: C) -> RTDResult<PassportElement> {
    async_caller!(PassportElement);
    async_caller(&self.api, set_passport_element.as_ref()).await
  }

  pub async fn set_passport_element_errors<C: AsRef<SetPassportElementErrors>>(&self, set_passport_element_errors: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_passport_element_errors.as_ref()).await
  }

  pub async fn set_password<C: AsRef<SetPassword>>(&self, set_password: C) -> RTDResult<PasswordState> {
    async_caller!(PasswordState);
    async_caller(&self.api, set_password.as_ref()).await
  }

  pub async fn set_pinned_chats<C: AsRef<SetPinnedChats>>(&self, set_pinned_chats: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_pinned_chats.as_ref()).await
  }

  pub async fn set_poll_answer<C: AsRef<SetPollAnswer>>(&self, set_poll_answer: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_poll_answer.as_ref()).await
  }

  pub async fn set_profile_photo<C: AsRef<SetProfilePhoto>>(&self, set_profile_photo: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_profile_photo.as_ref()).await
  }

  pub async fn set_recovery_email_address<C: AsRef<SetRecoveryEmailAddress>>(&self, set_recovery_email_address: C) -> RTDResult<PasswordState> {
    async_caller!(PasswordState);
    async_caller(&self.api, set_recovery_email_address.as_ref()).await
  }

  pub async fn set_scope_notification_settings<C: AsRef<SetScopeNotificationSettings>>(&self, set_scope_notification_settings: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_scope_notification_settings.as_ref()).await
  }

  pub async fn set_sticker_position_in_set<C: AsRef<SetStickerPositionInSet>>(&self, set_sticker_position_in_set: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_sticker_position_in_set.as_ref()).await
  }

  pub async fn set_supergroup_sticker_set<C: AsRef<SetSupergroupStickerSet>>(&self, set_supergroup_sticker_set: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_supergroup_sticker_set.as_ref()).await
  }

  pub async fn set_supergroup_username<C: AsRef<SetSupergroupUsername>>(&self, set_supergroup_username: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_supergroup_username.as_ref()).await
  }

  pub async fn set_tdlib_parameters<C: AsRef<SetTdlibParameters>>(&self, set_tdlib_parameters: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_tdlib_parameters.as_ref()).await
  }

  pub async fn set_user_privacy_setting_rules<C: AsRef<SetUserPrivacySettingRules>>(&self, set_user_privacy_setting_rules: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_user_privacy_setting_rules.as_ref()).await
  }

  pub async fn set_username<C: AsRef<SetUsername>>(&self, set_username: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_username.as_ref()).await
  }

  pub async fn stop_poll<C: AsRef<StopPoll>>(&self, stop_poll: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, stop_poll.as_ref()).await
  }

  pub async fn synchronize_language_pack<C: AsRef<SynchronizeLanguagePack>>(&self, synchronize_language_pack: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, synchronize_language_pack.as_ref()).await
  }

  pub async fn terminate_all_other_sessions<C: AsRef<TerminateAllOtherSessions>>(&self, terminate_all_other_sessions: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, terminate_all_other_sessions.as_ref()).await
  }

  pub async fn terminate_session<C: AsRef<TerminateSession>>(&self, terminate_session: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, terminate_session.as_ref()).await
  }

  pub async fn test_call_bytes<C: AsRef<TestCallBytes>>(&self, test_call_bytes: C) -> RTDResult<TestBytes> {
    async_caller!(TestBytes);
    async_caller(&self.api, test_call_bytes.as_ref()).await
  }

  pub async fn test_call_empty<C: AsRef<TestCallEmpty>>(&self, test_call_empty: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, test_call_empty.as_ref()).await
  }

  pub async fn test_call_string<C: AsRef<TestCallString>>(&self, test_call_string: C) -> RTDResult<TestString> {
    async_caller!(TestString);
    async_caller(&self.api, test_call_string.as_ref()).await
  }

  pub async fn test_call_vector_int<C: AsRef<TestCallVectorInt>>(&self, test_call_vector_int: C) -> RTDResult<TestVectorInt> {
    async_caller!(TestVectorInt);
    async_caller(&self.api, test_call_vector_int.as_ref()).await
  }

  pub async fn test_call_vector_int_object<C: AsRef<TestCallVectorIntObject>>(&self, test_call_vector_int_object: C) -> RTDResult<TestVectorIntObject> {
    async_caller!(TestVectorIntObject);
    async_caller(&self.api, test_call_vector_int_object.as_ref()).await
  }

  pub async fn test_call_vector_string<C: AsRef<TestCallVectorString>>(&self, test_call_vector_string: C) -> RTDResult<TestVectorString> {
    async_caller!(TestVectorString);
    async_caller(&self.api, test_call_vector_string.as_ref()).await
  }

  pub async fn test_call_vector_string_object<C: AsRef<TestCallVectorStringObject>>(&self, test_call_vector_string_object: C) -> RTDResult<TestVectorStringObject> {
    async_caller!(TestVectorStringObject);
    async_caller(&self.api, test_call_vector_string_object.as_ref()).await
  }

  pub async fn test_get_difference<C: AsRef<TestGetDifference>>(&self, test_get_difference: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, test_get_difference.as_ref()).await
  }

  pub async fn test_network<C: AsRef<TestNetwork>>(&self, test_network: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, test_network.as_ref()).await
  }

  pub async fn test_proxy<C: AsRef<TestProxy>>(&self, test_proxy: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, test_proxy.as_ref()).await
  }

  pub async fn test_return_error<C: AsRef<TestReturnError>>(&self, test_return_error: C) -> RTDResult<Error> {
    async_caller!(Error);
    async_caller(&self.api, test_return_error.as_ref()).await
  }

  pub async fn test_square_int<C: AsRef<TestSquareInt>>(&self, test_square_int: C) -> RTDResult<TestInt> {
    async_caller!(TestInt);
    async_caller(&self.api, test_square_int.as_ref()).await
  }

  pub async fn test_use_update<C: AsRef<TestUseUpdate>>(&self, test_use_update: C) -> RTDResult<Update> {
    async_caller!(Update);
    async_caller(&self.api, test_use_update.as_ref()).await
  }

  pub async fn toggle_chat_default_disable_notification<C: AsRef<ToggleChatDefaultDisableNotification>>(&self, toggle_chat_default_disable_notification: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_chat_default_disable_notification.as_ref()).await
  }

  pub async fn toggle_chat_is_marked_as_unread<C: AsRef<ToggleChatIsMarkedAsUnread>>(&self, toggle_chat_is_marked_as_unread: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_chat_is_marked_as_unread.as_ref()).await
  }

  pub async fn toggle_chat_is_pinned<C: AsRef<ToggleChatIsPinned>>(&self, toggle_chat_is_pinned: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_chat_is_pinned.as_ref()).await
  }

  pub async fn toggle_supergroup_is_all_history_available<C: AsRef<ToggleSupergroupIsAllHistoryAvailable>>(&self, toggle_supergroup_is_all_history_available: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_supergroup_is_all_history_available.as_ref()).await
  }

  pub async fn toggle_supergroup_sign_messages<C: AsRef<ToggleSupergroupSignMessages>>(&self, toggle_supergroup_sign_messages: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_supergroup_sign_messages.as_ref()).await
  }

  pub async fn unblock_user<C: AsRef<UnblockUser>>(&self, unblock_user: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, unblock_user.as_ref()).await
  }

  pub async fn unpin_chat_message<C: AsRef<UnpinChatMessage>>(&self, unpin_chat_message: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, unpin_chat_message.as_ref()).await
  }

  pub async fn upgrade_basic_group_chat_to_supergroup_chat<C: AsRef<UpgradeBasicGroupChatToSupergroupChat>>(&self, upgrade_basic_group_chat_to_supergroup_chat: C) -> RTDResult<Chat> {
    async_caller!(Chat);
    async_caller(&self.api, upgrade_basic_group_chat_to_supergroup_chat.as_ref()).await
  }

  pub async fn upload_file<C: AsRef<UploadFile>>(&self, upload_file: C) -> RTDResult<File> {
    async_caller!(File);
    async_caller(&self.api, upload_file.as_ref()).await
  }

  pub async fn upload_sticker_file<C: AsRef<UploadStickerFile>>(&self, upload_sticker_file: C) -> RTDResult<File> {
    async_caller!(File);
    async_caller(&self.api, upload_sticker_file.as_ref()).await
  }

  pub async fn validate_order_info<C: AsRef<ValidateOrderInfo>>(&self, validate_order_info: C) -> RTDResult<ValidatedOrderInfo> {
    async_caller!(ValidatedOrderInfo);
    async_caller(&self.api, validate_order_info.as_ref()).await
  }

  pub async fn view_messages<C: AsRef<ViewMessages>>(&self, view_messages: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, view_messages.as_ref()).await
  }

  pub async fn view_trending_sticker_sets<C: AsRef<ViewTrendingStickerSets>>(&self, view_trending_sticker_sets: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, view_trending_sticker_sets.as_ref()).await
  }

  pub async fn write_generated_file_part<C: AsRef<WriteGeneratedFilePart>>(&self, write_generated_file_part: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, write_generated_file_part.as_ref()).await
  }


}

