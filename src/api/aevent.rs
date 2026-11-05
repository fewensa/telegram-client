// Generated from TDLib 1.8.69 td_api.tl (commit 4c6377aa7386c39217878a9a56d581922ad37ca4).
// Callback signatures are the public API.
#![allow(clippy::type_complexity)]

use crate::api::Api;
use rtdlib::errors::*;
use rtdlib::types::*;

#[derive(Debug, Clone)]
pub struct EventApi {
  api: Api,
}

impl EventApi {
  pub fn new(api: Api) -> Self {
    Self { api }
  }

  #[doc(hidden)]
  pub fn api(&self) -> &Api {
    &self.api
  }

  pub fn accept_call<C: AsRef<AcceptCall>>(&self, accept_call: C) -> RTDResult<()> {
    self.api.send(accept_call.as_ref())
  }

  pub fn accept_oauth_request<C: AsRef<AcceptOauthRequest>>(
    &self,
    accept_oauth_request: C,
  ) -> RTDResult<()> {
    self.api.send(accept_oauth_request.as_ref())
  }

  pub fn accept_terms_of_service<C: AsRef<AcceptTermsOfService>>(
    &self,
    accept_terms_of_service: C,
  ) -> RTDResult<()> {
    self.api.send(accept_terms_of_service.as_ref())
  }

  pub fn activate_story_stealth_mode<C: AsRef<ActivateStoryStealthMode>>(
    &self,
    activate_story_stealth_mode: C,
  ) -> RTDResult<()> {
    self.api.send(activate_story_stealth_mode.as_ref())
  }

  pub fn add_bot_media_preview<C: AsRef<AddBotMediaPreview>>(
    &self,
    add_bot_media_preview: C,
  ) -> RTDResult<()> {
    self.api.send(add_bot_media_preview.as_ref())
  }

  pub fn add_bot_secondary_username<C: AsRef<AddBotSecondaryUsername>>(
    &self,
    add_bot_secondary_username: C,
  ) -> RTDResult<()> {
    self.api.send(add_bot_secondary_username.as_ref())
  }

  pub fn add_chat_folder_by_invite_link<C: AsRef<AddChatFolderByInviteLink>>(
    &self,
    add_chat_folder_by_invite_link: C,
  ) -> RTDResult<()> {
    self.api.send(add_chat_folder_by_invite_link.as_ref())
  }

  pub fn add_chat_member<C: AsRef<AddChatMember>>(&self, add_chat_member: C) -> RTDResult<()> {
    self.api.send(add_chat_member.as_ref())
  }

  pub fn add_chat_members<C: AsRef<AddChatMembers>>(&self, add_chat_members: C) -> RTDResult<()> {
    self.api.send(add_chat_members.as_ref())
  }

  pub fn add_chat_to_list<C: AsRef<AddChatToList>>(&self, add_chat_to_list: C) -> RTDResult<()> {
    self.api.send(add_chat_to_list.as_ref())
  }

  pub fn add_chat_welcome_message<C: AsRef<AddChatWelcomeMessage>>(
    &self,
    add_chat_welcome_message: C,
  ) -> RTDResult<()> {
    self.api.send(add_chat_welcome_message.as_ref())
  }

  pub fn add_checklist_tasks<C: AsRef<AddChecklistTasks>>(
    &self,
    add_checklist_tasks: C,
  ) -> RTDResult<()> {
    self.api.send(add_checklist_tasks.as_ref())
  }

  pub fn add_contact<C: AsRef<AddContact>>(&self, add_contact: C) -> RTDResult<()> {
    self.api.send(add_contact.as_ref())
  }

  pub fn add_custom_server_language_pack<C: AsRef<AddCustomServerLanguagePack>>(
    &self,
    add_custom_server_language_pack: C,
  ) -> RTDResult<()> {
    self.api.send(add_custom_server_language_pack.as_ref())
  }

  pub fn add_favorite_sticker<C: AsRef<AddFavoriteSticker>>(
    &self,
    add_favorite_sticker: C,
  ) -> RTDResult<()> {
    self.api.send(add_favorite_sticker.as_ref())
  }

  pub fn add_file_to_downloads<C: AsRef<AddFileToDownloads>>(
    &self,
    add_file_to_downloads: C,
  ) -> RTDResult<()> {
    self.api.send(add_file_to_downloads.as_ref())
  }

  pub fn add_gift_collection_gifts<C: AsRef<AddGiftCollectionGifts>>(
    &self,
    add_gift_collection_gifts: C,
  ) -> RTDResult<()> {
    self.api.send(add_gift_collection_gifts.as_ref())
  }

  pub fn add_local_message<C: AsRef<AddLocalMessage>>(
    &self,
    add_local_message: C,
  ) -> RTDResult<()> {
    self.api.send(add_local_message.as_ref())
  }

  pub fn add_log_message<C: AsRef<AddLogMessage>>(&self, add_log_message: C) -> RTDResult<()> {
    self.api.send(add_log_message.as_ref())
  }

  pub fn add_login_passkey<C: AsRef<AddLoginPasskey>>(
    &self,
    add_login_passkey: C,
  ) -> RTDResult<()> {
    self.api.send(add_login_passkey.as_ref())
  }

  pub fn add_message_reaction<C: AsRef<AddMessageReaction>>(
    &self,
    add_message_reaction: C,
  ) -> RTDResult<()> {
    self.api.send(add_message_reaction.as_ref())
  }

  pub fn add_network_statistics<C: AsRef<AddNetworkStatistics>>(
    &self,
    add_network_statistics: C,
  ) -> RTDResult<()> {
    self.api.send(add_network_statistics.as_ref())
  }

  pub fn add_offer<C: AsRef<AddOffer>>(&self, add_offer: C) -> RTDResult<()> {
    self.api.send(add_offer.as_ref())
  }

  pub fn add_pending_live_story_reaction<C: AsRef<AddPendingLiveStoryReaction>>(
    &self,
    add_pending_live_story_reaction: C,
  ) -> RTDResult<()> {
    self.api.send(add_pending_live_story_reaction.as_ref())
  }

  pub fn add_pending_paid_message_reaction<C: AsRef<AddPendingPaidMessageReaction>>(
    &self,
    add_pending_paid_message_reaction: C,
  ) -> RTDResult<()> {
    self.api.send(add_pending_paid_message_reaction.as_ref())
  }

  pub fn add_poll_option<C: AsRef<AddPollOption>>(&self, add_poll_option: C) -> RTDResult<()> {
    self.api.send(add_poll_option.as_ref())
  }

  pub fn add_profile_audio<C: AsRef<AddProfileAudio>>(
    &self,
    add_profile_audio: C,
  ) -> RTDResult<()> {
    self.api.send(add_profile_audio.as_ref())
  }

  pub fn add_proxy<C: AsRef<AddProxy>>(&self, add_proxy: C) -> RTDResult<()> {
    self.api.send(add_proxy.as_ref())
  }

  pub fn add_quick_reply_shortcut_inline_query_result_message<
    C: AsRef<AddQuickReplyShortcutInlineQueryResultMessage>,
  >(
    &self,
    add_quick_reply_shortcut_inline_query_result_message: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(add_quick_reply_shortcut_inline_query_result_message.as_ref())
  }

  pub fn add_quick_reply_shortcut_message<C: AsRef<AddQuickReplyShortcutMessage>>(
    &self,
    add_quick_reply_shortcut_message: C,
  ) -> RTDResult<()> {
    self.api.send(add_quick_reply_shortcut_message.as_ref())
  }

  pub fn add_quick_reply_shortcut_message_album<C: AsRef<AddQuickReplyShortcutMessageAlbum>>(
    &self,
    add_quick_reply_shortcut_message_album: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(add_quick_reply_shortcut_message_album.as_ref())
  }

  pub fn add_recent_sticker<C: AsRef<AddRecentSticker>>(
    &self,
    add_recent_sticker: C,
  ) -> RTDResult<()> {
    self.api.send(add_recent_sticker.as_ref())
  }

  pub fn add_recently_found_chat<C: AsRef<AddRecentlyFoundChat>>(
    &self,
    add_recently_found_chat: C,
  ) -> RTDResult<()> {
    self.api.send(add_recently_found_chat.as_ref())
  }

  pub fn add_saved_animation<C: AsRef<AddSavedAnimation>>(
    &self,
    add_saved_animation: C,
  ) -> RTDResult<()> {
    self.api.send(add_saved_animation.as_ref())
  }

  pub fn add_saved_notification_sound<C: AsRef<AddSavedNotificationSound>>(
    &self,
    add_saved_notification_sound: C,
  ) -> RTDResult<()> {
    self.api.send(add_saved_notification_sound.as_ref())
  }

  pub fn add_sticker_to_set<C: AsRef<AddStickerToSet>>(
    &self,
    add_sticker_to_set: C,
  ) -> RTDResult<()> {
    self.api.send(add_sticker_to_set.as_ref())
  }

  pub fn add_story_album_stories<C: AsRef<AddStoryAlbumStories>>(
    &self,
    add_story_album_stories: C,
  ) -> RTDResult<()> {
    self.api.send(add_story_album_stories.as_ref())
  }

  pub fn add_text_composition_style<C: AsRef<AddTextCompositionStyle>>(
    &self,
    add_text_composition_style: C,
  ) -> RTDResult<()> {
    self.api.send(add_text_composition_style.as_ref())
  }

  pub fn add_web_browser_settings_exception<C: AsRef<AddWebBrowserSettingsException>>(
    &self,
    add_web_browser_settings_exception: C,
  ) -> RTDResult<()> {
    self.api.send(add_web_browser_settings_exception.as_ref())
  }

  pub fn allow_bot_to_send_messages<C: AsRef<AllowBotToSendMessages>>(
    &self,
    allow_bot_to_send_messages: C,
  ) -> RTDResult<()> {
    self.api.send(allow_bot_to_send_messages.as_ref())
  }

  pub fn allow_unpaid_messages_from_user<C: AsRef<AllowUnpaidMessagesFromUser>>(
    &self,
    allow_unpaid_messages_from_user: C,
  ) -> RTDResult<()> {
    self.api.send(allow_unpaid_messages_from_user.as_ref())
  }

  pub fn answer_callback_query<C: AsRef<AnswerCallbackQuery>>(
    &self,
    answer_callback_query: C,
  ) -> RTDResult<()> {
    self.api.send(answer_callback_query.as_ref())
  }

  pub fn answer_chat_join_request_query<C: AsRef<AnswerChatJoinRequestQuery>>(
    &self,
    answer_chat_join_request_query: C,
  ) -> RTDResult<()> {
    self.api.send(answer_chat_join_request_query.as_ref())
  }

  pub fn answer_custom_query<C: AsRef<AnswerCustomQuery>>(
    &self,
    answer_custom_query: C,
  ) -> RTDResult<()> {
    self.api.send(answer_custom_query.as_ref())
  }

  pub fn answer_guest_query<C: AsRef<AnswerGuestQuery>>(
    &self,
    answer_guest_query: C,
  ) -> RTDResult<()> {
    self.api.send(answer_guest_query.as_ref())
  }

  pub fn answer_inline_query<C: AsRef<AnswerInlineQuery>>(
    &self,
    answer_inline_query: C,
  ) -> RTDResult<()> {
    self.api.send(answer_inline_query.as_ref())
  }

  pub fn answer_pre_checkout_query<C: AsRef<AnswerPreCheckoutQuery>>(
    &self,
    answer_pre_checkout_query: C,
  ) -> RTDResult<()> {
    self.api.send(answer_pre_checkout_query.as_ref())
  }

  pub fn answer_shipping_query<C: AsRef<AnswerShippingQuery>>(
    &self,
    answer_shipping_query: C,
  ) -> RTDResult<()> {
    self.api.send(answer_shipping_query.as_ref())
  }

  pub fn answer_ton_connect_request<C: AsRef<AnswerTonConnectRequest>>(
    &self,
    answer_ton_connect_request: C,
  ) -> RTDResult<()> {
    self.api.send(answer_ton_connect_request.as_ref())
  }

  pub fn answer_web_app_query<C: AsRef<AnswerWebAppQuery>>(
    &self,
    answer_web_app_query: C,
  ) -> RTDResult<()> {
    self.api.send(answer_web_app_query.as_ref())
  }

  pub fn apply_premium_gift_code<C: AsRef<ApplyPremiumGiftCode>>(
    &self,
    apply_premium_gift_code: C,
  ) -> RTDResult<()> {
    self.api.send(apply_premium_gift_code.as_ref())
  }

  pub fn apply_unpaid_messages_token<C: AsRef<ApplyUnpaidMessagesToken>>(
    &self,
    apply_unpaid_messages_token: C,
  ) -> RTDResult<()> {
    self.api.send(apply_unpaid_messages_token.as_ref())
  }

  pub fn approve_suggested_post<C: AsRef<ApproveSuggestedPost>>(
    &self,
    approve_suggested_post: C,
  ) -> RTDResult<()> {
    self.api.send(approve_suggested_post.as_ref())
  }

  pub fn assign_store_transaction<C: AsRef<AssignStoreTransaction>>(
    &self,
    assign_store_transaction: C,
  ) -> RTDResult<()> {
    self.api.send(assign_store_transaction.as_ref())
  }

  pub fn ban_chat_member<C: AsRef<BanChatMember>>(&self, ban_chat_member: C) -> RTDResult<()> {
    self.api.send(ban_chat_member.as_ref())
  }

  pub fn ban_group_call_participants<C: AsRef<BanGroupCallParticipants>>(
    &self,
    ban_group_call_participants: C,
  ) -> RTDResult<()> {
    self.api.send(ban_group_call_participants.as_ref())
  }

  pub fn block_message_sender_from_replies<C: AsRef<BlockMessageSenderFromReplies>>(
    &self,
    block_message_sender_from_replies: C,
  ) -> RTDResult<()> {
    self.api.send(block_message_sender_from_replies.as_ref())
  }

  pub fn boost_chat<C: AsRef<BoostChat>>(&self, boost_chat: C) -> RTDResult<()> {
    self.api.send(boost_chat.as_ref())
  }

  pub fn buy_gift_upgrade<C: AsRef<BuyGiftUpgrade>>(&self, buy_gift_upgrade: C) -> RTDResult<()> {
    self.api.send(buy_gift_upgrade.as_ref())
  }

  pub fn can_bot_send_messages<C: AsRef<CanBotSendMessages>>(
    &self,
    can_bot_send_messages: C,
  ) -> RTDResult<()> {
    self.api.send(can_bot_send_messages.as_ref())
  }

  pub fn can_post_story<C: AsRef<CanPostStory>>(&self, can_post_story: C) -> RTDResult<()> {
    self.api.send(can_post_story.as_ref())
  }

  pub fn can_purchase_from_store<C: AsRef<CanPurchaseFromStore>>(
    &self,
    can_purchase_from_store: C,
  ) -> RTDResult<()> {
    self.api.send(can_purchase_from_store.as_ref())
  }

  pub fn can_send_gift<C: AsRef<CanSendGift>>(&self, can_send_gift: C) -> RTDResult<()> {
    self.api.send(can_send_gift.as_ref())
  }

  pub fn can_send_message_to_user<C: AsRef<CanSendMessageToUser>>(
    &self,
    can_send_message_to_user: C,
  ) -> RTDResult<()> {
    self.api.send(can_send_message_to_user.as_ref())
  }

  pub fn can_transfer_ownership<C: AsRef<CanTransferOwnership>>(
    &self,
    can_transfer_ownership: C,
  ) -> RTDResult<()> {
    self.api.send(can_transfer_ownership.as_ref())
  }

  pub fn cancel_download_file<C: AsRef<CancelDownloadFile>>(
    &self,
    cancel_download_file: C,
  ) -> RTDResult<()> {
    self.api.send(cancel_download_file.as_ref())
  }

  pub fn cancel_password_reset<C: AsRef<CancelPasswordReset>>(
    &self,
    cancel_password_reset: C,
  ) -> RTDResult<()> {
    self.api.send(cancel_password_reset.as_ref())
  }

  pub fn cancel_preliminary_upload_file<C: AsRef<CancelPreliminaryUploadFile>>(
    &self,
    cancel_preliminary_upload_file: C,
  ) -> RTDResult<()> {
    self.api.send(cancel_preliminary_upload_file.as_ref())
  }

  pub fn cancel_recovery_email_address_verification<
    C: AsRef<CancelRecoveryEmailAddressVerification>,
  >(
    &self,
    cancel_recovery_email_address_verification: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(cancel_recovery_email_address_verification.as_ref())
  }

  pub fn change_imported_contacts<C: AsRef<ChangeImportedContacts>>(
    &self,
    change_imported_contacts: C,
  ) -> RTDResult<()> {
    self.api.send(change_imported_contacts.as_ref())
  }

  pub fn change_sticker_set<C: AsRef<ChangeStickerSet>>(
    &self,
    change_sticker_set: C,
  ) -> RTDResult<()> {
    self.api.send(change_sticker_set.as_ref())
  }

  pub fn change_web_browser_settings<C: AsRef<ChangeWebBrowserSettings>>(
    &self,
    change_web_browser_settings: C,
  ) -> RTDResult<()> {
    self.api.send(change_web_browser_settings.as_ref())
  }

  pub fn check_authentication_bot_token<C: AsRef<CheckAuthenticationBotToken>>(
    &self,
    check_authentication_bot_token: C,
  ) -> RTDResult<()> {
    self.api.send(check_authentication_bot_token.as_ref())
  }

  pub fn check_authentication_code<C: AsRef<CheckAuthenticationCode>>(
    &self,
    check_authentication_code: C,
  ) -> RTDResult<()> {
    self.api.send(check_authentication_code.as_ref())
  }

  pub fn check_authentication_email_code<C: AsRef<CheckAuthenticationEmailCode>>(
    &self,
    check_authentication_email_code: C,
  ) -> RTDResult<()> {
    self.api.send(check_authentication_email_code.as_ref())
  }

  pub fn check_authentication_passkey<C: AsRef<CheckAuthenticationPasskey>>(
    &self,
    check_authentication_passkey: C,
  ) -> RTDResult<()> {
    self.api.send(check_authentication_passkey.as_ref())
  }

  pub fn check_authentication_password<C: AsRef<CheckAuthenticationPassword>>(
    &self,
    check_authentication_password: C,
  ) -> RTDResult<()> {
    self.api.send(check_authentication_password.as_ref())
  }

  pub fn check_authentication_password_recovery_code<
    C: AsRef<CheckAuthenticationPasswordRecoveryCode>,
  >(
    &self,
    check_authentication_password_recovery_code: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(check_authentication_password_recovery_code.as_ref())
  }

  pub fn check_authentication_premium_purchase<C: AsRef<CheckAuthenticationPremiumPurchase>>(
    &self,
    check_authentication_premium_purchase: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(check_authentication_premium_purchase.as_ref())
  }

  pub fn check_authentication_web_token<C: AsRef<CheckAuthenticationWebToken>>(
    &self,
    check_authentication_web_token: C,
  ) -> RTDResult<()> {
    self.api.send(check_authentication_web_token.as_ref())
  }

  pub fn check_bot_username<C: AsRef<CheckBotUsername>>(
    &self,
    check_bot_username: C,
  ) -> RTDResult<()> {
    self.api.send(check_bot_username.as_ref())
  }

  pub fn check_chat_folder_invite_link<C: AsRef<CheckChatFolderInviteLink>>(
    &self,
    check_chat_folder_invite_link: C,
  ) -> RTDResult<()> {
    self.api.send(check_chat_folder_invite_link.as_ref())
  }

  pub fn check_chat_invite_link<C: AsRef<CheckChatInviteLink>>(
    &self,
    check_chat_invite_link: C,
  ) -> RTDResult<()> {
    self.api.send(check_chat_invite_link.as_ref())
  }

  pub fn check_chat_username<C: AsRef<CheckChatUsername>>(
    &self,
    check_chat_username: C,
  ) -> RTDResult<()> {
    self.api.send(check_chat_username.as_ref())
  }

  pub fn check_created_public_chats_limit<C: AsRef<CheckCreatedPublicChatsLimit>>(
    &self,
    check_created_public_chats_limit: C,
  ) -> RTDResult<()> {
    self.api.send(check_created_public_chats_limit.as_ref())
  }

  pub fn check_email_address_verification_code<C: AsRef<CheckEmailAddressVerificationCode>>(
    &self,
    check_email_address_verification_code: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(check_email_address_verification_code.as_ref())
  }

  pub fn check_login_email_address_code<C: AsRef<CheckLoginEmailAddressCode>>(
    &self,
    check_login_email_address_code: C,
  ) -> RTDResult<()> {
    self.api.send(check_login_email_address_code.as_ref())
  }

  pub fn check_oauth_request_match_code<C: AsRef<CheckOauthRequestMatchCode>>(
    &self,
    check_oauth_request_match_code: C,
  ) -> RTDResult<()> {
    self.api.send(check_oauth_request_match_code.as_ref())
  }

  pub fn check_password_recovery_code<C: AsRef<CheckPasswordRecoveryCode>>(
    &self,
    check_password_recovery_code: C,
  ) -> RTDResult<()> {
    self.api.send(check_password_recovery_code.as_ref())
  }

  pub fn check_phone_number_code<C: AsRef<CheckPhoneNumberCode>>(
    &self,
    check_phone_number_code: C,
  ) -> RTDResult<()> {
    self.api.send(check_phone_number_code.as_ref())
  }

  pub fn check_premium_gift_code<C: AsRef<CheckPremiumGiftCode>>(
    &self,
    check_premium_gift_code: C,
  ) -> RTDResult<()> {
    self.api.send(check_premium_gift_code.as_ref())
  }

  pub fn check_quick_reply_shortcut_name<C: AsRef<CheckQuickReplyShortcutName>>(
    &self,
    check_quick_reply_shortcut_name: C,
  ) -> RTDResult<()> {
    self.api.send(check_quick_reply_shortcut_name.as_ref())
  }

  pub fn check_recovery_email_address_code<C: AsRef<CheckRecoveryEmailAddressCode>>(
    &self,
    check_recovery_email_address_code: C,
  ) -> RTDResult<()> {
    self.api.send(check_recovery_email_address_code.as_ref())
  }

  pub fn check_sticker_set_name<C: AsRef<CheckStickerSetName>>(
    &self,
    check_sticker_set_name: C,
  ) -> RTDResult<()> {
    self.api.send(check_sticker_set_name.as_ref())
  }

  pub fn check_wallet_bot_balance<C: AsRef<CheckWalletBotBalance>>(
    &self,
    check_wallet_bot_balance: C,
  ) -> RTDResult<()> {
    self.api.send(check_wallet_bot_balance.as_ref())
  }

  pub fn check_web_app_file_download<C: AsRef<CheckWebAppFileDownload>>(
    &self,
    check_web_app_file_download: C,
  ) -> RTDResult<()> {
    self.api.send(check_web_app_file_download.as_ref())
  }

  pub fn claim_ton_connect_request<C: AsRef<ClaimTonConnectRequest>>(
    &self,
    claim_ton_connect_request: C,
  ) -> RTDResult<()> {
    self.api.send(claim_ton_connect_request.as_ref())
  }

  pub fn clean_file_name<C: AsRef<CleanFileName>>(&self, clean_file_name: C) -> RTDResult<()> {
    self.api.send(clean_file_name.as_ref())
  }

  pub fn clear_all_draft_messages<C: AsRef<ClearAllDraftMessages>>(
    &self,
    clear_all_draft_messages: C,
  ) -> RTDResult<()> {
    self.api.send(clear_all_draft_messages.as_ref())
  }

  pub fn clear_autosave_settings_exceptions<C: AsRef<ClearAutosaveSettingsExceptions>>(
    &self,
    clear_autosave_settings_exceptions: C,
  ) -> RTDResult<()> {
    self.api.send(clear_autosave_settings_exceptions.as_ref())
  }

  pub fn clear_imported_contacts<C: AsRef<ClearImportedContacts>>(
    &self,
    clear_imported_contacts: C,
  ) -> RTDResult<()> {
    self.api.send(clear_imported_contacts.as_ref())
  }

  pub fn clear_recent_emoji_statuses<C: AsRef<ClearRecentEmojiStatuses>>(
    &self,
    clear_recent_emoji_statuses: C,
  ) -> RTDResult<()> {
    self.api.send(clear_recent_emoji_statuses.as_ref())
  }

  pub fn clear_recent_reactions<C: AsRef<ClearRecentReactions>>(
    &self,
    clear_recent_reactions: C,
  ) -> RTDResult<()> {
    self.api.send(clear_recent_reactions.as_ref())
  }

  pub fn clear_recent_stickers<C: AsRef<ClearRecentStickers>>(
    &self,
    clear_recent_stickers: C,
  ) -> RTDResult<()> {
    self.api.send(clear_recent_stickers.as_ref())
  }

  pub fn clear_recently_found_chats<C: AsRef<ClearRecentlyFoundChats>>(
    &self,
    clear_recently_found_chats: C,
  ) -> RTDResult<()> {
    self.api.send(clear_recently_found_chats.as_ref())
  }

  pub fn clear_searched_for_tags<C: AsRef<ClearSearchedForTags>>(
    &self,
    clear_searched_for_tags: C,
  ) -> RTDResult<()> {
    self.api.send(clear_searched_for_tags.as_ref())
  }

  pub fn click_animated_emoji_message<C: AsRef<ClickAnimatedEmojiMessage>>(
    &self,
    click_animated_emoji_message: C,
  ) -> RTDResult<()> {
    self.api.send(click_animated_emoji_message.as_ref())
  }

  pub fn click_chat_sponsored_message<C: AsRef<ClickChatSponsoredMessage>>(
    &self,
    click_chat_sponsored_message: C,
  ) -> RTDResult<()> {
    self.api.send(click_chat_sponsored_message.as_ref())
  }

  pub fn click_premium_subscription_button<C: AsRef<ClickPremiumSubscriptionButton>>(
    &self,
    click_premium_subscription_button: C,
  ) -> RTDResult<()> {
    self.api.send(click_premium_subscription_button.as_ref())
  }

  pub fn click_video_message_advertisement<C: AsRef<ClickVideoMessageAdvertisement>>(
    &self,
    click_video_message_advertisement: C,
  ) -> RTDResult<()> {
    self.api.send(click_video_message_advertisement.as_ref())
  }

  pub fn close<C: AsRef<Close>>(&self, close: C) -> RTDResult<()> {
    self.api.send(close.as_ref())
  }

  pub fn close_chat<C: AsRef<CloseChat>>(&self, close_chat: C) -> RTDResult<()> {
    self.api.send(close_chat.as_ref())
  }

  pub fn close_gift_auction<C: AsRef<CloseGiftAuction>>(
    &self,
    close_gift_auction: C,
  ) -> RTDResult<()> {
    self.api.send(close_gift_auction.as_ref())
  }

  pub fn close_secret_chat<C: AsRef<CloseSecretChat>>(
    &self,
    close_secret_chat: C,
  ) -> RTDResult<()> {
    self.api.send(close_secret_chat.as_ref())
  }

  pub fn close_story<C: AsRef<CloseStory>>(&self, close_story: C) -> RTDResult<()> {
    self.api.send(close_story.as_ref())
  }

  pub fn close_web_app<C: AsRef<CloseWebApp>>(&self, close_web_app: C) -> RTDResult<()> {
    self.api.send(close_web_app.as_ref())
  }

  pub fn commit_pending_live_story_reactions<C: AsRef<CommitPendingLiveStoryReactions>>(
    &self,
    commit_pending_live_story_reactions: C,
  ) -> RTDResult<()> {
    self.api.send(commit_pending_live_story_reactions.as_ref())
  }

  pub fn commit_pending_paid_message_reactions<C: AsRef<CommitPendingPaidMessageReactions>>(
    &self,
    commit_pending_paid_message_reactions: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(commit_pending_paid_message_reactions.as_ref())
  }

  pub fn compose_rich_message_with_ai<C: AsRef<ComposeRichMessageWithAi>>(
    &self,
    compose_rich_message_with_ai: C,
  ) -> RTDResult<()> {
    self.api.send(compose_rich_message_with_ai.as_ref())
  }

  pub fn compose_text_with_ai<C: AsRef<ComposeTextWithAi>>(
    &self,
    compose_text_with_ai: C,
  ) -> RTDResult<()> {
    self.api.send(compose_text_with_ai.as_ref())
  }

  pub fn confirm_business_connected_bot<C: AsRef<ConfirmBusinessConnectedBot>>(
    &self,
    confirm_business_connected_bot: C,
  ) -> RTDResult<()> {
    self.api.send(confirm_business_connected_bot.as_ref())
  }

  pub fn confirm_qr_code_authentication<C: AsRef<ConfirmQrCodeAuthentication>>(
    &self,
    confirm_qr_code_authentication: C,
  ) -> RTDResult<()> {
    self.api.send(confirm_qr_code_authentication.as_ref())
  }

  pub fn confirm_session<C: AsRef<ConfirmSession>>(&self, confirm_session: C) -> RTDResult<()> {
    self.api.send(confirm_session.as_ref())
  }

  pub fn connect_affiliate_program<C: AsRef<ConnectAffiliateProgram>>(
    &self,
    connect_affiliate_program: C,
  ) -> RTDResult<()> {
    self.api.send(connect_affiliate_program.as_ref())
  }

  pub fn craft_gift<C: AsRef<CraftGift>>(&self, craft_gift: C) -> RTDResult<()> {
    self.api.send(craft_gift.as_ref())
  }

  pub fn create_basic_group_chat<C: AsRef<CreateBasicGroupChat>>(
    &self,
    create_basic_group_chat: C,
  ) -> RTDResult<()> {
    self.api.send(create_basic_group_chat.as_ref())
  }

  pub fn create_bot<C: AsRef<CreateBot>>(&self, create_bot: C) -> RTDResult<()> {
    self.api.send(create_bot.as_ref())
  }

  pub fn create_business_chat_link<C: AsRef<CreateBusinessChatLink>>(
    &self,
    create_business_chat_link: C,
  ) -> RTDResult<()> {
    self.api.send(create_business_chat_link.as_ref())
  }

  pub fn create_call<C: AsRef<CreateCall>>(&self, create_call: C) -> RTDResult<()> {
    self.api.send(create_call.as_ref())
  }

  pub fn create_chat_folder<C: AsRef<CreateChatFolder>>(
    &self,
    create_chat_folder: C,
  ) -> RTDResult<()> {
    self.api.send(create_chat_folder.as_ref())
  }

  pub fn create_chat_folder_invite_link<C: AsRef<CreateChatFolderInviteLink>>(
    &self,
    create_chat_folder_invite_link: C,
  ) -> RTDResult<()> {
    self.api.send(create_chat_folder_invite_link.as_ref())
  }

  pub fn create_chat_invite_link<C: AsRef<CreateChatInviteLink>>(
    &self,
    create_chat_invite_link: C,
  ) -> RTDResult<()> {
    self.api.send(create_chat_invite_link.as_ref())
  }

  pub fn create_chat_subscription_invite_link<C: AsRef<CreateChatSubscriptionInviteLink>>(
    &self,
    create_chat_subscription_invite_link: C,
  ) -> RTDResult<()> {
    self.api.send(create_chat_subscription_invite_link.as_ref())
  }

