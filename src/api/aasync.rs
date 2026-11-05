// Generated from TDLib 1.8.69 td_api.tl (commit 4c6377aa7386c39217878a9a56d581922ad37ca4).
// Callback signatures are the public API.
#![allow(clippy::type_complexity)]

use futures::StreamExt;
use rtdlib::errors::{RTDError, RTDResult};
use rtdlib::types::*;

use crate::api::Api;
use crate::observer;

macro_rules! async_caller {
  ($td_type:ident) => {
    async fn async_caller<Fnc: RFunction>(api: &Api, fnc: Fnc) -> RTDResult<$td_type> {
      let extra = fnc.extra().ok_or(RTDError::Custom(
        "invalid tdjson response type, not have `extra` field".to_string(),
      ))?;
      let mut rec = observer::subscribe(&extra);
      api.send(&fnc)?;
      let val = rec.next().await;
      observer::unsubscribe(&extra);
      if let Some(TdType::Error(v)) = val {
        return Err(RTDError::custom(format!("[{}] {}", v.code(), v.message())));
      }
      match val {
        Some(TdType::$td_type(v)) => Ok(v),
        _ => Err(RTDError::custom(
          "invalid tdjson response type, unexpected `extra` field".to_string(),
        )),
      }
    }
  };
}

#[derive(Clone)]
pub struct AsyncApi {
  api: Api,
}

impl AsyncApi {
  pub fn new(api: Api) -> Self {
    Self { api }
  }

  #[doc(hidden)]
  pub fn api(&self) -> &Api {
    &self.api
  }