  pub fn create_community<C: AsRef<CreateCommunity>>(&self, create_community: C) -> RTDResult<()> {
    self.api.send(create_community.as_ref())
  }

  pub fn create_forum_topic<C: AsRef<CreateForumTopic>>(
    &self,
    create_forum_topic: C,
  ) -> RTDResult<()> {
    self.api.send(create_forum_topic.as_ref())
  }

  pub fn create_gift_collection<C: AsRef<CreateGiftCollection>>(
    &self,
    create_gift_collection: C,
  ) -> RTDResult<()> {
    self.api.send(create_gift_collection.as_ref())
  }

  pub fn create_group_call<C: AsRef<CreateGroupCall>>(
    &self,
    create_group_call: C,
  ) -> RTDResult<()> {
    self.api.send(create_group_call.as_ref())
  }

  pub fn create_invoice_link<C: AsRef<CreateInvoiceLink>>(
    &self,
    create_invoice_link: C,
  ) -> RTDResult<()> {
    self.api.send(create_invoice_link.as_ref())
  }

  pub fn create_new_basic_group_chat<C: AsRef<CreateNewBasicGroupChat>>(
    &self,
    create_new_basic_group_chat: C,
  ) -> RTDResult<()> {
    self.api.send(create_new_basic_group_chat.as_ref())
  }

  pub fn create_new_secret_chat<C: AsRef<CreateNewSecretChat>>(
    &self,
    create_new_secret_chat: C,
  ) -> RTDResult<()> {
    self.api.send(create_new_secret_chat.as_ref())
  }

  pub fn create_new_sticker_set<C: AsRef<CreateNewStickerSet>>(
    &self,
    create_new_sticker_set: C,
  ) -> RTDResult<()> {
    self.api.send(create_new_sticker_set.as_ref())
  }

  pub fn create_new_supergroup_chat<C: AsRef<CreateNewSupergroupChat>>(
    &self,
    create_new_supergroup_chat: C,
  ) -> RTDResult<()> {
    self.api.send(create_new_supergroup_chat.as_ref())
  }

  pub fn create_on_ramp_payment_session<C: AsRef<CreateOnRampPaymentSession>>(
    &self,
    create_on_ramp_payment_session: C,
  ) -> RTDResult<()> {
    self.api.send(create_on_ramp_payment_session.as_ref())
  }

  pub fn create_private_chat<C: AsRef<CreatePrivateChat>>(
    &self,
    create_private_chat: C,
  ) -> RTDResult<()> {
    self.api.send(create_private_chat.as_ref())
  }

  pub fn create_rich_message_with_ai<C: AsRef<CreateRichMessageWithAi>>(
    &self,
    create_rich_message_with_ai: C,
  ) -> RTDResult<()> {
    self.api.send(create_rich_message_with_ai.as_ref())
  }

  pub fn create_secret_chat<C: AsRef<CreateSecretChat>>(
    &self,
    create_secret_chat: C,
  ) -> RTDResult<()> {
    self.api.send(create_secret_chat.as_ref())
  }

  pub fn create_story_album<C: AsRef<CreateStoryAlbum>>(
    &self,
    create_story_album: C,
  ) -> RTDResult<()> {
    self.api.send(create_story_album.as_ref())
  }

  pub fn create_supergroup_chat<C: AsRef<CreateSupergroupChat>>(
    &self,
    create_supergroup_chat: C,
  ) -> RTDResult<()> {
    self.api.send(create_supergroup_chat.as_ref())
  }

  pub fn create_temporary_password<C: AsRef<CreateTemporaryPassword>>(
    &self,
    create_temporary_password: C,
  ) -> RTDResult<()> {
    self.api.send(create_temporary_password.as_ref())
  }

  pub fn create_text_composition_style<C: AsRef<CreateTextCompositionStyle>>(
    &self,
    create_text_composition_style: C,
  ) -> RTDResult<()> {
    self.api.send(create_text_composition_style.as_ref())
  }

  pub fn create_ton_connect_session<C: AsRef<CreateTonConnectSession>>(
    &self,
    create_ton_connect_session: C,
  ) -> RTDResult<()> {
    self.api.send(create_ton_connect_session.as_ref())
  }

  pub fn create_user_ton_wallet<C: AsRef<CreateUserTonWallet>>(
    &self,
    create_user_ton_wallet: C,
  ) -> RTDResult<()> {
    self.api.send(create_user_ton_wallet.as_ref())
  }

  pub fn create_video_chat<C: AsRef<CreateVideoChat>>(
    &self,
    create_video_chat: C,
  ) -> RTDResult<()> {
    self.api.send(create_video_chat.as_ref())
  }

  pub fn decline_group_call_invitation<C: AsRef<DeclineGroupCallInvitation>>(
    &self,
    decline_group_call_invitation: C,
  ) -> RTDResult<()> {
    self.api.send(decline_group_call_invitation.as_ref())
  }

  pub fn decline_oauth_request<C: AsRef<DeclineOauthRequest>>(
    &self,
    decline_oauth_request: C,
  ) -> RTDResult<()> {
    self.api.send(decline_oauth_request.as_ref())
  }

  pub fn decline_suggested_post<C: AsRef<DeclineSuggestedPost>>(
    &self,
    decline_suggested_post: C,
  ) -> RTDResult<()> {
    self.api.send(decline_suggested_post.as_ref())
  }

  pub fn decrypt_group_call_data<C: AsRef<DecryptGroupCallData>>(
    &self,
    decrypt_group_call_data: C,
  ) -> RTDResult<()> {
    self.api.send(decrypt_group_call_data.as_ref())
  }

  pub fn delete_account<C: AsRef<DeleteAccount>>(&self, delete_account: C) -> RTDResult<()> {
    self.api.send(delete_account.as_ref())
  }

  pub fn delete_all_call_messages<C: AsRef<DeleteAllCallMessages>>(
    &self,
    delete_all_call_messages: C,
  ) -> RTDResult<()> {
    self.api.send(delete_all_call_messages.as_ref())
  }

  pub fn delete_all_chat_welcome_messages<C: AsRef<DeleteAllChatWelcomeMessages>>(
    &self,
    delete_all_chat_welcome_messages: C,
  ) -> RTDResult<()> {
    self.api.send(delete_all_chat_welcome_messages.as_ref())
  }

  pub fn delete_all_recent_message_reactions_from_sender<
    C: AsRef<DeleteAllRecentMessageReactionsFromSender>,
  >(
    &self,
    delete_all_recent_message_reactions_from_sender: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(delete_all_recent_message_reactions_from_sender.as_ref())
  }

  pub fn delete_all_revoked_chat_invite_links<C: AsRef<DeleteAllRevokedChatInviteLinks>>(
    &self,
    delete_all_revoked_chat_invite_links: C,
  ) -> RTDResult<()> {
    self.api.send(delete_all_revoked_chat_invite_links.as_ref())
  }

  pub fn delete_bot_media_previews<C: AsRef<DeleteBotMediaPreviews>>(
    &self,
    delete_bot_media_previews: C,
  ) -> RTDResult<()> {
    self.api.send(delete_bot_media_previews.as_ref())
  }

  pub fn delete_bot_secondary_username<C: AsRef<DeleteBotSecondaryUsername>>(
    &self,
    delete_bot_secondary_username: C,
  ) -> RTDResult<()> {
    self.api.send(delete_bot_secondary_username.as_ref())
  }

  pub fn delete_business_chat_link<C: AsRef<DeleteBusinessChatLink>>(
    &self,
    delete_business_chat_link: C,
  ) -> RTDResult<()> {
    self.api.send(delete_business_chat_link.as_ref())
  }

  pub fn delete_business_connected_bot<C: AsRef<DeleteBusinessConnectedBot>>(
    &self,
    delete_business_connected_bot: C,
  ) -> RTDResult<()> {
    self.api.send(delete_business_connected_bot.as_ref())
  }

  pub fn delete_business_messages<C: AsRef<DeleteBusinessMessages>>(
    &self,
    delete_business_messages: C,
  ) -> RTDResult<()> {
    self.api.send(delete_business_messages.as_ref())
  }

  pub fn delete_business_story<C: AsRef<DeleteBusinessStory>>(
    &self,
    delete_business_story: C,
  ) -> RTDResult<()> {
    self.api.send(delete_business_story.as_ref())
  }

  pub fn delete_chat<C: AsRef<DeleteChat>>(&self, delete_chat: C) -> RTDResult<()> {
    self.api.send(delete_chat.as_ref())
  }

  pub fn delete_chat_background<C: AsRef<DeleteChatBackground>>(
    &self,
    delete_chat_background: C,
  ) -> RTDResult<()> {
    self.api.send(delete_chat_background.as_ref())
  }

  pub fn delete_chat_folder<C: AsRef<DeleteChatFolder>>(
    &self,
    delete_chat_folder: C,
  ) -> RTDResult<()> {
    self.api.send(delete_chat_folder.as_ref())
  }

  pub fn delete_chat_folder_invite_link<C: AsRef<DeleteChatFolderInviteLink>>(
    &self,
    delete_chat_folder_invite_link: C,
  ) -> RTDResult<()> {
    self.api.send(delete_chat_folder_invite_link.as_ref())
  }

  pub fn delete_chat_history<C: AsRef<DeleteChatHistory>>(
    &self,
    delete_chat_history: C,
  ) -> RTDResult<()> {
    self.api.send(delete_chat_history.as_ref())
  }

  pub fn delete_chat_messages_by_date<C: AsRef<DeleteChatMessagesByDate>>(
    &self,
    delete_chat_messages_by_date: C,
  ) -> RTDResult<()> {
    self.api.send(delete_chat_messages_by_date.as_ref())
  }

  pub fn delete_chat_messages_by_sender<C: AsRef<DeleteChatMessagesBySender>>(
    &self,
    delete_chat_messages_by_sender: C,
  ) -> RTDResult<()> {
    self.api.send(delete_chat_messages_by_sender.as_ref())
  }

  pub fn delete_chat_reply_markup<C: AsRef<DeleteChatReplyMarkup>>(
    &self,
    delete_chat_reply_markup: C,
  ) -> RTDResult<()> {
    self.api.send(delete_chat_reply_markup.as_ref())
  }

  pub fn delete_chat_welcome_message<C: AsRef<DeleteChatWelcomeMessage>>(
    &self,
    delete_chat_welcome_message: C,
  ) -> RTDResult<()> {
    self.api.send(delete_chat_welcome_message.as_ref())
  }

  pub fn delete_commands<C: AsRef<DeleteCommands>>(&self, delete_commands: C) -> RTDResult<()> {
    self.api.send(delete_commands.as_ref())
  }

  pub fn delete_community<C: AsRef<DeleteCommunity>>(&self, delete_community: C) -> RTDResult<()> {
    self.api.send(delete_community.as_ref())
  }

  pub fn delete_default_background<C: AsRef<DeleteDefaultBackground>>(
    &self,
    delete_default_background: C,
  ) -> RTDResult<()> {
    self.api.send(delete_default_background.as_ref())
  }

  pub fn delete_direct_messages_chat_topic_history<
    C: AsRef<DeleteDirectMessagesChatTopicHistory>,
  >(
    &self,
    delete_direct_messages_chat_topic_history: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(delete_direct_messages_chat_topic_history.as_ref())
  }

  pub fn delete_direct_messages_chat_topic_messages_by_date<
    C: AsRef<DeleteDirectMessagesChatTopicMessagesByDate>,
  >(
    &self,
    delete_direct_messages_chat_topic_messages_by_date: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(delete_direct_messages_chat_topic_messages_by_date.as_ref())
  }

  pub fn delete_ephemeral_message<C: AsRef<DeleteEphemeralMessage>>(
    &self,
    delete_ephemeral_message: C,
  ) -> RTDResult<()> {
    self.api.send(delete_ephemeral_message.as_ref())
  }

  pub fn delete_file<C: AsRef<DeleteFile>>(&self, delete_file: C) -> RTDResult<()> {
    self.api.send(delete_file.as_ref())
  }

  pub fn delete_forum_topic<C: AsRef<DeleteForumTopic>>(
    &self,
    delete_forum_topic: C,
  ) -> RTDResult<()> {
    self.api.send(delete_forum_topic.as_ref())
  }

  pub fn delete_gift<C: AsRef<DeleteGift>>(&self, delete_gift: C) -> RTDResult<()> {
    self.api.send(delete_gift.as_ref())
  }

  pub fn delete_gift_collection<C: AsRef<DeleteGiftCollection>>(
    &self,
    delete_gift_collection: C,
  ) -> RTDResult<()> {
    self.api.send(delete_gift_collection.as_ref())
  }

  pub fn delete_group_call_messages<C: AsRef<DeleteGroupCallMessages>>(
    &self,
    delete_group_call_messages: C,
  ) -> RTDResult<()> {
    self.api.send(delete_group_call_messages.as_ref())
  }

  pub fn delete_group_call_messages_by_sender<C: AsRef<DeleteGroupCallMessagesBySender>>(
    &self,
    delete_group_call_messages_by_sender: C,
  ) -> RTDResult<()> {
    self.api.send(delete_group_call_messages_by_sender.as_ref())
  }

  pub fn delete_language_pack<C: AsRef<DeleteLanguagePack>>(
    &self,
    delete_language_pack: C,
  ) -> RTDResult<()> {
    self.api.send(delete_language_pack.as_ref())
  }

  pub fn delete_message_ephemeral_content<C: AsRef<DeleteMessageEphemeralContent>>(
    &self,
    delete_message_ephemeral_content: C,
  ) -> RTDResult<()> {
    self.api.send(delete_message_ephemeral_content.as_ref())
  }

  pub fn delete_message_reactions_from_sender<C: AsRef<DeleteMessageReactionsFromSender>>(
    &self,
    delete_message_reactions_from_sender: C,
  ) -> RTDResult<()> {
    self.api.send(delete_message_reactions_from_sender.as_ref())
  }

  pub fn delete_messages<C: AsRef<DeleteMessages>>(&self, delete_messages: C) -> RTDResult<()> {
    self.api.send(delete_messages.as_ref())
  }

  pub fn delete_passport_element<C: AsRef<DeletePassportElement>>(
    &self,
    delete_passport_element: C,
  ) -> RTDResult<()> {
    self.api.send(delete_passport_element.as_ref())
  }

  pub fn delete_poll_option<C: AsRef<DeletePollOption>>(
    &self,
    delete_poll_option: C,
  ) -> RTDResult<()> {
    self.api.send(delete_poll_option.as_ref())
  }

  pub fn delete_profile_photo<C: AsRef<DeleteProfilePhoto>>(
    &self,
    delete_profile_photo: C,
  ) -> RTDResult<()> {
    self.api.send(delete_profile_photo.as_ref())
  }

  pub fn delete_quick_reply_shortcut<C: AsRef<DeleteQuickReplyShortcut>>(
    &self,
    delete_quick_reply_shortcut: C,
  ) -> RTDResult<()> {
    self.api.send(delete_quick_reply_shortcut.as_ref())
  }

  pub fn delete_quick_reply_shortcut_messages<C: AsRef<DeleteQuickReplyShortcutMessages>>(
    &self,
    delete_quick_reply_shortcut_messages: C,
  ) -> RTDResult<()> {
    self.api.send(delete_quick_reply_shortcut_messages.as_ref())
  }

  pub fn delete_revoked_chat_invite_link<C: AsRef<DeleteRevokedChatInviteLink>>(
    &self,
    delete_revoked_chat_invite_link: C,
  ) -> RTDResult<()> {
    self.api.send(delete_revoked_chat_invite_link.as_ref())
  }

  pub fn delete_saved_credentials<C: AsRef<DeleteSavedCredentials>>(
    &self,
    delete_saved_credentials: C,
  ) -> RTDResult<()> {
    self.api.send(delete_saved_credentials.as_ref())
  }

  pub fn delete_saved_messages_topic_history<C: AsRef<DeleteSavedMessagesTopicHistory>>(
    &self,
    delete_saved_messages_topic_history: C,
  ) -> RTDResult<()> {
    self.api.send(delete_saved_messages_topic_history.as_ref())
  }

  pub fn delete_saved_messages_topic_messages_by_date<
    C: AsRef<DeleteSavedMessagesTopicMessagesByDate>,
  >(
    &self,
    delete_saved_messages_topic_messages_by_date: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(delete_saved_messages_topic_messages_by_date.as_ref())
  }

  pub fn delete_saved_order_info<C: AsRef<DeleteSavedOrderInfo>>(
    &self,
    delete_saved_order_info: C,
  ) -> RTDResult<()> {
    self.api.send(delete_saved_order_info.as_ref())
  }

  pub fn delete_sticker_set<C: AsRef<DeleteStickerSet>>(
    &self,
    delete_sticker_set: C,
  ) -> RTDResult<()> {
    self.api.send(delete_sticker_set.as_ref())
  }

  pub fn delete_story<C: AsRef<DeleteStory>>(&self, delete_story: C) -> RTDResult<()> {
    self.api.send(delete_story.as_ref())
  }

  pub fn delete_story_album<C: AsRef<DeleteStoryAlbum>>(
    &self,
    delete_story_album: C,
  ) -> RTDResult<()> {
    self.api.send(delete_story_album.as_ref())
  }

  pub fn delete_text_composition_style<C: AsRef<DeleteTextCompositionStyle>>(
    &self,
    delete_text_composition_style: C,
  ) -> RTDResult<()> {
    self.api.send(delete_text_composition_style.as_ref())
  }

  pub fn delete_ton_wallet<C: AsRef<DeleteTonWallet>>(
    &self,
    delete_ton_wallet: C,
  ) -> RTDResult<()> {
    self.api.send(delete_ton_wallet.as_ref())
  }

  pub fn destroy<C: AsRef<Destroy>>(&self, destroy: C) -> RTDResult<()> {
    self.api.send(destroy.as_ref())
  }

  pub fn disable_all_supergroup_usernames<C: AsRef<DisableAllSupergroupUsernames>>(
    &self,
    disable_all_supergroup_usernames: C,
  ) -> RTDResult<()> {
    self.api.send(disable_all_supergroup_usernames.as_ref())
  }

  pub fn disable_proxy<C: AsRef<DisableProxy>>(&self, disable_proxy: C) -> RTDResult<()> {
    self.api.send(disable_proxy.as_ref())
  }

  pub fn disable_ton_wallet_backup<C: AsRef<DisableTonWalletBackup>>(
    &self,
    disable_ton_wallet_backup: C,
  ) -> RTDResult<()> {
    self.api.send(disable_ton_wallet_backup.as_ref())
  }

  pub fn disable_ton_wallet_backup_with_proof<C: AsRef<DisableTonWalletBackupWithProof>>(
    &self,
    disable_ton_wallet_backup_with_proof: C,
  ) -> RTDResult<()> {
    self.api.send(disable_ton_wallet_backup_with_proof.as_ref())
  }

  pub fn discard_call<C: AsRef<DiscardCall>>(&self, discard_call: C) -> RTDResult<()> {
    self.api.send(discard_call.as_ref())
  }

  pub fn disconnect_affiliate_program<C: AsRef<DisconnectAffiliateProgram>>(
    &self,
    disconnect_affiliate_program: C,
  ) -> RTDResult<()> {
    self.api.send(disconnect_affiliate_program.as_ref())
  }

  pub fn disconnect_all_websites<C: AsRef<DisconnectAllWebsites>>(
    &self,
    disconnect_all_websites: C,
  ) -> RTDResult<()> {
    self.api.send(disconnect_all_websites.as_ref())
  }

  pub fn disconnect_ton_connect_session<C: AsRef<DisconnectTonConnectSession>>(
    &self,
    disconnect_ton_connect_session: C,
  ) -> RTDResult<()> {
    self.api.send(disconnect_ton_connect_session.as_ref())
  }

  pub fn disconnect_website<C: AsRef<DisconnectWebsite>>(
    &self,
    disconnect_website: C,
  ) -> RTDResult<()> {
    self.api.send(disconnect_website.as_ref())
  }

  pub fn dismiss_web_token<C: AsRef<DismissWebToken>>(
    &self,
    dismiss_web_token: C,
  ) -> RTDResult<()> {
    self.api.send(dismiss_web_token.as_ref())
  }

  pub fn display_upgraded_gift<C: AsRef<DisplayUpgradedGift>>(
    &self,
    display_upgraded_gift: C,
  ) -> RTDResult<()> {
    self.api.send(display_upgraded_gift.as_ref())
  }

  pub fn download_file<C: AsRef<DownloadFile>>(&self, download_file: C) -> RTDResult<()> {
    self.api.send(download_file.as_ref())
  }

  pub fn drop_gift_original_details<C: AsRef<DropGiftOriginalDetails>>(
    &self,
    drop_gift_original_details: C,
  ) -> RTDResult<()> {
    self.api.send(drop_gift_original_details.as_ref())
  }

  pub fn edit_bot_media_preview<C: AsRef<EditBotMediaPreview>>(
    &self,
    edit_bot_media_preview: C,
  ) -> RTDResult<()> {
    self.api.send(edit_bot_media_preview.as_ref())
  }

  pub fn edit_business_chat_link<C: AsRef<EditBusinessChatLink>>(
    &self,
    edit_business_chat_link: C,
  ) -> RTDResult<()> {
    self.api.send(edit_business_chat_link.as_ref())
  }

  pub fn edit_business_message_caption<C: AsRef<EditBusinessMessageCaption>>(
    &self,
    edit_business_message_caption: C,
  ) -> RTDResult<()> {
    self.api.send(edit_business_message_caption.as_ref())
  }

  pub fn edit_business_message_checklist<C: AsRef<EditBusinessMessageChecklist>>(
    &self,
    edit_business_message_checklist: C,
  ) -> RTDResult<()> {
    self.api.send(edit_business_message_checklist.as_ref())
  }

  pub fn edit_business_message_live_location<C: AsRef<EditBusinessMessageLiveLocation>>(
    &self,
    edit_business_message_live_location: C,
  ) -> RTDResult<()> {
    self.api.send(edit_business_message_live_location.as_ref())
  }

  pub fn edit_business_message_media<C: AsRef<EditBusinessMessageMedia>>(
    &self,
    edit_business_message_media: C,
  ) -> RTDResult<()> {
    self.api.send(edit_business_message_media.as_ref())
  }

  pub fn edit_business_message_reply_markup<C: AsRef<EditBusinessMessageReplyMarkup>>(
    &self,
    edit_business_message_reply_markup: C,
  ) -> RTDResult<()> {
    self.api.send(edit_business_message_reply_markup.as_ref())
  }

  pub fn edit_business_message_text<C: AsRef<EditBusinessMessageText>>(
    &self,
    edit_business_message_text: C,
  ) -> RTDResult<()> {
    self.api.send(edit_business_message_text.as_ref())
  }

  pub fn edit_business_story<C: AsRef<EditBusinessStory>>(
    &self,
    edit_business_story: C,
  ) -> RTDResult<()> {
    self.api.send(edit_business_story.as_ref())
  }

  pub fn edit_chat_folder<C: AsRef<EditChatFolder>>(&self, edit_chat_folder: C) -> RTDResult<()> {
    self.api.send(edit_chat_folder.as_ref())
  }

  pub fn edit_chat_folder_invite_link<C: AsRef<EditChatFolderInviteLink>>(
    &self,
    edit_chat_folder_invite_link: C,
  ) -> RTDResult<()> {
    self.api.send(edit_chat_folder_invite_link.as_ref())
  }

  pub fn edit_chat_invite_link<C: AsRef<EditChatInviteLink>>(
    &self,
    edit_chat_invite_link: C,
  ) -> RTDResult<()> {
    self.api.send(edit_chat_invite_link.as_ref())
  }

  pub fn edit_chat_subscription_invite_link<C: AsRef<EditChatSubscriptionInviteLink>>(
    &self,
    edit_chat_subscription_invite_link: C,
  ) -> RTDResult<()> {
    self.api.send(edit_chat_subscription_invite_link.as_ref())
  }

  pub fn edit_chat_welcome_message<C: AsRef<EditChatWelcomeMessage>>(
    &self,
    edit_chat_welcome_message: C,
  ) -> RTDResult<()> {
    self.api.send(edit_chat_welcome_message.as_ref())
  }

  pub fn edit_custom_language_pack_info<C: AsRef<EditCustomLanguagePackInfo>>(
    &self,
    edit_custom_language_pack_info: C,
  ) -> RTDResult<()> {
    self.api.send(edit_custom_language_pack_info.as_ref())
  }

  pub fn edit_ephemeral_message<C: AsRef<EditEphemeralMessage>>(
    &self,
    edit_ephemeral_message: C,
  ) -> RTDResult<()> {
    self.api.send(edit_ephemeral_message.as_ref())
  }

  pub fn edit_ephemeral_message_caption<C: AsRef<EditEphemeralMessageCaption>>(
    &self,
    edit_ephemeral_message_caption: C,
  ) -> RTDResult<()> {
    self.api.send(edit_ephemeral_message_caption.as_ref())
  }

  pub fn edit_forum_topic<C: AsRef<EditForumTopic>>(&self, edit_forum_topic: C) -> RTDResult<()> {
    self.api.send(edit_forum_topic.as_ref())
  }

  pub fn edit_inline_message_caption<C: AsRef<EditInlineMessageCaption>>(
    &self,
    edit_inline_message_caption: C,
  ) -> RTDResult<()> {
    self.api.send(edit_inline_message_caption.as_ref())
  }

  pub fn edit_inline_message_live_location<C: AsRef<EditInlineMessageLiveLocation>>(
    &self,
    edit_inline_message_live_location: C,
  ) -> RTDResult<()> {
    self.api.send(edit_inline_message_live_location.as_ref())
  }

  pub fn edit_inline_message_media<C: AsRef<EditInlineMessageMedia>>(
    &self,
    edit_inline_message_media: C,
  ) -> RTDResult<()> {
    self.api.send(edit_inline_message_media.as_ref())
  }

  pub fn edit_inline_message_reply_markup<C: AsRef<EditInlineMessageReplyMarkup>>(
    &self,
    edit_inline_message_reply_markup: C,
  ) -> RTDResult<()> {
    self.api.send(edit_inline_message_reply_markup.as_ref())
  }

  pub fn edit_inline_message_text<C: AsRef<EditInlineMessageText>>(
    &self,
    edit_inline_message_text: C,
  ) -> RTDResult<()> {
    self.api.send(edit_inline_message_text.as_ref())
  }

  pub fn edit_message_caption<C: AsRef<EditMessageCaption>>(
    &self,
    edit_message_caption: C,
  ) -> RTDResult<()> {
    self.api.send(edit_message_caption.as_ref())
  }

  pub fn edit_message_checklist<C: AsRef<EditMessageChecklist>>(
    &self,
    edit_message_checklist: C,
  ) -> RTDResult<()> {
    self.api.send(edit_message_checklist.as_ref())
  }

  pub fn edit_message_live_location<C: AsRef<EditMessageLiveLocation>>(
    &self,
    edit_message_live_location: C,
  ) -> RTDResult<()> {
    self.api.send(edit_message_live_location.as_ref())
  }

  pub fn edit_message_media<C: AsRef<EditMessageMedia>>(
    &self,
    edit_message_media: C,
  ) -> RTDResult<()> {
    self.api.send(edit_message_media.as_ref())
  }

  pub fn edit_message_reply_markup<C: AsRef<EditMessageReplyMarkup>>(
    &self,
    edit_message_reply_markup: C,
  ) -> RTDResult<()> {
    self.api.send(edit_message_reply_markup.as_ref())
  }

  pub fn edit_message_scheduling_state<C: AsRef<EditMessageSchedulingState>>(
    &self,
    edit_message_scheduling_state: C,
  ) -> RTDResult<()> {
    self.api.send(edit_message_scheduling_state.as_ref())
  }

  pub fn edit_message_text<C: AsRef<EditMessageText>>(
    &self,
    edit_message_text: C,
  ) -> RTDResult<()> {
    self.api.send(edit_message_text.as_ref())
  }

  pub fn edit_proxy<C: AsRef<EditProxy>>(&self, edit_proxy: C) -> RTDResult<()> {
    self.api.send(edit_proxy.as_ref())
  }

  pub fn edit_quick_reply_message<C: AsRef<EditQuickReplyMessage>>(
    &self,
    edit_quick_reply_message: C,
  ) -> RTDResult<()> {
    self.api.send(edit_quick_reply_message.as_ref())
  }

  pub fn edit_star_subscription<C: AsRef<EditStarSubscription>>(
    &self,
    edit_star_subscription: C,
  ) -> RTDResult<()> {
    self.api.send(edit_star_subscription.as_ref())
  }

  pub fn edit_story<C: AsRef<EditStory>>(&self, edit_story: C) -> RTDResult<()> {
    self.api.send(edit_story.as_ref())
  }

  pub fn edit_story_cover<C: AsRef<EditStoryCover>>(&self, edit_story_cover: C) -> RTDResult<()> {
    self.api.send(edit_story_cover.as_ref())
  }

  pub fn edit_text_composition_style<C: AsRef<EditTextCompositionStyle>>(
    &self,
    edit_text_composition_style: C,
  ) -> RTDResult<()> {
    self.api.send(edit_text_composition_style.as_ref())
  }

  pub fn edit_user_star_subscription<C: AsRef<EditUserStarSubscription>>(
    &self,
    edit_user_star_subscription: C,
  ) -> RTDResult<()> {
    self.api.send(edit_user_star_subscription.as_ref())
  }

  pub fn enable_proxy<C: AsRef<EnableProxy>>(&self, enable_proxy: C) -> RTDResult<()> {
    self.api.send(enable_proxy.as_ref())
  }

  pub fn enable_ton_wallet_backup<C: AsRef<EnableTonWalletBackup>>(
    &self,
    enable_ton_wallet_backup: C,
  ) -> RTDResult<()> {
    self.api.send(enable_ton_wallet_backup.as_ref())
  }

  pub fn encrypt_group_call_data<C: AsRef<EncryptGroupCallData>>(
    &self,
    encrypt_group_call_data: C,
  ) -> RTDResult<()> {
    self.api.send(encrypt_group_call_data.as_ref())
  }

  pub fn end_group_call<C: AsRef<EndGroupCall>>(&self, end_group_call: C) -> RTDResult<()> {
    self.api.send(end_group_call.as_ref())
  }

  pub fn end_group_call_recording<C: AsRef<EndGroupCallRecording>>(
    &self,
    end_group_call_recording: C,
  ) -> RTDResult<()> {
    self.api.send(end_group_call_recording.as_ref())
  }

  pub fn end_group_call_screen_sharing<C: AsRef<EndGroupCallScreenSharing>>(
    &self,
    end_group_call_screen_sharing: C,
  ) -> RTDResult<()> {
    self.api.send(end_group_call_screen_sharing.as_ref())
  }

  pub fn finish_file_generation<C: AsRef<FinishFileGeneration>>(
    &self,
    finish_file_generation: C,
  ) -> RTDResult<()> {
    self.api.send(finish_file_generation.as_ref())
  }

  pub fn fix_rich_message_with_ai<C: AsRef<FixRichMessageWithAi>>(
    &self,
    fix_rich_message_with_ai: C,
  ) -> RTDResult<()> {
    self.api.send(fix_rich_message_with_ai.as_ref())
  }

  pub fn fix_text_with_ai<C: AsRef<FixTextWithAi>>(&self, fix_text_with_ai: C) -> RTDResult<()> {
    self.api.send(fix_text_with_ai.as_ref())
  }

  pub fn forward_messages<C: AsRef<ForwardMessages>>(&self, forward_messages: C) -> RTDResult<()> {
    self.api.send(forward_messages.as_ref())
  }

  pub fn get_account_ttl<C: AsRef<GetAccountTtl>>(&self, get_account_ttl: C) -> RTDResult<()> {
    self.api.send(get_account_ttl.as_ref())
  }

  pub fn get_active_sessions<C: AsRef<GetActiveSessions>>(
    &self,
    get_active_sessions: C,
  ) -> RTDResult<()> {
    self.api.send(get_active_sessions.as_ref())
  }

  pub fn get_address_ton_wallet<C: AsRef<GetAddressTonWallet>>(
    &self,
    get_address_ton_wallet: C,
  ) -> RTDResult<()> {
    self.api.send(get_address_ton_wallet.as_ref())
  }

  pub fn get_all_passport_elements<C: AsRef<GetAllPassportElements>>(
    &self,
    get_all_passport_elements: C,
  ) -> RTDResult<()> {
    self.api.send(get_all_passport_elements.as_ref())
  }

  pub fn get_all_sticker_emojis<C: AsRef<GetAllStickerEmojis>>(
    &self,
    get_all_sticker_emojis: C,
  ) -> RTDResult<()> {
    self.api.send(get_all_sticker_emojis.as_ref())
  }

  pub fn get_animated_emoji<C: AsRef<GetAnimatedEmoji>>(
    &self,
    get_animated_emoji: C,
  ) -> RTDResult<()> {
    self.api.send(get_animated_emoji.as_ref())
  }

  pub fn get_application_config<C: AsRef<GetApplicationConfig>>(
    &self,
    get_application_config: C,
  ) -> RTDResult<()> {
    self.api.send(get_application_config.as_ref())
  }

  pub fn get_application_download_link<C: AsRef<GetApplicationDownloadLink>>(
    &self,
    get_application_download_link: C,
  ) -> RTDResult<()> {
    self.api.send(get_application_download_link.as_ref())
  }

  pub fn get_archive_chat_list_settings<C: AsRef<GetArchiveChatListSettings>>(
    &self,
    get_archive_chat_list_settings: C,
  ) -> RTDResult<()> {
    self.api.send(get_archive_chat_list_settings.as_ref())
  }

  pub fn get_archived_sticker_sets<C: AsRef<GetArchivedStickerSets>>(
    &self,
    get_archived_sticker_sets: C,
  ) -> RTDResult<()> {
    self.api.send(get_archived_sticker_sets.as_ref())
  }

  pub fn get_attached_sticker_sets<C: AsRef<GetAttachedStickerSets>>(
    &self,
    get_attached_sticker_sets: C,
  ) -> RTDResult<()> {
    self.api.send(get_attached_sticker_sets.as_ref())
  }

  pub fn get_attachment_menu_bot<C: AsRef<GetAttachmentMenuBot>>(
    &self,
    get_attachment_menu_bot: C,
  ) -> RTDResult<()> {
    self.api.send(get_attachment_menu_bot.as_ref())
  }

  pub fn get_authentication_passkey_parameters<C: AsRef<GetAuthenticationPasskeyParameters>>(
    &self,
    get_authentication_passkey_parameters: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(get_authentication_passkey_parameters.as_ref())
  }

  pub fn get_authorization_state<C: AsRef<GetAuthorizationState>>(
    &self,
    get_authorization_state: C,
  ) -> RTDResult<()> {
    self.api.send(get_authorization_state.as_ref())
  }

  pub fn get_auto_download_settings_presets<C: AsRef<GetAutoDownloadSettingsPresets>>(
    &self,
    get_auto_download_settings_presets: C,
  ) -> RTDResult<()> {
    self.api.send(get_auto_download_settings_presets.as_ref())
  }

  pub fn get_autosave_settings<C: AsRef<GetAutosaveSettings>>(
    &self,
    get_autosave_settings: C,
  ) -> RTDResult<()> {
    self.api.send(get_autosave_settings.as_ref())
  }

  pub fn get_available_chat_boost_slots<C: AsRef<GetAvailableChatBoostSlots>>(
    &self,
    get_available_chat_boost_slots: C,
  ) -> RTDResult<()> {
    self.api.send(get_available_chat_boost_slots.as_ref())
  }

  pub fn get_available_gifts<C: AsRef<GetAvailableGifts>>(
    &self,
    get_available_gifts: C,
  ) -> RTDResult<()> {
    self.api.send(get_available_gifts.as_ref())
  }

  pub fn get_background_url<C: AsRef<GetBackgroundUrl>>(
    &self,
    get_background_url: C,
  ) -> RTDResult<()> {
    self.api.send(get_background_url.as_ref())
  }

  pub fn get_bank_card_info<C: AsRef<GetBankCardInfo>>(
    &self,
    get_bank_card_info: C,
  ) -> RTDResult<()> {
    self.api.send(get_bank_card_info.as_ref())
  }

  pub fn get_basic_group<C: AsRef<GetBasicGroup>>(&self, get_basic_group: C) -> RTDResult<()> {
    self.api.send(get_basic_group.as_ref())
  }

  pub fn get_basic_group_full_info<C: AsRef<GetBasicGroupFullInfo>>(
    &self,
    get_basic_group_full_info: C,
  ) -> RTDResult<()> {
    self.api.send(get_basic_group_full_info.as_ref())
  }

  pub fn get_blocked_message_senders<C: AsRef<GetBlockedMessageSenders>>(
    &self,
    get_blocked_message_senders: C,
  ) -> RTDResult<()> {
    self.api.send(get_blocked_message_senders.as_ref())
  }

  pub fn get_bot_info_description<C: AsRef<GetBotInfoDescription>>(
    &self,
    get_bot_info_description: C,
  ) -> RTDResult<()> {
    self.api.send(get_bot_info_description.as_ref())
  }

  pub fn get_bot_info_short_description<C: AsRef<GetBotInfoShortDescription>>(
    &self,
    get_bot_info_short_description: C,
  ) -> RTDResult<()> {
    self.api.send(get_bot_info_short_description.as_ref())
  }

  pub fn get_bot_media_preview_info<C: AsRef<GetBotMediaPreviewInfo>>(
    &self,
    get_bot_media_preview_info: C,
  ) -> RTDResult<()> {
    self.api.send(get_bot_media_preview_info.as_ref())
  }

  pub fn get_bot_media_previews<C: AsRef<GetBotMediaPreviews>>(
    &self,
    get_bot_media_previews: C,
  ) -> RTDResult<()> {
    self.api.send(get_bot_media_previews.as_ref())
  }

  pub fn get_bot_name<C: AsRef<GetBotName>>(&self, get_bot_name: C) -> RTDResult<()> {
    self.api.send(get_bot_name.as_ref())
  }

  pub fn get_bot_similar_bot_count<C: AsRef<GetBotSimilarBotCount>>(
    &self,
    get_bot_similar_bot_count: C,
  ) -> RTDResult<()> {
    self.api.send(get_bot_similar_bot_count.as_ref())
  }

  pub fn get_bot_similar_bots<C: AsRef<GetBotSimilarBots>>(
    &self,
    get_bot_similar_bots: C,
  ) -> RTDResult<()> {
    self.api.send(get_bot_similar_bots.as_ref())
  }

  pub fn get_business_account_star_amount<C: AsRef<GetBusinessAccountStarAmount>>(
    &self,
    get_business_account_star_amount: C,
  ) -> RTDResult<()> {
    self.api.send(get_business_account_star_amount.as_ref())
  }

  pub fn get_business_chat_link_info<C: AsRef<GetBusinessChatLinkInfo>>(
    &self,
    get_business_chat_link_info: C,
  ) -> RTDResult<()> {
    self.api.send(get_business_chat_link_info.as_ref())
  }

  pub fn get_business_chat_links<C: AsRef<GetBusinessChatLinks>>(
    &self,
    get_business_chat_links: C,
  ) -> RTDResult<()> {
    self.api.send(get_business_chat_links.as_ref())
  }

  pub fn get_business_connected_bot<C: AsRef<GetBusinessConnectedBot>>(
    &self,
    get_business_connected_bot: C,
  ) -> RTDResult<()> {
    self.api.send(get_business_connected_bot.as_ref())
  }

  pub fn get_business_connection<C: AsRef<GetBusinessConnection>>(
    &self,
    get_business_connection: C,
  ) -> RTDResult<()> {
    self.api.send(get_business_connection.as_ref())
  }

  pub fn get_business_features<C: AsRef<GetBusinessFeatures>>(
    &self,
    get_business_features: C,
  ) -> RTDResult<()> {
    self.api.send(get_business_features.as_ref())
  }

  pub fn get_callback_query_answer<C: AsRef<GetCallbackQueryAnswer>>(
    &self,
    get_callback_query_answer: C,
  ) -> RTDResult<()> {
    self.api.send(get_callback_query_answer.as_ref())
  }

  pub fn get_callback_query_message<C: AsRef<GetCallbackQueryMessage>>(
    &self,
    get_callback_query_message: C,
  ) -> RTDResult<()> {
    self.api.send(get_callback_query_message.as_ref())
  }

  pub fn get_chat<C: AsRef<GetChat>>(&self, get_chat: C) -> RTDResult<()> {
    self.api.send(get_chat.as_ref())
  }