  pub async fn accept_call<C: AsRef<AcceptCall>>(&self, accept_call: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, accept_call.as_ref()).await
  }

  pub async fn accept_oauth_request<C: AsRef<AcceptOauthRequest>>(
    &self,
    accept_oauth_request: C,
  ) -> RTDResult<HttpUrl> {
    async_caller!(HttpUrl);
    async_caller(&self.api, accept_oauth_request.as_ref()).await
  }

  pub async fn accept_terms_of_service<C: AsRef<AcceptTermsOfService>>(
    &self,
    accept_terms_of_service: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, accept_terms_of_service.as_ref()).await
  }

  pub async fn activate_story_stealth_mode<C: AsRef<ActivateStoryStealthMode>>(
    &self,
    activate_story_stealth_mode: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, activate_story_stealth_mode.as_ref()).await
  }

  pub async fn add_bot_media_preview<C: AsRef<AddBotMediaPreview>>(
    &self,
    add_bot_media_preview: C,
  ) -> RTDResult<BotMediaPreview> {
    async_caller!(BotMediaPreview);
    async_caller(&self.api, add_bot_media_preview.as_ref()).await
  }

  pub async fn add_bot_secondary_username<C: AsRef<AddBotSecondaryUsername>>(
    &self,
    add_bot_secondary_username: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, add_bot_secondary_username.as_ref()).await
  }

  pub async fn add_chat_folder_by_invite_link<C: AsRef<AddChatFolderByInviteLink>>(
    &self,
    add_chat_folder_by_invite_link: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, add_chat_folder_by_invite_link.as_ref()).await
  }

  pub async fn add_chat_member<C: AsRef<AddChatMember>>(
    &self,
    add_chat_member: C,
  ) -> RTDResult<FailedToAddMembers> {
    async_caller!(FailedToAddMembers);
    async_caller(&self.api, add_chat_member.as_ref()).await
  }

  pub async fn add_chat_members<C: AsRef<AddChatMembers>>(
    &self,
    add_chat_members: C,
  ) -> RTDResult<FailedToAddMembers> {
    async_caller!(FailedToAddMembers);
    async_caller(&self.api, add_chat_members.as_ref()).await
  }

  pub async fn add_chat_to_list<C: AsRef<AddChatToList>>(
    &self,
    add_chat_to_list: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, add_chat_to_list.as_ref()).await
  }

  pub async fn add_chat_welcome_message<C: AsRef<AddChatWelcomeMessage>>(
    &self,
    add_chat_welcome_message: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, add_chat_welcome_message.as_ref()).await
  }

  pub async fn add_checklist_tasks<C: AsRef<AddChecklistTasks>>(
    &self,
    add_checklist_tasks: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, add_checklist_tasks.as_ref()).await
  }

  pub async fn add_contact<C: AsRef<AddContact>>(&self, add_contact: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, add_contact.as_ref()).await
  }

  pub async fn add_custom_server_language_pack<C: AsRef<AddCustomServerLanguagePack>>(
    &self,
    add_custom_server_language_pack: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, add_custom_server_language_pack.as_ref()).await
  }

  pub async fn add_favorite_sticker<C: AsRef<AddFavoriteSticker>>(
    &self,
    add_favorite_sticker: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, add_favorite_sticker.as_ref()).await
  }

  pub async fn add_file_to_downloads<C: AsRef<AddFileToDownloads>>(
    &self,
    add_file_to_downloads: C,
  ) -> RTDResult<File> {
    async_caller!(File);
    async_caller(&self.api, add_file_to_downloads.as_ref()).await
  }

  pub async fn add_gift_collection_gifts<C: AsRef<AddGiftCollectionGifts>>(
    &self,
    add_gift_collection_gifts: C,
  ) -> RTDResult<GiftCollection> {
    async_caller!(GiftCollection);
    async_caller(&self.api, add_gift_collection_gifts.as_ref()).await
  }

  pub async fn add_local_message<C: AsRef<AddLocalMessage>>(
    &self,
    add_local_message: C,
  ) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, add_local_message.as_ref()).await
  }

  pub async fn add_log_message<C: AsRef<AddLogMessage>>(
    &self,
    add_log_message: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, add_log_message.as_ref()).await
  }

  pub async fn add_login_passkey<C: AsRef<AddLoginPasskey>>(
    &self,
    add_login_passkey: C,
  ) -> RTDResult<Passkey> {
    async_caller!(Passkey);
    async_caller(&self.api, add_login_passkey.as_ref()).await
  }

  pub async fn add_message_reaction<C: AsRef<AddMessageReaction>>(
    &self,
    add_message_reaction: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, add_message_reaction.as_ref()).await
  }

  pub async fn add_network_statistics<C: AsRef<AddNetworkStatistics>>(
    &self,
    add_network_statistics: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, add_network_statistics.as_ref()).await
  }

  pub async fn add_offer<C: AsRef<AddOffer>>(&self, add_offer: C) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, add_offer.as_ref()).await
  }

  pub async fn add_pending_live_story_reaction<C: AsRef<AddPendingLiveStoryReaction>>(
    &self,
    add_pending_live_story_reaction: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, add_pending_live_story_reaction.as_ref()).await
  }

  pub async fn add_pending_paid_message_reaction<C: AsRef<AddPendingPaidMessageReaction>>(
    &self,
    add_pending_paid_message_reaction: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, add_pending_paid_message_reaction.as_ref()).await
  }

  pub async fn add_poll_option<C: AsRef<AddPollOption>>(
    &self,
    add_poll_option: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, add_poll_option.as_ref()).await
  }

  pub async fn add_profile_audio<C: AsRef<AddProfileAudio>>(
    &self,
    add_profile_audio: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, add_profile_audio.as_ref()).await
  }

  pub async fn add_proxy<C: AsRef<AddProxy>>(&self, add_proxy: C) -> RTDResult<AddedProxy> {
    async_caller!(AddedProxy);
    async_caller(&self.api, add_proxy.as_ref()).await
  }

  pub async fn add_quick_reply_shortcut_inline_query_result_message<
    C: AsRef<AddQuickReplyShortcutInlineQueryResultMessage>,
  >(
    &self,
    add_quick_reply_shortcut_inline_query_result_message: C,
  ) -> RTDResult<QuickReplyMessage> {
    async_caller!(QuickReplyMessage);
    async_caller(
      &self.api,
      add_quick_reply_shortcut_inline_query_result_message.as_ref(),
    )
    .await
  }

  pub async fn add_quick_reply_shortcut_message<C: AsRef<AddQuickReplyShortcutMessage>>(
    &self,
    add_quick_reply_shortcut_message: C,
  ) -> RTDResult<QuickReplyMessage> {
    async_caller!(QuickReplyMessage);
    async_caller(&self.api, add_quick_reply_shortcut_message.as_ref()).await
  }

  pub async fn add_quick_reply_shortcut_message_album<
    C: AsRef<AddQuickReplyShortcutMessageAlbum>,
  >(
    &self,
    add_quick_reply_shortcut_message_album: C,
  ) -> RTDResult<QuickReplyMessages> {
    async_caller!(QuickReplyMessages);
    async_caller(&self.api, add_quick_reply_shortcut_message_album.as_ref()).await
  }

  pub async fn add_recent_sticker<C: AsRef<AddRecentSticker>>(
    &self,
    add_recent_sticker: C,
  ) -> RTDResult<Stickers> {
    async_caller!(Stickers);
    async_caller(&self.api, add_recent_sticker.as_ref()).await
  }

  pub async fn add_recently_found_chat<C: AsRef<AddRecentlyFoundChat>>(
    &self,
    add_recently_found_chat: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, add_recently_found_chat.as_ref()).await
  }

  pub async fn add_saved_animation<C: AsRef<AddSavedAnimation>>(
    &self,
    add_saved_animation: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, add_saved_animation.as_ref()).await
  }

  pub async fn add_saved_notification_sound<C: AsRef<AddSavedNotificationSound>>(
    &self,
    add_saved_notification_sound: C,
  ) -> RTDResult<NotificationSound> {
    async_caller!(NotificationSound);
    async_caller(&self.api, add_saved_notification_sound.as_ref()).await
  }

  pub async fn add_sticker_to_set<C: AsRef<AddStickerToSet>>(
    &self,
    add_sticker_to_set: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, add_sticker_to_set.as_ref()).await
  }

  pub async fn add_story_album_stories<C: AsRef<AddStoryAlbumStories>>(
    &self,
    add_story_album_stories: C,
  ) -> RTDResult<StoryAlbum> {
    async_caller!(StoryAlbum);
    async_caller(&self.api, add_story_album_stories.as_ref()).await
  }

  pub async fn add_text_composition_style<C: AsRef<AddTextCompositionStyle>>(
    &self,
    add_text_composition_style: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, add_text_composition_style.as_ref()).await
  }

  pub async fn add_web_browser_settings_exception<C: AsRef<AddWebBrowserSettingsException>>(
    &self,
    add_web_browser_settings_exception: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, add_web_browser_settings_exception.as_ref()).await
  }

  pub async fn allow_bot_to_send_messages<C: AsRef<AllowBotToSendMessages>>(
    &self,
    allow_bot_to_send_messages: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, allow_bot_to_send_messages.as_ref()).await
  }

  pub async fn allow_unpaid_messages_from_user<C: AsRef<AllowUnpaidMessagesFromUser>>(
    &self,
    allow_unpaid_messages_from_user: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, allow_unpaid_messages_from_user.as_ref()).await
  }

  pub async fn answer_callback_query<C: AsRef<AnswerCallbackQuery>>(
    &self,
    answer_callback_query: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, answer_callback_query.as_ref()).await
  }

  pub async fn answer_chat_join_request_query<C: AsRef<AnswerChatJoinRequestQuery>>(
    &self,
    answer_chat_join_request_query: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, answer_chat_join_request_query.as_ref()).await
  }

  pub async fn answer_custom_query<C: AsRef<AnswerCustomQuery>>(
    &self,
    answer_custom_query: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, answer_custom_query.as_ref()).await
  }

  pub async fn answer_guest_query<C: AsRef<AnswerGuestQuery>>(
    &self,
    answer_guest_query: C,
  ) -> RTDResult<InlineMessageId> {
    async_caller!(InlineMessageId);
    async_caller(&self.api, answer_guest_query.as_ref()).await
  }

  pub async fn answer_inline_query<C: AsRef<AnswerInlineQuery>>(
    &self,
    answer_inline_query: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, answer_inline_query.as_ref()).await
  }

  pub async fn answer_pre_checkout_query<C: AsRef<AnswerPreCheckoutQuery>>(
    &self,
    answer_pre_checkout_query: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, answer_pre_checkout_query.as_ref()).await
  }

  pub async fn answer_shipping_query<C: AsRef<AnswerShippingQuery>>(
    &self,
    answer_shipping_query: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, answer_shipping_query.as_ref()).await
  }

  pub async fn answer_ton_connect_request<C: AsRef<AnswerTonConnectRequest>>(
    &self,
    answer_ton_connect_request: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, answer_ton_connect_request.as_ref()).await
  }

  pub async fn answer_web_app_query<C: AsRef<AnswerWebAppQuery>>(
    &self,
    answer_web_app_query: C,
  ) -> RTDResult<InlineMessageId> {
    async_caller!(InlineMessageId);
    async_caller(&self.api, answer_web_app_query.as_ref()).await
  }

  pub async fn apply_premium_gift_code<C: AsRef<ApplyPremiumGiftCode>>(
    &self,
    apply_premium_gift_code: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, apply_premium_gift_code.as_ref()).await
  }

  pub async fn apply_unpaid_messages_token<C: AsRef<ApplyUnpaidMessagesToken>>(
    &self,
    apply_unpaid_messages_token: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, apply_unpaid_messages_token.as_ref()).await
  }

  pub async fn approve_suggested_post<C: AsRef<ApproveSuggestedPost>>(
    &self,
    approve_suggested_post: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, approve_suggested_post.as_ref()).await
  }

  pub async fn assign_store_transaction<C: AsRef<AssignStoreTransaction>>(
    &self,
    assign_store_transaction: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, assign_store_transaction.as_ref()).await
  }

  pub async fn ban_chat_member<C: AsRef<BanChatMember>>(
    &self,
    ban_chat_member: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, ban_chat_member.as_ref()).await
  }

  pub async fn ban_group_call_participants<C: AsRef<BanGroupCallParticipants>>(
    &self,
    ban_group_call_participants: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, ban_group_call_participants.as_ref()).await
  }

  pub async fn block_message_sender_from_replies<C: AsRef<BlockMessageSenderFromReplies>>(
    &self,
    block_message_sender_from_replies: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, block_message_sender_from_replies.as_ref()).await
  }

  pub async fn boost_chat<C: AsRef<BoostChat>>(&self, boost_chat: C) -> RTDResult<ChatBoostSlots> {
    async_caller!(ChatBoostSlots);
    async_caller(&self.api, boost_chat.as_ref()).await
  }

  pub async fn buy_gift_upgrade<C: AsRef<BuyGiftUpgrade>>(
    &self,
    buy_gift_upgrade: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, buy_gift_upgrade.as_ref()).await
  }

  pub async fn can_bot_send_messages<C: AsRef<CanBotSendMessages>>(
    &self,
    can_bot_send_messages: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, can_bot_send_messages.as_ref()).await
  }

  pub async fn can_post_story<C: AsRef<CanPostStory>>(
    &self,
    can_post_story: C,
  ) -> RTDResult<CanPostStoryResult> {
    async_caller!(CanPostStoryResult);
    async_caller(&self.api, can_post_story.as_ref()).await
  }

  pub async fn can_purchase_from_store<C: AsRef<CanPurchaseFromStore>>(
    &self,
    can_purchase_from_store: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, can_purchase_from_store.as_ref()).await
  }

  pub async fn can_send_gift<C: AsRef<CanSendGift>>(
    &self,
    can_send_gift: C,
  ) -> RTDResult<CanSendGiftResult> {
    async_caller!(CanSendGiftResult);
    async_caller(&self.api, can_send_gift.as_ref()).await
  }

  pub async fn can_send_message_to_user<C: AsRef<CanSendMessageToUser>>(
    &self,
    can_send_message_to_user: C,
  ) -> RTDResult<CanSendMessageToUserResult> {
    async_caller!(CanSendMessageToUserResult);
    async_caller(&self.api, can_send_message_to_user.as_ref()).await
  }

  pub async fn can_transfer_ownership<C: AsRef<CanTransferOwnership>>(
    &self,
    can_transfer_ownership: C,
  ) -> RTDResult<CanTransferOwnershipResult> {
    async_caller!(CanTransferOwnershipResult);
    async_caller(&self.api, can_transfer_ownership.as_ref()).await
  }

  pub async fn cancel_download_file<C: AsRef<CancelDownloadFile>>(
    &self,
    cancel_download_file: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, cancel_download_file.as_ref()).await
  }

  pub async fn cancel_password_reset<C: AsRef<CancelPasswordReset>>(
    &self,
    cancel_password_reset: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, cancel_password_reset.as_ref()).await
  }

  pub async fn cancel_preliminary_upload_file<C: AsRef<CancelPreliminaryUploadFile>>(
    &self,
    cancel_preliminary_upload_file: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, cancel_preliminary_upload_file.as_ref()).await
  }

  pub async fn cancel_recovery_email_address_verification<
    C: AsRef<CancelRecoveryEmailAddressVerification>,
  >(
    &self,
    cancel_recovery_email_address_verification: C,
  ) -> RTDResult<PasswordState> {
    async_caller!(PasswordState);
    async_caller(
      &self.api,
      cancel_recovery_email_address_verification.as_ref(),
    )
    .await
  }

  pub async fn change_imported_contacts<C: AsRef<ChangeImportedContacts>>(
    &self,
    change_imported_contacts: C,
  ) -> RTDResult<ImportedContacts> {
    async_caller!(ImportedContacts);
    async_caller(&self.api, change_imported_contacts.as_ref()).await
  }

  pub async fn change_sticker_set<C: AsRef<ChangeStickerSet>>(
    &self,
    change_sticker_set: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, change_sticker_set.as_ref()).await
  }

  pub async fn change_web_browser_settings<C: AsRef<ChangeWebBrowserSettings>>(
    &self,
    change_web_browser_settings: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, change_web_browser_settings.as_ref()).await
  }

  pub async fn check_authentication_bot_token<C: AsRef<CheckAuthenticationBotToken>>(
    &self,
    check_authentication_bot_token: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, check_authentication_bot_token.as_ref()).await
  }

  pub async fn check_authentication_code<C: AsRef<CheckAuthenticationCode>>(
    &self,
    check_authentication_code: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, check_authentication_code.as_ref()).await
  }

  pub async fn check_authentication_email_code<C: AsRef<CheckAuthenticationEmailCode>>(
    &self,
    check_authentication_email_code: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, check_authentication_email_code.as_ref()).await
  }

  pub async fn check_authentication_passkey<C: AsRef<CheckAuthenticationPasskey>>(
    &self,
    check_authentication_passkey: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, check_authentication_passkey.as_ref()).await
  }

  pub async fn check_authentication_password<C: AsRef<CheckAuthenticationPassword>>(
    &self,
    check_authentication_password: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, check_authentication_password.as_ref()).await
  }

  pub async fn check_authentication_password_recovery_code<
    C: AsRef<CheckAuthenticationPasswordRecoveryCode>,
  >(
    &self,
    check_authentication_password_recovery_code: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(
      &self.api,
      check_authentication_password_recovery_code.as_ref(),
    )
    .await
  }

  pub async fn check_authentication_premium_purchase<
    C: AsRef<CheckAuthenticationPremiumPurchase>,
  >(
    &self,
    check_authentication_premium_purchase: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, check_authentication_premium_purchase.as_ref()).await
  }

  pub async fn check_authentication_web_token<C: AsRef<CheckAuthenticationWebToken>>(
    &self,
    check_authentication_web_token: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, check_authentication_web_token.as_ref()).await
  }

  pub async fn check_bot_username<C: AsRef<CheckBotUsername>>(
    &self,
    check_bot_username: C,
  ) -> RTDResult<CheckChatUsernameResult> {
    async_caller!(CheckChatUsernameResult);
    async_caller(&self.api, check_bot_username.as_ref()).await
  }

  pub async fn check_chat_folder_invite_link<C: AsRef<CheckChatFolderInviteLink>>(
    &self,
    check_chat_folder_invite_link: C,
  ) -> RTDResult<ChatFolderInviteLinkInfo> {
    async_caller!(ChatFolderInviteLinkInfo);
    async_caller(&self.api, check_chat_folder_invite_link.as_ref()).await
  }

  pub async fn check_chat_invite_link<C: AsRef<CheckChatInviteLink>>(
    &self,
    check_chat_invite_link: C,
  ) -> RTDResult<ChatInviteLinkInfo> {
    async_caller!(ChatInviteLinkInfo);
    async_caller(&self.api, check_chat_invite_link.as_ref()).await
  }

  pub async fn check_chat_username<C: AsRef<CheckChatUsername>>(
    &self,
    check_chat_username: C,
  ) -> RTDResult<CheckChatUsernameResult> {
    async_caller!(CheckChatUsernameResult);
    async_caller(&self.api, check_chat_username.as_ref()).await
  }

  pub async fn check_created_public_chats_limit<C: AsRef<CheckCreatedPublicChatsLimit>>(
    &self,
    check_created_public_chats_limit: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, check_created_public_chats_limit.as_ref()).await
  }

  pub async fn check_email_address_verification_code<
    C: AsRef<CheckEmailAddressVerificationCode>,
  >(
    &self,
    check_email_address_verification_code: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, check_email_address_verification_code.as_ref()).await
  }

  pub async fn check_login_email_address_code<C: AsRef<CheckLoginEmailAddressCode>>(
    &self,
    check_login_email_address_code: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, check_login_email_address_code.as_ref()).await
  }

  pub async fn check_oauth_request_match_code<C: AsRef<CheckOauthRequestMatchCode>>(
    &self,
    check_oauth_request_match_code: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, check_oauth_request_match_code.as_ref()).await
  }

  pub async fn check_password_recovery_code<C: AsRef<CheckPasswordRecoveryCode>>(
    &self,
    check_password_recovery_code: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, check_password_recovery_code.as_ref()).await
  }

  pub async fn check_phone_number_code<C: AsRef<CheckPhoneNumberCode>>(
    &self,
    check_phone_number_code: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, check_phone_number_code.as_ref()).await
  }

  pub async fn check_premium_gift_code<C: AsRef<CheckPremiumGiftCode>>(
    &self,
    check_premium_gift_code: C,
  ) -> RTDResult<PremiumGiftCodeInfo> {
    async_caller!(PremiumGiftCodeInfo);
    async_caller(&self.api, check_premium_gift_code.as_ref()).await
  }

  pub async fn check_quick_reply_shortcut_name<C: AsRef<CheckQuickReplyShortcutName>>(
    &self,
    check_quick_reply_shortcut_name: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, check_quick_reply_shortcut_name.as_ref()).await
  }

  pub async fn check_recovery_email_address_code<C: AsRef<CheckRecoveryEmailAddressCode>>(
    &self,
    check_recovery_email_address_code: C,
  ) -> RTDResult<PasswordState> {
    async_caller!(PasswordState);
    async_caller(&self.api, check_recovery_email_address_code.as_ref()).await
  }

  pub async fn check_sticker_set_name<C: AsRef<CheckStickerSetName>>(
    &self,
    check_sticker_set_name: C,
  ) -> RTDResult<CheckStickerSetNameResult> {
    async_caller!(CheckStickerSetNameResult);
    async_caller(&self.api, check_sticker_set_name.as_ref()).await
  }

  pub async fn check_wallet_bot_balance<C: AsRef<CheckWalletBotBalance>>(
    &self,
    check_wallet_bot_balance: C,
  ) -> RTDResult<WalletBotBalance> {
    async_caller!(WalletBotBalance);
    async_caller(&self.api, check_wallet_bot_balance.as_ref()).await
  }

  pub async fn check_web_app_file_download<C: AsRef<CheckWebAppFileDownload>>(
    &self,
    check_web_app_file_download: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, check_web_app_file_download.as_ref()).await
  }

  pub async fn claim_ton_connect_request<C: AsRef<ClaimTonConnectRequest>>(
    &self,
    claim_ton_connect_request: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, claim_ton_connect_request.as_ref()).await
  }

  pub async fn clean_file_name<C: AsRef<CleanFileName>>(
    &self,
    clean_file_name: C,
  ) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, clean_file_name.as_ref()).await
  }

  pub async fn clear_all_draft_messages<C: AsRef<ClearAllDraftMessages>>(
    &self,
    clear_all_draft_messages: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, clear_all_draft_messages.as_ref()).await
  }

  pub async fn clear_autosave_settings_exceptions<C: AsRef<ClearAutosaveSettingsExceptions>>(
    &self,
    clear_autosave_settings_exceptions: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, clear_autosave_settings_exceptions.as_ref()).await
  }

  pub async fn clear_imported_contacts<C: AsRef<ClearImportedContacts>>(
    &self,
    clear_imported_contacts: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, clear_imported_contacts.as_ref()).await
  }

  pub async fn clear_recent_emoji_statuses<C: AsRef<ClearRecentEmojiStatuses>>(
    &self,
    clear_recent_emoji_statuses: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, clear_recent_emoji_statuses.as_ref()).await
  }

  pub async fn clear_recent_reactions<C: AsRef<ClearRecentReactions>>(
    &self,
    clear_recent_reactions: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, clear_recent_reactions.as_ref()).await
  }

  pub async fn clear_recent_stickers<C: AsRef<ClearRecentStickers>>(
    &self,
    clear_recent_stickers: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, clear_recent_stickers.as_ref()).await
  }

  pub async fn clear_recently_found_chats<C: AsRef<ClearRecentlyFoundChats>>(
    &self,
    clear_recently_found_chats: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, clear_recently_found_chats.as_ref()).await
  }

  pub async fn clear_searched_for_tags<C: AsRef<ClearSearchedForTags>>(
    &self,
    clear_searched_for_tags: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, clear_searched_for_tags.as_ref()).await
  }

  pub async fn click_animated_emoji_message<C: AsRef<ClickAnimatedEmojiMessage>>(
    &self,
    click_animated_emoji_message: C,
  ) -> RTDResult<Sticker> {
    async_caller!(Sticker);
    async_caller(&self.api, click_animated_emoji_message.as_ref()).await
  }

  pub async fn click_chat_sponsored_message<C: AsRef<ClickChatSponsoredMessage>>(
    &self,
    click_chat_sponsored_message: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, click_chat_sponsored_message.as_ref()).await
  }

  pub async fn click_premium_subscription_button<C: AsRef<ClickPremiumSubscriptionButton>>(
    &self,
    click_premium_subscription_button: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, click_premium_subscription_button.as_ref()).await
  }

  pub async fn click_video_message_advertisement<C: AsRef<ClickVideoMessageAdvertisement>>(
    &self,
    click_video_message_advertisement: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, click_video_message_advertisement.as_ref()).await
  }

  pub async fn close<C: AsRef<Close>>(&self, close: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, close.as_ref()).await
  }

  pub async fn close_chat<C: AsRef<CloseChat>>(&self, close_chat: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, close_chat.as_ref()).await
  }

  pub async fn close_gift_auction<C: AsRef<CloseGiftAuction>>(
    &self,
    close_gift_auction: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, close_gift_auction.as_ref()).await
  }

  pub async fn close_secret_chat<C: AsRef<CloseSecretChat>>(
    &self,
    close_secret_chat: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, close_secret_chat.as_ref()).await
  }

  pub async fn close_story<C: AsRef<CloseStory>>(&self, close_story: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, close_story.as_ref()).await
  }

  pub async fn close_web_app<C: AsRef<CloseWebApp>>(&self, close_web_app: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, close_web_app.as_ref()).await
  }

  pub async fn commit_pending_live_story_reactions<C: AsRef<CommitPendingLiveStoryReactions>>(
    &self,
    commit_pending_live_story_reactions: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, commit_pending_live_story_reactions.as_ref()).await
  }

  pub async fn commit_pending_paid_message_reactions<
    C: AsRef<CommitPendingPaidMessageReactions>,
  >(
    &self,
    commit_pending_paid_message_reactions: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, commit_pending_paid_message_reactions.as_ref()).await
  }

  pub async fn compose_rich_message_with_ai<C: AsRef<ComposeRichMessageWithAi>>(
    &self,
    compose_rich_message_with_ai: C,
  ) -> RTDResult<RichMessage> {
    async_caller!(RichMessage);
    async_caller(&self.api, compose_rich_message_with_ai.as_ref()).await
  }

  pub async fn compose_text_with_ai<C: AsRef<ComposeTextWithAi>>(
    &self,
    compose_text_with_ai: C,
  ) -> RTDResult<FormattedText> {
    async_caller!(FormattedText);
    async_caller(&self.api, compose_text_with_ai.as_ref()).await
  }

  pub async fn confirm_business_connected_bot<C: AsRef<ConfirmBusinessConnectedBot>>(
    &self,
    confirm_business_connected_bot: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, confirm_business_connected_bot.as_ref()).await
  }

  pub async fn confirm_qr_code_authentication<C: AsRef<ConfirmQrCodeAuthentication>>(
    &self,
    confirm_qr_code_authentication: C,
  ) -> RTDResult<Session> {
    async_caller!(Session);
    async_caller(&self.api, confirm_qr_code_authentication.as_ref()).await
  }

  pub async fn confirm_session<C: AsRef<ConfirmSession>>(
    &self,
    confirm_session: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, confirm_session.as_ref()).await
  }

  pub async fn connect_affiliate_program<C: AsRef<ConnectAffiliateProgram>>(
    &self,
    connect_affiliate_program: C,
  ) -> RTDResult<ConnectedAffiliateProgram> {
    async_caller!(ConnectedAffiliateProgram);
    async_caller(&self.api, connect_affiliate_program.as_ref()).await
  }

  pub async fn craft_gift<C: AsRef<CraftGift>>(&self, craft_gift: C) -> RTDResult<CraftGiftResult> {
    async_caller!(CraftGiftResult);
    async_caller(&self.api, craft_gift.as_ref()).await
  }

  pub async fn create_basic_group_chat<C: AsRef<CreateBasicGroupChat>>(
    &self,
    create_basic_group_chat: C,
  ) -> RTDResult<Chat> {
    async_caller!(Chat);
    async_caller(&self.api, create_basic_group_chat.as_ref()).await
  }

  pub async fn create_bot<C: AsRef<CreateBot>>(&self, create_bot: C) -> RTDResult<User> {
    async_caller!(User);
    async_caller(&self.api, create_bot.as_ref()).await
  }

  pub async fn create_business_chat_link<C: AsRef<CreateBusinessChatLink>>(
    &self,
    create_business_chat_link: C,
  ) -> RTDResult<BusinessChatLink> {
    async_caller!(BusinessChatLink);
    async_caller(&self.api, create_business_chat_link.as_ref()).await
  }

  pub async fn create_call<C: AsRef<CreateCall>>(&self, create_call: C) -> RTDResult<CallId> {
    async_caller!(CallId);
    async_caller(&self.api, create_call.as_ref()).await
  }

  pub async fn create_chat_folder<C: AsRef<CreateChatFolder>>(
    &self,
    create_chat_folder: C,
  ) -> RTDResult<ChatFolderInfo> {
    async_caller!(ChatFolderInfo);
    async_caller(&self.api, create_chat_folder.as_ref()).await
  }

  pub async fn create_chat_folder_invite_link<C: AsRef<CreateChatFolderInviteLink>>(
    &self,
    create_chat_folder_invite_link: C,
  ) -> RTDResult<ChatFolderInviteLink> {
    async_caller!(ChatFolderInviteLink);
    async_caller(&self.api, create_chat_folder_invite_link.as_ref()).await
  }

  pub async fn create_chat_invite_link<C: AsRef<CreateChatInviteLink>>(
    &self,
    create_chat_invite_link: C,
  ) -> RTDResult<ChatInviteLink> {
    async_caller!(ChatInviteLink);
    async_caller(&self.api, create_chat_invite_link.as_ref()).await
  }

  pub async fn create_chat_subscription_invite_link<C: AsRef<CreateChatSubscriptionInviteLink>>(
    &self,
    create_chat_subscription_invite_link: C,
  ) -> RTDResult<ChatInviteLink> {
    async_caller!(ChatInviteLink);
    async_caller(&self.api, create_chat_subscription_invite_link.as_ref()).await
  }

  pub async fn create_community<C: AsRef<CreateCommunity>>(
    &self,
    create_community: C,
  ) -> RTDResult<CommunityId> {
    async_caller!(CommunityId);
    async_caller(&self.api, create_community.as_ref()).await
  }

  pub async fn create_forum_topic<C: AsRef<CreateForumTopic>>(
    &self,
    create_forum_topic: C,
  ) -> RTDResult<ForumTopicInfo> {
    async_caller!(ForumTopicInfo);
    async_caller(&self.api, create_forum_topic.as_ref()).await
  }

  pub async fn create_gift_collection<C: AsRef<CreateGiftCollection>>(
    &self,
    create_gift_collection: C,
  ) -> RTDResult<GiftCollection> {
    async_caller!(GiftCollection);
    async_caller(&self.api, create_gift_collection.as_ref()).await
  }

  pub async fn create_group_call<C: AsRef<CreateGroupCall>>(
    &self,
    create_group_call: C,
  ) -> RTDResult<GroupCallInfo> {
    async_caller!(GroupCallInfo);
    async_caller(&self.api, create_group_call.as_ref()).await
  }

  pub async fn create_invoice_link<C: AsRef<CreateInvoiceLink>>(
    &self,
    create_invoice_link: C,
  ) -> RTDResult<HttpUrl> {
    async_caller!(HttpUrl);
    async_caller(&self.api, create_invoice_link.as_ref()).await
  }

  pub async fn create_new_basic_group_chat<C: AsRef<CreateNewBasicGroupChat>>(
    &self,
    create_new_basic_group_chat: C,
  ) -> RTDResult<CreatedBasicGroupChat> {
    async_caller!(CreatedBasicGroupChat);
    async_caller(&self.api, create_new_basic_group_chat.as_ref()).await
  }

  pub async fn create_new_secret_chat<C: AsRef<CreateNewSecretChat>>(
    &self,
    create_new_secret_chat: C,
  ) -> RTDResult<Chat> {
    async_caller!(Chat);
    async_caller(&self.api, create_new_secret_chat.as_ref()).await
  }

  pub async fn create_new_sticker_set<C: AsRef<CreateNewStickerSet>>(
    &self,
    create_new_sticker_set: C,
  ) -> RTDResult<StickerSet> {
    async_caller!(StickerSet);
    async_caller(&self.api, create_new_sticker_set.as_ref()).await
  }

  pub async fn create_new_supergroup_chat<C: AsRef<CreateNewSupergroupChat>>(
    &self,
    create_new_supergroup_chat: C,
  ) -> RTDResult<Chat> {
    async_caller!(Chat);
    async_caller(&self.api, create_new_supergroup_chat.as_ref()).await
  }

  pub async fn create_on_ramp_payment_session<C: AsRef<CreateOnRampPaymentSession>>(
    &self,
    create_on_ramp_payment_session: C,
  ) -> RTDResult<OnRampPaymentSession> {
    async_caller!(OnRampPaymentSession);
    async_caller(&self.api, create_on_ramp_payment_session.as_ref()).await
  }

  pub async fn create_private_chat<C: AsRef<CreatePrivateChat>>(
    &self,
    create_private_chat: C,
  ) -> RTDResult<Chat> {
    async_caller!(Chat);
    async_caller(&self.api, create_private_chat.as_ref()).await
  }

  pub async fn create_rich_message_with_ai<C: AsRef<CreateRichMessageWithAi>>(
    &self,
    create_rich_message_with_ai: C,
  ) -> RTDResult<RichMessage> {
    async_caller!(RichMessage);
    async_caller(&self.api, create_rich_message_with_ai.as_ref()).await
  }

  pub async fn create_secret_chat<C: AsRef<CreateSecretChat>>(
    &self,
    create_secret_chat: C,
  ) -> RTDResult<Chat> {
    async_caller!(Chat);
    async_caller(&self.api, create_secret_chat.as_ref()).await
  }

  pub async fn create_story_album<C: AsRef<CreateStoryAlbum>>(
    &self,
    create_story_album: C,
  ) -> RTDResult<StoryAlbum> {
    async_caller!(StoryAlbum);
    async_caller(&self.api, create_story_album.as_ref()).await
  }

  pub async fn create_supergroup_chat<C: AsRef<CreateSupergroupChat>>(
    &self,
    create_supergroup_chat: C,
  ) -> RTDResult<Chat> {
    async_caller!(Chat);
    async_caller(&self.api, create_supergroup_chat.as_ref()).await
  }

  pub async fn create_temporary_password<C: AsRef<CreateTemporaryPassword>>(
    &self,
    create_temporary_password: C,
  ) -> RTDResult<TemporaryPasswordState> {
    async_caller!(TemporaryPasswordState);
    async_caller(&self.api, create_temporary_password.as_ref()).await
  }

  pub async fn create_text_composition_style<C: AsRef<CreateTextCompositionStyle>>(
    &self,
    create_text_composition_style: C,
  ) -> RTDResult<TextCompositionStyle> {
    async_caller!(TextCompositionStyle);
    async_caller(&self.api, create_text_composition_style.as_ref()).await
  }

  pub async fn create_ton_connect_session<C: AsRef<CreateTonConnectSession>>(
    &self,
    create_ton_connect_session: C,
  ) -> RTDResult<TonConnectSession> {
    async_caller!(TonConnectSession);
    async_caller(&self.api, create_ton_connect_session.as_ref()).await
  }

  pub async fn create_user_ton_wallet<C: AsRef<CreateUserTonWallet>>(
    &self,
    create_user_ton_wallet: C,
  ) -> RTDResult<UserTonWalletAddress> {
    async_caller!(UserTonWalletAddress);
    async_caller(&self.api, create_user_ton_wallet.as_ref()).await
  }

  pub async fn create_video_chat<C: AsRef<CreateVideoChat>>(
    &self,
    create_video_chat: C,
  ) -> RTDResult<GroupCallId> {
    async_caller!(GroupCallId);
    async_caller(&self.api, create_video_chat.as_ref()).await
  }

  pub async fn decline_group_call_invitation<C: AsRef<DeclineGroupCallInvitation>>(
    &self,
    decline_group_call_invitation: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, decline_group_call_invitation.as_ref()).await
  }

  pub async fn decline_oauth_request<C: AsRef<DeclineOauthRequest>>(
    &self,
    decline_oauth_request: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, decline_oauth_request.as_ref()).await
  }

  pub async fn decline_suggested_post<C: AsRef<DeclineSuggestedPost>>(
    &self,
    decline_suggested_post: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, decline_suggested_post.as_ref()).await
  }

  pub async fn decrypt_group_call_data<C: AsRef<DecryptGroupCallData>>(
    &self,
    decrypt_group_call_data: C,
  ) -> RTDResult<Data> {
    async_caller!(Data);
    async_caller(&self.api, decrypt_group_call_data.as_ref()).await
  }

  pub async fn delete_account<C: AsRef<DeleteAccount>>(&self, delete_account: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_account.as_ref()).await
  }

  pub async fn delete_all_call_messages<C: AsRef<DeleteAllCallMessages>>(
    &self,
    delete_all_call_messages: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_all_call_messages.as_ref()).await
  }

  pub async fn delete_all_chat_welcome_messages<C: AsRef<DeleteAllChatWelcomeMessages>>(
    &self,
    delete_all_chat_welcome_messages: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_all_chat_welcome_messages.as_ref()).await
  }

  pub async fn delete_all_recent_message_reactions_from_sender<
    C: AsRef<DeleteAllRecentMessageReactionsFromSender>,
  >(
    &self,
    delete_all_recent_message_reactions_from_sender: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(
      &self.api,
      delete_all_recent_message_reactions_from_sender.as_ref(),
    )
    .await
  }

  pub async fn delete_all_revoked_chat_invite_links<C: AsRef<DeleteAllRevokedChatInviteLinks>>(
    &self,
    delete_all_revoked_chat_invite_links: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_all_revoked_chat_invite_links.as_ref()).await
  }

  pub async fn delete_bot_media_previews<C: AsRef<DeleteBotMediaPreviews>>(
    &self,
    delete_bot_media_previews: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_bot_media_previews.as_ref()).await
  }

  pub async fn delete_bot_secondary_username<C: AsRef<DeleteBotSecondaryUsername>>(
    &self,
    delete_bot_secondary_username: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_bot_secondary_username.as_ref()).await
  }

  pub async fn delete_business_chat_link<C: AsRef<DeleteBusinessChatLink>>(
    &self,
    delete_business_chat_link: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_business_chat_link.as_ref()).await
  }

  pub async fn delete_business_connected_bot<C: AsRef<DeleteBusinessConnectedBot>>(
    &self,
    delete_business_connected_bot: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_business_connected_bot.as_ref()).await
  }

  pub async fn delete_business_messages<C: AsRef<DeleteBusinessMessages>>(
    &self,
    delete_business_messages: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_business_messages.as_ref()).await
  }

  pub async fn delete_business_story<C: AsRef<DeleteBusinessStory>>(
    &self,
    delete_business_story: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_business_story.as_ref()).await
  }

  pub async fn delete_chat<C: AsRef<DeleteChat>>(&self, delete_chat: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_chat.as_ref()).await
  }

  pub async fn delete_chat_background<C: AsRef<DeleteChatBackground>>(
    &self,
    delete_chat_background: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_chat_background.as_ref()).await
  }

  pub async fn delete_chat_folder<C: AsRef<DeleteChatFolder>>(
    &self,
    delete_chat_folder: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_chat_folder.as_ref()).await
  }

  pub async fn delete_chat_folder_invite_link<C: AsRef<DeleteChatFolderInviteLink>>(
    &self,
    delete_chat_folder_invite_link: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_chat_folder_invite_link.as_ref()).await
  }

  pub async fn delete_chat_history<C: AsRef<DeleteChatHistory>>(
    &self,
    delete_chat_history: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_chat_history.as_ref()).await
  }

  pub async fn delete_chat_messages_by_date<C: AsRef<DeleteChatMessagesByDate>>(
    &self,
    delete_chat_messages_by_date: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_chat_messages_by_date.as_ref()).await
  }

  pub async fn delete_chat_messages_by_sender<C: AsRef<DeleteChatMessagesBySender>>(
    &self,
    delete_chat_messages_by_sender: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_chat_messages_by_sender.as_ref()).await
  }

  pub async fn delete_chat_reply_markup<C: AsRef<DeleteChatReplyMarkup>>(
    &self,
    delete_chat_reply_markup: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_chat_reply_markup.as_ref()).await
  }

  pub async fn delete_chat_welcome_message<C: AsRef<DeleteChatWelcomeMessage>>(
    &self,
    delete_chat_welcome_message: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_chat_welcome_message.as_ref()).await
  }

  pub async fn delete_commands<C: AsRef<DeleteCommands>>(
    &self,
    delete_commands: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_commands.as_ref()).await
  }

  pub async fn delete_community<C: AsRef<DeleteCommunity>>(
    &self,
    delete_community: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_community.as_ref()).await
  }

  pub async fn delete_default_background<C: AsRef<DeleteDefaultBackground>>(
    &self,
    delete_default_background: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_default_background.as_ref()).await
  }

  pub async fn delete_direct_messages_chat_topic_history<
    C: AsRef<DeleteDirectMessagesChatTopicHistory>,
  >(
    &self,
    delete_direct_messages_chat_topic_history: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(
      &self.api,
      delete_direct_messages_chat_topic_history.as_ref(),
    )
    .await
  }

  pub async fn delete_direct_messages_chat_topic_messages_by_date<
    C: AsRef<DeleteDirectMessagesChatTopicMessagesByDate>,
  >(
    &self,
    delete_direct_messages_chat_topic_messages_by_date: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(
      &self.api,
      delete_direct_messages_chat_topic_messages_by_date.as_ref(),
    )
    .await
  }

  pub async fn delete_ephemeral_message<C: AsRef<DeleteEphemeralMessage>>(
    &self,
    delete_ephemeral_message: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_ephemeral_message.as_ref()).await
  }

  pub async fn delete_file<C: AsRef<DeleteFile>>(&self, delete_file: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_file.as_ref()).await
  }

  pub async fn delete_forum_topic<C: AsRef<DeleteForumTopic>>(
    &self,
    delete_forum_topic: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_forum_topic.as_ref()).await
  }

  pub async fn delete_gift<C: AsRef<DeleteGift>>(&self, delete_gift: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_gift.as_ref()).await
  }

  pub async fn delete_gift_collection<C: AsRef<DeleteGiftCollection>>(
    &self,
    delete_gift_collection: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_gift_collection.as_ref()).await
  }

  pub async fn delete_group_call_messages<C: AsRef<DeleteGroupCallMessages>>(
    &self,
    delete_group_call_messages: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_group_call_messages.as_ref()).await
  }

  pub async fn delete_group_call_messages_by_sender<C: AsRef<DeleteGroupCallMessagesBySender>>(
    &self,
    delete_group_call_messages_by_sender: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_group_call_messages_by_sender.as_ref()).await
  }

  pub async fn delete_language_pack<C: AsRef<DeleteLanguagePack>>(
    &self,
    delete_language_pack: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_language_pack.as_ref()).await
  }

  pub async fn delete_message_ephemeral_content<C: AsRef<DeleteMessageEphemeralContent>>(
    &self,
    delete_message_ephemeral_content: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_message_ephemeral_content.as_ref()).await
  }

  pub async fn delete_message_reactions_from_sender<C: AsRef<DeleteMessageReactionsFromSender>>(
    &self,
    delete_message_reactions_from_sender: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_message_reactions_from_sender.as_ref()).await
  }

  pub async fn delete_messages<C: AsRef<DeleteMessages>>(
    &self,
    delete_messages: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_messages.as_ref()).await
  }

  pub async fn delete_passport_element<C: AsRef<DeletePassportElement>>(
    &self,
    delete_passport_element: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_passport_element.as_ref()).await
  }

  pub async fn delete_poll_option<C: AsRef<DeletePollOption>>(
    &self,
    delete_poll_option: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_poll_option.as_ref()).await
  }

  pub async fn delete_profile_photo<C: AsRef<DeleteProfilePhoto>>(
    &self,
    delete_profile_photo: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_profile_photo.as_ref()).await
  }

  pub async fn delete_quick_reply_shortcut<C: AsRef<DeleteQuickReplyShortcut>>(
    &self,
    delete_quick_reply_shortcut: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_quick_reply_shortcut.as_ref()).await
  }

  pub async fn delete_quick_reply_shortcut_messages<C: AsRef<DeleteQuickReplyShortcutMessages>>(
    &self,
    delete_quick_reply_shortcut_messages: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_quick_reply_shortcut_messages.as_ref()).await
  }

  pub async fn delete_revoked_chat_invite_link<C: AsRef<DeleteRevokedChatInviteLink>>(
    &self,
    delete_revoked_chat_invite_link: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_revoked_chat_invite_link.as_ref()).await
  }

  pub async fn delete_saved_credentials<C: AsRef<DeleteSavedCredentials>>(
    &self,
    delete_saved_credentials: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_saved_credentials.as_ref()).await
  }

  pub async fn delete_saved_messages_topic_history<C: AsRef<DeleteSavedMessagesTopicHistory>>(
    &self,
    delete_saved_messages_topic_history: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_saved_messages_topic_history.as_ref()).await
  }

  pub async fn delete_saved_messages_topic_messages_by_date<
    C: AsRef<DeleteSavedMessagesTopicMessagesByDate>,
  >(
    &self,
    delete_saved_messages_topic_messages_by_date: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(
      &self.api,
      delete_saved_messages_topic_messages_by_date.as_ref(),
    )
    .await
  }

  pub async fn delete_saved_order_info<C: AsRef<DeleteSavedOrderInfo>>(
    &self,
    delete_saved_order_info: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_saved_order_info.as_ref()).await
  }

  pub async fn delete_sticker_set<C: AsRef<DeleteStickerSet>>(
    &self,
    delete_sticker_set: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_sticker_set.as_ref()).await
  }

  pub async fn delete_story<C: AsRef<DeleteStory>>(&self, delete_story: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_story.as_ref()).await
  }

  pub async fn delete_story_album<C: AsRef<DeleteStoryAlbum>>(
    &self,
    delete_story_album: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_story_album.as_ref()).await
  }

  pub async fn delete_text_composition_style<C: AsRef<DeleteTextCompositionStyle>>(
    &self,
    delete_text_composition_style: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_text_composition_style.as_ref()).await
  }

  pub async fn delete_ton_wallet<C: AsRef<DeleteTonWallet>>(
    &self,
    delete_ton_wallet: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, delete_ton_wallet.as_ref()).await
  }

  pub async fn destroy<C: AsRef<Destroy>>(&self, destroy: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, destroy.as_ref()).await
  }

  pub async fn disable_all_supergroup_usernames<C: AsRef<DisableAllSupergroupUsernames>>(
    &self,
    disable_all_supergroup_usernames: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, disable_all_supergroup_usernames.as_ref()).await
  }

  pub async fn disable_proxy<C: AsRef<DisableProxy>>(&self, disable_proxy: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, disable_proxy.as_ref()).await
  }

  pub async fn disable_ton_wallet_backup<C: AsRef<DisableTonWalletBackup>>(
    &self,
    disable_ton_wallet_backup: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, disable_ton_wallet_backup.as_ref()).await
  }

  pub async fn disable_ton_wallet_backup_with_proof<C: AsRef<DisableTonWalletBackupWithProof>>(
    &self,
    disable_ton_wallet_backup_with_proof: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, disable_ton_wallet_backup_with_proof.as_ref()).await
  }

  pub async fn discard_call<C: AsRef<DiscardCall>>(&self, discard_call: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, discard_call.as_ref()).await
  }

  pub async fn disconnect_affiliate_program<C: AsRef<DisconnectAffiliateProgram>>(
    &self,
    disconnect_affiliate_program: C,
  ) -> RTDResult<ConnectedAffiliateProgram> {
    async_caller!(ConnectedAffiliateProgram);
    async_caller(&self.api, disconnect_affiliate_program.as_ref()).await
  }

  pub async fn disconnect_all_websites<C: AsRef<DisconnectAllWebsites>>(
    &self,
    disconnect_all_websites: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, disconnect_all_websites.as_ref()).await
  }

  pub async fn disconnect_ton_connect_session<C: AsRef<DisconnectTonConnectSession>>(
    &self,
    disconnect_ton_connect_session: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, disconnect_ton_connect_session.as_ref()).await
  }

  pub async fn disconnect_website<C: AsRef<DisconnectWebsite>>(
    &self,
    disconnect_website: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, disconnect_website.as_ref()).await
  }

  pub async fn dismiss_web_token<C: AsRef<DismissWebToken>>(
    &self,
    dismiss_web_token: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, dismiss_web_token.as_ref()).await
  }

  pub async fn display_upgraded_gift<C: AsRef<DisplayUpgradedGift>>(
    &self,
    display_upgraded_gift: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, display_upgraded_gift.as_ref()).await
  }

  pub async fn download_file<C: AsRef<DownloadFile>>(&self, download_file: C) -> RTDResult<File> {
    async_caller!(File);
    async_caller(&self.api, download_file.as_ref()).await
  }

  pub async fn drop_gift_original_details<C: AsRef<DropGiftOriginalDetails>>(
    &self,
    drop_gift_original_details: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, drop_gift_original_details.as_ref()).await
  }

  pub async fn edit_bot_media_preview<C: AsRef<EditBotMediaPreview>>(
    &self,
    edit_bot_media_preview: C,
  ) -> RTDResult<BotMediaPreview> {
    async_caller!(BotMediaPreview);
    async_caller(&self.api, edit_bot_media_preview.as_ref()).await
  }

  pub async fn edit_business_chat_link<C: AsRef<EditBusinessChatLink>>(
    &self,
    edit_business_chat_link: C,
  ) -> RTDResult<BusinessChatLink> {
    async_caller!(BusinessChatLink);
    async_caller(&self.api, edit_business_chat_link.as_ref()).await
  }

  pub async fn edit_business_message_caption<C: AsRef<EditBusinessMessageCaption>>(
    &self,
    edit_business_message_caption: C,
  ) -> RTDResult<BusinessMessage> {
    async_caller!(BusinessMessage);
    async_caller(&self.api, edit_business_message_caption.as_ref()).await
  }

  pub async fn edit_business_message_checklist<C: AsRef<EditBusinessMessageChecklist>>(
    &self,
    edit_business_message_checklist: C,
  ) -> RTDResult<BusinessMessage> {
    async_caller!(BusinessMessage);
    async_caller(&self.api, edit_business_message_checklist.as_ref()).await
  }

  pub async fn edit_business_message_live_location<C: AsRef<EditBusinessMessageLiveLocation>>(
    &self,
    edit_business_message_live_location: C,
  ) -> RTDResult<BusinessMessage> {
    async_caller!(BusinessMessage);
    async_caller(&self.api, edit_business_message_live_location.as_ref()).await
  }

  pub async fn edit_business_message_media<C: AsRef<EditBusinessMessageMedia>>(
    &self,
    edit_business_message_media: C,
  ) -> RTDResult<BusinessMessage> {
    async_caller!(BusinessMessage);
    async_caller(&self.api, edit_business_message_media.as_ref()).await
  }

  pub async fn edit_business_message_reply_markup<C: AsRef<EditBusinessMessageReplyMarkup>>(
    &self,
    edit_business_message_reply_markup: C,
  ) -> RTDResult<BusinessMessage> {
    async_caller!(BusinessMessage);
    async_caller(&self.api, edit_business_message_reply_markup.as_ref()).await
  }

  pub async fn edit_business_message_text<C: AsRef<EditBusinessMessageText>>(
    &self,
    edit_business_message_text: C,
  ) -> RTDResult<BusinessMessage> {
    async_caller!(BusinessMessage);
    async_caller(&self.api, edit_business_message_text.as_ref()).await
  }

  pub async fn edit_business_story<C: AsRef<EditBusinessStory>>(
    &self,
    edit_business_story: C,
  ) -> RTDResult<Story> {
    async_caller!(Story);
    async_caller(&self.api, edit_business_story.as_ref()).await
  }

  pub async fn edit_chat_folder<C: AsRef<EditChatFolder>>(
    &self,
    edit_chat_folder: C,
  ) -> RTDResult<ChatFolderInfo> {
    async_caller!(ChatFolderInfo);
    async_caller(&self.api, edit_chat_folder.as_ref()).await
  }

  pub async fn edit_chat_folder_invite_link<C: AsRef<EditChatFolderInviteLink>>(
    &self,
    edit_chat_folder_invite_link: C,
  ) -> RTDResult<ChatFolderInviteLink> {
    async_caller!(ChatFolderInviteLink);
    async_caller(&self.api, edit_chat_folder_invite_link.as_ref()).await
  }

  pub async fn edit_chat_invite_link<C: AsRef<EditChatInviteLink>>(
    &self,
    edit_chat_invite_link: C,
  ) -> RTDResult<ChatInviteLink> {
    async_caller!(ChatInviteLink);
    async_caller(&self.api, edit_chat_invite_link.as_ref()).await
  }

  pub async fn edit_chat_subscription_invite_link<C: AsRef<EditChatSubscriptionInviteLink>>(
    &self,
    edit_chat_subscription_invite_link: C,
  ) -> RTDResult<ChatInviteLink> {
    async_caller!(ChatInviteLink);
    async_caller(&self.api, edit_chat_subscription_invite_link.as_ref()).await
  }

  pub async fn edit_chat_welcome_message<C: AsRef<EditChatWelcomeMessage>>(
    &self,
    edit_chat_welcome_message: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, edit_chat_welcome_message.as_ref()).await
  }

  pub async fn edit_custom_language_pack_info<C: AsRef<EditCustomLanguagePackInfo>>(
    &self,
    edit_custom_language_pack_info: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, edit_custom_language_pack_info.as_ref()).await
  }

  pub async fn edit_ephemeral_message<C: AsRef<EditEphemeralMessage>>(
    &self,
    edit_ephemeral_message: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, edit_ephemeral_message.as_ref()).await
  }

  pub async fn edit_ephemeral_message_caption<C: AsRef<EditEphemeralMessageCaption>>(
    &self,
    edit_ephemeral_message_caption: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, edit_ephemeral_message_caption.as_ref()).await
  }

  pub async fn edit_forum_topic<C: AsRef<EditForumTopic>>(
    &self,
    edit_forum_topic: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, edit_forum_topic.as_ref()).await
  }

  pub async fn edit_inline_message_caption<C: AsRef<EditInlineMessageCaption>>(
    &self,
    edit_inline_message_caption: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, edit_inline_message_caption.as_ref()).await
  }

  pub async fn edit_inline_message_live_location<C: AsRef<EditInlineMessageLiveLocation>>(
    &self,
    edit_inline_message_live_location: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, edit_inline_message_live_location.as_ref()).await
  }

  pub async fn edit_inline_message_media<C: AsRef<EditInlineMessageMedia>>(
    &self,
    edit_inline_message_media: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, edit_inline_message_media.as_ref()).await
  }

  pub async fn edit_inline_message_reply_markup<C: AsRef<EditInlineMessageReplyMarkup>>(
    &self,
    edit_inline_message_reply_markup: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, edit_inline_message_reply_markup.as_ref()).await
  }

  pub async fn edit_inline_message_text<C: AsRef<EditInlineMessageText>>(
    &self,
    edit_inline_message_text: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, edit_inline_message_text.as_ref()).await
  }

  pub async fn edit_message_caption<C: AsRef<EditMessageCaption>>(
    &self,
    edit_message_caption: C,
  ) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, edit_message_caption.as_ref()).await
  }

  pub async fn edit_message_checklist<C: AsRef<EditMessageChecklist>>(
    &self,
    edit_message_checklist: C,
  ) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, edit_message_checklist.as_ref()).await
  }

  pub async fn edit_message_live_location<C: AsRef<EditMessageLiveLocation>>(
    &self,
    edit_message_live_location: C,
  ) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, edit_message_live_location.as_ref()).await
  }

  pub async fn edit_message_media<C: AsRef<EditMessageMedia>>(
    &self,
    edit_message_media: C,
  ) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, edit_message_media.as_ref()).await
  }

  pub async fn edit_message_reply_markup<C: AsRef<EditMessageReplyMarkup>>(
    &self,
    edit_message_reply_markup: C,
  ) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, edit_message_reply_markup.as_ref()).await
  }

  pub async fn edit_message_scheduling_state<C: AsRef<EditMessageSchedulingState>>(
    &self,
    edit_message_scheduling_state: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, edit_message_scheduling_state.as_ref()).await
  }

  pub async fn edit_message_text<C: AsRef<EditMessageText>>(
    &self,
    edit_message_text: C,
  ) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, edit_message_text.as_ref()).await
  }

  pub async fn edit_proxy<C: AsRef<EditProxy>>(&self, edit_proxy: C) -> RTDResult<AddedProxy> {
    async_caller!(AddedProxy);
    async_caller(&self.api, edit_proxy.as_ref()).await
  }

  pub async fn edit_quick_reply_message<C: AsRef<EditQuickReplyMessage>>(
    &self,
    edit_quick_reply_message: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, edit_quick_reply_message.as_ref()).await
  }

  pub async fn edit_star_subscription<C: AsRef<EditStarSubscription>>(
    &self,
    edit_star_subscription: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, edit_star_subscription.as_ref()).await
  }

  pub async fn edit_story<C: AsRef<EditStory>>(&self, edit_story: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, edit_story.as_ref()).await
  }

  pub async fn edit_story_cover<C: AsRef<EditStoryCover>>(
    &self,
    edit_story_cover: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, edit_story_cover.as_ref()).await
  }

  pub async fn edit_text_composition_style<C: AsRef<EditTextCompositionStyle>>(
    &self,
    edit_text_composition_style: C,
  ) -> RTDResult<TextCompositionStyle> {
    async_caller!(TextCompositionStyle);
    async_caller(&self.api, edit_text_composition_style.as_ref()).await
  }

  pub async fn edit_user_star_subscription<C: AsRef<EditUserStarSubscription>>(
    &self,
    edit_user_star_subscription: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, edit_user_star_subscription.as_ref()).await
  }

  pub async fn enable_proxy<C: AsRef<EnableProxy>>(&self, enable_proxy: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, enable_proxy.as_ref()).await
  }

  pub async fn enable_ton_wallet_backup<C: AsRef<EnableTonWalletBackup>>(
    &self,
    enable_ton_wallet_backup: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, enable_ton_wallet_backup.as_ref()).await
  }

  pub async fn encrypt_group_call_data<C: AsRef<EncryptGroupCallData>>(
    &self,
    encrypt_group_call_data: C,
  ) -> RTDResult<Data> {
    async_caller!(Data);
    async_caller(&self.api, encrypt_group_call_data.as_ref()).await
  }

  pub async fn end_group_call<C: AsRef<EndGroupCall>>(&self, end_group_call: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, end_group_call.as_ref()).await
  }

  pub async fn end_group_call_recording<C: AsRef<EndGroupCallRecording>>(
    &self,
    end_group_call_recording: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, end_group_call_recording.as_ref()).await
  }

  pub async fn end_group_call_screen_sharing<C: AsRef<EndGroupCallScreenSharing>>(
    &self,
    end_group_call_screen_sharing: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, end_group_call_screen_sharing.as_ref()).await
  }

  pub async fn finish_file_generation<C: AsRef<FinishFileGeneration>>(
    &self,
    finish_file_generation: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, finish_file_generation.as_ref()).await
  }

  pub async fn fix_rich_message_with_ai<C: AsRef<FixRichMessageWithAi>>(
    &self,
    fix_rich_message_with_ai: C,
  ) -> RTDResult<RichMessage> {
    async_caller!(RichMessage);
    async_caller(&self.api, fix_rich_message_with_ai.as_ref()).await
  }

  pub async fn fix_text_with_ai<C: AsRef<FixTextWithAi>>(
    &self,
    fix_text_with_ai: C,
  ) -> RTDResult<FixedText> {
    async_caller!(FixedText);
    async_caller(&self.api, fix_text_with_ai.as_ref()).await
  }

  pub async fn forward_messages<C: AsRef<ForwardMessages>>(
    &self,
    forward_messages: C,
  ) -> RTDResult<Messages> {
    async_caller!(Messages);
    async_caller(&self.api, forward_messages.as_ref()).await
  }

  pub async fn get_account_ttl<C: AsRef<GetAccountTtl>>(
    &self,
    get_account_ttl: C,
  ) -> RTDResult<AccountTtl> {
    async_caller!(AccountTtl);
    async_caller(&self.api, get_account_ttl.as_ref()).await
  }

  pub async fn get_active_sessions<C: AsRef<GetActiveSessions>>(
    &self,
    get_active_sessions: C,
  ) -> RTDResult<Sessions> {
    async_caller!(Sessions);
    async_caller(&self.api, get_active_sessions.as_ref()).await
  }

  pub async fn get_address_ton_wallet<C: AsRef<GetAddressTonWallet>>(
    &self,
    get_address_ton_wallet: C,
  ) -> RTDResult<UserTonWalletAddress> {
    async_caller!(UserTonWalletAddress);
    async_caller(&self.api, get_address_ton_wallet.as_ref()).await
  }

  pub async fn get_all_passport_elements<C: AsRef<GetAllPassportElements>>(
    &self,
    get_all_passport_elements: C,
  ) -> RTDResult<PassportElements> {
    async_caller!(PassportElements);
    async_caller(&self.api, get_all_passport_elements.as_ref()).await
  }

  pub async fn get_all_sticker_emojis<C: AsRef<GetAllStickerEmojis>>(
    &self,
    get_all_sticker_emojis: C,
  ) -> RTDResult<Emojis> {
    async_caller!(Emojis);
    async_caller(&self.api, get_all_sticker_emojis.as_ref()).await
  }

  pub async fn get_animated_emoji<C: AsRef<GetAnimatedEmoji>>(
    &self,
    get_animated_emoji: C,
  ) -> RTDResult<AnimatedEmoji> {
    async_caller!(AnimatedEmoji);
    async_caller(&self.api, get_animated_emoji.as_ref()).await
  }

  pub async fn get_application_config<C: AsRef<GetApplicationConfig>>(
    &self,
    get_application_config: C,
  ) -> RTDResult<JsonValue> {
    async_caller!(JsonValue);
    async_caller(&self.api, get_application_config.as_ref()).await
  }

  pub async fn get_application_download_link<C: AsRef<GetApplicationDownloadLink>>(
    &self,
    get_application_download_link: C,
  ) -> RTDResult<HttpUrl> {
    async_caller!(HttpUrl);
    async_caller(&self.api, get_application_download_link.as_ref()).await
  }

  pub async fn get_archive_chat_list_settings<C: AsRef<GetArchiveChatListSettings>>(
    &self,
    get_archive_chat_list_settings: C,
  ) -> RTDResult<ArchiveChatListSettings> {
    async_caller!(ArchiveChatListSettings);
    async_caller(&self.api, get_archive_chat_list_settings.as_ref()).await
  }

  pub async fn get_archived_sticker_sets<C: AsRef<GetArchivedStickerSets>>(
    &self,
    get_archived_sticker_sets: C,
  ) -> RTDResult<StickerSets> {
    async_caller!(StickerSets);
    async_caller(&self.api, get_archived_sticker_sets.as_ref()).await
  }

  pub async fn get_attached_sticker_sets<C: AsRef<GetAttachedStickerSets>>(
    &self,
    get_attached_sticker_sets: C,
  ) -> RTDResult<StickerSets> {
    async_caller!(StickerSets);
    async_caller(&self.api, get_attached_sticker_sets.as_ref()).await
  }

  pub async fn get_attachment_menu_bot<C: AsRef<GetAttachmentMenuBot>>(
    &self,
    get_attachment_menu_bot: C,
  ) -> RTDResult<AttachmentMenuBot> {
    async_caller!(AttachmentMenuBot);
    async_caller(&self.api, get_attachment_menu_bot.as_ref()).await
  }

  pub async fn get_authentication_passkey_parameters<
    C: AsRef<GetAuthenticationPasskeyParameters>,
  >(
    &self,
    get_authentication_passkey_parameters: C,
  ) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, get_authentication_passkey_parameters.as_ref()).await
  }

  pub async fn get_authorization_state<C: AsRef<GetAuthorizationState>>(
    &self,
    get_authorization_state: C,
  ) -> RTDResult<AuthorizationState> {
    async_caller!(AuthorizationState);
    async_caller(&self.api, get_authorization_state.as_ref()).await
  }

  pub async fn get_auto_download_settings_presets<C: AsRef<GetAutoDownloadSettingsPresets>>(
    &self,
    get_auto_download_settings_presets: C,
  ) -> RTDResult<AutoDownloadSettingsPresets> {
    async_caller!(AutoDownloadSettingsPresets);
    async_caller(&self.api, get_auto_download_settings_presets.as_ref()).await
  }

  pub async fn get_autosave_settings<C: AsRef<GetAutosaveSettings>>(
    &self,
    get_autosave_settings: C,
  ) -> RTDResult<AutosaveSettings> {
    async_caller!(AutosaveSettings);
    async_caller(&self.api, get_autosave_settings.as_ref()).await
  }

  pub async fn get_available_chat_boost_slots<C: AsRef<GetAvailableChatBoostSlots>>(
    &self,
    get_available_chat_boost_slots: C,
  ) -> RTDResult<ChatBoostSlots> {
    async_caller!(ChatBoostSlots);
    async_caller(&self.api, get_available_chat_boost_slots.as_ref()).await
  }

  pub async fn get_available_gifts<C: AsRef<GetAvailableGifts>>(
    &self,
    get_available_gifts: C,
  ) -> RTDResult<AvailableGifts> {
    async_caller!(AvailableGifts);
    async_caller(&self.api, get_available_gifts.as_ref()).await
  }

  pub async fn get_background_url<C: AsRef<GetBackgroundUrl>>(
    &self,
    get_background_url: C,
  ) -> RTDResult<HttpUrl> {
    async_caller!(HttpUrl);
    async_caller(&self.api, get_background_url.as_ref()).await
  }

  pub async fn get_bank_card_info<C: AsRef<GetBankCardInfo>>(
    &self,
    get_bank_card_info: C,
  ) -> RTDResult<BankCardInfo> {
    async_caller!(BankCardInfo);
    async_caller(&self.api, get_bank_card_info.as_ref()).await
  }

  pub async fn get_basic_group<C: AsRef<GetBasicGroup>>(
    &self,
    get_basic_group: C,
  ) -> RTDResult<BasicGroup> {
    async_caller!(BasicGroup);
    async_caller(&self.api, get_basic_group.as_ref()).await
  }

  pub async fn get_basic_group_full_info<C: AsRef<GetBasicGroupFullInfo>>(
    &self,
    get_basic_group_full_info: C,
  ) -> RTDResult<BasicGroupFullInfo> {
    async_caller!(BasicGroupFullInfo);
    async_caller(&self.api, get_basic_group_full_info.as_ref()).await
  }

  pub async fn get_blocked_message_senders<C: AsRef<GetBlockedMessageSenders>>(
    &self,
    get_blocked_message_senders: C,
  ) -> RTDResult<MessageSenders> {
    async_caller!(MessageSenders);
    async_caller(&self.api, get_blocked_message_senders.as_ref()).await
  }

  pub async fn get_bot_info_description<C: AsRef<GetBotInfoDescription>>(
    &self,
    get_bot_info_description: C,
  ) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, get_bot_info_description.as_ref()).await
  }

  pub async fn get_bot_info_short_description<C: AsRef<GetBotInfoShortDescription>>(
    &self,
    get_bot_info_short_description: C,
  ) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, get_bot_info_short_description.as_ref()).await
  }

  pub async fn get_bot_media_preview_info<C: AsRef<GetBotMediaPreviewInfo>>(
    &self,
    get_bot_media_preview_info: C,
  ) -> RTDResult<BotMediaPreviewInfo> {
    async_caller!(BotMediaPreviewInfo);
    async_caller(&self.api, get_bot_media_preview_info.as_ref()).await
  }

  pub async fn get_bot_media_previews<C: AsRef<GetBotMediaPreviews>>(
    &self,
    get_bot_media_previews: C,
  ) -> RTDResult<BotMediaPreviews> {
    async_caller!(BotMediaPreviews);
    async_caller(&self.api, get_bot_media_previews.as_ref()).await
  }

  pub async fn get_bot_name<C: AsRef<GetBotName>>(&self, get_bot_name: C) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, get_bot_name.as_ref()).await
  }

  pub async fn get_bot_similar_bot_count<C: AsRef<GetBotSimilarBotCount>>(
    &self,
    get_bot_similar_bot_count: C,
  ) -> RTDResult<Count> {
    async_caller!(Count);
    async_caller(&self.api, get_bot_similar_bot_count.as_ref()).await
  }

  pub async fn get_bot_similar_bots<C: AsRef<GetBotSimilarBots>>(
    &self,
    get_bot_similar_bots: C,
  ) -> RTDResult<Users> {
    async_caller!(Users);
    async_caller(&self.api, get_bot_similar_bots.as_ref()).await
  }

  pub async fn get_business_account_star_amount<C: AsRef<GetBusinessAccountStarAmount>>(
    &self,
    get_business_account_star_amount: C,
  ) -> RTDResult<StarAmount> {
    async_caller!(StarAmount);
    async_caller(&self.api, get_business_account_star_amount.as_ref()).await
  }

  pub async fn get_business_chat_link_info<C: AsRef<GetBusinessChatLinkInfo>>(
    &self,
    get_business_chat_link_info: C,
  ) -> RTDResult<BusinessChatLinkInfo> {
    async_caller!(BusinessChatLinkInfo);
    async_caller(&self.api, get_business_chat_link_info.as_ref()).await
  }

  pub async fn get_business_chat_links<C: AsRef<GetBusinessChatLinks>>(
    &self,
    get_business_chat_links: C,
  ) -> RTDResult<BusinessChatLinks> {
    async_caller!(BusinessChatLinks);
    async_caller(&self.api, get_business_chat_links.as_ref()).await
  }

  pub async fn get_business_connected_bot<C: AsRef<GetBusinessConnectedBot>>(
    &self,
    get_business_connected_bot: C,
  ) -> RTDResult<BusinessConnectedBotInfo> {
    async_caller!(BusinessConnectedBotInfo);
    async_caller(&self.api, get_business_connected_bot.as_ref()).await
  }

  pub async fn get_business_connection<C: AsRef<GetBusinessConnection>>(
    &self,
    get_business_connection: C,
  ) -> RTDResult<BusinessConnection> {
    async_caller!(BusinessConnection);
    async_caller(&self.api, get_business_connection.as_ref()).await
  }

  pub async fn get_business_features<C: AsRef<GetBusinessFeatures>>(
    &self,
    get_business_features: C,
  ) -> RTDResult<BusinessFeatures> {
    async_caller!(BusinessFeatures);
    async_caller(&self.api, get_business_features.as_ref()).await
  }

  pub async fn get_callback_query_answer<C: AsRef<GetCallbackQueryAnswer>>(
    &self,
    get_callback_query_answer: C,
  ) -> RTDResult<CallbackQueryAnswer> {
    async_caller!(CallbackQueryAnswer);
    async_caller(&self.api, get_callback_query_answer.as_ref()).await
  }

  pub async fn get_callback_query_message<C: AsRef<GetCallbackQueryMessage>>(
    &self,
    get_callback_query_message: C,
  ) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, get_callback_query_message.as_ref()).await
  }

  pub async fn get_chat<C: AsRef<GetChat>>(&self, get_chat: C) -> RTDResult<Chat> {
    async_caller!(Chat);
    async_caller(&self.api, get_chat.as_ref()).await
  }

  pub async fn get_chat_active_stories<C: AsRef<GetChatActiveStories>>(
    &self,
    get_chat_active_stories: C,
  ) -> RTDResult<ChatActiveStories> {
    async_caller!(ChatActiveStories);
    async_caller(&self.api, get_chat_active_stories.as_ref()).await
  }

  pub async fn get_chat_administrators<C: AsRef<GetChatAdministrators>>(
    &self,
    get_chat_administrators: C,
  ) -> RTDResult<ChatAdministrators> {
    async_caller!(ChatAdministrators);
    async_caller(&self.api, get_chat_administrators.as_ref()).await
  }

  pub async fn get_chat_archived_stories<C: AsRef<GetChatArchivedStories>>(
    &self,
    get_chat_archived_stories: C,
  ) -> RTDResult<Stories> {
    async_caller!(Stories);
    async_caller(&self.api, get_chat_archived_stories.as_ref()).await
  }

  pub async fn get_chat_available_message_senders<C: AsRef<GetChatAvailableMessageSenders>>(
    &self,
    get_chat_available_message_senders: C,
  ) -> RTDResult<ChatMessageSenders> {
    async_caller!(ChatMessageSenders);
    async_caller(&self.api, get_chat_available_message_senders.as_ref()).await
  }

  pub async fn get_chat_available_paid_message_reaction_senders<
    C: AsRef<GetChatAvailablePaidMessageReactionSenders>,
  >(
    &self,
    get_chat_available_paid_message_reaction_senders: C,
  ) -> RTDResult<MessageSenders> {
    async_caller!(MessageSenders);
    async_caller(
      &self.api,
      get_chat_available_paid_message_reaction_senders.as_ref(),
    )
    .await
  }

  pub async fn get_chat_boost_features<C: AsRef<GetChatBoostFeatures>>(
    &self,
    get_chat_boost_features: C,
  ) -> RTDResult<ChatBoostFeatures> {
    async_caller!(ChatBoostFeatures);
    async_caller(&self.api, get_chat_boost_features.as_ref()).await
  }

  pub async fn get_chat_boost_level_features<C: AsRef<GetChatBoostLevelFeatures>>(
    &self,
    get_chat_boost_level_features: C,
  ) -> RTDResult<ChatBoostLevelFeatures> {
    async_caller!(ChatBoostLevelFeatures);
    async_caller(&self.api, get_chat_boost_level_features.as_ref()).await
  }

  pub async fn get_chat_boost_link<C: AsRef<GetChatBoostLink>>(
    &self,
    get_chat_boost_link: C,
  ) -> RTDResult<ChatBoostLink> {
    async_caller!(ChatBoostLink);
    async_caller(&self.api, get_chat_boost_link.as_ref()).await
  }

  pub async fn get_chat_boost_link_info<C: AsRef<GetChatBoostLinkInfo>>(
    &self,
    get_chat_boost_link_info: C,
  ) -> RTDResult<ChatBoostLinkInfo> {
    async_caller!(ChatBoostLinkInfo);
    async_caller(&self.api, get_chat_boost_link_info.as_ref()).await
  }

  pub async fn get_chat_boost_status<C: AsRef<GetChatBoostStatus>>(
    &self,
    get_chat_boost_status: C,
  ) -> RTDResult<ChatBoostStatus> {
    async_caller!(ChatBoostStatus);
    async_caller(&self.api, get_chat_boost_status.as_ref()).await
  }

  pub async fn get_chat_boosts<C: AsRef<GetChatBoosts>>(
    &self,
    get_chat_boosts: C,
  ) -> RTDResult<FoundChatBoosts> {
    async_caller!(FoundChatBoosts);
    async_caller(&self.api, get_chat_boosts.as_ref()).await
  }

  pub async fn get_chat_event_log<C: AsRef<GetChatEventLog>>(
    &self,
    get_chat_event_log: C,
  ) -> RTDResult<ChatEvents> {
    async_caller!(ChatEvents);
    async_caller(&self.api, get_chat_event_log.as_ref()).await
  }

  pub async fn get_chat_folder<C: AsRef<GetChatFolder>>(
    &self,
    get_chat_folder: C,
  ) -> RTDResult<ChatFolder> {
    async_caller!(ChatFolder);
    async_caller(&self.api, get_chat_folder.as_ref()).await
  }

  pub async fn get_chat_folder_chat_count<C: AsRef<GetChatFolderChatCount>>(
    &self,
    get_chat_folder_chat_count: C,
  ) -> RTDResult<Count> {
    async_caller!(Count);
    async_caller(&self.api, get_chat_folder_chat_count.as_ref()).await
  }

  pub async fn get_chat_folder_chats_to_leave<C: AsRef<GetChatFolderChatsToLeave>>(
    &self,
    get_chat_folder_chats_to_leave: C,
  ) -> RTDResult<Chats> {
    async_caller!(Chats);
    async_caller(&self.api, get_chat_folder_chats_to_leave.as_ref()).await
  }

  pub async fn get_chat_folder_default_icon_name<C: AsRef<GetChatFolderDefaultIconName>>(
    &self,
    get_chat_folder_default_icon_name: C,
  ) -> RTDResult<ChatFolderIcon> {
    async_caller!(ChatFolderIcon);
    async_caller(&self.api, get_chat_folder_default_icon_name.as_ref()).await
  }

  pub async fn get_chat_folder_invite_links<C: AsRef<GetChatFolderInviteLinks>>(
    &self,
    get_chat_folder_invite_links: C,
  ) -> RTDResult<ChatFolderInviteLinks> {
    async_caller!(ChatFolderInviteLinks);
    async_caller(&self.api, get_chat_folder_invite_links.as_ref()).await
  }

  pub async fn get_chat_folder_new_chats<C: AsRef<GetChatFolderNewChats>>(
    &self,
    get_chat_folder_new_chats: C,
  ) -> RTDResult<Chats> {
    async_caller!(Chats);
    async_caller(&self.api, get_chat_folder_new_chats.as_ref()).await
  }

  pub async fn get_chat_history<C: AsRef<GetChatHistory>>(
    &self,
    get_chat_history: C,
  ) -> RTDResult<Messages> {
    async_caller!(Messages);
    async_caller(&self.api, get_chat_history.as_ref()).await
  }

  pub async fn get_chat_invite_link<C: AsRef<GetChatInviteLink>>(
    &self,
    get_chat_invite_link: C,
  ) -> RTDResult<ChatInviteLink> {
    async_caller!(ChatInviteLink);
    async_caller(&self.api, get_chat_invite_link.as_ref()).await
  }

  pub async fn get_chat_invite_link_counts<C: AsRef<GetChatInviteLinkCounts>>(
    &self,
    get_chat_invite_link_counts: C,
  ) -> RTDResult<ChatInviteLinkCounts> {
    async_caller!(ChatInviteLinkCounts);
    async_caller(&self.api, get_chat_invite_link_counts.as_ref()).await
  }

  pub async fn get_chat_invite_link_members<C: AsRef<GetChatInviteLinkMembers>>(
    &self,
    get_chat_invite_link_members: C,
  ) -> RTDResult<ChatInviteLinkMembers> {
    async_caller!(ChatInviteLinkMembers);
    async_caller(&self.api, get_chat_invite_link_members.as_ref()).await
  }

  pub async fn get_chat_invite_links<C: AsRef<GetChatInviteLinks>>(
    &self,
    get_chat_invite_links: C,
  ) -> RTDResult<ChatInviteLinks> {
    async_caller!(ChatInviteLinks);
    async_caller(&self.api, get_chat_invite_links.as_ref()).await
  }

  pub async fn get_chat_join_requests<C: AsRef<GetChatJoinRequests>>(
    &self,
    get_chat_join_requests: C,
  ) -> RTDResult<ChatJoinRequests> {
    async_caller!(ChatJoinRequests);
    async_caller(&self.api, get_chat_join_requests.as_ref()).await
  }

  pub async fn get_chat_lists_to_add_chat<C: AsRef<GetChatListsToAddChat>>(
    &self,
    get_chat_lists_to_add_chat: C,
  ) -> RTDResult<ChatLists> {
    async_caller!(ChatLists);
    async_caller(&self.api, get_chat_lists_to_add_chat.as_ref()).await
  }

  pub async fn get_chat_member<C: AsRef<GetChatMember>>(
    &self,
    get_chat_member: C,
  ) -> RTDResult<ChatMember> {
    async_caller!(ChatMember);
    async_caller(&self.api, get_chat_member.as_ref()).await
  }

  pub async fn get_chat_message_by_date<C: AsRef<GetChatMessageByDate>>(
    &self,
    get_chat_message_by_date: C,
  ) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, get_chat_message_by_date.as_ref()).await
  }

  pub async fn get_chat_message_calendar<C: AsRef<GetChatMessageCalendar>>(
    &self,
    get_chat_message_calendar: C,
  ) -> RTDResult<MessageCalendar> {
    async_caller!(MessageCalendar);
    async_caller(&self.api, get_chat_message_calendar.as_ref()).await
  }

  pub async fn get_chat_message_count<C: AsRef<GetChatMessageCount>>(
    &self,
    get_chat_message_count: C,
  ) -> RTDResult<Count> {
    async_caller!(Count);
    async_caller(&self.api, get_chat_message_count.as_ref()).await
  }

  pub async fn get_chat_message_position<C: AsRef<GetChatMessagePosition>>(
    &self,
    get_chat_message_position: C,
  ) -> RTDResult<Count> {
    async_caller!(Count);
    async_caller(&self.api, get_chat_message_position.as_ref()).await
  }

  pub async fn get_chat_notification_settings_exceptions<
    C: AsRef<GetChatNotificationSettingsExceptions>,
  >(
    &self,
    get_chat_notification_settings_exceptions: C,
  ) -> RTDResult<Chats> {
    async_caller!(Chats);
    async_caller(
      &self.api,
      get_chat_notification_settings_exceptions.as_ref(),
    )
    .await
  }

  pub async fn get_chat_owner_after_leaving<C: AsRef<GetChatOwnerAfterLeaving>>(
    &self,
    get_chat_owner_after_leaving: C,
  ) -> RTDResult<User> {
    async_caller!(User);
    async_caller(&self.api, get_chat_owner_after_leaving.as_ref()).await
  }

  pub async fn get_chat_pinned_message<C: AsRef<GetChatPinnedMessage>>(
    &self,
    get_chat_pinned_message: C,
  ) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, get_chat_pinned_message.as_ref()).await
  }

  pub async fn get_chat_posted_to_chat_page_stories<C: AsRef<GetChatPostedToChatPageStories>>(
    &self,
    get_chat_posted_to_chat_page_stories: C,
  ) -> RTDResult<Stories> {
    async_caller!(Stories);
    async_caller(&self.api, get_chat_posted_to_chat_page_stories.as_ref()).await
  }

  pub async fn get_chat_revenue_statistics<C: AsRef<GetChatRevenueStatistics>>(
    &self,
    get_chat_revenue_statistics: C,
  ) -> RTDResult<ChatRevenueStatistics> {
    async_caller!(ChatRevenueStatistics);
    async_caller(&self.api, get_chat_revenue_statistics.as_ref()).await
  }

  pub async fn get_chat_revenue_transactions<C: AsRef<GetChatRevenueTransactions>>(
    &self,
    get_chat_revenue_transactions: C,
  ) -> RTDResult<ChatRevenueTransactions> {
    async_caller!(ChatRevenueTransactions);
    async_caller(&self.api, get_chat_revenue_transactions.as_ref()).await
  }

  pub async fn get_chat_revenue_withdrawal_url<C: AsRef<GetChatRevenueWithdrawalUrl>>(
    &self,
    get_chat_revenue_withdrawal_url: C,
  ) -> RTDResult<HttpUrl> {
    async_caller!(HttpUrl);
    async_caller(&self.api, get_chat_revenue_withdrawal_url.as_ref()).await
  }

  pub async fn get_chat_scheduled_messages<C: AsRef<GetChatScheduledMessages>>(
    &self,
    get_chat_scheduled_messages: C,
  ) -> RTDResult<Messages> {
    async_caller!(Messages);
    async_caller(&self.api, get_chat_scheduled_messages.as_ref()).await
  }

  pub async fn get_chat_similar_chat_count<C: AsRef<GetChatSimilarChatCount>>(
    &self,
    get_chat_similar_chat_count: C,
  ) -> RTDResult<Count> {
    async_caller!(Count);
    async_caller(&self.api, get_chat_similar_chat_count.as_ref()).await
  }

  pub async fn get_chat_similar_chats<C: AsRef<GetChatSimilarChats>>(
    &self,
    get_chat_similar_chats: C,
  ) -> RTDResult<Chats> {
    async_caller!(Chats);
    async_caller(&self.api, get_chat_similar_chats.as_ref()).await
  }

  pub async fn get_chat_sparse_message_positions<C: AsRef<GetChatSparseMessagePositions>>(
    &self,
    get_chat_sparse_message_positions: C,
  ) -> RTDResult<MessagePositions> {
    async_caller!(MessagePositions);
    async_caller(&self.api, get_chat_sparse_message_positions.as_ref()).await
  }

  pub async fn get_chat_sponsored_messages<C: AsRef<GetChatSponsoredMessages>>(
    &self,
    get_chat_sponsored_messages: C,
  ) -> RTDResult<SponsoredMessages> {
    async_caller!(SponsoredMessages);
    async_caller(&self.api, get_chat_sponsored_messages.as_ref()).await
  }

  pub async fn get_chat_statistics<C: AsRef<GetChatStatistics>>(
    &self,
    get_chat_statistics: C,
  ) -> RTDResult<ChatStatistics> {
    async_caller!(ChatStatistics);
    async_caller(&self.api, get_chat_statistics.as_ref()).await
  }

  pub async fn get_chat_story_albums<C: AsRef<GetChatStoryAlbums>>(
    &self,
    get_chat_story_albums: C,
  ) -> RTDResult<StoryAlbums> {
    async_caller!(StoryAlbums);
    async_caller(&self.api, get_chat_story_albums.as_ref()).await
  }

  pub async fn get_chat_story_interactions<C: AsRef<GetChatStoryInteractions>>(
    &self,
    get_chat_story_interactions: C,
  ) -> RTDResult<StoryInteractions> {
    async_caller!(StoryInteractions);
    async_caller(&self.api, get_chat_story_interactions.as_ref()).await
  }

  pub async fn get_chats<C: AsRef<GetChats>>(&self, get_chats: C) -> RTDResult<Chats> {
    async_caller!(Chats);
    async_caller(&self.api, get_chats.as_ref()).await
  }

  pub async fn get_chats_for_chat_folder_invite_link<C: AsRef<GetChatsForChatFolderInviteLink>>(
    &self,
    get_chats_for_chat_folder_invite_link: C,
  ) -> RTDResult<Chats> {
    async_caller!(Chats);
    async_caller(&self.api, get_chats_for_chat_folder_invite_link.as_ref()).await
  }

  pub async fn get_chats_to_post_stories<C: AsRef<GetChatsToPostStories>>(
    &self,
    get_chats_to_post_stories: C,
  ) -> RTDResult<Chats> {
    async_caller!(Chats);
    async_caller(&self.api, get_chats_to_post_stories.as_ref()).await
  }

  pub async fn get_close_friends<C: AsRef<GetCloseFriends>>(
    &self,
    get_close_friends: C,
  ) -> RTDResult<Users> {
    async_caller!(Users);
    async_caller(&self.api, get_close_friends.as_ref()).await
  }

  pub async fn get_collectible_item_info<C: AsRef<GetCollectibleItemInfo>>(
    &self,
    get_collectible_item_info: C,
  ) -> RTDResult<CollectibleItemInfo> {
    async_caller!(CollectibleItemInfo);
    async_caller(&self.api, get_collectible_item_info.as_ref()).await
  }

  pub async fn get_commands<C: AsRef<GetCommands>>(
    &self,
    get_commands: C,
  ) -> RTDResult<BotCommands> {
    async_caller!(BotCommands);
    async_caller(&self.api, get_commands.as_ref()).await
  }

  pub async fn get_community_full_info<C: AsRef<GetCommunityFullInfo>>(
    &self,
    get_community_full_info: C,
  ) -> RTDResult<CommunityFullInfo> {
    async_caller!(CommunityFullInfo);
    async_caller(&self.api, get_community_full_info.as_ref()).await
  }

  pub async fn get_connected_affiliate_program<C: AsRef<GetConnectedAffiliateProgram>>(
    &self,
    get_connected_affiliate_program: C,
  ) -> RTDResult<ConnectedAffiliateProgram> {
    async_caller!(ConnectedAffiliateProgram);
    async_caller(&self.api, get_connected_affiliate_program.as_ref()).await
  }

  pub async fn get_connected_affiliate_programs<C: AsRef<GetConnectedAffiliatePrograms>>(
    &self,
    get_connected_affiliate_programs: C,
  ) -> RTDResult<ConnectedAffiliatePrograms> {
    async_caller!(ConnectedAffiliatePrograms);
    async_caller(&self.api, get_connected_affiliate_programs.as_ref()).await
  }

  pub async fn get_connected_websites<C: AsRef<GetConnectedWebsites>>(
    &self,
    get_connected_websites: C,
  ) -> RTDResult<ConnectedWebsites> {
    async_caller!(ConnectedWebsites);
    async_caller(&self.api, get_connected_websites.as_ref()).await
  }

  pub async fn get_contacts<C: AsRef<GetContacts>>(&self, get_contacts: C) -> RTDResult<Users> {
    async_caller!(Users);
    async_caller(&self.api, get_contacts.as_ref()).await
  }

  pub async fn get_countries<C: AsRef<GetCountries>>(
    &self,
    get_countries: C,
  ) -> RTDResult<Countries> {
    async_caller!(Countries);
    async_caller(&self.api, get_countries.as_ref()).await
  }

  pub async fn get_country<C: AsRef<GetCountry>>(&self, get_country: C) -> RTDResult<CountryInfo> {
    async_caller!(CountryInfo);
    async_caller(&self.api, get_country.as_ref()).await
  }

  pub async fn get_country_code<C: AsRef<GetCountryCode>>(
    &self,
    get_country_code: C,
  ) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, get_country_code.as_ref()).await
  }

  pub async fn get_country_flag_emoji<C: AsRef<GetCountryFlagEmoji>>(
    &self,
    get_country_flag_emoji: C,
  ) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, get_country_flag_emoji.as_ref()).await
  }

  pub async fn get_created_public_chats<C: AsRef<GetCreatedPublicChats>>(
    &self,
    get_created_public_chats: C,
  ) -> RTDResult<Chats> {
    async_caller!(Chats);
    async_caller(&self.api, get_created_public_chats.as_ref()).await
  }

  pub async fn get_currency_exchange_rates<C: AsRef<GetCurrencyExchangeRates>>(
    &self,
    get_currency_exchange_rates: C,
  ) -> RTDResult<CurrencyExchangeRates> {
    async_caller!(CurrencyExchangeRates);
    async_caller(&self.api, get_currency_exchange_rates.as_ref()).await
  }

  pub async fn get_current_state<C: AsRef<GetCurrentState>>(
    &self,
    get_current_state: C,
  ) -> RTDResult<Updates> {
    async_caller!(Updates);
    async_caller(&self.api, get_current_state.as_ref()).await
  }

  pub async fn get_current_weather<C: AsRef<GetCurrentWeather>>(
    &self,
    get_current_weather: C,
  ) -> RTDResult<CurrentWeather> {
    async_caller!(CurrentWeather);
    async_caller(&self.api, get_current_weather.as_ref()).await
  }

  pub async fn get_custom_emoji_reaction_animations<C: AsRef<GetCustomEmojiReactionAnimations>>(
    &self,
    get_custom_emoji_reaction_animations: C,
  ) -> RTDResult<Stickers> {
    async_caller!(Stickers);
    async_caller(&self.api, get_custom_emoji_reaction_animations.as_ref()).await
  }

  pub async fn get_custom_emoji_stickers<C: AsRef<GetCustomEmojiStickers>>(
    &self,
    get_custom_emoji_stickers: C,
  ) -> RTDResult<Stickers> {
    async_caller!(Stickers);
    async_caller(&self.api, get_custom_emoji_stickers.as_ref()).await
  }

  pub async fn get_database_statistics<C: AsRef<GetDatabaseStatistics>>(
    &self,
    get_database_statistics: C,
  ) -> RTDResult<DatabaseStatistics> {
    async_caller!(DatabaseStatistics);
    async_caller(&self.api, get_database_statistics.as_ref()).await
  }

  pub async fn get_deep_link_info<C: AsRef<GetDeepLinkInfo>>(
    &self,
    get_deep_link_info: C,
  ) -> RTDResult<DeepLinkInfo> {
    async_caller!(DeepLinkInfo);
    async_caller(&self.api, get_deep_link_info.as_ref()).await
  }

  pub async fn get_default_background_custom_emoji_stickers<
    C: AsRef<GetDefaultBackgroundCustomEmojiStickers>,
  >(
    &self,
    get_default_background_custom_emoji_stickers: C,
  ) -> RTDResult<Stickers> {
    async_caller!(Stickers);
    async_caller(
      &self.api,
      get_default_background_custom_emoji_stickers.as_ref(),
    )
    .await
  }

  pub async fn get_default_chat_emoji_statuses<C: AsRef<GetDefaultChatEmojiStatuses>>(
    &self,
    get_default_chat_emoji_statuses: C,
  ) -> RTDResult<EmojiStatusCustomEmojis> {
    async_caller!(EmojiStatusCustomEmojis);
    async_caller(&self.api, get_default_chat_emoji_statuses.as_ref()).await
  }

  pub async fn get_default_chat_photo_custom_emoji_stickers<
    C: AsRef<GetDefaultChatPhotoCustomEmojiStickers>,
  >(
    &self,
    get_default_chat_photo_custom_emoji_stickers: C,
  ) -> RTDResult<Stickers> {
    async_caller!(Stickers);
    async_caller(
      &self.api,
      get_default_chat_photo_custom_emoji_stickers.as_ref(),
    )
    .await
  }

  pub async fn get_default_emoji_statuses<C: AsRef<GetDefaultEmojiStatuses>>(
    &self,
    get_default_emoji_statuses: C,
  ) -> RTDResult<EmojiStatusCustomEmojis> {
    async_caller!(EmojiStatusCustomEmojis);
    async_caller(&self.api, get_default_emoji_statuses.as_ref()).await
  }

  pub async fn get_default_message_auto_delete_time<C: AsRef<GetDefaultMessageAutoDeleteTime>>(
    &self,
    get_default_message_auto_delete_time: C,
  ) -> RTDResult<MessageAutoDeleteTime> {
    async_caller!(MessageAutoDeleteTime);
    async_caller(&self.api, get_default_message_auto_delete_time.as_ref()).await
  }

  pub async fn get_default_profile_photo_custom_emoji_stickers<
    C: AsRef<GetDefaultProfilePhotoCustomEmojiStickers>,
  >(
    &self,
    get_default_profile_photo_custom_emoji_stickers: C,
  ) -> RTDResult<Stickers> {
    async_caller!(Stickers);
    async_caller(
      &self.api,
      get_default_profile_photo_custom_emoji_stickers.as_ref(),
    )
    .await
  }

  pub async fn get_direct_messages_chat_topic<C: AsRef<GetDirectMessagesChatTopic>>(
    &self,
    get_direct_messages_chat_topic: C,
  ) -> RTDResult<DirectMessagesChatTopic> {
    async_caller!(DirectMessagesChatTopic);
    async_caller(&self.api, get_direct_messages_chat_topic.as_ref()).await
  }

  pub async fn get_direct_messages_chat_topic_history<
    C: AsRef<GetDirectMessagesChatTopicHistory>,
  >(
    &self,
    get_direct_messages_chat_topic_history: C,
  ) -> RTDResult<Messages> {
    async_caller!(Messages);
    async_caller(&self.api, get_direct_messages_chat_topic_history.as_ref()).await
  }

  pub async fn get_direct_messages_chat_topic_message_by_date<
    C: AsRef<GetDirectMessagesChatTopicMessageByDate>,
  >(
    &self,
    get_direct_messages_chat_topic_message_by_date: C,
  ) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(
      &self.api,
      get_direct_messages_chat_topic_message_by_date.as_ref(),
    )
    .await
  }

  pub async fn get_direct_messages_chat_topic_revenue<
    C: AsRef<GetDirectMessagesChatTopicRevenue>,
  >(
    &self,
    get_direct_messages_chat_topic_revenue: C,
  ) -> RTDResult<StarCount> {
    async_caller!(StarCount);
    async_caller(&self.api, get_direct_messages_chat_topic_revenue.as_ref()).await
  }

  pub async fn get_disallowed_chat_emoji_statuses<C: AsRef<GetDisallowedChatEmojiStatuses>>(
    &self,
    get_disallowed_chat_emoji_statuses: C,
  ) -> RTDResult<EmojiStatusCustomEmojis> {
    async_caller!(EmojiStatusCustomEmojis);
    async_caller(&self.api, get_disallowed_chat_emoji_statuses.as_ref()).await
  }

  pub async fn get_emoji_categories<C: AsRef<GetEmojiCategories>>(
    &self,
    get_emoji_categories: C,
  ) -> RTDResult<EmojiCategories> {
    async_caller!(EmojiCategories);
    async_caller(&self.api, get_emoji_categories.as_ref()).await
  }

  pub async fn get_emoji_reaction<C: AsRef<GetEmojiReaction>>(
    &self,
    get_emoji_reaction: C,
  ) -> RTDResult<EmojiReaction> {
    async_caller!(EmojiReaction);
    async_caller(&self.api, get_emoji_reaction.as_ref()).await
  }

  pub async fn get_emoji_suggestions_url<C: AsRef<GetEmojiSuggestionsUrl>>(
    &self,
    get_emoji_suggestions_url: C,
  ) -> RTDResult<HttpUrl> {
    async_caller!(HttpUrl);
    async_caller(&self.api, get_emoji_suggestions_url.as_ref()).await
  }

  pub async fn get_external_link<C: AsRef<GetExternalLink>>(
    &self,
    get_external_link: C,
  ) -> RTDResult<HttpUrl> {
    async_caller!(HttpUrl);
    async_caller(&self.api, get_external_link.as_ref()).await
  }

  pub async fn get_external_link_info<C: AsRef<GetExternalLinkInfo>>(
    &self,
    get_external_link_info: C,
  ) -> RTDResult<LoginUrlInfo> {
    async_caller!(LoginUrlInfo);
    async_caller(&self.api, get_external_link_info.as_ref()).await
  }

  pub async fn get_favorite_stickers<C: AsRef<GetFavoriteStickers>>(
    &self,
    get_favorite_stickers: C,
  ) -> RTDResult<Stickers> {
    async_caller!(Stickers);
    async_caller(&self.api, get_favorite_stickers.as_ref()).await
  }

  pub async fn get_file<C: AsRef<GetFile>>(&self, get_file: C) -> RTDResult<File> {
    async_caller!(File);
    async_caller(&self.api, get_file.as_ref()).await
  }

  pub async fn get_file_downloaded_prefix_size<C: AsRef<GetFileDownloadedPrefixSize>>(
    &self,
    get_file_downloaded_prefix_size: C,
  ) -> RTDResult<FileDownloadedPrefixSize> {
    async_caller!(FileDownloadedPrefixSize);
    async_caller(&self.api, get_file_downloaded_prefix_size.as_ref()).await
  }

  pub async fn get_file_extension<C: AsRef<GetFileExtension>>(
    &self,
    get_file_extension: C,
  ) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, get_file_extension.as_ref()).await
  }

  pub async fn get_file_mime_type<C: AsRef<GetFileMimeType>>(
    &self,
    get_file_mime_type: C,
  ) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, get_file_mime_type.as_ref()).await
  }

  pub async fn get_forum_topic<C: AsRef<GetForumTopic>>(
    &self,
    get_forum_topic: C,
  ) -> RTDResult<ForumTopic> {
    async_caller!(ForumTopic);
    async_caller(&self.api, get_forum_topic.as_ref()).await
  }

  pub async fn get_forum_topic_default_icons<C: AsRef<GetForumTopicDefaultIcons>>(
    &self,
    get_forum_topic_default_icons: C,
  ) -> RTDResult<Stickers> {
    async_caller!(Stickers);
    async_caller(&self.api, get_forum_topic_default_icons.as_ref()).await
  }

  pub async fn get_forum_topic_history<C: AsRef<GetForumTopicHistory>>(
    &self,
    get_forum_topic_history: C,
  ) -> RTDResult<Messages> {
    async_caller!(Messages);
    async_caller(&self.api, get_forum_topic_history.as_ref()).await
  }

  pub async fn get_forum_topic_link<C: AsRef<GetForumTopicLink>>(
    &self,
    get_forum_topic_link: C,
  ) -> RTDResult<MessageLink> {
    async_caller!(MessageLink);
    async_caller(&self.api, get_forum_topic_link.as_ref()).await
  }

  pub async fn get_forum_topics<C: AsRef<GetForumTopics>>(
    &self,
    get_forum_topics: C,
  ) -> RTDResult<ForumTopics> {
    async_caller!(ForumTopics);
    async_caller(&self.api, get_forum_topics.as_ref()).await
  }

  pub async fn get_full_rich_message<C: AsRef<GetFullRichMessage>>(
    &self,
    get_full_rich_message: C,
  ) -> RTDResult<RichMessage> {
    async_caller!(RichMessage);
    async_caller(&self.api, get_full_rich_message.as_ref()).await
  }

  pub async fn get_game_high_scores<C: AsRef<GetGameHighScores>>(
    &self,
    get_game_high_scores: C,
  ) -> RTDResult<GameHighScores> {
    async_caller!(GameHighScores);
    async_caller(&self.api, get_game_high_scores.as_ref()).await
  }

  pub async fn get_gift_auction_acquired_gifts<C: AsRef<GetGiftAuctionAcquiredGifts>>(
    &self,
    get_gift_auction_acquired_gifts: C,
  ) -> RTDResult<GiftAuctionAcquiredGifts> {
    async_caller!(GiftAuctionAcquiredGifts);
    async_caller(&self.api, get_gift_auction_acquired_gifts.as_ref()).await
  }

  pub async fn get_gift_auction_state<C: AsRef<GetGiftAuctionState>>(
    &self,
    get_gift_auction_state: C,
  ) -> RTDResult<GiftAuctionState> {
    async_caller!(GiftAuctionState);
    async_caller(&self.api, get_gift_auction_state.as_ref()).await
  }

  pub async fn get_gift_chat_themes<C: AsRef<GetGiftChatThemes>>(
    &self,
    get_gift_chat_themes: C,
  ) -> RTDResult<GiftChatThemes> {
    async_caller!(GiftChatThemes);
    async_caller(&self.api, get_gift_chat_themes.as_ref()).await
  }

  pub async fn get_gift_collections<C: AsRef<GetGiftCollections>>(
    &self,
    get_gift_collections: C,
  ) -> RTDResult<GiftCollections> {
    async_caller!(GiftCollections);
    async_caller(&self.api, get_gift_collections.as_ref()).await
  }

  pub async fn get_gift_upgrade_preview<C: AsRef<GetGiftUpgradePreview>>(
    &self,
    get_gift_upgrade_preview: C,
  ) -> RTDResult<GiftUpgradePreview> {
    async_caller!(GiftUpgradePreview);
    async_caller(&self.api, get_gift_upgrade_preview.as_ref()).await
  }

  pub async fn get_gifts_for_crafting<C: AsRef<GetGiftsForCrafting>>(
    &self,
    get_gifts_for_crafting: C,
  ) -> RTDResult<GiftsForCrafting> {
    async_caller!(GiftsForCrafting);
    async_caller(&self.api, get_gifts_for_crafting.as_ref()).await
  }

  pub async fn get_giveaway_info<C: AsRef<GetGiveawayInfo>>(
    &self,
    get_giveaway_info: C,
  ) -> RTDResult<GiveawayInfo> {
    async_caller!(GiveawayInfo);
    async_caller(&self.api, get_giveaway_info.as_ref()).await
  }

  pub async fn get_gram_revenue_statistics<C: AsRef<GetGramRevenueStatistics>>(
    &self,
    get_gram_revenue_statistics: C,
  ) -> RTDResult<GramRevenueStatistics> {
    async_caller!(GramRevenueStatistics);
    async_caller(&self.api, get_gram_revenue_statistics.as_ref()).await
  }

  pub async fn get_gram_withdrawal_url<C: AsRef<GetGramWithdrawalUrl>>(
    &self,
    get_gram_withdrawal_url: C,
  ) -> RTDResult<HttpUrl> {
    async_caller!(HttpUrl);
    async_caller(&self.api, get_gram_withdrawal_url.as_ref()).await
  }

  pub async fn get_greeting_stickers<C: AsRef<GetGreetingStickers>>(
    &self,
    get_greeting_stickers: C,
  ) -> RTDResult<Stickers> {
    async_caller!(Stickers);
    async_caller(&self.api, get_greeting_stickers.as_ref()).await
  }

  pub async fn get_grossing_web_app_bots<C: AsRef<GetGrossingWebAppBots>>(
    &self,
    get_grossing_web_app_bots: C,
  ) -> RTDResult<FoundUsers> {
    async_caller!(FoundUsers);
    async_caller(&self.api, get_grossing_web_app_bots.as_ref()).await
  }

  pub async fn get_group_call<C: AsRef<GetGroupCall>>(
    &self,
    get_group_call: C,
  ) -> RTDResult<GroupCall> {
    async_caller!(GroupCall);
    async_caller(&self.api, get_group_call.as_ref()).await
  }

  pub async fn get_group_call_participants<C: AsRef<GetGroupCallParticipants>>(
    &self,
    get_group_call_participants: C,
  ) -> RTDResult<GroupCallParticipants> {
    async_caller!(GroupCallParticipants);
    async_caller(&self.api, get_group_call_participants.as_ref()).await
  }

  pub async fn get_group_call_stream_segment<C: AsRef<GetGroupCallStreamSegment>>(
    &self,
    get_group_call_stream_segment: C,
  ) -> RTDResult<Data> {
    async_caller!(Data);
    async_caller(&self.api, get_group_call_stream_segment.as_ref()).await
  }

  pub async fn get_group_call_streams<C: AsRef<GetGroupCallStreams>>(
    &self,
    get_group_call_streams: C,
  ) -> RTDResult<GroupCallStreams> {
    async_caller!(GroupCallStreams);
    async_caller(&self.api, get_group_call_streams.as_ref()).await
  }

  pub async fn get_groups_in_common<C: AsRef<GetGroupsInCommon>>(
    &self,
    get_groups_in_common: C,
  ) -> RTDResult<Chats> {
    async_caller!(Chats);
    async_caller(&self.api, get_groups_in_common.as_ref()).await
  }

  pub async fn get_guard_bot_web_app_url<C: AsRef<GetGuardBotWebAppUrl>>(
    &self,
    get_guard_bot_web_app_url: C,
  ) -> RTDResult<WebAppUrl> {
    async_caller!(WebAppUrl);
    async_caller(&self.api, get_guard_bot_web_app_url.as_ref()).await
  }

  pub async fn get_imported_contact_count<C: AsRef<GetImportedContactCount>>(
    &self,
    get_imported_contact_count: C,
  ) -> RTDResult<Count> {
    async_caller!(Count);
    async_caller(&self.api, get_imported_contact_count.as_ref()).await
  }

  pub async fn get_inactive_supergroup_chats<C: AsRef<GetInactiveSupergroupChats>>(
    &self,
    get_inactive_supergroup_chats: C,
  ) -> RTDResult<Chats> {
    async_caller!(Chats);
    async_caller(&self.api, get_inactive_supergroup_chats.as_ref()).await
  }

  pub async fn get_inline_game_high_scores<C: AsRef<GetInlineGameHighScores>>(
    &self,
    get_inline_game_high_scores: C,
  ) -> RTDResult<GameHighScores> {
    async_caller!(GameHighScores);
    async_caller(&self.api, get_inline_game_high_scores.as_ref()).await
  }

  pub async fn get_inline_query_results<C: AsRef<GetInlineQueryResults>>(
    &self,
    get_inline_query_results: C,
  ) -> RTDResult<InlineQueryResults> {
    async_caller!(InlineQueryResults);
    async_caller(&self.api, get_inline_query_results.as_ref()).await
  }

  pub async fn get_installed_backgrounds<C: AsRef<GetInstalledBackgrounds>>(
    &self,
    get_installed_backgrounds: C,
  ) -> RTDResult<Backgrounds> {
    async_caller!(Backgrounds);
    async_caller(&self.api, get_installed_backgrounds.as_ref()).await
  }

  pub async fn get_installed_sticker_sets<C: AsRef<GetInstalledStickerSets>>(
    &self,
    get_installed_sticker_sets: C,
  ) -> RTDResult<StickerSets> {
    async_caller!(StickerSets);
    async_caller(&self.api, get_installed_sticker_sets.as_ref()).await
  }

  pub async fn get_internal_link<C: AsRef<GetInternalLink>>(
    &self,
    get_internal_link: C,
  ) -> RTDResult<HttpUrl> {
    async_caller!(HttpUrl);
    async_caller(&self.api, get_internal_link.as_ref()).await
  }

  pub async fn get_internal_link_type<C: AsRef<GetInternalLinkType>>(
    &self,
    get_internal_link_type: C,
  ) -> RTDResult<InternalLinkType> {
    async_caller!(InternalLinkType);
    async_caller(&self.api, get_internal_link_type.as_ref()).await
  }

  pub async fn get_json_string<C: AsRef<GetJsonString>>(
    &self,
    get_json_string: C,
  ) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, get_json_string.as_ref()).await
  }

  pub async fn get_json_value<C: AsRef<GetJsonValue>>(
    &self,
    get_json_value: C,
  ) -> RTDResult<JsonValue> {
    async_caller!(JsonValue);
    async_caller(&self.api, get_json_value.as_ref()).await
  }

  pub async fn get_keyword_emojis<C: AsRef<GetKeywordEmojis>>(
    &self,
    get_keyword_emojis: C,
  ) -> RTDResult<Emojis> {
    async_caller!(Emojis);
    async_caller(&self.api, get_keyword_emojis.as_ref()).await
  }

  pub async fn get_language_pack_info<C: AsRef<GetLanguagePackInfo>>(
    &self,
    get_language_pack_info: C,
  ) -> RTDResult<LanguagePackInfo> {
    async_caller!(LanguagePackInfo);
    async_caller(&self.api, get_language_pack_info.as_ref()).await
  }

  pub async fn get_language_pack_string<C: AsRef<GetLanguagePackString>>(
    &self,
    get_language_pack_string: C,
  ) -> RTDResult<LanguagePackStringValue> {
    async_caller!(LanguagePackStringValue);
    async_caller(&self.api, get_language_pack_string.as_ref()).await
  }

  pub async fn get_language_pack_strings<C: AsRef<GetLanguagePackStrings>>(
    &self,
    get_language_pack_strings: C,
  ) -> RTDResult<LanguagePackStrings> {
    async_caller!(LanguagePackStrings);
    async_caller(&self.api, get_language_pack_strings.as_ref()).await
  }

  pub async fn get_link_preview<C: AsRef<GetLinkPreview>>(
    &self,
    get_link_preview: C,
  ) -> RTDResult<LinkPreview> {
    async_caller!(LinkPreview);
    async_caller(&self.api, get_link_preview.as_ref()).await
  }

  pub async fn get_link_web_browser_type<C: AsRef<GetLinkWebBrowserType>>(
    &self,
    get_link_web_browser_type: C,
  ) -> RTDResult<WebBrowserType> {
    async_caller!(WebBrowserType);
    async_caller(&self.api, get_link_web_browser_type.as_ref()).await
  }

  pub async fn get_live_story_available_message_senders<
    C: AsRef<GetLiveStoryAvailableMessageSenders>,
  >(
    &self,
    get_live_story_available_message_senders: C,
  ) -> RTDResult<ChatMessageSenders> {
    async_caller!(ChatMessageSenders);
    async_caller(&self.api, get_live_story_available_message_senders.as_ref()).await
  }

  pub async fn get_live_story_rtmp_url<C: AsRef<GetLiveStoryRtmpUrl>>(
    &self,
    get_live_story_rtmp_url: C,
  ) -> RTDResult<RtmpUrl> {
    async_caller!(RtmpUrl);
    async_caller(&self.api, get_live_story_rtmp_url.as_ref()).await
  }

  pub async fn get_live_story_streamer<C: AsRef<GetLiveStoryStreamer>>(
    &self,
    get_live_story_streamer: C,
  ) -> RTDResult<GroupCallParticipant> {
    async_caller!(GroupCallParticipant);
    async_caller(&self.api, get_live_story_streamer.as_ref()).await
  }

  pub async fn get_live_story_top_donors<C: AsRef<GetLiveStoryTopDonors>>(
    &self,
    get_live_story_top_donors: C,
  ) -> RTDResult<LiveStoryDonors> {
    async_caller!(LiveStoryDonors);
    async_caller(&self.api, get_live_story_top_donors.as_ref()).await
  }

  pub async fn get_localization_target_info<C: AsRef<GetLocalizationTargetInfo>>(
    &self,
    get_localization_target_info: C,
  ) -> RTDResult<LocalizationTargetInfo> {
    async_caller!(LocalizationTargetInfo);
    async_caller(&self.api, get_localization_target_info.as_ref()).await
  }

  pub async fn get_log_stream<C: AsRef<GetLogStream>>(
    &self,
    get_log_stream: C,
  ) -> RTDResult<LogStream> {
    async_caller!(LogStream);
    async_caller(&self.api, get_log_stream.as_ref()).await
  }

  pub async fn get_log_tag_verbosity_level<C: AsRef<GetLogTagVerbosityLevel>>(
    &self,
    get_log_tag_verbosity_level: C,
  ) -> RTDResult<LogVerbosityLevel> {
    async_caller!(LogVerbosityLevel);
    async_caller(&self.api, get_log_tag_verbosity_level.as_ref()).await
  }

  pub async fn get_log_tags<C: AsRef<GetLogTags>>(&self, get_log_tags: C) -> RTDResult<LogTags> {
    async_caller!(LogTags);
    async_caller(&self.api, get_log_tags.as_ref()).await
  }

  pub async fn get_log_verbosity_level<C: AsRef<GetLogVerbosityLevel>>(
    &self,
    get_log_verbosity_level: C,
  ) -> RTDResult<LogVerbosityLevel> {
    async_caller!(LogVerbosityLevel);
    async_caller(&self.api, get_log_verbosity_level.as_ref()).await
  }

  pub async fn get_login_passkeys<C: AsRef<GetLoginPasskeys>>(
    &self,
    get_login_passkeys: C,
  ) -> RTDResult<Passkeys> {
    async_caller!(Passkeys);
    async_caller(&self.api, get_login_passkeys.as_ref()).await
  }

  pub async fn get_login_url<C: AsRef<GetLoginUrl>>(&self, get_login_url: C) -> RTDResult<HttpUrl> {
    async_caller!(HttpUrl);
    async_caller(&self.api, get_login_url.as_ref()).await
  }

  pub async fn get_login_url_info<C: AsRef<GetLoginUrlInfo>>(
    &self,
    get_login_url_info: C,
  ) -> RTDResult<LoginUrlInfo> {
    async_caller!(LoginUrlInfo);
    async_caller(&self.api, get_login_url_info.as_ref()).await
  }

  pub async fn get_main_web_app<C: AsRef<GetMainWebApp>>(
    &self,
    get_main_web_app: C,
  ) -> RTDResult<MainWebApp> {
    async_caller!(MainWebApp);
    async_caller(&self.api, get_main_web_app.as_ref()).await
  }

  pub async fn get_managed_bot_access_settings<C: AsRef<GetManagedBotAccessSettings>>(
    &self,
    get_managed_bot_access_settings: C,
  ) -> RTDResult<BotAccessSettings> {
    async_caller!(BotAccessSettings);
    async_caller(&self.api, get_managed_bot_access_settings.as_ref()).await
  }

  pub async fn get_managed_bot_token<C: AsRef<GetManagedBotToken>>(
    &self,
    get_managed_bot_token: C,
  ) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, get_managed_bot_token.as_ref()).await
  }

  pub async fn get_map_thumbnail_file<C: AsRef<GetMapThumbnailFile>>(
    &self,
    get_map_thumbnail_file: C,
  ) -> RTDResult<File> {
    async_caller!(File);
    async_caller(&self.api, get_map_thumbnail_file.as_ref()).await
  }

  pub async fn get_markdown_text<C: AsRef<GetMarkdownText>>(
    &self,
    get_markdown_text: C,
  ) -> RTDResult<FormattedText> {
    async_caller!(FormattedText);
    async_caller(&self.api, get_markdown_text.as_ref()).await
  }

  pub async fn get_me<C: AsRef<GetMe>>(&self, get_me: C) -> RTDResult<User> {
    async_caller!(User);
    async_caller(&self.api, get_me.as_ref()).await
  }

  pub async fn get_menu_button<C: AsRef<GetMenuButton>>(
    &self,
    get_menu_button: C,
  ) -> RTDResult<BotMenuButton> {
    async_caller!(BotMenuButton);
    async_caller(&self.api, get_menu_button.as_ref()).await
  }

  pub async fn get_message<C: AsRef<GetMessage>>(&self, get_message: C) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, get_message.as_ref()).await
  }

  pub async fn get_message_added_reactions<C: AsRef<GetMessageAddedReactions>>(
    &self,
    get_message_added_reactions: C,
  ) -> RTDResult<AddedReactions> {
    async_caller!(AddedReactions);
    async_caller(&self.api, get_message_added_reactions.as_ref()).await
  }

  pub async fn get_message_author<C: AsRef<GetMessageAuthor>>(
    &self,
    get_message_author: C,
  ) -> RTDResult<User> {
    async_caller!(User);
    async_caller(&self.api, get_message_author.as_ref()).await
  }

  pub async fn get_message_available_reactions<C: AsRef<GetMessageAvailableReactions>>(
    &self,
    get_message_available_reactions: C,
  ) -> RTDResult<AvailableReactions> {
    async_caller!(AvailableReactions);
    async_caller(&self.api, get_message_available_reactions.as_ref()).await
  }

  pub async fn get_message_effect<C: AsRef<GetMessageEffect>>(
    &self,
    get_message_effect: C,
  ) -> RTDResult<MessageEffect> {
    async_caller!(MessageEffect);
    async_caller(&self.api, get_message_effect.as_ref()).await
  }

  pub async fn get_message_embedding_code<C: AsRef<GetMessageEmbeddingCode>>(
    &self,
    get_message_embedding_code: C,
  ) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, get_message_embedding_code.as_ref()).await
  }

  pub async fn get_message_file_type<C: AsRef<GetMessageFileType>>(
    &self,
    get_message_file_type: C,
  ) -> RTDResult<MessageFileType> {
    async_caller!(MessageFileType);
    async_caller(&self.api, get_message_file_type.as_ref()).await
  }

  pub async fn get_message_import_confirmation_text<C: AsRef<GetMessageImportConfirmationText>>(
    &self,
    get_message_import_confirmation_text: C,
  ) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, get_message_import_confirmation_text.as_ref()).await
  }

  pub async fn get_message_link<C: AsRef<GetMessageLink>>(
    &self,
    get_message_link: C,
  ) -> RTDResult<MessageLink> {
    async_caller!(MessageLink);
    async_caller(&self.api, get_message_link.as_ref()).await
  }

  pub async fn get_message_link_info<C: AsRef<GetMessageLinkInfo>>(
    &self,
    get_message_link_info: C,
  ) -> RTDResult<MessageLinkInfo> {
    async_caller!(MessageLinkInfo);
    async_caller(&self.api, get_message_link_info.as_ref()).await
  }

  pub async fn get_message_locally<C: AsRef<GetMessageLocally>>(
    &self,
    get_message_locally: C,
  ) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, get_message_locally.as_ref()).await
  }

  pub async fn get_message_properties<C: AsRef<GetMessageProperties>>(
    &self,
    get_message_properties: C,
  ) -> RTDResult<MessageProperties> {
    async_caller!(MessageProperties);
    async_caller(&self.api, get_message_properties.as_ref()).await
  }

  pub async fn get_message_public_forwards<C: AsRef<GetMessagePublicForwards>>(
    &self,
    get_message_public_forwards: C,
  ) -> RTDResult<PublicForwards> {
    async_caller!(PublicForwards);
    async_caller(&self.api, get_message_public_forwards.as_ref()).await
  }

  pub async fn get_message_read_date<C: AsRef<GetMessageReadDate>>(
    &self,
    get_message_read_date: C,
  ) -> RTDResult<MessageReadDate> {
    async_caller!(MessageReadDate);
    async_caller(&self.api, get_message_read_date.as_ref()).await
  }

  pub async fn get_message_statistics<C: AsRef<GetMessageStatistics>>(
    &self,
    get_message_statistics: C,
  ) -> RTDResult<MessageStatistics> {
    async_caller!(MessageStatistics);
    async_caller(&self.api, get_message_statistics.as_ref()).await
  }

  pub async fn get_message_thread<C: AsRef<GetMessageThread>>(
    &self,
    get_message_thread: C,
  ) -> RTDResult<MessageThreadInfo> {
    async_caller!(MessageThreadInfo);
    async_caller(&self.api, get_message_thread.as_ref()).await
  }

  pub async fn get_message_thread_history<C: AsRef<GetMessageThreadHistory>>(
    &self,
    get_message_thread_history: C,
  ) -> RTDResult<Messages> {
    async_caller!(Messages);
    async_caller(&self.api, get_message_thread_history.as_ref()).await
  }

  pub async fn get_message_viewers<C: AsRef<GetMessageViewers>>(
    &self,
    get_message_viewers: C,
  ) -> RTDResult<MessageViewers> {
    async_caller!(MessageViewers);
    async_caller(&self.api, get_message_viewers.as_ref()).await
  }

  pub async fn get_messages<C: AsRef<GetMessages>>(&self, get_messages: C) -> RTDResult<Messages> {
    async_caller!(Messages);
    async_caller(&self.api, get_messages.as_ref()).await
  }

  pub async fn get_network_statistics<C: AsRef<GetNetworkStatistics>>(
    &self,
    get_network_statistics: C,
  ) -> RTDResult<NetworkStatistics> {
    async_caller!(NetworkStatistics);
    async_caller(&self.api, get_network_statistics.as_ref()).await
  }

  pub async fn get_new_chat_privacy_settings<C: AsRef<GetNewChatPrivacySettings>>(
    &self,
    get_new_chat_privacy_settings: C,
  ) -> RTDResult<NewChatPrivacySettings> {
    async_caller!(NewChatPrivacySettings);
    async_caller(&self.api, get_new_chat_privacy_settings.as_ref()).await
  }

  pub async fn get_oauth_link_info<C: AsRef<GetOauthLinkInfo>>(
    &self,
    get_oauth_link_info: C,
  ) -> RTDResult<OauthLinkInfo> {
    async_caller!(OauthLinkInfo);
    async_caller(&self.api, get_oauth_link_info.as_ref()).await
  }

  pub async fn get_on_ramp_payment_availability<C: AsRef<GetOnRampPaymentAvailability>>(
    &self,
    get_on_ramp_payment_availability: C,
  ) -> RTDResult<OnRampPaymentAvailability> {
    async_caller!(OnRampPaymentAvailability);
    async_caller(&self.api, get_on_ramp_payment_availability.as_ref()).await
  }

  pub async fn get_on_ramp_payment_limits<C: AsRef<GetOnRampPaymentLimits>>(
    &self,
    get_on_ramp_payment_limits: C,
  ) -> RTDResult<OnRampPaymentLimits> {
    async_caller!(OnRampPaymentLimits);
    async_caller(&self.api, get_on_ramp_payment_limits.as_ref()).await
  }

  pub async fn get_on_ramp_payment_quote<C: AsRef<GetOnRampPaymentQuote>>(
    &self,
    get_on_ramp_payment_quote: C,
  ) -> RTDResult<OnRampPaymentQuote> {
    async_caller!(OnRampPaymentQuote);
    async_caller(&self.api, get_on_ramp_payment_quote.as_ref()).await
  }

  pub async fn get_on_ramp_provider_base_currencies<C: AsRef<GetOnRampProviderBaseCurrencies>>(
    &self,
    get_on_ramp_provider_base_currencies: C,
  ) -> RTDResult<Currencies> {
    async_caller!(Currencies);
    async_caller(&self.api, get_on_ramp_provider_base_currencies.as_ref()).await
  }

  pub async fn get_on_ramp_providers<C: AsRef<GetOnRampProviders>>(
    &self,
    get_on_ramp_providers: C,
  ) -> RTDResult<OnRampProviders> {
    async_caller!(OnRampProviders);
    async_caller(&self.api, get_on_ramp_providers.as_ref()).await
  }

  pub async fn get_option<C: AsRef<GetOption>>(&self, get_option: C) -> RTDResult<OptionValue> {
    async_caller!(OptionValue);
    async_caller(&self.api, get_option.as_ref()).await
  }

  pub async fn get_owned_bots<C: AsRef<GetOwnedBots>>(
    &self,
    get_owned_bots: C,
  ) -> RTDResult<Users> {
    async_caller!(Users);
    async_caller(&self.api, get_owned_bots.as_ref()).await
  }

  pub async fn get_owned_sticker_sets<C: AsRef<GetOwnedStickerSets>>(
    &self,
    get_owned_sticker_sets: C,
  ) -> RTDResult<StickerSets> {
    async_caller!(StickerSets);
    async_caller(&self.api, get_owned_sticker_sets.as_ref()).await
  }

  pub async fn get_paid_message_revenue<C: AsRef<GetPaidMessageRevenue>>(
    &self,
    get_paid_message_revenue: C,
  ) -> RTDResult<StarCount> {
    async_caller!(StarCount);
    async_caller(&self.api, get_paid_message_revenue.as_ref()).await
  }

  pub async fn get_passkey_parameters<C: AsRef<GetPasskeyParameters>>(
    &self,
    get_passkey_parameters: C,
  ) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, get_passkey_parameters.as_ref()).await
  }

  pub async fn get_passport_authorization_form<C: AsRef<GetPassportAuthorizationForm>>(
    &self,
    get_passport_authorization_form: C,
  ) -> RTDResult<PassportAuthorizationForm> {
    async_caller!(PassportAuthorizationForm);
    async_caller(&self.api, get_passport_authorization_form.as_ref()).await
  }

  pub async fn get_passport_authorization_form_available_elements<
    C: AsRef<GetPassportAuthorizationFormAvailableElements>,
  >(
    &self,
    get_passport_authorization_form_available_elements: C,
  ) -> RTDResult<PassportElementsWithErrors> {
    async_caller!(PassportElementsWithErrors);
    async_caller(
      &self.api,
      get_passport_authorization_form_available_elements.as_ref(),
    )
    .await
  }

  pub async fn get_passport_element<C: AsRef<GetPassportElement>>(
    &self,
    get_passport_element: C,
  ) -> RTDResult<PassportElement> {
    async_caller!(PassportElement);
    async_caller(&self.api, get_passport_element.as_ref()).await
  }

  pub async fn get_password_state<C: AsRef<GetPasswordState>>(
    &self,
    get_password_state: C,
  ) -> RTDResult<PasswordState> {
    async_caller!(PasswordState);
    async_caller(&self.api, get_password_state.as_ref()).await
  }

  pub async fn get_payment_form<C: AsRef<GetPaymentForm>>(
    &self,
    get_payment_form: C,
  ) -> RTDResult<PaymentForm> {
    async_caller!(PaymentForm);
    async_caller(&self.api, get_payment_form.as_ref()).await
  }

  pub async fn get_payment_receipt<C: AsRef<GetPaymentReceipt>>(
    &self,
    get_payment_receipt: C,
  ) -> RTDResult<PaymentReceipt> {
    async_caller!(PaymentReceipt);
    async_caller(&self.api, get_payment_receipt.as_ref()).await
  }

  pub async fn get_personal_chat_history<C: AsRef<GetPersonalChatHistory>>(
    &self,
    get_personal_chat_history: C,
  ) -> RTDResult<Messages> {
    async_caller!(Messages);
    async_caller(&self.api, get_personal_chat_history.as_ref()).await
  }

  pub async fn get_phone_number_info<C: AsRef<GetPhoneNumberInfo>>(
    &self,
    get_phone_number_info: C,
  ) -> RTDResult<PhoneNumberInfo> {
    async_caller!(PhoneNumberInfo);
    async_caller(&self.api, get_phone_number_info.as_ref()).await
  }

  pub async fn get_phone_number_info_sync<C: AsRef<GetPhoneNumberInfoSync>>(
    &self,
    get_phone_number_info_sync: C,
  ) -> RTDResult<PhoneNumberInfo> {
    async_caller!(PhoneNumberInfo);
    async_caller(&self.api, get_phone_number_info_sync.as_ref()).await
  }

  pub async fn get_poll_option_properties<C: AsRef<GetPollOptionProperties>>(
    &self,
    get_poll_option_properties: C,
  ) -> RTDResult<PollOptionProperties> {
    async_caller!(PollOptionProperties);
    async_caller(&self.api, get_poll_option_properties.as_ref()).await
  }

  pub async fn get_poll_vote_statistics<C: AsRef<GetPollVoteStatistics>>(
    &self,
    get_poll_vote_statistics: C,
  ) -> RTDResult<PollVoteStatistics> {
    async_caller!(PollVoteStatistics);
    async_caller(&self.api, get_poll_vote_statistics.as_ref()).await
  }

  pub async fn get_poll_voters<C: AsRef<GetPollVoters>>(
    &self,
    get_poll_voters: C,
  ) -> RTDResult<PollVoters> {
    async_caller!(PollVoters);
    async_caller(&self.api, get_poll_voters.as_ref()).await
  }

  pub async fn get_preferred_country_language<C: AsRef<GetPreferredCountryLanguage>>(
    &self,
    get_preferred_country_language: C,
  ) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, get_preferred_country_language.as_ref()).await
  }

  pub async fn get_premium_features<C: AsRef<GetPremiumFeatures>>(
    &self,
    get_premium_features: C,
  ) -> RTDResult<PremiumFeatures> {
    async_caller!(PremiumFeatures);
    async_caller(&self.api, get_premium_features.as_ref()).await
  }

  pub async fn get_premium_gift_payment_options<C: AsRef<GetPremiumGiftPaymentOptions>>(
    &self,
    get_premium_gift_payment_options: C,
  ) -> RTDResult<PremiumGiftPaymentOptions> {
    async_caller!(PremiumGiftPaymentOptions);
    async_caller(&self.api, get_premium_gift_payment_options.as_ref()).await
  }

  pub async fn get_premium_giveaway_payment_options<C: AsRef<GetPremiumGiveawayPaymentOptions>>(
    &self,
    get_premium_giveaway_payment_options: C,
  ) -> RTDResult<PremiumGiveawayPaymentOptions> {
    async_caller!(PremiumGiveawayPaymentOptions);
    async_caller(&self.api, get_premium_giveaway_payment_options.as_ref()).await
  }

  pub async fn get_premium_info_sticker<C: AsRef<GetPremiumInfoSticker>>(
    &self,
    get_premium_info_sticker: C,
  ) -> RTDResult<Sticker> {
    async_caller!(Sticker);
    async_caller(&self.api, get_premium_info_sticker.as_ref()).await
  }

  pub async fn get_premium_limit<C: AsRef<GetPremiumLimit>>(
    &self,
    get_premium_limit: C,
  ) -> RTDResult<PremiumLimit> {
    async_caller!(PremiumLimit);
    async_caller(&self.api, get_premium_limit.as_ref()).await
  }

  pub async fn get_premium_state<C: AsRef<GetPremiumState>>(
    &self,
    get_premium_state: C,
  ) -> RTDResult<PremiumState> {
    async_caller!(PremiumState);
    async_caller(&self.api, get_premium_state.as_ref()).await
  }

  pub async fn get_premium_sticker_examples<C: AsRef<GetPremiumStickerExamples>>(
    &self,
    get_premium_sticker_examples: C,
  ) -> RTDResult<Stickers> {
    async_caller!(Stickers);
    async_caller(&self.api, get_premium_sticker_examples.as_ref()).await
  }

  pub async fn get_premium_stickers<C: AsRef<GetPremiumStickers>>(
    &self,
    get_premium_stickers: C,
  ) -> RTDResult<Stickers> {
    async_caller!(Stickers);
    async_caller(&self.api, get_premium_stickers.as_ref()).await
  }

  pub async fn get_prepared_inline_message<C: AsRef<GetPreparedInlineMessage>>(
    &self,
    get_prepared_inline_message: C,
  ) -> RTDResult<PreparedInlineMessage> {
    async_caller!(PreparedInlineMessage);
    async_caller(&self.api, get_prepared_inline_message.as_ref()).await
  }

  pub async fn get_prepared_keyboard_button<C: AsRef<GetPreparedKeyboardButton>>(
    &self,
    get_prepared_keyboard_button: C,
  ) -> RTDResult<KeyboardButton> {
    async_caller!(KeyboardButton);
    async_caller(&self.api, get_prepared_keyboard_button.as_ref()).await
  }

  pub async fn get_profile_share_theme_backdrops<C: AsRef<GetProfileShareThemeBackdrops>>(
    &self,
    get_profile_share_theme_backdrops: C,
  ) -> RTDResult<ProfileShareThemeBackdrops> {
    async_caller!(ProfileShareThemeBackdrops);
    async_caller(&self.api, get_profile_share_theme_backdrops.as_ref()).await
  }

  pub async fn get_proxies<C: AsRef<GetProxies>>(&self, get_proxies: C) -> RTDResult<AddedProxies> {
    async_caller!(AddedProxies);
    async_caller(&self.api, get_proxies.as_ref()).await
  }

  pub async fn get_public_post_search_limits<C: AsRef<GetPublicPostSearchLimits>>(
    &self,
    get_public_post_search_limits: C,
  ) -> RTDResult<PublicPostSearchLimits> {
    async_caller!(PublicPostSearchLimits);
    async_caller(&self.api, get_public_post_search_limits.as_ref()).await
  }

  pub async fn get_push_receiver_id<C: AsRef<GetPushReceiverId>>(
    &self,
    get_push_receiver_id: C,
  ) -> RTDResult<PushReceiverId> {
    async_caller!(PushReceiverId);
    async_caller(&self.api, get_push_receiver_id.as_ref()).await
  }

  pub async fn get_read_date_privacy_settings<C: AsRef<GetReadDatePrivacySettings>>(
    &self,
    get_read_date_privacy_settings: C,
  ) -> RTDResult<ReadDatePrivacySettings> {
    async_caller!(ReadDatePrivacySettings);
    async_caller(&self.api, get_read_date_privacy_settings.as_ref()).await
  }

  pub async fn get_received_gift<C: AsRef<GetReceivedGift>>(
    &self,
    get_received_gift: C,
  ) -> RTDResult<ReceivedGift> {
    async_caller!(ReceivedGift);
    async_caller(&self.api, get_received_gift.as_ref()).await
  }

  pub async fn get_received_gifts<C: AsRef<GetReceivedGifts>>(
    &self,
    get_received_gifts: C,
  ) -> RTDResult<ReceivedGifts> {
    async_caller!(ReceivedGifts);
    async_caller(&self.api, get_received_gifts.as_ref()).await
  }

  pub async fn get_recent_emoji_statuses<C: AsRef<GetRecentEmojiStatuses>>(
    &self,
    get_recent_emoji_statuses: C,
  ) -> RTDResult<EmojiStatuses> {
    async_caller!(EmojiStatuses);
    async_caller(&self.api, get_recent_emoji_statuses.as_ref()).await
  }

  pub async fn get_recent_inline_bots<C: AsRef<GetRecentInlineBots>>(
    &self,
    get_recent_inline_bots: C,
  ) -> RTDResult<Users> {
    async_caller!(Users);
    async_caller(&self.api, get_recent_inline_bots.as_ref()).await
  }

  pub async fn get_recent_stickers<C: AsRef<GetRecentStickers>>(
    &self,
    get_recent_stickers: C,
  ) -> RTDResult<Stickers> {
    async_caller!(Stickers);
    async_caller(&self.api, get_recent_stickers.as_ref()).await
  }

  pub async fn get_recently_opened_chats<C: AsRef<GetRecentlyOpenedChats>>(
    &self,
    get_recently_opened_chats: C,
  ) -> RTDResult<Chats> {
    async_caller!(Chats);
    async_caller(&self.api, get_recently_opened_chats.as_ref()).await
  }

  pub async fn get_recently_visited_t_me_urls<C: AsRef<GetRecentlyVisitedTMeUrls>>(
    &self,
    get_recently_visited_t_me_urls: C,
  ) -> RTDResult<TMeUrls> {
    async_caller!(TMeUrls);
    async_caller(&self.api, get_recently_visited_t_me_urls.as_ref()).await
  }

  pub async fn get_recommended_chat_folders<C: AsRef<GetRecommendedChatFolders>>(
    &self,
    get_recommended_chat_folders: C,
  ) -> RTDResult<RecommendedChatFolders> {
    async_caller!(RecommendedChatFolders);
    async_caller(&self.api, get_recommended_chat_folders.as_ref()).await
  }

  pub async fn get_recommended_chats<C: AsRef<GetRecommendedChats>>(
    &self,
    get_recommended_chats: C,
  ) -> RTDResult<Chats> {
    async_caller!(Chats);
    async_caller(&self.api, get_recommended_chats.as_ref()).await
  }

  pub async fn get_recovery_email_address<C: AsRef<GetRecoveryEmailAddress>>(
    &self,
    get_recovery_email_address: C,
  ) -> RTDResult<RecoveryEmailAddress> {
    async_caller!(RecoveryEmailAddress);
    async_caller(&self.api, get_recovery_email_address.as_ref()).await
  }

  pub async fn get_remote_file<C: AsRef<GetRemoteFile>>(
    &self,
    get_remote_file: C,
  ) -> RTDResult<File> {
    async_caller!(File);
    async_caller(&self.api, get_remote_file.as_ref()).await
  }

  pub async fn get_replied_message<C: AsRef<GetRepliedMessage>>(
    &self,
    get_replied_message: C,
  ) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, get_replied_message.as_ref()).await
  }

  pub async fn get_saved_animations<C: AsRef<GetSavedAnimations>>(
    &self,
    get_saved_animations: C,
  ) -> RTDResult<Animations> {
    async_caller!(Animations);
    async_caller(&self.api, get_saved_animations.as_ref()).await
  }

  pub async fn get_saved_messages_tags<C: AsRef<GetSavedMessagesTags>>(
    &self,
    get_saved_messages_tags: C,
  ) -> RTDResult<SavedMessagesTags> {
    async_caller!(SavedMessagesTags);
    async_caller(&self.api, get_saved_messages_tags.as_ref()).await
  }

  pub async fn get_saved_messages_topic_history<C: AsRef<GetSavedMessagesTopicHistory>>(
    &self,
    get_saved_messages_topic_history: C,
  ) -> RTDResult<Messages> {
    async_caller!(Messages);
    async_caller(&self.api, get_saved_messages_topic_history.as_ref()).await
  }

  pub async fn get_saved_messages_topic_message_by_date<
    C: AsRef<GetSavedMessagesTopicMessageByDate>,
  >(
    &self,
    get_saved_messages_topic_message_by_date: C,
  ) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, get_saved_messages_topic_message_by_date.as_ref()).await
  }

  pub async fn get_saved_notification_sound<C: AsRef<GetSavedNotificationSound>>(
    &self,
    get_saved_notification_sound: C,
  ) -> RTDResult<NotificationSound> {
    async_caller!(NotificationSound);
    async_caller(&self.api, get_saved_notification_sound.as_ref()).await
  }

  pub async fn get_saved_notification_sounds<C: AsRef<GetSavedNotificationSounds>>(
    &self,
    get_saved_notification_sounds: C,
  ) -> RTDResult<NotificationSounds> {
    async_caller!(NotificationSounds);
    async_caller(&self.api, get_saved_notification_sounds.as_ref()).await
  }

  pub async fn get_saved_order_info<C: AsRef<GetSavedOrderInfo>>(
    &self,
    get_saved_order_info: C,
  ) -> RTDResult<OrderInfo> {
    async_caller!(OrderInfo);
    async_caller(&self.api, get_saved_order_info.as_ref()).await
  }

  pub async fn get_scope_notification_settings<C: AsRef<GetScopeNotificationSettings>>(
    &self,
    get_scope_notification_settings: C,
  ) -> RTDResult<ScopeNotificationSettings> {
    async_caller!(ScopeNotificationSettings);
    async_caller(&self.api, get_scope_notification_settings.as_ref()).await
  }

  pub async fn get_search_sponsored_chats<C: AsRef<GetSearchSponsoredChats>>(
    &self,
    get_search_sponsored_chats: C,
  ) -> RTDResult<SponsoredChats> {
    async_caller!(SponsoredChats);
    async_caller(&self.api, get_search_sponsored_chats.as_ref()).await
  }

  pub async fn get_searched_for_tags<C: AsRef<GetSearchedForTags>>(
    &self,
    get_searched_for_tags: C,
  ) -> RTDResult<Hashtags> {
    async_caller!(Hashtags);
    async_caller(&self.api, get_searched_for_tags.as_ref()).await
  }

  pub async fn get_secret_chat<C: AsRef<GetSecretChat>>(
    &self,
    get_secret_chat: C,
  ) -> RTDResult<SecretChat> {
    async_caller!(SecretChat);
    async_caller(&self.api, get_secret_chat.as_ref()).await
  }

  pub async fn get_stake_dice_state<C: AsRef<GetStakeDiceState>>(
    &self,
    get_stake_dice_state: C,
  ) -> RTDResult<StakeDiceState> {
    async_caller!(StakeDiceState);
    async_caller(&self.api, get_stake_dice_state.as_ref()).await
  }

  pub async fn get_star_ad_account_url<C: AsRef<GetStarAdAccountUrl>>(
    &self,
    get_star_ad_account_url: C,
  ) -> RTDResult<HttpUrl> {
    async_caller!(HttpUrl);
    async_caller(&self.api, get_star_ad_account_url.as_ref()).await
  }

  pub async fn get_star_gift_payment_options<C: AsRef<GetStarGiftPaymentOptions>>(
    &self,
    get_star_gift_payment_options: C,
  ) -> RTDResult<StarPaymentOptions> {
    async_caller!(StarPaymentOptions);
    async_caller(&self.api, get_star_gift_payment_options.as_ref()).await
  }

  pub async fn get_star_giveaway_payment_options<C: AsRef<GetStarGiveawayPaymentOptions>>(
    &self,
    get_star_giveaway_payment_options: C,
  ) -> RTDResult<StarGiveawayPaymentOptions> {
    async_caller!(StarGiveawayPaymentOptions);
    async_caller(&self.api, get_star_giveaway_payment_options.as_ref()).await
  }

  pub async fn get_star_payment_options<C: AsRef<GetStarPaymentOptions>>(
    &self,
    get_star_payment_options: C,
  ) -> RTDResult<StarPaymentOptions> {
    async_caller!(StarPaymentOptions);
    async_caller(&self.api, get_star_payment_options.as_ref()).await
  }

  pub async fn get_star_revenue_statistics<C: AsRef<GetStarRevenueStatistics>>(
    &self,
    get_star_revenue_statistics: C,
  ) -> RTDResult<StarRevenueStatistics> {
    async_caller!(StarRevenueStatistics);
    async_caller(&self.api, get_star_revenue_statistics.as_ref()).await
  }

  pub async fn get_star_subscriptions<C: AsRef<GetStarSubscriptions>>(
    &self,
    get_star_subscriptions: C,
  ) -> RTDResult<StarSubscriptions> {
    async_caller!(StarSubscriptions);
    async_caller(&self.api, get_star_subscriptions.as_ref()).await
  }

  pub async fn get_star_transactions<C: AsRef<GetStarTransactions>>(
    &self,
    get_star_transactions: C,
  ) -> RTDResult<StarTransactions> {
    async_caller!(StarTransactions);
    async_caller(&self.api, get_star_transactions.as_ref()).await
  }

  pub async fn get_star_withdrawal_url<C: AsRef<GetStarWithdrawalUrl>>(
    &self,
    get_star_withdrawal_url: C,
  ) -> RTDResult<HttpUrl> {
    async_caller!(HttpUrl);
    async_caller(&self.api, get_star_withdrawal_url.as_ref()).await
  }

  pub async fn get_statistical_graph<C: AsRef<GetStatisticalGraph>>(
    &self,
    get_statistical_graph: C,
  ) -> RTDResult<StatisticalGraph> {
    async_caller!(StatisticalGraph);
    async_caller(&self.api, get_statistical_graph.as_ref()).await
  }

  pub async fn get_sticker_emojis<C: AsRef<GetStickerEmojis>>(
    &self,
    get_sticker_emojis: C,
  ) -> RTDResult<Emojis> {
    async_caller!(Emojis);
    async_caller(&self.api, get_sticker_emojis.as_ref()).await
  }

  pub async fn get_sticker_outline<C: AsRef<GetStickerOutline>>(
    &self,
    get_sticker_outline: C,
  ) -> RTDResult<Outline> {
    async_caller!(Outline);
    async_caller(&self.api, get_sticker_outline.as_ref()).await
  }

  pub async fn get_sticker_outline_svg_path<C: AsRef<GetStickerOutlineSvgPath>>(
    &self,
    get_sticker_outline_svg_path: C,
  ) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, get_sticker_outline_svg_path.as_ref()).await
  }

  pub async fn get_sticker_set<C: AsRef<GetStickerSet>>(
    &self,
    get_sticker_set: C,
  ) -> RTDResult<StickerSet> {
    async_caller!(StickerSet);
    async_caller(&self.api, get_sticker_set.as_ref()).await
  }

  pub async fn get_sticker_set_name<C: AsRef<GetStickerSetName>>(
    &self,
    get_sticker_set_name: C,
  ) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, get_sticker_set_name.as_ref()).await
  }

  pub async fn get_stickers<C: AsRef<GetStickers>>(&self, get_stickers: C) -> RTDResult<Stickers> {
    async_caller!(Stickers);
    async_caller(&self.api, get_stickers.as_ref()).await
  }

  pub async fn get_storage_statistics<C: AsRef<GetStorageStatistics>>(
    &self,
    get_storage_statistics: C,
  ) -> RTDResult<StorageStatistics> {
    async_caller!(StorageStatistics);
    async_caller(&self.api, get_storage_statistics.as_ref()).await
  }

  pub async fn get_storage_statistics_fast<C: AsRef<GetStorageStatisticsFast>>(
    &self,
    get_storage_statistics_fast: C,
  ) -> RTDResult<StorageStatisticsFast> {
    async_caller!(StorageStatisticsFast);
    async_caller(&self.api, get_storage_statistics_fast.as_ref()).await
  }

  pub async fn get_story<C: AsRef<GetStory>>(&self, get_story: C) -> RTDResult<Story> {
    async_caller!(Story);
    async_caller(&self.api, get_story.as_ref()).await
  }

  pub async fn get_story_album_stories<C: AsRef<GetStoryAlbumStories>>(
    &self,
    get_story_album_stories: C,
  ) -> RTDResult<Stories> {
    async_caller!(Stories);
    async_caller(&self.api, get_story_album_stories.as_ref()).await
  }

  pub async fn get_story_available_reactions<C: AsRef<GetStoryAvailableReactions>>(
    &self,
    get_story_available_reactions: C,
  ) -> RTDResult<AvailableReactions> {
    async_caller!(AvailableReactions);
    async_caller(&self.api, get_story_available_reactions.as_ref()).await
  }

  pub async fn get_story_interactions<C: AsRef<GetStoryInteractions>>(
    &self,
    get_story_interactions: C,
  ) -> RTDResult<StoryInteractions> {
    async_caller!(StoryInteractions);
    async_caller(&self.api, get_story_interactions.as_ref()).await
  }

  pub async fn get_story_notification_settings_exceptions<
    C: AsRef<GetStoryNotificationSettingsExceptions>,
  >(
    &self,
    get_story_notification_settings_exceptions: C,
  ) -> RTDResult<Chats> {
    async_caller!(Chats);
    async_caller(
      &self.api,
      get_story_notification_settings_exceptions.as_ref(),
    )
    .await
  }

  pub async fn get_story_public_forwards<C: AsRef<GetStoryPublicForwards>>(
    &self,
    get_story_public_forwards: C,
  ) -> RTDResult<PublicForwards> {
    async_caller!(PublicForwards);
    async_caller(&self.api, get_story_public_forwards.as_ref()).await
  }

  pub async fn get_story_statistics<C: AsRef<GetStoryStatistics>>(
    &self,
    get_story_statistics: C,
  ) -> RTDResult<StoryStatistics> {
    async_caller!(StoryStatistics);
    async_caller(&self.api, get_story_statistics.as_ref()).await
  }

  pub async fn get_suggested_file_name<C: AsRef<GetSuggestedFileName>>(
    &self,
    get_suggested_file_name: C,
  ) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, get_suggested_file_name.as_ref()).await
  }

  pub async fn get_suggested_sticker_set_name<C: AsRef<GetSuggestedStickerSetName>>(
    &self,
    get_suggested_sticker_set_name: C,
  ) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, get_suggested_sticker_set_name.as_ref()).await
  }

  pub async fn get_suitable_discussion_chats<C: AsRef<GetSuitableDiscussionChats>>(
    &self,
    get_suitable_discussion_chats: C,
  ) -> RTDResult<Chats> {
    async_caller!(Chats);
    async_caller(&self.api, get_suitable_discussion_chats.as_ref()).await
  }

  pub async fn get_suitable_personal_chats<C: AsRef<GetSuitablePersonalChats>>(
    &self,
    get_suitable_personal_chats: C,
  ) -> RTDResult<Chats> {
    async_caller!(Chats);
    async_caller(&self.api, get_suitable_personal_chats.as_ref()).await
  }

  pub async fn get_supergroup<C: AsRef<GetSupergroup>>(
    &self,
    get_supergroup: C,
  ) -> RTDResult<Supergroup> {
    async_caller!(Supergroup);
    async_caller(&self.api, get_supergroup.as_ref()).await
  }

  pub async fn get_supergroup_full_info<C: AsRef<GetSupergroupFullInfo>>(
    &self,
    get_supergroup_full_info: C,
  ) -> RTDResult<SupergroupFullInfo> {
    async_caller!(SupergroupFullInfo);
    async_caller(&self.api, get_supergroup_full_info.as_ref()).await
  }

  pub async fn get_supergroup_members<C: AsRef<GetSupergroupMembers>>(
    &self,
    get_supergroup_members: C,
  ) -> RTDResult<ChatMembers> {
    async_caller!(ChatMembers);
    async_caller(&self.api, get_supergroup_members.as_ref()).await
  }

  pub async fn get_support_name<C: AsRef<GetSupportName>>(
    &self,
    get_support_name: C,
  ) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, get_support_name.as_ref()).await
  }

  pub async fn get_support_user<C: AsRef<GetSupportUser>>(
    &self,
    get_support_user: C,
  ) -> RTDResult<User> {
    async_caller!(User);
    async_caller(&self.api, get_support_user.as_ref()).await
  }

  pub async fn get_temporary_password_state<C: AsRef<GetTemporaryPasswordState>>(
    &self,
    get_temporary_password_state: C,
  ) -> RTDResult<TemporaryPasswordState> {
    async_caller!(TemporaryPasswordState);
    async_caller(&self.api, get_temporary_password_state.as_ref()).await
  }

  pub async fn get_text_composition_style_example<C: AsRef<GetTextCompositionStyleExample>>(
    &self,
    get_text_composition_style_example: C,
  ) -> RTDResult<TextCompositionStyleExample> {
    async_caller!(TextCompositionStyleExample);
    async_caller(&self.api, get_text_composition_style_example.as_ref()).await
  }

  pub async fn get_text_entities<C: AsRef<GetTextEntities>>(
    &self,
    get_text_entities: C,
  ) -> RTDResult<TextEntities> {
    async_caller!(TextEntities);
    async_caller(&self.api, get_text_entities.as_ref()).await
  }

  pub async fn get_theme_parameters_json_string<C: AsRef<GetThemeParametersJsonString>>(
    &self,
    get_theme_parameters_json_string: C,
  ) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, get_theme_parameters_json_string.as_ref()).await
  }

  pub async fn get_themed_chat_emoji_statuses<C: AsRef<GetThemedChatEmojiStatuses>>(
    &self,
    get_themed_chat_emoji_statuses: C,
  ) -> RTDResult<EmojiStatusCustomEmojis> {
    async_caller!(EmojiStatusCustomEmojis);
    async_caller(&self.api, get_themed_chat_emoji_statuses.as_ref()).await
  }

  pub async fn get_themed_emoji_statuses<C: AsRef<GetThemedEmojiStatuses>>(
    &self,
    get_themed_emoji_statuses: C,
  ) -> RTDResult<EmojiStatusCustomEmojis> {
    async_caller!(EmojiStatusCustomEmojis);
    async_caller(&self.api, get_themed_emoji_statuses.as_ref()).await
  }

  pub async fn get_time_zones<C: AsRef<GetTimeZones>>(
    &self,
    get_time_zones: C,
  ) -> RTDResult<TimeZones> {
    async_caller!(TimeZones);
    async_caller(&self.api, get_time_zones.as_ref()).await
  }

  pub async fn get_ton_center_streaming_api_url<C: AsRef<GetTonCenterStreamingApiUrl>>(
    &self,
    get_ton_center_streaming_api_url: C,
  ) -> RTDResult<TonCenterStreamingApiUrl> {
    async_caller!(TonCenterStreamingApiUrl);
    async_caller(&self.api, get_ton_center_streaming_api_url.as_ref()).await
  }

  pub async fn get_ton_connect_d_app_pending_requests<
    C: AsRef<GetTonConnectDAppPendingRequests>,
  >(
    &self,
    get_ton_connect_d_app_pending_requests: C,
  ) -> RTDResult<TonConnectRequests> {
    async_caller!(TonConnectRequests);
    async_caller(&self.api, get_ton_connect_d_app_pending_requests.as_ref()).await
  }

  pub async fn get_ton_connect_session_next_event_id<C: AsRef<GetTonConnectSessionNextEventId>>(
    &self,
    get_ton_connect_session_next_event_id: C,
  ) -> RTDResult<TonConnectSessionEventId> {
    async_caller!(TonConnectSessionEventId);
    async_caller(&self.api, get_ton_connect_session_next_event_id.as_ref()).await
  }

  pub async fn get_ton_connect_session_pending_requests<
    C: AsRef<GetTonConnectSessionPendingRequests>,
  >(
    &self,
    get_ton_connect_session_pending_requests: C,
  ) -> RTDResult<TonConnectRequests> {
    async_caller!(TonConnectRequests);
    async_caller(&self.api, get_ton_connect_session_pending_requests.as_ref()).await
  }

  pub async fn get_ton_connect_sessions<C: AsRef<GetTonConnectSessions>>(
    &self,
    get_ton_connect_sessions: C,
  ) -> RTDResult<TonConnectSessions> {
    async_caller!(TonConnectSessions);
    async_caller(&self.api, get_ton_connect_sessions.as_ref()).await
  }

  pub async fn get_ton_transactions<C: AsRef<GetTonTransactions>>(
    &self,
    get_ton_transactions: C,
  ) -> RTDResult<TonTransactions> {
    async_caller!(TonTransactions);
    async_caller(&self.api, get_ton_transactions.as_ref()).await
  }

  pub async fn get_ton_wallet_card_background_gift_collection_names<
    C: AsRef<GetTonWalletCardBackgroundGiftCollectionNames>,
  >(
    &self,
    get_ton_wallet_card_background_gift_collection_names: C,
  ) -> RTDResult<UpgradedGiftCollectionNames> {
    async_caller!(UpgradedGiftCollectionNames);
    async_caller(
      &self.api,
      get_ton_wallet_card_background_gift_collection_names.as_ref(),
    )
    .await
  }

  pub async fn get_ton_wallet_nfts<C: AsRef<GetTonWalletNfts>>(
    &self,
    get_ton_wallet_nfts: C,
  ) -> RTDResult<TonNfts> {
    async_caller!(TonNfts);
    async_caller(&self.api, get_ton_wallet_nfts.as_ref()).await
  }

  pub async fn get_ton_wallet_ownership_proof_challenge<
    C: AsRef<GetTonWalletOwnershipProofChallenge>,
  >(
    &self,
    get_ton_wallet_ownership_proof_challenge: C,
  ) -> RTDResult<TonWalletOwnershipProofChallenge> {
    async_caller!(TonWalletOwnershipProofChallenge);
    async_caller(&self.api, get_ton_wallet_ownership_proof_challenge.as_ref()).await
  }

  pub async fn get_ton_wallet_secret_phrase<C: AsRef<GetTonWalletSecretPhrase>>(
    &self,
    get_ton_wallet_secret_phrase: C,
  ) -> RTDResult<TonWalletSecretPhrase> {
    async_caller!(TonWalletSecretPhrase);
    async_caller(&self.api, get_ton_wallet_secret_phrase.as_ref()).await
  }

  pub async fn get_ton_wallet_transaction<C: AsRef<GetTonWalletTransaction>>(
    &self,
    get_ton_wallet_transaction: C,
  ) -> RTDResult<TonWalletTransaction> {
    async_caller!(TonWalletTransaction);
    async_caller(&self.api, get_ton_wallet_transaction.as_ref()).await
  }

  pub async fn get_ton_wallet_transaction_by_msg_hash<
    C: AsRef<GetTonWalletTransactionByMsgHash>,
  >(
    &self,
    get_ton_wallet_transaction_by_msg_hash: C,
  ) -> RTDResult<TonWalletTransaction> {
    async_caller!(TonWalletTransaction);
    async_caller(&self.api, get_ton_wallet_transaction_by_msg_hash.as_ref()).await
  }

  pub async fn get_ton_wallet_transactions<C: AsRef<GetTonWalletTransactions>>(
    &self,
    get_ton_wallet_transactions: C,
  ) -> RTDResult<TonWalletTransactions> {
    async_caller!(TonWalletTransactions);
    async_caller(&self.api, get_ton_wallet_transactions.as_ref()).await
  }

  pub async fn get_top_chats<C: AsRef<GetTopChats>>(&self, get_top_chats: C) -> RTDResult<Chats> {
    async_caller!(Chats);
    async_caller(&self.api, get_top_chats.as_ref()).await
  }

  pub async fn get_trending_sticker_sets<C: AsRef<GetTrendingStickerSets>>(
    &self,
    get_trending_sticker_sets: C,
  ) -> RTDResult<TrendingStickerSets> {
    async_caller!(TrendingStickerSets);
    async_caller(&self.api, get_trending_sticker_sets.as_ref()).await
  }

  pub async fn get_upgraded_gift<C: AsRef<GetUpgradedGift>>(
    &self,
    get_upgraded_gift: C,
  ) -> RTDResult<UpgradedGift> {
    async_caller!(UpgradedGift);
    async_caller(&self.api, get_upgraded_gift.as_ref()).await
  }

  pub async fn get_upgraded_gift_deposit_info<C: AsRef<GetUpgradedGiftDepositInfo>>(
    &self,
    get_upgraded_gift_deposit_info: C,
  ) -> RTDResult<UpgradedGiftDepositInfo> {
    async_caller!(UpgradedGiftDepositInfo);
    async_caller(&self.api, get_upgraded_gift_deposit_info.as_ref()).await
  }

  pub async fn get_upgraded_gift_emoji_statuses<C: AsRef<GetUpgradedGiftEmojiStatuses>>(
    &self,
    get_upgraded_gift_emoji_statuses: C,
  ) -> RTDResult<EmojiStatuses> {
    async_caller!(EmojiStatuses);
    async_caller(&self.api, get_upgraded_gift_emoji_statuses.as_ref()).await
  }

  pub async fn get_upgraded_gift_value_info<C: AsRef<GetUpgradedGiftValueInfo>>(
    &self,
    get_upgraded_gift_value_info: C,
  ) -> RTDResult<UpgradedGiftValueInfo> {
    async_caller!(UpgradedGiftValueInfo);
    async_caller(&self.api, get_upgraded_gift_value_info.as_ref()).await
  }

  pub async fn get_upgraded_gift_variants<C: AsRef<GetUpgradedGiftVariants>>(
    &self,
    get_upgraded_gift_variants: C,
  ) -> RTDResult<GiftUpgradeVariants> {
    async_caller!(GiftUpgradeVariants);
    async_caller(&self.api, get_upgraded_gift_variants.as_ref()).await
  }

  pub async fn get_upgraded_gift_withdrawal_url<C: AsRef<GetUpgradedGiftWithdrawalUrl>>(
    &self,
    get_upgraded_gift_withdrawal_url: C,
  ) -> RTDResult<HttpUrl> {
    async_caller!(HttpUrl);
    async_caller(&self.api, get_upgraded_gift_withdrawal_url.as_ref()).await
  }

  pub async fn get_upgraded_gifts_promotional_animation<
    C: AsRef<GetUpgradedGiftsPromotionalAnimation>,
  >(
    &self,
    get_upgraded_gifts_promotional_animation: C,
  ) -> RTDResult<Animation> {
    async_caller!(Animation);
    async_caller(&self.api, get_upgraded_gifts_promotional_animation.as_ref()).await
  }

  pub async fn get_user<C: AsRef<GetUser>>(&self, get_user: C) -> RTDResult<User> {
    async_caller!(User);
    async_caller(&self.api, get_user.as_ref()).await
  }

  pub async fn get_user_chat_boosts<C: AsRef<GetUserChatBoosts>>(
    &self,
    get_user_chat_boosts: C,
  ) -> RTDResult<FoundChatBoosts> {
    async_caller!(FoundChatBoosts);
    async_caller(&self.api, get_user_chat_boosts.as_ref()).await
  }

  pub async fn get_user_full_info<C: AsRef<GetUserFullInfo>>(
    &self,
    get_user_full_info: C,
  ) -> RTDResult<UserFullInfo> {
    async_caller!(UserFullInfo);
    async_caller(&self.api, get_user_full_info.as_ref()).await
  }

  pub async fn get_user_link<C: AsRef<GetUserLink>>(
    &self,
    get_user_link: C,
  ) -> RTDResult<UserLink> {
    async_caller!(UserLink);
    async_caller(&self.api, get_user_link.as_ref()).await
  }

  pub async fn get_user_privacy_setting_rules<C: AsRef<GetUserPrivacySettingRules>>(
    &self,
    get_user_privacy_setting_rules: C,
  ) -> RTDResult<UserPrivacySettingRules> {
    async_caller!(UserPrivacySettingRules);
    async_caller(&self.api, get_user_privacy_setting_rules.as_ref()).await
  }

  pub async fn get_user_profile_audios<C: AsRef<GetUserProfileAudios>>(
    &self,
    get_user_profile_audios: C,
  ) -> RTDResult<Audios> {
    async_caller!(Audios);
    async_caller(&self.api, get_user_profile_audios.as_ref()).await
  }

  pub async fn get_user_profile_photos<C: AsRef<GetUserProfilePhotos>>(
    &self,
    get_user_profile_photos: C,
  ) -> RTDResult<ChatPhotos> {
    async_caller!(ChatPhotos);
    async_caller(&self.api, get_user_profile_photos.as_ref()).await
  }

  pub async fn get_user_support_info<C: AsRef<GetUserSupportInfo>>(
    &self,
    get_user_support_info: C,
  ) -> RTDResult<UserSupportInfo> {
    async_caller!(UserSupportInfo);
    async_caller(&self.api, get_user_support_info.as_ref()).await
  }

  pub async fn get_user_ton_wallet_addresses<C: AsRef<GetUserTonWalletAddresses>>(
    &self,
    get_user_ton_wallet_addresses: C,
  ) -> RTDResult<UserTonWalletAddresses> {
    async_caller!(UserTonWalletAddresses);
    async_caller(&self.api, get_user_ton_wallet_addresses.as_ref()).await
  }

  pub async fn get_video_chat_available_participants<
    C: AsRef<GetVideoChatAvailableParticipants>,
  >(
    &self,
    get_video_chat_available_participants: C,
  ) -> RTDResult<MessageSenders> {
    async_caller!(MessageSenders);
    async_caller(&self.api, get_video_chat_available_participants.as_ref()).await
  }

  pub async fn get_video_chat_invite_link<C: AsRef<GetVideoChatInviteLink>>(
    &self,
    get_video_chat_invite_link: C,
  ) -> RTDResult<HttpUrl> {
    async_caller!(HttpUrl);
    async_caller(&self.api, get_video_chat_invite_link.as_ref()).await
  }

  pub async fn get_video_chat_rtmp_url<C: AsRef<GetVideoChatRtmpUrl>>(
    &self,
    get_video_chat_rtmp_url: C,
  ) -> RTDResult<RtmpUrl> {
    async_caller!(RtmpUrl);
    async_caller(&self.api, get_video_chat_rtmp_url.as_ref()).await
  }

  pub async fn get_video_message_advertisements<C: AsRef<GetVideoMessageAdvertisements>>(
    &self,
    get_video_message_advertisements: C,
  ) -> RTDResult<VideoMessageAdvertisements> {
    async_caller!(VideoMessageAdvertisements);
    async_caller(&self.api, get_video_message_advertisements.as_ref()).await
  }

  pub async fn get_web_app_link_url<C: AsRef<GetWebAppLinkUrl>>(
    &self,
    get_web_app_link_url: C,
  ) -> RTDResult<WebAppUrl> {
    async_caller!(WebAppUrl);
    async_caller(&self.api, get_web_app_link_url.as_ref()).await
  }

  pub async fn get_web_app_placeholder<C: AsRef<GetWebAppPlaceholder>>(
    &self,
    get_web_app_placeholder: C,
  ) -> RTDResult<Outline> {
    async_caller!(Outline);
    async_caller(&self.api, get_web_app_placeholder.as_ref()).await
  }

  pub async fn get_web_app_url<C: AsRef<GetWebAppUrl>>(
    &self,
    get_web_app_url: C,
  ) -> RTDResult<WebAppUrl> {
    async_caller!(WebAppUrl);
    async_caller(&self.api, get_web_app_url.as_ref()).await
  }

  pub async fn get_web_page_instant_view<C: AsRef<GetWebPageInstantView>>(
    &self,
    get_web_page_instant_view: C,
  ) -> RTDResult<WebPageInstantView> {
    async_caller!(WebPageInstantView);
    async_caller(&self.api, get_web_page_instant_view.as_ref()).await
  }

  pub async fn gift_premium_with_stars<C: AsRef<GiftPremiumWithStars>>(
    &self,
    gift_premium_with_stars: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, gift_premium_with_stars.as_ref()).await
  }

  pub async fn hide_contact_close_birthdays<C: AsRef<HideContactCloseBirthdays>>(
    &self,
    hide_contact_close_birthdays: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, hide_contact_close_birthdays.as_ref()).await
  }

  pub async fn hide_suggested_action<C: AsRef<HideSuggestedAction>>(
    &self,
    hide_suggested_action: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, hide_suggested_action.as_ref()).await
  }

  pub async fn import_contacts<C: AsRef<ImportContacts>>(
    &self,
    import_contacts: C,
  ) -> RTDResult<ImportedContacts> {
    async_caller!(ImportedContacts);
    async_caller(&self.api, import_contacts.as_ref()).await
  }

  pub async fn import_messages<C: AsRef<ImportMessages>>(
    &self,
    import_messages: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, import_messages.as_ref()).await
  }

  pub async fn increase_gift_auction_bid<C: AsRef<IncreaseGiftAuctionBid>>(
    &self,
    increase_gift_auction_bid: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, increase_gift_auction_bid.as_ref()).await
  }

  pub async fn invite_group_call_participant<C: AsRef<InviteGroupCallParticipant>>(
    &self,
    invite_group_call_participant: C,
  ) -> RTDResult<InviteGroupCallParticipantResult> {
    async_caller!(InviteGroupCallParticipantResult);
    async_caller(&self.api, invite_group_call_participant.as_ref()).await
  }

  pub async fn invite_video_chat_participants<C: AsRef<InviteVideoChatParticipants>>(
    &self,
    invite_video_chat_participants: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, invite_video_chat_participants.as_ref()).await
  }

  pub async fn is_login_email_address_required<C: AsRef<IsLoginEmailAddressRequired>>(
    &self,
    is_login_email_address_required: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, is_login_email_address_required.as_ref()).await
  }

  pub async fn is_profile_audio<C: AsRef<IsProfileAudio>>(
    &self,
    is_profile_audio: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, is_profile_audio.as_ref()).await
  }

  pub async fn join_chat<C: AsRef<JoinChat>>(&self, join_chat: C) -> RTDResult<ChatJoinResult> {
    async_caller!(ChatJoinResult);
    async_caller(&self.api, join_chat.as_ref()).await
  }

  pub async fn join_chat_by_invite_link<C: AsRef<JoinChatByInviteLink>>(
    &self,
    join_chat_by_invite_link: C,
  ) -> RTDResult<ChatJoinResult> {
    async_caller!(ChatJoinResult);
    async_caller(&self.api, join_chat_by_invite_link.as_ref()).await
  }

  pub async fn join_group_call<C: AsRef<JoinGroupCall>>(
    &self,
    join_group_call: C,
  ) -> RTDResult<GroupCallInfo> {
    async_caller!(GroupCallInfo);
    async_caller(&self.api, join_group_call.as_ref()).await
  }

  pub async fn join_live_story<C: AsRef<JoinLiveStory>>(
    &self,
    join_live_story: C,
  ) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, join_live_story.as_ref()).await
  }

  pub async fn join_video_chat<C: AsRef<JoinVideoChat>>(
    &self,
    join_video_chat: C,
  ) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, join_video_chat.as_ref()).await
  }

  pub async fn launch_prepaid_giveaway<C: AsRef<LaunchPrepaidGiveaway>>(
    &self,
    launch_prepaid_giveaway: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, launch_prepaid_giveaway.as_ref()).await
  }

  pub async fn leave_chat<C: AsRef<LeaveChat>>(&self, leave_chat: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, leave_chat.as_ref()).await
  }

  pub async fn leave_group_call<C: AsRef<LeaveGroupCall>>(
    &self,
    leave_group_call: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, leave_group_call.as_ref()).await
  }

  pub async fn listen_to_audio<C: AsRef<ListenToAudio>>(
    &self,
    listen_to_audio: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, listen_to_audio.as_ref()).await
  }

  pub async fn load_active_stories<C: AsRef<LoadActiveStories>>(
    &self,
    load_active_stories: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, load_active_stories.as_ref()).await
  }

  pub async fn load_chat_welcome_messages<C: AsRef<LoadChatWelcomeMessages>>(
    &self,
    load_chat_welcome_messages: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, load_chat_welcome_messages.as_ref()).await
  }

  pub async fn load_chats<C: AsRef<LoadChats>>(&self, load_chats: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, load_chats.as_ref()).await
  }

  pub async fn load_direct_messages_chat_topics<C: AsRef<LoadDirectMessagesChatTopics>>(
    &self,
    load_direct_messages_chat_topics: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, load_direct_messages_chat_topics.as_ref()).await
  }

  pub async fn load_group_call_participants<C: AsRef<LoadGroupCallParticipants>>(
    &self,
    load_group_call_participants: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, load_group_call_participants.as_ref()).await
  }

  pub async fn load_quick_reply_shortcut_messages<C: AsRef<LoadQuickReplyShortcutMessages>>(
    &self,
    load_quick_reply_shortcut_messages: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, load_quick_reply_shortcut_messages.as_ref()).await
  }

  pub async fn load_quick_reply_shortcuts<C: AsRef<LoadQuickReplyShortcuts>>(
    &self,
    load_quick_reply_shortcuts: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, load_quick_reply_shortcuts.as_ref()).await
  }

  pub async fn load_saved_messages_topics<C: AsRef<LoadSavedMessagesTopics>>(
    &self,
    load_saved_messages_topics: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, load_saved_messages_topics.as_ref()).await
  }

  pub async fn load_ton_wallet_gasless_transfers_info<
    C: AsRef<LoadTonWalletGaslessTransfersInfo>,
  >(
    &self,
    load_ton_wallet_gasless_transfers_info: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, load_ton_wallet_gasless_transfers_info.as_ref()).await
  }

  pub async fn load_ton_wallet_state<C: AsRef<LoadTonWalletState>>(
    &self,
    load_ton_wallet_state: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, load_ton_wallet_state.as_ref()).await
  }

  pub async fn log_out<C: AsRef<LogOut>>(&self, log_out: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, log_out.as_ref()).await
  }

  pub async fn mark_checklist_tasks_as_done<C: AsRef<MarkChecklistTasksAsDone>>(
    &self,
    mark_checklist_tasks_as_done: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, mark_checklist_tasks_as_done.as_ref()).await
  }

  pub async fn open_bot_similar_bot<C: AsRef<OpenBotSimilarBot>>(
    &self,
    open_bot_similar_bot: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, open_bot_similar_bot.as_ref()).await
  }

  pub async fn open_chat<C: AsRef<OpenChat>>(&self, open_chat: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, open_chat.as_ref()).await
  }

  pub async fn open_chat_similar_chat<C: AsRef<OpenChatSimilarChat>>(
    &self,
    open_chat_similar_chat: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, open_chat_similar_chat.as_ref()).await
  }

  pub async fn open_gift_auction<C: AsRef<OpenGiftAuction>>(
    &self,
    open_gift_auction: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, open_gift_auction.as_ref()).await
  }

  pub async fn open_message_content<C: AsRef<OpenMessageContent>>(
    &self,
    open_message_content: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, open_message_content.as_ref()).await
  }

  pub async fn open_sponsored_chat<C: AsRef<OpenSponsoredChat>>(
    &self,
    open_sponsored_chat: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, open_sponsored_chat.as_ref()).await
  }

  pub async fn open_story<C: AsRef<OpenStory>>(&self, open_story: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, open_story.as_ref()).await
  }

  pub async fn open_web_app<C: AsRef<OpenWebApp>>(&self, open_web_app: C) -> RTDResult<WebAppInfo> {
    async_caller!(WebAppInfo);
    async_caller(&self.api, open_web_app.as_ref()).await
  }

  pub async fn optimize_storage<C: AsRef<OptimizeStorage>>(
    &self,
    optimize_storage: C,
  ) -> RTDResult<StorageStatistics> {
    async_caller!(StorageStatistics);
    async_caller(&self.api, optimize_storage.as_ref()).await
  }

  pub async fn parse_markdown<C: AsRef<ParseMarkdown>>(
    &self,
    parse_markdown: C,
  ) -> RTDResult<FormattedText> {
    async_caller!(FormattedText);
    async_caller(&self.api, parse_markdown.as_ref()).await
  }

  pub async fn parse_text_entities<C: AsRef<ParseTextEntities>>(
    &self,
    parse_text_entities: C,
  ) -> RTDResult<FormattedText> {
    async_caller!(FormattedText);
    async_caller(&self.api, parse_text_entities.as_ref()).await
  }

  pub async fn pin_chat_message<C: AsRef<PinChatMessage>>(
    &self,
    pin_chat_message: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, pin_chat_message.as_ref()).await
  }

  pub async fn ping_proxy<C: AsRef<PingProxy>>(&self, ping_proxy: C) -> RTDResult<Seconds> {
    async_caller!(Seconds);
    async_caller(&self.api, ping_proxy.as_ref()).await
  }

  pub async fn place_gift_auction_bid<C: AsRef<PlaceGiftAuctionBid>>(
    &self,
    place_gift_auction_bid: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, place_gift_auction_bid.as_ref()).await
  }

  pub async fn post_story<C: AsRef<PostStory>>(&self, post_story: C) -> RTDResult<Story> {
    async_caller!(Story);
    async_caller(&self.api, post_story.as_ref()).await
  }

  pub async fn preliminary_upload_file<C: AsRef<PreliminaryUploadFile>>(
    &self,
    preliminary_upload_file: C,
  ) -> RTDResult<File> {
    async_caller!(File);
    async_caller(&self.api, preliminary_upload_file.as_ref()).await
  }

  pub async fn process_chat_folder_new_chats<C: AsRef<ProcessChatFolderNewChats>>(
    &self,
    process_chat_folder_new_chats: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, process_chat_folder_new_chats.as_ref()).await
  }

  pub async fn process_chat_has_protected_content_disable_request<
    C: AsRef<ProcessChatHasProtectedContentDisableRequest>,
  >(
    &self,
    process_chat_has_protected_content_disable_request: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(
      &self.api,
      process_chat_has_protected_content_disable_request.as_ref(),
    )
    .await
  }

  pub async fn process_chat_join_request<C: AsRef<ProcessChatJoinRequest>>(
    &self,
    process_chat_join_request: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, process_chat_join_request.as_ref()).await
  }

  pub async fn process_chat_join_requests<C: AsRef<ProcessChatJoinRequests>>(
    &self,
    process_chat_join_requests: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, process_chat_join_requests.as_ref()).await
  }

  pub async fn process_gift_purchase_offer<C: AsRef<ProcessGiftPurchaseOffer>>(
    &self,
    process_gift_purchase_offer: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, process_gift_purchase_offer.as_ref()).await
  }

  pub async fn process_push_notification<C: AsRef<ProcessPushNotification>>(
    &self,
    process_push_notification: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, process_push_notification.as_ref()).await
  }

  pub async fn process_web_proxy_carrier_message<C: AsRef<ProcessWebProxyCarrierMessage>>(
    &self,
    process_web_proxy_carrier_message: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, process_web_proxy_carrier_message.as_ref()).await
  }

  pub async fn rate_speech_recognition<C: AsRef<RateSpeechRecognition>>(
    &self,
    rate_speech_recognition: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, rate_speech_recognition.as_ref()).await
  }

  pub async fn read_all_chat_mentions<C: AsRef<ReadAllChatMentions>>(
    &self,
    read_all_chat_mentions: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, read_all_chat_mentions.as_ref()).await
  }

  pub async fn read_all_chat_poll_votes<C: AsRef<ReadAllChatPollVotes>>(
    &self,
    read_all_chat_poll_votes: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, read_all_chat_poll_votes.as_ref()).await
  }

  pub async fn read_all_chat_reactions<C: AsRef<ReadAllChatReactions>>(
    &self,
    read_all_chat_reactions: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, read_all_chat_reactions.as_ref()).await
  }

  pub async fn read_all_direct_messages_chat_topic_reactions<
    C: AsRef<ReadAllDirectMessagesChatTopicReactions>,
  >(
    &self,
    read_all_direct_messages_chat_topic_reactions: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(
      &self.api,
      read_all_direct_messages_chat_topic_reactions.as_ref(),
    )
    .await
  }

  pub async fn read_all_forum_topic_mentions<C: AsRef<ReadAllForumTopicMentions>>(
    &self,
    read_all_forum_topic_mentions: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, read_all_forum_topic_mentions.as_ref()).await
  }

  pub async fn read_all_forum_topic_poll_votes<C: AsRef<ReadAllForumTopicPollVotes>>(
    &self,
    read_all_forum_topic_poll_votes: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, read_all_forum_topic_poll_votes.as_ref()).await
  }

  pub async fn read_all_forum_topic_reactions<C: AsRef<ReadAllForumTopicReactions>>(
    &self,
    read_all_forum_topic_reactions: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, read_all_forum_topic_reactions.as_ref()).await
  }

  pub async fn read_business_message<C: AsRef<ReadBusinessMessage>>(
    &self,
    read_business_message: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, read_business_message.as_ref()).await
  }

  pub async fn read_chat_list<C: AsRef<ReadChatList>>(&self, read_chat_list: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, read_chat_list.as_ref()).await
  }

  pub async fn read_file_part<C: AsRef<ReadFilePart>>(&self, read_file_part: C) -> RTDResult<Data> {
    async_caller!(Data);
    async_caller(&self.api, read_file_part.as_ref()).await
  }

  pub async fn readd_quick_reply_shortcut_messages<C: AsRef<ReaddQuickReplyShortcutMessages>>(
    &self,
    readd_quick_reply_shortcut_messages: C,
  ) -> RTDResult<QuickReplyMessages> {
    async_caller!(QuickReplyMessages);
    async_caller(&self.api, readd_quick_reply_shortcut_messages.as_ref()).await
  }

  pub async fn recognize_speech<C: AsRef<RecognizeSpeech>>(
    &self,
    recognize_speech: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, recognize_speech.as_ref()).await
  }

  pub async fn recover_authentication_password<C: AsRef<RecoverAuthenticationPassword>>(
    &self,
    recover_authentication_password: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, recover_authentication_password.as_ref()).await
  }

  pub async fn recover_password<C: AsRef<RecoverPassword>>(
    &self,
    recover_password: C,
  ) -> RTDResult<PasswordState> {
    async_caller!(PasswordState);
    async_caller(&self.api, recover_password.as_ref()).await
  }

  pub async fn refund_star_payment<C: AsRef<RefundStarPayment>>(
    &self,
    refund_star_payment: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, refund_star_payment.as_ref()).await
  }

  pub async fn register_device<C: AsRef<RegisterDevice>>(
    &self,
    register_device: C,
  ) -> RTDResult<PushReceiverId> {
    async_caller!(PushReceiverId);
    async_caller(&self.api, register_device.as_ref()).await
  }

  pub async fn register_user<C: AsRef<RegisterUser>>(&self, register_user: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, register_user.as_ref()).await
  }

  pub async fn remove_all_files_from_downloads<C: AsRef<RemoveAllFilesFromDownloads>>(
    &self,
    remove_all_files_from_downloads: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_all_files_from_downloads.as_ref()).await
  }

  pub async fn remove_all_web_browser_settings_exceptions<
    C: AsRef<RemoveAllWebBrowserSettingsExceptions>,
  >(
    &self,
    remove_all_web_browser_settings_exceptions: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(
      &self.api,
      remove_all_web_browser_settings_exceptions.as_ref(),
    )
    .await
  }

  pub async fn remove_business_connected_bot_from_chat<
    C: AsRef<RemoveBusinessConnectedBotFromChat>,
  >(
    &self,
    remove_business_connected_bot_from_chat: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_business_connected_bot_from_chat.as_ref()).await
  }

  pub async fn remove_chat_action_bar<C: AsRef<RemoveChatActionBar>>(
    &self,
    remove_chat_action_bar: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_chat_action_bar.as_ref()).await
  }

  pub async fn remove_contacts<C: AsRef<RemoveContacts>>(
    &self,
    remove_contacts: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_contacts.as_ref()).await
  }

  pub async fn remove_favorite_sticker<C: AsRef<RemoveFavoriteSticker>>(
    &self,
    remove_favorite_sticker: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_favorite_sticker.as_ref()).await
  }

  pub async fn remove_file_from_downloads<C: AsRef<RemoveFileFromDownloads>>(
    &self,
    remove_file_from_downloads: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_file_from_downloads.as_ref()).await
  }

  pub async fn remove_gift_collection_gifts<C: AsRef<RemoveGiftCollectionGifts>>(
    &self,
    remove_gift_collection_gifts: C,
  ) -> RTDResult<GiftCollection> {
    async_caller!(GiftCollection);
    async_caller(&self.api, remove_gift_collection_gifts.as_ref()).await
  }

  pub async fn remove_installed_background<C: AsRef<RemoveInstalledBackground>>(
    &self,
    remove_installed_background: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_installed_background.as_ref()).await
  }

  pub async fn remove_login_passkey<C: AsRef<RemoveLoginPasskey>>(
    &self,
    remove_login_passkey: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_login_passkey.as_ref()).await
  }

  pub async fn remove_message_reaction<C: AsRef<RemoveMessageReaction>>(
    &self,
    remove_message_reaction: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_message_reaction.as_ref()).await
  }

  pub async fn remove_message_sender_bot_verification<
    C: AsRef<RemoveMessageSenderBotVerification>,
  >(
    &self,
    remove_message_sender_bot_verification: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_message_sender_bot_verification.as_ref()).await
  }

  pub async fn remove_notification<C: AsRef<RemoveNotification>>(
    &self,
    remove_notification: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_notification.as_ref()).await
  }

  pub async fn remove_notification_group<C: AsRef<RemoveNotificationGroup>>(
    &self,
    remove_notification_group: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_notification_group.as_ref()).await
  }

  pub async fn remove_pending_live_story_reactions<C: AsRef<RemovePendingLiveStoryReactions>>(
    &self,
    remove_pending_live_story_reactions: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_pending_live_story_reactions.as_ref()).await
  }

  pub async fn remove_pending_paid_message_reactions<
    C: AsRef<RemovePendingPaidMessageReactions>,
  >(
    &self,
    remove_pending_paid_message_reactions: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_pending_paid_message_reactions.as_ref()).await
  }

  pub async fn remove_profile_audio<C: AsRef<RemoveProfileAudio>>(
    &self,
    remove_profile_audio: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_profile_audio.as_ref()).await
  }

  pub async fn remove_proxy<C: AsRef<RemoveProxy>>(&self, remove_proxy: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_proxy.as_ref()).await
  }

  pub async fn remove_recent_hashtag<C: AsRef<RemoveRecentHashtag>>(
    &self,
    remove_recent_hashtag: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_recent_hashtag.as_ref()).await
  }

  pub async fn remove_recent_sticker<C: AsRef<RemoveRecentSticker>>(
    &self,
    remove_recent_sticker: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_recent_sticker.as_ref()).await
  }

  pub async fn remove_recently_found_chat<C: AsRef<RemoveRecentlyFoundChat>>(
    &self,
    remove_recently_found_chat: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_recently_found_chat.as_ref()).await
  }

  pub async fn remove_saved_animation<C: AsRef<RemoveSavedAnimation>>(
    &self,
    remove_saved_animation: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_saved_animation.as_ref()).await
  }

  pub async fn remove_saved_notification_sound<C: AsRef<RemoveSavedNotificationSound>>(
    &self,
    remove_saved_notification_sound: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_saved_notification_sound.as_ref()).await
  }

  pub async fn remove_searched_for_tag<C: AsRef<RemoveSearchedForTag>>(
    &self,
    remove_searched_for_tag: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_searched_for_tag.as_ref()).await
  }

  pub async fn remove_sticker_from_set<C: AsRef<RemoveStickerFromSet>>(
    &self,
    remove_sticker_from_set: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_sticker_from_set.as_ref()).await
  }

  pub async fn remove_story_album_stories<C: AsRef<RemoveStoryAlbumStories>>(
    &self,
    remove_story_album_stories: C,
  ) -> RTDResult<StoryAlbum> {
    async_caller!(StoryAlbum);
    async_caller(&self.api, remove_story_album_stories.as_ref()).await
  }

  pub async fn remove_text_composition_style<C: AsRef<RemoveTextCompositionStyle>>(
    &self,
    remove_text_composition_style: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_text_composition_style.as_ref()).await
  }

  pub async fn remove_top_chat<C: AsRef<RemoveTopChat>>(
    &self,
    remove_top_chat: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_top_chat.as_ref()).await
  }

  pub async fn remove_web_browser_settings_exception<
    C: AsRef<RemoveWebBrowserSettingsException>,
  >(
    &self,
    remove_web_browser_settings_exception: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, remove_web_browser_settings_exception.as_ref()).await
  }

  pub async fn reorder_active_usernames<C: AsRef<ReorderActiveUsernames>>(
    &self,
    reorder_active_usernames: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, reorder_active_usernames.as_ref()).await
  }

  pub async fn reorder_bot_active_usernames<C: AsRef<ReorderBotActiveUsernames>>(
    &self,
    reorder_bot_active_usernames: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, reorder_bot_active_usernames.as_ref()).await
  }

  pub async fn reorder_bot_media_previews<C: AsRef<ReorderBotMediaPreviews>>(
    &self,
    reorder_bot_media_previews: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, reorder_bot_media_previews.as_ref()).await
  }

  pub async fn reorder_chat_folders<C: AsRef<ReorderChatFolders>>(
    &self,
    reorder_chat_folders: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, reorder_chat_folders.as_ref()).await
  }

  pub async fn reorder_gift_collection_gifts<C: AsRef<ReorderGiftCollectionGifts>>(
    &self,
    reorder_gift_collection_gifts: C,
  ) -> RTDResult<GiftCollection> {
    async_caller!(GiftCollection);
    async_caller(&self.api, reorder_gift_collection_gifts.as_ref()).await
  }

  pub async fn reorder_gift_collections<C: AsRef<ReorderGiftCollections>>(
    &self,
    reorder_gift_collections: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, reorder_gift_collections.as_ref()).await
  }

  pub async fn reorder_installed_sticker_sets<C: AsRef<ReorderInstalledStickerSets>>(
    &self,
    reorder_installed_sticker_sets: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, reorder_installed_sticker_sets.as_ref()).await
  }

  pub async fn reorder_quick_reply_shortcuts<C: AsRef<ReorderQuickReplyShortcuts>>(
    &self,
    reorder_quick_reply_shortcuts: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, reorder_quick_reply_shortcuts.as_ref()).await
  }

  pub async fn reorder_story_album_stories<C: AsRef<ReorderStoryAlbumStories>>(
    &self,
    reorder_story_album_stories: C,
  ) -> RTDResult<StoryAlbum> {
    async_caller!(StoryAlbum);
    async_caller(&self.api, reorder_story_album_stories.as_ref()).await
  }

  pub async fn reorder_story_albums<C: AsRef<ReorderStoryAlbums>>(
    &self,
    reorder_story_albums: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, reorder_story_albums.as_ref()).await
  }

  pub async fn reorder_supergroup_active_usernames<C: AsRef<ReorderSupergroupActiveUsernames>>(
    &self,
    reorder_supergroup_active_usernames: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, reorder_supergroup_active_usernames.as_ref()).await
  }

  pub async fn replace_live_story_rtmp_url<C: AsRef<ReplaceLiveStoryRtmpUrl>>(
    &self,
    replace_live_story_rtmp_url: C,
  ) -> RTDResult<RtmpUrl> {
    async_caller!(RtmpUrl);
    async_caller(&self.api, replace_live_story_rtmp_url.as_ref()).await
  }

  pub async fn replace_primary_chat_invite_link<C: AsRef<ReplacePrimaryChatInviteLink>>(
    &self,
    replace_primary_chat_invite_link: C,
  ) -> RTDResult<ChatInviteLink> {
    async_caller!(ChatInviteLink);
    async_caller(&self.api, replace_primary_chat_invite_link.as_ref()).await
  }

  pub async fn replace_sticker_in_set<C: AsRef<ReplaceStickerInSet>>(
    &self,
    replace_sticker_in_set: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, replace_sticker_in_set.as_ref()).await
  }

  pub async fn replace_ton_wallet<C: AsRef<ReplaceTonWallet>>(
    &self,
    replace_ton_wallet: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, replace_ton_wallet.as_ref()).await
  }

  pub async fn replace_video_chat_rtmp_url<C: AsRef<ReplaceVideoChatRtmpUrl>>(
    &self,
    replace_video_chat_rtmp_url: C,
  ) -> RTDResult<RtmpUrl> {
    async_caller!(RtmpUrl);
    async_caller(&self.api, replace_video_chat_rtmp_url.as_ref()).await
  }

  pub async fn reply_to_callback_query_with_ephemeral_message<
    C: AsRef<ReplyToCallbackQueryWithEphemeralMessage>,
  >(
    &self,
    reply_to_callback_query_with_ephemeral_message: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(
      &self.api,
      reply_to_callback_query_with_ephemeral_message.as_ref(),
    )
    .await
  }

  pub async fn report_authentication_code_missing<C: AsRef<ReportAuthenticationCodeMissing>>(
    &self,
    report_authentication_code_missing: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, report_authentication_code_missing.as_ref()).await
  }

  pub async fn report_chat<C: AsRef<ReportChat>>(
    &self,
    report_chat: C,
  ) -> RTDResult<ReportChatResult> {
    async_caller!(ReportChatResult);
    async_caller(&self.api, report_chat.as_ref()).await
  }

  pub async fn report_chat_photo<C: AsRef<ReportChatPhoto>>(
    &self,
    report_chat_photo: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, report_chat_photo.as_ref()).await
  }

  pub async fn report_chat_sponsored_message<C: AsRef<ReportChatSponsoredMessage>>(
    &self,
    report_chat_sponsored_message: C,
  ) -> RTDResult<ReportSponsoredResult> {
    async_caller!(ReportSponsoredResult);
    async_caller(&self.api, report_chat_sponsored_message.as_ref()).await
  }

  pub async fn report_message_reactions<C: AsRef<ReportMessageReactions>>(
    &self,
    report_message_reactions: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, report_message_reactions.as_ref()).await
  }

  pub async fn report_phone_number_code_missing<C: AsRef<ReportPhoneNumberCodeMissing>>(
    &self,
    report_phone_number_code_missing: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, report_phone_number_code_missing.as_ref()).await
  }

  pub async fn report_sponsored_chat<C: AsRef<ReportSponsoredChat>>(
    &self,
    report_sponsored_chat: C,
  ) -> RTDResult<ReportSponsoredResult> {
    async_caller!(ReportSponsoredResult);
    async_caller(&self.api, report_sponsored_chat.as_ref()).await
  }

  pub async fn report_story<C: AsRef<ReportStory>>(
    &self,
    report_story: C,
  ) -> RTDResult<ReportStoryResult> {
    async_caller!(ReportStoryResult);
    async_caller(&self.api, report_story.as_ref()).await
  }

  pub async fn report_supergroup_anti_spam_false_positive<
    C: AsRef<ReportSupergroupAntiSpamFalsePositive>,
  >(
    &self,
    report_supergroup_anti_spam_false_positive: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(
      &self.api,
      report_supergroup_anti_spam_false_positive.as_ref(),
    )
    .await
  }

  pub async fn report_supergroup_spam<C: AsRef<ReportSupergroupSpam>>(
    &self,
    report_supergroup_spam: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, report_supergroup_spam.as_ref()).await
  }

  pub async fn report_video_message_advertisement<C: AsRef<ReportVideoMessageAdvertisement>>(
    &self,
    report_video_message_advertisement: C,
  ) -> RTDResult<ReportSponsoredResult> {
    async_caller!(ReportSponsoredResult);
    async_caller(&self.api, report_video_message_advertisement.as_ref()).await
  }

  pub async fn report_web_proxy_carrier_error<C: AsRef<ReportWebProxyCarrierError>>(
    &self,
    report_web_proxy_carrier_error: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, report_web_proxy_carrier_error.as_ref()).await
  }

  pub async fn request_authentication_password_recovery<
    C: AsRef<RequestAuthenticationPasswordRecovery>,
  >(
    &self,
    request_authentication_password_recovery: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, request_authentication_password_recovery.as_ref()).await
  }

  pub async fn request_password_recovery<C: AsRef<RequestPasswordRecovery>>(
    &self,
    request_password_recovery: C,
  ) -> RTDResult<EmailAddressAuthenticationCodeInfo> {
    async_caller!(EmailAddressAuthenticationCodeInfo);
    async_caller(&self.api, request_password_recovery.as_ref()).await
  }

  pub async fn request_qr_code_authentication<C: AsRef<RequestQrCodeAuthentication>>(
    &self,
    request_qr_code_authentication: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, request_qr_code_authentication.as_ref()).await
  }

  pub async fn resend_authentication_code<C: AsRef<ResendAuthenticationCode>>(
    &self,
    resend_authentication_code: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, resend_authentication_code.as_ref()).await
  }

  pub async fn resend_email_address_verification_code<
    C: AsRef<ResendEmailAddressVerificationCode>,
  >(
    &self,
    resend_email_address_verification_code: C,
  ) -> RTDResult<EmailAddressAuthenticationCodeInfo> {
    async_caller!(EmailAddressAuthenticationCodeInfo);
    async_caller(&self.api, resend_email_address_verification_code.as_ref()).await
  }

  pub async fn resend_login_email_address_code<C: AsRef<ResendLoginEmailAddressCode>>(
    &self,
    resend_login_email_address_code: C,
  ) -> RTDResult<EmailAddressAuthenticationCodeInfo> {
    async_caller!(EmailAddressAuthenticationCodeInfo);
    async_caller(&self.api, resend_login_email_address_code.as_ref()).await
  }

  pub async fn resend_messages<C: AsRef<ResendMessages>>(
    &self,
    resend_messages: C,
  ) -> RTDResult<Messages> {
    async_caller!(Messages);
    async_caller(&self.api, resend_messages.as_ref()).await
  }

  pub async fn resend_phone_number_code<C: AsRef<ResendPhoneNumberCode>>(
    &self,
    resend_phone_number_code: C,
  ) -> RTDResult<AuthenticationCodeInfo> {
    async_caller!(AuthenticationCodeInfo);
    async_caller(&self.api, resend_phone_number_code.as_ref()).await
  }

  pub async fn resend_recovery_email_address_code<C: AsRef<ResendRecoveryEmailAddressCode>>(
    &self,
    resend_recovery_email_address_code: C,
  ) -> RTDResult<PasswordState> {
    async_caller!(PasswordState);
    async_caller(&self.api, resend_recovery_email_address_code.as_ref()).await
  }

  pub async fn reset_all_notification_settings<C: AsRef<ResetAllNotificationSettings>>(
    &self,
    reset_all_notification_settings: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, reset_all_notification_settings.as_ref()).await
  }

  pub async fn reset_authentication_email_address<C: AsRef<ResetAuthenticationEmailAddress>>(
    &self,
    reset_authentication_email_address: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, reset_authentication_email_address.as_ref()).await
  }

  pub async fn reset_installed_backgrounds<C: AsRef<ResetInstalledBackgrounds>>(
    &self,
    reset_installed_backgrounds: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, reset_installed_backgrounds.as_ref()).await
  }

  pub async fn reset_network_statistics<C: AsRef<ResetNetworkStatistics>>(
    &self,
    reset_network_statistics: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, reset_network_statistics.as_ref()).await
  }

  pub async fn reset_password<C: AsRef<ResetPassword>>(
    &self,
    reset_password: C,
  ) -> RTDResult<ResetPasswordResult> {
    async_caller!(ResetPasswordResult);
    async_caller(&self.api, reset_password.as_ref()).await
  }

  pub async fn reuse_star_subscription<C: AsRef<ReuseStarSubscription>>(
    &self,
    reuse_star_subscription: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, reuse_star_subscription.as_ref()).await
  }

  pub async fn revoke_chat_invite_link<C: AsRef<RevokeChatInviteLink>>(
    &self,
    revoke_chat_invite_link: C,
  ) -> RTDResult<ChatInviteLinks> {
    async_caller!(ChatInviteLinks);
    async_caller(&self.api, revoke_chat_invite_link.as_ref()).await
  }

  pub async fn revoke_group_call_invite_link<C: AsRef<RevokeGroupCallInviteLink>>(
    &self,
    revoke_group_call_invite_link: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, revoke_group_call_invite_link.as_ref()).await
  }

  pub async fn save_application_log_event<C: AsRef<SaveApplicationLogEvent>>(
    &self,
    save_application_log_event: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, save_application_log_event.as_ref()).await
  }

  pub async fn save_prepared_inline_message<C: AsRef<SavePreparedInlineMessage>>(
    &self,
    save_prepared_inline_message: C,
  ) -> RTDResult<PreparedInlineMessageId> {
    async_caller!(PreparedInlineMessageId);
    async_caller(&self.api, save_prepared_inline_message.as_ref()).await
  }

  pub async fn save_prepared_keyboard_button<C: AsRef<SavePreparedKeyboardButton>>(
    &self,
    save_prepared_keyboard_button: C,
  ) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, save_prepared_keyboard_button.as_ref()).await
  }

  pub async fn search_affiliate_programs<C: AsRef<SearchAffiliatePrograms>>(
    &self,
    search_affiliate_programs: C,
  ) -> RTDResult<FoundAffiliatePrograms> {
    async_caller!(FoundAffiliatePrograms);
    async_caller(&self.api, search_affiliate_programs.as_ref()).await
  }

  pub async fn search_background<C: AsRef<SearchBackground>>(
    &self,
    search_background: C,
  ) -> RTDResult<Background> {
    async_caller!(Background);
    async_caller(&self.api, search_background.as_ref()).await
  }

  pub async fn search_call_messages<C: AsRef<SearchCallMessages>>(
    &self,
    search_call_messages: C,
  ) -> RTDResult<FoundMessages> {
    async_caller!(FoundMessages);
    async_caller(&self.api, search_call_messages.as_ref()).await
  }

  pub async fn search_chat_affiliate_program<C: AsRef<SearchChatAffiliateProgram>>(
    &self,
    search_chat_affiliate_program: C,
  ) -> RTDResult<Chat> {
    async_caller!(Chat);
    async_caller(&self.api, search_chat_affiliate_program.as_ref()).await
  }

  pub async fn search_chat_members<C: AsRef<SearchChatMembers>>(
    &self,
    search_chat_members: C,
  ) -> RTDResult<ChatMembers> {
    async_caller!(ChatMembers);
    async_caller(&self.api, search_chat_members.as_ref()).await
  }

  pub async fn search_chat_messages<C: AsRef<SearchChatMessages>>(
    &self,
    search_chat_messages: C,
  ) -> RTDResult<FoundChatMessages> {
    async_caller!(FoundChatMessages);
    async_caller(&self.api, search_chat_messages.as_ref()).await
  }

  pub async fn search_chat_recent_location_messages<C: AsRef<SearchChatRecentLocationMessages>>(
    &self,
    search_chat_recent_location_messages: C,
  ) -> RTDResult<Messages> {
    async_caller!(Messages);
    async_caller(&self.api, search_chat_recent_location_messages.as_ref()).await
  }

  pub async fn search_chats<C: AsRef<SearchChats>>(&self, search_chats: C) -> RTDResult<Chats> {
    async_caller!(Chats);
    async_caller(&self.api, search_chats.as_ref()).await
  }

  pub async fn search_chats_on_server<C: AsRef<SearchChatsOnServer>>(
    &self,
    search_chats_on_server: C,
  ) -> RTDResult<Chats> {
    async_caller!(Chats);
    async_caller(&self.api, search_chats_on_server.as_ref()).await
  }

  pub async fn search_contacts<C: AsRef<SearchContacts>>(
    &self,
    search_contacts: C,
  ) -> RTDResult<Users> {
    async_caller!(Users);
    async_caller(&self.api, search_contacts.as_ref()).await
  }

  pub async fn search_emojis<C: AsRef<SearchEmojis>>(
    &self,
    search_emojis: C,
  ) -> RTDResult<EmojiKeywords> {
    async_caller!(EmojiKeywords);
    async_caller(&self.api, search_emojis.as_ref()).await
  }

  pub async fn search_file_downloads<C: AsRef<SearchFileDownloads>>(
    &self,
    search_file_downloads: C,
  ) -> RTDResult<FoundFileDownloads> {
    async_caller!(FoundFileDownloads);
    async_caller(&self.api, search_file_downloads.as_ref()).await
  }

  pub async fn search_gifts_for_resale<C: AsRef<SearchGiftsForResale>>(
    &self,
    search_gifts_for_resale: C,
  ) -> RTDResult<GiftsForResale> {
    async_caller!(GiftsForResale);
    async_caller(&self.api, search_gifts_for_resale.as_ref()).await
  }

  pub async fn search_hashtags<C: AsRef<SearchHashtags>>(
    &self,
    search_hashtags: C,
  ) -> RTDResult<Hashtags> {
    async_caller!(Hashtags);
    async_caller(&self.api, search_hashtags.as_ref()).await
  }

  pub async fn search_installed_sticker_sets<C: AsRef<SearchInstalledStickerSets>>(
    &self,
    search_installed_sticker_sets: C,
  ) -> RTDResult<StickerSets> {
    async_caller!(StickerSets);
    async_caller(&self.api, search_installed_sticker_sets.as_ref()).await
  }

  pub async fn search_messages<C: AsRef<SearchMessages>>(
    &self,
    search_messages: C,
  ) -> RTDResult<FoundMessages> {
    async_caller!(FoundMessages);
    async_caller(&self.api, search_messages.as_ref()).await
  }

  pub async fn search_outgoing_document_messages<C: AsRef<SearchOutgoingDocumentMessages>>(
    &self,
    search_outgoing_document_messages: C,
  ) -> RTDResult<FoundMessages> {
    async_caller!(FoundMessages);
    async_caller(&self.api, search_outgoing_document_messages.as_ref()).await
  }

  pub async fn search_public_chat<C: AsRef<SearchPublicChat>>(
    &self,
    search_public_chat: C,
  ) -> RTDResult<Chat> {
    async_caller!(Chat);
    async_caller(&self.api, search_public_chat.as_ref()).await
  }

  pub async fn search_public_chats<C: AsRef<SearchPublicChats>>(
    &self,
    search_public_chats: C,
  ) -> RTDResult<Chats> {
    async_caller!(Chats);
    async_caller(&self.api, search_public_chats.as_ref()).await
  }

  pub async fn search_public_messages_by_tag<C: AsRef<SearchPublicMessagesByTag>>(
    &self,
    search_public_messages_by_tag: C,
  ) -> RTDResult<FoundMessages> {
    async_caller!(FoundMessages);
    async_caller(&self.api, search_public_messages_by_tag.as_ref()).await
  }

  pub async fn search_public_posts<C: AsRef<SearchPublicPosts>>(
    &self,
    search_public_posts: C,
  ) -> RTDResult<FoundPublicPosts> {
    async_caller!(FoundPublicPosts);
    async_caller(&self.api, search_public_posts.as_ref()).await
  }

  pub async fn search_public_stories_by_location<C: AsRef<SearchPublicStoriesByLocation>>(
    &self,
    search_public_stories_by_location: C,
  ) -> RTDResult<FoundStories> {
    async_caller!(FoundStories);
    async_caller(&self.api, search_public_stories_by_location.as_ref()).await
  }

  pub async fn search_public_stories_by_tag<C: AsRef<SearchPublicStoriesByTag>>(
    &self,
    search_public_stories_by_tag: C,
  ) -> RTDResult<FoundStories> {
    async_caller!(FoundStories);
    async_caller(&self.api, search_public_stories_by_tag.as_ref()).await
  }

  pub async fn search_public_stories_by_venue<C: AsRef<SearchPublicStoriesByVenue>>(
    &self,
    search_public_stories_by_venue: C,
  ) -> RTDResult<FoundStories> {
    async_caller!(FoundStories);
    async_caller(&self.api, search_public_stories_by_venue.as_ref()).await
  }

  pub async fn search_quote<C: AsRef<SearchQuote>>(
    &self,
    search_quote: C,
  ) -> RTDResult<FoundPosition> {
    async_caller!(FoundPosition);
    async_caller(&self.api, search_quote.as_ref()).await
  }

  pub async fn search_recently_found_chats<C: AsRef<SearchRecentlyFoundChats>>(
    &self,
    search_recently_found_chats: C,
  ) -> RTDResult<Chats> {
    async_caller!(Chats);
    async_caller(&self.api, search_recently_found_chats.as_ref()).await
  }

  pub async fn search_saved_messages<C: AsRef<SearchSavedMessages>>(
    &self,
    search_saved_messages: C,
  ) -> RTDResult<FoundChatMessages> {
    async_caller!(FoundChatMessages);
    async_caller(&self.api, search_saved_messages.as_ref()).await
  }

  pub async fn search_secret_messages<C: AsRef<SearchSecretMessages>>(
    &self,
    search_secret_messages: C,
  ) -> RTDResult<FoundMessages> {
    async_caller!(FoundMessages);
    async_caller(&self.api, search_secret_messages.as_ref()).await
  }

  pub async fn search_sticker_set<C: AsRef<SearchStickerSet>>(
    &self,
    search_sticker_set: C,
  ) -> RTDResult<StickerSet> {
    async_caller!(StickerSet);
    async_caller(&self.api, search_sticker_set.as_ref()).await
  }

  pub async fn search_sticker_sets<C: AsRef<SearchStickerSets>>(
    &self,
    search_sticker_sets: C,
  ) -> RTDResult<StickerSets> {
    async_caller!(StickerSets);
    async_caller(&self.api, search_sticker_sets.as_ref()).await
  }

  pub async fn search_stickers<C: AsRef<SearchStickers>>(
    &self,
    search_stickers: C,
  ) -> RTDResult<Stickers> {
    async_caller!(Stickers);
    async_caller(&self.api, search_stickers.as_ref()).await
  }

  pub async fn search_strings_by_prefix<C: AsRef<SearchStringsByPrefix>>(
    &self,
    search_strings_by_prefix: C,
  ) -> RTDResult<FoundPositions> {
    async_caller!(FoundPositions);
    async_caller(&self.api, search_strings_by_prefix.as_ref()).await
  }

  pub async fn search_text_composition_style<C: AsRef<SearchTextCompositionStyle>>(
    &self,
    search_text_composition_style: C,
  ) -> RTDResult<TextCompositionStyle> {
    async_caller!(TextCompositionStyle);
    async_caller(&self.api, search_text_composition_style.as_ref()).await
  }

  pub async fn search_user_by_phone_number<C: AsRef<SearchUserByPhoneNumber>>(
    &self,
    search_user_by_phone_number: C,
  ) -> RTDResult<User> {
    async_caller!(User);
    async_caller(&self.api, search_user_by_phone_number.as_ref()).await
  }

  pub async fn search_user_by_token<C: AsRef<SearchUserByToken>>(
    &self,
    search_user_by_token: C,
  ) -> RTDResult<User> {
    async_caller!(User);
    async_caller(&self.api, search_user_by_token.as_ref()).await
  }

  pub async fn search_web_app<C: AsRef<SearchWebApp>>(
    &self,
    search_web_app: C,
  ) -> RTDResult<FoundWebApp> {
    async_caller!(FoundWebApp);
    async_caller(&self.api, search_web_app.as_ref()).await
  }

  pub async fn sell_gift<C: AsRef<SellGift>>(&self, sell_gift: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, sell_gift.as_ref()).await
  }

  pub async fn send_authentication_firebase_sms<C: AsRef<SendAuthenticationFirebaseSms>>(
    &self,
    send_authentication_firebase_sms: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, send_authentication_firebase_sms.as_ref()).await
  }

  pub async fn send_bot_start_message<C: AsRef<SendBotStartMessage>>(
    &self,
    send_bot_start_message: C,
  ) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, send_bot_start_message.as_ref()).await
  }

  pub async fn send_business_message<C: AsRef<SendBusinessMessage>>(
    &self,
    send_business_message: C,
  ) -> RTDResult<BusinessMessage> {
    async_caller!(BusinessMessage);
    async_caller(&self.api, send_business_message.as_ref()).await
  }

  pub async fn send_business_message_album<C: AsRef<SendBusinessMessageAlbum>>(
    &self,
    send_business_message_album: C,
  ) -> RTDResult<BusinessMessages> {
    async_caller!(BusinessMessages);
    async_caller(&self.api, send_business_message_album.as_ref()).await
  }

  pub async fn send_call_debug_information<C: AsRef<SendCallDebugInformation>>(
    &self,
    send_call_debug_information: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, send_call_debug_information.as_ref()).await
  }

  pub async fn send_call_log<C: AsRef<SendCallLog>>(&self, send_call_log: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, send_call_log.as_ref()).await
  }

  pub async fn send_call_rating<C: AsRef<SendCallRating>>(
    &self,
    send_call_rating: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, send_call_rating.as_ref()).await
  }

  pub async fn send_call_signaling_data<C: AsRef<SendCallSignalingData>>(
    &self,
    send_call_signaling_data: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, send_call_signaling_data.as_ref()).await
  }

  pub async fn send_chat_action<C: AsRef<SendChatAction>>(
    &self,
    send_chat_action: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, send_chat_action.as_ref()).await
  }

  pub async fn send_custom_request<C: AsRef<SendCustomRequest>>(
    &self,
    send_custom_request: C,
  ) -> RTDResult<CustomRequestResult> {
    async_caller!(CustomRequestResult);
    async_caller(&self.api, send_custom_request.as_ref()).await
  }

  pub async fn send_email_address_verification_code<C: AsRef<SendEmailAddressVerificationCode>>(
    &self,
    send_email_address_verification_code: C,
  ) -> RTDResult<EmailAddressAuthenticationCodeInfo> {
    async_caller!(EmailAddressAuthenticationCodeInfo);
    async_caller(&self.api, send_email_address_verification_code.as_ref()).await
  }

  pub async fn send_ephemeral_message<C: AsRef<SendEphemeralMessage>>(
    &self,
    send_ephemeral_message: C,
  ) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, send_ephemeral_message.as_ref()).await
  }

  pub async fn send_gift<C: AsRef<SendGift>>(&self, send_gift: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, send_gift.as_ref()).await
  }

  pub async fn send_gift_purchase_offer<C: AsRef<SendGiftPurchaseOffer>>(
    &self,
    send_gift_purchase_offer: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, send_gift_purchase_offer.as_ref()).await
  }

  pub async fn send_group_call_message<C: AsRef<SendGroupCallMessage>>(
    &self,
    send_group_call_message: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, send_group_call_message.as_ref()).await
  }

  pub async fn send_inline_query_result_message<C: AsRef<SendInlineQueryResultMessage>>(
    &self,
    send_inline_query_result_message: C,
  ) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, send_inline_query_result_message.as_ref()).await
  }

  pub async fn send_message<C: AsRef<SendMessage>>(&self, send_message: C) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, send_message.as_ref()).await
  }

  pub async fn send_message_album<C: AsRef<SendMessageAlbum>>(
    &self,
    send_message_album: C,
  ) -> RTDResult<Messages> {
    async_caller!(Messages);
    async_caller(&self.api, send_message_album.as_ref()).await
  }

  pub async fn send_message_view_metrics<C: AsRef<SendMessageViewMetrics>>(
    &self,
    send_message_view_metrics: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, send_message_view_metrics.as_ref()).await
  }

  pub async fn send_passport_authorization_form<C: AsRef<SendPassportAuthorizationForm>>(
    &self,
    send_passport_authorization_form: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, send_passport_authorization_form.as_ref()).await
  }

  pub async fn send_payment_form<C: AsRef<SendPaymentForm>>(
    &self,
    send_payment_form: C,
  ) -> RTDResult<PaymentResult> {
    async_caller!(PaymentResult);
    async_caller(&self.api, send_payment_form.as_ref()).await
  }

  pub async fn send_phone_number_code<C: AsRef<SendPhoneNumberCode>>(
    &self,
    send_phone_number_code: C,
  ) -> RTDResult<AuthenticationCodeInfo> {
    async_caller!(AuthenticationCodeInfo);
    async_caller(&self.api, send_phone_number_code.as_ref()).await
  }

  pub async fn send_phone_number_firebase_sms<C: AsRef<SendPhoneNumberFirebaseSms>>(
    &self,
    send_phone_number_firebase_sms: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, send_phone_number_firebase_sms.as_ref()).await
  }

  pub async fn send_quick_reply_shortcut_messages<C: AsRef<SendQuickReplyShortcutMessages>>(
    &self,
    send_quick_reply_shortcut_messages: C,
  ) -> RTDResult<Messages> {
    async_caller!(Messages);
    async_caller(&self.api, send_quick_reply_shortcut_messages.as_ref()).await
  }

  pub async fn send_resold_gift<C: AsRef<SendResoldGift>>(
    &self,
    send_resold_gift: C,
  ) -> RTDResult<GiftResaleResult> {
    async_caller!(GiftResaleResult);
    async_caller(&self.api, send_resold_gift.as_ref()).await
  }

  pub async fn send_rich_message_draft<C: AsRef<SendRichMessageDraft>>(
    &self,
    send_rich_message_draft: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, send_rich_message_draft.as_ref()).await
  }

  pub async fn send_text_message_draft<C: AsRef<SendTextMessageDraft>>(
    &self,
    send_text_message_draft: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, send_text_message_draft.as_ref()).await
  }

  pub async fn send_ton_center_api_request<C: AsRef<SendTonCenterApiRequest>>(
    &self,
    send_ton_center_api_request: C,
  ) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, send_ton_center_api_request.as_ref()).await
  }

  pub async fn send_ton_connect_session_connect_result<
    C: AsRef<SendTonConnectSessionConnectResult>,
  >(
    &self,
    send_ton_connect_session_connect_result: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, send_ton_connect_session_connect_result.as_ref()).await
  }

  pub async fn send_ton_wallet_transfer<C: AsRef<SendTonWalletTransfer>>(
    &self,
    send_ton_wallet_transfer: C,
  ) -> RTDResult<TonWalletTransferResult> {
    async_caller!(TonWalletTransferResult);
    async_caller(&self.api, send_ton_wallet_transfer.as_ref()).await
  }

  pub async fn send_web_app_custom_request<C: AsRef<SendWebAppCustomRequest>>(
    &self,
    send_web_app_custom_request: C,
  ) -> RTDResult<CustomRequestResult> {
    async_caller!(CustomRequestResult);
    async_caller(&self.api, send_web_app_custom_request.as_ref()).await
  }

  pub async fn send_web_app_data<C: AsRef<SendWebAppData>>(
    &self,
    send_web_app_data: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, send_web_app_data.as_ref()).await
  }

  pub async fn set_accent_color<C: AsRef<SetAccentColor>>(
    &self,
    set_accent_color: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_accent_color.as_ref()).await
  }

  pub async fn set_account_ttl<C: AsRef<SetAccountTtl>>(
    &self,
    set_account_ttl: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_account_ttl.as_ref()).await
  }

  pub async fn set_alarm<C: AsRef<SetAlarm>>(&self, set_alarm: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_alarm.as_ref()).await
  }

  pub async fn set_application_verification_token<C: AsRef<SetApplicationVerificationToken>>(
    &self,
    set_application_verification_token: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_application_verification_token.as_ref()).await
  }

  pub async fn set_archive_chat_list_settings<C: AsRef<SetArchiveChatListSettings>>(
    &self,
    set_archive_chat_list_settings: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_archive_chat_list_settings.as_ref()).await
  }

  pub async fn set_authentication_email_address<C: AsRef<SetAuthenticationEmailAddress>>(
    &self,
    set_authentication_email_address: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_authentication_email_address.as_ref()).await
  }

  pub async fn set_authentication_phone_number<C: AsRef<SetAuthenticationPhoneNumber>>(
    &self,
    set_authentication_phone_number: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_authentication_phone_number.as_ref()).await
  }

  pub async fn set_authentication_premium_purchase_transaction<
    C: AsRef<SetAuthenticationPremiumPurchaseTransaction>,
  >(
    &self,
    set_authentication_premium_purchase_transaction: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(
      &self.api,
      set_authentication_premium_purchase_transaction.as_ref(),
    )
    .await
  }

  pub async fn set_auto_download_settings<C: AsRef<SetAutoDownloadSettings>>(
    &self,
    set_auto_download_settings: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_auto_download_settings.as_ref()).await
  }

  pub async fn set_autosave_settings<C: AsRef<SetAutosaveSettings>>(
    &self,
    set_autosave_settings: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_autosave_settings.as_ref()).await
  }

  pub async fn set_bio<C: AsRef<SetBio>>(&self, set_bio: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_bio.as_ref()).await
  }

  pub async fn set_birthdate<C: AsRef<SetBirthdate>>(&self, set_birthdate: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_birthdate.as_ref()).await
  }

  pub async fn set_bot_info_description<C: AsRef<SetBotInfoDescription>>(
    &self,
    set_bot_info_description: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_bot_info_description.as_ref()).await
  }

  pub async fn set_bot_info_short_description<C: AsRef<SetBotInfoShortDescription>>(
    &self,
    set_bot_info_short_description: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_bot_info_short_description.as_ref()).await
  }

  pub async fn set_bot_name<C: AsRef<SetBotName>>(&self, set_bot_name: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_bot_name.as_ref()).await
  }

  pub async fn set_bot_profile_photo<C: AsRef<SetBotProfilePhoto>>(
    &self,
    set_bot_profile_photo: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_bot_profile_photo.as_ref()).await
  }

  pub async fn set_bot_updates_status<C: AsRef<SetBotUpdatesStatus>>(
    &self,
    set_bot_updates_status: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_bot_updates_status.as_ref()).await
  }

  pub async fn set_business_account_bio<C: AsRef<SetBusinessAccountBio>>(
    &self,
    set_business_account_bio: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_business_account_bio.as_ref()).await
  }

  pub async fn set_business_account_gift_settings<C: AsRef<SetBusinessAccountGiftSettings>>(
    &self,
    set_business_account_gift_settings: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_business_account_gift_settings.as_ref()).await
  }

  pub async fn set_business_account_name<C: AsRef<SetBusinessAccountName>>(
    &self,
    set_business_account_name: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_business_account_name.as_ref()).await
  }

  pub async fn set_business_account_profile_photo<C: AsRef<SetBusinessAccountProfilePhoto>>(
    &self,
    set_business_account_profile_photo: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_business_account_profile_photo.as_ref()).await
  }

  pub async fn set_business_account_username<C: AsRef<SetBusinessAccountUsername>>(
    &self,
    set_business_account_username: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_business_account_username.as_ref()).await
  }

  pub async fn set_business_away_message_settings<C: AsRef<SetBusinessAwayMessageSettings>>(
    &self,
    set_business_away_message_settings: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_business_away_message_settings.as_ref()).await
  }

  pub async fn set_business_connected_bot<C: AsRef<SetBusinessConnectedBot>>(
    &self,
    set_business_connected_bot: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_business_connected_bot.as_ref()).await
  }

  pub async fn set_business_greeting_message_settings<
    C: AsRef<SetBusinessGreetingMessageSettings>,
  >(
    &self,
    set_business_greeting_message_settings: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_business_greeting_message_settings.as_ref()).await
  }

  pub async fn set_business_location<C: AsRef<SetBusinessLocation>>(
    &self,
    set_business_location: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_business_location.as_ref()).await
  }

  pub async fn set_business_message_is_pinned<C: AsRef<SetBusinessMessageIsPinned>>(
    &self,
    set_business_message_is_pinned: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_business_message_is_pinned.as_ref()).await
  }

  pub async fn set_business_opening_hours<C: AsRef<SetBusinessOpeningHours>>(
    &self,
    set_business_opening_hours: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_business_opening_hours.as_ref()).await
  }

  pub async fn set_business_start_page<C: AsRef<SetBusinessStartPage>>(
    &self,
    set_business_start_page: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_business_start_page.as_ref()).await
  }

  pub async fn set_chat_accent_color<C: AsRef<SetChatAccentColor>>(
    &self,
    set_chat_accent_color: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_accent_color.as_ref()).await
  }

  pub async fn set_chat_active_stories_list<C: AsRef<SetChatActiveStoriesList>>(
    &self,
    set_chat_active_stories_list: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_active_stories_list.as_ref()).await
  }

  pub async fn set_chat_affiliate_program<C: AsRef<SetChatAffiliateProgram>>(
    &self,
    set_chat_affiliate_program: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_affiliate_program.as_ref()).await
  }

  pub async fn set_chat_available_reactions<C: AsRef<SetChatAvailableReactions>>(
    &self,
    set_chat_available_reactions: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_available_reactions.as_ref()).await
  }

  pub async fn set_chat_background<C: AsRef<SetChatBackground>>(
    &self,
    set_chat_background: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_background.as_ref()).await
  }

  pub async fn set_chat_client_data<C: AsRef<SetChatClientData>>(
    &self,
    set_chat_client_data: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_client_data.as_ref()).await
  }

  pub async fn set_chat_description<C: AsRef<SetChatDescription>>(
    &self,
    set_chat_description: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_description.as_ref()).await
  }

  pub async fn set_chat_direct_messages_group<C: AsRef<SetChatDirectMessagesGroup>>(
    &self,
    set_chat_direct_messages_group: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_direct_messages_group.as_ref()).await
  }

  pub async fn set_chat_discussion_group<C: AsRef<SetChatDiscussionGroup>>(
    &self,
    set_chat_discussion_group: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_discussion_group.as_ref()).await
  }

  pub async fn set_chat_draft_message<C: AsRef<SetChatDraftMessage>>(
    &self,
    set_chat_draft_message: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_draft_message.as_ref()).await
  }

  pub async fn set_chat_emoji_status<C: AsRef<SetChatEmojiStatus>>(
    &self,
    set_chat_emoji_status: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_emoji_status.as_ref()).await
  }

  pub async fn set_chat_location<C: AsRef<SetChatLocation>>(
    &self,
    set_chat_location: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_location.as_ref()).await
  }

  pub async fn set_chat_member_status<C: AsRef<SetChatMemberStatus>>(
    &self,
    set_chat_member_status: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_member_status.as_ref()).await
  }

  pub async fn set_chat_member_tag<C: AsRef<SetChatMemberTag>>(
    &self,
    set_chat_member_tag: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_member_tag.as_ref()).await
  }

  pub async fn set_chat_message_auto_delete_time<C: AsRef<SetChatMessageAutoDeleteTime>>(
    &self,
    set_chat_message_auto_delete_time: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_message_auto_delete_time.as_ref()).await
  }

  pub async fn set_chat_message_sender<C: AsRef<SetChatMessageSender>>(
    &self,
    set_chat_message_sender: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_message_sender.as_ref()).await
  }

  pub async fn set_chat_notification_settings<C: AsRef<SetChatNotificationSettings>>(
    &self,
    set_chat_notification_settings: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_notification_settings.as_ref()).await
  }

  pub async fn set_chat_paid_message_star_count<C: AsRef<SetChatPaidMessageStarCount>>(
    &self,
    set_chat_paid_message_star_count: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_paid_message_star_count.as_ref()).await
  }

  pub async fn set_chat_permissions<C: AsRef<SetChatPermissions>>(
    &self,
    set_chat_permissions: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_permissions.as_ref()).await
  }

  pub async fn set_chat_photo<C: AsRef<SetChatPhoto>>(&self, set_chat_photo: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_photo.as_ref()).await
  }

  pub async fn set_chat_pinned_stories<C: AsRef<SetChatPinnedStories>>(
    &self,
    set_chat_pinned_stories: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_pinned_stories.as_ref()).await
  }

  pub async fn set_chat_profile_accent_color<C: AsRef<SetChatProfileAccentColor>>(
    &self,
    set_chat_profile_accent_color: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_profile_accent_color.as_ref()).await
  }

  pub async fn set_chat_slow_mode_delay<C: AsRef<SetChatSlowModeDelay>>(
    &self,
    set_chat_slow_mode_delay: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_slow_mode_delay.as_ref()).await
  }

  pub async fn set_chat_theme<C: AsRef<SetChatTheme>>(&self, set_chat_theme: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_theme.as_ref()).await
  }

  pub async fn set_chat_title<C: AsRef<SetChatTitle>>(&self, set_chat_title: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_chat_title.as_ref()).await
  }

  pub async fn set_close_friends<C: AsRef<SetCloseFriends>>(
    &self,
    set_close_friends: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_close_friends.as_ref()).await
  }

  pub async fn set_commands<C: AsRef<SetCommands>>(&self, set_commands: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_commands.as_ref()).await
  }

  pub async fn set_community_name<C: AsRef<SetCommunityName>>(
    &self,
    set_community_name: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_community_name.as_ref()).await
  }

  pub async fn set_community_permissions<C: AsRef<SetCommunityPermissions>>(
    &self,
    set_community_permissions: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_community_permissions.as_ref()).await
  }

  pub async fn set_community_photo<C: AsRef<SetCommunityPhoto>>(
    &self,
    set_community_photo: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_community_photo.as_ref()).await
  }

  pub async fn set_custom_emoji_sticker_set_thumbnail<
    C: AsRef<SetCustomEmojiStickerSetThumbnail>,
  >(
    &self,
    set_custom_emoji_sticker_set_thumbnail: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_custom_emoji_sticker_set_thumbnail.as_ref()).await
  }

  pub async fn set_custom_language_pack<C: AsRef<SetCustomLanguagePack>>(
    &self,
    set_custom_language_pack: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_custom_language_pack.as_ref()).await
  }

  pub async fn set_custom_language_pack_string<C: AsRef<SetCustomLanguagePackString>>(
    &self,
    set_custom_language_pack_string: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_custom_language_pack_string.as_ref()).await
  }

  pub async fn set_database_encryption_key<C: AsRef<SetDatabaseEncryptionKey>>(
    &self,
    set_database_encryption_key: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_database_encryption_key.as_ref()).await
  }

  pub async fn set_default_background<C: AsRef<SetDefaultBackground>>(
    &self,
    set_default_background: C,
  ) -> RTDResult<Background> {
    async_caller!(Background);
    async_caller(&self.api, set_default_background.as_ref()).await
  }

  pub async fn set_default_channel_administrator_rights<
    C: AsRef<SetDefaultChannelAdministratorRights>,
  >(
    &self,
    set_default_channel_administrator_rights: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_default_channel_administrator_rights.as_ref()).await
  }

  pub async fn set_default_group_administrator_rights<
    C: AsRef<SetDefaultGroupAdministratorRights>,
  >(
    &self,
    set_default_group_administrator_rights: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_default_group_administrator_rights.as_ref()).await
  }

  pub async fn set_default_message_auto_delete_time<C: AsRef<SetDefaultMessageAutoDeleteTime>>(
    &self,
    set_default_message_auto_delete_time: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_default_message_auto_delete_time.as_ref()).await
  }

  pub async fn set_default_reaction_type<C: AsRef<SetDefaultReactionType>>(
    &self,
    set_default_reaction_type: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_default_reaction_type.as_ref()).await
  }

  pub async fn set_direct_messages_chat_topic_is_marked_as_unread<
    C: AsRef<SetDirectMessagesChatTopicIsMarkedAsUnread>,
  >(
    &self,
    set_direct_messages_chat_topic_is_marked_as_unread: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(
      &self.api,
      set_direct_messages_chat_topic_is_marked_as_unread.as_ref(),
    )
    .await
  }

  pub async fn set_emoji_status<C: AsRef<SetEmojiStatus>>(
    &self,
    set_emoji_status: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_emoji_status.as_ref()).await
  }

  pub async fn set_file_generation_progress<C: AsRef<SetFileGenerationProgress>>(
    &self,
    set_file_generation_progress: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_file_generation_progress.as_ref()).await
  }

  pub async fn set_forum_topic_notification_settings<
    C: AsRef<SetForumTopicNotificationSettings>,
  >(
    &self,
    set_forum_topic_notification_settings: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_forum_topic_notification_settings.as_ref()).await
  }

  pub async fn set_game_score<C: AsRef<SetGameScore>>(
    &self,
    set_game_score: C,
  ) -> RTDResult<Message> {
    async_caller!(Message);
    async_caller(&self.api, set_game_score.as_ref()).await
  }

  pub async fn set_gift_collection_name<C: AsRef<SetGiftCollectionName>>(
    &self,
    set_gift_collection_name: C,
  ) -> RTDResult<GiftCollection> {
    async_caller!(GiftCollection);
    async_caller(&self.api, set_gift_collection_name.as_ref()).await
  }

  pub async fn set_gift_resale_price<C: AsRef<SetGiftResalePrice>>(
    &self,
    set_gift_resale_price: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_gift_resale_price.as_ref()).await
  }

  pub async fn set_gift_settings<C: AsRef<SetGiftSettings>>(
    &self,
    set_gift_settings: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_gift_settings.as_ref()).await
  }

  pub async fn set_group_call_paid_message_star_count<
    C: AsRef<SetGroupCallPaidMessageStarCount>,
  >(
    &self,
    set_group_call_paid_message_star_count: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_group_call_paid_message_star_count.as_ref()).await
  }

  pub async fn set_group_call_participant_is_speaking<
    C: AsRef<SetGroupCallParticipantIsSpeaking>,
  >(
    &self,
    set_group_call_participant_is_speaking: C,
  ) -> RTDResult<MessageSender> {
    async_caller!(MessageSender);
    async_caller(&self.api, set_group_call_participant_is_speaking.as_ref()).await
  }

  pub async fn set_group_call_participant_volume_level<
    C: AsRef<SetGroupCallParticipantVolumeLevel>,
  >(
    &self,
    set_group_call_participant_volume_level: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_group_call_participant_volume_level.as_ref()).await
  }

  pub async fn set_inactive_session_ttl<C: AsRef<SetInactiveSessionTtl>>(
    &self,
    set_inactive_session_ttl: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_inactive_session_ttl.as_ref()).await
  }

  pub async fn set_inline_game_score<C: AsRef<SetInlineGameScore>>(
    &self,
    set_inline_game_score: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_inline_game_score.as_ref()).await
  }

  pub async fn set_live_story_message_sender<C: AsRef<SetLiveStoryMessageSender>>(
    &self,
    set_live_story_message_sender: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_live_story_message_sender.as_ref()).await
  }

  pub async fn set_log_stream<C: AsRef<SetLogStream>>(&self, set_log_stream: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_log_stream.as_ref()).await
  }

  pub async fn set_log_tag_verbosity_level<C: AsRef<SetLogTagVerbosityLevel>>(
    &self,
    set_log_tag_verbosity_level: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_log_tag_verbosity_level.as_ref()).await
  }

  pub async fn set_log_verbosity_level<C: AsRef<SetLogVerbosityLevel>>(
    &self,
    set_log_verbosity_level: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_log_verbosity_level.as_ref()).await
  }

  pub async fn set_login_email_address<C: AsRef<SetLoginEmailAddress>>(
    &self,
    set_login_email_address: C,
  ) -> RTDResult<EmailAddressAuthenticationCodeInfo> {
    async_caller!(EmailAddressAuthenticationCodeInfo);
    async_caller(&self.api, set_login_email_address.as_ref()).await
  }

  pub async fn set_main_profile_tab<C: AsRef<SetMainProfileTab>>(
    &self,
    set_main_profile_tab: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_main_profile_tab.as_ref()).await
  }

  pub async fn set_managed_bot_access_settings<C: AsRef<SetManagedBotAccessSettings>>(
    &self,
    set_managed_bot_access_settings: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_managed_bot_access_settings.as_ref()).await
  }

  pub async fn set_menu_button<C: AsRef<SetMenuButton>>(
    &self,
    set_menu_button: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_menu_button.as_ref()).await
  }

  pub async fn set_message_fact_check<C: AsRef<SetMessageFactCheck>>(
    &self,
    set_message_fact_check: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_message_fact_check.as_ref()).await
  }

  pub async fn set_message_reactions<C: AsRef<SetMessageReactions>>(
    &self,
    set_message_reactions: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_message_reactions.as_ref()).await
  }

  pub async fn set_message_sender_block_list<C: AsRef<SetMessageSenderBlockList>>(
    &self,
    set_message_sender_block_list: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_message_sender_block_list.as_ref()).await
  }

  pub async fn set_message_sender_bot_verification<C: AsRef<SetMessageSenderBotVerification>>(
    &self,
    set_message_sender_bot_verification: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_message_sender_bot_verification.as_ref()).await
  }

  pub async fn set_name<C: AsRef<SetName>>(&self, set_name: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_name.as_ref()).await
  }

  pub async fn set_network_type<C: AsRef<SetNetworkType>>(
    &self,
    set_network_type: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_network_type.as_ref()).await
  }

  pub async fn set_new_chat_privacy_settings<C: AsRef<SetNewChatPrivacySettings>>(
    &self,
    set_new_chat_privacy_settings: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_new_chat_privacy_settings.as_ref()).await
  }

  pub async fn set_option<C: AsRef<SetOption>>(&self, set_option: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_option.as_ref()).await
  }

  pub async fn set_paid_message_reaction_type<C: AsRef<SetPaidMessageReactionType>>(
    &self,
    set_paid_message_reaction_type: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_paid_message_reaction_type.as_ref()).await
  }

  pub async fn set_passport_element<C: AsRef<SetPassportElement>>(
    &self,
    set_passport_element: C,
  ) -> RTDResult<PassportElement> {
    async_caller!(PassportElement);
    async_caller(&self.api, set_passport_element.as_ref()).await
  }

  pub async fn set_passport_element_errors<C: AsRef<SetPassportElementErrors>>(
    &self,
    set_passport_element_errors: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_passport_element_errors.as_ref()).await
  }

  pub async fn set_password<C: AsRef<SetPassword>>(
    &self,
    set_password: C,
  ) -> RTDResult<PasswordState> {
    async_caller!(PasswordState);
    async_caller(&self.api, set_password.as_ref()).await
  }

  pub async fn set_personal_chat<C: AsRef<SetPersonalChat>>(
    &self,
    set_personal_chat: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_personal_chat.as_ref()).await
  }

  pub async fn set_pinned_chats<C: AsRef<SetPinnedChats>>(
    &self,
    set_pinned_chats: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_pinned_chats.as_ref()).await
  }

  pub async fn set_pinned_forum_topics<C: AsRef<SetPinnedForumTopics>>(
    &self,
    set_pinned_forum_topics: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_pinned_forum_topics.as_ref()).await
  }

  pub async fn set_pinned_gifts<C: AsRef<SetPinnedGifts>>(
    &self,
    set_pinned_gifts: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_pinned_gifts.as_ref()).await
  }

  pub async fn set_pinned_saved_messages_topics<C: AsRef<SetPinnedSavedMessagesTopics>>(
    &self,
    set_pinned_saved_messages_topics: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_pinned_saved_messages_topics.as_ref()).await
  }

  pub async fn set_poll_answer<C: AsRef<SetPollAnswer>>(
    &self,
    set_poll_answer: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_poll_answer.as_ref()).await
  }

  pub async fn set_profile_accent_color<C: AsRef<SetProfileAccentColor>>(
    &self,
    set_profile_accent_color: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_profile_accent_color.as_ref()).await
  }

  pub async fn set_profile_audio_position<C: AsRef<SetProfileAudioPosition>>(
    &self,
    set_profile_audio_position: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_profile_audio_position.as_ref()).await
  }

  pub async fn set_profile_photo<C: AsRef<SetProfilePhoto>>(
    &self,
    set_profile_photo: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_profile_photo.as_ref()).await
  }

  pub async fn set_profile_share_theme<C: AsRef<SetProfileShareTheme>>(
    &self,
    set_profile_share_theme: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_profile_share_theme.as_ref()).await
  }

  pub async fn set_quick_reply_shortcut_name<C: AsRef<SetQuickReplyShortcutName>>(
    &self,
    set_quick_reply_shortcut_name: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_quick_reply_shortcut_name.as_ref()).await
  }

  pub async fn set_reaction_notification_settings<C: AsRef<SetReactionNotificationSettings>>(
    &self,
    set_reaction_notification_settings: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_reaction_notification_settings.as_ref()).await
  }

  pub async fn set_read_date_privacy_settings<C: AsRef<SetReadDatePrivacySettings>>(
    &self,
    set_read_date_privacy_settings: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_read_date_privacy_settings.as_ref()).await
  }

  pub async fn set_recovery_email_address<C: AsRef<SetRecoveryEmailAddress>>(
    &self,
    set_recovery_email_address: C,
  ) -> RTDResult<PasswordState> {
    async_caller!(PasswordState);
    async_caller(&self.api, set_recovery_email_address.as_ref()).await
  }

  pub async fn set_saved_messages_tag_label<C: AsRef<SetSavedMessagesTagLabel>>(
    &self,
    set_saved_messages_tag_label: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_saved_messages_tag_label.as_ref()).await
  }

  pub async fn set_scope_notification_settings<C: AsRef<SetScopeNotificationSettings>>(
    &self,
    set_scope_notification_settings: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_scope_notification_settings.as_ref()).await
  }

  pub async fn set_sticker_emojis<C: AsRef<SetStickerEmojis>>(
    &self,
    set_sticker_emojis: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_sticker_emojis.as_ref()).await
  }

  pub async fn set_sticker_keywords<C: AsRef<SetStickerKeywords>>(
    &self,
    set_sticker_keywords: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_sticker_keywords.as_ref()).await
  }

  pub async fn set_sticker_mask_position<C: AsRef<SetStickerMaskPosition>>(
    &self,
    set_sticker_mask_position: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_sticker_mask_position.as_ref()).await
  }

  pub async fn set_sticker_position_in_set<C: AsRef<SetStickerPositionInSet>>(
    &self,
    set_sticker_position_in_set: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_sticker_position_in_set.as_ref()).await
  }

  pub async fn set_sticker_set_thumbnail<C: AsRef<SetStickerSetThumbnail>>(
    &self,
    set_sticker_set_thumbnail: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_sticker_set_thumbnail.as_ref()).await
  }

  pub async fn set_sticker_set_title<C: AsRef<SetStickerSetTitle>>(
    &self,
    set_sticker_set_title: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_sticker_set_title.as_ref()).await
  }

  pub async fn set_story_album_name<C: AsRef<SetStoryAlbumName>>(
    &self,
    set_story_album_name: C,
  ) -> RTDResult<StoryAlbum> {
    async_caller!(StoryAlbum);
    async_caller(&self.api, set_story_album_name.as_ref()).await
  }

  pub async fn set_story_privacy_settings<C: AsRef<SetStoryPrivacySettings>>(
    &self,
    set_story_privacy_settings: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_story_privacy_settings.as_ref()).await
  }

  pub async fn set_story_reaction<C: AsRef<SetStoryReaction>>(
    &self,
    set_story_reaction: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_story_reaction.as_ref()).await
  }

  pub async fn set_supergroup_custom_emoji_sticker_set<
    C: AsRef<SetSupergroupCustomEmojiStickerSet>,
  >(
    &self,
    set_supergroup_custom_emoji_sticker_set: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_supergroup_custom_emoji_sticker_set.as_ref()).await
  }

  pub async fn set_supergroup_main_profile_tab<C: AsRef<SetSupergroupMainProfileTab>>(
    &self,
    set_supergroup_main_profile_tab: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_supergroup_main_profile_tab.as_ref()).await
  }

  pub async fn set_supergroup_sticker_set<C: AsRef<SetSupergroupStickerSet>>(
    &self,
    set_supergroup_sticker_set: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_supergroup_sticker_set.as_ref()).await
  }

  pub async fn set_supergroup_unrestrict_boost_count<
    C: AsRef<SetSupergroupUnrestrictBoostCount>,
  >(
    &self,
    set_supergroup_unrestrict_boost_count: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_supergroup_unrestrict_boost_count.as_ref()).await
  }

  pub async fn set_supergroup_username<C: AsRef<SetSupergroupUsername>>(
    &self,
    set_supergroup_username: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_supergroup_username.as_ref()).await
  }

  pub async fn set_tdlib_parameters<C: AsRef<SetTdlibParameters>>(
    &self,
    set_tdlib_parameters: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_tdlib_parameters.as_ref()).await
  }

  pub async fn set_ton_connect_session_wallet_client_id<
    C: AsRef<SetTonConnectSessionWalletClientId>,
  >(
    &self,
    set_ton_connect_session_wallet_client_id: C,
  ) -> RTDResult<TonConnectChallenge> {
    async_caller!(TonConnectChallenge);
    async_caller(&self.api, set_ton_connect_session_wallet_client_id.as_ref()).await
  }

  pub async fn set_ton_wallet_card_stickers<C: AsRef<SetTonWalletCardStickers>>(
    &self,
    set_ton_wallet_card_stickers: C,
  ) -> RTDResult<TonWalletCard> {
    async_caller!(TonWalletCard);
    async_caller(&self.api, set_ton_wallet_card_stickers.as_ref()).await
  }

  pub async fn set_upgraded_gift_colors<C: AsRef<SetUpgradedGiftColors>>(
    &self,
    set_upgraded_gift_colors: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_upgraded_gift_colors.as_ref()).await
  }

  pub async fn set_user_emoji_status<C: AsRef<SetUserEmojiStatus>>(
    &self,
    set_user_emoji_status: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_user_emoji_status.as_ref()).await
  }

  pub async fn set_user_note<C: AsRef<SetUserNote>>(&self, set_user_note: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_user_note.as_ref()).await
  }

  pub async fn set_user_personal_profile_photo<C: AsRef<SetUserPersonalProfilePhoto>>(
    &self,
    set_user_personal_profile_photo: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_user_personal_profile_photo.as_ref()).await
  }

  pub async fn set_user_privacy_setting_rules<C: AsRef<SetUserPrivacySettingRules>>(
    &self,
    set_user_privacy_setting_rules: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_user_privacy_setting_rules.as_ref()).await
  }

  pub async fn set_user_support_info<C: AsRef<SetUserSupportInfo>>(
    &self,
    set_user_support_info: C,
  ) -> RTDResult<UserSupportInfo> {
    async_caller!(UserSupportInfo);
    async_caller(&self.api, set_user_support_info.as_ref()).await
  }

  pub async fn set_username<C: AsRef<SetUsername>>(&self, set_username: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_username.as_ref()).await
  }

  pub async fn set_video_chat_default_participant<C: AsRef<SetVideoChatDefaultParticipant>>(
    &self,
    set_video_chat_default_participant: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_video_chat_default_participant.as_ref()).await
  }

  pub async fn set_video_chat_title<C: AsRef<SetVideoChatTitle>>(
    &self,
    set_video_chat_title: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, set_video_chat_title.as_ref()).await
  }

  pub async fn share_chat_with_bot<C: AsRef<ShareChatWithBot>>(
    &self,
    share_chat_with_bot: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, share_chat_with_bot.as_ref()).await
  }

  pub async fn share_phone_number<C: AsRef<SharePhoneNumber>>(
    &self,
    share_phone_number: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, share_phone_number.as_ref()).await
  }

  pub async fn share_users_with_bot<C: AsRef<ShareUsersWithBot>>(
    &self,
    share_users_with_bot: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, share_users_with_bot.as_ref()).await
  }

  pub async fn start_group_call_recording<C: AsRef<StartGroupCallRecording>>(
    &self,
    start_group_call_recording: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, start_group_call_recording.as_ref()).await
  }

  pub async fn start_group_call_screen_sharing<C: AsRef<StartGroupCallScreenSharing>>(
    &self,
    start_group_call_screen_sharing: C,
  ) -> RTDResult<Text> {
    async_caller!(Text);
    async_caller(&self.api, start_group_call_screen_sharing.as_ref()).await
  }

  pub async fn start_live_story<C: AsRef<StartLiveStory>>(
    &self,
    start_live_story: C,
  ) -> RTDResult<StartLiveStoryResult> {
    async_caller!(StartLiveStoryResult);
    async_caller(&self.api, start_live_story.as_ref()).await
  }

  pub async fn start_scheduled_video_chat<C: AsRef<StartScheduledVideoChat>>(
    &self,
    start_scheduled_video_chat: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, start_scheduled_video_chat.as_ref()).await
  }

  pub async fn stop_business_poll<C: AsRef<StopBusinessPoll>>(
    &self,
    stop_business_poll: C,
  ) -> RTDResult<BusinessMessage> {
    async_caller!(BusinessMessage);
    async_caller(&self.api, stop_business_poll.as_ref()).await
  }

  pub async fn stop_pending_message<C: AsRef<StopPendingMessage>>(
    &self,
    stop_pending_message: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, stop_pending_message.as_ref()).await
  }

  pub async fn stop_poll<C: AsRef<StopPoll>>(&self, stop_poll: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, stop_poll.as_ref()).await
  }

  pub async fn suggest_user_birthdate<C: AsRef<SuggestUserBirthdate>>(
    &self,
    suggest_user_birthdate: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, suggest_user_birthdate.as_ref()).await
  }

  pub async fn suggest_user_profile_photo<C: AsRef<SuggestUserProfilePhoto>>(
    &self,
    suggest_user_profile_photo: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, suggest_user_profile_photo.as_ref()).await
  }

  pub async fn summarize_message<C: AsRef<SummarizeMessage>>(
    &self,
    summarize_message: C,
  ) -> RTDResult<FormattedText> {
    async_caller!(FormattedText);
    async_caller(&self.api, summarize_message.as_ref()).await
  }

  pub async fn synchronize_language_pack<C: AsRef<SynchronizeLanguagePack>>(
    &self,
    synchronize_language_pack: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, synchronize_language_pack.as_ref()).await
  }

  pub async fn terminate_all_other_sessions<C: AsRef<TerminateAllOtherSessions>>(
    &self,
    terminate_all_other_sessions: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, terminate_all_other_sessions.as_ref()).await
  }

  pub async fn terminate_session<C: AsRef<TerminateSession>>(
    &self,
    terminate_session: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, terminate_session.as_ref()).await
  }

  pub async fn test_call_bytes<C: AsRef<TestCallBytes>>(
    &self,
    test_call_bytes: C,
  ) -> RTDResult<TestBytes> {
    async_caller!(TestBytes);
    async_caller(&self.api, test_call_bytes.as_ref()).await
  }

  pub async fn test_call_empty<C: AsRef<TestCallEmpty>>(
    &self,
    test_call_empty: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, test_call_empty.as_ref()).await
  }

  pub async fn test_call_string<C: AsRef<TestCallString>>(
    &self,
    test_call_string: C,
  ) -> RTDResult<TestString> {
    async_caller!(TestString);
    async_caller(&self.api, test_call_string.as_ref()).await
  }

  pub async fn test_call_vector_int<C: AsRef<TestCallVectorInt>>(
    &self,
    test_call_vector_int: C,
  ) -> RTDResult<TestVectorInt> {
    async_caller!(TestVectorInt);
    async_caller(&self.api, test_call_vector_int.as_ref()).await
  }

  pub async fn test_call_vector_int_object<C: AsRef<TestCallVectorIntObject>>(
    &self,
    test_call_vector_int_object: C,
  ) -> RTDResult<TestVectorIntObject> {
    async_caller!(TestVectorIntObject);
    async_caller(&self.api, test_call_vector_int_object.as_ref()).await
  }

  pub async fn test_call_vector_string<C: AsRef<TestCallVectorString>>(
    &self,
    test_call_vector_string: C,
  ) -> RTDResult<TestVectorString> {
    async_caller!(TestVectorString);
    async_caller(&self.api, test_call_vector_string.as_ref()).await
  }

  pub async fn test_call_vector_string_object<C: AsRef<TestCallVectorStringObject>>(
    &self,
    test_call_vector_string_object: C,
  ) -> RTDResult<TestVectorStringObject> {
    async_caller!(TestVectorStringObject);
    async_caller(&self.api, test_call_vector_string_object.as_ref()).await
  }

  pub async fn test_get_difference<C: AsRef<TestGetDifference>>(
    &self,
    test_get_difference: C,
  ) -> RTDResult<Ok> {
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

  pub async fn test_return_error<C: AsRef<TestReturnError>>(
    &self,
    test_return_error: C,
  ) -> RTDResult<Error> {
    async_caller!(Error);
    async_caller(&self.api, test_return_error.as_ref()).await
  }

  pub async fn test_square_int<C: AsRef<TestSquareInt>>(
    &self,
    test_square_int: C,
  ) -> RTDResult<TestInt> {
    async_caller!(TestInt);
    async_caller(&self.api, test_square_int.as_ref()).await
  }

  pub async fn test_use_update<C: AsRef<TestUseUpdate>>(
    &self,
    test_use_update: C,
  ) -> RTDResult<Update> {
    async_caller!(Update);
    async_caller(&self.api, test_use_update.as_ref()).await
  }

  pub async fn toggle_all_downloads_are_paused<C: AsRef<ToggleAllDownloadsArePaused>>(
    &self,
    toggle_all_downloads_are_paused: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_all_downloads_are_paused.as_ref()).await
  }

  pub async fn toggle_bot_can_manage_emoji_status<C: AsRef<ToggleBotCanManageEmojiStatus>>(
    &self,
    toggle_bot_can_manage_emoji_status: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_bot_can_manage_emoji_status.as_ref()).await
  }

  pub async fn toggle_bot_is_added_to_attachment_menu<
    C: AsRef<ToggleBotIsAddedToAttachmentMenu>,
  >(
    &self,
    toggle_bot_is_added_to_attachment_menu: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_bot_is_added_to_attachment_menu.as_ref()).await
  }

  pub async fn toggle_bot_username_is_active<C: AsRef<ToggleBotUsernameIsActive>>(
    &self,
    toggle_bot_username_is_active: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_bot_username_is_active.as_ref()).await
  }

  pub async fn toggle_business_connected_bot_chat_is_paused<
    C: AsRef<ToggleBusinessConnectedBotChatIsPaused>,
  >(
    &self,
    toggle_business_connected_bot_chat_is_paused: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(
      &self.api,
      toggle_business_connected_bot_chat_is_paused.as_ref(),
    )
    .await
  }

  pub async fn toggle_chat_default_disable_notification<
    C: AsRef<ToggleChatDefaultDisableNotification>,
  >(
    &self,
    toggle_chat_default_disable_notification: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_chat_default_disable_notification.as_ref()).await
  }

  pub async fn toggle_chat_folder_tags<C: AsRef<ToggleChatFolderTags>>(
    &self,
    toggle_chat_folder_tags: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_chat_folder_tags.as_ref()).await
  }

  pub async fn toggle_chat_gift_notifications<C: AsRef<ToggleChatGiftNotifications>>(
    &self,
    toggle_chat_gift_notifications: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_chat_gift_notifications.as_ref()).await
  }

  pub async fn toggle_chat_has_protected_content<C: AsRef<ToggleChatHasProtectedContent>>(
    &self,
    toggle_chat_has_protected_content: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_chat_has_protected_content.as_ref()).await
  }

  pub async fn toggle_chat_is_marked_as_unread<C: AsRef<ToggleChatIsMarkedAsUnread>>(
    &self,
    toggle_chat_is_marked_as_unread: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_chat_is_marked_as_unread.as_ref()).await
  }

  pub async fn toggle_chat_is_pinned<C: AsRef<ToggleChatIsPinned>>(
    &self,
    toggle_chat_is_pinned: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_chat_is_pinned.as_ref()).await
  }

  pub async fn toggle_chat_is_translatable<C: AsRef<ToggleChatIsTranslatable>>(
    &self,
    toggle_chat_is_translatable: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_chat_is_translatable.as_ref()).await
  }

  pub async fn toggle_chat_view_as_topics<C: AsRef<ToggleChatViewAsTopics>>(
    &self,
    toggle_chat_view_as_topics: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_chat_view_as_topics.as_ref()).await
  }

  pub async fn toggle_direct_messages_chat_topic_can_send_unpaid_messages<
    C: AsRef<ToggleDirectMessagesChatTopicCanSendUnpaidMessages>,
  >(
    &self,
    toggle_direct_messages_chat_topic_can_send_unpaid_messages: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(
      &self.api,
      toggle_direct_messages_chat_topic_can_send_unpaid_messages.as_ref(),
    )
    .await
  }

  pub async fn toggle_download_is_paused<C: AsRef<ToggleDownloadIsPaused>>(
    &self,
    toggle_download_is_paused: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_download_is_paused.as_ref()).await
  }

  pub async fn toggle_forum_topic_is_closed<C: AsRef<ToggleForumTopicIsClosed>>(
    &self,
    toggle_forum_topic_is_closed: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_forum_topic_is_closed.as_ref()).await
  }

  pub async fn toggle_forum_topic_is_pinned<C: AsRef<ToggleForumTopicIsPinned>>(
    &self,
    toggle_forum_topic_is_pinned: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_forum_topic_is_pinned.as_ref()).await
  }

  pub async fn toggle_general_forum_topic_is_hidden<C: AsRef<ToggleGeneralForumTopicIsHidden>>(
    &self,
    toggle_general_forum_topic_is_hidden: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_general_forum_topic_is_hidden.as_ref()).await
  }

  pub async fn toggle_gift_is_saved<C: AsRef<ToggleGiftIsSaved>>(
    &self,
    toggle_gift_is_saved: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_gift_is_saved.as_ref()).await
  }

  pub async fn toggle_group_call_are_messages_allowed<
    C: AsRef<ToggleGroupCallAreMessagesAllowed>,
  >(
    &self,
    toggle_group_call_are_messages_allowed: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_group_call_are_messages_allowed.as_ref()).await
  }

  pub async fn toggle_group_call_is_my_video_enabled<C: AsRef<ToggleGroupCallIsMyVideoEnabled>>(
    &self,
    toggle_group_call_is_my_video_enabled: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_group_call_is_my_video_enabled.as_ref()).await
  }

  pub async fn toggle_group_call_is_my_video_paused<C: AsRef<ToggleGroupCallIsMyVideoPaused>>(
    &self,
    toggle_group_call_is_my_video_paused: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_group_call_is_my_video_paused.as_ref()).await
  }

  pub async fn toggle_group_call_participant_is_hand_raised<
    C: AsRef<ToggleGroupCallParticipantIsHandRaised>,
  >(
    &self,
    toggle_group_call_participant_is_hand_raised: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(
      &self.api,
      toggle_group_call_participant_is_hand_raised.as_ref(),
    )
    .await
  }

  pub async fn toggle_group_call_participant_is_muted<
    C: AsRef<ToggleGroupCallParticipantIsMuted>,
  >(
    &self,
    toggle_group_call_participant_is_muted: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_group_call_participant_is_muted.as_ref()).await
  }

  pub async fn toggle_group_call_screen_sharing_is_paused<
    C: AsRef<ToggleGroupCallScreenSharingIsPaused>,
  >(
    &self,
    toggle_group_call_screen_sharing_is_paused: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(
      &self.api,
      toggle_group_call_screen_sharing_is_paused.as_ref(),
    )
    .await
  }

  pub async fn toggle_has_sponsored_messages_enabled<
    C: AsRef<ToggleHasSponsoredMessagesEnabled>,
  >(
    &self,
    toggle_has_sponsored_messages_enabled: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_has_sponsored_messages_enabled.as_ref()).await
  }

  pub async fn toggle_saved_messages_topic_is_pinned<C: AsRef<ToggleSavedMessagesTopicIsPinned>>(
    &self,
    toggle_saved_messages_topic_is_pinned: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_saved_messages_topic_is_pinned.as_ref()).await
  }

  pub async fn toggle_session_can_accept_calls<C: AsRef<ToggleSessionCanAcceptCalls>>(
    &self,
    toggle_session_can_accept_calls: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_session_can_accept_calls.as_ref()).await
  }

  pub async fn toggle_session_can_accept_secret_chats<
    C: AsRef<ToggleSessionCanAcceptSecretChats>,
  >(
    &self,
    toggle_session_can_accept_secret_chats: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_session_can_accept_secret_chats.as_ref()).await
  }

  pub async fn toggle_story_is_posted_to_chat_page<C: AsRef<ToggleStoryIsPostedToChatPage>>(
    &self,
    toggle_story_is_posted_to_chat_page: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_story_is_posted_to_chat_page.as_ref()).await
  }

  pub async fn toggle_supergroup_can_have_sponsored_messages<
    C: AsRef<ToggleSupergroupCanHaveSponsoredMessages>,
  >(
    &self,
    toggle_supergroup_can_have_sponsored_messages: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(
      &self.api,
      toggle_supergroup_can_have_sponsored_messages.as_ref(),
    )
    .await
  }

  pub async fn toggle_supergroup_has_aggressive_anti_spam_enabled<
    C: AsRef<ToggleSupergroupHasAggressiveAntiSpamEnabled>,
  >(
    &self,
    toggle_supergroup_has_aggressive_anti_spam_enabled: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(
      &self.api,
      toggle_supergroup_has_aggressive_anti_spam_enabled.as_ref(),
    )
    .await
  }

  pub async fn toggle_supergroup_has_automatic_translation<
    C: AsRef<ToggleSupergroupHasAutomaticTranslation>,
  >(
    &self,
    toggle_supergroup_has_automatic_translation: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(
      &self.api,
      toggle_supergroup_has_automatic_translation.as_ref(),
    )
    .await
  }

  pub async fn toggle_supergroup_has_hidden_members<C: AsRef<ToggleSupergroupHasHiddenMembers>>(
    &self,
    toggle_supergroup_has_hidden_members: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_supergroup_has_hidden_members.as_ref()).await
  }

  pub async fn toggle_supergroup_is_all_history_available<
    C: AsRef<ToggleSupergroupIsAllHistoryAvailable>,
  >(
    &self,
    toggle_supergroup_is_all_history_available: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(
      &self.api,
      toggle_supergroup_is_all_history_available.as_ref(),
    )
    .await
  }

  pub async fn toggle_supergroup_is_broadcast_group<C: AsRef<ToggleSupergroupIsBroadcastGroup>>(
    &self,
    toggle_supergroup_is_broadcast_group: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_supergroup_is_broadcast_group.as_ref()).await
  }

  pub async fn toggle_supergroup_is_forum<C: AsRef<ToggleSupergroupIsForum>>(
    &self,
    toggle_supergroup_is_forum: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_supergroup_is_forum.as_ref()).await
  }

  pub async fn toggle_supergroup_join_by_request<C: AsRef<ToggleSupergroupJoinByRequest>>(
    &self,
    toggle_supergroup_join_by_request: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_supergroup_join_by_request.as_ref()).await
  }

  pub async fn toggle_supergroup_join_to_send_messages<
    C: AsRef<ToggleSupergroupJoinToSendMessages>,
  >(
    &self,
    toggle_supergroup_join_to_send_messages: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_supergroup_join_to_send_messages.as_ref()).await
  }

  pub async fn toggle_supergroup_sign_messages<C: AsRef<ToggleSupergroupSignMessages>>(
    &self,
    toggle_supergroup_sign_messages: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_supergroup_sign_messages.as_ref()).await
  }

  pub async fn toggle_supergroup_username_is_active<C: AsRef<ToggleSupergroupUsernameIsActive>>(
    &self,
    toggle_supergroup_username_is_active: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_supergroup_username_is_active.as_ref()).await
  }

  pub async fn toggle_username_is_active<C: AsRef<ToggleUsernameIsActive>>(
    &self,
    toggle_username_is_active: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_username_is_active.as_ref()).await
  }

  pub async fn toggle_video_chat_enabled_start_notification<
    C: AsRef<ToggleVideoChatEnabledStartNotification>,
  >(
    &self,
    toggle_video_chat_enabled_start_notification: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(
      &self.api,
      toggle_video_chat_enabled_start_notification.as_ref(),
    )
    .await
  }

  pub async fn toggle_video_chat_mute_new_participants<
    C: AsRef<ToggleVideoChatMuteNewParticipants>,
  >(
    &self,
    toggle_video_chat_mute_new_participants: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, toggle_video_chat_mute_new_participants.as_ref()).await
  }

  pub async fn transfer_business_account_stars<C: AsRef<TransferBusinessAccountStars>>(
    &self,
    transfer_business_account_stars: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, transfer_business_account_stars.as_ref()).await
  }

  pub async fn transfer_chat_ownership<C: AsRef<TransferChatOwnership>>(
    &self,
    transfer_chat_ownership: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, transfer_chat_ownership.as_ref()).await
  }

  pub async fn transfer_gift<C: AsRef<TransferGift>>(&self, transfer_gift: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, transfer_gift.as_ref()).await
  }

  pub async fn translate_message_rich_message<C: AsRef<TranslateMessageRichMessage>>(
    &self,
    translate_message_rich_message: C,
  ) -> RTDResult<RichMessage> {
    async_caller!(RichMessage);
    async_caller(&self.api, translate_message_rich_message.as_ref()).await
  }

  pub async fn translate_message_text<C: AsRef<TranslateMessageText>>(
    &self,
    translate_message_text: C,
  ) -> RTDResult<FormattedText> {
    async_caller!(FormattedText);
    async_caller(&self.api, translate_message_text.as_ref()).await
  }

  pub async fn translate_rich_message<C: AsRef<TranslateRichMessage>>(
    &self,
    translate_rich_message: C,
  ) -> RTDResult<RichMessage> {
    async_caller!(RichMessage);
    async_caller(&self.api, translate_rich_message.as_ref()).await
  }

  pub async fn translate_text<C: AsRef<TranslateText>>(
    &self,
    translate_text: C,
  ) -> RTDResult<FormattedText> {
    async_caller!(FormattedText);
    async_caller(&self.api, translate_text.as_ref()).await
  }

  pub async fn unpin_all_chat_messages<C: AsRef<UnpinAllChatMessages>>(
    &self,
    unpin_all_chat_messages: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, unpin_all_chat_messages.as_ref()).await
  }

  pub async fn unpin_all_direct_messages_chat_topic_messages<
    C: AsRef<UnpinAllDirectMessagesChatTopicMessages>,
  >(
    &self,
    unpin_all_direct_messages_chat_topic_messages: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(
      &self.api,
      unpin_all_direct_messages_chat_topic_messages.as_ref(),
    )
    .await
  }

  pub async fn unpin_all_forum_topic_messages<C: AsRef<UnpinAllForumTopicMessages>>(
    &self,
    unpin_all_forum_topic_messages: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, unpin_all_forum_topic_messages.as_ref()).await
  }

  pub async fn unpin_chat_message<C: AsRef<UnpinChatMessage>>(
    &self,
    unpin_chat_message: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, unpin_chat_message.as_ref()).await
  }

  pub async fn upgrade_basic_group_chat_to_supergroup_chat<
    C: AsRef<UpgradeBasicGroupChatToSupergroupChat>,
  >(
    &self,
    upgrade_basic_group_chat_to_supergroup_chat: C,
  ) -> RTDResult<Chat> {
    async_caller!(Chat);
    async_caller(
      &self.api,
      upgrade_basic_group_chat_to_supergroup_chat.as_ref(),
    )
    .await
  }

  pub async fn upgrade_gift<C: AsRef<UpgradeGift>>(
    &self,
    upgrade_gift: C,
  ) -> RTDResult<UpgradeGiftResult> {
    async_caller!(UpgradeGiftResult);
    async_caller(&self.api, upgrade_gift.as_ref()).await
  }

  pub async fn upload_sticker_file<C: AsRef<UploadStickerFile>>(
    &self,
    upload_sticker_file: C,
  ) -> RTDResult<File> {
    async_caller!(File);
    async_caller(&self.api, upload_sticker_file.as_ref()).await
  }

  pub async fn validate_order_info<C: AsRef<ValidateOrderInfo>>(
    &self,
    validate_order_info: C,
  ) -> RTDResult<ValidatedOrderInfo> {
    async_caller!(ValidatedOrderInfo);
    async_caller(&self.api, validate_order_info.as_ref()).await
  }

  pub async fn view_messages<C: AsRef<ViewMessages>>(&self, view_messages: C) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, view_messages.as_ref()).await
  }

  pub async fn view_premium_feature<C: AsRef<ViewPremiumFeature>>(
    &self,
    view_premium_feature: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, view_premium_feature.as_ref()).await
  }

  pub async fn view_sponsored_chat<C: AsRef<ViewSponsoredChat>>(
    &self,
    view_sponsored_chat: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, view_sponsored_chat.as_ref()).await
  }

  pub async fn view_trending_sticker_sets<C: AsRef<ViewTrendingStickerSets>>(
    &self,
    view_trending_sticker_sets: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, view_trending_sticker_sets.as_ref()).await
  }

  pub async fn view_video_message_advertisement<C: AsRef<ViewVideoMessageAdvertisement>>(
    &self,
    view_video_message_advertisement: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, view_video_message_advertisement.as_ref()).await
  }

  pub async fn withdraw_upgraded_gift<C: AsRef<WithdrawUpgradedGift>>(
    &self,
    withdraw_upgraded_gift: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, withdraw_upgraded_gift.as_ref()).await
  }

  pub async fn write_generated_file_part<C: AsRef<WriteGeneratedFilePart>>(
    &self,
    write_generated_file_part: C,
  ) -> RTDResult<Ok> {
    async_caller!(Ok);
    async_caller(&self.api, write_generated_file_part.as_ref()).await
  }
}