  pub fn get_chat_active_stories<C: AsRef<GetChatActiveStories>>(
    &self,
    get_chat_active_stories: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_active_stories.as_ref())
  }

  pub fn get_chat_administrators<C: AsRef<GetChatAdministrators>>(
    &self,
    get_chat_administrators: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_administrators.as_ref())
  }

  pub fn get_chat_archived_stories<C: AsRef<GetChatArchivedStories>>(
    &self,
    get_chat_archived_stories: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_archived_stories.as_ref())
  }

  pub fn get_chat_available_message_senders<C: AsRef<GetChatAvailableMessageSenders>>(
    &self,
    get_chat_available_message_senders: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_available_message_senders.as_ref())
  }

  pub fn get_chat_available_paid_message_reaction_senders<
    C: AsRef<GetChatAvailablePaidMessageReactionSenders>,
  >(
    &self,
    get_chat_available_paid_message_reaction_senders: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(get_chat_available_paid_message_reaction_senders.as_ref())
  }

  pub fn get_chat_boost_features<C: AsRef<GetChatBoostFeatures>>(
    &self,
    get_chat_boost_features: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_boost_features.as_ref())
  }

  pub fn get_chat_boost_level_features<C: AsRef<GetChatBoostLevelFeatures>>(
    &self,
    get_chat_boost_level_features: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_boost_level_features.as_ref())
  }

  pub fn get_chat_boost_link<C: AsRef<GetChatBoostLink>>(
    &self,
    get_chat_boost_link: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_boost_link.as_ref())
  }

  pub fn get_chat_boost_link_info<C: AsRef<GetChatBoostLinkInfo>>(
    &self,
    get_chat_boost_link_info: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_boost_link_info.as_ref())
  }

  pub fn get_chat_boost_status<C: AsRef<GetChatBoostStatus>>(
    &self,
    get_chat_boost_status: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_boost_status.as_ref())
  }

  pub fn get_chat_boosts<C: AsRef<GetChatBoosts>>(&self, get_chat_boosts: C) -> RTDResult<()> {
    self.api.send(get_chat_boosts.as_ref())
  }

  pub fn get_chat_event_log<C: AsRef<GetChatEventLog>>(
    &self,
    get_chat_event_log: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_event_log.as_ref())
  }

  pub fn get_chat_folder<C: AsRef<GetChatFolder>>(&self, get_chat_folder: C) -> RTDResult<()> {
    self.api.send(get_chat_folder.as_ref())
  }

  pub fn get_chat_folder_chat_count<C: AsRef<GetChatFolderChatCount>>(
    &self,
    get_chat_folder_chat_count: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_folder_chat_count.as_ref())
  }

  pub fn get_chat_folder_chats_to_leave<C: AsRef<GetChatFolderChatsToLeave>>(
    &self,
    get_chat_folder_chats_to_leave: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_folder_chats_to_leave.as_ref())
  }

  pub fn get_chat_folder_default_icon_name<C: AsRef<GetChatFolderDefaultIconName>>(
    &self,
    get_chat_folder_default_icon_name: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_folder_default_icon_name.as_ref())
  }

  pub fn get_chat_folder_invite_links<C: AsRef<GetChatFolderInviteLinks>>(
    &self,
    get_chat_folder_invite_links: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_folder_invite_links.as_ref())
  }

  pub fn get_chat_folder_new_chats<C: AsRef<GetChatFolderNewChats>>(
    &self,
    get_chat_folder_new_chats: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_folder_new_chats.as_ref())
  }

  pub fn get_chat_history<C: AsRef<GetChatHistory>>(&self, get_chat_history: C) -> RTDResult<()> {
    self.api.send(get_chat_history.as_ref())
  }

  pub fn get_chat_invite_link<C: AsRef<GetChatInviteLink>>(
    &self,
    get_chat_invite_link: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_invite_link.as_ref())
  }

  pub fn get_chat_invite_link_counts<C: AsRef<GetChatInviteLinkCounts>>(
    &self,
    get_chat_invite_link_counts: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_invite_link_counts.as_ref())
  }

  pub fn get_chat_invite_link_members<C: AsRef<GetChatInviteLinkMembers>>(
    &self,
    get_chat_invite_link_members: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_invite_link_members.as_ref())
  }

  pub fn get_chat_invite_links<C: AsRef<GetChatInviteLinks>>(
    &self,
    get_chat_invite_links: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_invite_links.as_ref())
  }

  pub fn get_chat_join_requests<C: AsRef<GetChatJoinRequests>>(
    &self,
    get_chat_join_requests: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_join_requests.as_ref())
  }

  pub fn get_chat_lists_to_add_chat<C: AsRef<GetChatListsToAddChat>>(
    &self,
    get_chat_lists_to_add_chat: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_lists_to_add_chat.as_ref())
  }

  pub fn get_chat_member<C: AsRef<GetChatMember>>(&self, get_chat_member: C) -> RTDResult<()> {
    self.api.send(get_chat_member.as_ref())
  }

  pub fn get_chat_message_by_date<C: AsRef<GetChatMessageByDate>>(
    &self,
    get_chat_message_by_date: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_message_by_date.as_ref())
  }

  pub fn get_chat_message_calendar<C: AsRef<GetChatMessageCalendar>>(
    &self,
    get_chat_message_calendar: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_message_calendar.as_ref())
  }

  pub fn get_chat_message_count<C: AsRef<GetChatMessageCount>>(
    &self,
    get_chat_message_count: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_message_count.as_ref())
  }

  pub fn get_chat_message_position<C: AsRef<GetChatMessagePosition>>(
    &self,
    get_chat_message_position: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_message_position.as_ref())
  }

  pub fn get_chat_notification_settings_exceptions<
    C: AsRef<GetChatNotificationSettingsExceptions>,
  >(
    &self,
    get_chat_notification_settings_exceptions: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(get_chat_notification_settings_exceptions.as_ref())
  }

  pub fn get_chat_owner_after_leaving<C: AsRef<GetChatOwnerAfterLeaving>>(
    &self,
    get_chat_owner_after_leaving: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_owner_after_leaving.as_ref())
  }

  pub fn get_chat_pinned_message<C: AsRef<GetChatPinnedMessage>>(
    &self,
    get_chat_pinned_message: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_pinned_message.as_ref())
  }

  pub fn get_chat_posted_to_chat_page_stories<C: AsRef<GetChatPostedToChatPageStories>>(
    &self,
    get_chat_posted_to_chat_page_stories: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_posted_to_chat_page_stories.as_ref())
  }

  pub fn get_chat_revenue_statistics<C: AsRef<GetChatRevenueStatistics>>(
    &self,
    get_chat_revenue_statistics: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_revenue_statistics.as_ref())
  }

  pub fn get_chat_revenue_transactions<C: AsRef<GetChatRevenueTransactions>>(
    &self,
    get_chat_revenue_transactions: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_revenue_transactions.as_ref())
  }

  pub fn get_chat_revenue_withdrawal_url<C: AsRef<GetChatRevenueWithdrawalUrl>>(
    &self,
    get_chat_revenue_withdrawal_url: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_revenue_withdrawal_url.as_ref())
  }

  pub fn get_chat_scheduled_messages<C: AsRef<GetChatScheduledMessages>>(
    &self,
    get_chat_scheduled_messages: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_scheduled_messages.as_ref())
  }

  pub fn get_chat_similar_chat_count<C: AsRef<GetChatSimilarChatCount>>(
    &self,
    get_chat_similar_chat_count: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_similar_chat_count.as_ref())
  }

  pub fn get_chat_similar_chats<C: AsRef<GetChatSimilarChats>>(
    &self,
    get_chat_similar_chats: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_similar_chats.as_ref())
  }

  pub fn get_chat_sparse_message_positions<C: AsRef<GetChatSparseMessagePositions>>(
    &self,
    get_chat_sparse_message_positions: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_sparse_message_positions.as_ref())
  }

  pub fn get_chat_sponsored_messages<C: AsRef<GetChatSponsoredMessages>>(
    &self,
    get_chat_sponsored_messages: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_sponsored_messages.as_ref())
  }

  pub fn get_chat_statistics<C: AsRef<GetChatStatistics>>(
    &self,
    get_chat_statistics: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_statistics.as_ref())
  }

  pub fn get_chat_story_albums<C: AsRef<GetChatStoryAlbums>>(
    &self,
    get_chat_story_albums: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_story_albums.as_ref())
  }

  pub fn get_chat_story_interactions<C: AsRef<GetChatStoryInteractions>>(
    &self,
    get_chat_story_interactions: C,
  ) -> RTDResult<()> {
    self.api.send(get_chat_story_interactions.as_ref())
  }

  pub fn get_chats<C: AsRef<GetChats>>(&self, get_chats: C) -> RTDResult<()> {
    self.api.send(get_chats.as_ref())
  }

  pub fn get_chats_for_chat_folder_invite_link<C: AsRef<GetChatsForChatFolderInviteLink>>(
    &self,
    get_chats_for_chat_folder_invite_link: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(get_chats_for_chat_folder_invite_link.as_ref())
  }

  pub fn get_chats_to_post_stories<C: AsRef<GetChatsToPostStories>>(
    &self,
    get_chats_to_post_stories: C,
  ) -> RTDResult<()> {
    self.api.send(get_chats_to_post_stories.as_ref())
  }

  pub fn get_close_friends<C: AsRef<GetCloseFriends>>(
    &self,
    get_close_friends: C,
  ) -> RTDResult<()> {
    self.api.send(get_close_friends.as_ref())
  }

  pub fn get_collectible_item_info<C: AsRef<GetCollectibleItemInfo>>(
    &self,
    get_collectible_item_info: C,
  ) -> RTDResult<()> {
    self.api.send(get_collectible_item_info.as_ref())
  }

  pub fn get_commands<C: AsRef<GetCommands>>(&self, get_commands: C) -> RTDResult<()> {
    self.api.send(get_commands.as_ref())
  }

  pub fn get_community_full_info<C: AsRef<GetCommunityFullInfo>>(
    &self,
    get_community_full_info: C,
  ) -> RTDResult<()> {
    self.api.send(get_community_full_info.as_ref())
  }

  pub fn get_connected_affiliate_program<C: AsRef<GetConnectedAffiliateProgram>>(
    &self,
    get_connected_affiliate_program: C,
  ) -> RTDResult<()> {
    self.api.send(get_connected_affiliate_program.as_ref())
  }

  pub fn get_connected_affiliate_programs<C: AsRef<GetConnectedAffiliatePrograms>>(
    &self,
    get_connected_affiliate_programs: C,
  ) -> RTDResult<()> {
    self.api.send(get_connected_affiliate_programs.as_ref())
  }

  pub fn get_connected_websites<C: AsRef<GetConnectedWebsites>>(
    &self,
    get_connected_websites: C,
  ) -> RTDResult<()> {
    self.api.send(get_connected_websites.as_ref())
  }

  pub fn get_contacts<C: AsRef<GetContacts>>(&self, get_contacts: C) -> RTDResult<()> {
    self.api.send(get_contacts.as_ref())
  }

  pub fn get_countries<C: AsRef<GetCountries>>(&self, get_countries: C) -> RTDResult<()> {
    self.api.send(get_countries.as_ref())
  }

  pub fn get_country<C: AsRef<GetCountry>>(&self, get_country: C) -> RTDResult<()> {
    self.api.send(get_country.as_ref())
  }

  pub fn get_country_code<C: AsRef<GetCountryCode>>(&self, get_country_code: C) -> RTDResult<()> {
    self.api.send(get_country_code.as_ref())
  }

  pub fn get_country_flag_emoji<C: AsRef<GetCountryFlagEmoji>>(
    &self,
    get_country_flag_emoji: C,
  ) -> RTDResult<()> {
    self.api.send(get_country_flag_emoji.as_ref())
  }

  pub fn get_created_public_chats<C: AsRef<GetCreatedPublicChats>>(
    &self,
    get_created_public_chats: C,
  ) -> RTDResult<()> {
    self.api.send(get_created_public_chats.as_ref())
  }

  pub fn get_currency_exchange_rates<C: AsRef<GetCurrencyExchangeRates>>(
    &self,
    get_currency_exchange_rates: C,
  ) -> RTDResult<()> {
    self.api.send(get_currency_exchange_rates.as_ref())
  }

  pub fn get_current_state<C: AsRef<GetCurrentState>>(
    &self,
    get_current_state: C,
  ) -> RTDResult<()> {
    self.api.send(get_current_state.as_ref())
  }

  pub fn get_current_weather<C: AsRef<GetCurrentWeather>>(
    &self,
    get_current_weather: C,
  ) -> RTDResult<()> {
    self.api.send(get_current_weather.as_ref())
  }

  pub fn get_custom_emoji_reaction_animations<C: AsRef<GetCustomEmojiReactionAnimations>>(
    &self,
    get_custom_emoji_reaction_animations: C,
  ) -> RTDResult<()> {
    self.api.send(get_custom_emoji_reaction_animations.as_ref())
  }

  pub fn get_custom_emoji_stickers<C: AsRef<GetCustomEmojiStickers>>(
    &self,
    get_custom_emoji_stickers: C,
  ) -> RTDResult<()> {
    self.api.send(get_custom_emoji_stickers.as_ref())
  }

  pub fn get_database_statistics<C: AsRef<GetDatabaseStatistics>>(
    &self,
    get_database_statistics: C,
  ) -> RTDResult<()> {
    self.api.send(get_database_statistics.as_ref())
  }

  pub fn get_deep_link_info<C: AsRef<GetDeepLinkInfo>>(
    &self,
    get_deep_link_info: C,
  ) -> RTDResult<()> {
    self.api.send(get_deep_link_info.as_ref())
  }

  pub fn get_default_background_custom_emoji_stickers<
    C: AsRef<GetDefaultBackgroundCustomEmojiStickers>,
  >(
    &self,
    get_default_background_custom_emoji_stickers: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(get_default_background_custom_emoji_stickers.as_ref())
  }

  pub fn get_default_chat_emoji_statuses<C: AsRef<GetDefaultChatEmojiStatuses>>(
    &self,
    get_default_chat_emoji_statuses: C,
  ) -> RTDResult<()> {
    self.api.send(get_default_chat_emoji_statuses.as_ref())
  }

  pub fn get_default_chat_photo_custom_emoji_stickers<
    C: AsRef<GetDefaultChatPhotoCustomEmojiStickers>,
  >(
    &self,
    get_default_chat_photo_custom_emoji_stickers: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(get_default_chat_photo_custom_emoji_stickers.as_ref())
  }

  pub fn get_default_emoji_statuses<C: AsRef<GetDefaultEmojiStatuses>>(
    &self,
    get_default_emoji_statuses: C,
  ) -> RTDResult<()> {
    self.api.send(get_default_emoji_statuses.as_ref())
  }

  pub fn get_default_message_auto_delete_time<C: AsRef<GetDefaultMessageAutoDeleteTime>>(
    &self,
    get_default_message_auto_delete_time: C,
  ) -> RTDResult<()> {
    self.api.send(get_default_message_auto_delete_time.as_ref())
  }

  pub fn get_default_profile_photo_custom_emoji_stickers<
    C: AsRef<GetDefaultProfilePhotoCustomEmojiStickers>,
  >(
    &self,
    get_default_profile_photo_custom_emoji_stickers: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(get_default_profile_photo_custom_emoji_stickers.as_ref())
  }

  pub fn get_direct_messages_chat_topic<C: AsRef<GetDirectMessagesChatTopic>>(
    &self,
    get_direct_messages_chat_topic: C,
  ) -> RTDResult<()> {
    self.api.send(get_direct_messages_chat_topic.as_ref())
  }

  pub fn get_direct_messages_chat_topic_history<C: AsRef<GetDirectMessagesChatTopicHistory>>(
    &self,
    get_direct_messages_chat_topic_history: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(get_direct_messages_chat_topic_history.as_ref())
  }

  pub fn get_direct_messages_chat_topic_message_by_date<
    C: AsRef<GetDirectMessagesChatTopicMessageByDate>,
  >(
    &self,
    get_direct_messages_chat_topic_message_by_date: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(get_direct_messages_chat_topic_message_by_date.as_ref())
  }

  pub fn get_direct_messages_chat_topic_revenue<C: AsRef<GetDirectMessagesChatTopicRevenue>>(
    &self,
    get_direct_messages_chat_topic_revenue: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(get_direct_messages_chat_topic_revenue.as_ref())
  }

  pub fn get_disallowed_chat_emoji_statuses<C: AsRef<GetDisallowedChatEmojiStatuses>>(
    &self,
    get_disallowed_chat_emoji_statuses: C,
  ) -> RTDResult<()> {
    self.api.send(get_disallowed_chat_emoji_statuses.as_ref())
  }

  pub fn get_emoji_categories<C: AsRef<GetEmojiCategories>>(
    &self,
    get_emoji_categories: C,
  ) -> RTDResult<()> {
    self.api.send(get_emoji_categories.as_ref())
  }

  pub fn get_emoji_reaction<C: AsRef<GetEmojiReaction>>(
    &self,
    get_emoji_reaction: C,
  ) -> RTDResult<()> {
    self.api.send(get_emoji_reaction.as_ref())
  }

  pub fn get_emoji_suggestions_url<C: AsRef<GetEmojiSuggestionsUrl>>(
    &self,
    get_emoji_suggestions_url: C,
  ) -> RTDResult<()> {
    self.api.send(get_emoji_suggestions_url.as_ref())
  }

  pub fn get_external_link<C: AsRef<GetExternalLink>>(
    &self,
    get_external_link: C,
  ) -> RTDResult<()> {
    self.api.send(get_external_link.as_ref())
  }

  pub fn get_external_link_info<C: AsRef<GetExternalLinkInfo>>(
    &self,
    get_external_link_info: C,
  ) -> RTDResult<()> {
    self.api.send(get_external_link_info.as_ref())
  }

  pub fn get_favorite_stickers<C: AsRef<GetFavoriteStickers>>(
    &self,
    get_favorite_stickers: C,
  ) -> RTDResult<()> {
    self.api.send(get_favorite_stickers.as_ref())
  }

  pub fn get_file<C: AsRef<GetFile>>(&self, get_file: C) -> RTDResult<()> {
    self.api.send(get_file.as_ref())
  }

  pub fn get_file_downloaded_prefix_size<C: AsRef<GetFileDownloadedPrefixSize>>(
    &self,
    get_file_downloaded_prefix_size: C,
  ) -> RTDResult<()> {
    self.api.send(get_file_downloaded_prefix_size.as_ref())
  }

  pub fn get_file_extension<C: AsRef<GetFileExtension>>(
    &self,
    get_file_extension: C,
  ) -> RTDResult<()> {
    self.api.send(get_file_extension.as_ref())
  }

  pub fn get_file_mime_type<C: AsRef<GetFileMimeType>>(
    &self,
    get_file_mime_type: C,
  ) -> RTDResult<()> {
    self.api.send(get_file_mime_type.as_ref())
  }

  pub fn get_forum_topic<C: AsRef<GetForumTopic>>(&self, get_forum_topic: C) -> RTDResult<()> {
    self.api.send(get_forum_topic.as_ref())
  }

  pub fn get_forum_topic_default_icons<C: AsRef<GetForumTopicDefaultIcons>>(
    &self,
    get_forum_topic_default_icons: C,
  ) -> RTDResult<()> {
    self.api.send(get_forum_topic_default_icons.as_ref())
  }

  pub fn get_forum_topic_history<C: AsRef<GetForumTopicHistory>>(
    &self,
    get_forum_topic_history: C,
  ) -> RTDResult<()> {
    self.api.send(get_forum_topic_history.as_ref())
  }

  pub fn get_forum_topic_link<C: AsRef<GetForumTopicLink>>(
    &self,
    get_forum_topic_link: C,
  ) -> RTDResult<()> {
    self.api.send(get_forum_topic_link.as_ref())
  }

  pub fn get_forum_topics<C: AsRef<GetForumTopics>>(&self, get_forum_topics: C) -> RTDResult<()> {
    self.api.send(get_forum_topics.as_ref())
  }

  pub fn get_full_rich_message<C: AsRef<GetFullRichMessage>>(
    &self,
    get_full_rich_message: C,
  ) -> RTDResult<()> {
    self.api.send(get_full_rich_message.as_ref())
  }

  pub fn get_game_high_scores<C: AsRef<GetGameHighScores>>(
    &self,
    get_game_high_scores: C,
  ) -> RTDResult<()> {
    self.api.send(get_game_high_scores.as_ref())
  }

  pub fn get_gift_auction_acquired_gifts<C: AsRef<GetGiftAuctionAcquiredGifts>>(
    &self,
    get_gift_auction_acquired_gifts: C,
  ) -> RTDResult<()> {
    self.api.send(get_gift_auction_acquired_gifts.as_ref())
  }

  pub fn get_gift_auction_state<C: AsRef<GetGiftAuctionState>>(
    &self,
    get_gift_auction_state: C,
  ) -> RTDResult<()> {
    self.api.send(get_gift_auction_state.as_ref())
  }

  pub fn get_gift_chat_themes<C: AsRef<GetGiftChatThemes>>(
    &self,
    get_gift_chat_themes: C,
  ) -> RTDResult<()> {
    self.api.send(get_gift_chat_themes.as_ref())
  }

  pub fn get_gift_collections<C: AsRef<GetGiftCollections>>(
    &self,
    get_gift_collections: C,
  ) -> RTDResult<()> {
    self.api.send(get_gift_collections.as_ref())
  }

  pub fn get_gift_upgrade_preview<C: AsRef<GetGiftUpgradePreview>>(
    &self,
    get_gift_upgrade_preview: C,
  ) -> RTDResult<()> {
    self.api.send(get_gift_upgrade_preview.as_ref())
  }

  pub fn get_gifts_for_crafting<C: AsRef<GetGiftsForCrafting>>(
    &self,
    get_gifts_for_crafting: C,
  ) -> RTDResult<()> {
    self.api.send(get_gifts_for_crafting.as_ref())
  }

  pub fn get_giveaway_info<C: AsRef<GetGiveawayInfo>>(
    &self,
    get_giveaway_info: C,
  ) -> RTDResult<()> {
    self.api.send(get_giveaway_info.as_ref())
  }

  pub fn get_gram_revenue_statistics<C: AsRef<GetGramRevenueStatistics>>(
    &self,
    get_gram_revenue_statistics: C,
  ) -> RTDResult<()> {
    self.api.send(get_gram_revenue_statistics.as_ref())
  }

  pub fn get_gram_withdrawal_url<C: AsRef<GetGramWithdrawalUrl>>(
    &self,
    get_gram_withdrawal_url: C,
  ) -> RTDResult<()> {
    self.api.send(get_gram_withdrawal_url.as_ref())
  }

  pub fn get_greeting_stickers<C: AsRef<GetGreetingStickers>>(
    &self,
    get_greeting_stickers: C,
  ) -> RTDResult<()> {
    self.api.send(get_greeting_stickers.as_ref())
  }

  pub fn get_grossing_web_app_bots<C: AsRef<GetGrossingWebAppBots>>(
    &self,
    get_grossing_web_app_bots: C,
  ) -> RTDResult<()> {
    self.api.send(get_grossing_web_app_bots.as_ref())
  }

  pub fn get_group_call<C: AsRef<GetGroupCall>>(&self, get_group_call: C) -> RTDResult<()> {
    self.api.send(get_group_call.as_ref())
  }

  pub fn get_group_call_participants<C: AsRef<GetGroupCallParticipants>>(
    &self,
    get_group_call_participants: C,
  ) -> RTDResult<()> {
    self.api.send(get_group_call_participants.as_ref())
  }

  pub fn get_group_call_stream_segment<C: AsRef<GetGroupCallStreamSegment>>(
    &self,
    get_group_call_stream_segment: C,
  ) -> RTDResult<()> {
    self.api.send(get_group_call_stream_segment.as_ref())
  }

  pub fn get_group_call_streams<C: AsRef<GetGroupCallStreams>>(
    &self,
    get_group_call_streams: C,
  ) -> RTDResult<()> {
    self.api.send(get_group_call_streams.as_ref())
  }

  pub fn get_groups_in_common<C: AsRef<GetGroupsInCommon>>(
    &self,
    get_groups_in_common: C,
  ) -> RTDResult<()> {
    self.api.send(get_groups_in_common.as_ref())
  }

  pub fn get_guard_bot_web_app_url<C: AsRef<GetGuardBotWebAppUrl>>(
    &self,
    get_guard_bot_web_app_url: C,
  ) -> RTDResult<()> {
    self.api.send(get_guard_bot_web_app_url.as_ref())
  }

  pub fn get_imported_contact_count<C: AsRef<GetImportedContactCount>>(
    &self,
    get_imported_contact_count: C,
  ) -> RTDResult<()> {
    self.api.send(get_imported_contact_count.as_ref())
  }

  pub fn get_inactive_supergroup_chats<C: AsRef<GetInactiveSupergroupChats>>(
    &self,
    get_inactive_supergroup_chats: C,
  ) -> RTDResult<()> {
    self.api.send(get_inactive_supergroup_chats.as_ref())
  }

  pub fn get_inline_game_high_scores<C: AsRef<GetInlineGameHighScores>>(
    &self,
    get_inline_game_high_scores: C,
  ) -> RTDResult<()> {
    self.api.send(get_inline_game_high_scores.as_ref())
  }

  pub fn get_inline_query_results<C: AsRef<GetInlineQueryResults>>(
    &self,
    get_inline_query_results: C,
  ) -> RTDResult<()> {
    self.api.send(get_inline_query_results.as_ref())
  }

  pub fn get_installed_backgrounds<C: AsRef<GetInstalledBackgrounds>>(
    &self,
    get_installed_backgrounds: C,
  ) -> RTDResult<()> {
    self.api.send(get_installed_backgrounds.as_ref())
  }

  pub fn get_installed_sticker_sets<C: AsRef<GetInstalledStickerSets>>(
    &self,
    get_installed_sticker_sets: C,
  ) -> RTDResult<()> {
    self.api.send(get_installed_sticker_sets.as_ref())
  }

  pub fn get_internal_link<C: AsRef<GetInternalLink>>(
    &self,
    get_internal_link: C,
  ) -> RTDResult<()> {
    self.api.send(get_internal_link.as_ref())
  }

  pub fn get_internal_link_type<C: AsRef<GetInternalLinkType>>(
    &self,
    get_internal_link_type: C,
  ) -> RTDResult<()> {
    self.api.send(get_internal_link_type.as_ref())
  }

  pub fn get_json_string<C: AsRef<GetJsonString>>(&self, get_json_string: C) -> RTDResult<()> {
    self.api.send(get_json_string.as_ref())
  }

  pub fn get_json_value<C: AsRef<GetJsonValue>>(&self, get_json_value: C) -> RTDResult<()> {
    self.api.send(get_json_value.as_ref())
  }

  pub fn get_keyword_emojis<C: AsRef<GetKeywordEmojis>>(
    &self,
    get_keyword_emojis: C,
  ) -> RTDResult<()> {
    self.api.send(get_keyword_emojis.as_ref())
  }

  pub fn get_language_pack_info<C: AsRef<GetLanguagePackInfo>>(
    &self,
    get_language_pack_info: C,
  ) -> RTDResult<()> {
    self.api.send(get_language_pack_info.as_ref())
  }

  pub fn get_language_pack_string<C: AsRef<GetLanguagePackString>>(
    &self,
    get_language_pack_string: C,
  ) -> RTDResult<()> {
    self.api.send(get_language_pack_string.as_ref())
  }

  pub fn get_language_pack_strings<C: AsRef<GetLanguagePackStrings>>(
    &self,
    get_language_pack_strings: C,
  ) -> RTDResult<()> {
    self.api.send(get_language_pack_strings.as_ref())
  }

  pub fn get_link_preview<C: AsRef<GetLinkPreview>>(&self, get_link_preview: C) -> RTDResult<()> {
    self.api.send(get_link_preview.as_ref())
  }

  pub fn get_link_web_browser_type<C: AsRef<GetLinkWebBrowserType>>(
    &self,
    get_link_web_browser_type: C,
  ) -> RTDResult<()> {
    self.api.send(get_link_web_browser_type.as_ref())
  }

  pub fn get_live_story_available_message_senders<C: AsRef<GetLiveStoryAvailableMessageSenders>>(
    &self,
    get_live_story_available_message_senders: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(get_live_story_available_message_senders.as_ref())
  }

  pub fn get_live_story_rtmp_url<C: AsRef<GetLiveStoryRtmpUrl>>(
    &self,
    get_live_story_rtmp_url: C,
  ) -> RTDResult<()> {
    self.api.send(get_live_story_rtmp_url.as_ref())
  }

  pub fn get_live_story_streamer<C: AsRef<GetLiveStoryStreamer>>(
    &self,
    get_live_story_streamer: C,
  ) -> RTDResult<()> {
    self.api.send(get_live_story_streamer.as_ref())
  }

  pub fn get_live_story_top_donors<C: AsRef<GetLiveStoryTopDonors>>(
    &self,
    get_live_story_top_donors: C,
  ) -> RTDResult<()> {
    self.api.send(get_live_story_top_donors.as_ref())
  }

  pub fn get_localization_target_info<C: AsRef<GetLocalizationTargetInfo>>(
    &self,
    get_localization_target_info: C,
  ) -> RTDResult<()> {
    self.api.send(get_localization_target_info.as_ref())
  }

  pub fn get_log_stream<C: AsRef<GetLogStream>>(&self, get_log_stream: C) -> RTDResult<()> {
    self.api.send(get_log_stream.as_ref())
  }

  pub fn get_log_tag_verbosity_level<C: AsRef<GetLogTagVerbosityLevel>>(
    &self,
    get_log_tag_verbosity_level: C,
  ) -> RTDResult<()> {
    self.api.send(get_log_tag_verbosity_level.as_ref())
  }

  pub fn get_log_tags<C: AsRef<GetLogTags>>(&self, get_log_tags: C) -> RTDResult<()> {
    self.api.send(get_log_tags.as_ref())
  }

  pub fn get_log_verbosity_level<C: AsRef<GetLogVerbosityLevel>>(
    &self,
    get_log_verbosity_level: C,
  ) -> RTDResult<()> {
    self.api.send(get_log_verbosity_level.as_ref())
  }

  pub fn get_login_passkeys<C: AsRef<GetLoginPasskeys>>(
    &self,
    get_login_passkeys: C,
  ) -> RTDResult<()> {
    self.api.send(get_login_passkeys.as_ref())
  }

  pub fn get_login_url<C: AsRef<GetLoginUrl>>(&self, get_login_url: C) -> RTDResult<()> {
    self.api.send(get_login_url.as_ref())
  }

  pub fn get_login_url_info<C: AsRef<GetLoginUrlInfo>>(
    &self,
    get_login_url_info: C,
  ) -> RTDResult<()> {
    self.api.send(get_login_url_info.as_ref())
  }

  pub fn get_main_web_app<C: AsRef<GetMainWebApp>>(&self, get_main_web_app: C) -> RTDResult<()> {
    self.api.send(get_main_web_app.as_ref())
  }

  pub fn get_managed_bot_access_settings<C: AsRef<GetManagedBotAccessSettings>>(
    &self,
    get_managed_bot_access_settings: C,
  ) -> RTDResult<()> {
    self.api.send(get_managed_bot_access_settings.as_ref())
  }

  pub fn get_managed_bot_token<C: AsRef<GetManagedBotToken>>(
    &self,
    get_managed_bot_token: C,
  ) -> RTDResult<()> {
    self.api.send(get_managed_bot_token.as_ref())
  }

  pub fn get_map_thumbnail_file<C: AsRef<GetMapThumbnailFile>>(
    &self,
    get_map_thumbnail_file: C,
  ) -> RTDResult<()> {
    self.api.send(get_map_thumbnail_file.as_ref())
  }

  pub fn get_markdown_text<C: AsRef<GetMarkdownText>>(
    &self,
    get_markdown_text: C,
  ) -> RTDResult<()> {
    self.api.send(get_markdown_text.as_ref())
  }

  pub fn get_me<C: AsRef<GetMe>>(&self, get_me: C) -> RTDResult<()> {
    self.api.send(get_me.as_ref())
  }

  pub fn get_menu_button<C: AsRef<GetMenuButton>>(&self, get_menu_button: C) -> RTDResult<()> {
    self.api.send(get_menu_button.as_ref())
  }

  pub fn get_message<C: AsRef<GetMessage>>(&self, get_message: C) -> RTDResult<()> {
    self.api.send(get_message.as_ref())
  }

  pub fn get_message_added_reactions<C: AsRef<GetMessageAddedReactions>>(
    &self,
    get_message_added_reactions: C,
  ) -> RTDResult<()> {
    self.api.send(get_message_added_reactions.as_ref())
  }

  pub fn get_message_author<C: AsRef<GetMessageAuthor>>(
    &self,
    get_message_author: C,
  ) -> RTDResult<()> {
    self.api.send(get_message_author.as_ref())
  }

  pub fn get_message_available_reactions<C: AsRef<GetMessageAvailableReactions>>(
    &self,
    get_message_available_reactions: C,
  ) -> RTDResult<()> {
    self.api.send(get_message_available_reactions.as_ref())
  }

  pub fn get_message_effect<C: AsRef<GetMessageEffect>>(
    &self,
    get_message_effect: C,
  ) -> RTDResult<()> {
    self.api.send(get_message_effect.as_ref())
  }

  pub fn get_message_embedding_code<C: AsRef<GetMessageEmbeddingCode>>(
    &self,
    get_message_embedding_code: C,
  ) -> RTDResult<()> {
    self.api.send(get_message_embedding_code.as_ref())
  }

  pub fn get_message_file_type<C: AsRef<GetMessageFileType>>(
    &self,
    get_message_file_type: C,
  ) -> RTDResult<()> {
    self.api.send(get_message_file_type.as_ref())
  }

  pub fn get_message_import_confirmation_text<C: AsRef<GetMessageImportConfirmationText>>(
    &self,
    get_message_import_confirmation_text: C,
  ) -> RTDResult<()> {
    self.api.send(get_message_import_confirmation_text.as_ref())
  }

  pub fn get_message_link<C: AsRef<GetMessageLink>>(&self, get_message_link: C) -> RTDResult<()> {
    self.api.send(get_message_link.as_ref())
  }

  pub fn get_message_link_info<C: AsRef<GetMessageLinkInfo>>(
    &self,
    get_message_link_info: C,
  ) -> RTDResult<()> {
    self.api.send(get_message_link_info.as_ref())
  }

  pub fn get_message_locally<C: AsRef<GetMessageLocally>>(
    &self,
    get_message_locally: C,
  ) -> RTDResult<()> {
    self.api.send(get_message_locally.as_ref())
  }

  pub fn get_message_properties<C: AsRef<GetMessageProperties>>(
    &self,
    get_message_properties: C,
  ) -> RTDResult<()> {
    self.api.send(get_message_properties.as_ref())
  }

  pub fn get_message_public_forwards<C: AsRef<GetMessagePublicForwards>>(
    &self,
    get_message_public_forwards: C,
  ) -> RTDResult<()> {
    self.api.send(get_message_public_forwards.as_ref())
  }

  pub fn get_message_read_date<C: AsRef<GetMessageReadDate>>(
    &self,
    get_message_read_date: C,
  ) -> RTDResult<()> {
    self.api.send(get_message_read_date.as_ref())
  }

  pub fn get_message_statistics<C: AsRef<GetMessageStatistics>>(
    &self,
    get_message_statistics: C,
  ) -> RTDResult<()> {
    self.api.send(get_message_statistics.as_ref())
  }

  pub fn get_message_thread<C: AsRef<GetMessageThread>>(
    &self,
    get_message_thread: C,
  ) -> RTDResult<()> {
    self.api.send(get_message_thread.as_ref())
  }

  pub fn get_message_thread_history<C: AsRef<GetMessageThreadHistory>>(
    &self,
    get_message_thread_history: C,
  ) -> RTDResult<()> {
    self.api.send(get_message_thread_history.as_ref())
  }

  pub fn get_message_viewers<C: AsRef<GetMessageViewers>>(
    &self,
    get_message_viewers: C,
  ) -> RTDResult<()> {
    self.api.send(get_message_viewers.as_ref())
  }

  pub fn get_messages<C: AsRef<GetMessages>>(&self, get_messages: C) -> RTDResult<()> {
    self.api.send(get_messages.as_ref())
  }

  pub fn get_network_statistics<C: AsRef<GetNetworkStatistics>>(
    &self,
    get_network_statistics: C,
  ) -> RTDResult<()> {
    self.api.send(get_network_statistics.as_ref())
  }

  pub fn get_new_chat_privacy_settings<C: AsRef<GetNewChatPrivacySettings>>(
    &self,
    get_new_chat_privacy_settings: C,
  ) -> RTDResult<()> {
    self.api.send(get_new_chat_privacy_settings.as_ref())
  }

  pub fn get_oauth_link_info<C: AsRef<GetOauthLinkInfo>>(
    &self,
    get_oauth_link_info: C,
  ) -> RTDResult<()> {
    self.api.send(get_oauth_link_info.as_ref())
  }

  pub fn get_on_ramp_payment_availability<C: AsRef<GetOnRampPaymentAvailability>>(
    &self,
    get_on_ramp_payment_availability: C,
  ) -> RTDResult<()> {
    self.api.send(get_on_ramp_payment_availability.as_ref())
  }

  pub fn get_on_ramp_payment_limits<C: AsRef<GetOnRampPaymentLimits>>(
    &self,
    get_on_ramp_payment_limits: C,
  ) -> RTDResult<()> {
    self.api.send(get_on_ramp_payment_limits.as_ref())
  }

  pub fn get_on_ramp_payment_quote<C: AsRef<GetOnRampPaymentQuote>>(
    &self,
    get_on_ramp_payment_quote: C,
  ) -> RTDResult<()> {
    self.api.send(get_on_ramp_payment_quote.as_ref())
  }

  pub fn get_on_ramp_provider_base_currencies<C: AsRef<GetOnRampProviderBaseCurrencies>>(
    &self,
    get_on_ramp_provider_base_currencies: C,
  ) -> RTDResult<()> {
    self.api.send(get_on_ramp_provider_base_currencies.as_ref())
  }

  pub fn get_on_ramp_providers<C: AsRef<GetOnRampProviders>>(
    &self,
    get_on_ramp_providers: C,
  ) -> RTDResult<()> {
    self.api.send(get_on_ramp_providers.as_ref())
  }

  pub fn get_option<C: AsRef<GetOption>>(&self, get_option: C) -> RTDResult<()> {
    self.api.send(get_option.as_ref())
  }

  pub fn get_owned_bots<C: AsRef<GetOwnedBots>>(&self, get_owned_bots: C) -> RTDResult<()> {
    self.api.send(get_owned_bots.as_ref())
  }

  pub fn get_owned_sticker_sets<C: AsRef<GetOwnedStickerSets>>(
    &self,
    get_owned_sticker_sets: C,
  ) -> RTDResult<()> {
    self.api.send(get_owned_sticker_sets.as_ref())
  }

  pub fn get_paid_message_revenue<C: AsRef<GetPaidMessageRevenue>>(
    &self,
    get_paid_message_revenue: C,
  ) -> RTDResult<()> {
    self.api.send(get_paid_message_revenue.as_ref())
  }

  pub fn get_passkey_parameters<C: AsRef<GetPasskeyParameters>>(
    &self,
    get_passkey_parameters: C,
  ) -> RTDResult<()> {
    self.api.send(get_passkey_parameters.as_ref())
  }

  pub fn get_passport_authorization_form<C: AsRef<GetPassportAuthorizationForm>>(
    &self,
    get_passport_authorization_form: C,
  ) -> RTDResult<()> {
    self.api.send(get_passport_authorization_form.as_ref())
  }

  pub fn get_passport_authorization_form_available_elements<
    C: AsRef<GetPassportAuthorizationFormAvailableElements>,
  >(
    &self,
    get_passport_authorization_form_available_elements: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(get_passport_authorization_form_available_elements.as_ref())
  }

  pub fn get_passport_element<C: AsRef<GetPassportElement>>(
    &self,
    get_passport_element: C,
  ) -> RTDResult<()> {
    self.api.send(get_passport_element.as_ref())
  }

  pub fn get_password_state<C: AsRef<GetPasswordState>>(
    &self,
    get_password_state: C,
  ) -> RTDResult<()> {
    self.api.send(get_password_state.as_ref())
  }

  pub fn get_payment_form<C: AsRef<GetPaymentForm>>(&self, get_payment_form: C) -> RTDResult<()> {
    self.api.send(get_payment_form.as_ref())
  }

  pub fn get_payment_receipt<C: AsRef<GetPaymentReceipt>>(
    &self,
    get_payment_receipt: C,
  ) -> RTDResult<()> {
    self.api.send(get_payment_receipt.as_ref())
  }

  pub fn get_personal_chat_history<C: AsRef<GetPersonalChatHistory>>(
    &self,
    get_personal_chat_history: C,
  ) -> RTDResult<()> {
    self.api.send(get_personal_chat_history.as_ref())
  }

  pub fn get_phone_number_info<C: AsRef<GetPhoneNumberInfo>>(
    &self,
    get_phone_number_info: C,
  ) -> RTDResult<()> {
    self.api.send(get_phone_number_info.as_ref())
  }

  pub fn get_phone_number_info_sync<C: AsRef<GetPhoneNumberInfoSync>>(
    &self,
    get_phone_number_info_sync: C,
  ) -> RTDResult<()> {
    self.api.send(get_phone_number_info_sync.as_ref())
  }

  pub fn get_poll_option_properties<C: AsRef<GetPollOptionProperties>>(
    &self,
    get_poll_option_properties: C,
  ) -> RTDResult<()> {
    self.api.send(get_poll_option_properties.as_ref())
  }

  pub fn get_poll_vote_statistics<C: AsRef<GetPollVoteStatistics>>(
    &self,
    get_poll_vote_statistics: C,
  ) -> RTDResult<()> {
    self.api.send(get_poll_vote_statistics.as_ref())
  }

  pub fn get_poll_voters<C: AsRef<GetPollVoters>>(&self, get_poll_voters: C) -> RTDResult<()> {
    self.api.send(get_poll_voters.as_ref())
  }

  pub fn get_preferred_country_language<C: AsRef<GetPreferredCountryLanguage>>(
    &self,
    get_preferred_country_language: C,
  ) -> RTDResult<()> {
    self.api.send(get_preferred_country_language.as_ref())
  }

  pub fn get_premium_features<C: AsRef<GetPremiumFeatures>>(
    &self,
    get_premium_features: C,
  ) -> RTDResult<()> {
    self.api.send(get_premium_features.as_ref())
  }

  pub fn get_premium_gift_payment_options<C: AsRef<GetPremiumGiftPaymentOptions>>(
    &self,
    get_premium_gift_payment_options: C,
  ) -> RTDResult<()> {
    self.api.send(get_premium_gift_payment_options.as_ref())
  }

  pub fn get_premium_giveaway_payment_options<C: AsRef<GetPremiumGiveawayPaymentOptions>>(
    &self,
    get_premium_giveaway_payment_options: C,
  ) -> RTDResult<()> {
    self.api.send(get_premium_giveaway_payment_options.as_ref())
  }

  pub fn get_premium_info_sticker<C: AsRef<GetPremiumInfoSticker>>(
    &self,
    get_premium_info_sticker: C,
  ) -> RTDResult<()> {
    self.api.send(get_premium_info_sticker.as_ref())
  }

  pub fn get_premium_limit<C: AsRef<GetPremiumLimit>>(
    &self,
    get_premium_limit: C,
  ) -> RTDResult<()> {
    self.api.send(get_premium_limit.as_ref())
  }

  pub fn get_premium_state<C: AsRef<GetPremiumState>>(
    &self,
    get_premium_state: C,
  ) -> RTDResult<()> {
    self.api.send(get_premium_state.as_ref())
  }

  pub fn get_premium_sticker_examples<C: AsRef<GetPremiumStickerExamples>>(
    &self,
    get_premium_sticker_examples: C,
  ) -> RTDResult<()> {
    self.api.send(get_premium_sticker_examples.as_ref())
  }

  pub fn get_premium_stickers<C: AsRef<GetPremiumStickers>>(
    &self,
    get_premium_stickers: C,
  ) -> RTDResult<()> {
    self.api.send(get_premium_stickers.as_ref())
  }

  pub fn get_prepared_inline_message<C: AsRef<GetPreparedInlineMessage>>(
    &self,
    get_prepared_inline_message: C,
  ) -> RTDResult<()> {
    self.api.send(get_prepared_inline_message.as_ref())
  }

  pub fn get_prepared_keyboard_button<C: AsRef<GetPreparedKeyboardButton>>(
    &self,
    get_prepared_keyboard_button: C,
  ) -> RTDResult<()> {
    self.api.send(get_prepared_keyboard_button.as_ref())
  }

  pub fn get_profile_share_theme_backdrops<C: AsRef<GetProfileShareThemeBackdrops>>(
    &self,
    get_profile_share_theme_backdrops: C,
  ) -> RTDResult<()> {
    self.api.send(get_profile_share_theme_backdrops.as_ref())
  }

  pub fn get_proxies<C: AsRef<GetProxies>>(&self, get_proxies: C) -> RTDResult<()> {
    self.api.send(get_proxies.as_ref())
  }

  pub fn get_public_post_search_limits<C: AsRef<GetPublicPostSearchLimits>>(
    &self,
    get_public_post_search_limits: C,
  ) -> RTDResult<()> {
    self.api.send(get_public_post_search_limits.as_ref())
  }

  pub fn get_push_receiver_id<C: AsRef<GetPushReceiverId>>(
    &self,
    get_push_receiver_id: C,
  ) -> RTDResult<()> {
    self.api.send(get_push_receiver_id.as_ref())
  }

  pub fn get_read_date_privacy_settings<C: AsRef<GetReadDatePrivacySettings>>(
    &self,
    get_read_date_privacy_settings: C,
  ) -> RTDResult<()> {
    self.api.send(get_read_date_privacy_settings.as_ref())
  }

  pub fn get_received_gift<C: AsRef<GetReceivedGift>>(
    &self,
    get_received_gift: C,
  ) -> RTDResult<()> {
    self.api.send(get_received_gift.as_ref())
  }

  pub fn get_received_gifts<C: AsRef<GetReceivedGifts>>(
    &self,
    get_received_gifts: C,
  ) -> RTDResult<()> {
    self.api.send(get_received_gifts.as_ref())
  }

  pub fn get_recent_emoji_statuses<C: AsRef<GetRecentEmojiStatuses>>(
    &self,
    get_recent_emoji_statuses: C,
  ) -> RTDResult<()> {
    self.api.send(get_recent_emoji_statuses.as_ref())
  }

  pub fn get_recent_inline_bots<C: AsRef<GetRecentInlineBots>>(
    &self,
    get_recent_inline_bots: C,
  ) -> RTDResult<()> {
    self.api.send(get_recent_inline_bots.as_ref())
  }

  pub fn get_recent_stickers<C: AsRef<GetRecentStickers>>(
    &self,
    get_recent_stickers: C,
  ) -> RTDResult<()> {
    self.api.send(get_recent_stickers.as_ref())
  }

  pub fn get_recently_opened_chats<C: AsRef<GetRecentlyOpenedChats>>(
    &self,
    get_recently_opened_chats: C,
  ) -> RTDResult<()> {
    self.api.send(get_recently_opened_chats.as_ref())
  }

  pub fn get_recently_visited_t_me_urls<C: AsRef<GetRecentlyVisitedTMeUrls>>(
    &self,
    get_recently_visited_t_me_urls: C,
  ) -> RTDResult<()> {
    self.api.send(get_recently_visited_t_me_urls.as_ref())
  }

  pub fn get_recommended_chat_folders<C: AsRef<GetRecommendedChatFolders>>(
    &self,
    get_recommended_chat_folders: C,
  ) -> RTDResult<()> {
    self.api.send(get_recommended_chat_folders.as_ref())
  }

  pub fn get_recommended_chats<C: AsRef<GetRecommendedChats>>(
    &self,
    get_recommended_chats: C,
  ) -> RTDResult<()> {
    self.api.send(get_recommended_chats.as_ref())
  }

  pub fn get_recovery_email_address<C: AsRef<GetRecoveryEmailAddress>>(
    &self,
    get_recovery_email_address: C,
  ) -> RTDResult<()> {
    self.api.send(get_recovery_email_address.as_ref())
  }

  pub fn get_remote_file<C: AsRef<GetRemoteFile>>(&self, get_remote_file: C) -> RTDResult<()> {
    self.api.send(get_remote_file.as_ref())
  }

  pub fn get_replied_message<C: AsRef<GetRepliedMessage>>(
    &self,
    get_replied_message: C,
  ) -> RTDResult<()> {
    self.api.send(get_replied_message.as_ref())
  }

  pub fn get_saved_animations<C: AsRef<GetSavedAnimations>>(
    &self,
    get_saved_animations: C,
  ) -> RTDResult<()> {
    self.api.send(get_saved_animations.as_ref())
  }

  pub fn get_saved_messages_tags<C: AsRef<GetSavedMessagesTags>>(
    &self,
    get_saved_messages_tags: C,
  ) -> RTDResult<()> {
    self.api.send(get_saved_messages_tags.as_ref())
  }

  pub fn get_saved_messages_topic_history<C: AsRef<GetSavedMessagesTopicHistory>>(
    &self,
    get_saved_messages_topic_history: C,
  ) -> RTDResult<()> {
    self.api.send(get_saved_messages_topic_history.as_ref())
  }

  pub fn get_saved_messages_topic_message_by_date<C: AsRef<GetSavedMessagesTopicMessageByDate>>(
    &self,
    get_saved_messages_topic_message_by_date: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(get_saved_messages_topic_message_by_date.as_ref())
  }

  pub fn get_saved_notification_sound<C: AsRef<GetSavedNotificationSound>>(
    &self,
    get_saved_notification_sound: C,
  ) -> RTDResult<()> {
    self.api.send(get_saved_notification_sound.as_ref())
  }

  pub fn get_saved_notification_sounds<C: AsRef<GetSavedNotificationSounds>>(
    &self,
    get_saved_notification_sounds: C,
  ) -> RTDResult<()> {
    self.api.send(get_saved_notification_sounds.as_ref())
  }

  pub fn get_saved_order_info<C: AsRef<GetSavedOrderInfo>>(
    &self,
    get_saved_order_info: C,
  ) -> RTDResult<()> {
    self.api.send(get_saved_order_info.as_ref())
  }

  pub fn get_scope_notification_settings<C: AsRef<GetScopeNotificationSettings>>(
    &self,
    get_scope_notification_settings: C,
  ) -> RTDResult<()> {
    self.api.send(get_scope_notification_settings.as_ref())
  }

  pub fn get_search_sponsored_chats<C: AsRef<GetSearchSponsoredChats>>(
    &self,
    get_search_sponsored_chats: C,
  ) -> RTDResult<()> {
    self.api.send(get_search_sponsored_chats.as_ref())
  }

  pub fn get_searched_for_tags<C: AsRef<GetSearchedForTags>>(
    &self,
    get_searched_for_tags: C,
  ) -> RTDResult<()> {
    self.api.send(get_searched_for_tags.as_ref())
  }

  pub fn get_secret_chat<C: AsRef<GetSecretChat>>(&self, get_secret_chat: C) -> RTDResult<()> {
    self.api.send(get_secret_chat.as_ref())
  }

  pub fn get_stake_dice_state<C: AsRef<GetStakeDiceState>>(
    &self,
    get_stake_dice_state: C,
  ) -> RTDResult<()> {
    self.api.send(get_stake_dice_state.as_ref())
  }

  pub fn get_star_ad_account_url<C: AsRef<GetStarAdAccountUrl>>(
    &self,
    get_star_ad_account_url: C,
  ) -> RTDResult<()> {
    self.api.send(get_star_ad_account_url.as_ref())
  }

  pub fn get_star_gift_payment_options<C: AsRef<GetStarGiftPaymentOptions>>(
    &self,
    get_star_gift_payment_options: C,
  ) -> RTDResult<()> {
    self.api.send(get_star_gift_payment_options.as_ref())
  }

  pub fn get_star_giveaway_payment_options<C: AsRef<GetStarGiveawayPaymentOptions>>(
    &self,
    get_star_giveaway_payment_options: C,
  ) -> RTDResult<()> {
    self.api.send(get_star_giveaway_payment_options.as_ref())
  }

  pub fn get_star_payment_options<C: AsRef<GetStarPaymentOptions>>(
    &self,
    get_star_payment_options: C,
  ) -> RTDResult<()> {
    self.api.send(get_star_payment_options.as_ref())
  }

  pub fn get_star_revenue_statistics<C: AsRef<GetStarRevenueStatistics>>(
    &self,
    get_star_revenue_statistics: C,
  ) -> RTDResult<()> {
    self.api.send(get_star_revenue_statistics.as_ref())
  }

  pub fn get_star_subscriptions<C: AsRef<GetStarSubscriptions>>(
    &self,
    get_star_subscriptions: C,
  ) -> RTDResult<()> {
    self.api.send(get_star_subscriptions.as_ref())
  }

  pub fn get_star_transactions<C: AsRef<GetStarTransactions>>(
    &self,
    get_star_transactions: C,
  ) -> RTDResult<()> {
    self.api.send(get_star_transactions.as_ref())
  }

  pub fn get_star_withdrawal_url<C: AsRef<GetStarWithdrawalUrl>>(
    &self,
    get_star_withdrawal_url: C,
  ) -> RTDResult<()> {
    self.api.send(get_star_withdrawal_url.as_ref())
  }

  pub fn get_statistical_graph<C: AsRef<GetStatisticalGraph>>(
    &self,
    get_statistical_graph: C,
  ) -> RTDResult<()> {
    self.api.send(get_statistical_graph.as_ref())
  }

  pub fn get_sticker_emojis<C: AsRef<GetStickerEmojis>>(
    &self,
    get_sticker_emojis: C,
  ) -> RTDResult<()> {
    self.api.send(get_sticker_emojis.as_ref())
  }

  pub fn get_sticker_outline<C: AsRef<GetStickerOutline>>(
    &self,
    get_sticker_outline: C,
  ) -> RTDResult<()> {
    self.api.send(get_sticker_outline.as_ref())
  }

  pub fn get_sticker_outline_svg_path<C: AsRef<GetStickerOutlineSvgPath>>(
    &self,
    get_sticker_outline_svg_path: C,
  ) -> RTDResult<()> {
    self.api.send(get_sticker_outline_svg_path.as_ref())
  }

  pub fn get_sticker_set<C: AsRef<GetStickerSet>>(&self, get_sticker_set: C) -> RTDResult<()> {
    self.api.send(get_sticker_set.as_ref())
  }

  pub fn get_sticker_set_name<C: AsRef<GetStickerSetName>>(
    &self,
    get_sticker_set_name: C,
  ) -> RTDResult<()> {
    self.api.send(get_sticker_set_name.as_ref())
  }

  pub fn get_stickers<C: AsRef<GetStickers>>(&self, get_stickers: C) -> RTDResult<()> {
    self.api.send(get_stickers.as_ref())
  }

  pub fn get_storage_statistics<C: AsRef<GetStorageStatistics>>(
    &self,
    get_storage_statistics: C,
  ) -> RTDResult<()> {
    self.api.send(get_storage_statistics.as_ref())
  }

  pub fn get_storage_statistics_fast<C: AsRef<GetStorageStatisticsFast>>(
    &self,
    get_storage_statistics_fast: C,
  ) -> RTDResult<()> {
    self.api.send(get_storage_statistics_fast.as_ref())
  }

  pub fn get_story<C: AsRef<GetStory>>(&self, get_story: C) -> RTDResult<()> {
    self.api.send(get_story.as_ref())
  }

  pub fn get_story_album_stories<C: AsRef<GetStoryAlbumStories>>(
    &self,
    get_story_album_stories: C,
  ) -> RTDResult<()> {
    self.api.send(get_story_album_stories.as_ref())
  }

  pub fn get_story_available_reactions<C: AsRef<GetStoryAvailableReactions>>(
    &self,
    get_story_available_reactions: C,
  ) -> RTDResult<()> {
    self.api.send(get_story_available_reactions.as_ref())
  }

  pub fn get_story_interactions<C: AsRef<GetStoryInteractions>>(
    &self,
    get_story_interactions: C,
  ) -> RTDResult<()> {
    self.api.send(get_story_interactions.as_ref())
  }

  pub fn get_story_notification_settings_exceptions<
    C: AsRef<GetStoryNotificationSettingsExceptions>,
  >(
    &self,
    get_story_notification_settings_exceptions: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(get_story_notification_settings_exceptions.as_ref())
  }

  pub fn get_story_public_forwards<C: AsRef<GetStoryPublicForwards>>(
    &self,
    get_story_public_forwards: C,
  ) -> RTDResult<()> {
    self.api.send(get_story_public_forwards.as_ref())
  }

  pub fn get_story_statistics<C: AsRef<GetStoryStatistics>>(
    &self,
    get_story_statistics: C,
  ) -> RTDResult<()> {
    self.api.send(get_story_statistics.as_ref())
  }

  pub fn get_suggested_file_name<C: AsRef<GetSuggestedFileName>>(
    &self,
    get_suggested_file_name: C,
  ) -> RTDResult<()> {
    self.api.send(get_suggested_file_name.as_ref())
  }

  pub fn get_suggested_sticker_set_name<C: AsRef<GetSuggestedStickerSetName>>(
    &self,
    get_suggested_sticker_set_name: C,
  ) -> RTDResult<()> {
    self.api.send(get_suggested_sticker_set_name.as_ref())
  }

  pub fn get_suitable_discussion_chats<C: AsRef<GetSuitableDiscussionChats>>(
    &self,
    get_suitable_discussion_chats: C,
  ) -> RTDResult<()> {
    self.api.send(get_suitable_discussion_chats.as_ref())
  }

  pub fn get_suitable_personal_chats<C: AsRef<GetSuitablePersonalChats>>(
    &self,
    get_suitable_personal_chats: C,
  ) -> RTDResult<()> {
    self.api.send(get_suitable_personal_chats.as_ref())
  }

  pub fn get_supergroup<C: AsRef<GetSupergroup>>(&self, get_supergroup: C) -> RTDResult<()> {
    self.api.send(get_supergroup.as_ref())
  }

  pub fn get_supergroup_full_info<C: AsRef<GetSupergroupFullInfo>>(
    &self,
    get_supergroup_full_info: C,
  ) -> RTDResult<()> {
    self.api.send(get_supergroup_full_info.as_ref())
  }

  pub fn get_supergroup_members<C: AsRef<GetSupergroupMembers>>(
    &self,
    get_supergroup_members: C,
  ) -> RTDResult<()> {
    self.api.send(get_supergroup_members.as_ref())
  }

  pub fn get_support_name<C: AsRef<GetSupportName>>(&self, get_support_name: C) -> RTDResult<()> {
    self.api.send(get_support_name.as_ref())
  }

  pub fn get_support_user<C: AsRef<GetSupportUser>>(&self, get_support_user: C) -> RTDResult<()> {
    self.api.send(get_support_user.as_ref())
  }

  pub fn get_temporary_password_state<C: AsRef<GetTemporaryPasswordState>>(
    &self,
    get_temporary_password_state: C,
  ) -> RTDResult<()> {
    self.api.send(get_temporary_password_state.as_ref())
  }

  pub fn get_text_composition_style_example<C: AsRef<GetTextCompositionStyleExample>>(
    &self,
    get_text_composition_style_example: C,
  ) -> RTDResult<()> {
    self.api.send(get_text_composition_style_example.as_ref())
  }

  pub fn get_text_entities<C: AsRef<GetTextEntities>>(
    &self,
    get_text_entities: C,
  ) -> RTDResult<()> {
    self.api.send(get_text_entities.as_ref())
  }

  pub fn get_theme_parameters_json_string<C: AsRef<GetThemeParametersJsonString>>(
    &self,
    get_theme_parameters_json_string: C,
  ) -> RTDResult<()> {
    self.api.send(get_theme_parameters_json_string.as_ref())
  }

  pub fn get_themed_chat_emoji_statuses<C: AsRef<GetThemedChatEmojiStatuses>>(
    &self,
    get_themed_chat_emoji_statuses: C,
  ) -> RTDResult<()> {
    self.api.send(get_themed_chat_emoji_statuses.as_ref())
  }

  pub fn get_themed_emoji_statuses<C: AsRef<GetThemedEmojiStatuses>>(
    &self,
    get_themed_emoji_statuses: C,
  ) -> RTDResult<()> {
    self.api.send(get_themed_emoji_statuses.as_ref())
  }

  pub fn get_time_zones<C: AsRef<GetTimeZones>>(&self, get_time_zones: C) -> RTDResult<()> {
    self.api.send(get_time_zones.as_ref())
  }

  pub fn get_ton_center_streaming_api_url<C: AsRef<GetTonCenterStreamingApiUrl>>(
    &self,
    get_ton_center_streaming_api_url: C,
  ) -> RTDResult<()> {
    self.api.send(get_ton_center_streaming_api_url.as_ref())
  }

  pub fn get_ton_connect_d_app_pending_requests<C: AsRef<GetTonConnectDAppPendingRequests>>(
    &self,
    get_ton_connect_d_app_pending_requests: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(get_ton_connect_d_app_pending_requests.as_ref())
  }

  pub fn get_ton_connect_session_next_event_id<C: AsRef<GetTonConnectSessionNextEventId>>(
    &self,
    get_ton_connect_session_next_event_id: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(get_ton_connect_session_next_event_id.as_ref())
  }

  pub fn get_ton_connect_session_pending_requests<C: AsRef<GetTonConnectSessionPendingRequests>>(
    &self,
    get_ton_connect_session_pending_requests: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(get_ton_connect_session_pending_requests.as_ref())
  }

  pub fn get_ton_connect_sessions<C: AsRef<GetTonConnectSessions>>(
    &self,
    get_ton_connect_sessions: C,
  ) -> RTDResult<()> {
    self.api.send(get_ton_connect_sessions.as_ref())
  }

  pub fn get_ton_transactions<C: AsRef<GetTonTransactions>>(
    &self,
    get_ton_transactions: C,
  ) -> RTDResult<()> {
    self.api.send(get_ton_transactions.as_ref())
  }

  pub fn get_ton_wallet_card_background_gift_collection_names<
    C: AsRef<GetTonWalletCardBackgroundGiftCollectionNames>,
  >(
    &self,
    get_ton_wallet_card_background_gift_collection_names: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(get_ton_wallet_card_background_gift_collection_names.as_ref())
  }

  pub fn get_ton_wallet_nfts<C: AsRef<GetTonWalletNfts>>(
    &self,
    get_ton_wallet_nfts: C,
  ) -> RTDResult<()> {
    self.api.send(get_ton_wallet_nfts.as_ref())
  }

  pub fn get_ton_wallet_ownership_proof_challenge<C: AsRef<GetTonWalletOwnershipProofChallenge>>(
    &self,
    get_ton_wallet_ownership_proof_challenge: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(get_ton_wallet_ownership_proof_challenge.as_ref())
  }

  pub fn get_ton_wallet_secret_phrase<C: AsRef<GetTonWalletSecretPhrase>>(
    &self,
    get_ton_wallet_secret_phrase: C,
  ) -> RTDResult<()> {
    self.api.send(get_ton_wallet_secret_phrase.as_ref())
  }

  pub fn get_ton_wallet_transaction<C: AsRef<GetTonWalletTransaction>>(
    &self,
    get_ton_wallet_transaction: C,
  ) -> RTDResult<()> {
    self.api.send(get_ton_wallet_transaction.as_ref())
  }

  pub fn get_ton_wallet_transaction_by_msg_hash<C: AsRef<GetTonWalletTransactionByMsgHash>>(
    &self,
    get_ton_wallet_transaction_by_msg_hash: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(get_ton_wallet_transaction_by_msg_hash.as_ref())
  }

  pub fn get_ton_wallet_transactions<C: AsRef<GetTonWalletTransactions>>(
    &self,
    get_ton_wallet_transactions: C,
  ) -> RTDResult<()> {
    self.api.send(get_ton_wallet_transactions.as_ref())
  }

  pub fn get_top_chats<C: AsRef<GetTopChats>>(&self, get_top_chats: C) -> RTDResult<()> {
    self.api.send(get_top_chats.as_ref())
  }

  pub fn get_trending_sticker_sets<C: AsRef<GetTrendingStickerSets>>(
    &self,
    get_trending_sticker_sets: C,
  ) -> RTDResult<()> {
    self.api.send(get_trending_sticker_sets.as_ref())
  }

  pub fn get_upgraded_gift<C: AsRef<GetUpgradedGift>>(
    &self,
    get_upgraded_gift: C,
  ) -> RTDResult<()> {
    self.api.send(get_upgraded_gift.as_ref())
  }

  pub fn get_upgraded_gift_deposit_info<C: AsRef<GetUpgradedGiftDepositInfo>>(
    &self,
    get_upgraded_gift_deposit_info: C,
  ) -> RTDResult<()> {
    self.api.send(get_upgraded_gift_deposit_info.as_ref())
  }

  pub fn get_upgraded_gift_emoji_statuses<C: AsRef<GetUpgradedGiftEmojiStatuses>>(
    &self,
    get_upgraded_gift_emoji_statuses: C,
  ) -> RTDResult<()> {
    self.api.send(get_upgraded_gift_emoji_statuses.as_ref())
  }

  pub fn get_upgraded_gift_value_info<C: AsRef<GetUpgradedGiftValueInfo>>(
    &self,
    get_upgraded_gift_value_info: C,
  ) -> RTDResult<()> {
    self.api.send(get_upgraded_gift_value_info.as_ref())
  }

  pub fn get_upgraded_gift_variants<C: AsRef<GetUpgradedGiftVariants>>(
    &self,
    get_upgraded_gift_variants: C,
  ) -> RTDResult<()> {
    self.api.send(get_upgraded_gift_variants.as_ref())
  }

  pub fn get_upgraded_gift_withdrawal_url<C: AsRef<GetUpgradedGiftWithdrawalUrl>>(
    &self,
    get_upgraded_gift_withdrawal_url: C,
  ) -> RTDResult<()> {
    self.api.send(get_upgraded_gift_withdrawal_url.as_ref())
  }

  pub fn get_upgraded_gifts_promotional_animation<
    C: AsRef<GetUpgradedGiftsPromotionalAnimation>,
  >(
    &self,
    get_upgraded_gifts_promotional_animation: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(get_upgraded_gifts_promotional_animation.as_ref())
  }

  pub fn get_user<C: AsRef<GetUser>>(&self, get_user: C) -> RTDResult<()> {
    self.api.send(get_user.as_ref())
  }

  pub fn get_user_chat_boosts<C: AsRef<GetUserChatBoosts>>(
    &self,
    get_user_chat_boosts: C,
  ) -> RTDResult<()> {
    self.api.send(get_user_chat_boosts.as_ref())
  }

  pub fn get_user_full_info<C: AsRef<GetUserFullInfo>>(
    &self,
    get_user_full_info: C,
  ) -> RTDResult<()> {
    self.api.send(get_user_full_info.as_ref())
  }

  pub fn get_user_link<C: AsRef<GetUserLink>>(&self, get_user_link: C) -> RTDResult<()> {
    self.api.send(get_user_link.as_ref())
  }

  pub fn get_user_privacy_setting_rules<C: AsRef<GetUserPrivacySettingRules>>(
    &self,
    get_user_privacy_setting_rules: C,
  ) -> RTDResult<()> {
    self.api.send(get_user_privacy_setting_rules.as_ref())
  }

  pub fn get_user_profile_audios<C: AsRef<GetUserProfileAudios>>(
    &self,
    get_user_profile_audios: C,
  ) -> RTDResult<()> {
    self.api.send(get_user_profile_audios.as_ref())
  }

  pub fn get_user_profile_photos<C: AsRef<GetUserProfilePhotos>>(
    &self,
    get_user_profile_photos: C,
  ) -> RTDResult<()> {
    self.api.send(get_user_profile_photos.as_ref())
  }

  pub fn get_user_support_info<C: AsRef<GetUserSupportInfo>>(
    &self,
    get_user_support_info: C,
  ) -> RTDResult<()> {
    self.api.send(get_user_support_info.as_ref())
  }

  pub fn get_user_ton_wallet_addresses<C: AsRef<GetUserTonWalletAddresses>>(
    &self,
    get_user_ton_wallet_addresses: C,
  ) -> RTDResult<()> {
    self.api.send(get_user_ton_wallet_addresses.as_ref())
  }

  pub fn get_video_chat_available_participants<C: AsRef<GetVideoChatAvailableParticipants>>(
    &self,
    get_video_chat_available_participants: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(get_video_chat_available_participants.as_ref())
  }

  pub fn get_video_chat_invite_link<C: AsRef<GetVideoChatInviteLink>>(
    &self,
    get_video_chat_invite_link: C,
  ) -> RTDResult<()> {
    self.api.send(get_video_chat_invite_link.as_ref())
  }

  pub fn get_video_chat_rtmp_url<C: AsRef<GetVideoChatRtmpUrl>>(
    &self,
    get_video_chat_rtmp_url: C,
  ) -> RTDResult<()> {
    self.api.send(get_video_chat_rtmp_url.as_ref())
  }

  pub fn get_video_message_advertisements<C: AsRef<GetVideoMessageAdvertisements>>(
    &self,
    get_video_message_advertisements: C,
  ) -> RTDResult<()> {
    self.api.send(get_video_message_advertisements.as_ref())
  }

  pub fn get_web_app_link_url<C: AsRef<GetWebAppLinkUrl>>(
    &self,
    get_web_app_link_url: C,
  ) -> RTDResult<()> {
    self.api.send(get_web_app_link_url.as_ref())
  }

  pub fn get_web_app_placeholder<C: AsRef<GetWebAppPlaceholder>>(
    &self,
    get_web_app_placeholder: C,
  ) -> RTDResult<()> {
    self.api.send(get_web_app_placeholder.as_ref())
  }

  pub fn get_web_app_url<C: AsRef<GetWebAppUrl>>(&self, get_web_app_url: C) -> RTDResult<()> {
    self.api.send(get_web_app_url.as_ref())
  }

  pub fn get_web_page_instant_view<C: AsRef<GetWebPageInstantView>>(
    &self,
    get_web_page_instant_view: C,
  ) -> RTDResult<()> {
    self.api.send(get_web_page_instant_view.as_ref())
  }

  pub fn gift_premium_with_stars<C: AsRef<GiftPremiumWithStars>>(
    &self,
    gift_premium_with_stars: C,
  ) -> RTDResult<()> {
    self.api.send(gift_premium_with_stars.as_ref())
  }

  pub fn hide_contact_close_birthdays<C: AsRef<HideContactCloseBirthdays>>(
    &self,
    hide_contact_close_birthdays: C,
  ) -> RTDResult<()> {
    self.api.send(hide_contact_close_birthdays.as_ref())
  }

  pub fn hide_suggested_action<C: AsRef<HideSuggestedAction>>(
    &self,
    hide_suggested_action: C,
  ) -> RTDResult<()> {
    self.api.send(hide_suggested_action.as_ref())
  }

  pub fn import_contacts<C: AsRef<ImportContacts>>(&self, import_contacts: C) -> RTDResult<()> {
    self.api.send(import_contacts.as_ref())
  }

  pub fn import_messages<C: AsRef<ImportMessages>>(&self, import_messages: C) -> RTDResult<()> {
    self.api.send(import_messages.as_ref())
  }

  pub fn increase_gift_auction_bid<C: AsRef<IncreaseGiftAuctionBid>>(
    &self,
    increase_gift_auction_bid: C,
  ) -> RTDResult<()> {
    self.api.send(increase_gift_auction_bid.as_ref())
  }

  pub fn invite_group_call_participant<C: AsRef<InviteGroupCallParticipant>>(
    &self,
    invite_group_call_participant: C,
  ) -> RTDResult<()> {
    self.api.send(invite_group_call_participant.as_ref())
  }

  pub fn invite_video_chat_participants<C: AsRef<InviteVideoChatParticipants>>(
    &self,
    invite_video_chat_participants: C,
  ) -> RTDResult<()> {
    self.api.send(invite_video_chat_participants.as_ref())
  }

  pub fn is_login_email_address_required<C: AsRef<IsLoginEmailAddressRequired>>(
    &self,
    is_login_email_address_required: C,
  ) -> RTDResult<()> {
    self.api.send(is_login_email_address_required.as_ref())
  }

  pub fn is_profile_audio<C: AsRef<IsProfileAudio>>(&self, is_profile_audio: C) -> RTDResult<()> {
    self.api.send(is_profile_audio.as_ref())
  }

  pub fn join_chat<C: AsRef<JoinChat>>(&self, join_chat: C) -> RTDResult<()> {
    self.api.send(join_chat.as_ref())
  }

  pub fn join_chat_by_invite_link<C: AsRef<JoinChatByInviteLink>>(
    &self,
    join_chat_by_invite_link: C,
  ) -> RTDResult<()> {
    self.api.send(join_chat_by_invite_link.as_ref())
  }

  pub fn join_group_call<C: AsRef<JoinGroupCall>>(&self, join_group_call: C) -> RTDResult<()> {
    self.api.send(join_group_call.as_ref())
  }

  pub fn join_live_story<C: AsRef<JoinLiveStory>>(&self, join_live_story: C) -> RTDResult<()> {
    self.api.send(join_live_story.as_ref())
  }

  pub fn join_video_chat<C: AsRef<JoinVideoChat>>(&self, join_video_chat: C) -> RTDResult<()> {
    self.api.send(join_video_chat.as_ref())
  }

  pub fn launch_prepaid_giveaway<C: AsRef<LaunchPrepaidGiveaway>>(
    &self,
    launch_prepaid_giveaway: C,
  ) -> RTDResult<()> {
    self.api.send(launch_prepaid_giveaway.as_ref())
  }

  pub fn leave_chat<C: AsRef<LeaveChat>>(&self, leave_chat: C) -> RTDResult<()> {
    self.api.send(leave_chat.as_ref())
  }

  pub fn leave_group_call<C: AsRef<LeaveGroupCall>>(&self, leave_group_call: C) -> RTDResult<()> {
    self.api.send(leave_group_call.as_ref())
  }

  pub fn listen_to_audio<C: AsRef<ListenToAudio>>(&self, listen_to_audio: C) -> RTDResult<()> {
    self.api.send(listen_to_audio.as_ref())
  }

  pub fn load_active_stories<C: AsRef<LoadActiveStories>>(
    &self,
    load_active_stories: C,
  ) -> RTDResult<()> {
    self.api.send(load_active_stories.as_ref())
  }

  pub fn load_chat_welcome_messages<C: AsRef<LoadChatWelcomeMessages>>(
    &self,
    load_chat_welcome_messages: C,
  ) -> RTDResult<()> {
    self.api.send(load_chat_welcome_messages.as_ref())
  }

  pub fn load_chats<C: AsRef<LoadChats>>(&self, load_chats: C) -> RTDResult<()> {
    self.api.send(load_chats.as_ref())
  }

  pub fn load_direct_messages_chat_topics<C: AsRef<LoadDirectMessagesChatTopics>>(
    &self,
    load_direct_messages_chat_topics: C,
  ) -> RTDResult<()> {
    self.api.send(load_direct_messages_chat_topics.as_ref())
  }

  pub fn load_group_call_participants<C: AsRef<LoadGroupCallParticipants>>(
    &self,
    load_group_call_participants: C,
  ) -> RTDResult<()> {
    self.api.send(load_group_call_participants.as_ref())
  }

  pub fn load_quick_reply_shortcut_messages<C: AsRef<LoadQuickReplyShortcutMessages>>(
    &self,
    load_quick_reply_shortcut_messages: C,
  ) -> RTDResult<()> {
    self.api.send(load_quick_reply_shortcut_messages.as_ref())
  }

  pub fn load_quick_reply_shortcuts<C: AsRef<LoadQuickReplyShortcuts>>(
    &self,
    load_quick_reply_shortcuts: C,
  ) -> RTDResult<()> {
    self.api.send(load_quick_reply_shortcuts.as_ref())
  }

  pub fn load_saved_messages_topics<C: AsRef<LoadSavedMessagesTopics>>(
    &self,
    load_saved_messages_topics: C,
  ) -> RTDResult<()> {
    self.api.send(load_saved_messages_topics.as_ref())
  }

  pub fn load_ton_wallet_gasless_transfers_info<C: AsRef<LoadTonWalletGaslessTransfersInfo>>(
    &self,
    load_ton_wallet_gasless_transfers_info: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(load_ton_wallet_gasless_transfers_info.as_ref())
  }

  pub fn load_ton_wallet_state<C: AsRef<LoadTonWalletState>>(
    &self,
    load_ton_wallet_state: C,
  ) -> RTDResult<()> {
    self.api.send(load_ton_wallet_state.as_ref())
  }

  pub fn log_out<C: AsRef<LogOut>>(&self, log_out: C) -> RTDResult<()> {
    self.api.send(log_out.as_ref())
  }

  pub fn mark_checklist_tasks_as_done<C: AsRef<MarkChecklistTasksAsDone>>(
    &self,
    mark_checklist_tasks_as_done: C,
  ) -> RTDResult<()> {
    self.api.send(mark_checklist_tasks_as_done.as_ref())
  }

  pub fn open_bot_similar_bot<C: AsRef<OpenBotSimilarBot>>(
    &self,
    open_bot_similar_bot: C,
  ) -> RTDResult<()> {
    self.api.send(open_bot_similar_bot.as_ref())
  }

  pub fn open_chat<C: AsRef<OpenChat>>(&self, open_chat: C) -> RTDResult<()> {
    self.api.send(open_chat.as_ref())
  }

  pub fn open_chat_similar_chat<C: AsRef<OpenChatSimilarChat>>(
    &self,
    open_chat_similar_chat: C,
  ) -> RTDResult<()> {
    self.api.send(open_chat_similar_chat.as_ref())
  }

  pub fn open_gift_auction<C: AsRef<OpenGiftAuction>>(
    &self,
    open_gift_auction: C,
  ) -> RTDResult<()> {
    self.api.send(open_gift_auction.as_ref())
  }

  pub fn open_message_content<C: AsRef<OpenMessageContent>>(
    &self,
    open_message_content: C,
  ) -> RTDResult<()> {
    self.api.send(open_message_content.as_ref())
  }

  pub fn open_sponsored_chat<C: AsRef<OpenSponsoredChat>>(
    &self,
    open_sponsored_chat: C,
  ) -> RTDResult<()> {
    self.api.send(open_sponsored_chat.as_ref())
  }

  pub fn open_story<C: AsRef<OpenStory>>(&self, open_story: C) -> RTDResult<()> {
    self.api.send(open_story.as_ref())
  }

  pub fn open_web_app<C: AsRef<OpenWebApp>>(&self, open_web_app: C) -> RTDResult<()> {
    self.api.send(open_web_app.as_ref())
  }

  pub fn optimize_storage<C: AsRef<OptimizeStorage>>(&self, optimize_storage: C) -> RTDResult<()> {
    self.api.send(optimize_storage.as_ref())
  }

  pub fn parse_markdown<C: AsRef<ParseMarkdown>>(&self, parse_markdown: C) -> RTDResult<()> {
    self.api.send(parse_markdown.as_ref())
  }

  pub fn parse_text_entities<C: AsRef<ParseTextEntities>>(
    &self,
    parse_text_entities: C,
  ) -> RTDResult<()> {
    self.api.send(parse_text_entities.as_ref())
  }

  pub fn pin_chat_message<C: AsRef<PinChatMessage>>(&self, pin_chat_message: C) -> RTDResult<()> {
    self.api.send(pin_chat_message.as_ref())
  }

  pub fn ping_proxy<C: AsRef<PingProxy>>(&self, ping_proxy: C) -> RTDResult<()> {
    self.api.send(ping_proxy.as_ref())
  }

  pub fn place_gift_auction_bid<C: AsRef<PlaceGiftAuctionBid>>(
    &self,
    place_gift_auction_bid: C,
  ) -> RTDResult<()> {
    self.api.send(place_gift_auction_bid.as_ref())
  }

  pub fn post_story<C: AsRef<PostStory>>(&self, post_story: C) -> RTDResult<()> {
    self.api.send(post_story.as_ref())
  }

  pub fn preliminary_upload_file<C: AsRef<PreliminaryUploadFile>>(
    &self,
    preliminary_upload_file: C,
  ) -> RTDResult<()> {
    self.api.send(preliminary_upload_file.as_ref())
  }

  pub fn process_chat_folder_new_chats<C: AsRef<ProcessChatFolderNewChats>>(
    &self,
    process_chat_folder_new_chats: C,
  ) -> RTDResult<()> {
    self.api.send(process_chat_folder_new_chats.as_ref())
  }

  pub fn process_chat_has_protected_content_disable_request<
    C: AsRef<ProcessChatHasProtectedContentDisableRequest>,
  >(
    &self,
    process_chat_has_protected_content_disable_request: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(process_chat_has_protected_content_disable_request.as_ref())
  }

  pub fn process_chat_join_request<C: AsRef<ProcessChatJoinRequest>>(
    &self,
    process_chat_join_request: C,
  ) -> RTDResult<()> {
    self.api.send(process_chat_join_request.as_ref())
  }

  pub fn process_chat_join_requests<C: AsRef<ProcessChatJoinRequests>>(
    &self,
    process_chat_join_requests: C,
  ) -> RTDResult<()> {
    self.api.send(process_chat_join_requests.as_ref())
  }

  pub fn process_gift_purchase_offer<C: AsRef<ProcessGiftPurchaseOffer>>(
    &self,
    process_gift_purchase_offer: C,
  ) -> RTDResult<()> {
    self.api.send(process_gift_purchase_offer.as_ref())
  }

  pub fn process_push_notification<C: AsRef<ProcessPushNotification>>(
    &self,
    process_push_notification: C,
  ) -> RTDResult<()> {
    self.api.send(process_push_notification.as_ref())
  }

  pub fn process_web_proxy_carrier_message<C: AsRef<ProcessWebProxyCarrierMessage>>(
    &self,
    process_web_proxy_carrier_message: C,
  ) -> RTDResult<()> {
    self.api.send(process_web_proxy_carrier_message.as_ref())
  }

  pub fn rate_speech_recognition<C: AsRef<RateSpeechRecognition>>(
    &self,
    rate_speech_recognition: C,
  ) -> RTDResult<()> {
    self.api.send(rate_speech_recognition.as_ref())
  }

  pub fn read_all_chat_mentions<C: AsRef<ReadAllChatMentions>>(
    &self,
    read_all_chat_mentions: C,
  ) -> RTDResult<()> {
    self.api.send(read_all_chat_mentions.as_ref())
  }

  pub fn read_all_chat_poll_votes<C: AsRef<ReadAllChatPollVotes>>(
    &self,
    read_all_chat_poll_votes: C,
  ) -> RTDResult<()> {
    self.api.send(read_all_chat_poll_votes.as_ref())
  }

  pub fn read_all_chat_reactions<C: AsRef<ReadAllChatReactions>>(
    &self,
    read_all_chat_reactions: C,
  ) -> RTDResult<()> {
    self.api.send(read_all_chat_reactions.as_ref())
  }

  pub fn read_all_direct_messages_chat_topic_reactions<
    C: AsRef<ReadAllDirectMessagesChatTopicReactions>,
  >(
    &self,
    read_all_direct_messages_chat_topic_reactions: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(read_all_direct_messages_chat_topic_reactions.as_ref())
  }

  pub fn read_all_forum_topic_mentions<C: AsRef<ReadAllForumTopicMentions>>(
    &self,
    read_all_forum_topic_mentions: C,
  ) -> RTDResult<()> {
    self.api.send(read_all_forum_topic_mentions.as_ref())
  }

  pub fn read_all_forum_topic_poll_votes<C: AsRef<ReadAllForumTopicPollVotes>>(
    &self,
    read_all_forum_topic_poll_votes: C,
  ) -> RTDResult<()> {
    self.api.send(read_all_forum_topic_poll_votes.as_ref())
  }

  pub fn read_all_forum_topic_reactions<C: AsRef<ReadAllForumTopicReactions>>(
    &self,
    read_all_forum_topic_reactions: C,
  ) -> RTDResult<()> {
    self.api.send(read_all_forum_topic_reactions.as_ref())
  }

  pub fn read_business_message<C: AsRef<ReadBusinessMessage>>(
    &self,
    read_business_message: C,
  ) -> RTDResult<()> {
    self.api.send(read_business_message.as_ref())
  }

  pub fn read_chat_list<C: AsRef<ReadChatList>>(&self, read_chat_list: C) -> RTDResult<()> {
    self.api.send(read_chat_list.as_ref())
  }

  pub fn read_file_part<C: AsRef<ReadFilePart>>(&self, read_file_part: C) -> RTDResult<()> {
    self.api.send(read_file_part.as_ref())
  }

  pub fn readd_quick_reply_shortcut_messages<C: AsRef<ReaddQuickReplyShortcutMessages>>(
    &self,
    readd_quick_reply_shortcut_messages: C,
  ) -> RTDResult<()> {
    self.api.send(readd_quick_reply_shortcut_messages.as_ref())
  }

  pub fn recognize_speech<C: AsRef<RecognizeSpeech>>(&self, recognize_speech: C) -> RTDResult<()> {
    self.api.send(recognize_speech.as_ref())
  }

  pub fn recover_authentication_password<C: AsRef<RecoverAuthenticationPassword>>(
    &self,
    recover_authentication_password: C,
  ) -> RTDResult<()> {
    self.api.send(recover_authentication_password.as_ref())
  }

  pub fn recover_password<C: AsRef<RecoverPassword>>(&self, recover_password: C) -> RTDResult<()> {
    self.api.send(recover_password.as_ref())
  }

  pub fn refund_star_payment<C: AsRef<RefundStarPayment>>(
    &self,
    refund_star_payment: C,
  ) -> RTDResult<()> {
    self.api.send(refund_star_payment.as_ref())
  }

  pub fn register_device<C: AsRef<RegisterDevice>>(&self, register_device: C) -> RTDResult<()> {
    self.api.send(register_device.as_ref())
  }

  pub fn register_user<C: AsRef<RegisterUser>>(&self, register_user: C) -> RTDResult<()> {
    self.api.send(register_user.as_ref())
  }

  pub fn remove_all_files_from_downloads<C: AsRef<RemoveAllFilesFromDownloads>>(
    &self,
    remove_all_files_from_downloads: C,
  ) -> RTDResult<()> {
    self.api.send(remove_all_files_from_downloads.as_ref())
  }

  pub fn remove_all_web_browser_settings_exceptions<
    C: AsRef<RemoveAllWebBrowserSettingsExceptions>,
  >(
    &self,
    remove_all_web_browser_settings_exceptions: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(remove_all_web_browser_settings_exceptions.as_ref())
  }

  pub fn remove_business_connected_bot_from_chat<C: AsRef<RemoveBusinessConnectedBotFromChat>>(
    &self,
    remove_business_connected_bot_from_chat: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(remove_business_connected_bot_from_chat.as_ref())
  }

  pub fn remove_chat_action_bar<C: AsRef<RemoveChatActionBar>>(
    &self,
    remove_chat_action_bar: C,
  ) -> RTDResult<()> {
    self.api.send(remove_chat_action_bar.as_ref())
  }

  pub fn remove_contacts<C: AsRef<RemoveContacts>>(&self, remove_contacts: C) -> RTDResult<()> {
    self.api.send(remove_contacts.as_ref())
  }

  pub fn remove_favorite_sticker<C: AsRef<RemoveFavoriteSticker>>(
    &self,
    remove_favorite_sticker: C,
  ) -> RTDResult<()> {
    self.api.send(remove_favorite_sticker.as_ref())
  }

  pub fn remove_file_from_downloads<C: AsRef<RemoveFileFromDownloads>>(
    &self,
    remove_file_from_downloads: C,
  ) -> RTDResult<()> {
    self.api.send(remove_file_from_downloads.as_ref())
  }

  pub fn remove_gift_collection_gifts<C: AsRef<RemoveGiftCollectionGifts>>(
    &self,
    remove_gift_collection_gifts: C,
  ) -> RTDResult<()> {
    self.api.send(remove_gift_collection_gifts.as_ref())
  }

  pub fn remove_installed_background<C: AsRef<RemoveInstalledBackground>>(
    &self,
    remove_installed_background: C,
  ) -> RTDResult<()> {
    self.api.send(remove_installed_background.as_ref())
  }

  pub fn remove_login_passkey<C: AsRef<RemoveLoginPasskey>>(
    &self,
    remove_login_passkey: C,
  ) -> RTDResult<()> {
    self.api.send(remove_login_passkey.as_ref())
  }

  pub fn remove_message_reaction<C: AsRef<RemoveMessageReaction>>(
    &self,
    remove_message_reaction: C,
  ) -> RTDResult<()> {
    self.api.send(remove_message_reaction.as_ref())
  }

  pub fn remove_message_sender_bot_verification<C: AsRef<RemoveMessageSenderBotVerification>>(
    &self,
    remove_message_sender_bot_verification: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(remove_message_sender_bot_verification.as_ref())
  }

  pub fn remove_notification<C: AsRef<RemoveNotification>>(
    &self,
    remove_notification: C,
  ) -> RTDResult<()> {
    self.api.send(remove_notification.as_ref())
  }

  pub fn remove_notification_group<C: AsRef<RemoveNotificationGroup>>(
    &self,
    remove_notification_group: C,
  ) -> RTDResult<()> {
    self.api.send(remove_notification_group.as_ref())
  }

  pub fn remove_pending_live_story_reactions<C: AsRef<RemovePendingLiveStoryReactions>>(
    &self,
    remove_pending_live_story_reactions: C,
  ) -> RTDResult<()> {
    self.api.send(remove_pending_live_story_reactions.as_ref())
  }

  pub fn remove_pending_paid_message_reactions<C: AsRef<RemovePendingPaidMessageReactions>>(
    &self,
    remove_pending_paid_message_reactions: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(remove_pending_paid_message_reactions.as_ref())
  }

  pub fn remove_profile_audio<C: AsRef<RemoveProfileAudio>>(
    &self,
    remove_profile_audio: C,
  ) -> RTDResult<()> {
    self.api.send(remove_profile_audio.as_ref())
  }

  pub fn remove_proxy<C: AsRef<RemoveProxy>>(&self, remove_proxy: C) -> RTDResult<()> {
    self.api.send(remove_proxy.as_ref())
  }

  pub fn remove_recent_hashtag<C: AsRef<RemoveRecentHashtag>>(
    &self,
    remove_recent_hashtag: C,
  ) -> RTDResult<()> {
    self.api.send(remove_recent_hashtag.as_ref())
  }

  pub fn remove_recent_sticker<C: AsRef<RemoveRecentSticker>>(
    &self,
    remove_recent_sticker: C,
  ) -> RTDResult<()> {
    self.api.send(remove_recent_sticker.as_ref())
  }

  pub fn remove_recently_found_chat<C: AsRef<RemoveRecentlyFoundChat>>(
    &self,
    remove_recently_found_chat: C,
  ) -> RTDResult<()> {
    self.api.send(remove_recently_found_chat.as_ref())
  }

  pub fn remove_saved_animation<C: AsRef<RemoveSavedAnimation>>(
    &self,
    remove_saved_animation: C,
  ) -> RTDResult<()> {
    self.api.send(remove_saved_animation.as_ref())
  }

  pub fn remove_saved_notification_sound<C: AsRef<RemoveSavedNotificationSound>>(
    &self,
    remove_saved_notification_sound: C,
  ) -> RTDResult<()> {
    self.api.send(remove_saved_notification_sound.as_ref())
  }

  pub fn remove_searched_for_tag<C: AsRef<RemoveSearchedForTag>>(
    &self,
    remove_searched_for_tag: C,
  ) -> RTDResult<()> {
    self.api.send(remove_searched_for_tag.as_ref())
  }

  pub fn remove_sticker_from_set<C: AsRef<RemoveStickerFromSet>>(
    &self,
    remove_sticker_from_set: C,
  ) -> RTDResult<()> {
    self.api.send(remove_sticker_from_set.as_ref())
  }

  pub fn remove_story_album_stories<C: AsRef<RemoveStoryAlbumStories>>(
    &self,
    remove_story_album_stories: C,
  ) -> RTDResult<()> {
    self.api.send(remove_story_album_stories.as_ref())
  }

  pub fn remove_text_composition_style<C: AsRef<RemoveTextCompositionStyle>>(
    &self,
    remove_text_composition_style: C,
  ) -> RTDResult<()> {
    self.api.send(remove_text_composition_style.as_ref())
  }

  pub fn remove_top_chat<C: AsRef<RemoveTopChat>>(&self, remove_top_chat: C) -> RTDResult<()> {
    self.api.send(remove_top_chat.as_ref())
  }

  pub fn remove_web_browser_settings_exception<C: AsRef<RemoveWebBrowserSettingsException>>(
    &self,
    remove_web_browser_settings_exception: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(remove_web_browser_settings_exception.as_ref())
  }

  pub fn reorder_active_usernames<C: AsRef<ReorderActiveUsernames>>(
    &self,
    reorder_active_usernames: C,
  ) -> RTDResult<()> {
    self.api.send(reorder_active_usernames.as_ref())
  }

  pub fn reorder_bot_active_usernames<C: AsRef<ReorderBotActiveUsernames>>(
    &self,
    reorder_bot_active_usernames: C,
  ) -> RTDResult<()> {
    self.api.send(reorder_bot_active_usernames.as_ref())
  }

  pub fn reorder_bot_media_previews<C: AsRef<ReorderBotMediaPreviews>>(
    &self,
    reorder_bot_media_previews: C,
  ) -> RTDResult<()> {
    self.api.send(reorder_bot_media_previews.as_ref())
  }

  pub fn reorder_chat_folders<C: AsRef<ReorderChatFolders>>(
    &self,
    reorder_chat_folders: C,
  ) -> RTDResult<()> {
    self.api.send(reorder_chat_folders.as_ref())
  }

  pub fn reorder_gift_collection_gifts<C: AsRef<ReorderGiftCollectionGifts>>(
    &self,
    reorder_gift_collection_gifts: C,
  ) -> RTDResult<()> {
    self.api.send(reorder_gift_collection_gifts.as_ref())
  }

  pub fn reorder_gift_collections<C: AsRef<ReorderGiftCollections>>(
    &self,
    reorder_gift_collections: C,
  ) -> RTDResult<()> {
    self.api.send(reorder_gift_collections.as_ref())
  }

  pub fn reorder_installed_sticker_sets<C: AsRef<ReorderInstalledStickerSets>>(
    &self,
    reorder_installed_sticker_sets: C,
  ) -> RTDResult<()> {
    self.api.send(reorder_installed_sticker_sets.as_ref())
  }

  pub fn reorder_quick_reply_shortcuts<C: AsRef<ReorderQuickReplyShortcuts>>(
    &self,
    reorder_quick_reply_shortcuts: C,
  ) -> RTDResult<()> {
    self.api.send(reorder_quick_reply_shortcuts.as_ref())
  }

  pub fn reorder_story_album_stories<C: AsRef<ReorderStoryAlbumStories>>(
    &self,
    reorder_story_album_stories: C,
  ) -> RTDResult<()> {
    self.api.send(reorder_story_album_stories.as_ref())
  }

  pub fn reorder_story_albums<C: AsRef<ReorderStoryAlbums>>(
    &self,
    reorder_story_albums: C,
  ) -> RTDResult<()> {
    self.api.send(reorder_story_albums.as_ref())
  }

  pub fn reorder_supergroup_active_usernames<C: AsRef<ReorderSupergroupActiveUsernames>>(
    &self,
    reorder_supergroup_active_usernames: C,
  ) -> RTDResult<()> {
    self.api.send(reorder_supergroup_active_usernames.as_ref())
  }

  pub fn replace_live_story_rtmp_url<C: AsRef<ReplaceLiveStoryRtmpUrl>>(
    &self,
    replace_live_story_rtmp_url: C,
  ) -> RTDResult<()> {
    self.api.send(replace_live_story_rtmp_url.as_ref())
  }

  pub fn replace_primary_chat_invite_link<C: AsRef<ReplacePrimaryChatInviteLink>>(
    &self,
    replace_primary_chat_invite_link: C,
  ) -> RTDResult<()> {
    self.api.send(replace_primary_chat_invite_link.as_ref())
  }

  pub fn replace_sticker_in_set<C: AsRef<ReplaceStickerInSet>>(
    &self,
    replace_sticker_in_set: C,
  ) -> RTDResult<()> {
    self.api.send(replace_sticker_in_set.as_ref())
  }

  pub fn replace_ton_wallet<C: AsRef<ReplaceTonWallet>>(
    &self,
    replace_ton_wallet: C,
  ) -> RTDResult<()> {
    self.api.send(replace_ton_wallet.as_ref())
  }

  pub fn replace_video_chat_rtmp_url<C: AsRef<ReplaceVideoChatRtmpUrl>>(
    &self,
    replace_video_chat_rtmp_url: C,
  ) -> RTDResult<()> {
    self.api.send(replace_video_chat_rtmp_url.as_ref())
  }

  pub fn reply_to_callback_query_with_ephemeral_message<
    C: AsRef<ReplyToCallbackQueryWithEphemeralMessage>,
  >(
    &self,
    reply_to_callback_query_with_ephemeral_message: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(reply_to_callback_query_with_ephemeral_message.as_ref())
  }

  pub fn report_authentication_code_missing<C: AsRef<ReportAuthenticationCodeMissing>>(
    &self,
    report_authentication_code_missing: C,
  ) -> RTDResult<()> {
    self.api.send(report_authentication_code_missing.as_ref())
  }

  pub fn report_chat<C: AsRef<ReportChat>>(&self, report_chat: C) -> RTDResult<()> {
    self.api.send(report_chat.as_ref())
  }

  pub fn report_chat_photo<C: AsRef<ReportChatPhoto>>(
    &self,
    report_chat_photo: C,
  ) -> RTDResult<()> {
    self.api.send(report_chat_photo.as_ref())
  }

  pub fn report_chat_sponsored_message<C: AsRef<ReportChatSponsoredMessage>>(
    &self,
    report_chat_sponsored_message: C,
  ) -> RTDResult<()> {
    self.api.send(report_chat_sponsored_message.as_ref())
  }

  pub fn report_message_reactions<C: AsRef<ReportMessageReactions>>(
    &self,
    report_message_reactions: C,
  ) -> RTDResult<()> {
    self.api.send(report_message_reactions.as_ref())
  }

  pub fn report_phone_number_code_missing<C: AsRef<ReportPhoneNumberCodeMissing>>(
    &self,
    report_phone_number_code_missing: C,
  ) -> RTDResult<()> {
    self.api.send(report_phone_number_code_missing.as_ref())
  }

  pub fn report_sponsored_chat<C: AsRef<ReportSponsoredChat>>(
    &self,
    report_sponsored_chat: C,
  ) -> RTDResult<()> {
    self.api.send(report_sponsored_chat.as_ref())
  }

  pub fn report_story<C: AsRef<ReportStory>>(&self, report_story: C) -> RTDResult<()> {
    self.api.send(report_story.as_ref())
  }

  pub fn report_supergroup_anti_spam_false_positive<
    C: AsRef<ReportSupergroupAntiSpamFalsePositive>,
  >(
    &self,
    report_supergroup_anti_spam_false_positive: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(report_supergroup_anti_spam_false_positive.as_ref())
  }

  pub fn report_supergroup_spam<C: AsRef<ReportSupergroupSpam>>(
    &self,
    report_supergroup_spam: C,
  ) -> RTDResult<()> {
    self.api.send(report_supergroup_spam.as_ref())
  }

  pub fn report_video_message_advertisement<C: AsRef<ReportVideoMessageAdvertisement>>(
    &self,
    report_video_message_advertisement: C,
  ) -> RTDResult<()> {
    self.api.send(report_video_message_advertisement.as_ref())
  }

  pub fn report_web_proxy_carrier_error<C: AsRef<ReportWebProxyCarrierError>>(
    &self,
    report_web_proxy_carrier_error: C,
  ) -> RTDResult<()> {
    self.api.send(report_web_proxy_carrier_error.as_ref())
  }

  pub fn request_authentication_password_recovery<
    C: AsRef<RequestAuthenticationPasswordRecovery>,
  >(
    &self,
    request_authentication_password_recovery: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(request_authentication_password_recovery.as_ref())
  }

  pub fn request_password_recovery<C: AsRef<RequestPasswordRecovery>>(
    &self,
    request_password_recovery: C,
  ) -> RTDResult<()> {
    self.api.send(request_password_recovery.as_ref())
  }

  pub fn request_qr_code_authentication<C: AsRef<RequestQrCodeAuthentication>>(
    &self,
    request_qr_code_authentication: C,
  ) -> RTDResult<()> {
    self.api.send(request_qr_code_authentication.as_ref())
  }

  pub fn resend_authentication_code<C: AsRef<ResendAuthenticationCode>>(
    &self,
    resend_authentication_code: C,
  ) -> RTDResult<()> {
    self.api.send(resend_authentication_code.as_ref())
  }

  pub fn resend_email_address_verification_code<C: AsRef<ResendEmailAddressVerificationCode>>(
    &self,
    resend_email_address_verification_code: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(resend_email_address_verification_code.as_ref())
  }

  pub fn resend_login_email_address_code<C: AsRef<ResendLoginEmailAddressCode>>(
    &self,
    resend_login_email_address_code: C,
  ) -> RTDResult<()> {
    self.api.send(resend_login_email_address_code.as_ref())
  }

  pub fn resend_messages<C: AsRef<ResendMessages>>(&self, resend_messages: C) -> RTDResult<()> {
    self.api.send(resend_messages.as_ref())
  }

  pub fn resend_phone_number_code<C: AsRef<ResendPhoneNumberCode>>(
    &self,
    resend_phone_number_code: C,
  ) -> RTDResult<()> {
    self.api.send(resend_phone_number_code.as_ref())
  }

  pub fn resend_recovery_email_address_code<C: AsRef<ResendRecoveryEmailAddressCode>>(
    &self,
    resend_recovery_email_address_code: C,
  ) -> RTDResult<()> {
    self.api.send(resend_recovery_email_address_code.as_ref())
  }

  pub fn reset_all_notification_settings<C: AsRef<ResetAllNotificationSettings>>(
    &self,
    reset_all_notification_settings: C,
  ) -> RTDResult<()> {
    self.api.send(reset_all_notification_settings.as_ref())
  }

  pub fn reset_authentication_email_address<C: AsRef<ResetAuthenticationEmailAddress>>(
    &self,
    reset_authentication_email_address: C,
  ) -> RTDResult<()> {
    self.api.send(reset_authentication_email_address.as_ref())
  }

  pub fn reset_installed_backgrounds<C: AsRef<ResetInstalledBackgrounds>>(
    &self,
    reset_installed_backgrounds: C,
  ) -> RTDResult<()> {
    self.api.send(reset_installed_backgrounds.as_ref())
  }

  pub fn reset_network_statistics<C: AsRef<ResetNetworkStatistics>>(
    &self,
    reset_network_statistics: C,
  ) -> RTDResult<()> {
    self.api.send(reset_network_statistics.as_ref())
  }

  pub fn reset_password<C: AsRef<ResetPassword>>(&self, reset_password: C) -> RTDResult<()> {
    self.api.send(reset_password.as_ref())
  }

  pub fn reuse_star_subscription<C: AsRef<ReuseStarSubscription>>(
    &self,
    reuse_star_subscription: C,
  ) -> RTDResult<()> {
    self.api.send(reuse_star_subscription.as_ref())
  }

  pub fn revoke_chat_invite_link<C: AsRef<RevokeChatInviteLink>>(
    &self,
    revoke_chat_invite_link: C,
  ) -> RTDResult<()> {
    self.api.send(revoke_chat_invite_link.as_ref())
  }

  pub fn revoke_group_call_invite_link<C: AsRef<RevokeGroupCallInviteLink>>(
    &self,
    revoke_group_call_invite_link: C,
  ) -> RTDResult<()> {
    self.api.send(revoke_group_call_invite_link.as_ref())
  }

  pub fn save_application_log_event<C: AsRef<SaveApplicationLogEvent>>(
    &self,
    save_application_log_event: C,
  ) -> RTDResult<()> {
    self.api.send(save_application_log_event.as_ref())
  }

  pub fn save_prepared_inline_message<C: AsRef<SavePreparedInlineMessage>>(
    &self,
    save_prepared_inline_message: C,
  ) -> RTDResult<()> {
    self.api.send(save_prepared_inline_message.as_ref())
  }

  pub fn save_prepared_keyboard_button<C: AsRef<SavePreparedKeyboardButton>>(
    &self,
    save_prepared_keyboard_button: C,
  ) -> RTDResult<()> {
    self.api.send(save_prepared_keyboard_button.as_ref())
  }

  pub fn search_affiliate_programs<C: AsRef<SearchAffiliatePrograms>>(
    &self,
    search_affiliate_programs: C,
  ) -> RTDResult<()> {
    self.api.send(search_affiliate_programs.as_ref())
  }

  pub fn search_background<C: AsRef<SearchBackground>>(
    &self,
    search_background: C,
  ) -> RTDResult<()> {
    self.api.send(search_background.as_ref())
  }

  pub fn search_call_messages<C: AsRef<SearchCallMessages>>(
    &self,
    search_call_messages: C,
  ) -> RTDResult<()> {
    self.api.send(search_call_messages.as_ref())
  }

  pub fn search_chat_affiliate_program<C: AsRef<SearchChatAffiliateProgram>>(
    &self,
    search_chat_affiliate_program: C,
  ) -> RTDResult<()> {
    self.api.send(search_chat_affiliate_program.as_ref())
  }

  pub fn search_chat_members<C: AsRef<SearchChatMembers>>(
    &self,
    search_chat_members: C,
  ) -> RTDResult<()> {
    self.api.send(search_chat_members.as_ref())
  }

  pub fn search_chat_messages<C: AsRef<SearchChatMessages>>(
    &self,
    search_chat_messages: C,
  ) -> RTDResult<()> {
    self.api.send(search_chat_messages.as_ref())
  }

  pub fn search_chat_recent_location_messages<C: AsRef<SearchChatRecentLocationMessages>>(
    &self,
    search_chat_recent_location_messages: C,
  ) -> RTDResult<()> {
    self.api.send(search_chat_recent_location_messages.as_ref())
  }

  pub fn search_chats<C: AsRef<SearchChats>>(&self, search_chats: C) -> RTDResult<()> {
    self.api.send(search_chats.as_ref())
  }

  pub fn search_chats_on_server<C: AsRef<SearchChatsOnServer>>(
    &self,
    search_chats_on_server: C,
  ) -> RTDResult<()> {
    self.api.send(search_chats_on_server.as_ref())
  }

  pub fn search_contacts<C: AsRef<SearchContacts>>(&self, search_contacts: C) -> RTDResult<()> {
    self.api.send(search_contacts.as_ref())
  }

  pub fn search_emojis<C: AsRef<SearchEmojis>>(&self, search_emojis: C) -> RTDResult<()> {
    self.api.send(search_emojis.as_ref())
  }

  pub fn search_file_downloads<C: AsRef<SearchFileDownloads>>(
    &self,
    search_file_downloads: C,
  ) -> RTDResult<()> {
    self.api.send(search_file_downloads.as_ref())
  }

  pub fn search_gifts_for_resale<C: AsRef<SearchGiftsForResale>>(
    &self,
    search_gifts_for_resale: C,
  ) -> RTDResult<()> {
    self.api.send(search_gifts_for_resale.as_ref())
  }

  pub fn search_hashtags<C: AsRef<SearchHashtags>>(&self, search_hashtags: C) -> RTDResult<()> {
    self.api.send(search_hashtags.as_ref())
  }

  pub fn search_installed_sticker_sets<C: AsRef<SearchInstalledStickerSets>>(
    &self,
    search_installed_sticker_sets: C,
  ) -> RTDResult<()> {
    self.api.send(search_installed_sticker_sets.as_ref())
  }

  pub fn search_messages<C: AsRef<SearchMessages>>(&self, search_messages: C) -> RTDResult<()> {
    self.api.send(search_messages.as_ref())
  }

  pub fn search_outgoing_document_messages<C: AsRef<SearchOutgoingDocumentMessages>>(
    &self,
    search_outgoing_document_messages: C,
  ) -> RTDResult<()> {
    self.api.send(search_outgoing_document_messages.as_ref())
  }

  pub fn search_public_chat<C: AsRef<SearchPublicChat>>(
    &self,
    search_public_chat: C,
  ) -> RTDResult<()> {
    self.api.send(search_public_chat.as_ref())
  }

  pub fn search_public_chats<C: AsRef<SearchPublicChats>>(
    &self,
    search_public_chats: C,
  ) -> RTDResult<()> {
    self.api.send(search_public_chats.as_ref())
  }

  pub fn search_public_messages_by_tag<C: AsRef<SearchPublicMessagesByTag>>(
    &self,
    search_public_messages_by_tag: C,
  ) -> RTDResult<()> {
    self.api.send(search_public_messages_by_tag.as_ref())
  }

  pub fn search_public_posts<C: AsRef<SearchPublicPosts>>(
    &self,
    search_public_posts: C,
  ) -> RTDResult<()> {
    self.api.send(search_public_posts.as_ref())
  }

  pub fn search_public_stories_by_location<C: AsRef<SearchPublicStoriesByLocation>>(
    &self,
    search_public_stories_by_location: C,
  ) -> RTDResult<()> {
    self.api.send(search_public_stories_by_location.as_ref())
  }

  pub fn search_public_stories_by_tag<C: AsRef<SearchPublicStoriesByTag>>(
    &self,
    search_public_stories_by_tag: C,
  ) -> RTDResult<()> {
    self.api.send(search_public_stories_by_tag.as_ref())
  }

  pub fn search_public_stories_by_venue<C: AsRef<SearchPublicStoriesByVenue>>(
    &self,
    search_public_stories_by_venue: C,
  ) -> RTDResult<()> {
    self.api.send(search_public_stories_by_venue.as_ref())
  }

  pub fn search_quote<C: AsRef<SearchQuote>>(&self, search_quote: C) -> RTDResult<()> {
    self.api.send(search_quote.as_ref())
  }

  pub fn search_recently_found_chats<C: AsRef<SearchRecentlyFoundChats>>(
    &self,
    search_recently_found_chats: C,
  ) -> RTDResult<()> {
    self.api.send(search_recently_found_chats.as_ref())
  }

  pub fn search_saved_messages<C: AsRef<SearchSavedMessages>>(
    &self,
    search_saved_messages: C,
  ) -> RTDResult<()> {
    self.api.send(search_saved_messages.as_ref())
  }

  pub fn search_secret_messages<C: AsRef<SearchSecretMessages>>(
    &self,
    search_secret_messages: C,
  ) -> RTDResult<()> {
    self.api.send(search_secret_messages.as_ref())
  }

  pub fn search_sticker_set<C: AsRef<SearchStickerSet>>(
    &self,
    search_sticker_set: C,
  ) -> RTDResult<()> {
    self.api.send(search_sticker_set.as_ref())
  }

  pub fn search_sticker_sets<C: AsRef<SearchStickerSets>>(
    &self,
    search_sticker_sets: C,
  ) -> RTDResult<()> {
    self.api.send(search_sticker_sets.as_ref())
  }

  pub fn search_stickers<C: AsRef<SearchStickers>>(&self, search_stickers: C) -> RTDResult<()> {
    self.api.send(search_stickers.as_ref())
  }

  pub fn search_strings_by_prefix<C: AsRef<SearchStringsByPrefix>>(
    &self,
    search_strings_by_prefix: C,
  ) -> RTDResult<()> {
    self.api.send(search_strings_by_prefix.as_ref())
  }

  pub fn search_text_composition_style<C: AsRef<SearchTextCompositionStyle>>(
    &self,
    search_text_composition_style: C,
  ) -> RTDResult<()> {
    self.api.send(search_text_composition_style.as_ref())
  }

  pub fn search_user_by_phone_number<C: AsRef<SearchUserByPhoneNumber>>(
    &self,
    search_user_by_phone_number: C,
  ) -> RTDResult<()> {
    self.api.send(search_user_by_phone_number.as_ref())
  }

  pub fn search_user_by_token<C: AsRef<SearchUserByToken>>(
    &self,
    search_user_by_token: C,
  ) -> RTDResult<()> {
    self.api.send(search_user_by_token.as_ref())
  }

  pub fn search_web_app<C: AsRef<SearchWebApp>>(&self, search_web_app: C) -> RTDResult<()> {
    self.api.send(search_web_app.as_ref())
  }

  pub fn sell_gift<C: AsRef<SellGift>>(&self, sell_gift: C) -> RTDResult<()> {
    self.api.send(sell_gift.as_ref())
  }

  pub fn send_authentication_firebase_sms<C: AsRef<SendAuthenticationFirebaseSms>>(
    &self,
    send_authentication_firebase_sms: C,
  ) -> RTDResult<()> {
    self.api.send(send_authentication_firebase_sms.as_ref())
  }

  pub fn send_bot_start_message<C: AsRef<SendBotStartMessage>>(
    &self,
    send_bot_start_message: C,
  ) -> RTDResult<()> {
    self.api.send(send_bot_start_message.as_ref())
  }

  pub fn send_business_message<C: AsRef<SendBusinessMessage>>(
    &self,
    send_business_message: C,
  ) -> RTDResult<()> {
    self.api.send(send_business_message.as_ref())
  }

  pub fn send_business_message_album<C: AsRef<SendBusinessMessageAlbum>>(
    &self,
    send_business_message_album: C,
  ) -> RTDResult<()> {
    self.api.send(send_business_message_album.as_ref())
  }

  pub fn send_call_debug_information<C: AsRef<SendCallDebugInformation>>(
    &self,
    send_call_debug_information: C,
  ) -> RTDResult<()> {
    self.api.send(send_call_debug_information.as_ref())
  }

  pub fn send_call_log<C: AsRef<SendCallLog>>(&self, send_call_log: C) -> RTDResult<()> {
    self.api.send(send_call_log.as_ref())
  }

  pub fn send_call_rating<C: AsRef<SendCallRating>>(&self, send_call_rating: C) -> RTDResult<()> {
    self.api.send(send_call_rating.as_ref())
  }

  pub fn send_call_signaling_data<C: AsRef<SendCallSignalingData>>(
    &self,
    send_call_signaling_data: C,
  ) -> RTDResult<()> {
    self.api.send(send_call_signaling_data.as_ref())
  }

  pub fn send_chat_action<C: AsRef<SendChatAction>>(&self, send_chat_action: C) -> RTDResult<()> {
    self.api.send(send_chat_action.as_ref())
  }

  pub fn send_custom_request<C: AsRef<SendCustomRequest>>(
    &self,
    send_custom_request: C,
  ) -> RTDResult<()> {
    self.api.send(send_custom_request.as_ref())
  }

  pub fn send_email_address_verification_code<C: AsRef<SendEmailAddressVerificationCode>>(
    &self,
    send_email_address_verification_code: C,
  ) -> RTDResult<()> {
    self.api.send(send_email_address_verification_code.as_ref())
  }

  pub fn send_ephemeral_message<C: AsRef<SendEphemeralMessage>>(
    &self,
    send_ephemeral_message: C,
  ) -> RTDResult<()> {
    self.api.send(send_ephemeral_message.as_ref())
  }

  pub fn send_gift<C: AsRef<SendGift>>(&self, send_gift: C) -> RTDResult<()> {
    self.api.send(send_gift.as_ref())
  }

  pub fn send_gift_purchase_offer<C: AsRef<SendGiftPurchaseOffer>>(
    &self,
    send_gift_purchase_offer: C,
  ) -> RTDResult<()> {
    self.api.send(send_gift_purchase_offer.as_ref())
  }

  pub fn send_group_call_message<C: AsRef<SendGroupCallMessage>>(
    &self,
    send_group_call_message: C,
  ) -> RTDResult<()> {
    self.api.send(send_group_call_message.as_ref())
  }

  pub fn send_inline_query_result_message<C: AsRef<SendInlineQueryResultMessage>>(
    &self,
    send_inline_query_result_message: C,
  ) -> RTDResult<()> {
    self.api.send(send_inline_query_result_message.as_ref())
  }

  pub fn send_message<C: AsRef<SendMessage>>(&self, send_message: C) -> RTDResult<()> {
    self.api.send(send_message.as_ref())
  }

  pub fn send_message_album<C: AsRef<SendMessageAlbum>>(
    &self,
    send_message_album: C,
  ) -> RTDResult<()> {
    self.api.send(send_message_album.as_ref())
  }

  pub fn send_message_view_metrics<C: AsRef<SendMessageViewMetrics>>(
    &self,
    send_message_view_metrics: C,
  ) -> RTDResult<()> {
    self.api.send(send_message_view_metrics.as_ref())
  }

  pub fn send_passport_authorization_form<C: AsRef<SendPassportAuthorizationForm>>(
    &self,
    send_passport_authorization_form: C,
  ) -> RTDResult<()> {
    self.api.send(send_passport_authorization_form.as_ref())
  }

  pub fn send_payment_form<C: AsRef<SendPaymentForm>>(
    &self,
    send_payment_form: C,
  ) -> RTDResult<()> {
    self.api.send(send_payment_form.as_ref())
  }

  pub fn send_phone_number_code<C: AsRef<SendPhoneNumberCode>>(
    &self,
    send_phone_number_code: C,
  ) -> RTDResult<()> {
    self.api.send(send_phone_number_code.as_ref())
  }

  pub fn send_phone_number_firebase_sms<C: AsRef<SendPhoneNumberFirebaseSms>>(
    &self,
    send_phone_number_firebase_sms: C,
  ) -> RTDResult<()> {
    self.api.send(send_phone_number_firebase_sms.as_ref())
  }

  pub fn send_quick_reply_shortcut_messages<C: AsRef<SendQuickReplyShortcutMessages>>(
    &self,
    send_quick_reply_shortcut_messages: C,
  ) -> RTDResult<()> {
    self.api.send(send_quick_reply_shortcut_messages.as_ref())
  }

  pub fn send_resold_gift<C: AsRef<SendResoldGift>>(&self, send_resold_gift: C) -> RTDResult<()> {
    self.api.send(send_resold_gift.as_ref())
  }

  pub fn send_rich_message_draft<C: AsRef<SendRichMessageDraft>>(
    &self,
    send_rich_message_draft: C,
  ) -> RTDResult<()> {
    self.api.send(send_rich_message_draft.as_ref())
  }

  pub fn send_text_message_draft<C: AsRef<SendTextMessageDraft>>(
    &self,
    send_text_message_draft: C,
  ) -> RTDResult<()> {
    self.api.send(send_text_message_draft.as_ref())
  }

  pub fn send_ton_center_api_request<C: AsRef<SendTonCenterApiRequest>>(
    &self,
    send_ton_center_api_request: C,
  ) -> RTDResult<()> {
    self.api.send(send_ton_center_api_request.as_ref())
  }

  pub fn send_ton_connect_session_connect_result<C: AsRef<SendTonConnectSessionConnectResult>>(
    &self,
    send_ton_connect_session_connect_result: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(send_ton_connect_session_connect_result.as_ref())
  }

  pub fn send_ton_wallet_transfer<C: AsRef<SendTonWalletTransfer>>(
    &self,
    send_ton_wallet_transfer: C,
  ) -> RTDResult<()> {
    self.api.send(send_ton_wallet_transfer.as_ref())
  }

  pub fn send_web_app_custom_request<C: AsRef<SendWebAppCustomRequest>>(
    &self,
    send_web_app_custom_request: C,
  ) -> RTDResult<()> {
    self.api.send(send_web_app_custom_request.as_ref())
  }

  pub fn send_web_app_data<C: AsRef<SendWebAppData>>(&self, send_web_app_data: C) -> RTDResult<()> {
    self.api.send(send_web_app_data.as_ref())
  }

  pub fn set_accent_color<C: AsRef<SetAccentColor>>(&self, set_accent_color: C) -> RTDResult<()> {
    self.api.send(set_accent_color.as_ref())
  }

  pub fn set_account_ttl<C: AsRef<SetAccountTtl>>(&self, set_account_ttl: C) -> RTDResult<()> {
    self.api.send(set_account_ttl.as_ref())
  }

  pub fn set_alarm<C: AsRef<SetAlarm>>(&self, set_alarm: C) -> RTDResult<()> {
    self.api.send(set_alarm.as_ref())
  }

  pub fn set_application_verification_token<C: AsRef<SetApplicationVerificationToken>>(
    &self,
    set_application_verification_token: C,
  ) -> RTDResult<()> {
    self.api.send(set_application_verification_token.as_ref())
  }

  pub fn set_archive_chat_list_settings<C: AsRef<SetArchiveChatListSettings>>(
    &self,
    set_archive_chat_list_settings: C,
  ) -> RTDResult<()> {
    self.api.send(set_archive_chat_list_settings.as_ref())
  }

  pub fn set_authentication_email_address<C: AsRef<SetAuthenticationEmailAddress>>(
    &self,
    set_authentication_email_address: C,
  ) -> RTDResult<()> {
    self.api.send(set_authentication_email_address.as_ref())
  }

  pub fn set_authentication_phone_number<C: AsRef<SetAuthenticationPhoneNumber>>(
    &self,
    set_authentication_phone_number: C,
  ) -> RTDResult<()> {
    self.api.send(set_authentication_phone_number.as_ref())
  }

  pub fn set_authentication_premium_purchase_transaction<
    C: AsRef<SetAuthenticationPremiumPurchaseTransaction>,
  >(
    &self,
    set_authentication_premium_purchase_transaction: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(set_authentication_premium_purchase_transaction.as_ref())
  }

  pub fn set_auto_download_settings<C: AsRef<SetAutoDownloadSettings>>(
    &self,
    set_auto_download_settings: C,
  ) -> RTDResult<()> {
    self.api.send(set_auto_download_settings.as_ref())
  }

  pub fn set_autosave_settings<C: AsRef<SetAutosaveSettings>>(
    &self,
    set_autosave_settings: C,
  ) -> RTDResult<()> {
    self.api.send(set_autosave_settings.as_ref())
  }

  pub fn set_bio<C: AsRef<SetBio>>(&self, set_bio: C) -> RTDResult<()> {
    self.api.send(set_bio.as_ref())
  }

  pub fn set_birthdate<C: AsRef<SetBirthdate>>(&self, set_birthdate: C) -> RTDResult<()> {
    self.api.send(set_birthdate.as_ref())
  }

  pub fn set_bot_info_description<C: AsRef<SetBotInfoDescription>>(
    &self,
    set_bot_info_description: C,
  ) -> RTDResult<()> {
    self.api.send(set_bot_info_description.as_ref())
  }

  pub fn set_bot_info_short_description<C: AsRef<SetBotInfoShortDescription>>(
    &self,
    set_bot_info_short_description: C,
  ) -> RTDResult<()> {
    self.api.send(set_bot_info_short_description.as_ref())
  }

  pub fn set_bot_name<C: AsRef<SetBotName>>(&self, set_bot_name: C) -> RTDResult<()> {
    self.api.send(set_bot_name.as_ref())
  }

  pub fn set_bot_profile_photo<C: AsRef<SetBotProfilePhoto>>(
    &self,
    set_bot_profile_photo: C,
  ) -> RTDResult<()> {
    self.api.send(set_bot_profile_photo.as_ref())
  }

  pub fn set_bot_updates_status<C: AsRef<SetBotUpdatesStatus>>(
    &self,
    set_bot_updates_status: C,
  ) -> RTDResult<()> {
    self.api.send(set_bot_updates_status.as_ref())
  }

  pub fn set_business_account_bio<C: AsRef<SetBusinessAccountBio>>(
    &self,
    set_business_account_bio: C,
  ) -> RTDResult<()> {
    self.api.send(set_business_account_bio.as_ref())
  }

  pub fn set_business_account_gift_settings<C: AsRef<SetBusinessAccountGiftSettings>>(
    &self,
    set_business_account_gift_settings: C,
  ) -> RTDResult<()> {
    self.api.send(set_business_account_gift_settings.as_ref())
  }

  pub fn set_business_account_name<C: AsRef<SetBusinessAccountName>>(
    &self,
    set_business_account_name: C,
  ) -> RTDResult<()> {
    self.api.send(set_business_account_name.as_ref())
  }

  pub fn set_business_account_profile_photo<C: AsRef<SetBusinessAccountProfilePhoto>>(
    &self,
    set_business_account_profile_photo: C,
  ) -> RTDResult<()> {
    self.api.send(set_business_account_profile_photo.as_ref())
  }

  pub fn set_business_account_username<C: AsRef<SetBusinessAccountUsername>>(
    &self,
    set_business_account_username: C,
  ) -> RTDResult<()> {
    self.api.send(set_business_account_username.as_ref())
  }

  pub fn set_business_away_message_settings<C: AsRef<SetBusinessAwayMessageSettings>>(
    &self,
    set_business_away_message_settings: C,
  ) -> RTDResult<()> {
    self.api.send(set_business_away_message_settings.as_ref())
  }

  pub fn set_business_connected_bot<C: AsRef<SetBusinessConnectedBot>>(
    &self,
    set_business_connected_bot: C,
  ) -> RTDResult<()> {
    self.api.send(set_business_connected_bot.as_ref())
  }

  pub fn set_business_greeting_message_settings<C: AsRef<SetBusinessGreetingMessageSettings>>(
    &self,
    set_business_greeting_message_settings: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(set_business_greeting_message_settings.as_ref())
  }

  pub fn set_business_location<C: AsRef<SetBusinessLocation>>(
    &self,
    set_business_location: C,
  ) -> RTDResult<()> {
    self.api.send(set_business_location.as_ref())
  }

  pub fn set_business_message_is_pinned<C: AsRef<SetBusinessMessageIsPinned>>(
    &self,
    set_business_message_is_pinned: C,
  ) -> RTDResult<()> {
    self.api.send(set_business_message_is_pinned.as_ref())
  }

  pub fn set_business_opening_hours<C: AsRef<SetBusinessOpeningHours>>(
    &self,
    set_business_opening_hours: C,
  ) -> RTDResult<()> {
    self.api.send(set_business_opening_hours.as_ref())
  }

  pub fn set_business_start_page<C: AsRef<SetBusinessStartPage>>(
    &self,
    set_business_start_page: C,
  ) -> RTDResult<()> {
    self.api.send(set_business_start_page.as_ref())
  }

  pub fn set_chat_accent_color<C: AsRef<SetChatAccentColor>>(
    &self,
    set_chat_accent_color: C,
  ) -> RTDResult<()> {
    self.api.send(set_chat_accent_color.as_ref())
  }

  pub fn set_chat_active_stories_list<C: AsRef<SetChatActiveStoriesList>>(
    &self,
    set_chat_active_stories_list: C,
  ) -> RTDResult<()> {
    self.api.send(set_chat_active_stories_list.as_ref())
  }

  pub fn set_chat_affiliate_program<C: AsRef<SetChatAffiliateProgram>>(
    &self,
    set_chat_affiliate_program: C,
  ) -> RTDResult<()> {
    self.api.send(set_chat_affiliate_program.as_ref())
  }

  pub fn set_chat_available_reactions<C: AsRef<SetChatAvailableReactions>>(
    &self,
    set_chat_available_reactions: C,
  ) -> RTDResult<()> {
    self.api.send(set_chat_available_reactions.as_ref())
  }

  pub fn set_chat_background<C: AsRef<SetChatBackground>>(
    &self,
    set_chat_background: C,
  ) -> RTDResult<()> {
    self.api.send(set_chat_background.as_ref())
  }

  pub fn set_chat_client_data<C: AsRef<SetChatClientData>>(
    &self,
    set_chat_client_data: C,
  ) -> RTDResult<()> {
    self.api.send(set_chat_client_data.as_ref())
  }

  pub fn set_chat_description<C: AsRef<SetChatDescription>>(
    &self,
    set_chat_description: C,
  ) -> RTDResult<()> {
    self.api.send(set_chat_description.as_ref())
  }

  pub fn set_chat_direct_messages_group<C: AsRef<SetChatDirectMessagesGroup>>(
    &self,
    set_chat_direct_messages_group: C,
  ) -> RTDResult<()> {
    self.api.send(set_chat_direct_messages_group.as_ref())
  }

  pub fn set_chat_discussion_group<C: AsRef<SetChatDiscussionGroup>>(
    &self,
    set_chat_discussion_group: C,
  ) -> RTDResult<()> {
    self.api.send(set_chat_discussion_group.as_ref())
  }

  pub fn set_chat_draft_message<C: AsRef<SetChatDraftMessage>>(
    &self,
    set_chat_draft_message: C,
  ) -> RTDResult<()> {
    self.api.send(set_chat_draft_message.as_ref())
  }

  pub fn set_chat_emoji_status<C: AsRef<SetChatEmojiStatus>>(
    &self,
    set_chat_emoji_status: C,
  ) -> RTDResult<()> {
    self.api.send(set_chat_emoji_status.as_ref())
  }

  pub fn set_chat_location<C: AsRef<SetChatLocation>>(
    &self,
    set_chat_location: C,
  ) -> RTDResult<()> {
    self.api.send(set_chat_location.as_ref())
  }

  pub fn set_chat_member_status<C: AsRef<SetChatMemberStatus>>(
    &self,
    set_chat_member_status: C,
  ) -> RTDResult<()> {
    self.api.send(set_chat_member_status.as_ref())
  }

  pub fn set_chat_member_tag<C: AsRef<SetChatMemberTag>>(
    &self,
    set_chat_member_tag: C,
  ) -> RTDResult<()> {
    self.api.send(set_chat_member_tag.as_ref())
  }

  pub fn set_chat_message_auto_delete_time<C: AsRef<SetChatMessageAutoDeleteTime>>(
    &self,
    set_chat_message_auto_delete_time: C,
  ) -> RTDResult<()> {
    self.api.send(set_chat_message_auto_delete_time.as_ref())
  }

  pub fn set_chat_message_sender<C: AsRef<SetChatMessageSender>>(
    &self,
    set_chat_message_sender: C,
  ) -> RTDResult<()> {
    self.api.send(set_chat_message_sender.as_ref())
  }

  pub fn set_chat_notification_settings<C: AsRef<SetChatNotificationSettings>>(
    &self,
    set_chat_notification_settings: C,
  ) -> RTDResult<()> {
    self.api.send(set_chat_notification_settings.as_ref())
  }

  pub fn set_chat_paid_message_star_count<C: AsRef<SetChatPaidMessageStarCount>>(
    &self,
    set_chat_paid_message_star_count: C,
  ) -> RTDResult<()> {
    self.api.send(set_chat_paid_message_star_count.as_ref())
  }

  pub fn set_chat_permissions<C: AsRef<SetChatPermissions>>(
    &self,
    set_chat_permissions: C,
  ) -> RTDResult<()> {
    self.api.send(set_chat_permissions.as_ref())
  }

  pub fn set_chat_photo<C: AsRef<SetChatPhoto>>(&self, set_chat_photo: C) -> RTDResult<()> {
    self.api.send(set_chat_photo.as_ref())
  }

  pub fn set_chat_pinned_stories<C: AsRef<SetChatPinnedStories>>(
    &self,
    set_chat_pinned_stories: C,
  ) -> RTDResult<()> {
    self.api.send(set_chat_pinned_stories.as_ref())
  }

  pub fn set_chat_profile_accent_color<C: AsRef<SetChatProfileAccentColor>>(
    &self,
    set_chat_profile_accent_color: C,
  ) -> RTDResult<()> {
    self.api.send(set_chat_profile_accent_color.as_ref())
  }

  pub fn set_chat_slow_mode_delay<C: AsRef<SetChatSlowModeDelay>>(
    &self,
    set_chat_slow_mode_delay: C,
  ) -> RTDResult<()> {
    self.api.send(set_chat_slow_mode_delay.as_ref())
  }

  pub fn set_chat_theme<C: AsRef<SetChatTheme>>(&self, set_chat_theme: C) -> RTDResult<()> {
    self.api.send(set_chat_theme.as_ref())
  }

  pub fn set_chat_title<C: AsRef<SetChatTitle>>(&self, set_chat_title: C) -> RTDResult<()> {
    self.api.send(set_chat_title.as_ref())
  }

  pub fn set_close_friends<C: AsRef<SetCloseFriends>>(
    &self,
    set_close_friends: C,
  ) -> RTDResult<()> {
    self.api.send(set_close_friends.as_ref())
  }

  pub fn set_commands<C: AsRef<SetCommands>>(&self, set_commands: C) -> RTDResult<()> {
    self.api.send(set_commands.as_ref())
  }

  pub fn set_community_name<C: AsRef<SetCommunityName>>(
    &self,
    set_community_name: C,
  ) -> RTDResult<()> {
    self.api.send(set_community_name.as_ref())
  }

  pub fn set_community_permissions<C: AsRef<SetCommunityPermissions>>(
    &self,
    set_community_permissions: C,
  ) -> RTDResult<()> {
    self.api.send(set_community_permissions.as_ref())
  }

  pub fn set_community_photo<C: AsRef<SetCommunityPhoto>>(
    &self,
    set_community_photo: C,
  ) -> RTDResult<()> {
    self.api.send(set_community_photo.as_ref())
  }

  pub fn set_custom_emoji_sticker_set_thumbnail<C: AsRef<SetCustomEmojiStickerSetThumbnail>>(
    &self,
    set_custom_emoji_sticker_set_thumbnail: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(set_custom_emoji_sticker_set_thumbnail.as_ref())
  }

  pub fn set_custom_language_pack<C: AsRef<SetCustomLanguagePack>>(
    &self,
    set_custom_language_pack: C,
  ) -> RTDResult<()> {
    self.api.send(set_custom_language_pack.as_ref())
  }

  pub fn set_custom_language_pack_string<C: AsRef<SetCustomLanguagePackString>>(
    &self,
    set_custom_language_pack_string: C,
  ) -> RTDResult<()> {
    self.api.send(set_custom_language_pack_string.as_ref())
  }

  pub fn set_database_encryption_key<C: AsRef<SetDatabaseEncryptionKey>>(
    &self,
    set_database_encryption_key: C,
  ) -> RTDResult<()> {
    self.api.send(set_database_encryption_key.as_ref())
  }

  pub fn set_default_background<C: AsRef<SetDefaultBackground>>(
    &self,
    set_default_background: C,
  ) -> RTDResult<()> {
    self.api.send(set_default_background.as_ref())
  }

  pub fn set_default_channel_administrator_rights<
    C: AsRef<SetDefaultChannelAdministratorRights>,
  >(
    &self,
    set_default_channel_administrator_rights: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(set_default_channel_administrator_rights.as_ref())
  }

  pub fn set_default_group_administrator_rights<C: AsRef<SetDefaultGroupAdministratorRights>>(
    &self,
    set_default_group_administrator_rights: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(set_default_group_administrator_rights.as_ref())
  }

  pub fn set_default_message_auto_delete_time<C: AsRef<SetDefaultMessageAutoDeleteTime>>(
    &self,
    set_default_message_auto_delete_time: C,
  ) -> RTDResult<()> {
    self.api.send(set_default_message_auto_delete_time.as_ref())
  }

  pub fn set_default_reaction_type<C: AsRef<SetDefaultReactionType>>(
    &self,
    set_default_reaction_type: C,
  ) -> RTDResult<()> {
    self.api.send(set_default_reaction_type.as_ref())
  }

  pub fn set_direct_messages_chat_topic_is_marked_as_unread<
    C: AsRef<SetDirectMessagesChatTopicIsMarkedAsUnread>,
  >(
    &self,
    set_direct_messages_chat_topic_is_marked_as_unread: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(set_direct_messages_chat_topic_is_marked_as_unread.as_ref())
  }

  pub fn set_emoji_status<C: AsRef<SetEmojiStatus>>(&self, set_emoji_status: C) -> RTDResult<()> {
    self.api.send(set_emoji_status.as_ref())
  }

  pub fn set_file_generation_progress<C: AsRef<SetFileGenerationProgress>>(
    &self,
    set_file_generation_progress: C,
  ) -> RTDResult<()> {
    self.api.send(set_file_generation_progress.as_ref())
  }

  pub fn set_forum_topic_notification_settings<C: AsRef<SetForumTopicNotificationSettings>>(
    &self,
    set_forum_topic_notification_settings: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(set_forum_topic_notification_settings.as_ref())
  }

  pub fn set_game_score<C: AsRef<SetGameScore>>(&self, set_game_score: C) -> RTDResult<()> {
    self.api.send(set_game_score.as_ref())
  }

  pub fn set_gift_collection_name<C: AsRef<SetGiftCollectionName>>(
    &self,
    set_gift_collection_name: C,
  ) -> RTDResult<()> {
    self.api.send(set_gift_collection_name.as_ref())
  }

  pub fn set_gift_resale_price<C: AsRef<SetGiftResalePrice>>(
    &self,
    set_gift_resale_price: C,
  ) -> RTDResult<()> {
    self.api.send(set_gift_resale_price.as_ref())
  }

  pub fn set_gift_settings<C: AsRef<SetGiftSettings>>(
    &self,
    set_gift_settings: C,
  ) -> RTDResult<()> {
    self.api.send(set_gift_settings.as_ref())
  }

  pub fn set_group_call_paid_message_star_count<C: AsRef<SetGroupCallPaidMessageStarCount>>(
    &self,
    set_group_call_paid_message_star_count: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(set_group_call_paid_message_star_count.as_ref())
  }

  pub fn set_group_call_participant_is_speaking<C: AsRef<SetGroupCallParticipantIsSpeaking>>(
    &self,
    set_group_call_participant_is_speaking: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(set_group_call_participant_is_speaking.as_ref())
  }

  pub fn set_group_call_participant_volume_level<C: AsRef<SetGroupCallParticipantVolumeLevel>>(
    &self,
    set_group_call_participant_volume_level: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(set_group_call_participant_volume_level.as_ref())
  }

  pub fn set_inactive_session_ttl<C: AsRef<SetInactiveSessionTtl>>(
    &self,
    set_inactive_session_ttl: C,
  ) -> RTDResult<()> {
    self.api.send(set_inactive_session_ttl.as_ref())
  }

  pub fn set_inline_game_score<C: AsRef<SetInlineGameScore>>(
    &self,
    set_inline_game_score: C,
  ) -> RTDResult<()> {
    self.api.send(set_inline_game_score.as_ref())
  }

  pub fn set_live_story_message_sender<C: AsRef<SetLiveStoryMessageSender>>(
    &self,
    set_live_story_message_sender: C,
  ) -> RTDResult<()> {
    self.api.send(set_live_story_message_sender.as_ref())
  }

  pub fn set_log_stream<C: AsRef<SetLogStream>>(&self, set_log_stream: C) -> RTDResult<()> {
    self.api.send(set_log_stream.as_ref())
  }

  pub fn set_log_tag_verbosity_level<C: AsRef<SetLogTagVerbosityLevel>>(
    &self,
    set_log_tag_verbosity_level: C,
  ) -> RTDResult<()> {
    self.api.send(set_log_tag_verbosity_level.as_ref())
  }

  pub fn set_log_verbosity_level<C: AsRef<SetLogVerbosityLevel>>(
    &self,
    set_log_verbosity_level: C,
  ) -> RTDResult<()> {
    self.api.send(set_log_verbosity_level.as_ref())
  }

  pub fn set_login_email_address<C: AsRef<SetLoginEmailAddress>>(
    &self,
    set_login_email_address: C,
  ) -> RTDResult<()> {
    self.api.send(set_login_email_address.as_ref())
  }

  pub fn set_main_profile_tab<C: AsRef<SetMainProfileTab>>(
    &self,
    set_main_profile_tab: C,
  ) -> RTDResult<()> {
    self.api.send(set_main_profile_tab.as_ref())
  }

  pub fn set_managed_bot_access_settings<C: AsRef<SetManagedBotAccessSettings>>(
    &self,
    set_managed_bot_access_settings: C,
  ) -> RTDResult<()> {
    self.api.send(set_managed_bot_access_settings.as_ref())
  }

  pub fn set_menu_button<C: AsRef<SetMenuButton>>(&self, set_menu_button: C) -> RTDResult<()> {
    self.api.send(set_menu_button.as_ref())
  }

  pub fn set_message_fact_check<C: AsRef<SetMessageFactCheck>>(
    &self,
    set_message_fact_check: C,
  ) -> RTDResult<()> {
    self.api.send(set_message_fact_check.as_ref())
  }

  pub fn set_message_reactions<C: AsRef<SetMessageReactions>>(
    &self,
    set_message_reactions: C,
  ) -> RTDResult<()> {
    self.api.send(set_message_reactions.as_ref())
  }

  pub fn set_message_sender_block_list<C: AsRef<SetMessageSenderBlockList>>(
    &self,
    set_message_sender_block_list: C,
  ) -> RTDResult<()> {
    self.api.send(set_message_sender_block_list.as_ref())
  }

  pub fn set_message_sender_bot_verification<C: AsRef<SetMessageSenderBotVerification>>(
    &self,
    set_message_sender_bot_verification: C,
  ) -> RTDResult<()> {
    self.api.send(set_message_sender_bot_verification.as_ref())
  }

  pub fn set_name<C: AsRef<SetName>>(&self, set_name: C) -> RTDResult<()> {
    self.api.send(set_name.as_ref())
  }

  pub fn set_network_type<C: AsRef<SetNetworkType>>(&self, set_network_type: C) -> RTDResult<()> {
    self.api.send(set_network_type.as_ref())
  }

  pub fn set_new_chat_privacy_settings<C: AsRef<SetNewChatPrivacySettings>>(
    &self,
    set_new_chat_privacy_settings: C,
  ) -> RTDResult<()> {
    self.api.send(set_new_chat_privacy_settings.as_ref())
  }

  pub fn set_option<C: AsRef<SetOption>>(&self, set_option: C) -> RTDResult<()> {
    self.api.send(set_option.as_ref())
  }

  pub fn set_paid_message_reaction_type<C: AsRef<SetPaidMessageReactionType>>(
    &self,
    set_paid_message_reaction_type: C,
  ) -> RTDResult<()> {
    self.api.send(set_paid_message_reaction_type.as_ref())
  }

  pub fn set_passport_element<C: AsRef<SetPassportElement>>(
    &self,
    set_passport_element: C,
  ) -> RTDResult<()> {
    self.api.send(set_passport_element.as_ref())
  }

  pub fn set_passport_element_errors<C: AsRef<SetPassportElementErrors>>(
    &self,
    set_passport_element_errors: C,
  ) -> RTDResult<()> {
    self.api.send(set_passport_element_errors.as_ref())
  }

  pub fn set_password<C: AsRef<SetPassword>>(&self, set_password: C) -> RTDResult<()> {
    self.api.send(set_password.as_ref())
  }

  pub fn set_personal_chat<C: AsRef<SetPersonalChat>>(
    &self,
    set_personal_chat: C,
  ) -> RTDResult<()> {
    self.api.send(set_personal_chat.as_ref())
  }

  pub fn set_pinned_chats<C: AsRef<SetPinnedChats>>(&self, set_pinned_chats: C) -> RTDResult<()> {
    self.api.send(set_pinned_chats.as_ref())
  }

  pub fn set_pinned_forum_topics<C: AsRef<SetPinnedForumTopics>>(
    &self,
    set_pinned_forum_topics: C,
  ) -> RTDResult<()> {
    self.api.send(set_pinned_forum_topics.as_ref())
  }

  pub fn set_pinned_gifts<C: AsRef<SetPinnedGifts>>(&self, set_pinned_gifts: C) -> RTDResult<()> {
    self.api.send(set_pinned_gifts.as_ref())
  }

  pub fn set_pinned_saved_messages_topics<C: AsRef<SetPinnedSavedMessagesTopics>>(
    &self,
    set_pinned_saved_messages_topics: C,
  ) -> RTDResult<()> {
    self.api.send(set_pinned_saved_messages_topics.as_ref())
  }

  pub fn set_poll_answer<C: AsRef<SetPollAnswer>>(&self, set_poll_answer: C) -> RTDResult<()> {
    self.api.send(set_poll_answer.as_ref())
  }

  pub fn set_profile_accent_color<C: AsRef<SetProfileAccentColor>>(
    &self,
    set_profile_accent_color: C,
  ) -> RTDResult<()> {
    self.api.send(set_profile_accent_color.as_ref())
  }

  pub fn set_profile_audio_position<C: AsRef<SetProfileAudioPosition>>(
    &self,
    set_profile_audio_position: C,
  ) -> RTDResult<()> {
    self.api.send(set_profile_audio_position.as_ref())
  }

  pub fn set_profile_photo<C: AsRef<SetProfilePhoto>>(
    &self,
    set_profile_photo: C,
  ) -> RTDResult<()> {
    self.api.send(set_profile_photo.as_ref())
  }

  pub fn set_profile_share_theme<C: AsRef<SetProfileShareTheme>>(
    &self,
    set_profile_share_theme: C,
  ) -> RTDResult<()> {
    self.api.send(set_profile_share_theme.as_ref())
  }

  pub fn set_quick_reply_shortcut_name<C: AsRef<SetQuickReplyShortcutName>>(
    &self,
    set_quick_reply_shortcut_name: C,
  ) -> RTDResult<()> {
    self.api.send(set_quick_reply_shortcut_name.as_ref())
  }

  pub fn set_reaction_notification_settings<C: AsRef<SetReactionNotificationSettings>>(
    &self,
    set_reaction_notification_settings: C,
  ) -> RTDResult<()> {
    self.api.send(set_reaction_notification_settings.as_ref())
  }

  pub fn set_read_date_privacy_settings<C: AsRef<SetReadDatePrivacySettings>>(
    &self,
    set_read_date_privacy_settings: C,
  ) -> RTDResult<()> {
    self.api.send(set_read_date_privacy_settings.as_ref())
  }

  pub fn set_recovery_email_address<C: AsRef<SetRecoveryEmailAddress>>(
    &self,
    set_recovery_email_address: C,
  ) -> RTDResult<()> {
    self.api.send(set_recovery_email_address.as_ref())
  }

  pub fn set_saved_messages_tag_label<C: AsRef<SetSavedMessagesTagLabel>>(
    &self,
    set_saved_messages_tag_label: C,
  ) -> RTDResult<()> {
    self.api.send(set_saved_messages_tag_label.as_ref())
  }

  pub fn set_scope_notification_settings<C: AsRef<SetScopeNotificationSettings>>(
    &self,
    set_scope_notification_settings: C,
  ) -> RTDResult<()> {
    self.api.send(set_scope_notification_settings.as_ref())
  }

  pub fn set_sticker_emojis<C: AsRef<SetStickerEmojis>>(
    &self,
    set_sticker_emojis: C,
  ) -> RTDResult<()> {
    self.api.send(set_sticker_emojis.as_ref())
  }

  pub fn set_sticker_keywords<C: AsRef<SetStickerKeywords>>(
    &self,
    set_sticker_keywords: C,
  ) -> RTDResult<()> {
    self.api.send(set_sticker_keywords.as_ref())
  }

  pub fn set_sticker_mask_position<C: AsRef<SetStickerMaskPosition>>(
    &self,
    set_sticker_mask_position: C,
  ) -> RTDResult<()> {
    self.api.send(set_sticker_mask_position.as_ref())
  }

  pub fn set_sticker_position_in_set<C: AsRef<SetStickerPositionInSet>>(
    &self,
    set_sticker_position_in_set: C,
  ) -> RTDResult<()> {
    self.api.send(set_sticker_position_in_set.as_ref())
  }

  pub fn set_sticker_set_thumbnail<C: AsRef<SetStickerSetThumbnail>>(
    &self,
    set_sticker_set_thumbnail: C,
  ) -> RTDResult<()> {
    self.api.send(set_sticker_set_thumbnail.as_ref())
  }

  pub fn set_sticker_set_title<C: AsRef<SetStickerSetTitle>>(
    &self,
    set_sticker_set_title: C,
  ) -> RTDResult<()> {
    self.api.send(set_sticker_set_title.as_ref())
  }

  pub fn set_story_album_name<C: AsRef<SetStoryAlbumName>>(
    &self,
    set_story_album_name: C,
  ) -> RTDResult<()> {
    self.api.send(set_story_album_name.as_ref())
  }

  pub fn set_story_privacy_settings<C: AsRef<SetStoryPrivacySettings>>(
    &self,
    set_story_privacy_settings: C,
  ) -> RTDResult<()> {
    self.api.send(set_story_privacy_settings.as_ref())
  }

  pub fn set_story_reaction<C: AsRef<SetStoryReaction>>(
    &self,
    set_story_reaction: C,
  ) -> RTDResult<()> {
    self.api.send(set_story_reaction.as_ref())
  }

  pub fn set_supergroup_custom_emoji_sticker_set<C: AsRef<SetSupergroupCustomEmojiStickerSet>>(
    &self,
    set_supergroup_custom_emoji_sticker_set: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(set_supergroup_custom_emoji_sticker_set.as_ref())
  }

  pub fn set_supergroup_main_profile_tab<C: AsRef<SetSupergroupMainProfileTab>>(
    &self,
    set_supergroup_main_profile_tab: C,
  ) -> RTDResult<()> {
    self.api.send(set_supergroup_main_profile_tab.as_ref())
  }

  pub fn set_supergroup_sticker_set<C: AsRef<SetSupergroupStickerSet>>(
    &self,
    set_supergroup_sticker_set: C,
  ) -> RTDResult<()> {
    self.api.send(set_supergroup_sticker_set.as_ref())
  }

  pub fn set_supergroup_unrestrict_boost_count<C: AsRef<SetSupergroupUnrestrictBoostCount>>(
    &self,
    set_supergroup_unrestrict_boost_count: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(set_supergroup_unrestrict_boost_count.as_ref())
  }

  pub fn set_supergroup_username<C: AsRef<SetSupergroupUsername>>(
    &self,
    set_supergroup_username: C,
  ) -> RTDResult<()> {
    self.api.send(set_supergroup_username.as_ref())
  }

  pub fn set_tdlib_parameters<C: AsRef<SetTdlibParameters>>(
    &self,
    set_tdlib_parameters: C,
  ) -> RTDResult<()> {
    self.api.send(set_tdlib_parameters.as_ref())
  }

  pub fn set_ton_connect_session_wallet_client_id<C: AsRef<SetTonConnectSessionWalletClientId>>(
    &self,
    set_ton_connect_session_wallet_client_id: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(set_ton_connect_session_wallet_client_id.as_ref())
  }

  pub fn set_ton_wallet_card_stickers<C: AsRef<SetTonWalletCardStickers>>(
    &self,
    set_ton_wallet_card_stickers: C,
  ) -> RTDResult<()> {
    self.api.send(set_ton_wallet_card_stickers.as_ref())
  }

  pub fn set_upgraded_gift_colors<C: AsRef<SetUpgradedGiftColors>>(
    &self,
    set_upgraded_gift_colors: C,
  ) -> RTDResult<()> {
    self.api.send(set_upgraded_gift_colors.as_ref())
  }

  pub fn set_user_emoji_status<C: AsRef<SetUserEmojiStatus>>(
    &self,
    set_user_emoji_status: C,
  ) -> RTDResult<()> {
    self.api.send(set_user_emoji_status.as_ref())
  }

  pub fn set_user_note<C: AsRef<SetUserNote>>(&self, set_user_note: C) -> RTDResult<()> {
    self.api.send(set_user_note.as_ref())
  }

  pub fn set_user_personal_profile_photo<C: AsRef<SetUserPersonalProfilePhoto>>(
    &self,
    set_user_personal_profile_photo: C,
  ) -> RTDResult<()> {
    self.api.send(set_user_personal_profile_photo.as_ref())
  }

  pub fn set_user_privacy_setting_rules<C: AsRef<SetUserPrivacySettingRules>>(
    &self,
    set_user_privacy_setting_rules: C,
  ) -> RTDResult<()> {
    self.api.send(set_user_privacy_setting_rules.as_ref())
  }

  pub fn set_user_support_info<C: AsRef<SetUserSupportInfo>>(
    &self,
    set_user_support_info: C,
  ) -> RTDResult<()> {
    self.api.send(set_user_support_info.as_ref())
  }

  pub fn set_username<C: AsRef<SetUsername>>(&self, set_username: C) -> RTDResult<()> {
    self.api.send(set_username.as_ref())
  }

  pub fn set_video_chat_default_participant<C: AsRef<SetVideoChatDefaultParticipant>>(
    &self,
    set_video_chat_default_participant: C,
  ) -> RTDResult<()> {
    self.api.send(set_video_chat_default_participant.as_ref())
  }

  pub fn set_video_chat_title<C: AsRef<SetVideoChatTitle>>(
    &self,
    set_video_chat_title: C,
  ) -> RTDResult<()> {
    self.api.send(set_video_chat_title.as_ref())
  }

  pub fn share_chat_with_bot<C: AsRef<ShareChatWithBot>>(
    &self,
    share_chat_with_bot: C,
  ) -> RTDResult<()> {
    self.api.send(share_chat_with_bot.as_ref())
  }

  pub fn share_phone_number<C: AsRef<SharePhoneNumber>>(
    &self,
    share_phone_number: C,
  ) -> RTDResult<()> {
    self.api.send(share_phone_number.as_ref())
  }

  pub fn share_users_with_bot<C: AsRef<ShareUsersWithBot>>(
    &self,
    share_users_with_bot: C,
  ) -> RTDResult<()> {
    self.api.send(share_users_with_bot.as_ref())
  }

  pub fn start_group_call_recording<C: AsRef<StartGroupCallRecording>>(
    &self,
    start_group_call_recording: C,
  ) -> RTDResult<()> {
    self.api.send(start_group_call_recording.as_ref())
  }

  pub fn start_group_call_screen_sharing<C: AsRef<StartGroupCallScreenSharing>>(
    &self,
    start_group_call_screen_sharing: C,
  ) -> RTDResult<()> {
    self.api.send(start_group_call_screen_sharing.as_ref())
  }

  pub fn start_live_story<C: AsRef<StartLiveStory>>(&self, start_live_story: C) -> RTDResult<()> {
    self.api.send(start_live_story.as_ref())
  }

  pub fn start_scheduled_video_chat<C: AsRef<StartScheduledVideoChat>>(
    &self,
    start_scheduled_video_chat: C,
  ) -> RTDResult<()> {
    self.api.send(start_scheduled_video_chat.as_ref())
  }

  pub fn stop_business_poll<C: AsRef<StopBusinessPoll>>(
    &self,
    stop_business_poll: C,
  ) -> RTDResult<()> {
    self.api.send(stop_business_poll.as_ref())
  }

  pub fn stop_pending_message<C: AsRef<StopPendingMessage>>(
    &self,
    stop_pending_message: C,
  ) -> RTDResult<()> {
    self.api.send(stop_pending_message.as_ref())
  }

  pub fn stop_poll<C: AsRef<StopPoll>>(&self, stop_poll: C) -> RTDResult<()> {
    self.api.send(stop_poll.as_ref())
  }

  pub fn suggest_user_birthdate<C: AsRef<SuggestUserBirthdate>>(
    &self,
    suggest_user_birthdate: C,
  ) -> RTDResult<()> {
    self.api.send(suggest_user_birthdate.as_ref())
  }

  pub fn suggest_user_profile_photo<C: AsRef<SuggestUserProfilePhoto>>(
    &self,
    suggest_user_profile_photo: C,
  ) -> RTDResult<()> {
    self.api.send(suggest_user_profile_photo.as_ref())
  }

  pub fn summarize_message<C: AsRef<SummarizeMessage>>(
    &self,
    summarize_message: C,
  ) -> RTDResult<()> {
    self.api.send(summarize_message.as_ref())
  }

  pub fn synchronize_language_pack<C: AsRef<SynchronizeLanguagePack>>(
    &self,
    synchronize_language_pack: C,
  ) -> RTDResult<()> {
    self.api.send(synchronize_language_pack.as_ref())
  }

  pub fn terminate_all_other_sessions<C: AsRef<TerminateAllOtherSessions>>(
    &self,
    terminate_all_other_sessions: C,
  ) -> RTDResult<()> {
    self.api.send(terminate_all_other_sessions.as_ref())
  }

  pub fn terminate_session<C: AsRef<TerminateSession>>(
    &self,
    terminate_session: C,
  ) -> RTDResult<()> {
    self.api.send(terminate_session.as_ref())
  }

  pub fn test_call_bytes<C: AsRef<TestCallBytes>>(&self, test_call_bytes: C) -> RTDResult<()> {
    self.api.send(test_call_bytes.as_ref())
  }

  pub fn test_call_empty<C: AsRef<TestCallEmpty>>(&self, test_call_empty: C) -> RTDResult<()> {
    self.api.send(test_call_empty.as_ref())
  }

  pub fn test_call_string<C: AsRef<TestCallString>>(&self, test_call_string: C) -> RTDResult<()> {
    self.api.send(test_call_string.as_ref())
  }

  pub fn test_call_vector_int<C: AsRef<TestCallVectorInt>>(
    &self,
    test_call_vector_int: C,
  ) -> RTDResult<()> {
    self.api.send(test_call_vector_int.as_ref())
  }

  pub fn test_call_vector_int_object<C: AsRef<TestCallVectorIntObject>>(
    &self,
    test_call_vector_int_object: C,
  ) -> RTDResult<()> {
    self.api.send(test_call_vector_int_object.as_ref())
  }

  pub fn test_call_vector_string<C: AsRef<TestCallVectorString>>(
    &self,
    test_call_vector_string: C,
  ) -> RTDResult<()> {
    self.api.send(test_call_vector_string.as_ref())
  }

  pub fn test_call_vector_string_object<C: AsRef<TestCallVectorStringObject>>(
    &self,
    test_call_vector_string_object: C,
  ) -> RTDResult<()> {
    self.api.send(test_call_vector_string_object.as_ref())
  }

  pub fn test_get_difference<C: AsRef<TestGetDifference>>(
    &self,
    test_get_difference: C,
  ) -> RTDResult<()> {
    self.api.send(test_get_difference.as_ref())
  }

  pub fn test_network<C: AsRef<TestNetwork>>(&self, test_network: C) -> RTDResult<()> {
    self.api.send(test_network.as_ref())
  }

  pub fn test_proxy<C: AsRef<TestProxy>>(&self, test_proxy: C) -> RTDResult<()> {
    self.api.send(test_proxy.as_ref())
  }

  pub fn test_return_error<C: AsRef<TestReturnError>>(
    &self,
    test_return_error: C,
  ) -> RTDResult<()> {
    self.api.send(test_return_error.as_ref())
  }

  pub fn test_square_int<C: AsRef<TestSquareInt>>(&self, test_square_int: C) -> RTDResult<()> {
    self.api.send(test_square_int.as_ref())
  }

  pub fn test_use_update<C: AsRef<TestUseUpdate>>(&self, test_use_update: C) -> RTDResult<()> {
    self.api.send(test_use_update.as_ref())
  }

  pub fn toggle_all_downloads_are_paused<C: AsRef<ToggleAllDownloadsArePaused>>(
    &self,
    toggle_all_downloads_are_paused: C,
  ) -> RTDResult<()> {
    self.api.send(toggle_all_downloads_are_paused.as_ref())
  }

  pub fn toggle_bot_can_manage_emoji_status<C: AsRef<ToggleBotCanManageEmojiStatus>>(
    &self,
    toggle_bot_can_manage_emoji_status: C,
  ) -> RTDResult<()> {
    self.api.send(toggle_bot_can_manage_emoji_status.as_ref())
  }

  pub fn toggle_bot_is_added_to_attachment_menu<C: AsRef<ToggleBotIsAddedToAttachmentMenu>>(
    &self,
    toggle_bot_is_added_to_attachment_menu: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(toggle_bot_is_added_to_attachment_menu.as_ref())
  }

  pub fn toggle_bot_username_is_active<C: AsRef<ToggleBotUsernameIsActive>>(
    &self,
    toggle_bot_username_is_active: C,
  ) -> RTDResult<()> {
    self.api.send(toggle_bot_username_is_active.as_ref())
  }

  pub fn toggle_business_connected_bot_chat_is_paused<
    C: AsRef<ToggleBusinessConnectedBotChatIsPaused>,
  >(
    &self,
    toggle_business_connected_bot_chat_is_paused: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(toggle_business_connected_bot_chat_is_paused.as_ref())
  }

  pub fn toggle_chat_default_disable_notification<
    C: AsRef<ToggleChatDefaultDisableNotification>,
  >(
    &self,
    toggle_chat_default_disable_notification: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(toggle_chat_default_disable_notification.as_ref())
  }

  pub fn toggle_chat_folder_tags<C: AsRef<ToggleChatFolderTags>>(
    &self,
    toggle_chat_folder_tags: C,
  ) -> RTDResult<()> {
    self.api.send(toggle_chat_folder_tags.as_ref())
  }

  pub fn toggle_chat_gift_notifications<C: AsRef<ToggleChatGiftNotifications>>(
    &self,
    toggle_chat_gift_notifications: C,
  ) -> RTDResult<()> {
    self.api.send(toggle_chat_gift_notifications.as_ref())
  }

  pub fn toggle_chat_has_protected_content<C: AsRef<ToggleChatHasProtectedContent>>(
    &self,
    toggle_chat_has_protected_content: C,
  ) -> RTDResult<()> {
    self.api.send(toggle_chat_has_protected_content.as_ref())
  }

  pub fn toggle_chat_is_marked_as_unread<C: AsRef<ToggleChatIsMarkedAsUnread>>(
    &self,
    toggle_chat_is_marked_as_unread: C,
  ) -> RTDResult<()> {
    self.api.send(toggle_chat_is_marked_as_unread.as_ref())
  }

  pub fn toggle_chat_is_pinned<C: AsRef<ToggleChatIsPinned>>(
    &self,
    toggle_chat_is_pinned: C,
  ) -> RTDResult<()> {
    self.api.send(toggle_chat_is_pinned.as_ref())
  }

  pub fn toggle_chat_is_translatable<C: AsRef<ToggleChatIsTranslatable>>(
    &self,
    toggle_chat_is_translatable: C,
  ) -> RTDResult<()> {
    self.api.send(toggle_chat_is_translatable.as_ref())
  }

  pub fn toggle_chat_view_as_topics<C: AsRef<ToggleChatViewAsTopics>>(
    &self,
    toggle_chat_view_as_topics: C,
  ) -> RTDResult<()> {
    self.api.send(toggle_chat_view_as_topics.as_ref())
  }

  pub fn toggle_direct_messages_chat_topic_can_send_unpaid_messages<
    C: AsRef<ToggleDirectMessagesChatTopicCanSendUnpaidMessages>,
  >(
    &self,
    toggle_direct_messages_chat_topic_can_send_unpaid_messages: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(toggle_direct_messages_chat_topic_can_send_unpaid_messages.as_ref())
  }

  pub fn toggle_download_is_paused<C: AsRef<ToggleDownloadIsPaused>>(
    &self,
    toggle_download_is_paused: C,
  ) -> RTDResult<()> {
    self.api.send(toggle_download_is_paused.as_ref())
  }

  pub fn toggle_forum_topic_is_closed<C: AsRef<ToggleForumTopicIsClosed>>(
    &self,
    toggle_forum_topic_is_closed: C,
  ) -> RTDResult<()> {
    self.api.send(toggle_forum_topic_is_closed.as_ref())
  }

  pub fn toggle_forum_topic_is_pinned<C: AsRef<ToggleForumTopicIsPinned>>(
    &self,
    toggle_forum_topic_is_pinned: C,
  ) -> RTDResult<()> {
    self.api.send(toggle_forum_topic_is_pinned.as_ref())
  }

  pub fn toggle_general_forum_topic_is_hidden<C: AsRef<ToggleGeneralForumTopicIsHidden>>(
    &self,
    toggle_general_forum_topic_is_hidden: C,
  ) -> RTDResult<()> {
    self.api.send(toggle_general_forum_topic_is_hidden.as_ref())
  }

  pub fn toggle_gift_is_saved<C: AsRef<ToggleGiftIsSaved>>(
    &self,
    toggle_gift_is_saved: C,
  ) -> RTDResult<()> {
    self.api.send(toggle_gift_is_saved.as_ref())
  }

  pub fn toggle_group_call_are_messages_allowed<C: AsRef<ToggleGroupCallAreMessagesAllowed>>(
    &self,
    toggle_group_call_are_messages_allowed: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(toggle_group_call_are_messages_allowed.as_ref())
  }

  pub fn toggle_group_call_is_my_video_enabled<C: AsRef<ToggleGroupCallIsMyVideoEnabled>>(
    &self,
    toggle_group_call_is_my_video_enabled: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(toggle_group_call_is_my_video_enabled.as_ref())
  }

  pub fn toggle_group_call_is_my_video_paused<C: AsRef<ToggleGroupCallIsMyVideoPaused>>(
    &self,
    toggle_group_call_is_my_video_paused: C,
  ) -> RTDResult<()> {
    self.api.send(toggle_group_call_is_my_video_paused.as_ref())
  }

  pub fn toggle_group_call_participant_is_hand_raised<
    C: AsRef<ToggleGroupCallParticipantIsHandRaised>,
  >(
    &self,
    toggle_group_call_participant_is_hand_raised: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(toggle_group_call_participant_is_hand_raised.as_ref())
  }

  pub fn toggle_group_call_participant_is_muted<C: AsRef<ToggleGroupCallParticipantIsMuted>>(
    &self,
    toggle_group_call_participant_is_muted: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(toggle_group_call_participant_is_muted.as_ref())
  }

  pub fn toggle_group_call_screen_sharing_is_paused<
    C: AsRef<ToggleGroupCallScreenSharingIsPaused>,
  >(
    &self,
    toggle_group_call_screen_sharing_is_paused: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(toggle_group_call_screen_sharing_is_paused.as_ref())
  }

  pub fn toggle_has_sponsored_messages_enabled<C: AsRef<ToggleHasSponsoredMessagesEnabled>>(
    &self,
    toggle_has_sponsored_messages_enabled: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(toggle_has_sponsored_messages_enabled.as_ref())
  }

  pub fn toggle_saved_messages_topic_is_pinned<C: AsRef<ToggleSavedMessagesTopicIsPinned>>(
    &self,
    toggle_saved_messages_topic_is_pinned: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(toggle_saved_messages_topic_is_pinned.as_ref())
  }

  pub fn toggle_session_can_accept_calls<C: AsRef<ToggleSessionCanAcceptCalls>>(
    &self,
    toggle_session_can_accept_calls: C,
  ) -> RTDResult<()> {
    self.api.send(toggle_session_can_accept_calls.as_ref())
  }

  pub fn toggle_session_can_accept_secret_chats<C: AsRef<ToggleSessionCanAcceptSecretChats>>(
    &self,
    toggle_session_can_accept_secret_chats: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(toggle_session_can_accept_secret_chats.as_ref())
  }

  pub fn toggle_story_is_posted_to_chat_page<C: AsRef<ToggleStoryIsPostedToChatPage>>(
    &self,
    toggle_story_is_posted_to_chat_page: C,
  ) -> RTDResult<()> {
    self.api.send(toggle_story_is_posted_to_chat_page.as_ref())
  }

  pub fn toggle_supergroup_can_have_sponsored_messages<
    C: AsRef<ToggleSupergroupCanHaveSponsoredMessages>,
  >(
    &self,
    toggle_supergroup_can_have_sponsored_messages: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(toggle_supergroup_can_have_sponsored_messages.as_ref())
  }

  pub fn toggle_supergroup_has_aggressive_anti_spam_enabled<
    C: AsRef<ToggleSupergroupHasAggressiveAntiSpamEnabled>,
  >(
    &self,
    toggle_supergroup_has_aggressive_anti_spam_enabled: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(toggle_supergroup_has_aggressive_anti_spam_enabled.as_ref())
  }

  pub fn toggle_supergroup_has_automatic_translation<
    C: AsRef<ToggleSupergroupHasAutomaticTranslation>,
  >(
    &self,
    toggle_supergroup_has_automatic_translation: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(toggle_supergroup_has_automatic_translation.as_ref())
  }

  pub fn toggle_supergroup_has_hidden_members<C: AsRef<ToggleSupergroupHasHiddenMembers>>(
    &self,
    toggle_supergroup_has_hidden_members: C,
  ) -> RTDResult<()> {
    self.api.send(toggle_supergroup_has_hidden_members.as_ref())
  }

  pub fn toggle_supergroup_is_all_history_available<
    C: AsRef<ToggleSupergroupIsAllHistoryAvailable>,
  >(
    &self,
    toggle_supergroup_is_all_history_available: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(toggle_supergroup_is_all_history_available.as_ref())
  }

  pub fn toggle_supergroup_is_broadcast_group<C: AsRef<ToggleSupergroupIsBroadcastGroup>>(
    &self,
    toggle_supergroup_is_broadcast_group: C,
  ) -> RTDResult<()> {
    self.api.send(toggle_supergroup_is_broadcast_group.as_ref())
  }

  pub fn toggle_supergroup_is_forum<C: AsRef<ToggleSupergroupIsForum>>(
    &self,
    toggle_supergroup_is_forum: C,
  ) -> RTDResult<()> {
    self.api.send(toggle_supergroup_is_forum.as_ref())
  }

  pub fn toggle_supergroup_join_by_request<C: AsRef<ToggleSupergroupJoinByRequest>>(
    &self,
    toggle_supergroup_join_by_request: C,
  ) -> RTDResult<()> {
    self.api.send(toggle_supergroup_join_by_request.as_ref())
  }

  pub fn toggle_supergroup_join_to_send_messages<C: AsRef<ToggleSupergroupJoinToSendMessages>>(
    &self,
    toggle_supergroup_join_to_send_messages: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(toggle_supergroup_join_to_send_messages.as_ref())
  }

  pub fn toggle_supergroup_sign_messages<C: AsRef<ToggleSupergroupSignMessages>>(
    &self,
    toggle_supergroup_sign_messages: C,
  ) -> RTDResult<()> {
    self.api.send(toggle_supergroup_sign_messages.as_ref())
  }

  pub fn toggle_supergroup_username_is_active<C: AsRef<ToggleSupergroupUsernameIsActive>>(
    &self,
    toggle_supergroup_username_is_active: C,
  ) -> RTDResult<()> {
    self.api.send(toggle_supergroup_username_is_active.as_ref())
  }

  pub fn toggle_username_is_active<C: AsRef<ToggleUsernameIsActive>>(
    &self,
    toggle_username_is_active: C,
  ) -> RTDResult<()> {
    self.api.send(toggle_username_is_active.as_ref())
  }

  pub fn toggle_video_chat_enabled_start_notification<
    C: AsRef<ToggleVideoChatEnabledStartNotification>,
  >(
    &self,
    toggle_video_chat_enabled_start_notification: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(toggle_video_chat_enabled_start_notification.as_ref())
  }

  pub fn toggle_video_chat_mute_new_participants<C: AsRef<ToggleVideoChatMuteNewParticipants>>(
    &self,
    toggle_video_chat_mute_new_participants: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(toggle_video_chat_mute_new_participants.as_ref())
  }

  pub fn transfer_business_account_stars<C: AsRef<TransferBusinessAccountStars>>(
    &self,
    transfer_business_account_stars: C,
  ) -> RTDResult<()> {
    self.api.send(transfer_business_account_stars.as_ref())
  }

  pub fn transfer_chat_ownership<C: AsRef<TransferChatOwnership>>(
    &self,
    transfer_chat_ownership: C,
  ) -> RTDResult<()> {
    self.api.send(transfer_chat_ownership.as_ref())
  }

  pub fn transfer_gift<C: AsRef<TransferGift>>(&self, transfer_gift: C) -> RTDResult<()> {
    self.api.send(transfer_gift.as_ref())
  }

  pub fn translate_message_rich_message<C: AsRef<TranslateMessageRichMessage>>(
    &self,
    translate_message_rich_message: C,
  ) -> RTDResult<()> {
    self.api.send(translate_message_rich_message.as_ref())
  }

  pub fn translate_message_text<C: AsRef<TranslateMessageText>>(
    &self,
    translate_message_text: C,
  ) -> RTDResult<()> {
    self.api.send(translate_message_text.as_ref())
  }

  pub fn translate_rich_message<C: AsRef<TranslateRichMessage>>(
    &self,
    translate_rich_message: C,
  ) -> RTDResult<()> {
    self.api.send(translate_rich_message.as_ref())
  }

  pub fn translate_text<C: AsRef<TranslateText>>(&self, translate_text: C) -> RTDResult<()> {
    self.api.send(translate_text.as_ref())
  }

  pub fn unpin_all_chat_messages<C: AsRef<UnpinAllChatMessages>>(
    &self,
    unpin_all_chat_messages: C,
  ) -> RTDResult<()> {
    self.api.send(unpin_all_chat_messages.as_ref())
  }

  pub fn unpin_all_direct_messages_chat_topic_messages<
    C: AsRef<UnpinAllDirectMessagesChatTopicMessages>,
  >(
    &self,
    unpin_all_direct_messages_chat_topic_messages: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(unpin_all_direct_messages_chat_topic_messages.as_ref())
  }

  pub fn unpin_all_forum_topic_messages<C: AsRef<UnpinAllForumTopicMessages>>(
    &self,
    unpin_all_forum_topic_messages: C,
  ) -> RTDResult<()> {
    self.api.send(unpin_all_forum_topic_messages.as_ref())
  }

  pub fn unpin_chat_message<C: AsRef<UnpinChatMessage>>(
    &self,
    unpin_chat_message: C,
  ) -> RTDResult<()> {
    self.api.send(unpin_chat_message.as_ref())
  }

  pub fn upgrade_basic_group_chat_to_supergroup_chat<
    C: AsRef<UpgradeBasicGroupChatToSupergroupChat>,
  >(
    &self,
    upgrade_basic_group_chat_to_supergroup_chat: C,
  ) -> RTDResult<()> {
    self
      .api
      .send(upgrade_basic_group_chat_to_supergroup_chat.as_ref())
  }

  pub fn upgrade_gift<C: AsRef<UpgradeGift>>(&self, upgrade_gift: C) -> RTDResult<()> {
    self.api.send(upgrade_gift.as_ref())
  }

  pub fn upload_sticker_file<C: AsRef<UploadStickerFile>>(
    &self,
    upload_sticker_file: C,
  ) -> RTDResult<()> {
    self.api.send(upload_sticker_file.as_ref())
  }

  pub fn validate_order_info<C: AsRef<ValidateOrderInfo>>(
    &self,
    validate_order_info: C,
  ) -> RTDResult<()> {
    self.api.send(validate_order_info.as_ref())
  }

  pub fn view_messages<C: AsRef<ViewMessages>>(&self, view_messages: C) -> RTDResult<()> {
    self.api.send(view_messages.as_ref())
  }

  pub fn view_premium_feature<C: AsRef<ViewPremiumFeature>>(
    &self,
    view_premium_feature: C,
  ) -> RTDResult<()> {
    self.api.send(view_premium_feature.as_ref())
  }

  pub fn view_sponsored_chat<C: AsRef<ViewSponsoredChat>>(
    &self,
    view_sponsored_chat: C,
  ) -> RTDResult<()> {
    self.api.send(view_sponsored_chat.as_ref())
  }

  pub fn view_trending_sticker_sets<C: AsRef<ViewTrendingStickerSets>>(
    &self,
    view_trending_sticker_sets: C,
  ) -> RTDResult<()> {
    self.api.send(view_trending_sticker_sets.as_ref())
  }

  pub fn view_video_message_advertisement<C: AsRef<ViewVideoMessageAdvertisement>>(
    &self,
    view_video_message_advertisement: C,
  ) -> RTDResult<()> {
    self.api.send(view_video_message_advertisement.as_ref())
  }

  pub fn withdraw_upgraded_gift<C: AsRef<WithdrawUpgradedGift>>(
    &self,
    withdraw_upgraded_gift: C,
  ) -> RTDResult<()> {
    self.api.send(withdraw_upgraded_gift.as_ref())
  }

  pub fn write_generated_file_part<C: AsRef<WriteGeneratedFilePart>>(
    &self,
    write_generated_file_part: C,
  ) -> RTDResult<()> {
    self.api.send(write_generated_file_part.as_ref())
  }
}
