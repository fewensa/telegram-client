// Generated from TDLib 1.8.69 td_api.tl (commit 4c6377aa7386c39217878a9a56d581922ad37ca4).
// Callback signatures are the public API.
#![allow(clippy::type_complexity)]

use futures::future::LocalBoxFuture;
use rtdlib::types::*;
use std::sync::Arc;

use crate::api::Api;
use crate::errors::*;

/// Telegram client event listener
#[derive(Clone, Default)]
pub struct RasyncListener {
  exception: Option<Arc<dyn Send + Sync + Fn((Api, TGError)) -> LocalBoxFuture<'static, ()>>>,
  receive:
    Option<Arc<dyn Send + Sync + Fn((Api, String)) -> LocalBoxFuture<'static, TGResult<()>>>>,

  test_use_update: Option<
    Arc<dyn Send + Sync + Fn((Api, TestUseUpdate)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_accent_colors: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateAccentColors)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_active_emoji_reactions: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateActiveEmojiReactions)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_active_gift_auctions: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateActiveGiftAuctions)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_active_live_location_messages: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateActiveLiveLocationMessages)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_active_notifications: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateActiveNotifications)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_age_verification_parameters: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateAgeVerificationParameters)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_allowed_bot_username_premium_suffixes: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateAllowedBotUsernamePremiumSuffixes)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_animated_emoji_message_clicked: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateAnimatedEmojiMessageClicked)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_animation_search_parameters: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateAnimationSearchParameters)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_application_recaptcha_verification_required: Option<
    Arc<
      dyn Send
        + Sync
        + Fn(
          (Api, UpdateApplicationRecaptchaVerificationRequired),
        ) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_application_verification_required: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateApplicationVerificationRequired)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_attachment_menu_bots: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateAttachmentMenuBots)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_authorization_state: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateAuthorizationState)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_autosave_settings: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateAutosaveSettings)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_available_message_effects: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateAvailableMessageEffects)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_basic_group: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateBasicGroup)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_basic_group_full_info: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateBasicGroupFullInfo)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_business_connection: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateBusinessConnection)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_business_message_edited: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateBusinessMessageEdited)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_business_messages_deleted: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateBusinessMessagesDeleted)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_call:
    Option<Arc<dyn Send + Sync + Fn((Api, UpdateCall)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_chat_accent_colors: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateChatAccentColors)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_chat_action: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateChatAction)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_chat_action_bar: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateChatActionBar)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_chat_active_stories: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateChatActiveStories)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_chat_added_to_list: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateChatAddedToList)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_chat_available_reactions: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatAvailableReactions)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_chat_background: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateChatBackground)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_chat_block_list: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateChatBlockList)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_chat_boost: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateChatBoost)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_chat_business_bot_manage_bar: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatBusinessBotManageBar)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_chat_default_disable_notification: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatDefaultDisableNotification)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_chat_draft_message: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateChatDraftMessage)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_chat_emoji_status: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateChatEmojiStatus)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_chat_folders: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateChatFolders)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_chat_has_protected_content: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatHasProtectedContent)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_chat_has_scheduled_messages: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatHasScheduledMessages)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_chat_has_welcome_messages: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatHasWelcomeMessages)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_chat_is_marked_as_unread: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatIsMarkedAsUnread)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_chat_is_translatable: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatIsTranslatable)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_chat_join_result: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateChatJoinResult)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_chat_last_message: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateChatLastMessage)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_chat_member: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateChatMember)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_chat_message_auto_delete_time: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatMessageAutoDeleteTime)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_chat_message_sender: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateChatMessageSender)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_chat_notification_settings: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatNotificationSettings)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_chat_online_member_count: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatOnlineMemberCount)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_chat_pending_join_requests: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatPendingJoinRequests)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_chat_permissions: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateChatPermissions)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_chat_photo: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateChatPhoto)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_chat_position: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateChatPosition)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_chat_read_inbox: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateChatReadInbox)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_chat_read_outbox: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateChatReadOutbox)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_chat_removed_from_list: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatRemovedFromList)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_chat_reply_markup: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateChatReplyMarkup)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_chat_revenue_amount: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateChatRevenueAmount)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_chat_theme: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateChatTheme)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_chat_title: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateChatTitle)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_chat_unread_mention_count: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatUnreadMentionCount)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_chat_unread_poll_vote_count: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatUnreadPollVoteCount)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_chat_unread_reaction_count: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatUnreadReactionCount)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_chat_video_chat: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateChatVideoChat)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_chat_view_as_topics: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateChatViewAsTopics)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_chat_welcome_messages: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatWelcomeMessages)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_community: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateCommunity)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_community_full_info: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateCommunityFullInfo)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_connection_state: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateConnectionState)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_contact_close_birthdays: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateContactCloseBirthdays)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_default_background: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateDefaultBackground)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_default_paid_reaction_type: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateDefaultPaidReactionType)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_default_reaction_type: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateDefaultReactionType)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_delete_messages: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateDeleteMessages)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_dice_emojis: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateDiceEmojis)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_direct_messages_chat_topic: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateDirectMessagesChatTopic)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_emoji_chat_themes: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateEmojiChatThemes)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_favorite_stickers: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateFavoriteStickers)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_file:
    Option<Arc<dyn Send + Sync + Fn((Api, UpdateFile)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_file_added_to_downloads: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateFileAddedToDownloads)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_file_download: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateFileDownload)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_file_downloads: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateFileDownloads)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_file_generation_start: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateFileGenerationStart)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_file_generation_stop: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateFileGenerationStop)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_file_removed_from_downloads: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateFileRemovedFromDownloads)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_forum_topic: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateForumTopic)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_forum_topic_info: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateForumTopicInfo)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_freeze_state: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateFreezeState)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_gift_auction_state: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateGiftAuctionState)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_gram_revenue_status: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateGramRevenueStatus)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_group_call: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateGroupCall)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_group_call_message_levels: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateGroupCallMessageLevels)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_group_call_message_send_failed: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateGroupCallMessageSendFailed)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_group_call_messages_deleted: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateGroupCallMessagesDeleted)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_group_call_participant: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateGroupCallParticipant)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_group_call_participants: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateGroupCallParticipants)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_group_call_verification_state: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateGroupCallVerificationState)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_have_pending_notifications: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateHavePendingNotifications)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_installed_sticker_sets: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateInstalledStickerSets)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_language_pack_strings: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateLanguagePackStrings)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_live_story_top_donors: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateLiveStoryTopDonors)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_managed_bot: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateManagedBot)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_message_contains_unread_poll_votes: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateMessageContainsUnreadPollVotes)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_message_content: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateMessageContent)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_message_content_opened: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateMessageContentOpened)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_message_edited: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateMessageEdited)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_message_ephemeral_content: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateMessageEphemeralContent)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_message_fact_check: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateMessageFactCheck)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_message_interaction_info: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateMessageInteractionInfo)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_message_is_pinned: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateMessageIsPinned)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_message_live_location_viewed: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateMessageLiveLocationViewed)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_message_mention_read: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateMessageMentionRead)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_message_reaction: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateMessageReaction)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_message_reactions: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateMessageReactions)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_message_send_acknowledged: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateMessageSendAcknowledged)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_message_send_failed: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateMessageSendFailed)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_message_send_succeeded: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateMessageSendSucceeded)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_message_suggested_post_info: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateMessageSuggestedPostInfo)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_message_unread_reactions: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateMessageUnreadReactions)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_new_business_callback_query: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateNewBusinessCallbackQuery)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_new_business_message: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateNewBusinessMessage)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_new_call_signaling_data: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateNewCallSignalingData)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_new_callback_query: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateNewCallbackQuery)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_new_chat: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateNewChat)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_new_chat_join_request: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateNewChatJoinRequest)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_new_chosen_inline_result: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateNewChosenInlineResult)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_new_custom_event: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateNewCustomEvent)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_new_custom_query: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateNewCustomQuery)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_new_group_call_message: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateNewGroupCallMessage)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_new_group_call_paid_reaction: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateNewGroupCallPaidReaction)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_new_guest_query: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateNewGuestQuery)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_new_inline_callback_query: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateNewInlineCallbackQuery)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_new_inline_query: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateNewInlineQuery)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_new_message: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateNewMessage)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_new_oauth_request: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateNewOauthRequest)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_new_pre_checkout_query: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateNewPreCheckoutQuery)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_new_shipping_query: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateNewShippingQuery)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_notification: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateNotification)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_notification_group: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateNotificationGroup)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_option:
    Option<Arc<dyn Send + Sync + Fn((Api, UpdateOption)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_owned_gram_count: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateOwnedGramCount)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_owned_star_count: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateOwnedStarCount)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_paid_media_purchased: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdatePaidMediaPurchased)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_pending_message: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdatePendingMessage)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_poll:
    Option<Arc<dyn Send + Sync + Fn((Api, UpdatePoll)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_poll_answer: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdatePollAnswer)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_profile_accent_colors: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateProfileAccentColors)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_quick_reply_shortcut: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateQuickReplyShortcut)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_quick_reply_shortcut_deleted: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateQuickReplyShortcutDeleted)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_quick_reply_shortcut_messages: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateQuickReplyShortcutMessages)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_quick_reply_shortcuts: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateQuickReplyShortcuts)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_reaction_notification_settings: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateReactionNotificationSettings)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_recent_stickers: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateRecentStickers)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_saved_animations: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateSavedAnimations)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_saved_messages_tags: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateSavedMessagesTags)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_saved_messages_topic: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateSavedMessagesTopic)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_saved_messages_topic_count: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateSavedMessagesTopicCount)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_saved_notification_sounds: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateSavedNotificationSounds)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_scope_notification_settings: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateScopeNotificationSettings)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_secret_chat: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateSecretChat)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_service_notification: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateServiceNotification)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_speech_recognition_trial: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateSpeechRecognitionTrial)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_speed_limit_notification: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateSpeedLimitNotification)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_stake_dice_state: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateStakeDiceState)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_star_revenue_status: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateStarRevenueStatus)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_sticker_set: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateStickerSet)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_stop_message_draft: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateStopMessageDraft)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_story:
    Option<Arc<dyn Send + Sync + Fn((Api, UpdateStory)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_story_deleted: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateStoryDeleted)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_story_list_chat_count: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateStoryListChatCount)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_story_post_failed: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateStoryPostFailed)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_story_post_succeeded: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateStoryPostSucceeded)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_story_stealth_mode: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateStoryStealthMode)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_suggested_actions: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateSuggestedActions)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_supergroup: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateSupergroup)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_supergroup_full_info: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateSupergroupFullInfo)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_terms_of_service: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateTermsOfService)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_text_composition_styles: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateTextCompositionStyles)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_ton_wallet_gasless_transfers_info: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateTonWalletGaslessTransfersInfo)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_ton_wallet_state: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateTonWalletState)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_ton_wallet_ton_connect_session: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateTonWalletTonConnectSession)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_ton_wallet_ton_connect_session_disconnect_required: Option<
    Arc<
      dyn Send
        + Sync
        + Fn(
          (Api, UpdateTonWalletTonConnectSessionDisconnectRequired),
        ) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_topic_message_count: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateTopicMessageCount)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_trending_sticker_sets: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateTrendingStickerSets)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_trusted_mini_app_bots: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateTrustedMiniAppBots)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_unconfirmed_session: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateUnconfirmedSession)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_unread_chat_count: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateUnreadChatCount)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_unread_message_count: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateUnreadMessageCount)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_user:
    Option<Arc<dyn Send + Sync + Fn((Api, UpdateUser)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  update_user_full_info: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateUserFullInfo)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_user_privacy_setting_rules: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateUserPrivacySettingRules)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_user_status: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateUserStatus)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_user_subscription: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateUserSubscription)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_video_published: Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateVideoPublished)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update_web_app_message_sent: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateWebAppMessageSent)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_web_browser_settings: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateWebBrowserSettings)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_web_proxy_carrier_closed: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateWebProxyCarrierClosed)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_web_proxy_carrier_data: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateWebProxyCarrierData)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  update_web_proxy_carrier_opened: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateWebProxyCarrierOpened)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,

  authorization_state: Option<
    Arc<dyn Send + Sync + Fn((Api, AuthorizationState)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  can_post_story_result: Option<
    Arc<dyn Send + Sync + Fn((Api, CanPostStoryResult)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  can_send_gift_result: Option<
    Arc<dyn Send + Sync + Fn((Api, CanSendGiftResult)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  can_send_message_to_user_result: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, CanSendMessageToUserResult)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  can_transfer_ownership_result: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, CanTransferOwnershipResult)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  chat_join_result: Option<
    Arc<dyn Send + Sync + Fn((Api, ChatJoinResult)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  chat_statistics: Option<
    Arc<dyn Send + Sync + Fn((Api, ChatStatistics)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  check_chat_username_result: Option<
    Arc<
      dyn Send + Sync + Fn((Api, CheckChatUsernameResult)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  check_sticker_set_name_result: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, CheckStickerSetNameResult)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  craft_gift_result: Option<
    Arc<dyn Send + Sync + Fn((Api, CraftGiftResult)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  gift_resale_result: Option<
    Arc<dyn Send + Sync + Fn((Api, GiftResaleResult)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  giveaway_info:
    Option<Arc<dyn Send + Sync + Fn((Api, GiveawayInfo)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  internal_link_type: Option<
    Arc<dyn Send + Sync + Fn((Api, InternalLinkType)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  invite_group_call_participant_result: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, InviteGroupCallParticipantResult)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  json_value:
    Option<Arc<dyn Send + Sync + Fn((Api, JsonValue)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  language_pack_string_value: Option<
    Arc<
      dyn Send + Sync + Fn((Api, LanguagePackStringValue)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  log_stream:
    Option<Arc<dyn Send + Sync + Fn((Api, LogStream)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  login_url_info:
    Option<Arc<dyn Send + Sync + Fn((Api, LoginUrlInfo)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  message_file_type: Option<
    Arc<dyn Send + Sync + Fn((Api, MessageFileType)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  message_read_date: Option<
    Arc<dyn Send + Sync + Fn((Api, MessageReadDate)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  message_sender: Option<
    Arc<dyn Send + Sync + Fn((Api, MessageSender)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  option_value:
    Option<Arc<dyn Send + Sync + Fn((Api, OptionValue)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  passport_element: Option<
    Arc<dyn Send + Sync + Fn((Api, PassportElement)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  report_chat_result: Option<
    Arc<dyn Send + Sync + Fn((Api, ReportChatResult)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  report_sponsored_result: Option<
    Arc<
      dyn Send + Sync + Fn((Api, ReportSponsoredResult)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  report_story_result: Option<
    Arc<dyn Send + Sync + Fn((Api, ReportStoryResult)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  reset_password_result: Option<
    Arc<dyn Send + Sync + Fn((Api, ResetPasswordResult)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  start_live_story_result: Option<
    Arc<dyn Send + Sync + Fn((Api, StartLiveStoryResult)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  statistical_graph: Option<
    Arc<dyn Send + Sync + Fn((Api, StatisticalGraph)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  update: Option<Arc<dyn Send + Sync + Fn((Api, Update)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  web_browser_type: Option<
    Arc<dyn Send + Sync + Fn((Api, WebBrowserType)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  account_ttl:
    Option<Arc<dyn Send + Sync + Fn((Api, AccountTtl)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  added_proxies:
    Option<Arc<dyn Send + Sync + Fn((Api, AddedProxies)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  added_proxy:
    Option<Arc<dyn Send + Sync + Fn((Api, AddedProxy)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  added_reactions: Option<
    Arc<dyn Send + Sync + Fn((Api, AddedReactions)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  animated_emoji: Option<
    Arc<dyn Send + Sync + Fn((Api, AnimatedEmoji)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  animation:
    Option<Arc<dyn Send + Sync + Fn((Api, Animation)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  animations:
    Option<Arc<dyn Send + Sync + Fn((Api, Animations)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  archive_chat_list_settings: Option<
    Arc<
      dyn Send + Sync + Fn((Api, ArchiveChatListSettings)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  attachment_menu_bot: Option<
    Arc<dyn Send + Sync + Fn((Api, AttachmentMenuBot)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  audios: Option<Arc<dyn Send + Sync + Fn((Api, Audios)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  authentication_code_info: Option<
    Arc<
      dyn Send + Sync + Fn((Api, AuthenticationCodeInfo)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  auto_download_settings_presets: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, AutoDownloadSettingsPresets)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  autosave_settings: Option<
    Arc<dyn Send + Sync + Fn((Api, AutosaveSettings)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  available_gifts: Option<
    Arc<dyn Send + Sync + Fn((Api, AvailableGifts)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  available_reactions: Option<
    Arc<dyn Send + Sync + Fn((Api, AvailableReactions)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  background:
    Option<Arc<dyn Send + Sync + Fn((Api, Background)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  backgrounds:
    Option<Arc<dyn Send + Sync + Fn((Api, Backgrounds)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  bank_card_info:
    Option<Arc<dyn Send + Sync + Fn((Api, BankCardInfo)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  basic_group:
    Option<Arc<dyn Send + Sync + Fn((Api, BasicGroup)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  basic_group_full_info: Option<
    Arc<dyn Send + Sync + Fn((Api, BasicGroupFullInfo)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  bot_access_settings: Option<
    Arc<dyn Send + Sync + Fn((Api, BotAccessSettings)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  bot_commands:
    Option<Arc<dyn Send + Sync + Fn((Api, BotCommands)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  bot_media_preview: Option<
    Arc<dyn Send + Sync + Fn((Api, BotMediaPreview)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  bot_media_preview_info: Option<
    Arc<dyn Send + Sync + Fn((Api, BotMediaPreviewInfo)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  bot_media_previews: Option<
    Arc<dyn Send + Sync + Fn((Api, BotMediaPreviews)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  bot_menu_button: Option<
    Arc<dyn Send + Sync + Fn((Api, BotMenuButton)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  business_chat_link: Option<
    Arc<dyn Send + Sync + Fn((Api, BusinessChatLink)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  business_chat_link_info: Option<
    Arc<dyn Send + Sync + Fn((Api, BusinessChatLinkInfo)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  business_chat_links: Option<
    Arc<dyn Send + Sync + Fn((Api, BusinessChatLinks)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  business_connected_bot_info: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, BusinessConnectedBotInfo)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  business_connection: Option<
    Arc<dyn Send + Sync + Fn((Api, BusinessConnection)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  business_features: Option<
    Arc<dyn Send + Sync + Fn((Api, BusinessFeatures)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  business_message: Option<
    Arc<dyn Send + Sync + Fn((Api, BusinessMessage)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  business_messages: Option<
    Arc<dyn Send + Sync + Fn((Api, BusinessMessages)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  call_id:
    Option<Arc<dyn Send + Sync + Fn((Api, CallId)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  callback_query_answer: Option<
    Arc<dyn Send + Sync + Fn((Api, CallbackQueryAnswer)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  chat: Option<Arc<dyn Send + Sync + Fn((Api, Chat)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  chat_active_stories: Option<
    Arc<dyn Send + Sync + Fn((Api, ChatActiveStories)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  chat_administrators: Option<
    Arc<dyn Send + Sync + Fn((Api, ChatAdministrators)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  chat_boost_features: Option<
    Arc<dyn Send + Sync + Fn((Api, ChatBoostFeatures)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  chat_boost_level_features: Option<
    Arc<
      dyn Send + Sync + Fn((Api, ChatBoostLevelFeatures)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  chat_boost_link: Option<
    Arc<dyn Send + Sync + Fn((Api, ChatBoostLink)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  chat_boost_link_info: Option<
    Arc<dyn Send + Sync + Fn((Api, ChatBoostLinkInfo)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  chat_boost_slots: Option<
    Arc<dyn Send + Sync + Fn((Api, ChatBoostSlots)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  chat_boost_status: Option<
    Arc<dyn Send + Sync + Fn((Api, ChatBoostStatus)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  chat_events:
    Option<Arc<dyn Send + Sync + Fn((Api, ChatEvents)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  chat_folder:
    Option<Arc<dyn Send + Sync + Fn((Api, ChatFolder)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  chat_folder_icon: Option<
    Arc<dyn Send + Sync + Fn((Api, ChatFolderIcon)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  chat_folder_info: Option<
    Arc<dyn Send + Sync + Fn((Api, ChatFolderInfo)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  chat_folder_invite_link: Option<
    Arc<dyn Send + Sync + Fn((Api, ChatFolderInviteLink)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  chat_folder_invite_link_info: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ChatFolderInviteLinkInfo)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  chat_folder_invite_links: Option<
    Arc<
      dyn Send + Sync + Fn((Api, ChatFolderInviteLinks)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  chat_invite_link: Option<
    Arc<dyn Send + Sync + Fn((Api, ChatInviteLink)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  chat_invite_link_counts: Option<
    Arc<dyn Send + Sync + Fn((Api, ChatInviteLinkCounts)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  chat_invite_link_info: Option<
    Arc<dyn Send + Sync + Fn((Api, ChatInviteLinkInfo)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  chat_invite_link_members: Option<
    Arc<
      dyn Send + Sync + Fn((Api, ChatInviteLinkMembers)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  chat_invite_links: Option<
    Arc<dyn Send + Sync + Fn((Api, ChatInviteLinks)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  chat_join_requests: Option<
    Arc<dyn Send + Sync + Fn((Api, ChatJoinRequests)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  chat_lists:
    Option<Arc<dyn Send + Sync + Fn((Api, ChatLists)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  chat_member:
    Option<Arc<dyn Send + Sync + Fn((Api, ChatMember)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  chat_members:
    Option<Arc<dyn Send + Sync + Fn((Api, ChatMembers)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  chat_message_senders: Option<
    Arc<dyn Send + Sync + Fn((Api, ChatMessageSenders)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  chat_photos:
    Option<Arc<dyn Send + Sync + Fn((Api, ChatPhotos)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  chat_revenue_statistics: Option<
    Arc<
      dyn Send + Sync + Fn((Api, ChatRevenueStatistics)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  chat_revenue_transactions: Option<
    Arc<
      dyn Send + Sync + Fn((Api, ChatRevenueTransactions)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  chats: Option<Arc<dyn Send + Sync + Fn((Api, Chats)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  collectible_item_info: Option<
    Arc<dyn Send + Sync + Fn((Api, CollectibleItemInfo)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  community_full_info: Option<
    Arc<dyn Send + Sync + Fn((Api, CommunityFullInfo)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  community_id:
    Option<Arc<dyn Send + Sync + Fn((Api, CommunityId)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  connected_affiliate_program: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ConnectedAffiliateProgram)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  connected_affiliate_programs: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ConnectedAffiliatePrograms)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  connected_websites: Option<
    Arc<dyn Send + Sync + Fn((Api, ConnectedWebsites)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  count: Option<Arc<dyn Send + Sync + Fn((Api, Count)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  countries:
    Option<Arc<dyn Send + Sync + Fn((Api, Countries)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  country_info:
    Option<Arc<dyn Send + Sync + Fn((Api, CountryInfo)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  created_basic_group_chat: Option<
    Arc<
      dyn Send + Sync + Fn((Api, CreatedBasicGroupChat)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  currencies:
    Option<Arc<dyn Send + Sync + Fn((Api, Currencies)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  currency_exchange_rates: Option<
    Arc<
      dyn Send + Sync + Fn((Api, CurrencyExchangeRates)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  current_weather: Option<
    Arc<dyn Send + Sync + Fn((Api, CurrentWeather)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  custom_request_result: Option<
    Arc<dyn Send + Sync + Fn((Api, CustomRequestResult)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  data: Option<Arc<dyn Send + Sync + Fn((Api, Data)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  database_statistics: Option<
    Arc<dyn Send + Sync + Fn((Api, DatabaseStatistics)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  deep_link_info:
    Option<Arc<dyn Send + Sync + Fn((Api, DeepLinkInfo)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  direct_messages_chat_topic: Option<
    Arc<
      dyn Send + Sync + Fn((Api, DirectMessagesChatTopic)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  email_address_authentication_code_info: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, EmailAddressAuthenticationCodeInfo)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  emoji_categories: Option<
    Arc<dyn Send + Sync + Fn((Api, EmojiCategories)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  emoji_keywords: Option<
    Arc<dyn Send + Sync + Fn((Api, EmojiKeywords)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  emoji_reaction: Option<
    Arc<dyn Send + Sync + Fn((Api, EmojiReaction)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  emoji_status_custom_emojis: Option<
    Arc<
      dyn Send + Sync + Fn((Api, EmojiStatusCustomEmojis)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  emoji_statuses: Option<
    Arc<dyn Send + Sync + Fn((Api, EmojiStatuses)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  emojis: Option<Arc<dyn Send + Sync + Fn((Api, Emojis)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  error: Option<Arc<dyn Send + Sync + Fn((Api, Error)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  failed_to_add_members: Option<
    Arc<dyn Send + Sync + Fn((Api, FailedToAddMembers)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  file: Option<Arc<dyn Send + Sync + Fn((Api, File)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  file_downloaded_prefix_size: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, FileDownloadedPrefixSize)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  fixed_text:
    Option<Arc<dyn Send + Sync + Fn((Api, FixedText)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  formatted_text: Option<
    Arc<dyn Send + Sync + Fn((Api, FormattedText)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  forum_topic:
    Option<Arc<dyn Send + Sync + Fn((Api, ForumTopic)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  forum_topic_info: Option<
    Arc<dyn Send + Sync + Fn((Api, ForumTopicInfo)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  forum_topics:
    Option<Arc<dyn Send + Sync + Fn((Api, ForumTopics)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  found_affiliate_programs: Option<
    Arc<
      dyn Send + Sync + Fn((Api, FoundAffiliatePrograms)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  found_chat_boosts: Option<
    Arc<dyn Send + Sync + Fn((Api, FoundChatBoosts)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  found_chat_messages: Option<
    Arc<dyn Send + Sync + Fn((Api, FoundChatMessages)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  found_file_downloads: Option<
    Arc<dyn Send + Sync + Fn((Api, FoundFileDownloads)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  found_messages: Option<
    Arc<dyn Send + Sync + Fn((Api, FoundMessages)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  found_position: Option<
    Arc<dyn Send + Sync + Fn((Api, FoundPosition)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  found_positions: Option<
    Arc<dyn Send + Sync + Fn((Api, FoundPositions)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  found_public_posts: Option<
    Arc<dyn Send + Sync + Fn((Api, FoundPublicPosts)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  found_stories:
    Option<Arc<dyn Send + Sync + Fn((Api, FoundStories)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  found_users:
    Option<Arc<dyn Send + Sync + Fn((Api, FoundUsers)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  found_web_app:
    Option<Arc<dyn Send + Sync + Fn((Api, FoundWebApp)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  game_high_scores: Option<
    Arc<dyn Send + Sync + Fn((Api, GameHighScores)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  gift_auction_acquired_gifts: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, GiftAuctionAcquiredGifts)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  gift_auction_state: Option<
    Arc<dyn Send + Sync + Fn((Api, GiftAuctionState)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  gift_chat_themes: Option<
    Arc<dyn Send + Sync + Fn((Api, GiftChatThemes)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  gift_collection: Option<
    Arc<dyn Send + Sync + Fn((Api, GiftCollection)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  gift_collections: Option<
    Arc<dyn Send + Sync + Fn((Api, GiftCollections)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  gift_upgrade_preview: Option<
    Arc<dyn Send + Sync + Fn((Api, GiftUpgradePreview)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  gift_upgrade_variants: Option<
    Arc<dyn Send + Sync + Fn((Api, GiftUpgradeVariants)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  gifts_for_crafting: Option<
    Arc<dyn Send + Sync + Fn((Api, GiftsForCrafting)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  gifts_for_resale: Option<
    Arc<dyn Send + Sync + Fn((Api, GiftsForResale)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  gram_revenue_statistics: Option<
    Arc<
      dyn Send + Sync + Fn((Api, GramRevenueStatistics)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  group_call:
    Option<Arc<dyn Send + Sync + Fn((Api, GroupCall)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  group_call_id:
    Option<Arc<dyn Send + Sync + Fn((Api, GroupCallId)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  group_call_info: Option<
    Arc<dyn Send + Sync + Fn((Api, GroupCallInfo)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  group_call_participant: Option<
    Arc<dyn Send + Sync + Fn((Api, GroupCallParticipant)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  group_call_participants: Option<
    Arc<
      dyn Send + Sync + Fn((Api, GroupCallParticipants)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  group_call_streams: Option<
    Arc<dyn Send + Sync + Fn((Api, GroupCallStreams)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  hashtags:
    Option<Arc<dyn Send + Sync + Fn((Api, Hashtags)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  http_url:
    Option<Arc<dyn Send + Sync + Fn((Api, HttpUrl)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  imported_contacts: Option<
    Arc<dyn Send + Sync + Fn((Api, ImportedContacts)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  inline_message_id: Option<
    Arc<dyn Send + Sync + Fn((Api, InlineMessageId)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  inline_query_results: Option<
    Arc<dyn Send + Sync + Fn((Api, InlineQueryResults)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  keyboard_button: Option<
    Arc<dyn Send + Sync + Fn((Api, KeyboardButton)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  language_pack_info: Option<
    Arc<dyn Send + Sync + Fn((Api, LanguagePackInfo)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  language_pack_strings: Option<
    Arc<dyn Send + Sync + Fn((Api, LanguagePackStrings)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  link_preview:
    Option<Arc<dyn Send + Sync + Fn((Api, LinkPreview)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  live_story_donors: Option<
    Arc<dyn Send + Sync + Fn((Api, LiveStoryDonors)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  localization_target_info: Option<
    Arc<
      dyn Send + Sync + Fn((Api, LocalizationTargetInfo)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  log_tags:
    Option<Arc<dyn Send + Sync + Fn((Api, LogTags)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  log_verbosity_level: Option<
    Arc<dyn Send + Sync + Fn((Api, LogVerbosityLevel)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  main_web_app:
    Option<Arc<dyn Send + Sync + Fn((Api, MainWebApp)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  message:
    Option<Arc<dyn Send + Sync + Fn((Api, Message)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  message_auto_delete_time: Option<
    Arc<
      dyn Send + Sync + Fn((Api, MessageAutoDeleteTime)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  message_calendar: Option<
    Arc<dyn Send + Sync + Fn((Api, MessageCalendar)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  message_effect: Option<
    Arc<dyn Send + Sync + Fn((Api, MessageEffect)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  message_link:
    Option<Arc<dyn Send + Sync + Fn((Api, MessageLink)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  message_link_info: Option<
    Arc<dyn Send + Sync + Fn((Api, MessageLinkInfo)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  message_positions: Option<
    Arc<dyn Send + Sync + Fn((Api, MessagePositions)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  message_properties: Option<
    Arc<dyn Send + Sync + Fn((Api, MessageProperties)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  message_senders: Option<
    Arc<dyn Send + Sync + Fn((Api, MessageSenders)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  message_statistics: Option<
    Arc<dyn Send + Sync + Fn((Api, MessageStatistics)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  message_thread_info: Option<
    Arc<dyn Send + Sync + Fn((Api, MessageThreadInfo)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  message_viewers: Option<
    Arc<dyn Send + Sync + Fn((Api, MessageViewers)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  messages:
    Option<Arc<dyn Send + Sync + Fn((Api, Messages)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  network_statistics: Option<
    Arc<dyn Send + Sync + Fn((Api, NetworkStatistics)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  new_chat_privacy_settings: Option<
    Arc<
      dyn Send + Sync + Fn((Api, NewChatPrivacySettings)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  notification_sound: Option<
    Arc<dyn Send + Sync + Fn((Api, NotificationSound)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  notification_sounds: Option<
    Arc<dyn Send + Sync + Fn((Api, NotificationSounds)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  oauth_link_info: Option<
    Arc<dyn Send + Sync + Fn((Api, OauthLinkInfo)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  ok: Option<Arc<dyn Send + Sync + Fn((Api, Ok)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  on_ramp_payment_availability: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, OnRampPaymentAvailability)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  on_ramp_payment_limits: Option<
    Arc<dyn Send + Sync + Fn((Api, OnRampPaymentLimits)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  on_ramp_payment_quote: Option<
    Arc<dyn Send + Sync + Fn((Api, OnRampPaymentQuote)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  on_ramp_payment_session: Option<
    Arc<dyn Send + Sync + Fn((Api, OnRampPaymentSession)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  on_ramp_providers: Option<
    Arc<dyn Send + Sync + Fn((Api, OnRampProviders)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  order_info:
    Option<Arc<dyn Send + Sync + Fn((Api, OrderInfo)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  outline:
    Option<Arc<dyn Send + Sync + Fn((Api, Outline)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  passkey:
    Option<Arc<dyn Send + Sync + Fn((Api, Passkey)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  passkeys:
    Option<Arc<dyn Send + Sync + Fn((Api, Passkeys)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  passport_authorization_form: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, PassportAuthorizationForm)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  passport_elements: Option<
    Arc<dyn Send + Sync + Fn((Api, PassportElements)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  passport_elements_with_errors: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, PassportElementsWithErrors)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  password_state: Option<
    Arc<dyn Send + Sync + Fn((Api, PasswordState)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  payment_form:
    Option<Arc<dyn Send + Sync + Fn((Api, PaymentForm)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  payment_receipt: Option<
    Arc<dyn Send + Sync + Fn((Api, PaymentReceipt)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  payment_result: Option<
    Arc<dyn Send + Sync + Fn((Api, PaymentResult)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  phone_number_info: Option<
    Arc<dyn Send + Sync + Fn((Api, PhoneNumberInfo)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  poll_option_properties: Option<
    Arc<dyn Send + Sync + Fn((Api, PollOptionProperties)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  poll_vote_statistics: Option<
    Arc<dyn Send + Sync + Fn((Api, PollVoteStatistics)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  poll_voters:
    Option<Arc<dyn Send + Sync + Fn((Api, PollVoters)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  premium_features: Option<
    Arc<dyn Send + Sync + Fn((Api, PremiumFeatures)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  premium_gift_code_info: Option<
    Arc<dyn Send + Sync + Fn((Api, PremiumGiftCodeInfo)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  premium_gift_payment_options: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, PremiumGiftPaymentOptions)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  premium_giveaway_payment_options: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, PremiumGiveawayPaymentOptions)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  premium_limit:
    Option<Arc<dyn Send + Sync + Fn((Api, PremiumLimit)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  premium_state:
    Option<Arc<dyn Send + Sync + Fn((Api, PremiumState)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  prepared_inline_message: Option<
    Arc<
      dyn Send + Sync + Fn((Api, PreparedInlineMessage)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  prepared_inline_message_id: Option<
    Arc<
      dyn Send + Sync + Fn((Api, PreparedInlineMessageId)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  profile_share_theme_backdrops: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ProfileShareThemeBackdrops)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  public_forwards: Option<
    Arc<dyn Send + Sync + Fn((Api, PublicForwards)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  public_post_search_limits: Option<
    Arc<
      dyn Send + Sync + Fn((Api, PublicPostSearchLimits)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  push_receiver_id: Option<
    Arc<dyn Send + Sync + Fn((Api, PushReceiverId)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  quick_reply_message: Option<
    Arc<dyn Send + Sync + Fn((Api, QuickReplyMessage)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  quick_reply_messages: Option<
    Arc<dyn Send + Sync + Fn((Api, QuickReplyMessages)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  read_date_privacy_settings: Option<
    Arc<
      dyn Send + Sync + Fn((Api, ReadDatePrivacySettings)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  received_gift:
    Option<Arc<dyn Send + Sync + Fn((Api, ReceivedGift)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  received_gifts: Option<
    Arc<dyn Send + Sync + Fn((Api, ReceivedGifts)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  recommended_chat_folders: Option<
    Arc<
      dyn Send + Sync + Fn((Api, RecommendedChatFolders)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  recovery_email_address: Option<
    Arc<dyn Send + Sync + Fn((Api, RecoveryEmailAddress)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  rich_message:
    Option<Arc<dyn Send + Sync + Fn((Api, RichMessage)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  rtmp_url:
    Option<Arc<dyn Send + Sync + Fn((Api, RtmpUrl)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  saved_messages_tags: Option<
    Arc<dyn Send + Sync + Fn((Api, SavedMessagesTags)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  scope_notification_settings: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ScopeNotificationSettings)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  seconds:
    Option<Arc<dyn Send + Sync + Fn((Api, Seconds)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  secret_chat:
    Option<Arc<dyn Send + Sync + Fn((Api, SecretChat)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  session:
    Option<Arc<dyn Send + Sync + Fn((Api, Session)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  sessions:
    Option<Arc<dyn Send + Sync + Fn((Api, Sessions)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  sponsored_chats: Option<
    Arc<dyn Send + Sync + Fn((Api, SponsoredChats)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  sponsored_messages: Option<
    Arc<dyn Send + Sync + Fn((Api, SponsoredMessages)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  stake_dice_state: Option<
    Arc<dyn Send + Sync + Fn((Api, StakeDiceState)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  star_amount:
    Option<Arc<dyn Send + Sync + Fn((Api, StarAmount)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  star_count:
    Option<Arc<dyn Send + Sync + Fn((Api, StarCount)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  star_giveaway_payment_options: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, StarGiveawayPaymentOptions)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  star_payment_options: Option<
    Arc<dyn Send + Sync + Fn((Api, StarPaymentOptions)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  star_revenue_statistics: Option<
    Arc<
      dyn Send + Sync + Fn((Api, StarRevenueStatistics)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  star_subscriptions: Option<
    Arc<dyn Send + Sync + Fn((Api, StarSubscriptions)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  star_transactions: Option<
    Arc<dyn Send + Sync + Fn((Api, StarTransactions)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  sticker:
    Option<Arc<dyn Send + Sync + Fn((Api, Sticker)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  sticker_set:
    Option<Arc<dyn Send + Sync + Fn((Api, StickerSet)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  sticker_sets:
    Option<Arc<dyn Send + Sync + Fn((Api, StickerSets)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  stickers:
    Option<Arc<dyn Send + Sync + Fn((Api, Stickers)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  storage_statistics: Option<
    Arc<dyn Send + Sync + Fn((Api, StorageStatistics)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  storage_statistics_fast: Option<
    Arc<
      dyn Send + Sync + Fn((Api, StorageStatisticsFast)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  stories:
    Option<Arc<dyn Send + Sync + Fn((Api, Stories)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  story: Option<Arc<dyn Send + Sync + Fn((Api, Story)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  story_album:
    Option<Arc<dyn Send + Sync + Fn((Api, StoryAlbum)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  story_albums:
    Option<Arc<dyn Send + Sync + Fn((Api, StoryAlbums)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  story_interactions: Option<
    Arc<dyn Send + Sync + Fn((Api, StoryInteractions)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  story_statistics: Option<
    Arc<dyn Send + Sync + Fn((Api, StoryStatistics)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  supergroup:
    Option<Arc<dyn Send + Sync + Fn((Api, Supergroup)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  supergroup_full_info: Option<
    Arc<dyn Send + Sync + Fn((Api, SupergroupFullInfo)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  t_me_urls:
    Option<Arc<dyn Send + Sync + Fn((Api, TMeUrls)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  temporary_password_state: Option<
    Arc<
      dyn Send + Sync + Fn((Api, TemporaryPasswordState)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  test_bytes:
    Option<Arc<dyn Send + Sync + Fn((Api, TestBytes)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  test_int:
    Option<Arc<dyn Send + Sync + Fn((Api, TestInt)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  test_string:
    Option<Arc<dyn Send + Sync + Fn((Api, TestString)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  test_vector_int: Option<
    Arc<dyn Send + Sync + Fn((Api, TestVectorInt)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  test_vector_int_object: Option<
    Arc<dyn Send + Sync + Fn((Api, TestVectorIntObject)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  test_vector_string: Option<
    Arc<dyn Send + Sync + Fn((Api, TestVectorString)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  test_vector_string_object: Option<
    Arc<
      dyn Send + Sync + Fn((Api, TestVectorStringObject)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  text: Option<Arc<dyn Send + Sync + Fn((Api, Text)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  text_composition_style: Option<
    Arc<dyn Send + Sync + Fn((Api, TextCompositionStyle)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  text_composition_style_example: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, TextCompositionStyleExample)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  text_entities:
    Option<Arc<dyn Send + Sync + Fn((Api, TextEntities)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  time_zones:
    Option<Arc<dyn Send + Sync + Fn((Api, TimeZones)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  ton_center_streaming_api_url: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, TonCenterStreamingApiUrl)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  ton_connect_challenge: Option<
    Arc<dyn Send + Sync + Fn((Api, TonConnectChallenge)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  ton_connect_requests: Option<
    Arc<dyn Send + Sync + Fn((Api, TonConnectRequests)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  ton_connect_session: Option<
    Arc<dyn Send + Sync + Fn((Api, TonConnectSession)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  ton_connect_session_event_id: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, TonConnectSessionEventId)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  ton_connect_sessions: Option<
    Arc<dyn Send + Sync + Fn((Api, TonConnectSessions)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  ton_nfts:
    Option<Arc<dyn Send + Sync + Fn((Api, TonNfts)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  ton_transactions: Option<
    Arc<dyn Send + Sync + Fn((Api, TonTransactions)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  ton_wallet_card: Option<
    Arc<dyn Send + Sync + Fn((Api, TonWalletCard)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  ton_wallet_ownership_proof_challenge: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, TonWalletOwnershipProofChallenge)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  ton_wallet_secret_phrase: Option<
    Arc<
      dyn Send + Sync + Fn((Api, TonWalletSecretPhrase)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  ton_wallet_transaction: Option<
    Arc<dyn Send + Sync + Fn((Api, TonWalletTransaction)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  ton_wallet_transactions: Option<
    Arc<
      dyn Send + Sync + Fn((Api, TonWalletTransactions)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  ton_wallet_transfer_result: Option<
    Arc<
      dyn Send + Sync + Fn((Api, TonWalletTransferResult)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  trending_sticker_sets: Option<
    Arc<dyn Send + Sync + Fn((Api, TrendingStickerSets)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  updates:
    Option<Arc<dyn Send + Sync + Fn((Api, Updates)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  upgrade_gift_result: Option<
    Arc<dyn Send + Sync + Fn((Api, UpgradeGiftResult)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  upgraded_gift:
    Option<Arc<dyn Send + Sync + Fn((Api, UpgradedGift)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  upgraded_gift_collection_names: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpgradedGiftCollectionNames)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  upgraded_gift_deposit_info: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpgradedGiftDepositInfo)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  upgraded_gift_value_info: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpgradedGiftValueInfo)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  user: Option<Arc<dyn Send + Sync + Fn((Api, User)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  user_full_info:
    Option<Arc<dyn Send + Sync + Fn((Api, UserFullInfo)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  user_link:
    Option<Arc<dyn Send + Sync + Fn((Api, UserLink)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  user_privacy_setting_rules: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UserPrivacySettingRules)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  user_support_info: Option<
    Arc<dyn Send + Sync + Fn((Api, UserSupportInfo)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  user_ton_wallet_address: Option<
    Arc<dyn Send + Sync + Fn((Api, UserTonWalletAddress)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  user_ton_wallet_addresses: Option<
    Arc<
      dyn Send + Sync + Fn((Api, UserTonWalletAddresses)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  users: Option<Arc<dyn Send + Sync + Fn((Api, Users)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  validated_order_info: Option<
    Arc<dyn Send + Sync + Fn((Api, ValidatedOrderInfo)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  video_message_advertisements: Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, VideoMessageAdvertisements)) -> LocalBoxFuture<'static, TGResult<()>>,
    >,
  >,
  wallet_bot_balance: Option<
    Arc<dyn Send + Sync + Fn((Api, WalletBotBalance)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
  web_app_info:
    Option<Arc<dyn Send + Sync + Fn((Api, WebAppInfo)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  web_app_url:
    Option<Arc<dyn Send + Sync + Fn((Api, WebAppUrl)) -> LocalBoxFuture<'static, TGResult<()>>>>,
  web_page_instant_view: Option<
    Arc<dyn Send + Sync + Fn((Api, WebPageInstantView)) -> LocalBoxFuture<'static, TGResult<()>>>,
  >,
}

impl RasyncListener {
  pub fn new() -> Self {
    RasyncListener::default()
  }

  #[allow(dead_code)]
  pub(crate) fn has_receive_listen(&self) -> bool {
    self.receive.is_some()
  }

  pub(crate) fn lout(&self) -> RasyncLout {
    RasyncLout::new(self.clone())
  }

  /// when receive data from tdlib
  pub fn on_receive<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, String)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.receive = Some(Arc::new(fnc));
    self
  }

  /// when telegram client throw exception
  pub fn on_exception<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, TGError)) -> LocalBoxFuture<'static, ()> + 'static,
  {
    self.exception = Some(Arc::new(fnc));
    self
  }

  /// Does nothing and ensures that the Update object is used; for testing only. This is an offline method. Can be called before authorization
  pub fn on_test_use_update<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, TestUseUpdate)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.test_use_update = Some(Arc::new(fnc));
    self
  }

  /// The list of supported accent colors has changed
  pub fn on_update_accent_colors<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateAccentColors)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_accent_colors = Some(Arc::new(fnc));
    self
  }

  /// The list of active emoji reactions has changed
  pub fn on_update_active_emoji_reactions<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateActiveEmojiReactions)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_active_emoji_reactions = Some(Arc::new(fnc));
    self
  }

  /// The list of auctions in which the current user participates has changed
  pub fn on_update_active_gift_auctions<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateActiveGiftAuctions)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_active_gift_auctions = Some(Arc::new(fnc));
    self
  }

  /// The list of messages with active live location that need to be updated by the application has changed. The list is persistent across application restarts only if the message database is used
  pub fn on_update_active_live_location_messages<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateActiveLiveLocationMessages)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_active_live_location_messages = Some(Arc::new(fnc));
    self
  }

  /// Contains active notifications that were shown on previous application launches. This update is sent only if the message database is used. In that case it comes once before any updateNotification and updateNotificationGroup update
  pub fn on_update_active_notifications<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateActiveNotifications)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_active_notifications = Some(Arc::new(fnc));
    self
  }

  /// The parameters for age verification of the current user's account have changed
  pub fn on_update_age_verification_parameters<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateAgeVerificationParameters)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_age_verification_parameters = Some(Arc::new(fnc));
    self
  }

  /// Lists of suffixes allowed in secondary bot usernames, which can be added additionally to the main username if the bot's owner has a Telegram Premium subscription
  pub fn on_update_allowed_bot_username_premium_suffixes<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateAllowedBotUsernamePremiumSuffixes)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_allowed_bot_username_premium_suffixes = Some(Arc::new(fnc));
    self
  }

  /// Some animated emoji message was clicked and a big animated sticker must be played if the message is visible on the screen. chatActionWatchingAnimations with the text of the message needs to be sent if the sticker is played
  pub fn on_update_animated_emoji_message_clicked<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateAnimatedEmojiMessageClicked)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_animated_emoji_message_clicked = Some(Arc::new(fnc));
    self
  }

  /// The parameters of animation search through getOption("animation_search_bot_username") bot have changed
  pub fn on_update_animation_search_parameters<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateAnimationSearchParameters)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_animation_search_parameters = Some(Arc::new(fnc));
    self
  }

  /// A request can't be completed unless reCAPTCHA verification is performed; for official mobile applications only. The method setApplicationVerificationToken must be called once the verification is completed or failed
  pub fn on_update_application_recaptcha_verification_required<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn(
        (Api, UpdateApplicationRecaptchaVerificationRequired),
      ) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_application_recaptcha_verification_required = Some(Arc::new(fnc));
    self
  }

  /// A request can't be completed unless application verification is performed; for official mobile applications only. The method setApplicationVerificationToken must be called once the verification is completed or failed
  pub fn on_update_application_verification_required<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateApplicationVerificationRequired)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_application_verification_required = Some(Arc::new(fnc));
    self
  }

  /// The list of bots added to attachment or side menu has changed
  pub fn on_update_attachment_menu_bots<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateAttachmentMenuBots)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_attachment_menu_bots = Some(Arc::new(fnc));
    self
  }

  /// The user authorization state has changed
  pub fn on_update_authorization_state<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateAuthorizationState)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_authorization_state = Some(Arc::new(fnc));
    self
  }

  /// Autosave settings for some type of chats were updated
  pub fn on_update_autosave_settings<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateAutosaveSettings)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_autosave_settings = Some(Arc::new(fnc));
    self
  }

  /// The list of available message effects has changed
  pub fn on_update_available_message_effects<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateAvailableMessageEffects)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_available_message_effects = Some(Arc::new(fnc));
    self
  }

  /// Some data of a basic group has changed. This update is guaranteed to come before the basic group identifier is returned to the application
  pub fn on_update_basic_group<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UpdateBasicGroup)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.update_basic_group = Some(Arc::new(fnc));
    self
  }

  /// Some data in basicGroupFullInfo has been changed
  pub fn on_update_basic_group_full_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateBasicGroupFullInfo)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_basic_group_full_info = Some(Arc::new(fnc));
    self
  }

  /// A business connection has changed; for bots only
  pub fn on_update_business_connection<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateBusinessConnection)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_business_connection = Some(Arc::new(fnc));
    self
  }

  /// A message in a business account was edited; for bots only
  pub fn on_update_business_message_edited<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateBusinessMessageEdited)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_business_message_edited = Some(Arc::new(fnc));
    self
  }

  /// Messages in a business account were deleted; for bots only
  pub fn on_update_business_messages_deleted<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateBusinessMessagesDeleted)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_business_messages_deleted = Some(Arc::new(fnc));
    self
  }

  /// New call was created or information about a call was updated
  pub fn on_update_call<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UpdateCall)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.update_call = Some(Arc::new(fnc));
    self
  }

  /// Chat accent colors have changed
  pub fn on_update_chat_accent_colors<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatAccentColors)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_accent_colors = Some(Arc::new(fnc));
    self
  }

  /// A message sender activity in the chat has changed
  pub fn on_update_chat_action<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UpdateChatAction)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.update_chat_action = Some(Arc::new(fnc));
    self
  }

  /// The chat action bar was changed
  pub fn on_update_chat_action_bar<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatActionBar)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_action_bar = Some(Arc::new(fnc));
    self
  }

  /// The list of active stories posted by a specific chat has changed
  pub fn on_update_chat_active_stories<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatActiveStories)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_active_stories = Some(Arc::new(fnc));
    self
  }

  /// A chat was added to a chat list
  pub fn on_update_chat_added_to_list<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatAddedToList)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_added_to_list = Some(Arc::new(fnc));
    self
  }

  /// The chat available reactions were changed
  pub fn on_update_chat_available_reactions<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatAvailableReactions)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_available_reactions = Some(Arc::new(fnc));
    self
  }

  /// The chat background was changed
  pub fn on_update_chat_background<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatBackground)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_background = Some(Arc::new(fnc));
    self
  }

  /// A chat was blocked or unblocked
  pub fn on_update_chat_block_list<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatBlockList)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_block_list = Some(Arc::new(fnc));
    self
  }

  /// A chat boost has changed; for bots only
  pub fn on_update_chat_boost<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UpdateChatBoost)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.update_chat_boost = Some(Arc::new(fnc));
    self
  }

  /// The bar for managing business bot was changed in a chat
  pub fn on_update_chat_business_bot_manage_bar<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatBusinessBotManageBar)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_business_bot_manage_bar = Some(Arc::new(fnc));
    self
  }

  /// The value of the default disable_notification parameter, used when a message is sent to the chat, was changed
  pub fn on_update_chat_default_disable_notification<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatDefaultDisableNotification)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_default_disable_notification = Some(Arc::new(fnc));
    self
  }

  /// A chat draft has changed. Be aware that the update may come in the currently opened chat but with old content of the draft. If the user has changed the content of the draft, this update mustn't be applied
  pub fn on_update_chat_draft_message<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatDraftMessage)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_draft_message = Some(Arc::new(fnc));
    self
  }

  /// Chat emoji status has changed
  pub fn on_update_chat_emoji_status<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatEmojiStatus)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_emoji_status = Some(Arc::new(fnc));
    self
  }

  /// The list of chat folders or a chat folder has changed
  pub fn on_update_chat_folders<F>(&mut self, fnc: F) -> &mut Self
  where
    F:
      Send + Sync + Fn((Api, UpdateChatFolders)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.update_chat_folders = Some(Arc::new(fnc));
    self
  }

  /// A chat content was allowed or restricted for saving
  pub fn on_update_chat_has_protected_content<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatHasProtectedContent)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_has_protected_content = Some(Arc::new(fnc));
    self
  }

  /// A chat's has_scheduled_messages field has changed
  pub fn on_update_chat_has_scheduled_messages<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatHasScheduledMessages)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_has_scheduled_messages = Some(Arc::new(fnc));
    self
  }

  /// A chat's has_welcome_messages field has changed
  pub fn on_update_chat_has_welcome_messages<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatHasWelcomeMessages)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_has_welcome_messages = Some(Arc::new(fnc));
    self
  }

  /// A chat was marked as unread or was read
  pub fn on_update_chat_is_marked_as_unread<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatIsMarkedAsUnread)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_is_marked_as_unread = Some(Arc::new(fnc));
    self
  }

  /// Translation of chat messages was enabled or disabled
  pub fn on_update_chat_is_translatable<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatIsTranslatable)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_is_translatable = Some(Arc::new(fnc));
    self
  }

  /// A join request from the user was completed
  pub fn on_update_chat_join_result<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatJoinResult)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_join_result = Some(Arc::new(fnc));
    self
  }

  /// The last message of a chat was changed
  pub fn on_update_chat_last_message<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatLastMessage)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_last_message = Some(Arc::new(fnc));
    self
  }

  /// User rights changed in a chat; for bots only
  pub fn on_update_chat_member<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UpdateChatMember)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.update_chat_member = Some(Arc::new(fnc));
    self
  }

  /// The message auto-delete or self-destruct timer setting for a chat was changed
  pub fn on_update_chat_message_auto_delete_time<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatMessageAutoDeleteTime)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_message_auto_delete_time = Some(Arc::new(fnc));
    self
  }

  /// The message sender that is selected to send messages in a chat has changed
  pub fn on_update_chat_message_sender<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatMessageSender)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_message_sender = Some(Arc::new(fnc));
    self
  }

  /// Notification settings for a chat were changed
  pub fn on_update_chat_notification_settings<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatNotificationSettings)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_notification_settings = Some(Arc::new(fnc));
    self
  }

  /// The number of online group members has changed. This update with non-zero number of online group members is sent only for currently opened chats. There is no guarantee that it is sent just after the number of online users has changed
  pub fn on_update_chat_online_member_count<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatOnlineMemberCount)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_online_member_count = Some(Arc::new(fnc));
    self
  }

  /// The chat pending join requests were changed
  pub fn on_update_chat_pending_join_requests<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatPendingJoinRequests)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_pending_join_requests = Some(Arc::new(fnc));
    self
  }

  /// Chat permissions were changed
  pub fn on_update_chat_permissions<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatPermissions)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_permissions = Some(Arc::new(fnc));
    self
  }

  /// A chat photo was changed
  pub fn on_update_chat_photo<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UpdateChatPhoto)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.update_chat_photo = Some(Arc::new(fnc));
    self
  }

  /// The position of a chat in a chat list has changed. An updateChatLastMessage or updateChatDraftMessage update might be sent instead of the update
  pub fn on_update_chat_position<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatPosition)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_position = Some(Arc::new(fnc));
    self
  }

  /// Incoming messages were read or the number of unread messages has been changed
  pub fn on_update_chat_read_inbox<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatReadInbox)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_read_inbox = Some(Arc::new(fnc));
    self
  }

  /// Outgoing messages were read
  pub fn on_update_chat_read_outbox<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatReadOutbox)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_read_outbox = Some(Arc::new(fnc));
    self
  }

  /// A chat was removed from a chat list
  pub fn on_update_chat_removed_from_list<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatRemovedFromList)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_removed_from_list = Some(Arc::new(fnc));
    self
  }

  /// The chat reply markup was changed
  pub fn on_update_chat_reply_markup<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatReplyMarkup)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_reply_markup = Some(Arc::new(fnc));
    self
  }

  /// The revenue earned from sponsored messages in a chat has changed. If chat revenue screen is opened, then getChatRevenueTransactions may be called to fetch new transactions
  pub fn on_update_chat_revenue_amount<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatRevenueAmount)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_revenue_amount = Some(Arc::new(fnc));
    self
  }

  /// The chat theme was changed
  pub fn on_update_chat_theme<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UpdateChatTheme)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.update_chat_theme = Some(Arc::new(fnc));
    self
  }

  /// The title of a chat was changed
  pub fn on_update_chat_title<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UpdateChatTitle)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.update_chat_title = Some(Arc::new(fnc));
    self
  }

  /// The chat unread_mention_count has changed
  pub fn on_update_chat_unread_mention_count<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatUnreadMentionCount)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_unread_mention_count = Some(Arc::new(fnc));
    self
  }

  /// The chat unread_poll_vote_count has changed
  pub fn on_update_chat_unread_poll_vote_count<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatUnreadPollVoteCount)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_unread_poll_vote_count = Some(Arc::new(fnc));
    self
  }

  /// The chat unread_reaction_count has changed
  pub fn on_update_chat_unread_reaction_count<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatUnreadReactionCount)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_unread_reaction_count = Some(Arc::new(fnc));
    self
  }

  /// A chat video chat state has changed
  pub fn on_update_chat_video_chat<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatVideoChat)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_video_chat = Some(Arc::new(fnc));
    self
  }

  /// A chat default appearance has changed
  pub fn on_update_chat_view_as_topics<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatViewAsTopics)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_view_as_topics = Some(Arc::new(fnc));
    self
  }

  /// The list of welcome messages of a chat has changed
  pub fn on_update_chat_welcome_messages<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateChatWelcomeMessages)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_chat_welcome_messages = Some(Arc::new(fnc));
    self
  }

  /// Some data of a community has changed. This update is guaranteed to come before the community identifier is returned to the application
  pub fn on_update_community<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UpdateCommunity)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.update_community = Some(Arc::new(fnc));
    self
  }

  /// Some data in communityFullInfo has been changed
  pub fn on_update_community_full_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateCommunityFullInfo)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_community_full_info = Some(Arc::new(fnc));
    self
  }

  /// The connection state has changed. This update must be used only to show a human-readable description of the connection state
  pub fn on_update_connection_state<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateConnectionState)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_connection_state = Some(Arc::new(fnc));
    self
  }

  /// The list of contacts that had birthdays recently or will have birthday soon has changed
  pub fn on_update_contact_close_birthdays<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateContactCloseBirthdays)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_contact_close_birthdays = Some(Arc::new(fnc));
    self
  }

  /// The default background has changed
  pub fn on_update_default_background<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateDefaultBackground)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_default_background = Some(Arc::new(fnc));
    self
  }

  /// The type of default paid reaction has changed
  pub fn on_update_default_paid_reaction_type<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateDefaultPaidReactionType)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_default_paid_reaction_type = Some(Arc::new(fnc));
    self
  }

  /// The type of default reaction has changed
  pub fn on_update_default_reaction_type<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateDefaultReactionType)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_default_reaction_type = Some(Arc::new(fnc));
    self
  }

  /// Some messages were deleted
  pub fn on_update_delete_messages<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateDeleteMessages)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_delete_messages = Some(Arc::new(fnc));
    self
  }

  /// The list of supported dice emojis has changed
  pub fn on_update_dice_emojis<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UpdateDiceEmojis)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.update_dice_emojis = Some(Arc::new(fnc));
    self
  }

  /// Basic information about a topic in a channel direct messages chat administered by the current user has changed. This update is guaranteed to come before the topic identifier is returned to the application
  pub fn on_update_direct_messages_chat_topic<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateDirectMessagesChatTopic)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_direct_messages_chat_topic = Some(Arc::new(fnc));
    self
  }

  /// The list of available emoji chat themes has changed
  pub fn on_update_emoji_chat_themes<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateEmojiChatThemes)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_emoji_chat_themes = Some(Arc::new(fnc));
    self
  }

  /// The list of favorite stickers was updated
  pub fn on_update_favorite_stickers<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateFavoriteStickers)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_favorite_stickers = Some(Arc::new(fnc));
    self
  }

  /// Information about a file was updated
  pub fn on_update_file<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UpdateFile)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.update_file = Some(Arc::new(fnc));
    self
  }

  /// A file was added to the file download list. This update is sent only after file download list is loaded for the first time
  pub fn on_update_file_added_to_downloads<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateFileAddedToDownloads)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_file_added_to_downloads = Some(Arc::new(fnc));
    self
  }

  /// A file download was changed. This update is sent only after file download list is loaded for the first time
  pub fn on_update_file_download<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateFileDownload)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_file_download = Some(Arc::new(fnc));
    self
  }

  /// The state of the file download list has changed
  pub fn on_update_file_downloads<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateFileDownloads)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_file_downloads = Some(Arc::new(fnc));
    self
  }

  /// The file generation process needs to be started by the application. Use setFileGenerationProgress and finishFileGeneration to generate the file
  pub fn on_update_file_generation_start<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateFileGenerationStart)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_file_generation_start = Some(Arc::new(fnc));
    self
  }

  /// File generation is no longer needed
  pub fn on_update_file_generation_stop<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateFileGenerationStop)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_file_generation_stop = Some(Arc::new(fnc));
    self
  }

  /// A file was removed from the file download list. This update is sent only after file download list is loaded for the first time
  pub fn on_update_file_removed_from_downloads<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateFileRemovedFromDownloads)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_file_removed_from_downloads = Some(Arc::new(fnc));
    self
  }

  /// Information about a topic in a forum chat was changed
  pub fn on_update_forum_topic<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UpdateForumTopic)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.update_forum_topic = Some(Arc::new(fnc));
    self
  }

  /// Basic information about a topic in a forum chat was changed
  pub fn on_update_forum_topic_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateForumTopicInfo)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_forum_topic_info = Some(Arc::new(fnc));
    self
  }

  /// The freeze state of the current user's account has changed
  pub fn on_update_freeze_state<F>(&mut self, fnc: F) -> &mut Self
  where
    F:
      Send + Sync + Fn((Api, UpdateFreezeState)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.update_freeze_state = Some(Arc::new(fnc));
    self
  }

  /// State of a gift auction was updated
  pub fn on_update_gift_auction_state<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateGiftAuctionState)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_gift_auction_state = Some(Arc::new(fnc));
    self
  }

  /// The TON Gram revenue earned by the current user has changed. If Gram transaction screen of the chat is opened, then getTonTransactions may be called to fetch new transactions
  pub fn on_update_gram_revenue_status<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateGramRevenueStatus)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_gram_revenue_status = Some(Arc::new(fnc));
    self
  }

  /// Information about a group call was updated
  pub fn on_update_group_call<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UpdateGroupCall)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.update_group_call = Some(Arc::new(fnc));
    self
  }

  /// The levels of live story group call messages have changed
  pub fn on_update_group_call_message_levels<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateGroupCallMessageLevels)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_group_call_message_levels = Some(Arc::new(fnc));
    self
  }

  /// A group call message failed to send
  pub fn on_update_group_call_message_send_failed<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateGroupCallMessageSendFailed)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_group_call_message_send_failed = Some(Arc::new(fnc));
    self
  }

  /// Some group call messages were deleted
  pub fn on_update_group_call_messages_deleted<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateGroupCallMessagesDeleted)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_group_call_messages_deleted = Some(Arc::new(fnc));
    self
  }

  /// Information about a group call participant was changed. The updates are sent only after the group call is received through getGroupCall and only if the call is joined or being joined
  pub fn on_update_group_call_participant<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateGroupCallParticipant)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_group_call_participant = Some(Arc::new(fnc));
    self
  }

  /// The list of group call participants that can send and receive encrypted call data has changed; for group calls not bound to a chat only
  pub fn on_update_group_call_participants<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateGroupCallParticipants)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_group_call_participants = Some(Arc::new(fnc));
    self
  }

  /// The verification state of an encrypted group call has changed; for group calls not bound to a chat only
  pub fn on_update_group_call_verification_state<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateGroupCallVerificationState)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_group_call_verification_state = Some(Arc::new(fnc));
    self
  }

  /// Describes whether there are some pending notification updates. Can be used to prevent application from killing, while there are some pending notifications
  pub fn on_update_have_pending_notifications<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateHavePendingNotifications)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_have_pending_notifications = Some(Arc::new(fnc));
    self
  }

  /// The list of installed sticker sets was updated
  pub fn on_update_installed_sticker_sets<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateInstalledStickerSets)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_installed_sticker_sets = Some(Arc::new(fnc));
    self
  }

  /// Some language pack strings have been updated
  pub fn on_update_language_pack_strings<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateLanguagePackStrings)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_language_pack_strings = Some(Arc::new(fnc));
    self
  }

  /// The list of top donors in live story group call has changed
  pub fn on_update_live_story_top_donors<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateLiveStoryTopDonors)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_live_story_top_donors = Some(Arc::new(fnc));
    self
  }

  /// A bot that can be managed by the current bot was created or updated; for bots only
  pub fn on_update_managed_bot<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UpdateManagedBot)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.update_managed_bot = Some(Arc::new(fnc));
    self
  }

  /// Unread votes were added or removed from a poll message
  pub fn on_update_message_contains_unread_poll_votes<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateMessageContainsUnreadPollVotes)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_message_contains_unread_poll_votes = Some(Arc::new(fnc));
    self
  }

  /// The message content has changed
  pub fn on_update_message_content<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateMessageContent)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_message_content = Some(Arc::new(fnc));
    self
  }

  /// The message content was opened. Updates voice note messages to "listened", video note messages to "viewed" and starts the self-destruct timer
  pub fn on_update_message_content_opened<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateMessageContentOpened)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_message_content_opened = Some(Arc::new(fnc));
    self
  }

  /// A message was edited. Changes in the message content will come in a separate updateMessageContent
  pub fn on_update_message_edited<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateMessageEdited)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_message_edited = Some(Arc::new(fnc));
    self
  }

  /// The message ephemeral content has changed
  pub fn on_update_message_ephemeral_content<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateMessageEphemeralContent)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_message_ephemeral_content = Some(Arc::new(fnc));
    self
  }

  /// A fact-check added to a message was changed
  pub fn on_update_message_fact_check<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateMessageFactCheck)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_message_fact_check = Some(Arc::new(fnc));
    self
  }

  /// The information about interactions with a message has changed
  pub fn on_update_message_interaction_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateMessageInteractionInfo)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_message_interaction_info = Some(Arc::new(fnc));
    self
  }

  /// The message pinned state was changed
  pub fn on_update_message_is_pinned<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateMessageIsPinned)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_message_is_pinned = Some(Arc::new(fnc));
    self
  }

  /// A message with a live location was viewed. When the update is received, the application is expected to update the live location
  pub fn on_update_message_live_location_viewed<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateMessageLiveLocationViewed)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_message_live_location_viewed = Some(Arc::new(fnc));
    self
  }

  /// A message with an unread mention was read
  pub fn on_update_message_mention_read<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateMessageMentionRead)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_message_mention_read = Some(Arc::new(fnc));
    self
  }

  /// User changed its reactions on a message with public reactions; for bots only
  pub fn on_update_message_reaction<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateMessageReaction)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_message_reaction = Some(Arc::new(fnc));
    self
  }

  /// Reactions added to a message with anonymous reactions have changed; for bots only
  pub fn on_update_message_reactions<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateMessageReactions)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_message_reactions = Some(Arc::new(fnc));
    self
  }

  /// A request to send a message has reached the Telegram server. This doesn't mean that the message will be sent successfully. This update is sent only if the option "use_quick_ack" is set to true. This update may be sent multiple times for the same message
  pub fn on_update_message_send_acknowledged<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateMessageSendAcknowledged)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_message_send_acknowledged = Some(Arc::new(fnc));
    self
  }

  /// A message failed to send. Be aware that some messages being sent can be irrecoverably deleted, in which case updateDeleteMessages will be received instead of this update
  pub fn on_update_message_send_failed<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateMessageSendFailed)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_message_send_failed = Some(Arc::new(fnc));
    self
  }

  /// A message has been successfully sent
  pub fn on_update_message_send_succeeded<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateMessageSendSucceeded)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_message_send_succeeded = Some(Arc::new(fnc));
    self
  }

  /// Information about suggested post of a message was changed
  pub fn on_update_message_suggested_post_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateMessageSuggestedPostInfo)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_message_suggested_post_info = Some(Arc::new(fnc));
    self
  }

  /// The list of unread reactions added to a message was changed
  pub fn on_update_message_unread_reactions<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateMessageUnreadReactions)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_message_unread_reactions = Some(Arc::new(fnc));
    self
  }

  /// A new incoming callback query from a business message; for bots only
  pub fn on_update_new_business_callback_query<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateNewBusinessCallbackQuery)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_new_business_callback_query = Some(Arc::new(fnc));
    self
  }

  /// A new message was added to a business account; for bots only
  pub fn on_update_new_business_message<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateNewBusinessMessage)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_new_business_message = Some(Arc::new(fnc));
    self
  }

  /// New call signaling data arrived
  pub fn on_update_new_call_signaling_data<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateNewCallSignalingData)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_new_call_signaling_data = Some(Arc::new(fnc));
    self
  }

  /// A new incoming callback query; for bots only
  pub fn on_update_new_callback_query<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateNewCallbackQuery)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_new_callback_query = Some(Arc::new(fnc));
    self
  }

  /// A new chat has been loaded/created. This update is guaranteed to come before the chat identifier is returned to the application. The chat field changes will be reported through separate updates
  pub fn on_update_new_chat<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UpdateNewChat)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.update_new_chat = Some(Arc::new(fnc));
    self
  }

  /// A user sent a join request to a chat; for bots only
  pub fn on_update_new_chat_join_request<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateNewChatJoinRequest)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_new_chat_join_request = Some(Arc::new(fnc));
    self
  }

  /// The user has chosen a result of an inline query; for bots only
  pub fn on_update_new_chosen_inline_result<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateNewChosenInlineResult)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_new_chosen_inline_result = Some(Arc::new(fnc));
    self
  }

  /// A new incoming event; for bots only
  pub fn on_update_new_custom_event<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateNewCustomEvent)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_new_custom_event = Some(Arc::new(fnc));
    self
  }

  /// A new incoming query; for bots only
  pub fn on_update_new_custom_query<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateNewCustomQuery)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_new_custom_query = Some(Arc::new(fnc));
    self
  }

  /// A new message was received in a group call
  pub fn on_update_new_group_call_message<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateNewGroupCallMessage)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_new_group_call_message = Some(Arc::new(fnc));
    self
  }

  /// A new paid reaction was received in a live story group call
  pub fn on_update_new_group_call_paid_reaction<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateNewGroupCallPaidReaction)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_new_group_call_paid_reaction = Some(Arc::new(fnc));
    self
  }

  /// A new incoming guest query; for bots only
  pub fn on_update_new_guest_query<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateNewGuestQuery)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_new_guest_query = Some(Arc::new(fnc));
    self
  }

  /// A new incoming callback query from a message sent via a bot; for bots only
  pub fn on_update_new_inline_callback_query<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateNewInlineCallbackQuery)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_new_inline_callback_query = Some(Arc::new(fnc));
    self
  }

  /// A new incoming inline query; for bots only
  pub fn on_update_new_inline_query<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateNewInlineQuery)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_new_inline_query = Some(Arc::new(fnc));
    self
  }

  /// A new message was received; can also be an outgoing message
  pub fn on_update_new_message<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UpdateNewMessage)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.update_new_message = Some(Arc::new(fnc));
    self
  }

  /// An OAuth authorization request was received
  pub fn on_update_new_oauth_request<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateNewOauthRequest)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_new_oauth_request = Some(Arc::new(fnc));
    self
  }

  /// A new incoming pre-checkout query; for bots only. Contains full information about a checkout
  pub fn on_update_new_pre_checkout_query<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateNewPreCheckoutQuery)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_new_pre_checkout_query = Some(Arc::new(fnc));
    self
  }

  /// A new incoming shipping query; for bots only. Only for invoices with flexible price
  pub fn on_update_new_shipping_query<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateNewShippingQuery)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_new_shipping_query = Some(Arc::new(fnc));
    self
  }

  /// A notification was changed
  pub fn on_update_notification<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateNotification)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_notification = Some(Arc::new(fnc));
    self
  }

  /// A list of active notifications in a notification group has changed
  pub fn on_update_notification_group<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateNotificationGroup)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_notification_group = Some(Arc::new(fnc));
    self
  }

  /// An option changed its value
  pub fn on_update_option<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UpdateOption)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.update_option = Some(Arc::new(fnc));
    self
  }

  /// The number of TON Grams owned by the current user has changed
  pub fn on_update_owned_gram_count<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateOwnedGramCount)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_owned_gram_count = Some(Arc::new(fnc));
    self
  }

  /// The number of Telegram Stars owned by the current user has changed
  pub fn on_update_owned_star_count<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateOwnedStarCount)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_owned_star_count = Some(Arc::new(fnc));
    self
  }

  /// Paid media were purchased by a user; for bots only
  pub fn on_update_paid_media_purchased<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdatePaidMediaPurchased)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_paid_media_purchased = Some(Arc::new(fnc));
    self
  }

  /// A new pending text or rich message was received in a chat with a bot. The message must be shown in the chat for at most getOption("pending_text_message_period") seconds, replace any other pending message with the same draft_id with animation, and be deleted whenever any incoming message or a pending message with another draft_id is received in the message thread
  pub fn on_update_pending_message<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdatePendingMessage)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_pending_message = Some(Arc::new(fnc));
    self
  }

  /// A poll was updated; for bots only
  pub fn on_update_poll<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UpdatePoll)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.update_poll = Some(Arc::new(fnc));
    self
  }

  /// A user changed the answer to a poll; for bots only
  pub fn on_update_poll_answer<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UpdatePollAnswer)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.update_poll_answer = Some(Arc::new(fnc));
    self
  }

  /// The list of supported accent colors for user profiles has changed
  pub fn on_update_profile_accent_colors<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateProfileAccentColors)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_profile_accent_colors = Some(Arc::new(fnc));
    self
  }

  /// Basic information about a quick reply shortcut has changed. This update is guaranteed to come before the quick shortcut name is returned to the application
  pub fn on_update_quick_reply_shortcut<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateQuickReplyShortcut)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_quick_reply_shortcut = Some(Arc::new(fnc));
    self
  }

  /// A quick reply shortcut and all its messages were deleted
  pub fn on_update_quick_reply_shortcut_deleted<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateQuickReplyShortcutDeleted)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_quick_reply_shortcut_deleted = Some(Arc::new(fnc));
    self
  }

  /// The list of quick reply shortcut messages has changed
  pub fn on_update_quick_reply_shortcut_messages<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateQuickReplyShortcutMessages)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_quick_reply_shortcut_messages = Some(Arc::new(fnc));
    self
  }

  /// The list of quick reply shortcuts has changed
  pub fn on_update_quick_reply_shortcuts<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateQuickReplyShortcuts)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_quick_reply_shortcuts = Some(Arc::new(fnc));
    self
  }

  /// Notification settings for reactions were updated
  pub fn on_update_reaction_notification_settings<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateReactionNotificationSettings)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_reaction_notification_settings = Some(Arc::new(fnc));
    self
  }

  /// The list of recently used stickers was updated
  pub fn on_update_recent_stickers<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateRecentStickers)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_recent_stickers = Some(Arc::new(fnc));
    self
  }

  /// The list of saved animations was updated
  pub fn on_update_saved_animations<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateSavedAnimations)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_saved_animations = Some(Arc::new(fnc));
    self
  }

  /// Tags used in Saved Messages or a Saved Messages topic have changed
  pub fn on_update_saved_messages_tags<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateSavedMessagesTags)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_saved_messages_tags = Some(Arc::new(fnc));
    self
  }

  /// Basic information about a Saved Messages topic has changed. This update is guaranteed to come before the topic identifier is returned to the application
  pub fn on_update_saved_messages_topic<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateSavedMessagesTopic)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_saved_messages_topic = Some(Arc::new(fnc));
    self
  }

  /// Number of Saved Messages topics has changed
  pub fn on_update_saved_messages_topic_count<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateSavedMessagesTopicCount)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_saved_messages_topic_count = Some(Arc::new(fnc));
    self
  }

  /// The list of saved notification sounds was updated. This update may not be sent until information about a notification sound was requested for the first time
  pub fn on_update_saved_notification_sounds<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateSavedNotificationSounds)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_saved_notification_sounds = Some(Arc::new(fnc));
    self
  }

  /// Notification settings for some type of chats were updated
  pub fn on_update_scope_notification_settings<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateScopeNotificationSettings)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_scope_notification_settings = Some(Arc::new(fnc));
    self
  }

  /// Some data of a secret chat has changed. This update is guaranteed to come before the secret chat identifier is returned to the application
  pub fn on_update_secret_chat<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UpdateSecretChat)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.update_secret_chat = Some(Arc::new(fnc));
    self
  }

  /// A service notification from the server was received. Upon receiving this the application must show a popup with the content of the notification
  pub fn on_update_service_notification<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateServiceNotification)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_service_notification = Some(Arc::new(fnc));
    self
  }

  /// The parameters of speech recognition without Telegram Premium subscription have changed
  pub fn on_update_speech_recognition_trial<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateSpeechRecognitionTrial)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_speech_recognition_trial = Some(Arc::new(fnc));
    self
  }

  /// Download or upload file speed for the user was limited, but it can be restored by subscription to Telegram Premium. The notification can be postponed until a file being downloaded or uploaded is visible to the user. Use getOption("premium_download_speedup") or getOption("premium_upload_speedup") to get expected speedup after subscription to Telegram Premium
  pub fn on_update_speed_limit_notification<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateSpeedLimitNotification)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_speed_limit_notification = Some(Arc::new(fnc));
    self
  }

  /// The stake dice state has changed
  pub fn on_update_stake_dice_state<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateStakeDiceState)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_stake_dice_state = Some(Arc::new(fnc));
    self
  }

  /// The Telegram Star revenue earned by a user or a chat has changed. If Telegram Star transaction screen of the chat is opened, then getStarTransactions may be called to fetch new transactions
  pub fn on_update_star_revenue_status<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateStarRevenueStatus)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_star_revenue_status = Some(Arc::new(fnc));
    self
  }

  /// A sticker set has changed
  pub fn on_update_sticker_set<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UpdateStickerSet)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.update_sticker_set = Some(Arc::new(fnc));
    self
  }

  /// A message draft generation was stopped by the user
  pub fn on_update_stop_message_draft<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateStopMessageDraft)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_stop_message_draft = Some(Arc::new(fnc));
    self
  }

  /// A story was changed
  pub fn on_update_story<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UpdateStory)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.update_story = Some(Arc::new(fnc));
    self
  }

  /// A story became inaccessible
  pub fn on_update_story_deleted<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateStoryDeleted)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_story_deleted = Some(Arc::new(fnc));
    self
  }

  /// Number of chats in a story list has changed
  pub fn on_update_story_list_chat_count<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateStoryListChatCount)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_story_list_chat_count = Some(Arc::new(fnc));
    self
  }

  /// A story failed to post. If the story posting is canceled, then updateStoryDeleted will be received instead of this update
  pub fn on_update_story_post_failed<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateStoryPostFailed)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_story_post_failed = Some(Arc::new(fnc));
    self
  }

  /// A story has been successfully posted
  pub fn on_update_story_post_succeeded<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateStoryPostSucceeded)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_story_post_succeeded = Some(Arc::new(fnc));
    self
  }

  /// Story stealth mode settings have changed
  pub fn on_update_story_stealth_mode<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateStoryStealthMode)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_story_stealth_mode = Some(Arc::new(fnc));
    self
  }

  /// The list of suggested to the user actions has changed
  pub fn on_update_suggested_actions<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateSuggestedActions)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_suggested_actions = Some(Arc::new(fnc));
    self
  }

  /// Some data of a supergroup or a channel has changed. This update is guaranteed to come before the supergroup identifier is returned to the application
  pub fn on_update_supergroup<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UpdateSupergroup)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.update_supergroup = Some(Arc::new(fnc));
    self
  }

  /// Some data in supergroupFullInfo has been changed
  pub fn on_update_supergroup_full_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateSupergroupFullInfo)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_supergroup_full_info = Some(Arc::new(fnc));
    self
  }

  /// New terms of service must be accepted by the user. If the terms of service are declined, then the deleteAccount method must be called with the reason "Decline ToS update"
  pub fn on_update_terms_of_service<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateTermsOfService)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_terms_of_service = Some(Arc::new(fnc));
    self
  }

  /// The styles supported for text composition have changed
  pub fn on_update_text_composition_styles<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateTextCompositionStyles)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_text_composition_styles = Some(Arc::new(fnc));
    self
  }

  /// The information about gasless transfers has changed
  pub fn on_update_ton_wallet_gasless_transfers_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateTonWalletGaslessTransfersInfo)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_ton_wallet_gasless_transfers_info = Some(Arc::new(fnc));
    self
  }

  /// The state of the TON wallet of the current user has changed
  pub fn on_update_ton_wallet_state<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateTonWalletState)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_ton_wallet_state = Some(Arc::new(fnc));
    self
  }

  /// A session of a TON Connect has changed
  pub fn on_update_ton_wallet_ton_connect_session<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateTonWalletTonConnectSession)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_ton_wallet_ton_connect_session = Some(Arc::new(fnc));
    self
  }

  /// TON Connect sessions must be disconnected in a regular way
  pub fn on_update_ton_wallet_ton_connect_session_disconnect_required<F>(
    &mut self,
    fnc: F,
  ) -> &mut Self
  where
    F: Send
      + Sync
      + Fn(
        (Api, UpdateTonWalletTonConnectSessionDisconnectRequired),
      ) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_ton_wallet_ton_connect_session_disconnect_required = Some(Arc::new(fnc));
    self
  }

  /// Number of messages in a topic has changed; for Saved Messages and channel direct messages chat topics only
  pub fn on_update_topic_message_count<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateTopicMessageCount)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_topic_message_count = Some(Arc::new(fnc));
    self
  }

  /// The list of trending sticker sets was updated or some of them were viewed
  pub fn on_update_trending_sticker_sets<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateTrendingStickerSets)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_trending_sticker_sets = Some(Arc::new(fnc));
    self
  }

  /// Lists of bots which Mini Apps must be allowed to read text from clipboard and must be opened without a warning
  pub fn on_update_trusted_mini_app_bots<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateTrustedMiniAppBots)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_trusted_mini_app_bots = Some(Arc::new(fnc));
    self
  }

  /// The first unconfirmed session has changed
  pub fn on_update_unconfirmed_session<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateUnconfirmedSession)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_unconfirmed_session = Some(Arc::new(fnc));
    self
  }

  /// Number of unread chats, i.e. with unread messages or marked as unread, has changed. This update is sent only if the message database is used
  pub fn on_update_unread_chat_count<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateUnreadChatCount)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_unread_chat_count = Some(Arc::new(fnc));
    self
  }

  /// Number of unread messages in a chat list has changed. This update is sent only if the message database is used
  pub fn on_update_unread_message_count<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateUnreadMessageCount)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_unread_message_count = Some(Arc::new(fnc));
    self
  }

  /// Some data of a user has changed. This update is guaranteed to come before the user identifier is returned to the application
  pub fn on_update_user<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UpdateUser)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.update_user = Some(Arc::new(fnc));
    self
  }

  /// Some data in userFullInfo has been changed
  pub fn on_update_user_full_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateUserFullInfo)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_user_full_info = Some(Arc::new(fnc));
    self
  }

  /// Some privacy setting rules have been changed
  pub fn on_update_user_privacy_setting_rules<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateUserPrivacySettingRules)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_user_privacy_setting_rules = Some(Arc::new(fnc));
    self
  }

  /// The user went online or offline
  pub fn on_update_user_status<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UpdateUserStatus)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.update_user_status = Some(Arc::new(fnc));
    self
  }

  /// Subscription of a user to the bot was changed; for bots only
  pub fn on_update_user_subscription<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateUserSubscription)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_user_subscription = Some(Arc::new(fnc));
    self
  }

  /// An automatically scheduled message with video has been successfully sent after conversion
  pub fn on_update_video_published<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateVideoPublished)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_video_published = Some(Arc::new(fnc));
    self
  }

  /// A message was sent by an opened Web App, so the Web App needs to be closed
  pub fn on_update_web_app_message_sent<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateWebAppMessageSent)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_web_app_message_sent = Some(Arc::new(fnc));
    self
  }

  /// Web browser settings have been updated
  pub fn on_update_web_browser_settings<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateWebBrowserSettings)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_web_browser_settings = Some(Arc::new(fnc));
    self
  }

  /// A web proxy carrier must be closed. If the page has passed any message to the bridge object, then the application must deliver the string {"t":"close"} to the page, so that it can end its relay session, and destroy the web view shortly after, for example, in 200 milliseconds; otherwise, the web view must be destroyed immediately. Messages from the carrier received after this update must be ignored
  pub fn on_update_web_proxy_carrier_closed<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateWebProxyCarrierClosed)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_web_proxy_carrier_closed = Some(Arc::new(fnc));
    self
  }

  /// Data must be delivered as an ArrayBuffer to the page of a web proxy carrier through the onmessage function of the bridge object, in the order of the updates
  pub fn on_update_web_proxy_carrier_data<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateWebProxyCarrierData)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_web_proxy_carrier_data = Some(Arc::new(fnc));
    self
  }

  /// A web proxy carrier must be opened as described in webProxyCarrier. Messages passed by the page to the bridge object must be passed to processWebProxyCarrierMessage. Failures of the carrier must be reported with reportWebProxyCarrierError
  pub fn on_update_web_proxy_carrier_opened<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpdateWebProxyCarrierOpened)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.update_web_proxy_carrier_opened = Some(Arc::new(fnc));
    self
  }

  /// Represents the current authorization state of the TDLib client
  pub fn on_authorization_state<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, AuthorizationState)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.authorization_state = Some(Arc::new(fnc));
    self
  }

  /// Represents result of checking whether the current user can post a story on behalf of the specific chat
  pub fn on_can_post_story_result<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, CanPostStoryResult)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.can_post_story_result = Some(Arc::new(fnc));
    self
  }

  /// Describes whether a gift can be sent now by the current user
  pub fn on_can_send_gift_result<F>(&mut self, fnc: F) -> &mut Self
  where
    F:
      Send + Sync + Fn((Api, CanSendGiftResult)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.can_send_gift_result = Some(Arc::new(fnc));
    self
  }

  /// Describes result of canSendMessageToUser
  pub fn on_can_send_message_to_user_result<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, CanSendMessageToUserResult)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.can_send_message_to_user_result = Some(Arc::new(fnc));
    self
  }

  /// Represents result of checking whether the current session can be used to transfer a chat ownership to another user
  pub fn on_can_transfer_ownership_result<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, CanTransferOwnershipResult)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.can_transfer_ownership_result = Some(Arc::new(fnc));
    self
  }

  /// Describes result of join of a chat by the current user
  pub fn on_chat_join_result<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, ChatJoinResult)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.chat_join_result = Some(Arc::new(fnc));
    self
  }

  /// Contains a detailed statistics about a chat
  pub fn on_chat_statistics<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, ChatStatistics)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.chat_statistics = Some(Arc::new(fnc));
    self
  }

  /// Represents result of checking whether a username can be set for a chat
  pub fn on_check_chat_username_result<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, CheckChatUsernameResult)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.check_chat_username_result = Some(Arc::new(fnc));
    self
  }

  /// Represents result of checking whether a name can be used for a new sticker set
  pub fn on_check_sticker_set_name_result<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, CheckStickerSetNameResult)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.check_sticker_set_name_result = Some(Arc::new(fnc));
    self
  }

  /// Contains result of gift crafting
  pub fn on_craft_gift_result<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, CraftGiftResult)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.craft_gift_result = Some(Arc::new(fnc));
    self
  }

  /// Describes result of sending a resold gift
  pub fn on_gift_resale_result<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, GiftResaleResult)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.gift_resale_result = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a giveaway
  pub fn on_giveaway_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, GiveawayInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.giveaway_info = Some(Arc::new(fnc));
    self
  }

  /// Describes an internal https://t.me or tg: link, which must be processed by the application in a special way
  pub fn on_internal_link_type<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, InternalLinkType)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.internal_link_type = Some(Arc::new(fnc));
    self
  }

  /// Describes result of group call participant invitation
  pub fn on_invite_group_call_participant_result<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, InviteGroupCallParticipantResult)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.invite_group_call_participant_result = Some(Arc::new(fnc));
    self
  }

  /// Represents a JSON value
  pub fn on_json_value<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, JsonValue)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.json_value = Some(Arc::new(fnc));
    self
  }

  /// Represents the value of a string in a language pack
  pub fn on_language_pack_string_value<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, LanguagePackStringValue)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.language_pack_string_value = Some(Arc::new(fnc));
    self
  }

  /// Describes a stream to which TDLib internal log is written
  pub fn on_log_stream<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, LogStream)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.log_stream = Some(Arc::new(fnc));
    self
  }

  /// Contains information about an inline button of type inlineKeyboardButtonTypeLoginUrl or an external link
  pub fn on_login_url_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, LoginUrlInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.login_url_info = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a file with messages exported from another app
  pub fn on_message_file_type<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, MessageFileType)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.message_file_type = Some(Arc::new(fnc));
    self
  }

  /// Describes read date of a recent outgoing message in a private chat
  pub fn on_message_read_date<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, MessageReadDate)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.message_read_date = Some(Arc::new(fnc));
    self
  }

  /// Contains information about the sender of a message
  pub fn on_message_sender<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, MessageSender)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.message_sender = Some(Arc::new(fnc));
    self
  }

  /// Represents the value of an option
  pub fn on_option_value<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, OptionValue)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.option_value = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a Telegram Passport element
  pub fn on_passport_element<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, PassportElement)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.passport_element = Some(Arc::new(fnc));
    self
  }

  /// Describes result of chat report
  pub fn on_report_chat_result<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, ReportChatResult)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.report_chat_result = Some(Arc::new(fnc));
    self
  }

  /// Describes result of sponsored message or chat report
  pub fn on_report_sponsored_result<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, ReportSponsoredResult)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.report_sponsored_result = Some(Arc::new(fnc));
    self
  }

  /// Describes result of story report
  pub fn on_report_story_result<F>(&mut self, fnc: F) -> &mut Self
  where
    F:
      Send + Sync + Fn((Api, ReportStoryResult)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.report_story_result = Some(Arc::new(fnc));
    self
  }

  /// Represents result of 2-step verification password reset
  pub fn on_reset_password_result<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, ResetPasswordResult)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.reset_password_result = Some(Arc::new(fnc));
    self
  }

  /// Represents result of starting a live story
  pub fn on_start_live_story_result<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, StartLiveStoryResult)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.start_live_story_result = Some(Arc::new(fnc));
    self
  }

  /// Describes a statistical graph
  pub fn on_statistical_graph<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, StatisticalGraph)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.statistical_graph = Some(Arc::new(fnc));
    self
  }

  /// Contains notifications about data changes
  pub fn on_update<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Update)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.update = Some(Arc::new(fnc));
    self
  }

  /// Describes the type of web browser
  pub fn on_web_browser_type<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, WebBrowserType)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.web_browser_type = Some(Arc::new(fnc));
    self
  }

  /// Contains information about the period of inactivity after which the current user's account will automatically be deleted
  pub fn on_account_ttl<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, AccountTtl)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.account_ttl = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of added proxy servers
  pub fn on_added_proxies<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, AddedProxies)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.added_proxies = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a proxy server added to the list of proxies
  pub fn on_added_proxy<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, AddedProxy)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.added_proxy = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of reactions added to a message
  pub fn on_added_reactions<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, AddedReactions)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.added_reactions = Some(Arc::new(fnc));
    self
  }

  /// Describes an animated or custom representation of an emoji
  pub fn on_animated_emoji<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, AnimatedEmoji)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.animated_emoji = Some(Arc::new(fnc));
    self
  }

  /// Describes an animation file. The animation must be encoded in GIF or MPEG4 format
  pub fn on_animation<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Animation)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.animation = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of animations
  pub fn on_animations<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Animations)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.animations = Some(Arc::new(fnc));
    self
  }

  /// Contains settings for automatic moving of chats to and from the Archive chat lists
  pub fn on_archive_chat_list_settings<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, ArchiveChatListSettings)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.archive_chat_list_settings = Some(Arc::new(fnc));
    self
  }

  /// Represents a bot, which can be added to attachment or side menu
  pub fn on_attachment_menu_bot<F>(&mut self, fnc: F) -> &mut Self
  where
    F:
      Send + Sync + Fn((Api, AttachmentMenuBot)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.attachment_menu_bot = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of audio files
  pub fn on_audios<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Audios)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.audios = Some(Arc::new(fnc));
    self
  }

  /// Information about the authentication code that was sent
  pub fn on_authentication_code_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, AuthenticationCodeInfo)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.authentication_code_info = Some(Arc::new(fnc));
    self
  }

  /// Contains auto-download settings presets for the current user
  pub fn on_auto_download_settings_presets<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, AutoDownloadSettingsPresets)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.auto_download_settings_presets = Some(Arc::new(fnc));
    self
  }

  /// Describes autosave settings
  pub fn on_autosave_settings<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, AutosaveSettings)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.autosave_settings = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of gifts that can be sent to another user or channel chat
  pub fn on_available_gifts<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, AvailableGifts)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.available_gifts = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of reactions that can be added to a message
  pub fn on_available_reactions<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, AvailableReactions)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.available_reactions = Some(Arc::new(fnc));
    self
  }

  /// Describes a chat background
  pub fn on_background<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Background)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.background = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of backgrounds
  pub fn on_backgrounds<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Backgrounds)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.backgrounds = Some(Arc::new(fnc));
    self
  }

  /// Information about a bank card
  pub fn on_bank_card_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, BankCardInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.bank_card_info = Some(Arc::new(fnc));
    self
  }

  /// Represents a basic group of 0-200 users (must be upgraded to a supergroup to accommodate more than 200 users)
  pub fn on_basic_group<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, BasicGroup)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.basic_group = Some(Arc::new(fnc));
    self
  }

  /// Contains full information about a basic group
  pub fn on_basic_group_full_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, BasicGroupFullInfo)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.basic_group_full_info = Some(Arc::new(fnc));
    self
  }

  /// Describes users that have access to a bot
  pub fn on_bot_access_settings<F>(&mut self, fnc: F) -> &mut Self
  where
    F:
      Send + Sync + Fn((Api, BotAccessSettings)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.bot_access_settings = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of bot commands
  pub fn on_bot_commands<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, BotCommands)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.bot_commands = Some(Arc::new(fnc));
    self
  }

  /// Describes media previews of a bot
  pub fn on_bot_media_preview<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, BotMediaPreview)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.bot_media_preview = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of media previews of a bot for the given language and the list of languages for which the bot has dedicated previews
  pub fn on_bot_media_preview_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, BotMediaPreviewInfo)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.bot_media_preview_info = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of media previews of a bot
  pub fn on_bot_media_previews<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, BotMediaPreviews)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.bot_media_previews = Some(Arc::new(fnc));
    self
  }

  /// Describes a button to be shown instead of bot commands menu button
  pub fn on_bot_menu_button<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, BotMenuButton)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.bot_menu_button = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a business chat link
  pub fn on_business_chat_link<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, BusinessChatLink)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.business_chat_link = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a business chat link
  pub fn on_business_chat_link_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, BusinessChatLinkInfo)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.business_chat_link_info = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of business chat links created by the user
  pub fn on_business_chat_links<F>(&mut self, fnc: F) -> &mut Self
  where
    F:
      Send + Sync + Fn((Api, BusinessChatLinks)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.business_chat_links = Some(Arc::new(fnc));
    self
  }

  /// Describes a connection of a bot to an account
  pub fn on_business_connected_bot_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, BusinessConnectedBotInfo)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.business_connected_bot_info = Some(Arc::new(fnc));
    self
  }

  /// Describes a connection of the bot with a business account
  pub fn on_business_connection<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, BusinessConnection)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.business_connection = Some(Arc::new(fnc));
    self
  }

  /// Contains information about features, available to Business user accounts
  pub fn on_business_features<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, BusinessFeatures)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.business_features = Some(Arc::new(fnc));
    self
  }

  /// Describes a message from a business account as received by a bot
  pub fn on_business_message<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, BusinessMessage)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.business_message = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of messages from a business account as received by a bot
  pub fn on_business_messages<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, BusinessMessages)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.business_messages = Some(Arc::new(fnc));
    self
  }

  /// Contains the call identifier
  pub fn on_call_id<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, CallId)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.call_id = Some(Arc::new(fnc));
    self
  }

  /// Contains a bot's answer to a callback query
  pub fn on_callback_query_answer<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, CallbackQueryAnswer)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.callback_query_answer = Some(Arc::new(fnc));
    self
  }

  /// A chat. (Can be a private chat, basic group, supergroup, or secret chat)
  pub fn on_chat<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Chat)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.chat = Some(Arc::new(fnc));
    self
  }

  /// Describes active stories posted by a chat
  pub fn on_chat_active_stories<F>(&mut self, fnc: F) -> &mut Self
  where
    F:
      Send + Sync + Fn((Api, ChatActiveStories)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.chat_active_stories = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of chat administrators
  pub fn on_chat_administrators<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, ChatAdministrators)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.chat_administrators = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of features available on the first chat boost levels
  pub fn on_chat_boost_features<F>(&mut self, fnc: F) -> &mut Self
  where
    F:
      Send + Sync + Fn((Api, ChatBoostFeatures)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.chat_boost_features = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of features available on a specific chat boost level
  pub fn on_chat_boost_level_features<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, ChatBoostLevelFeatures)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.chat_boost_level_features = Some(Arc::new(fnc));
    self
  }

  /// Contains an HTTPS link to boost a chat
  pub fn on_chat_boost_link<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, ChatBoostLink)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.chat_boost_link = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a link to boost a chat
  pub fn on_chat_boost_link_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F:
      Send + Sync + Fn((Api, ChatBoostLinkInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.chat_boost_link_info = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of chat boost slots
  pub fn on_chat_boost_slots<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, ChatBoostSlots)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.chat_boost_slots = Some(Arc::new(fnc));
    self
  }

  /// Describes current boost status of a chat
  pub fn on_chat_boost_status<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, ChatBoostStatus)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.chat_boost_status = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of chat events
  pub fn on_chat_events<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, ChatEvents)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.chat_events = Some(Arc::new(fnc));
    self
  }

  /// Represents a folder for user chats
  pub fn on_chat_folder<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, ChatFolder)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.chat_folder = Some(Arc::new(fnc));
    self
  }

  /// Represents an icon for a chat folder
  pub fn on_chat_folder_icon<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, ChatFolderIcon)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.chat_folder_icon = Some(Arc::new(fnc));
    self
  }

  /// Contains basic information about a chat folder
  pub fn on_chat_folder_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, ChatFolderInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.chat_folder_info = Some(Arc::new(fnc));
    self
  }

  /// Contains a chat folder invite link
  pub fn on_chat_folder_invite_link<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, ChatFolderInviteLink)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.chat_folder_invite_link = Some(Arc::new(fnc));
    self
  }

  /// Contains information about an invite link to a chat folder
  pub fn on_chat_folder_invite_link_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, ChatFolderInviteLinkInfo)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.chat_folder_invite_link_info = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of chat folder invite links
  pub fn on_chat_folder_invite_links<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, ChatFolderInviteLinks)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.chat_folder_invite_links = Some(Arc::new(fnc));
    self
  }

  /// Contains a chat invite link
  pub fn on_chat_invite_link<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, ChatInviteLink)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.chat_invite_link = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of chat invite link counts
  pub fn on_chat_invite_link_counts<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, ChatInviteLinkCounts)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.chat_invite_link_counts = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a chat invite link
  pub fn on_chat_invite_link_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, ChatInviteLinkInfo)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.chat_invite_link_info = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of chat members joined a chat via an invite link
  pub fn on_chat_invite_link_members<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, ChatInviteLinkMembers)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.chat_invite_link_members = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of chat invite links
  pub fn on_chat_invite_links<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, ChatInviteLinks)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.chat_invite_links = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of requests to join a chat
  pub fn on_chat_join_requests<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, ChatJoinRequests)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.chat_join_requests = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of chat lists
  pub fn on_chat_lists<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, ChatLists)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.chat_lists = Some(Arc::new(fnc));
    self
  }

  /// Describes a user or a chat as a member of another chat
  pub fn on_chat_member<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, ChatMember)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.chat_member = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of chat members
  pub fn on_chat_members<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, ChatMembers)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.chat_members = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of message senders, which can be used to send messages in a chat
  pub fn on_chat_message_senders<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, ChatMessageSenders)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.chat_message_senders = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of chat or user profile photos
  pub fn on_chat_photos<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, ChatPhotos)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.chat_photos = Some(Arc::new(fnc));
    self
  }

  /// A detailed statistics about revenue earned from sponsored messages in a chat
  pub fn on_chat_revenue_statistics<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, ChatRevenueStatistics)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.chat_revenue_statistics = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of chat revenue transactions
  pub fn on_chat_revenue_transactions<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, ChatRevenueTransactions)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.chat_revenue_transactions = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of chats
  pub fn on_chats<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Chats)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.chats = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a collectible item and its last purchase
  pub fn on_collectible_item_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, CollectibleItemInfo)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.collectible_item_info = Some(Arc::new(fnc));
    self
  }

  /// Contains full information about a community
  pub fn on_community_full_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F:
      Send + Sync + Fn((Api, CommunityFullInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.community_full_info = Some(Arc::new(fnc));
    self
  }

  /// Contains identifier of a community
  pub fn on_community_id<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, CommunityId)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.community_id = Some(Arc::new(fnc));
    self
  }

  /// Describes an affiliate program that was connected to an affiliate
  pub fn on_connected_affiliate_program<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, ConnectedAffiliateProgram)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.connected_affiliate_program = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of affiliate programs that were connected to an affiliate
  pub fn on_connected_affiliate_programs<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, ConnectedAffiliatePrograms)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.connected_affiliate_programs = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of websites the current user is logged in with Telegram
  pub fn on_connected_websites<F>(&mut self, fnc: F) -> &mut Self
  where
    F:
      Send + Sync + Fn((Api, ConnectedWebsites)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.connected_websites = Some(Arc::new(fnc));
    self
  }

  /// Contains a counter
  pub fn on_count<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Count)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.count = Some(Arc::new(fnc));
    self
  }

  /// Contains information about countries
  pub fn on_countries<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Countries)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.countries = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a country
  pub fn on_country_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, CountryInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.country_info = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a newly created basic group chat
  pub fn on_created_basic_group_chat<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, CreatedBasicGroupChat)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.created_basic_group_chat = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of currencies
  pub fn on_currencies<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Currencies)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.currencies = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of currency exchange rates
  pub fn on_currency_exchange_rates<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, CurrencyExchangeRates)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.currency_exchange_rates = Some(Arc::new(fnc));
    self
  }

  /// Describes the current weather
  pub fn on_current_weather<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, CurrentWeather)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.current_weather = Some(Arc::new(fnc));
    self
  }

  /// Contains the result of a custom request
  pub fn on_custom_request_result<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, CustomRequestResult)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.custom_request_result = Some(Arc::new(fnc));
    self
  }

  /// Contains some binary data
  pub fn on_data<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Data)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.data = Some(Arc::new(fnc));
    self
  }

  /// Contains database statistics
  pub fn on_database_statistics<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, DatabaseStatistics)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.database_statistics = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a tg: deep link
  pub fn on_deep_link_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, DeepLinkInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.deep_link_info = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a topic in a channel direct messages chat administered by the current user
  pub fn on_direct_messages_chat_topic<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, DirectMessagesChatTopic)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.direct_messages_chat_topic = Some(Arc::new(fnc));
    self
  }

  /// Information about the email address authentication code that was sent
  pub fn on_email_address_authentication_code_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, EmailAddressAuthenticationCodeInfo)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.email_address_authentication_code_info = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of emoji categories
  pub fn on_emoji_categories<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, EmojiCategories)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.emoji_categories = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of emojis with their keywords
  pub fn on_emoji_keywords<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, EmojiKeywords)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.emoji_keywords = Some(Arc::new(fnc));
    self
  }

  /// Contains information about an emoji reaction
  pub fn on_emoji_reaction<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, EmojiReaction)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.emoji_reaction = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of custom emoji identifiers for emoji statuses
  pub fn on_emoji_status_custom_emojis<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, EmojiStatusCustomEmojis)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.emoji_status_custom_emojis = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of emoji statuses
  pub fn on_emoji_statuses<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, EmojiStatuses)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.emoji_statuses = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of emojis
  pub fn on_emojis<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Emojis)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.emojis = Some(Arc::new(fnc));
    self
  }

  /// An object of this type can be returned on every function call, in case of an error
  pub fn on_error<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Error)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.error = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of users that has failed to be added to a chat
  pub fn on_failed_to_add_members<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, FailedToAddMembers)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.failed_to_add_members = Some(Arc::new(fnc));
    self
  }

  /// Represents a file
  pub fn on_file<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, File)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.file = Some(Arc::new(fnc));
    self
  }

  /// Contains size of downloaded prefix of a file
  pub fn on_file_downloaded_prefix_size<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, FileDownloadedPrefixSize)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.file_downloaded_prefix_size = Some(Arc::new(fnc));
    self
  }

  /// A text fixed using fixTextWithAi
  pub fn on_fixed_text<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, FixedText)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.fixed_text = Some(Arc::new(fnc));
    self
  }

  /// A text with some entities
  pub fn on_formatted_text<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, FormattedText)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.formatted_text = Some(Arc::new(fnc));
    self
  }

  /// Describes a forum topic
  pub fn on_forum_topic<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, ForumTopic)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.forum_topic = Some(Arc::new(fnc));
    self
  }

  /// Contains basic information about a forum topic
  pub fn on_forum_topic_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, ForumTopicInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.forum_topic_info = Some(Arc::new(fnc));
    self
  }

  /// Describes a list of forum topics
  pub fn on_forum_topics<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, ForumTopics)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.forum_topics = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of found affiliate programs
  pub fn on_found_affiliate_programs<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, FoundAffiliatePrograms)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.found_affiliate_programs = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of boosts applied to a chat
  pub fn on_found_chat_boosts<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, FoundChatBoosts)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.found_chat_boosts = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of messages found by a search in a given chat
  pub fn on_found_chat_messages<F>(&mut self, fnc: F) -> &mut Self
  where
    F:
      Send + Sync + Fn((Api, FoundChatMessages)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.found_chat_messages = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of downloaded files, found by a search
  pub fn on_found_file_downloads<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, FoundFileDownloads)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.found_file_downloads = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of messages found by a search
  pub fn on_found_messages<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, FoundMessages)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.found_messages = Some(Arc::new(fnc));
    self
  }

  /// Contains 0-based match position
  pub fn on_found_position<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, FoundPosition)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.found_position = Some(Arc::new(fnc));
    self
  }

  /// Contains 0-based positions of matched objects
  pub fn on_found_positions<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, FoundPositions)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.found_positions = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of messages found by a public post search
  pub fn on_found_public_posts<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, FoundPublicPosts)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.found_public_posts = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of stories found by a search
  pub fn on_found_stories<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, FoundStories)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.found_stories = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of found users
  pub fn on_found_users<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, FoundUsers)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.found_users = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a Web App found by its short name
  pub fn on_found_web_app<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, FoundWebApp)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.found_web_app = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of game high scores
  pub fn on_game_high_scores<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, GameHighScores)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.game_high_scores = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of gifts that were acquired by the current user on an auction
  pub fn on_gift_auction_acquired_gifts<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, GiftAuctionAcquiredGifts)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.gift_auction_acquired_gifts = Some(Arc::new(fnc));
    self
  }

  /// Represents auction state of a gift
  pub fn on_gift_auction_state<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, GiftAuctionState)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.gift_auction_state = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of chat themes based on upgraded gifts
  pub fn on_gift_chat_themes<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, GiftChatThemes)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.gift_chat_themes = Some(Arc::new(fnc));
    self
  }

  /// Describes collection of gifts
  pub fn on_gift_collection<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, GiftCollection)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.gift_collection = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of gift collections
  pub fn on_gift_collections<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, GiftCollections)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.gift_collections = Some(Arc::new(fnc));
    self
  }

  /// Contains examples of possible upgraded gifts for the given regular gift
  pub fn on_gift_upgrade_preview<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, GiftUpgradePreview)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.gift_upgrade_preview = Some(Arc::new(fnc));
    self
  }

  /// Contains all possible variants of upgraded gifts for the given regular gift
  pub fn on_gift_upgrade_variants<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, GiftUpgradeVariants)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.gift_upgrade_variants = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of gifts received by a user or a chat
  pub fn on_gifts_for_crafting<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, GiftsForCrafting)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.gifts_for_crafting = Some(Arc::new(fnc));
    self
  }

  /// Describes gifts available for resale
  pub fn on_gifts_for_resale<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, GiftsForResale)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.gifts_for_resale = Some(Arc::new(fnc));
    self
  }

  /// A detailed statistics about TON Grams earned by the current user
  pub fn on_gram_revenue_statistics<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, GramRevenueStatistics)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.gram_revenue_statistics = Some(Arc::new(fnc));
    self
  }

  /// Describes a group call
  pub fn on_group_call<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, GroupCall)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.group_call = Some(Arc::new(fnc));
    self
  }

  /// Contains the group call identifier
  pub fn on_group_call_id<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, GroupCallId)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.group_call_id = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a just created or just joined group call
  pub fn on_group_call_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, GroupCallInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.group_call_info = Some(Arc::new(fnc));
    self
  }

  /// Represents a group call participant
  pub fn on_group_call_participant<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, GroupCallParticipant)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.group_call_participant = Some(Arc::new(fnc));
    self
  }

  /// Contains identifiers of group call participants
  pub fn on_group_call_participants<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, GroupCallParticipants)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.group_call_participants = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of group call streams
  pub fn on_group_call_streams<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, GroupCallStreams)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.group_call_streams = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of hashtags
  pub fn on_hashtags<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Hashtags)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.hashtags = Some(Arc::new(fnc));
    self
  }

  /// Contains an HTTP URL
  pub fn on_http_url<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, HttpUrl)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.http_url = Some(Arc::new(fnc));
    self
  }

  /// Represents the result of an importContacts request
  pub fn on_imported_contacts<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, ImportedContacts)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.imported_contacts = Some(Arc::new(fnc));
    self
  }

  /// Contains identifier of a sent guest message
  pub fn on_inline_message_id<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, InlineMessageId)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.inline_message_id = Some(Arc::new(fnc));
    self
  }

  /// Represents the results of the inline query. Use sendInlineQueryResultMessage to send the result of the query
  pub fn on_inline_query_results<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, InlineQueryResults)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.inline_query_results = Some(Arc::new(fnc));
    self
  }

  /// Represents a single button in a bot keyboard
  pub fn on_keyboard_button<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, KeyboardButton)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.keyboard_button = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a language pack
  pub fn on_language_pack_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, LanguagePackInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.language_pack_info = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of language pack strings
  pub fn on_language_pack_strings<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, LanguagePackStrings)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.language_pack_strings = Some(Arc::new(fnc));
    self
  }

  /// Describes a link preview
  pub fn on_link_preview<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, LinkPreview)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.link_preview = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of users and chats that spend most money on paid messages and reactions in a live story
  pub fn on_live_story_donors<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, LiveStoryDonors)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.live_story_donors = Some(Arc::new(fnc));
    self
  }

  /// Contains information about the current localization target
  pub fn on_localization_target_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, LocalizationTargetInfo)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.localization_target_info = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of available TDLib internal log tags
  pub fn on_log_tags<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, LogTags)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.log_tags = Some(Arc::new(fnc));
    self
  }

  /// Contains a TDLib internal log verbosity level
  pub fn on_log_verbosity_level<F>(&mut self, fnc: F) -> &mut Self
  where
    F:
      Send + Sync + Fn((Api, LogVerbosityLevel)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.log_verbosity_level = Some(Arc::new(fnc));
    self
  }

  /// Contains information about the main Web App of a bot
  pub fn on_main_web_app<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, MainWebApp)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.main_web_app = Some(Arc::new(fnc));
    self
  }

  /// Describes a message
  pub fn on_message<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Message)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.message = Some(Arc::new(fnc));
    self
  }

  /// Contains default auto-delete timer setting for new chats
  pub fn on_message_auto_delete_time<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, MessageAutoDeleteTime)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.message_auto_delete_time = Some(Arc::new(fnc));
    self
  }

  /// Contains information about found messages, split by days according to the option "utc_time_offset"
  pub fn on_message_calendar<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, MessageCalendar)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.message_calendar = Some(Arc::new(fnc));
    self
  }

  /// Contains information about an effect added to a message
  pub fn on_message_effect<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, MessageEffect)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.message_effect = Some(Arc::new(fnc));
    self
  }

  /// Contains an HTTPS link to a message in a supergroup or channel, or a forum topic
  pub fn on_message_link<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, MessageLink)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.message_link = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a link to a message or a forum topic in a chat
  pub fn on_message_link_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, MessageLinkInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.message_link_info = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of message positions
  pub fn on_message_positions<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, MessagePositions)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.message_positions = Some(Arc::new(fnc));
    self
  }

  /// Contains properties of a message and describes actions that can be done with the message right now
  pub fn on_message_properties<F>(&mut self, fnc: F) -> &mut Self
  where
    F:
      Send + Sync + Fn((Api, MessageProperties)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.message_properties = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of message senders
  pub fn on_message_senders<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, MessageSenders)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.message_senders = Some(Arc::new(fnc));
    self
  }

  /// A detailed statistics about a message
  pub fn on_message_statistics<F>(&mut self, fnc: F) -> &mut Self
  where
    F:
      Send + Sync + Fn((Api, MessageStatistics)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.message_statistics = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a message thread
  pub fn on_message_thread_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F:
      Send + Sync + Fn((Api, MessageThreadInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.message_thread_info = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of message viewers
  pub fn on_message_viewers<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, MessageViewers)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.message_viewers = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of messages
  pub fn on_messages<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Messages)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.messages = Some(Arc::new(fnc));
    self
  }

  /// A full list of available network statistic entries
  pub fn on_network_statistics<F>(&mut self, fnc: F) -> &mut Self
  where
    F:
      Send + Sync + Fn((Api, NetworkStatistics)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.network_statistics = Some(Arc::new(fnc));
    self
  }

  /// Contains privacy settings for chats with non-contacts
  pub fn on_new_chat_privacy_settings<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, NewChatPrivacySettings)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.new_chat_privacy_settings = Some(Arc::new(fnc));
    self
  }

  /// Describes a notification sound in MP3 format
  pub fn on_notification_sound<F>(&mut self, fnc: F) -> &mut Self
  where
    F:
      Send + Sync + Fn((Api, NotificationSound)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.notification_sound = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of notification sounds
  pub fn on_notification_sounds<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, NotificationSounds)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.notification_sounds = Some(Arc::new(fnc));
    self
  }

  /// Information about the OAuth authorization
  pub fn on_oauth_link_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, OauthLinkInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.oauth_link_info = Some(Arc::new(fnc));
    self
  }

  /// An object of this type is returned on a successful function call for certain functions
  pub fn on_ok<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Ok)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.ok = Some(Arc::new(fnc));
    self
  }

  /// Describes availability of cryptocurrency purchase through an on-ramp provider
  pub fn on_on_ramp_payment_availability<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, OnRampPaymentAvailability)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.on_ramp_payment_availability = Some(Arc::new(fnc));
    self
  }

  /// Describes limits for an on-ramp purchase
  pub fn on_on_ramp_payment_limits<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, OnRampPaymentLimits)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.on_ramp_payment_limits = Some(Arc::new(fnc));
    self
  }

  /// Describes a quote for an on-ramp purchase
  pub fn on_on_ramp_payment_quote<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, OnRampPaymentQuote)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.on_ramp_payment_quote = Some(Arc::new(fnc));
    self
  }

  /// Describes a session for an on-ramp purchase
  pub fn on_on_ramp_payment_session<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, OnRampPaymentSession)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.on_ramp_payment_session = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of on-ramp providers
  pub fn on_on_ramp_providers<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, OnRampProviders)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.on_ramp_providers = Some(Arc::new(fnc));
    self
  }

  /// Order information
  pub fn on_order_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, OrderInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.order_info = Some(Arc::new(fnc));
    self
  }

  /// Represents outline of an image
  pub fn on_outline<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Outline)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.outline = Some(Arc::new(fnc));
    self
  }

  /// Describes a passkey
  pub fn on_passkey<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Passkey)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.passkey = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of passkeys
  pub fn on_passkeys<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Passkeys)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.passkeys = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a Telegram Passport authorization form that was requested
  pub fn on_passport_authorization_form<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, PassportAuthorizationForm)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.passport_authorization_form = Some(Arc::new(fnc));
    self
  }

  /// Contains information about saved Telegram Passport elements
  pub fn on_passport_elements<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, PassportElements)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.passport_elements = Some(Arc::new(fnc));
    self
  }

  /// Contains information about Telegram Passport elements and corresponding errors
  pub fn on_passport_elements_with_errors<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, PassportElementsWithErrors)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.passport_elements_with_errors = Some(Arc::new(fnc));
    self
  }

  /// Represents the current state of 2-step verification
  pub fn on_password_state<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, PasswordState)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.password_state = Some(Arc::new(fnc));
    self
  }

  /// Contains information about an invoice payment form
  pub fn on_payment_form<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, PaymentForm)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.payment_form = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a successful payment
  pub fn on_payment_receipt<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, PaymentReceipt)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.payment_receipt = Some(Arc::new(fnc));
    self
  }

  /// Contains the result of a payment request
  pub fn on_payment_result<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, PaymentResult)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.payment_result = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a phone number
  pub fn on_phone_number_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, PhoneNumberInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.phone_number_info = Some(Arc::new(fnc));
    self
  }

  /// Contains properties of a poll option and describes actions that can be done with the option right now
  pub fn on_poll_option_properties<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, PollOptionProperties)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.poll_option_properties = Some(Arc::new(fnc));
    self
  }

  /// A detailed statistics about poll votes
  pub fn on_poll_vote_statistics<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, PollVoteStatistics)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.poll_vote_statistics = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of poll voters
  pub fn on_poll_voters<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, PollVoters)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.poll_voters = Some(Arc::new(fnc));
    self
  }

  /// Contains information about features, available to Premium users
  pub fn on_premium_features<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, PremiumFeatures)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.premium_features = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a Telegram Premium gift code
  pub fn on_premium_gift_code_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, PremiumGiftCodeInfo)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.premium_gift_code_info = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of options for gifting Telegram Premium to a user
  pub fn on_premium_gift_payment_options<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, PremiumGiftPaymentOptions)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.premium_gift_payment_options = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of options for creating of Telegram Premium giveaway or manual distribution of Telegram Premium among chat members
  pub fn on_premium_giveaway_payment_options<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, PremiumGiveawayPaymentOptions)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.premium_giveaway_payment_options = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a limit, increased for Premium users
  pub fn on_premium_limit<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, PremiumLimit)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.premium_limit = Some(Arc::new(fnc));
    self
  }

  /// Contains state of Telegram Premium subscription and promotion videos for Premium features
  pub fn on_premium_state<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, PremiumState)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.premium_state = Some(Arc::new(fnc));
    self
  }

  /// Represents a ready to send inline message. Use sendInlineQueryResultMessage to send the message
  pub fn on_prepared_inline_message<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, PreparedInlineMessage)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.prepared_inline_message = Some(Arc::new(fnc));
    self
  }

  /// Represents an inline message that can be sent via the bot
  pub fn on_prepared_inline_message_id<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, PreparedInlineMessageId)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.prepared_inline_message_id = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of backdrops, which can be used as a background of a theme of the screen for sharing a link to a user or a chat
  pub fn on_profile_share_theme_backdrops<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, ProfileShareThemeBackdrops)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.profile_share_theme_backdrops = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of public forwards and reposts as a story of a message or a story
  pub fn on_public_forwards<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, PublicForwards)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.public_forwards = Some(Arc::new(fnc));
    self
  }

  /// Contains information about public post search limits
  pub fn on_public_post_search_limits<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, PublicPostSearchLimits)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.public_post_search_limits = Some(Arc::new(fnc));
    self
  }

  /// Contains a globally unique push receiver identifier, which can be used to identify which account has received a push notification
  pub fn on_push_receiver_id<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, PushReceiverId)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.push_receiver_id = Some(Arc::new(fnc));
    self
  }

  /// Describes a message that can be used for quick reply
  pub fn on_quick_reply_message<F>(&mut self, fnc: F) -> &mut Self
  where
    F:
      Send + Sync + Fn((Api, QuickReplyMessage)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.quick_reply_message = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of quick reply messages
  pub fn on_quick_reply_messages<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, QuickReplyMessages)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.quick_reply_messages = Some(Arc::new(fnc));
    self
  }

  /// Contains privacy settings for message read date in private chats. Read dates are always shown to the users that can see online status of the current user regardless of this setting
  pub fn on_read_date_privacy_settings<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, ReadDatePrivacySettings)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.read_date_privacy_settings = Some(Arc::new(fnc));
    self
  }

  /// Represents a gift received by a user or a chat
  pub fn on_received_gift<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, ReceivedGift)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.received_gift = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of gifts received by a user or a chat
  pub fn on_received_gifts<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, ReceivedGifts)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.received_gifts = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of recommended chat folders
  pub fn on_recommended_chat_folders<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, RecommendedChatFolders)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.recommended_chat_folders = Some(Arc::new(fnc));
    self
  }

  /// Contains information about the current recovery email address
  pub fn on_recovery_email_address<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, RecoveryEmailAddress)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.recovery_email_address = Some(Arc::new(fnc));
    self
  }

  /// Describes a message with rich formatting
  pub fn on_rich_message<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, RichMessage)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.rich_message = Some(Arc::new(fnc));
    self
  }

  /// Represents an RTMP URL
  pub fn on_rtmp_url<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, RtmpUrl)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.rtmp_url = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of tags used in Saved Messages
  pub fn on_saved_messages_tags<F>(&mut self, fnc: F) -> &mut Self
  where
    F:
      Send + Sync + Fn((Api, SavedMessagesTags)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.saved_messages_tags = Some(Arc::new(fnc));
    self
  }

  /// Contains information about notification settings for several chats
  pub fn on_scope_notification_settings<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, ScopeNotificationSettings)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.scope_notification_settings = Some(Arc::new(fnc));
    self
  }

  /// Contains a value representing a number of seconds
  pub fn on_seconds<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Seconds)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.seconds = Some(Arc::new(fnc));
    self
  }

  /// Represents a secret chat
  pub fn on_secret_chat<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, SecretChat)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.secret_chat = Some(Arc::new(fnc));
    self
  }

  /// Contains information about one session in a Telegram application used by the current user. Sessions must be shown to the user in the returned order
  pub fn on_session<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Session)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.session = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of sessions
  pub fn on_sessions<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Sessions)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.sessions = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of sponsored chats
  pub fn on_sponsored_chats<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, SponsoredChats)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.sponsored_chats = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of sponsored messages
  pub fn on_sponsored_messages<F>(&mut self, fnc: F) -> &mut Self
  where
    F:
      Send + Sync + Fn((Api, SponsoredMessages)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.sponsored_messages = Some(Arc::new(fnc));
    self
  }

  /// Describes state of the stake dice
  pub fn on_stake_dice_state<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, StakeDiceState)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.stake_dice_state = Some(Arc::new(fnc));
    self
  }

  /// Describes a possibly non-integer Telegram Star amount
  pub fn on_star_amount<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, StarAmount)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.star_amount = Some(Arc::new(fnc));
    self
  }

  /// Contains a number of Telegram Stars
  pub fn on_star_count<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, StarCount)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.star_count = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of options for creating of Telegram Star giveaway
  pub fn on_star_giveaway_payment_options<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, StarGiveawayPaymentOptions)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.star_giveaway_payment_options = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of options for buying Telegram Stars
  pub fn on_star_payment_options<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, StarPaymentOptions)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.star_payment_options = Some(Arc::new(fnc));
    self
  }

  /// A detailed statistics about Telegram Stars earned by a user or a chat
  pub fn on_star_revenue_statistics<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, StarRevenueStatistics)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.star_revenue_statistics = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of Telegram Star subscriptions
  pub fn on_star_subscriptions<F>(&mut self, fnc: F) -> &mut Self
  where
    F:
      Send + Sync + Fn((Api, StarSubscriptions)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.star_subscriptions = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of Telegram Star transactions
  pub fn on_star_transactions<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, StarTransactions)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.star_transactions = Some(Arc::new(fnc));
    self
  }

  /// Describes a sticker
  pub fn on_sticker<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Sticker)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.sticker = Some(Arc::new(fnc));
    self
  }

  /// Represents a sticker set
  pub fn on_sticker_set<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, StickerSet)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.sticker_set = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of sticker sets
  pub fn on_sticker_sets<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, StickerSets)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.sticker_sets = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of stickers
  pub fn on_stickers<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Stickers)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.stickers = Some(Arc::new(fnc));
    self
  }

  /// Contains the exact storage usage statistics split by chats and file type
  pub fn on_storage_statistics<F>(&mut self, fnc: F) -> &mut Self
  where
    F:
      Send + Sync + Fn((Api, StorageStatistics)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.storage_statistics = Some(Arc::new(fnc));
    self
  }

  /// Contains approximate storage usage statistics, excluding files of unknown file type
  pub fn on_storage_statistics_fast<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, StorageStatisticsFast)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.storage_statistics_fast = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of stories
  pub fn on_stories<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Stories)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.stories = Some(Arc::new(fnc));
    self
  }

  /// Represents a story
  pub fn on_story<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Story)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.story = Some(Arc::new(fnc));
    self
  }

  /// Describes album of stories
  pub fn on_story_album<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, StoryAlbum)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.story_album = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of story albums
  pub fn on_story_albums<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, StoryAlbums)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.story_albums = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of interactions with a story
  pub fn on_story_interactions<F>(&mut self, fnc: F) -> &mut Self
  where
    F:
      Send + Sync + Fn((Api, StoryInteractions)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.story_interactions = Some(Arc::new(fnc));
    self
  }

  /// A detailed statistics about a story
  pub fn on_story_statistics<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, StoryStatistics)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.story_statistics = Some(Arc::new(fnc));
    self
  }

  /// Represents a supergroup or channel with zero or more members (subscribers in the case of channels)
  pub fn on_supergroup<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Supergroup)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.supergroup = Some(Arc::new(fnc));
    self
  }

  /// Contains full information about a supergroup or channel
  pub fn on_supergroup_full_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, SupergroupFullInfo)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.supergroup_full_info = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of t.me URLs
  pub fn on_t_me_urls<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, TMeUrls)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.t_me_urls = Some(Arc::new(fnc));
    self
  }

  /// Contains information about the availability of a temporary password, which can be used for payments and withdrawal of upgraded gifts
  pub fn on_temporary_password_state<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, TemporaryPasswordState)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.temporary_password_state = Some(Arc::new(fnc));
    self
  }

  /// A simple object containing a sequence of bytes; for testing only
  pub fn on_test_bytes<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, TestBytes)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.test_bytes = Some(Arc::new(fnc));
    self
  }

  /// A simple object containing a number; for testing only
  pub fn on_test_int<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, TestInt)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.test_int = Some(Arc::new(fnc));
    self
  }

  /// A simple object containing a string; for testing only
  pub fn on_test_string<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, TestString)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.test_string = Some(Arc::new(fnc));
    self
  }

  /// A simple object containing a vector of numbers; for testing only
  pub fn on_test_vector_int<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, TestVectorInt)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.test_vector_int = Some(Arc::new(fnc));
    self
  }

  /// A simple object containing a vector of objects that hold a number; for testing only
  pub fn on_test_vector_int_object<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, TestVectorIntObject)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.test_vector_int_object = Some(Arc::new(fnc));
    self
  }

  /// A simple object containing a vector of strings; for testing only
  pub fn on_test_vector_string<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, TestVectorString)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.test_vector_string = Some(Arc::new(fnc));
    self
  }

  /// A simple object containing a vector of objects that hold a string; for testing only
  pub fn on_test_vector_string_object<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, TestVectorStringObject)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.test_vector_string_object = Some(Arc::new(fnc));
    self
  }

  /// Contains some text
  pub fn on_text<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Text)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.text = Some(Arc::new(fnc));
    self
  }

  /// Describes a style that can be used to compose a text
  pub fn on_text_composition_style<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, TextCompositionStyle)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.text_composition_style = Some(Arc::new(fnc));
    self
  }

  /// Contains an example of text composition style usage
  pub fn on_text_composition_style_example<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, TextCompositionStyleExample)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.text_composition_style_example = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of text entities
  pub fn on_text_entities<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, TextEntities)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.text_entities = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of time zones
  pub fn on_time_zones<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, TimeZones)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.time_zones = Some(Arc::new(fnc));
    self
  }

  /// Contains an HTTP URL for TON Center Streaming API
  pub fn on_ton_center_streaming_api_url<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, TonCenterStreamingApiUrl)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.ton_center_streaming_api_url = Some(Arc::new(fnc));
    self
  }

  /// Represents a TON Connect challenge
  pub fn on_ton_connect_challenge<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, TonConnectChallenge)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.ton_connect_challenge = Some(Arc::new(fnc));
    self
  }

  /// Represents a session with its pending TON Connect requests
  pub fn on_ton_connect_requests<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, TonConnectRequests)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.ton_connect_requests = Some(Arc::new(fnc));
    self
  }

  /// Represents a TON Connect session. See https://github.com/ton-blockchain/ton-connect/tree/main/spec for full TON Connect protocol documentation
  pub fn on_ton_connect_session<F>(&mut self, fnc: F) -> &mut Self
  where
    F:
      Send + Sync + Fn((Api, TonConnectSession)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.ton_connect_session = Some(Arc::new(fnc));
    self
  }

  /// Represents identifier of an event in a TON Connect session
  pub fn on_ton_connect_session_event_id<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, TonConnectSessionEventId)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.ton_connect_session_event_id = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of TON Connect sessions
  pub fn on_ton_connect_sessions<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, TonConnectSessions)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.ton_connect_sessions = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of TON NFTs
  pub fn on_ton_nfts<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, TonNfts)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.ton_nfts = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of TON Gram transactions
  pub fn on_ton_transactions<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, TonTransactions)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.ton_transactions = Some(Arc::new(fnc));
    self
  }

  /// Describes a TON wallet card, which is shown on the profile page of a user to everyone
  pub fn on_ton_wallet_card<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, TonWalletCard)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.ton_wallet_card = Some(Arc::new(fnc));
    self
  }

  /// Represents a challenge for TON wallet ownership proof
  pub fn on_ton_wallet_ownership_proof_challenge<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, TonWalletOwnershipProofChallenge)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.ton_wallet_ownership_proof_challenge = Some(Arc::new(fnc));
    self
  }

  /// Contains the secret phrase of a TON wallet
  pub fn on_ton_wallet_secret_phrase<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, TonWalletSecretPhrase)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.ton_wallet_secret_phrase = Some(Arc::new(fnc));
    self
  }

  /// Represents a transaction with the TON wallet of the current user
  pub fn on_ton_wallet_transaction<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, TonWalletTransaction)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.ton_wallet_transaction = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of TON Gram transactions
  pub fn on_ton_wallet_transactions<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, TonWalletTransactions)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.ton_wallet_transactions = Some(Arc::new(fnc));
    self
  }

  /// Represents result of cryptocurrency transfer from the current user's TON wallet
  pub fn on_ton_wallet_transfer_result<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, TonWalletTransferResult)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.ton_wallet_transfer_result = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of trending sticker sets
  pub fn on_trending_sticker_sets<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, TrendingStickerSets)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.trending_sticker_sets = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of updates
  pub fn on_updates<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Updates)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.updates = Some(Arc::new(fnc));
    self
  }

  /// Contains result of gift upgrading
  pub fn on_upgrade_gift_result<F>(&mut self, fnc: F) -> &mut Self
  where
    F:
      Send + Sync + Fn((Api, UpgradeGiftResult)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.upgrade_gift_result = Some(Arc::new(fnc));
    self
  }

  /// Describes an upgraded gift that can be transferred to another owner or transferred to the TON blockchain as an NFT
  pub fn on_upgraded_gift<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UpgradedGift)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.upgraded_gift = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of names of upgraded gift collections. The name of the collection of an upgraded gift is the part of upgradedGift.name before the last "-"
  pub fn on_upgraded_gift_collection_names<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpgradedGiftCollectionNames)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.upgraded_gift_collection_names = Some(Arc::new(fnc));
    self
  }

  /// Contains information needed to deposit an upgraded gift from the TON wallet of the current user to a Telegram account. The gift must be transferred by an internal message from the wallet to nft_address with gram_amount attached and with payload as the message body
  pub fn on_upgraded_gift_deposit_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpgradedGiftDepositInfo)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.upgraded_gift_deposit_info = Some(Arc::new(fnc));
    self
  }

  /// Contains information about value of an upgraded gift
  pub fn on_upgraded_gift_value_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UpgradedGiftValueInfo)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.upgraded_gift_value_info = Some(Arc::new(fnc));
    self
  }

  /// Represents a user
  pub fn on_user<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, User)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.user = Some(Arc::new(fnc));
    self
  }

  /// Contains full information about a user
  pub fn on_user_full_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UserFullInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.user_full_info = Some(Arc::new(fnc));
    self
  }

  /// Contains an HTTPS URL, which can be used to get information about a user
  pub fn on_user_link<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UserLink)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.user_link = Some(Arc::new(fnc));
    self
  }

  /// A list of privacy rules. Rules are matched in the specified order. The first matched rule defines the privacy setting for a given user. If no rule matches, the action is not allowed
  pub fn on_user_privacy_setting_rules<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UserPrivacySettingRules)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.user_privacy_setting_rules = Some(Arc::new(fnc));
    self
  }

  /// Contains custom information about the user
  pub fn on_user_support_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, UserSupportInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.user_support_info = Some(Arc::new(fnc));
    self
  }

  /// Contains TON wallet address of a user
  pub fn on_user_ton_wallet_address<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UserTonWalletAddress)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.user_ton_wallet_address = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of TON wallet addresses of users
  pub fn on_user_ton_wallet_addresses<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, UserTonWalletAddresses)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.user_ton_wallet_addresses = Some(Arc::new(fnc));
    self
  }

  /// Represents a list of users
  pub fn on_users<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, Users)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.users = Some(Arc::new(fnc));
    self
  }

  /// Contains a temporary identifier of validated order information, which is stored for one hour, and the available shipping options
  pub fn on_validated_order_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, ValidatedOrderInfo)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.validated_order_info = Some(Arc::new(fnc));
    self
  }

  /// Contains a list of advertisements to be shown while a video from a message is watched
  pub fn on_video_message_advertisements<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, VideoMessageAdvertisements)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.video_message_advertisements = Some(Arc::new(fnc));
    self
  }

  /// Describes balance of the current user in the bot https://t.me/walt
  pub fn on_wallet_bot_balance<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, WalletBotBalance)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.wallet_bot_balance = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a Web App
  pub fn on_web_app_info<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, WebAppInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.web_app_info = Some(Arc::new(fnc));
    self
  }

  /// Contains information about a Web App URL
  pub fn on_web_app_url<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send + Sync + Fn((Api, WebAppUrl)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
  {
    self.web_app_url = Some(Arc::new(fnc));
    self
  }

  /// Describes an instant view page for a web page
  pub fn on_web_page_instant_view<F>(&mut self, fnc: F) -> &mut Self
  where
    F: Send
      + Sync
      + Fn((Api, WebPageInstantView)) -> LocalBoxFuture<'static, TGResult<()>>
      + 'static,
  {
    self.web_page_instant_view = Some(Arc::new(fnc));
    self
  }
}

/// Get listener
#[derive(Clone)]
pub struct RasyncLout {
  listener: RasyncListener,
  supports: Vec<&'static str>,
}

impl RasyncLout {
  fn new(listener: RasyncListener) -> Self {
    let supports = vec![
      "testUseUpdate",
      "updateAccentColors",
      "updateActiveEmojiReactions",
      "updateActiveGiftAuctions",
      "updateActiveLiveLocationMessages",
      "updateActiveNotifications",
      "updateAgeVerificationParameters",
      "updateAllowedBotUsernamePremiumSuffixes",
      "updateAnimatedEmojiMessageClicked",
      "updateAnimationSearchParameters",
      "updateApplicationRecaptchaVerificationRequired",
      "updateApplicationVerificationRequired",
      "updateAttachmentMenuBots",
      "updateAuthorizationState",
      "updateAutosaveSettings",
      "updateAvailableMessageEffects",
      "updateBasicGroup",
      "updateBasicGroupFullInfo",
      "updateBusinessConnection",
      "updateBusinessMessageEdited",
      "updateBusinessMessagesDeleted",
      "updateCall",
      "updateChatAccentColors",
      "updateChatAction",
      "updateChatActionBar",
      "updateChatActiveStories",
      "updateChatAddedToList",
      "updateChatAvailableReactions",
      "updateChatBackground",
      "updateChatBlockList",
      "updateChatBoost",
      "updateChatBusinessBotManageBar",
      "updateChatDefaultDisableNotification",
      "updateChatDraftMessage",
      "updateChatEmojiStatus",
      "updateChatFolders",
      "updateChatHasProtectedContent",
      "updateChatHasScheduledMessages",
      "updateChatHasWelcomeMessages",
      "updateChatIsMarkedAsUnread",
      "updateChatIsTranslatable",
      "updateChatJoinResult",
      "updateChatLastMessage",
      "updateChatMember",
      "updateChatMessageAutoDeleteTime",
      "updateChatMessageSender",
      "updateChatNotificationSettings",
      "updateChatOnlineMemberCount",
      "updateChatPendingJoinRequests",
      "updateChatPermissions",
      "updateChatPhoto",
      "updateChatPosition",
      "updateChatReadInbox",
      "updateChatReadOutbox",
      "updateChatRemovedFromList",
      "updateChatReplyMarkup",
      "updateChatRevenueAmount",
      "updateChatTheme",
      "updateChatTitle",
      "updateChatUnreadMentionCount",
      "updateChatUnreadPollVoteCount",
      "updateChatUnreadReactionCount",
      "updateChatVideoChat",
      "updateChatViewAsTopics",
      "updateChatWelcomeMessages",
      "updateCommunity",
      "updateCommunityFullInfo",
      "updateConnectionState",
      "updateContactCloseBirthdays",
      "updateDefaultBackground",
      "updateDefaultPaidReactionType",
      "updateDefaultReactionType",
      "updateDeleteMessages",
      "updateDiceEmojis",
      "updateDirectMessagesChatTopic",
      "updateEmojiChatThemes",
      "updateFavoriteStickers",
      "updateFile",
      "updateFileAddedToDownloads",
      "updateFileDownload",
      "updateFileDownloads",
      "updateFileGenerationStart",
      "updateFileGenerationStop",
      "updateFileRemovedFromDownloads",
      "updateForumTopic",
      "updateForumTopicInfo",
      "updateFreezeState",
      "updateGiftAuctionState",
      "updateGramRevenueStatus",
      "updateGroupCall",
      "updateGroupCallMessageLevels",
      "updateGroupCallMessageSendFailed",
      "updateGroupCallMessagesDeleted",
      "updateGroupCallParticipant",
      "updateGroupCallParticipants",
      "updateGroupCallVerificationState",
      "updateHavePendingNotifications",
      "updateInstalledStickerSets",
      "updateLanguagePackStrings",
      "updateLiveStoryTopDonors",
      "updateManagedBot",
      "updateMessageContainsUnreadPollVotes",
      "updateMessageContent",
      "updateMessageContentOpened",
      "updateMessageEdited",
      "updateMessageEphemeralContent",
      "updateMessageFactCheck",
      "updateMessageInteractionInfo",
      "updateMessageIsPinned",
      "updateMessageLiveLocationViewed",
      "updateMessageMentionRead",
      "updateMessageReaction",
      "updateMessageReactions",
      "updateMessageSendAcknowledged",
      "updateMessageSendFailed",
      "updateMessageSendSucceeded",
      "updateMessageSuggestedPostInfo",
      "updateMessageUnreadReactions",
      "updateNewBusinessCallbackQuery",
      "updateNewBusinessMessage",
      "updateNewCallSignalingData",
      "updateNewCallbackQuery",
      "updateNewChat",
      "updateNewChatJoinRequest",
      "updateNewChosenInlineResult",
      "updateNewCustomEvent",
      "updateNewCustomQuery",
      "updateNewGroupCallMessage",
      "updateNewGroupCallPaidReaction",
      "updateNewGuestQuery",
      "updateNewInlineCallbackQuery",
      "updateNewInlineQuery",
      "updateNewMessage",
      "updateNewOauthRequest",
      "updateNewPreCheckoutQuery",
      "updateNewShippingQuery",
      "updateNotification",
      "updateNotificationGroup",
      "updateOption",
      "updateOwnedGramCount",
      "updateOwnedStarCount",
      "updatePaidMediaPurchased",
      "updatePendingMessage",
      "updatePoll",
      "updatePollAnswer",
      "updateProfileAccentColors",
      "updateQuickReplyShortcut",
      "updateQuickReplyShortcutDeleted",
      "updateQuickReplyShortcutMessages",
      "updateQuickReplyShortcuts",
      "updateReactionNotificationSettings",
      "updateRecentStickers",
      "updateSavedAnimations",
      "updateSavedMessagesTags",
      "updateSavedMessagesTopic",
      "updateSavedMessagesTopicCount",
      "updateSavedNotificationSounds",
      "updateScopeNotificationSettings",
      "updateSecretChat",
      "updateServiceNotification",
      "updateSpeechRecognitionTrial",
      "updateSpeedLimitNotification",
      "updateStakeDiceState",
      "updateStarRevenueStatus",
      "updateStickerSet",
      "updateStopMessageDraft",
      "updateStory",
      "updateStoryDeleted",
      "updateStoryListChatCount",
      "updateStoryPostFailed",
      "updateStoryPostSucceeded",
      "updateStoryStealthMode",
      "updateSuggestedActions",
      "updateSupergroup",
      "updateSupergroupFullInfo",
      "updateTermsOfService",
      "updateTextCompositionStyles",
      "updateTonWalletGaslessTransfersInfo",
      "updateTonWalletState",
      "updateTonWalletTonConnectSession",
      "updateTonWalletTonConnectSessionDisconnectRequired",
      "updateTopicMessageCount",
      "updateTrendingStickerSets",
      "updateTrustedMiniAppBots",
      "updateUnconfirmedSession",
      "updateUnreadChatCount",
      "updateUnreadMessageCount",
      "updateUser",
      "updateUserFullInfo",
      "updateUserPrivacySettingRules",
      "updateUserStatus",
      "updateUserSubscription",
      "updateVideoPublished",
      "updateWebAppMessageSent",
      "updateWebBrowserSettings",
      "updateWebProxyCarrierClosed",
      "updateWebProxyCarrierData",
      "updateWebProxyCarrierOpened",
      "AuthorizationState",
      "CanPostStoryResult",
      "CanSendGiftResult",
      "CanSendMessageToUserResult",
      "CanTransferOwnershipResult",
      "ChatJoinResult",
      "ChatStatistics",
      "CheckChatUsernameResult",
      "CheckStickerSetNameResult",
      "CraftGiftResult",
      "GiftResaleResult",
      "GiveawayInfo",
      "InternalLinkType",
      "InviteGroupCallParticipantResult",
      "JsonValue",
      "LanguagePackStringValue",
      "LogStream",
      "LoginUrlInfo",
      "MessageFileType",
      "MessageReadDate",
      "MessageSender",
      "OptionValue",
      "PassportElement",
      "ReportChatResult",
      "ReportSponsoredResult",
      "ReportStoryResult",
      "ResetPasswordResult",
      "StartLiveStoryResult",
      "StatisticalGraph",
      "Update",
      "WebBrowserType",
      "accountTtl",
      "addedProxies",
      "addedProxy",
      "addedReactions",
      "animatedEmoji",
      "animation",
      "animations",
      "archiveChatListSettings",
      "attachmentMenuBot",
      "audios",
      "authenticationCodeInfo",
      "autoDownloadSettingsPresets",
      "autosaveSettings",
      "availableGifts",
      "availableReactions",
      "background",
      "backgrounds",
      "bankCardInfo",
      "basicGroup",
      "basicGroupFullInfo",
      "botAccessSettings",
      "botCommands",
      "botMediaPreview",
      "botMediaPreviewInfo",
      "botMediaPreviews",
      "botMenuButton",
      "businessChatLink",
      "businessChatLinkInfo",
      "businessChatLinks",
      "businessConnectedBotInfo",
      "businessConnection",
      "businessFeatures",
      "businessMessage",
      "businessMessages",
      "callId",
      "callbackQueryAnswer",
      "chat",
      "chatActiveStories",
      "chatAdministrators",
      "chatBoostFeatures",
      "chatBoostLevelFeatures",
      "chatBoostLink",
      "chatBoostLinkInfo",
      "chatBoostSlots",
      "chatBoostStatus",
      "chatEvents",
      "chatFolder",
      "chatFolderIcon",
      "chatFolderInfo",
      "chatFolderInviteLink",
      "chatFolderInviteLinkInfo",
      "chatFolderInviteLinks",
      "chatInviteLink",
      "chatInviteLinkCounts",
      "chatInviteLinkInfo",
      "chatInviteLinkMembers",
      "chatInviteLinks",
      "chatJoinRequests",
      "chatLists",
      "chatMember",
      "chatMembers",
      "chatMessageSenders",
      "chatPhotos",
      "chatRevenueStatistics",
      "chatRevenueTransactions",
      "chats",
      "collectibleItemInfo",
      "communityFullInfo",
      "communityId",
      "connectedAffiliateProgram",
      "connectedAffiliatePrograms",
      "connectedWebsites",
      "count",
      "countries",
      "countryInfo",
      "createdBasicGroupChat",
      "currencies",
      "currencyExchangeRates",
      "currentWeather",
      "customRequestResult",
      "data",
      "databaseStatistics",
      "deepLinkInfo",
      "directMessagesChatTopic",
      "emailAddressAuthenticationCodeInfo",
      "emojiCategories",
      "emojiKeywords",
      "emojiReaction",
      "emojiStatusCustomEmojis",
      "emojiStatuses",
      "emojis",
      "error",
      "failedToAddMembers",
      "file",
      "fileDownloadedPrefixSize",
      "fixedText",
      "formattedText",
      "forumTopic",
      "forumTopicInfo",
      "forumTopics",
      "foundAffiliatePrograms",
      "foundChatBoosts",
      "foundChatMessages",
      "foundFileDownloads",
      "foundMessages",
      "foundPosition",
      "foundPositions",
      "foundPublicPosts",
      "foundStories",
      "foundUsers",
      "foundWebApp",
      "gameHighScores",
      "giftAuctionAcquiredGifts",
      "giftAuctionState",
      "giftChatThemes",
      "giftCollection",
      "giftCollections",
      "giftUpgradePreview",
      "giftUpgradeVariants",
      "giftsForCrafting",
      "giftsForResale",
      "gramRevenueStatistics",
      "groupCall",
      "groupCallId",
      "groupCallInfo",
      "groupCallParticipant",
      "groupCallParticipants",
      "groupCallStreams",
      "hashtags",
      "httpUrl",
      "importedContacts",
      "inlineMessageId",
      "inlineQueryResults",
      "keyboardButton",
      "languagePackInfo",
      "languagePackStrings",
      "linkPreview",
      "liveStoryDonors",
      "localizationTargetInfo",
      "logTags",
      "logVerbosityLevel",
      "mainWebApp",
      "message",
      "messageAutoDeleteTime",
      "messageCalendar",
      "messageEffect",
      "messageLink",
      "messageLinkInfo",
      "messagePositions",
      "messageProperties",
      "messageSenders",
      "messageStatistics",
      "messageThreadInfo",
      "messageViewers",
      "messages",
      "networkStatistics",
      "newChatPrivacySettings",
      "notificationSound",
      "notificationSounds",
      "oauthLinkInfo",
      "ok",
      "onRampPaymentAvailability",
      "onRampPaymentLimits",
      "onRampPaymentQuote",
      "onRampPaymentSession",
      "onRampProviders",
      "orderInfo",
      "outline",
      "passkey",
      "passkeys",
      "passportAuthorizationForm",
      "passportElements",
      "passportElementsWithErrors",
      "passwordState",
      "paymentForm",
      "paymentReceipt",
      "paymentResult",
      "phoneNumberInfo",
      "pollOptionProperties",
      "pollVoteStatistics",
      "pollVoters",
      "premiumFeatures",
      "premiumGiftCodeInfo",
      "premiumGiftPaymentOptions",
      "premiumGiveawayPaymentOptions",
      "premiumLimit",
      "premiumState",
      "preparedInlineMessage",
      "preparedInlineMessageId",
      "profileShareThemeBackdrops",
      "publicForwards",
      "publicPostSearchLimits",
      "pushReceiverId",
      "quickReplyMessage",
      "quickReplyMessages",
      "readDatePrivacySettings",
      "receivedGift",
      "receivedGifts",
      "recommendedChatFolders",
      "recoveryEmailAddress",
      "richMessage",
      "rtmpUrl",
      "savedMessagesTags",
      "scopeNotificationSettings",
      "seconds",
      "secretChat",
      "session",
      "sessions",
      "sponsoredChats",
      "sponsoredMessages",
      "stakeDiceState",
      "starAmount",
      "starCount",
      "starGiveawayPaymentOptions",
      "starPaymentOptions",
      "starRevenueStatistics",
      "starSubscriptions",
      "starTransactions",
      "sticker",
      "stickerSet",
      "stickerSets",
      "stickers",
      "storageStatistics",
      "storageStatisticsFast",
      "stories",
      "story",
      "storyAlbum",
      "storyAlbums",
      "storyInteractions",
      "storyStatistics",
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
      "textCompositionStyle",
      "textCompositionStyleExample",
      "textEntities",
      "timeZones",
      "tonCenterStreamingApiUrl",
      "tonConnectChallenge",
      "tonConnectRequests",
      "tonConnectSession",
      "tonConnectSessionEventId",
      "tonConnectSessions",
      "tonNfts",
      "tonTransactions",
      "tonWalletCard",
      "tonWalletOwnershipProofChallenge",
      "tonWalletSecretPhrase",
      "tonWalletTransaction",
      "tonWalletTransactions",
      "tonWalletTransferResult",
      "trendingStickerSets",
      "updates",
      "upgradeGiftResult",
      "upgradedGift",
      "upgradedGiftCollectionNames",
      "upgradedGiftDepositInfo",
      "upgradedGiftValueInfo",
      "user",
      "userFullInfo",
      "userLink",
      "userPrivacySettingRules",
      "userSupportInfo",
      "userTonWalletAddress",
      "userTonWalletAddresses",
      "users",
      "validatedOrderInfo",
      "videoMessageAdvertisements",
      "walletBotBalance",
      "webAppInfo",
      "webAppUrl",
      "webPageInstantView",
    ];
    Self { listener, supports }
  }

  pub fn is_support<S: AsRef<str>>(&self, name: S) -> bool {
    self
      .supports
      .iter()
      .find(|&&item| item == name.as_ref())
      .is_some()
  }

  pub async fn handle_type(&self, api: &Api, td_type: &TdType) -> TGResult<bool> {
    match td_type {
      TdType::TestUseUpdate(value) => match &self.listener.test_use_update {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateAccentColors(value) => match &self.listener.update_accent_colors {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateActiveEmojiReactions(value) => {
        match &self.listener.update_active_emoji_reactions {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateActiveGiftAuctions(value) => match &self.listener.update_active_gift_auctions {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateActiveLiveLocationMessages(value) => {
        match &self.listener.update_active_live_location_messages {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateActiveNotifications(value) => {
        match &self.listener.update_active_notifications {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateAgeVerificationParameters(value) => {
        match &self.listener.update_age_verification_parameters {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateAllowedBotUsernamePremiumSuffixes(value) => {
        match &self.listener.update_allowed_bot_username_premium_suffixes {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateAnimatedEmojiMessageClicked(value) => {
        match &self.listener.update_animated_emoji_message_clicked {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateAnimationSearchParameters(value) => {
        match &self.listener.update_animation_search_parameters {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateApplicationRecaptchaVerificationRequired(value) => match &self
        .listener
        .update_application_recaptcha_verification_required
      {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateApplicationVerificationRequired(value) => {
        match &self.listener.update_application_verification_required {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateAttachmentMenuBots(value) => match &self.listener.update_attachment_menu_bots {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateAuthorizationState(value) => match &self.listener.update_authorization_state {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateAutosaveSettings(value) => match &self.listener.update_autosave_settings {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateAvailableMessageEffects(value) => {
        match &self.listener.update_available_message_effects {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateBasicGroup(value) => match &self.listener.update_basic_group {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateBasicGroupFullInfo(value) => {
        match &self.listener.update_basic_group_full_info {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateBusinessConnection(value) => match &self.listener.update_business_connection {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateBusinessMessageEdited(value) => {
        match &self.listener.update_business_message_edited {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateBusinessMessagesDeleted(value) => {
        match &self.listener.update_business_messages_deleted {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateCall(value) => match &self.listener.update_call {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatAccentColors(value) => match &self.listener.update_chat_accent_colors {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatAction(value) => match &self.listener.update_chat_action {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatActionBar(value) => match &self.listener.update_chat_action_bar {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatActiveStories(value) => match &self.listener.update_chat_active_stories {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatAddedToList(value) => match &self.listener.update_chat_added_to_list {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatAvailableReactions(value) => {
        match &self.listener.update_chat_available_reactions {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateChatBackground(value) => match &self.listener.update_chat_background {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatBlockList(value) => match &self.listener.update_chat_block_list {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatBoost(value) => match &self.listener.update_chat_boost {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatBusinessBotManageBar(value) => {
        match &self.listener.update_chat_business_bot_manage_bar {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateChatDefaultDisableNotification(value) => {
        match &self.listener.update_chat_default_disable_notification {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateChatDraftMessage(value) => match &self.listener.update_chat_draft_message {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatEmojiStatus(value) => match &self.listener.update_chat_emoji_status {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatFolders(value) => match &self.listener.update_chat_folders {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatHasProtectedContent(value) => {
        match &self.listener.update_chat_has_protected_content {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateChatHasScheduledMessages(value) => {
        match &self.listener.update_chat_has_scheduled_messages {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateChatHasWelcomeMessages(value) => {
        match &self.listener.update_chat_has_welcome_messages {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateChatIsMarkedAsUnread(value) => {
        match &self.listener.update_chat_is_marked_as_unread {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateChatIsTranslatable(value) => match &self.listener.update_chat_is_translatable {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatJoinResult(value) => match &self.listener.update_chat_join_result {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatLastMessage(value) => match &self.listener.update_chat_last_message {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatMember(value) => match &self.listener.update_chat_member {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatMessageAutoDeleteTime(value) => {
        match &self.listener.update_chat_message_auto_delete_time {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateChatMessageSender(value) => match &self.listener.update_chat_message_sender {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatNotificationSettings(value) => {
        match &self.listener.update_chat_notification_settings {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateChatOnlineMemberCount(value) => {
        match &self.listener.update_chat_online_member_count {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateChatPendingJoinRequests(value) => {
        match &self.listener.update_chat_pending_join_requests {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateChatPermissions(value) => match &self.listener.update_chat_permissions {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatPhoto(value) => match &self.listener.update_chat_photo {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatPosition(value) => match &self.listener.update_chat_position {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatReadInbox(value) => match &self.listener.update_chat_read_inbox {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatReadOutbox(value) => match &self.listener.update_chat_read_outbox {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatRemovedFromList(value) => {
        match &self.listener.update_chat_removed_from_list {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateChatReplyMarkup(value) => match &self.listener.update_chat_reply_markup {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatRevenueAmount(value) => match &self.listener.update_chat_revenue_amount {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatTheme(value) => match &self.listener.update_chat_theme {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatTitle(value) => match &self.listener.update_chat_title {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatUnreadMentionCount(value) => {
        match &self.listener.update_chat_unread_mention_count {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateChatUnreadPollVoteCount(value) => {
        match &self.listener.update_chat_unread_poll_vote_count {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateChatUnreadReactionCount(value) => {
        match &self.listener.update_chat_unread_reaction_count {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateChatVideoChat(value) => match &self.listener.update_chat_video_chat {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatViewAsTopics(value) => match &self.listener.update_chat_view_as_topics {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateChatWelcomeMessages(value) => match &self.listener.update_chat_welcome_messages
      {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateCommunity(value) => match &self.listener.update_community {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateCommunityFullInfo(value) => match &self.listener.update_community_full_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateConnectionState(value) => match &self.listener.update_connection_state {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateContactCloseBirthdays(value) => {
        match &self.listener.update_contact_close_birthdays {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateDefaultBackground(value) => match &self.listener.update_default_background {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateDefaultPaidReactionType(value) => {
        match &self.listener.update_default_paid_reaction_type {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateDefaultReactionType(value) => match &self.listener.update_default_reaction_type
      {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateDeleteMessages(value) => match &self.listener.update_delete_messages {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateDiceEmojis(value) => match &self.listener.update_dice_emojis {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateDirectMessagesChatTopic(value) => {
        match &self.listener.update_direct_messages_chat_topic {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateEmojiChatThemes(value) => match &self.listener.update_emoji_chat_themes {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateFavoriteStickers(value) => match &self.listener.update_favorite_stickers {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateFile(value) => match &self.listener.update_file {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateFileAddedToDownloads(value) => {
        match &self.listener.update_file_added_to_downloads {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateFileDownload(value) => match &self.listener.update_file_download {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateFileDownloads(value) => match &self.listener.update_file_downloads {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateFileGenerationStart(value) => match &self.listener.update_file_generation_start
      {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateFileGenerationStop(value) => match &self.listener.update_file_generation_stop {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateFileRemovedFromDownloads(value) => {
        match &self.listener.update_file_removed_from_downloads {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateForumTopic(value) => match &self.listener.update_forum_topic {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateForumTopicInfo(value) => match &self.listener.update_forum_topic_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateFreezeState(value) => match &self.listener.update_freeze_state {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateGiftAuctionState(value) => match &self.listener.update_gift_auction_state {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateGramRevenueStatus(value) => match &self.listener.update_gram_revenue_status {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateGroupCall(value) => match &self.listener.update_group_call {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateGroupCallMessageLevels(value) => {
        match &self.listener.update_group_call_message_levels {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateGroupCallMessageSendFailed(value) => {
        match &self.listener.update_group_call_message_send_failed {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateGroupCallMessagesDeleted(value) => {
        match &self.listener.update_group_call_messages_deleted {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateGroupCallParticipant(value) => {
        match &self.listener.update_group_call_participant {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateGroupCallParticipants(value) => {
        match &self.listener.update_group_call_participants {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateGroupCallVerificationState(value) => {
        match &self.listener.update_group_call_verification_state {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateHavePendingNotifications(value) => {
        match &self.listener.update_have_pending_notifications {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateInstalledStickerSets(value) => {
        match &self.listener.update_installed_sticker_sets {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateLanguagePackStrings(value) => match &self.listener.update_language_pack_strings
      {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateLiveStoryTopDonors(value) => {
        match &self.listener.update_live_story_top_donors {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateManagedBot(value) => match &self.listener.update_managed_bot {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateMessageContainsUnreadPollVotes(value) => {
        match &self.listener.update_message_contains_unread_poll_votes {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateMessageContent(value) => match &self.listener.update_message_content {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateMessageContentOpened(value) => {
        match &self.listener.update_message_content_opened {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateMessageEdited(value) => match &self.listener.update_message_edited {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateMessageEphemeralContent(value) => {
        match &self.listener.update_message_ephemeral_content {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateMessageFactCheck(value) => match &self.listener.update_message_fact_check {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateMessageInteractionInfo(value) => {
        match &self.listener.update_message_interaction_info {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateMessageIsPinned(value) => match &self.listener.update_message_is_pinned {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateMessageLiveLocationViewed(value) => {
        match &self.listener.update_message_live_location_viewed {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateMessageMentionRead(value) => match &self.listener.update_message_mention_read {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateMessageReaction(value) => match &self.listener.update_message_reaction {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateMessageReactions(value) => match &self.listener.update_message_reactions {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateMessageSendAcknowledged(value) => {
        match &self.listener.update_message_send_acknowledged {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateMessageSendFailed(value) => match &self.listener.update_message_send_failed {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateMessageSendSucceeded(value) => {
        match &self.listener.update_message_send_succeeded {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateMessageSuggestedPostInfo(value) => {
        match &self.listener.update_message_suggested_post_info {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateMessageUnreadReactions(value) => {
        match &self.listener.update_message_unread_reactions {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateNewBusinessCallbackQuery(value) => {
        match &self.listener.update_new_business_callback_query {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateNewBusinessMessage(value) => match &self.listener.update_new_business_message {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateNewCallSignalingData(value) => {
        match &self.listener.update_new_call_signaling_data {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateNewCallbackQuery(value) => match &self.listener.update_new_callback_query {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateNewChat(value) => match &self.listener.update_new_chat {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateNewChatJoinRequest(value) => {
        match &self.listener.update_new_chat_join_request {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateNewChosenInlineResult(value) => {
        match &self.listener.update_new_chosen_inline_result {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateNewCustomEvent(value) => match &self.listener.update_new_custom_event {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateNewCustomQuery(value) => match &self.listener.update_new_custom_query {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateNewGroupCallMessage(value) => {
        match &self.listener.update_new_group_call_message {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateNewGroupCallPaidReaction(value) => {
        match &self.listener.update_new_group_call_paid_reaction {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateNewGuestQuery(value) => match &self.listener.update_new_guest_query {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateNewInlineCallbackQuery(value) => {
        match &self.listener.update_new_inline_callback_query {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateNewInlineQuery(value) => match &self.listener.update_new_inline_query {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateNewMessage(value) => match &self.listener.update_new_message {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateNewOauthRequest(value) => match &self.listener.update_new_oauth_request {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateNewPreCheckoutQuery(value) => {
        match &self.listener.update_new_pre_checkout_query {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateNewShippingQuery(value) => match &self.listener.update_new_shipping_query {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateNotification(value) => match &self.listener.update_notification {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateNotificationGroup(value) => match &self.listener.update_notification_group {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateOption(value) => match &self.listener.update_option {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateOwnedGramCount(value) => match &self.listener.update_owned_gram_count {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateOwnedStarCount(value) => match &self.listener.update_owned_star_count {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdatePaidMediaPurchased(value) => match &self.listener.update_paid_media_purchased {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdatePendingMessage(value) => match &self.listener.update_pending_message {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdatePoll(value) => match &self.listener.update_poll {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdatePollAnswer(value) => match &self.listener.update_poll_answer {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateProfileAccentColors(value) => match &self.listener.update_profile_accent_colors
      {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateQuickReplyShortcut(value) => match &self.listener.update_quick_reply_shortcut {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateQuickReplyShortcutDeleted(value) => {
        match &self.listener.update_quick_reply_shortcut_deleted {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateQuickReplyShortcutMessages(value) => {
        match &self.listener.update_quick_reply_shortcut_messages {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateQuickReplyShortcuts(value) => match &self.listener.update_quick_reply_shortcuts
      {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateReactionNotificationSettings(value) => {
        match &self.listener.update_reaction_notification_settings {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateRecentStickers(value) => match &self.listener.update_recent_stickers {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateSavedAnimations(value) => match &self.listener.update_saved_animations {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateSavedMessagesTags(value) => match &self.listener.update_saved_messages_tags {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateSavedMessagesTopic(value) => match &self.listener.update_saved_messages_topic {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateSavedMessagesTopicCount(value) => {
        match &self.listener.update_saved_messages_topic_count {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateSavedNotificationSounds(value) => {
        match &self.listener.update_saved_notification_sounds {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateScopeNotificationSettings(value) => {
        match &self.listener.update_scope_notification_settings {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateSecretChat(value) => match &self.listener.update_secret_chat {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateServiceNotification(value) => {
        match &self.listener.update_service_notification {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateSpeechRecognitionTrial(value) => {
        match &self.listener.update_speech_recognition_trial {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateSpeedLimitNotification(value) => {
        match &self.listener.update_speed_limit_notification {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateStakeDiceState(value) => match &self.listener.update_stake_dice_state {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateStarRevenueStatus(value) => match &self.listener.update_star_revenue_status {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateStickerSet(value) => match &self.listener.update_sticker_set {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateStopMessageDraft(value) => match &self.listener.update_stop_message_draft {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateStory(value) => match &self.listener.update_story {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateStoryDeleted(value) => match &self.listener.update_story_deleted {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateStoryListChatCount(value) => {
        match &self.listener.update_story_list_chat_count {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateStoryPostFailed(value) => match &self.listener.update_story_post_failed {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateStoryPostSucceeded(value) => match &self.listener.update_story_post_succeeded {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateStoryStealthMode(value) => match &self.listener.update_story_stealth_mode {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateSuggestedActions(value) => match &self.listener.update_suggested_actions {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateSupergroup(value) => match &self.listener.update_supergroup {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateSupergroupFullInfo(value) => match &self.listener.update_supergroup_full_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateTermsOfService(value) => match &self.listener.update_terms_of_service {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateTextCompositionStyles(value) => {
        match &self.listener.update_text_composition_styles {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateTonWalletGaslessTransfersInfo(value) => {
        match &self.listener.update_ton_wallet_gasless_transfers_info {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateTonWalletState(value) => match &self.listener.update_ton_wallet_state {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateTonWalletTonConnectSession(value) => {
        match &self.listener.update_ton_wallet_ton_connect_session {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateTonWalletTonConnectSessionDisconnectRequired(value) => match &self
        .listener
        .update_ton_wallet_ton_connect_session_disconnect_required
      {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateTopicMessageCount(value) => match &self.listener.update_topic_message_count {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateTrendingStickerSets(value) => match &self.listener.update_trending_sticker_sets
      {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateTrustedMiniAppBots(value) => {
        match &self.listener.update_trusted_mini_app_bots {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateUnconfirmedSession(value) => match &self.listener.update_unconfirmed_session {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateUnreadChatCount(value) => match &self.listener.update_unread_chat_count {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateUnreadMessageCount(value) => match &self.listener.update_unread_message_count {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateUser(value) => match &self.listener.update_user {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateUserFullInfo(value) => match &self.listener.update_user_full_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateUserPrivacySettingRules(value) => {
        match &self.listener.update_user_privacy_setting_rules {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateUserStatus(value) => match &self.listener.update_user_status {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateUserSubscription(value) => match &self.listener.update_user_subscription {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateVideoPublished(value) => match &self.listener.update_video_published {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateWebAppMessageSent(value) => match &self.listener.update_web_app_message_sent {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateWebBrowserSettings(value) => match &self.listener.update_web_browser_settings {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpdateWebProxyCarrierClosed(value) => {
        match &self.listener.update_web_proxy_carrier_closed {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateWebProxyCarrierData(value) => {
        match &self.listener.update_web_proxy_carrier_data {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpdateWebProxyCarrierOpened(value) => {
        match &self.listener.update_web_proxy_carrier_opened {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::AuthorizationState(value) => match &self.listener.authorization_state {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::CanPostStoryResult(value) => match &self.listener.can_post_story_result {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::CanSendGiftResult(value) => match &self.listener.can_send_gift_result {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::CanSendMessageToUserResult(value) => {
        match &self.listener.can_send_message_to_user_result {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::CanTransferOwnershipResult(value) => {
        match &self.listener.can_transfer_ownership_result {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::ChatJoinResult(value) => match &self.listener.chat_join_result {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatStatistics(value) => match &self.listener.chat_statistics {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::CheckChatUsernameResult(value) => match &self.listener.check_chat_username_result {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::CheckStickerSetNameResult(value) => {
        match &self.listener.check_sticker_set_name_result {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::CraftGiftResult(value) => match &self.listener.craft_gift_result {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::GiftResaleResult(value) => match &self.listener.gift_resale_result {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::GiveawayInfo(value) => match &self.listener.giveaway_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::InternalLinkType(value) => match &self.listener.internal_link_type {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::InviteGroupCallParticipantResult(value) => {
        match &self.listener.invite_group_call_participant_result {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::JsonValue(value) => match &self.listener.json_value {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::LanguagePackStringValue(value) => match &self.listener.language_pack_string_value {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::LogStream(value) => match &self.listener.log_stream {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::LoginUrlInfo(value) => match &self.listener.login_url_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::MessageFileType(value) => match &self.listener.message_file_type {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::MessageReadDate(value) => match &self.listener.message_read_date {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::MessageSender(value) => match &self.listener.message_sender {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::OptionValue(value) => match &self.listener.option_value {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::PassportElement(value) => match &self.listener.passport_element {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ReportChatResult(value) => match &self.listener.report_chat_result {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ReportSponsoredResult(value) => match &self.listener.report_sponsored_result {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ReportStoryResult(value) => match &self.listener.report_story_result {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ResetPasswordResult(value) => match &self.listener.reset_password_result {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::StartLiveStoryResult(value) => match &self.listener.start_live_story_result {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::StatisticalGraph(value) => match &self.listener.statistical_graph {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Update(value) => match &self.listener.update {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::WebBrowserType(value) => match &self.listener.web_browser_type {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::AccountTtl(value) => match &self.listener.account_ttl {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::AddedProxies(value) => match &self.listener.added_proxies {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::AddedProxy(value) => match &self.listener.added_proxy {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::AddedReactions(value) => match &self.listener.added_reactions {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::AnimatedEmoji(value) => match &self.listener.animated_emoji {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Animation(value) => match &self.listener.animation {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Animations(value) => match &self.listener.animations {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ArchiveChatListSettings(value) => match &self.listener.archive_chat_list_settings {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::AttachmentMenuBot(value) => match &self.listener.attachment_menu_bot {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Audios(value) => match &self.listener.audios {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::AuthenticationCodeInfo(value) => match &self.listener.authentication_code_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::AutoDownloadSettingsPresets(value) => {
        match &self.listener.auto_download_settings_presets {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::AutosaveSettings(value) => match &self.listener.autosave_settings {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::AvailableGifts(value) => match &self.listener.available_gifts {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::AvailableReactions(value) => match &self.listener.available_reactions {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Background(value) => match &self.listener.background {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Backgrounds(value) => match &self.listener.backgrounds {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::BankCardInfo(value) => match &self.listener.bank_card_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::BasicGroup(value) => match &self.listener.basic_group {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::BasicGroupFullInfo(value) => match &self.listener.basic_group_full_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::BotAccessSettings(value) => match &self.listener.bot_access_settings {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::BotCommands(value) => match &self.listener.bot_commands {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::BotMediaPreview(value) => match &self.listener.bot_media_preview {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::BotMediaPreviewInfo(value) => match &self.listener.bot_media_preview_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::BotMediaPreviews(value) => match &self.listener.bot_media_previews {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::BotMenuButton(value) => match &self.listener.bot_menu_button {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::BusinessChatLink(value) => match &self.listener.business_chat_link {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::BusinessChatLinkInfo(value) => match &self.listener.business_chat_link_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::BusinessChatLinks(value) => match &self.listener.business_chat_links {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::BusinessConnectedBotInfo(value) => match &self.listener.business_connected_bot_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::BusinessConnection(value) => match &self.listener.business_connection {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::BusinessFeatures(value) => match &self.listener.business_features {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::BusinessMessage(value) => match &self.listener.business_message {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::BusinessMessages(value) => match &self.listener.business_messages {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::CallId(value) => match &self.listener.call_id {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::CallbackQueryAnswer(value) => match &self.listener.callback_query_answer {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Chat(value) => match &self.listener.chat {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatActiveStories(value) => match &self.listener.chat_active_stories {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatAdministrators(value) => match &self.listener.chat_administrators {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatBoostFeatures(value) => match &self.listener.chat_boost_features {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatBoostLevelFeatures(value) => match &self.listener.chat_boost_level_features {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatBoostLink(value) => match &self.listener.chat_boost_link {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatBoostLinkInfo(value) => match &self.listener.chat_boost_link_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatBoostSlots(value) => match &self.listener.chat_boost_slots {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatBoostStatus(value) => match &self.listener.chat_boost_status {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatEvents(value) => match &self.listener.chat_events {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatFolder(value) => match &self.listener.chat_folder {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatFolderIcon(value) => match &self.listener.chat_folder_icon {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatFolderInfo(value) => match &self.listener.chat_folder_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatFolderInviteLink(value) => match &self.listener.chat_folder_invite_link {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatFolderInviteLinkInfo(value) => {
        match &self.listener.chat_folder_invite_link_info {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::ChatFolderInviteLinks(value) => match &self.listener.chat_folder_invite_links {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatInviteLink(value) => match &self.listener.chat_invite_link {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatInviteLinkCounts(value) => match &self.listener.chat_invite_link_counts {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatInviteLinkInfo(value) => match &self.listener.chat_invite_link_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatInviteLinkMembers(value) => match &self.listener.chat_invite_link_members {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatInviteLinks(value) => match &self.listener.chat_invite_links {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatJoinRequests(value) => match &self.listener.chat_join_requests {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatLists(value) => match &self.listener.chat_lists {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatMember(value) => match &self.listener.chat_member {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatMembers(value) => match &self.listener.chat_members {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatMessageSenders(value) => match &self.listener.chat_message_senders {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatPhotos(value) => match &self.listener.chat_photos {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatRevenueStatistics(value) => match &self.listener.chat_revenue_statistics {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ChatRevenueTransactions(value) => match &self.listener.chat_revenue_transactions {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Chats(value) => match &self.listener.chats {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::CollectibleItemInfo(value) => match &self.listener.collectible_item_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::CommunityFullInfo(value) => match &self.listener.community_full_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::CommunityId(value) => match &self.listener.community_id {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ConnectedAffiliateProgram(value) => {
        match &self.listener.connected_affiliate_program {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::ConnectedAffiliatePrograms(value) => {
        match &self.listener.connected_affiliate_programs {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::ConnectedWebsites(value) => match &self.listener.connected_websites {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Count(value) => match &self.listener.count {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Countries(value) => match &self.listener.countries {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::CountryInfo(value) => match &self.listener.country_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::CreatedBasicGroupChat(value) => match &self.listener.created_basic_group_chat {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Currencies(value) => match &self.listener.currencies {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::CurrencyExchangeRates(value) => match &self.listener.currency_exchange_rates {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::CurrentWeather(value) => match &self.listener.current_weather {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::CustomRequestResult(value) => match &self.listener.custom_request_result {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Data(value) => match &self.listener.data {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::DatabaseStatistics(value) => match &self.listener.database_statistics {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::DeepLinkInfo(value) => match &self.listener.deep_link_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::DirectMessagesChatTopic(value) => match &self.listener.direct_messages_chat_topic {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::EmailAddressAuthenticationCodeInfo(value) => {
        match &self.listener.email_address_authentication_code_info {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::EmojiCategories(value) => match &self.listener.emoji_categories {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::EmojiKeywords(value) => match &self.listener.emoji_keywords {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::EmojiReaction(value) => match &self.listener.emoji_reaction {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::EmojiStatusCustomEmojis(value) => match &self.listener.emoji_status_custom_emojis {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::EmojiStatuses(value) => match &self.listener.emoji_statuses {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Emojis(value) => match &self.listener.emojis {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Error(value) => match &self.listener.error {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::FailedToAddMembers(value) => match &self.listener.failed_to_add_members {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::File(value) => match &self.listener.file {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::FileDownloadedPrefixSize(value) => match &self.listener.file_downloaded_prefix_size {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::FixedText(value) => match &self.listener.fixed_text {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::FormattedText(value) => match &self.listener.formatted_text {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ForumTopic(value) => match &self.listener.forum_topic {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ForumTopicInfo(value) => match &self.listener.forum_topic_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ForumTopics(value) => match &self.listener.forum_topics {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::FoundAffiliatePrograms(value) => match &self.listener.found_affiliate_programs {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::FoundChatBoosts(value) => match &self.listener.found_chat_boosts {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::FoundChatMessages(value) => match &self.listener.found_chat_messages {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::FoundFileDownloads(value) => match &self.listener.found_file_downloads {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::FoundMessages(value) => match &self.listener.found_messages {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::FoundPosition(value) => match &self.listener.found_position {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::FoundPositions(value) => match &self.listener.found_positions {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::FoundPublicPosts(value) => match &self.listener.found_public_posts {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::FoundStories(value) => match &self.listener.found_stories {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::FoundUsers(value) => match &self.listener.found_users {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::FoundWebApp(value) => match &self.listener.found_web_app {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::GameHighScores(value) => match &self.listener.game_high_scores {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::GiftAuctionAcquiredGifts(value) => match &self.listener.gift_auction_acquired_gifts {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::GiftAuctionState(value) => match &self.listener.gift_auction_state {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::GiftChatThemes(value) => match &self.listener.gift_chat_themes {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::GiftCollection(value) => match &self.listener.gift_collection {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::GiftCollections(value) => match &self.listener.gift_collections {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::GiftUpgradePreview(value) => match &self.listener.gift_upgrade_preview {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::GiftUpgradeVariants(value) => match &self.listener.gift_upgrade_variants {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::GiftsForCrafting(value) => match &self.listener.gifts_for_crafting {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::GiftsForResale(value) => match &self.listener.gifts_for_resale {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::GramRevenueStatistics(value) => match &self.listener.gram_revenue_statistics {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::GroupCall(value) => match &self.listener.group_call {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::GroupCallId(value) => match &self.listener.group_call_id {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::GroupCallInfo(value) => match &self.listener.group_call_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::GroupCallParticipant(value) => match &self.listener.group_call_participant {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::GroupCallParticipants(value) => match &self.listener.group_call_participants {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::GroupCallStreams(value) => match &self.listener.group_call_streams {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Hashtags(value) => match &self.listener.hashtags {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::HttpUrl(value) => match &self.listener.http_url {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ImportedContacts(value) => match &self.listener.imported_contacts {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::InlineMessageId(value) => match &self.listener.inline_message_id {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::InlineQueryResults(value) => match &self.listener.inline_query_results {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::KeyboardButton(value) => match &self.listener.keyboard_button {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::LanguagePackInfo(value) => match &self.listener.language_pack_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::LanguagePackStrings(value) => match &self.listener.language_pack_strings {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::LinkPreview(value) => match &self.listener.link_preview {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::LiveStoryDonors(value) => match &self.listener.live_story_donors {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::LocalizationTargetInfo(value) => match &self.listener.localization_target_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::LogTags(value) => match &self.listener.log_tags {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::LogVerbosityLevel(value) => match &self.listener.log_verbosity_level {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::MainWebApp(value) => match &self.listener.main_web_app {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Message(value) => match &self.listener.message {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::MessageAutoDeleteTime(value) => match &self.listener.message_auto_delete_time {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::MessageCalendar(value) => match &self.listener.message_calendar {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::MessageEffect(value) => match &self.listener.message_effect {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::MessageLink(value) => match &self.listener.message_link {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::MessageLinkInfo(value) => match &self.listener.message_link_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::MessagePositions(value) => match &self.listener.message_positions {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::MessageProperties(value) => match &self.listener.message_properties {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::MessageSenders(value) => match &self.listener.message_senders {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::MessageStatistics(value) => match &self.listener.message_statistics {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::MessageThreadInfo(value) => match &self.listener.message_thread_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::MessageViewers(value) => match &self.listener.message_viewers {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Messages(value) => match &self.listener.messages {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::NetworkStatistics(value) => match &self.listener.network_statistics {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::NewChatPrivacySettings(value) => match &self.listener.new_chat_privacy_settings {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::NotificationSound(value) => match &self.listener.notification_sound {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::NotificationSounds(value) => match &self.listener.notification_sounds {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::OauthLinkInfo(value) => match &self.listener.oauth_link_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Ok(value) => match &self.listener.ok {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::OnRampPaymentAvailability(value) => match &self.listener.on_ramp_payment_availability
      {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::OnRampPaymentLimits(value) => match &self.listener.on_ramp_payment_limits {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::OnRampPaymentQuote(value) => match &self.listener.on_ramp_payment_quote {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::OnRampPaymentSession(value) => match &self.listener.on_ramp_payment_session {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::OnRampProviders(value) => match &self.listener.on_ramp_providers {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::OrderInfo(value) => match &self.listener.order_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Outline(value) => match &self.listener.outline {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Passkey(value) => match &self.listener.passkey {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Passkeys(value) => match &self.listener.passkeys {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::PassportAuthorizationForm(value) => {
        match &self.listener.passport_authorization_form {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::PassportElements(value) => match &self.listener.passport_elements {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::PassportElementsWithErrors(value) => {
        match &self.listener.passport_elements_with_errors {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::PasswordState(value) => match &self.listener.password_state {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::PaymentForm(value) => match &self.listener.payment_form {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::PaymentReceipt(value) => match &self.listener.payment_receipt {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::PaymentResult(value) => match &self.listener.payment_result {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::PhoneNumberInfo(value) => match &self.listener.phone_number_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::PollOptionProperties(value) => match &self.listener.poll_option_properties {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::PollVoteStatistics(value) => match &self.listener.poll_vote_statistics {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::PollVoters(value) => match &self.listener.poll_voters {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::PremiumFeatures(value) => match &self.listener.premium_features {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::PremiumGiftCodeInfo(value) => match &self.listener.premium_gift_code_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::PremiumGiftPaymentOptions(value) => match &self.listener.premium_gift_payment_options
      {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::PremiumGiveawayPaymentOptions(value) => {
        match &self.listener.premium_giveaway_payment_options {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::PremiumLimit(value) => match &self.listener.premium_limit {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::PremiumState(value) => match &self.listener.premium_state {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::PreparedInlineMessage(value) => match &self.listener.prepared_inline_message {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::PreparedInlineMessageId(value) => match &self.listener.prepared_inline_message_id {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ProfileShareThemeBackdrops(value) => {
        match &self.listener.profile_share_theme_backdrops {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::PublicForwards(value) => match &self.listener.public_forwards {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::PublicPostSearchLimits(value) => match &self.listener.public_post_search_limits {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::PushReceiverId(value) => match &self.listener.push_receiver_id {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::QuickReplyMessage(value) => match &self.listener.quick_reply_message {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::QuickReplyMessages(value) => match &self.listener.quick_reply_messages {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ReadDatePrivacySettings(value) => match &self.listener.read_date_privacy_settings {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ReceivedGift(value) => match &self.listener.received_gift {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ReceivedGifts(value) => match &self.listener.received_gifts {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::RecommendedChatFolders(value) => match &self.listener.recommended_chat_folders {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::RecoveryEmailAddress(value) => match &self.listener.recovery_email_address {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::RichMessage(value) => match &self.listener.rich_message {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::RtmpUrl(value) => match &self.listener.rtmp_url {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::SavedMessagesTags(value) => match &self.listener.saved_messages_tags {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ScopeNotificationSettings(value) => {
        match &self.listener.scope_notification_settings {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::Seconds(value) => match &self.listener.seconds {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::SecretChat(value) => match &self.listener.secret_chat {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Session(value) => match &self.listener.session {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Sessions(value) => match &self.listener.sessions {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::SponsoredChats(value) => match &self.listener.sponsored_chats {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::SponsoredMessages(value) => match &self.listener.sponsored_messages {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::StakeDiceState(value) => match &self.listener.stake_dice_state {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::StarAmount(value) => match &self.listener.star_amount {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::StarCount(value) => match &self.listener.star_count {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::StarGiveawayPaymentOptions(value) => {
        match &self.listener.star_giveaway_payment_options {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::StarPaymentOptions(value) => match &self.listener.star_payment_options {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::StarRevenueStatistics(value) => match &self.listener.star_revenue_statistics {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::StarSubscriptions(value) => match &self.listener.star_subscriptions {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::StarTransactions(value) => match &self.listener.star_transactions {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Sticker(value) => match &self.listener.sticker {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::StickerSet(value) => match &self.listener.sticker_set {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::StickerSets(value) => match &self.listener.sticker_sets {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Stickers(value) => match &self.listener.stickers {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::StorageStatistics(value) => match &self.listener.storage_statistics {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::StorageStatisticsFast(value) => match &self.listener.storage_statistics_fast {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Stories(value) => match &self.listener.stories {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Story(value) => match &self.listener.story {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::StoryAlbum(value) => match &self.listener.story_album {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::StoryAlbums(value) => match &self.listener.story_albums {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::StoryInteractions(value) => match &self.listener.story_interactions {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::StoryStatistics(value) => match &self.listener.story_statistics {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Supergroup(value) => match &self.listener.supergroup {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::SupergroupFullInfo(value) => match &self.listener.supergroup_full_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TMeUrls(value) => match &self.listener.t_me_urls {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TemporaryPasswordState(value) => match &self.listener.temporary_password_state {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TestBytes(value) => match &self.listener.test_bytes {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TestInt(value) => match &self.listener.test_int {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TestString(value) => match &self.listener.test_string {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TestVectorInt(value) => match &self.listener.test_vector_int {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TestVectorIntObject(value) => match &self.listener.test_vector_int_object {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TestVectorString(value) => match &self.listener.test_vector_string {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TestVectorStringObject(value) => match &self.listener.test_vector_string_object {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Text(value) => match &self.listener.text {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TextCompositionStyle(value) => match &self.listener.text_composition_style {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TextCompositionStyleExample(value) => {
        match &self.listener.text_composition_style_example {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::TextEntities(value) => match &self.listener.text_entities {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TimeZones(value) => match &self.listener.time_zones {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TonCenterStreamingApiUrl(value) => {
        match &self.listener.ton_center_streaming_api_url {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::TonConnectChallenge(value) => match &self.listener.ton_connect_challenge {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TonConnectRequests(value) => match &self.listener.ton_connect_requests {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TonConnectSession(value) => match &self.listener.ton_connect_session {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TonConnectSessionEventId(value) => {
        match &self.listener.ton_connect_session_event_id {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::TonConnectSessions(value) => match &self.listener.ton_connect_sessions {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TonNfts(value) => match &self.listener.ton_nfts {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TonTransactions(value) => match &self.listener.ton_transactions {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TonWalletCard(value) => match &self.listener.ton_wallet_card {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TonWalletOwnershipProofChallenge(value) => {
        match &self.listener.ton_wallet_ownership_proof_challenge {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::TonWalletSecretPhrase(value) => match &self.listener.ton_wallet_secret_phrase {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TonWalletTransaction(value) => match &self.listener.ton_wallet_transaction {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TonWalletTransactions(value) => match &self.listener.ton_wallet_transactions {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TonWalletTransferResult(value) => match &self.listener.ton_wallet_transfer_result {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::TrendingStickerSets(value) => match &self.listener.trending_sticker_sets {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Updates(value) => match &self.listener.updates {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpgradeGiftResult(value) => match &self.listener.upgrade_gift_result {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpgradedGift(value) => match &self.listener.upgraded_gift {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpgradedGiftCollectionNames(value) => {
        match &self.listener.upgraded_gift_collection_names {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::UpgradedGiftDepositInfo(value) => match &self.listener.upgraded_gift_deposit_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UpgradedGiftValueInfo(value) => match &self.listener.upgraded_gift_value_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::User(value) => match &self.listener.user {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UserFullInfo(value) => match &self.listener.user_full_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UserLink(value) => match &self.listener.user_link {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UserPrivacySettingRules(value) => match &self.listener.user_privacy_setting_rules {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UserSupportInfo(value) => match &self.listener.user_support_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UserTonWalletAddress(value) => match &self.listener.user_ton_wallet_address {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::UserTonWalletAddresses(value) => match &self.listener.user_ton_wallet_addresses {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::Users(value) => match &self.listener.users {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::ValidatedOrderInfo(value) => match &self.listener.validated_order_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::VideoMessageAdvertisements(value) => {
        match &self.listener.video_message_advertisements {
          None => Ok(false),
          Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
        }
      }

      TdType::WalletBotBalance(value) => match &self.listener.wallet_bot_balance {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::WebAppInfo(value) => match &self.listener.web_app_info {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::WebAppUrl(value) => match &self.listener.web_app_url {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },

      TdType::WebPageInstantView(value) => match &self.listener.web_page_instant_view {
        None => Ok(false),
        Some(f) => f((api.clone(), value.clone())).await.map(|_| true),
      },
    }
  }

  /// when telegram client throw exception
  pub fn exception(
    &self,
  ) -> &Option<Arc<dyn Send + Sync + Fn((Api, TGError)) -> LocalBoxFuture<'static, ()> + 'static>>
  {
    &self.listener.exception
  }

  /// when receive data from tdlib
  pub fn receive(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, String)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.receive
  }

  /// Does nothing and ensures that the Update object is used; for testing only. This is an offline method. Can be called before authorization
  pub fn test_use_update(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, TestUseUpdate)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.test_use_update
  }

  /// The list of supported accent colors has changed
  pub fn update_accent_colors(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateAccentColors)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_accent_colors
  }

  /// The list of active emoji reactions has changed
  pub fn update_active_emoji_reactions(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateActiveEmojiReactions)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_active_emoji_reactions
  }

  /// The list of auctions in which the current user participates has changed
  pub fn update_active_gift_auctions(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateActiveGiftAuctions)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_active_gift_auctions
  }

  /// The list of messages with active live location that need to be updated by the application has changed. The list is persistent across application restarts only if the message database is used
  pub fn update_active_live_location_messages(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateActiveLiveLocationMessages)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_active_live_location_messages
  }

  /// Contains active notifications that were shown on previous application launches. This update is sent only if the message database is used. In that case it comes once before any updateNotification and updateNotificationGroup update
  pub fn update_active_notifications(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateActiveNotifications)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_active_notifications
  }

  /// The parameters for age verification of the current user's account have changed
  pub fn update_age_verification_parameters(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateAgeVerificationParameters)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_age_verification_parameters
  }

  /// Lists of suffixes allowed in secondary bot usernames, which can be added additionally to the main username if the bot's owner has a Telegram Premium subscription
  pub fn update_allowed_bot_username_premium_suffixes(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateAllowedBotUsernamePremiumSuffixes)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_allowed_bot_username_premium_suffixes
  }

  /// Some animated emoji message was clicked and a big animated sticker must be played if the message is visible on the screen. chatActionWatchingAnimations with the text of the message needs to be sent if the sticker is played
  pub fn update_animated_emoji_message_clicked(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateAnimatedEmojiMessageClicked)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_animated_emoji_message_clicked
  }

  /// The parameters of animation search through getOption("animation_search_bot_username") bot have changed
  pub fn update_animation_search_parameters(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateAnimationSearchParameters)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_animation_search_parameters
  }

  /// A request can't be completed unless reCAPTCHA verification is performed; for official mobile applications only. The method setApplicationVerificationToken must be called once the verification is completed or failed
  pub fn update_application_recaptcha_verification_required(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn(
          (Api, UpdateApplicationRecaptchaVerificationRequired),
        ) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self
      .listener
      .update_application_recaptcha_verification_required
  }

  /// A request can't be completed unless application verification is performed; for official mobile applications only. The method setApplicationVerificationToken must be called once the verification is completed or failed
  pub fn update_application_verification_required(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateApplicationVerificationRequired)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_application_verification_required
  }

  /// The list of bots added to attachment or side menu has changed
  pub fn update_attachment_menu_bots(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateAttachmentMenuBots)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_attachment_menu_bots
  }

  /// The user authorization state has changed
  pub fn update_authorization_state(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateAuthorizationState)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_authorization_state
  }

  /// Autosave settings for some type of chats were updated
  pub fn update_autosave_settings(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateAutosaveSettings)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_autosave_settings
  }

  /// The list of available message effects has changed
  pub fn update_available_message_effects(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateAvailableMessageEffects)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_available_message_effects
  }

  /// Some data of a basic group has changed. This update is guaranteed to come before the basic group identifier is returned to the application
  pub fn update_basic_group(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateBasicGroup)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_basic_group
  }

  /// Some data in basicGroupFullInfo has been changed
  pub fn update_basic_group_full_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateBasicGroupFullInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_basic_group_full_info
  }

  /// A business connection has changed; for bots only
  pub fn update_business_connection(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateBusinessConnection)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_business_connection
  }

  /// A message in a business account was edited; for bots only
  pub fn update_business_message_edited(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateBusinessMessageEdited)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_business_message_edited
  }

  /// Messages in a business account were deleted; for bots only
  pub fn update_business_messages_deleted(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateBusinessMessagesDeleted)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_business_messages_deleted
  }

  /// New call was created or information about a call was updated
  pub fn update_call(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateCall)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.update_call
  }

  /// Chat accent colors have changed
  pub fn update_chat_accent_colors(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatAccentColors)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_accent_colors
  }

  /// A message sender activity in the chat has changed
  pub fn update_chat_action(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatAction)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_action
  }

  /// The chat action bar was changed
  pub fn update_chat_action_bar(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatActionBar)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_action_bar
  }

  /// The list of active stories posted by a specific chat has changed
  pub fn update_chat_active_stories(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatActiveStories)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_active_stories
  }

  /// A chat was added to a chat list
  pub fn update_chat_added_to_list(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatAddedToList)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_added_to_list
  }

  /// The chat available reactions were changed
  pub fn update_chat_available_reactions(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatAvailableReactions)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_available_reactions
  }

  /// The chat background was changed
  pub fn update_chat_background(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatBackground)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_background
  }

  /// A chat was blocked or unblocked
  pub fn update_chat_block_list(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatBlockList)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_block_list
  }

  /// A chat boost has changed; for bots only
  pub fn update_chat_boost(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatBoost)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_boost
  }

  /// The bar for managing business bot was changed in a chat
  pub fn update_chat_business_bot_manage_bar(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatBusinessBotManageBar)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_business_bot_manage_bar
  }

  /// The value of the default disable_notification parameter, used when a message is sent to the chat, was changed
  pub fn update_chat_default_disable_notification(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatDefaultDisableNotification)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_default_disable_notification
  }

  /// A chat draft has changed. Be aware that the update may come in the currently opened chat but with old content of the draft. If the user has changed the content of the draft, this update mustn't be applied
  pub fn update_chat_draft_message(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatDraftMessage)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_draft_message
  }

  /// Chat emoji status has changed
  pub fn update_chat_emoji_status(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatEmojiStatus)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_emoji_status
  }

  /// The list of chat folders or a chat folder has changed
  pub fn update_chat_folders(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatFolders)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_folders
  }

  /// A chat content was allowed or restricted for saving
  pub fn update_chat_has_protected_content(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatHasProtectedContent)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_has_protected_content
  }

  /// A chat's has_scheduled_messages field has changed
  pub fn update_chat_has_scheduled_messages(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatHasScheduledMessages)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_has_scheduled_messages
  }

  /// A chat's has_welcome_messages field has changed
  pub fn update_chat_has_welcome_messages(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatHasWelcomeMessages)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_has_welcome_messages
  }

  /// A chat was marked as unread or was read
  pub fn update_chat_is_marked_as_unread(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatIsMarkedAsUnread)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_is_marked_as_unread
  }

  /// Translation of chat messages was enabled or disabled
  pub fn update_chat_is_translatable(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatIsTranslatable)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_is_translatable
  }

  /// A join request from the user was completed
  pub fn update_chat_join_result(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatJoinResult)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_join_result
  }

  /// The last message of a chat was changed
  pub fn update_chat_last_message(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatLastMessage)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_last_message
  }

  /// User rights changed in a chat; for bots only
  pub fn update_chat_member(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatMember)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_member
  }

  /// The message auto-delete or self-destruct timer setting for a chat was changed
  pub fn update_chat_message_auto_delete_time(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatMessageAutoDeleteTime)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_message_auto_delete_time
  }

  /// The message sender that is selected to send messages in a chat has changed
  pub fn update_chat_message_sender(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatMessageSender)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_message_sender
  }

  /// Notification settings for a chat were changed
  pub fn update_chat_notification_settings(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatNotificationSettings)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_notification_settings
  }

  /// The number of online group members has changed. This update with non-zero number of online group members is sent only for currently opened chats. There is no guarantee that it is sent just after the number of online users has changed
  pub fn update_chat_online_member_count(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatOnlineMemberCount)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_online_member_count
  }

  /// The chat pending join requests were changed
  pub fn update_chat_pending_join_requests(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatPendingJoinRequests)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_pending_join_requests
  }

  /// Chat permissions were changed
  pub fn update_chat_permissions(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatPermissions)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_permissions
  }

  /// A chat photo was changed
  pub fn update_chat_photo(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatPhoto)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_photo
  }

  /// The position of a chat in a chat list has changed. An updateChatLastMessage or updateChatDraftMessage update might be sent instead of the update
  pub fn update_chat_position(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatPosition)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_position
  }

  /// Incoming messages were read or the number of unread messages has been changed
  pub fn update_chat_read_inbox(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatReadInbox)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_read_inbox
  }

  /// Outgoing messages were read
  pub fn update_chat_read_outbox(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatReadOutbox)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_read_outbox
  }

  /// A chat was removed from a chat list
  pub fn update_chat_removed_from_list(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatRemovedFromList)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_removed_from_list
  }

  /// The chat reply markup was changed
  pub fn update_chat_reply_markup(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatReplyMarkup)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_reply_markup
  }

  /// The revenue earned from sponsored messages in a chat has changed. If chat revenue screen is opened, then getChatRevenueTransactions may be called to fetch new transactions
  pub fn update_chat_revenue_amount(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatRevenueAmount)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_revenue_amount
  }

  /// The chat theme was changed
  pub fn update_chat_theme(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatTheme)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_theme
  }

  /// The title of a chat was changed
  pub fn update_chat_title(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatTitle)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_title
  }

  /// The chat unread_mention_count has changed
  pub fn update_chat_unread_mention_count(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatUnreadMentionCount)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_unread_mention_count
  }

  /// The chat unread_poll_vote_count has changed
  pub fn update_chat_unread_poll_vote_count(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatUnreadPollVoteCount)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_unread_poll_vote_count
  }

  /// The chat unread_reaction_count has changed
  pub fn update_chat_unread_reaction_count(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatUnreadReactionCount)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_unread_reaction_count
  }

  /// A chat video chat state has changed
  pub fn update_chat_video_chat(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatVideoChat)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_video_chat
  }

  /// A chat default appearance has changed
  pub fn update_chat_view_as_topics(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatViewAsTopics)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_view_as_topics
  }

  /// The list of welcome messages of a chat has changed
  pub fn update_chat_welcome_messages(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateChatWelcomeMessages)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_chat_welcome_messages
  }

  /// Some data of a community has changed. This update is guaranteed to come before the community identifier is returned to the application
  pub fn update_community(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateCommunity)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_community
  }

  /// Some data in communityFullInfo has been changed
  pub fn update_community_full_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateCommunityFullInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_community_full_info
  }

  /// The connection state has changed. This update must be used only to show a human-readable description of the connection state
  pub fn update_connection_state(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateConnectionState)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_connection_state
  }

  /// The list of contacts that had birthdays recently or will have birthday soon has changed
  pub fn update_contact_close_birthdays(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateContactCloseBirthdays)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_contact_close_birthdays
  }

  /// The default background has changed
  pub fn update_default_background(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateDefaultBackground)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_default_background
  }

  /// The type of default paid reaction has changed
  pub fn update_default_paid_reaction_type(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateDefaultPaidReactionType)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_default_paid_reaction_type
  }

  /// The type of default reaction has changed
  pub fn update_default_reaction_type(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateDefaultReactionType)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_default_reaction_type
  }

  /// Some messages were deleted
  pub fn update_delete_messages(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateDeleteMessages)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_delete_messages
  }

  /// The list of supported dice emojis has changed
  pub fn update_dice_emojis(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateDiceEmojis)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_dice_emojis
  }

  /// Basic information about a topic in a channel direct messages chat administered by the current user has changed. This update is guaranteed to come before the topic identifier is returned to the application
  pub fn update_direct_messages_chat_topic(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateDirectMessagesChatTopic)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_direct_messages_chat_topic
  }

  /// The list of available emoji chat themes has changed
  pub fn update_emoji_chat_themes(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateEmojiChatThemes)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_emoji_chat_themes
  }

  /// The list of favorite stickers was updated
  pub fn update_favorite_stickers(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateFavoriteStickers)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_favorite_stickers
  }

  /// Information about a file was updated
  pub fn update_file(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateFile)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.update_file
  }

  /// A file was added to the file download list. This update is sent only after file download list is loaded for the first time
  pub fn update_file_added_to_downloads(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateFileAddedToDownloads)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_file_added_to_downloads
  }

  /// A file download was changed. This update is sent only after file download list is loaded for the first time
  pub fn update_file_download(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateFileDownload)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_file_download
  }

  /// The state of the file download list has changed
  pub fn update_file_downloads(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateFileDownloads)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_file_downloads
  }

  /// The file generation process needs to be started by the application. Use setFileGenerationProgress and finishFileGeneration to generate the file
  pub fn update_file_generation_start(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateFileGenerationStart)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_file_generation_start
  }

  /// File generation is no longer needed
  pub fn update_file_generation_stop(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateFileGenerationStop)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_file_generation_stop
  }

  /// A file was removed from the file download list. This update is sent only after file download list is loaded for the first time
  pub fn update_file_removed_from_downloads(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateFileRemovedFromDownloads)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_file_removed_from_downloads
  }

  /// Information about a topic in a forum chat was changed
  pub fn update_forum_topic(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateForumTopic)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_forum_topic
  }

  /// Basic information about a topic in a forum chat was changed
  pub fn update_forum_topic_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateForumTopicInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_forum_topic_info
  }

  /// The freeze state of the current user's account has changed
  pub fn update_freeze_state(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateFreezeState)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_freeze_state
  }

  /// State of a gift auction was updated
  pub fn update_gift_auction_state(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateGiftAuctionState)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_gift_auction_state
  }

  /// The TON Gram revenue earned by the current user has changed. If Gram transaction screen of the chat is opened, then getTonTransactions may be called to fetch new transactions
  pub fn update_gram_revenue_status(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateGramRevenueStatus)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_gram_revenue_status
  }

  /// Information about a group call was updated
  pub fn update_group_call(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateGroupCall)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_group_call
  }

  /// The levels of live story group call messages have changed
  pub fn update_group_call_message_levels(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateGroupCallMessageLevels)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_group_call_message_levels
  }

  /// A group call message failed to send
  pub fn update_group_call_message_send_failed(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateGroupCallMessageSendFailed)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_group_call_message_send_failed
  }

  /// Some group call messages were deleted
  pub fn update_group_call_messages_deleted(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateGroupCallMessagesDeleted)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_group_call_messages_deleted
  }

  /// Information about a group call participant was changed. The updates are sent only after the group call is received through getGroupCall and only if the call is joined or being joined
  pub fn update_group_call_participant(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateGroupCallParticipant)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_group_call_participant
  }

  /// The list of group call participants that can send and receive encrypted call data has changed; for group calls not bound to a chat only
  pub fn update_group_call_participants(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateGroupCallParticipants)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_group_call_participants
  }

  /// The verification state of an encrypted group call has changed; for group calls not bound to a chat only
  pub fn update_group_call_verification_state(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateGroupCallVerificationState)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_group_call_verification_state
  }

  /// Describes whether there are some pending notification updates. Can be used to prevent application from killing, while there are some pending notifications
  pub fn update_have_pending_notifications(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateHavePendingNotifications)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_have_pending_notifications
  }

  /// The list of installed sticker sets was updated
  pub fn update_installed_sticker_sets(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateInstalledStickerSets)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_installed_sticker_sets
  }

  /// Some language pack strings have been updated
  pub fn update_language_pack_strings(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateLanguagePackStrings)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_language_pack_strings
  }

  /// The list of top donors in live story group call has changed
  pub fn update_live_story_top_donors(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateLiveStoryTopDonors)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_live_story_top_donors
  }

  /// A bot that can be managed by the current bot was created or updated; for bots only
  pub fn update_managed_bot(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateManagedBot)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_managed_bot
  }

  /// Unread votes were added or removed from a poll message
  pub fn update_message_contains_unread_poll_votes(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateMessageContainsUnreadPollVotes)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_message_contains_unread_poll_votes
  }

  /// The message content has changed
  pub fn update_message_content(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateMessageContent)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_message_content
  }

  /// The message content was opened. Updates voice note messages to "listened", video note messages to "viewed" and starts the self-destruct timer
  pub fn update_message_content_opened(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateMessageContentOpened)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_message_content_opened
  }

  /// A message was edited. Changes in the message content will come in a separate updateMessageContent
  pub fn update_message_edited(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateMessageEdited)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_message_edited
  }

  /// The message ephemeral content has changed
  pub fn update_message_ephemeral_content(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateMessageEphemeralContent)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_message_ephemeral_content
  }

  /// A fact-check added to a message was changed
  pub fn update_message_fact_check(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateMessageFactCheck)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_message_fact_check
  }

  /// The information about interactions with a message has changed
  pub fn update_message_interaction_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateMessageInteractionInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_message_interaction_info
  }

  /// The message pinned state was changed
  pub fn update_message_is_pinned(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateMessageIsPinned)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_message_is_pinned
  }

  /// A message with a live location was viewed. When the update is received, the application is expected to update the live location
  pub fn update_message_live_location_viewed(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateMessageLiveLocationViewed)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_message_live_location_viewed
  }

  /// A message with an unread mention was read
  pub fn update_message_mention_read(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateMessageMentionRead)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_message_mention_read
  }

  /// User changed its reactions on a message with public reactions; for bots only
  pub fn update_message_reaction(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateMessageReaction)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_message_reaction
  }

  /// Reactions added to a message with anonymous reactions have changed; for bots only
  pub fn update_message_reactions(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateMessageReactions)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_message_reactions
  }

  /// A request to send a message has reached the Telegram server. This doesn't mean that the message will be sent successfully. This update is sent only if the option "use_quick_ack" is set to true. This update may be sent multiple times for the same message
  pub fn update_message_send_acknowledged(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateMessageSendAcknowledged)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_message_send_acknowledged
  }

  /// A message failed to send. Be aware that some messages being sent can be irrecoverably deleted, in which case updateDeleteMessages will be received instead of this update
  pub fn update_message_send_failed(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateMessageSendFailed)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_message_send_failed
  }

  /// A message has been successfully sent
  pub fn update_message_send_succeeded(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateMessageSendSucceeded)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_message_send_succeeded
  }

  /// Information about suggested post of a message was changed
  pub fn update_message_suggested_post_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateMessageSuggestedPostInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_message_suggested_post_info
  }

  /// The list of unread reactions added to a message was changed
  pub fn update_message_unread_reactions(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateMessageUnreadReactions)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_message_unread_reactions
  }

  /// A new incoming callback query from a business message; for bots only
  pub fn update_new_business_callback_query(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateNewBusinessCallbackQuery)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_new_business_callback_query
  }

  /// A new message was added to a business account; for bots only
  pub fn update_new_business_message(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateNewBusinessMessage)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_new_business_message
  }

  /// New call signaling data arrived
  pub fn update_new_call_signaling_data(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateNewCallSignalingData)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_new_call_signaling_data
  }

  /// A new incoming callback query; for bots only
  pub fn update_new_callback_query(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateNewCallbackQuery)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_new_callback_query
  }

  /// A new chat has been loaded/created. This update is guaranteed to come before the chat identifier is returned to the application. The chat field changes will be reported through separate updates
  pub fn update_new_chat(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateNewChat)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.update_new_chat
  }

  /// A user sent a join request to a chat; for bots only
  pub fn update_new_chat_join_request(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateNewChatJoinRequest)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_new_chat_join_request
  }

  /// The user has chosen a result of an inline query; for bots only
  pub fn update_new_chosen_inline_result(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateNewChosenInlineResult)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_new_chosen_inline_result
  }

  /// A new incoming event; for bots only
  pub fn update_new_custom_event(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateNewCustomEvent)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_new_custom_event
  }

  /// A new incoming query; for bots only
  pub fn update_new_custom_query(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateNewCustomQuery)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_new_custom_query
  }

  /// A new message was received in a group call
  pub fn update_new_group_call_message(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateNewGroupCallMessage)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_new_group_call_message
  }

  /// A new paid reaction was received in a live story group call
  pub fn update_new_group_call_paid_reaction(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateNewGroupCallPaidReaction)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_new_group_call_paid_reaction
  }

  /// A new incoming guest query; for bots only
  pub fn update_new_guest_query(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateNewGuestQuery)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_new_guest_query
  }

  /// A new incoming callback query from a message sent via a bot; for bots only
  pub fn update_new_inline_callback_query(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateNewInlineCallbackQuery)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_new_inline_callback_query
  }

  /// A new incoming inline query; for bots only
  pub fn update_new_inline_query(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateNewInlineQuery)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_new_inline_query
  }

  /// A new message was received; can also be an outgoing message
  pub fn update_new_message(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateNewMessage)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_new_message
  }

  /// An OAuth authorization request was received
  pub fn update_new_oauth_request(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateNewOauthRequest)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_new_oauth_request
  }

  /// A new incoming pre-checkout query; for bots only. Contains full information about a checkout
  pub fn update_new_pre_checkout_query(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateNewPreCheckoutQuery)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_new_pre_checkout_query
  }

  /// A new incoming shipping query; for bots only. Only for invoices with flexible price
  pub fn update_new_shipping_query(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateNewShippingQuery)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_new_shipping_query
  }

  /// A notification was changed
  pub fn update_notification(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateNotification)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_notification
  }

  /// A list of active notifications in a notification group has changed
  pub fn update_notification_group(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateNotificationGroup)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_notification_group
  }

  /// An option changed its value
  pub fn update_option(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateOption)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.update_option
  }

  /// The number of TON Grams owned by the current user has changed
  pub fn update_owned_gram_count(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateOwnedGramCount)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_owned_gram_count
  }

  /// The number of Telegram Stars owned by the current user has changed
  pub fn update_owned_star_count(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateOwnedStarCount)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_owned_star_count
  }

  /// Paid media were purchased by a user; for bots only
  pub fn update_paid_media_purchased(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdatePaidMediaPurchased)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_paid_media_purchased
  }

  /// A new pending text or rich message was received in a chat with a bot. The message must be shown in the chat for at most getOption("pending_text_message_period") seconds, replace any other pending message with the same draft_id with animation, and be deleted whenever any incoming message or a pending message with another draft_id is received in the message thread
  pub fn update_pending_message(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdatePendingMessage)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_pending_message
  }

  /// A poll was updated; for bots only
  pub fn update_poll(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, UpdatePoll)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.update_poll
  }

  /// A user changed the answer to a poll; for bots only
  pub fn update_poll_answer(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdatePollAnswer)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_poll_answer
  }

  /// The list of supported accent colors for user profiles has changed
  pub fn update_profile_accent_colors(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateProfileAccentColors)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_profile_accent_colors
  }

  /// Basic information about a quick reply shortcut has changed. This update is guaranteed to come before the quick shortcut name is returned to the application
  pub fn update_quick_reply_shortcut(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateQuickReplyShortcut)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_quick_reply_shortcut
  }

  /// A quick reply shortcut and all its messages were deleted
  pub fn update_quick_reply_shortcut_deleted(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateQuickReplyShortcutDeleted)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_quick_reply_shortcut_deleted
  }

  /// The list of quick reply shortcut messages has changed
  pub fn update_quick_reply_shortcut_messages(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateQuickReplyShortcutMessages)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_quick_reply_shortcut_messages
  }

  /// The list of quick reply shortcuts has changed
  pub fn update_quick_reply_shortcuts(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateQuickReplyShortcuts)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_quick_reply_shortcuts
  }

  /// Notification settings for reactions were updated
  pub fn update_reaction_notification_settings(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateReactionNotificationSettings)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_reaction_notification_settings
  }

  /// The list of recently used stickers was updated
  pub fn update_recent_stickers(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateRecentStickers)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_recent_stickers
  }

  /// The list of saved animations was updated
  pub fn update_saved_animations(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateSavedAnimations)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_saved_animations
  }

  /// Tags used in Saved Messages or a Saved Messages topic have changed
  pub fn update_saved_messages_tags(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateSavedMessagesTags)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_saved_messages_tags
  }

  /// Basic information about a Saved Messages topic has changed. This update is guaranteed to come before the topic identifier is returned to the application
  pub fn update_saved_messages_topic(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateSavedMessagesTopic)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_saved_messages_topic
  }

  /// Number of Saved Messages topics has changed
  pub fn update_saved_messages_topic_count(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateSavedMessagesTopicCount)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_saved_messages_topic_count
  }

  /// The list of saved notification sounds was updated. This update may not be sent until information about a notification sound was requested for the first time
  pub fn update_saved_notification_sounds(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateSavedNotificationSounds)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_saved_notification_sounds
  }

  /// Notification settings for some type of chats were updated
  pub fn update_scope_notification_settings(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateScopeNotificationSettings)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_scope_notification_settings
  }

  /// Some data of a secret chat has changed. This update is guaranteed to come before the secret chat identifier is returned to the application
  pub fn update_secret_chat(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateSecretChat)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_secret_chat
  }

  /// A service notification from the server was received. Upon receiving this the application must show a popup with the content of the notification
  pub fn update_service_notification(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateServiceNotification)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_service_notification
  }

  /// The parameters of speech recognition without Telegram Premium subscription have changed
  pub fn update_speech_recognition_trial(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateSpeechRecognitionTrial)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_speech_recognition_trial
  }

  /// Download or upload file speed for the user was limited, but it can be restored by subscription to Telegram Premium. The notification can be postponed until a file being downloaded or uploaded is visible to the user. Use getOption("premium_download_speedup") or getOption("premium_upload_speedup") to get expected speedup after subscription to Telegram Premium
  pub fn update_speed_limit_notification(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateSpeedLimitNotification)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_speed_limit_notification
  }

  /// The stake dice state has changed
  pub fn update_stake_dice_state(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateStakeDiceState)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_stake_dice_state
  }

  /// The Telegram Star revenue earned by a user or a chat has changed. If Telegram Star transaction screen of the chat is opened, then getStarTransactions may be called to fetch new transactions
  pub fn update_star_revenue_status(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateStarRevenueStatus)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_star_revenue_status
  }

  /// A sticker set has changed
  pub fn update_sticker_set(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateStickerSet)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_sticker_set
  }

  /// A message draft generation was stopped by the user
  pub fn update_stop_message_draft(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateStopMessageDraft)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_stop_message_draft
  }

  /// A story was changed
  pub fn update_story(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpdateStory)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.update_story
  }

  /// A story became inaccessible
  pub fn update_story_deleted(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateStoryDeleted)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_story_deleted
  }

  /// Number of chats in a story list has changed
  pub fn update_story_list_chat_count(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateStoryListChatCount)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_story_list_chat_count
  }

  /// A story failed to post. If the story posting is canceled, then updateStoryDeleted will be received instead of this update
  pub fn update_story_post_failed(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateStoryPostFailed)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_story_post_failed
  }

  /// A story has been successfully posted
  pub fn update_story_post_succeeded(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateStoryPostSucceeded)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_story_post_succeeded
  }

  /// Story stealth mode settings have changed
  pub fn update_story_stealth_mode(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateStoryStealthMode)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_story_stealth_mode
  }

  /// The list of suggested to the user actions has changed
  pub fn update_suggested_actions(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateSuggestedActions)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_suggested_actions
  }

  /// Some data of a supergroup or a channel has changed. This update is guaranteed to come before the supergroup identifier is returned to the application
  pub fn update_supergroup(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateSupergroup)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_supergroup
  }

  /// Some data in supergroupFullInfo has been changed
  pub fn update_supergroup_full_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateSupergroupFullInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_supergroup_full_info
  }

  /// New terms of service must be accepted by the user. If the terms of service are declined, then the deleteAccount method must be called with the reason "Decline ToS update"
  pub fn update_terms_of_service(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateTermsOfService)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_terms_of_service
  }

  /// The styles supported for text composition have changed
  pub fn update_text_composition_styles(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateTextCompositionStyles)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_text_composition_styles
  }

  /// The information about gasless transfers has changed
  pub fn update_ton_wallet_gasless_transfers_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateTonWalletGaslessTransfersInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_ton_wallet_gasless_transfers_info
  }

  /// The state of the TON wallet of the current user has changed
  pub fn update_ton_wallet_state(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateTonWalletState)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_ton_wallet_state
  }

  /// A session of a TON Connect has changed
  pub fn update_ton_wallet_ton_connect_session(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateTonWalletTonConnectSession)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_ton_wallet_ton_connect_session
  }

  /// TON Connect sessions must be disconnected in a regular way
  pub fn update_ton_wallet_ton_connect_session_disconnect_required(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn(
          (Api, UpdateTonWalletTonConnectSessionDisconnectRequired),
        ) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self
      .listener
      .update_ton_wallet_ton_connect_session_disconnect_required
  }

  /// Number of messages in a topic has changed; for Saved Messages and channel direct messages chat topics only
  pub fn update_topic_message_count(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateTopicMessageCount)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_topic_message_count
  }

  /// The list of trending sticker sets was updated or some of them were viewed
  pub fn update_trending_sticker_sets(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateTrendingStickerSets)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_trending_sticker_sets
  }

  /// Lists of bots which Mini Apps must be allowed to read text from clipboard and must be opened without a warning
  pub fn update_trusted_mini_app_bots(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateTrustedMiniAppBots)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_trusted_mini_app_bots
  }

  /// The first unconfirmed session has changed
  pub fn update_unconfirmed_session(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateUnconfirmedSession)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_unconfirmed_session
  }

  /// Number of unread chats, i.e. with unread messages or marked as unread, has changed. This update is sent only if the message database is used
  pub fn update_unread_chat_count(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateUnreadChatCount)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_unread_chat_count
  }

  /// Number of unread messages in a chat list has changed. This update is sent only if the message database is used
  pub fn update_unread_message_count(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateUnreadMessageCount)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_unread_message_count
  }

  /// Some data of a user has changed. This update is guaranteed to come before the user identifier is returned to the application
  pub fn update_user(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, UpdateUser)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.update_user
  }

  /// Some data in userFullInfo has been changed
  pub fn update_user_full_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateUserFullInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_user_full_info
  }

  /// Some privacy setting rules have been changed
  pub fn update_user_privacy_setting_rules(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateUserPrivacySettingRules)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_user_privacy_setting_rules
  }

  /// The user went online or offline
  pub fn update_user_status(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateUserStatus)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_user_status
  }

  /// Subscription of a user to the bot was changed; for bots only
  pub fn update_user_subscription(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateUserSubscription)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_user_subscription
  }

  /// An automatically scheduled message with video has been successfully sent after conversion
  pub fn update_video_published(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateVideoPublished)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_video_published
  }

  /// A message was sent by an opened Web App, so the Web App needs to be closed
  pub fn update_web_app_message_sent(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateWebAppMessageSent)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_web_app_message_sent
  }

  /// Web browser settings have been updated
  pub fn update_web_browser_settings(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateWebBrowserSettings)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_web_browser_settings
  }

  /// A web proxy carrier must be closed. If the page has passed any message to the bridge object, then the application must deliver the string {"t":"close"} to the page, so that it can end its relay session, and destroy the web view shortly after, for example, in 200 milliseconds; otherwise, the web view must be destroyed immediately. Messages from the carrier received after this update must be ignored
  pub fn update_web_proxy_carrier_closed(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateWebProxyCarrierClosed)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_web_proxy_carrier_closed
  }

  /// Data must be delivered as an ArrayBuffer to the page of a web proxy carrier through the onmessage function of the bridge object, in the order of the updates
  pub fn update_web_proxy_carrier_data(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateWebProxyCarrierData)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_web_proxy_carrier_data
  }

  /// A web proxy carrier must be opened as described in webProxyCarrier. Messages passed by the page to the bridge object must be passed to processWebProxyCarrierMessage. Failures of the carrier must be reported with reportWebProxyCarrierError
  pub fn update_web_proxy_carrier_opened(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpdateWebProxyCarrierOpened)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.update_web_proxy_carrier_opened
  }

  /// Represents the current authorization state of the TDLib client
  pub fn authorization_state(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, AuthorizationState)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.authorization_state
  }

  /// Represents result of checking whether the current user can post a story on behalf of the specific chat
  pub fn can_post_story_result(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, CanPostStoryResult)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.can_post_story_result
  }

  /// Describes whether a gift can be sent now by the current user
  pub fn can_send_gift_result(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, CanSendGiftResult)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.can_send_gift_result
  }

  /// Describes result of canSendMessageToUser
  pub fn can_send_message_to_user_result(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, CanSendMessageToUserResult)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.can_send_message_to_user_result
  }

  /// Represents result of checking whether the current session can be used to transfer a chat ownership to another user
  pub fn can_transfer_ownership_result(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, CanTransferOwnershipResult)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.can_transfer_ownership_result
  }

  /// Describes result of join of a chat by the current user
  pub fn chat_join_result(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ChatJoinResult)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.chat_join_result
  }

  /// Contains a detailed statistics about a chat
  pub fn chat_statistics(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ChatStatistics)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.chat_statistics
  }

  /// Represents result of checking whether a username can be set for a chat
  pub fn check_chat_username_result(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, CheckChatUsernameResult)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.check_chat_username_result
  }

  /// Represents result of checking whether a name can be used for a new sticker set
  pub fn check_sticker_set_name_result(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, CheckStickerSetNameResult)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.check_sticker_set_name_result
  }

  /// Contains result of gift crafting
  pub fn craft_gift_result(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, CraftGiftResult)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.craft_gift_result
  }

  /// Describes result of sending a resold gift
  pub fn gift_resale_result(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, GiftResaleResult)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.gift_resale_result
  }

  /// Contains information about a giveaway
  pub fn giveaway_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, GiveawayInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.giveaway_info
  }

  /// Describes an internal https://t.me or tg: link, which must be processed by the application in a special way
  pub fn internal_link_type(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, InternalLinkType)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.internal_link_type
  }

  /// Describes result of group call participant invitation
  pub fn invite_group_call_participant_result(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, InviteGroupCallParticipantResult)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.invite_group_call_participant_result
  }

  /// Represents a JSON value
  pub fn json_value(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, JsonValue)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.json_value
  }

  /// Represents the value of a string in a language pack
  pub fn language_pack_string_value(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, LanguagePackStringValue)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.language_pack_string_value
  }

  /// Describes a stream to which TDLib internal log is written
  pub fn log_stream(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, LogStream)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.log_stream
  }

  /// Contains information about an inline button of type inlineKeyboardButtonTypeLoginUrl or an external link
  pub fn login_url_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, LoginUrlInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.login_url_info
  }

  /// Contains information about a file with messages exported from another app
  pub fn message_file_type(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, MessageFileType)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.message_file_type
  }

  /// Describes read date of a recent outgoing message in a private chat
  pub fn message_read_date(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, MessageReadDate)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.message_read_date
  }

  /// Contains information about the sender of a message
  pub fn message_sender(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, MessageSender)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.message_sender
  }

  /// Represents the value of an option
  pub fn option_value(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, OptionValue)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.option_value
  }

  /// Contains information about a Telegram Passport element
  pub fn passport_element(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, PassportElement)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.passport_element
  }

  /// Describes result of chat report
  pub fn report_chat_result(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ReportChatResult)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.report_chat_result
  }

  /// Describes result of sponsored message or chat report
  pub fn report_sponsored_result(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ReportSponsoredResult)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.report_sponsored_result
  }

  /// Describes result of story report
  pub fn report_story_result(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ReportStoryResult)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.report_story_result
  }

  /// Represents result of 2-step verification password reset
  pub fn reset_password_result(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ResetPasswordResult)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.reset_password_result
  }

  /// Represents result of starting a live story
  pub fn start_live_story_result(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, StartLiveStoryResult)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.start_live_story_result
  }

  /// Describes a statistical graph
  pub fn statistical_graph(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, StatisticalGraph)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.statistical_graph
  }

  /// Contains notifications about data changes
  pub fn update(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Update)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.update
  }

  /// Describes the type of web browser
  pub fn web_browser_type(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, WebBrowserType)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.web_browser_type
  }

  /// Contains information about the period of inactivity after which the current user's account will automatically be deleted
  pub fn account_ttl(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, AccountTtl)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.account_ttl
  }

  /// Represents a list of added proxy servers
  pub fn added_proxies(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, AddedProxies)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.added_proxies
  }

  /// Contains information about a proxy server added to the list of proxies
  pub fn added_proxy(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, AddedProxy)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.added_proxy
  }

  /// Represents a list of reactions added to a message
  pub fn added_reactions(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, AddedReactions)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.added_reactions
  }

  /// Describes an animated or custom representation of an emoji
  pub fn animated_emoji(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, AnimatedEmoji)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.animated_emoji
  }

  /// Describes an animation file. The animation must be encoded in GIF or MPEG4 format
  pub fn animation(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Animation)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.animation
  }

  /// Represents a list of animations
  pub fn animations(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Animations)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.animations
  }

  /// Contains settings for automatic moving of chats to and from the Archive chat lists
  pub fn archive_chat_list_settings(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ArchiveChatListSettings)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.archive_chat_list_settings
  }

  /// Represents a bot, which can be added to attachment or side menu
  pub fn attachment_menu_bot(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, AttachmentMenuBot)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.attachment_menu_bot
  }

  /// Contains a list of audio files
  pub fn audios(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Audios)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.audios
  }

  /// Information about the authentication code that was sent
  pub fn authentication_code_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, AuthenticationCodeInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.authentication_code_info
  }

  /// Contains auto-download settings presets for the current user
  pub fn auto_download_settings_presets(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, AutoDownloadSettingsPresets)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.auto_download_settings_presets
  }

  /// Describes autosave settings
  pub fn autosave_settings(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, AutosaveSettings)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.autosave_settings
  }

  /// Contains a list of gifts that can be sent to another user or channel chat
  pub fn available_gifts(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, AvailableGifts)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.available_gifts
  }

  /// Represents a list of reactions that can be added to a message
  pub fn available_reactions(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, AvailableReactions)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.available_reactions
  }

  /// Describes a chat background
  pub fn background(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Background)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.background
  }

  /// Contains a list of backgrounds
  pub fn backgrounds(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, Backgrounds)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.backgrounds
  }

  /// Information about a bank card
  pub fn bank_card_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, BankCardInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.bank_card_info
  }

  /// Represents a basic group of 0-200 users (must be upgraded to a supergroup to accommodate more than 200 users)
  pub fn basic_group(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, BasicGroup)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.basic_group
  }

  /// Contains full information about a basic group
  pub fn basic_group_full_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, BasicGroupFullInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.basic_group_full_info
  }

  /// Describes users that have access to a bot
  pub fn bot_access_settings(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, BotAccessSettings)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.bot_access_settings
  }

  /// Contains a list of bot commands
  pub fn bot_commands(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, BotCommands)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.bot_commands
  }

  /// Describes media previews of a bot
  pub fn bot_media_preview(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, BotMediaPreview)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.bot_media_preview
  }

  /// Contains a list of media previews of a bot for the given language and the list of languages for which the bot has dedicated previews
  pub fn bot_media_preview_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, BotMediaPreviewInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.bot_media_preview_info
  }

  /// Contains a list of media previews of a bot
  pub fn bot_media_previews(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, BotMediaPreviews)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.bot_media_previews
  }

  /// Describes a button to be shown instead of bot commands menu button
  pub fn bot_menu_button(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, BotMenuButton)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.bot_menu_button
  }

  /// Contains information about a business chat link
  pub fn business_chat_link(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, BusinessChatLink)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.business_chat_link
  }

  /// Contains information about a business chat link
  pub fn business_chat_link_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, BusinessChatLinkInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.business_chat_link_info
  }

  /// Contains a list of business chat links created by the user
  pub fn business_chat_links(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, BusinessChatLinks)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.business_chat_links
  }

  /// Describes a connection of a bot to an account
  pub fn business_connected_bot_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, BusinessConnectedBotInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.business_connected_bot_info
  }

  /// Describes a connection of the bot with a business account
  pub fn business_connection(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, BusinessConnection)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.business_connection
  }

  /// Contains information about features, available to Business user accounts
  pub fn business_features(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, BusinessFeatures)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.business_features
  }

  /// Describes a message from a business account as received by a bot
  pub fn business_message(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, BusinessMessage)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.business_message
  }

  /// Contains a list of messages from a business account as received by a bot
  pub fn business_messages(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, BusinessMessages)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.business_messages
  }

  /// Contains the call identifier
  pub fn call_id(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, CallId)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.call_id
  }

  /// Contains a bot's answer to a callback query
  pub fn callback_query_answer(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, CallbackQueryAnswer)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.callback_query_answer
  }

  /// A chat. (Can be a private chat, basic group, supergroup, or secret chat)
  pub fn chat(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Chat)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.chat
  }

  /// Describes active stories posted by a chat
  pub fn chat_active_stories(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ChatActiveStories)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.chat_active_stories
  }

  /// Represents a list of chat administrators
  pub fn chat_administrators(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ChatAdministrators)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.chat_administrators
  }

  /// Contains a list of features available on the first chat boost levels
  pub fn chat_boost_features(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ChatBoostFeatures)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.chat_boost_features
  }

  /// Contains a list of features available on a specific chat boost level
  pub fn chat_boost_level_features(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ChatBoostLevelFeatures)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.chat_boost_level_features
  }

  /// Contains an HTTPS link to boost a chat
  pub fn chat_boost_link(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, ChatBoostLink)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.chat_boost_link
  }

  /// Contains information about a link to boost a chat
  pub fn chat_boost_link_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ChatBoostLinkInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.chat_boost_link_info
  }

  /// Contains a list of chat boost slots
  pub fn chat_boost_slots(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ChatBoostSlots)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.chat_boost_slots
  }

  /// Describes current boost status of a chat
  pub fn chat_boost_status(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ChatBoostStatus)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.chat_boost_status
  }

  /// Contains a list of chat events
  pub fn chat_events(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, ChatEvents)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.chat_events
  }

  /// Represents a folder for user chats
  pub fn chat_folder(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, ChatFolder)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.chat_folder
  }

  /// Represents an icon for a chat folder
  pub fn chat_folder_icon(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ChatFolderIcon)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.chat_folder_icon
  }

  /// Contains basic information about a chat folder
  pub fn chat_folder_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ChatFolderInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.chat_folder_info
  }

  /// Contains a chat folder invite link
  pub fn chat_folder_invite_link(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ChatFolderInviteLink)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.chat_folder_invite_link
  }

  /// Contains information about an invite link to a chat folder
  pub fn chat_folder_invite_link_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ChatFolderInviteLinkInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.chat_folder_invite_link_info
  }

  /// Represents a list of chat folder invite links
  pub fn chat_folder_invite_links(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ChatFolderInviteLinks)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.chat_folder_invite_links
  }

  /// Contains a chat invite link
  pub fn chat_invite_link(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ChatInviteLink)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.chat_invite_link
  }

  /// Contains a list of chat invite link counts
  pub fn chat_invite_link_counts(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ChatInviteLinkCounts)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.chat_invite_link_counts
  }

  /// Contains information about a chat invite link
  pub fn chat_invite_link_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ChatInviteLinkInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.chat_invite_link_info
  }

  /// Contains a list of chat members joined a chat via an invite link
  pub fn chat_invite_link_members(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ChatInviteLinkMembers)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.chat_invite_link_members
  }

  /// Contains a list of chat invite links
  pub fn chat_invite_links(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ChatInviteLinks)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.chat_invite_links
  }

  /// Contains a list of requests to join a chat
  pub fn chat_join_requests(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ChatJoinRequests)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.chat_join_requests
  }

  /// Contains a list of chat lists
  pub fn chat_lists(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, ChatLists)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.chat_lists
  }

  /// Describes a user or a chat as a member of another chat
  pub fn chat_member(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, ChatMember)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.chat_member
  }

  /// Contains a list of chat members
  pub fn chat_members(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, ChatMembers)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.chat_members
  }

  /// Represents a list of message senders, which can be used to send messages in a chat
  pub fn chat_message_senders(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ChatMessageSenders)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.chat_message_senders
  }

  /// Contains a list of chat or user profile photos
  pub fn chat_photos(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, ChatPhotos)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.chat_photos
  }

  /// A detailed statistics about revenue earned from sponsored messages in a chat
  pub fn chat_revenue_statistics(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ChatRevenueStatistics)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.chat_revenue_statistics
  }

  /// Contains a list of chat revenue transactions
  pub fn chat_revenue_transactions(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ChatRevenueTransactions)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.chat_revenue_transactions
  }

  /// Represents a list of chats
  pub fn chats(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Chats)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.chats
  }

  /// Contains information about a collectible item and its last purchase
  pub fn collectible_item_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, CollectibleItemInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.collectible_item_info
  }

  /// Contains full information about a community
  pub fn community_full_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, CommunityFullInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.community_full_info
  }

  /// Contains identifier of a community
  pub fn community_id(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, CommunityId)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.community_id
  }

  /// Describes an affiliate program that was connected to an affiliate
  pub fn connected_affiliate_program(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ConnectedAffiliateProgram)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.connected_affiliate_program
  }

  /// Represents a list of affiliate programs that were connected to an affiliate
  pub fn connected_affiliate_programs(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ConnectedAffiliatePrograms)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.connected_affiliate_programs
  }

  /// Contains a list of websites the current user is logged in with Telegram
  pub fn connected_websites(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ConnectedWebsites)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.connected_websites
  }

  /// Contains a counter
  pub fn count(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Count)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.count
  }

  /// Contains information about countries
  pub fn countries(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Countries)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.countries
  }

  /// Contains information about a country
  pub fn country_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, CountryInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.country_info
  }

  /// Contains information about a newly created basic group chat
  pub fn created_basic_group_chat(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, CreatedBasicGroupChat)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.created_basic_group_chat
  }

  /// Represents a list of currencies
  pub fn currencies(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Currencies)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.currencies
  }

  /// Represents a list of currency exchange rates
  pub fn currency_exchange_rates(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, CurrencyExchangeRates)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.currency_exchange_rates
  }

  /// Describes the current weather
  pub fn current_weather(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, CurrentWeather)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.current_weather
  }

  /// Contains the result of a custom request
  pub fn custom_request_result(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, CustomRequestResult)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.custom_request_result
  }

  /// Contains some binary data
  pub fn data(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Data)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.data
  }

  /// Contains database statistics
  pub fn database_statistics(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, DatabaseStatistics)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.database_statistics
  }

  /// Contains information about a tg: deep link
  pub fn deep_link_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, DeepLinkInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.deep_link_info
  }

  /// Contains information about a topic in a channel direct messages chat administered by the current user
  pub fn direct_messages_chat_topic(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, DirectMessagesChatTopic)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.direct_messages_chat_topic
  }

  /// Information about the email address authentication code that was sent
  pub fn email_address_authentication_code_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, EmailAddressAuthenticationCodeInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.email_address_authentication_code_info
  }

  /// Represents a list of emoji categories
  pub fn emoji_categories(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, EmojiCategories)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.emoji_categories
  }

  /// Represents a list of emojis with their keywords
  pub fn emoji_keywords(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, EmojiKeywords)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.emoji_keywords
  }

  /// Contains information about an emoji reaction
  pub fn emoji_reaction(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, EmojiReaction)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.emoji_reaction
  }

  /// Contains a list of custom emoji identifiers for emoji statuses
  pub fn emoji_status_custom_emojis(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, EmojiStatusCustomEmojis)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.emoji_status_custom_emojis
  }

  /// Contains a list of emoji statuses
  pub fn emoji_statuses(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, EmojiStatuses)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.emoji_statuses
  }

  /// Represents a list of emojis
  pub fn emojis(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Emojis)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.emojis
  }

  /// An object of this type can be returned on every function call, in case of an error
  pub fn error(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Error)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.error
  }

  /// Represents a list of users that has failed to be added to a chat
  pub fn failed_to_add_members(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, FailedToAddMembers)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.failed_to_add_members
  }

  /// Represents a file
  pub fn file(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, File)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.file
  }

  /// Contains size of downloaded prefix of a file
  pub fn file_downloaded_prefix_size(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, FileDownloadedPrefixSize)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.file_downloaded_prefix_size
  }

  /// A text fixed using fixTextWithAi
  pub fn fixed_text(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, FixedText)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.fixed_text
  }

  /// A text with some entities
  pub fn formatted_text(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, FormattedText)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.formatted_text
  }

  /// Describes a forum topic
  pub fn forum_topic(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, ForumTopic)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.forum_topic
  }

  /// Contains basic information about a forum topic
  pub fn forum_topic_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ForumTopicInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.forum_topic_info
  }

  /// Describes a list of forum topics
  pub fn forum_topics(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, ForumTopics)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.forum_topics
  }

  /// Represents a list of found affiliate programs
  pub fn found_affiliate_programs(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, FoundAffiliatePrograms)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.found_affiliate_programs
  }

  /// Contains a list of boosts applied to a chat
  pub fn found_chat_boosts(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, FoundChatBoosts)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.found_chat_boosts
  }

  /// Contains a list of messages found by a search in a given chat
  pub fn found_chat_messages(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, FoundChatMessages)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.found_chat_messages
  }

  /// Contains a list of downloaded files, found by a search
  pub fn found_file_downloads(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, FoundFileDownloads)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.found_file_downloads
  }

  /// Contains a list of messages found by a search
  pub fn found_messages(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, FoundMessages)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.found_messages
  }

  /// Contains 0-based match position
  pub fn found_position(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, FoundPosition)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.found_position
  }

  /// Contains 0-based positions of matched objects
  pub fn found_positions(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, FoundPositions)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.found_positions
  }

  /// Contains a list of messages found by a public post search
  pub fn found_public_posts(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, FoundPublicPosts)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.found_public_posts
  }

  /// Contains a list of stories found by a search
  pub fn found_stories(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, FoundStories)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.found_stories
  }

  /// Represents a list of found users
  pub fn found_users(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, FoundUsers)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.found_users
  }

  /// Contains information about a Web App found by its short name
  pub fn found_web_app(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, FoundWebApp)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.found_web_app
  }

  /// Contains a list of game high scores
  pub fn game_high_scores(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, GameHighScores)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.game_high_scores
  }

  /// Represents a list of gifts that were acquired by the current user on an auction
  pub fn gift_auction_acquired_gifts(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, GiftAuctionAcquiredGifts)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.gift_auction_acquired_gifts
  }

  /// Represents auction state of a gift
  pub fn gift_auction_state(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, GiftAuctionState)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.gift_auction_state
  }

  /// Contains a list of chat themes based on upgraded gifts
  pub fn gift_chat_themes(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, GiftChatThemes)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.gift_chat_themes
  }

  /// Describes collection of gifts
  pub fn gift_collection(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, GiftCollection)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.gift_collection
  }

  /// Contains a list of gift collections
  pub fn gift_collections(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, GiftCollections)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.gift_collections
  }

  /// Contains examples of possible upgraded gifts for the given regular gift
  pub fn gift_upgrade_preview(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, GiftUpgradePreview)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.gift_upgrade_preview
  }

  /// Contains all possible variants of upgraded gifts for the given regular gift
  pub fn gift_upgrade_variants(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, GiftUpgradeVariants)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.gift_upgrade_variants
  }

  /// Represents a list of gifts received by a user or a chat
  pub fn gifts_for_crafting(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, GiftsForCrafting)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.gifts_for_crafting
  }

  /// Describes gifts available for resale
  pub fn gifts_for_resale(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, GiftsForResale)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.gifts_for_resale
  }

  /// A detailed statistics about TON Grams earned by the current user
  pub fn gram_revenue_statistics(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, GramRevenueStatistics)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.gram_revenue_statistics
  }

  /// Describes a group call
  pub fn group_call(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, GroupCall)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.group_call
  }

  /// Contains the group call identifier
  pub fn group_call_id(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, GroupCallId)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.group_call_id
  }

  /// Contains information about a just created or just joined group call
  pub fn group_call_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, GroupCallInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.group_call_info
  }

  /// Represents a group call participant
  pub fn group_call_participant(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, GroupCallParticipant)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.group_call_participant
  }

  /// Contains identifiers of group call participants
  pub fn group_call_participants(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, GroupCallParticipants)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.group_call_participants
  }

  /// Represents a list of group call streams
  pub fn group_call_streams(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, GroupCallStreams)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.group_call_streams
  }

  /// Contains a list of hashtags
  pub fn hashtags(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Hashtags)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.hashtags
  }

  /// Contains an HTTP URL
  pub fn http_url(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, HttpUrl)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.http_url
  }

  /// Represents the result of an importContacts request
  pub fn imported_contacts(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ImportedContacts)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.imported_contacts
  }

  /// Contains identifier of a sent guest message
  pub fn inline_message_id(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, InlineMessageId)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.inline_message_id
  }

  /// Represents the results of the inline query. Use sendInlineQueryResultMessage to send the result of the query
  pub fn inline_query_results(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, InlineQueryResults)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.inline_query_results
  }

  /// Represents a single button in a bot keyboard
  pub fn keyboard_button(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, KeyboardButton)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.keyboard_button
  }

  /// Contains information about a language pack
  pub fn language_pack_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, LanguagePackInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.language_pack_info
  }

  /// Contains a list of language pack strings
  pub fn language_pack_strings(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, LanguagePackStrings)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.language_pack_strings
  }

  /// Describes a link preview
  pub fn link_preview(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, LinkPreview)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.link_preview
  }

  /// Contains a list of users and chats that spend most money on paid messages and reactions in a live story
  pub fn live_story_donors(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, LiveStoryDonors)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.live_story_donors
  }

  /// Contains information about the current localization target
  pub fn localization_target_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, LocalizationTargetInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.localization_target_info
  }

  /// Contains a list of available TDLib internal log tags
  pub fn log_tags(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, LogTags)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.log_tags
  }

  /// Contains a TDLib internal log verbosity level
  pub fn log_verbosity_level(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, LogVerbosityLevel)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.log_verbosity_level
  }

  /// Contains information about the main Web App of a bot
  pub fn main_web_app(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, MainWebApp)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.main_web_app
  }

  /// Describes a message
  pub fn message(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Message)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.message
  }

  /// Contains default auto-delete timer setting for new chats
  pub fn message_auto_delete_time(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, MessageAutoDeleteTime)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.message_auto_delete_time
  }

  /// Contains information about found messages, split by days according to the option "utc_time_offset"
  pub fn message_calendar(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, MessageCalendar)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.message_calendar
  }

  /// Contains information about an effect added to a message
  pub fn message_effect(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, MessageEffect)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.message_effect
  }

  /// Contains an HTTPS link to a message in a supergroup or channel, or a forum topic
  pub fn message_link(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, MessageLink)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.message_link
  }

  /// Contains information about a link to a message or a forum topic in a chat
  pub fn message_link_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, MessageLinkInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.message_link_info
  }

  /// Contains a list of message positions
  pub fn message_positions(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, MessagePositions)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.message_positions
  }

  /// Contains properties of a message and describes actions that can be done with the message right now
  pub fn message_properties(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, MessageProperties)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.message_properties
  }

  /// Represents a list of message senders
  pub fn message_senders(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, MessageSenders)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.message_senders
  }

  /// A detailed statistics about a message
  pub fn message_statistics(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, MessageStatistics)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.message_statistics
  }

  /// Contains information about a message thread
  pub fn message_thread_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, MessageThreadInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.message_thread_info
  }

  /// Represents a list of message viewers
  pub fn message_viewers(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, MessageViewers)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.message_viewers
  }

  /// Contains a list of messages
  pub fn messages(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Messages)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.messages
  }

  /// A full list of available network statistic entries
  pub fn network_statistics(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, NetworkStatistics)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.network_statistics
  }

  /// Contains privacy settings for chats with non-contacts
  pub fn new_chat_privacy_settings(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, NewChatPrivacySettings)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.new_chat_privacy_settings
  }

  /// Describes a notification sound in MP3 format
  pub fn notification_sound(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, NotificationSound)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.notification_sound
  }

  /// Contains a list of notification sounds
  pub fn notification_sounds(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, NotificationSounds)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.notification_sounds
  }

  /// Information about the OAuth authorization
  pub fn oauth_link_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, OauthLinkInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.oauth_link_info
  }

  /// An object of this type is returned on a successful function call for certain functions
  pub fn ok(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Ok)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.ok
  }

  /// Describes availability of cryptocurrency purchase through an on-ramp provider
  pub fn on_ramp_payment_availability(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, OnRampPaymentAvailability)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.on_ramp_payment_availability
  }

  /// Describes limits for an on-ramp purchase
  pub fn on_ramp_payment_limits(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, OnRampPaymentLimits)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.on_ramp_payment_limits
  }

  /// Describes a quote for an on-ramp purchase
  pub fn on_ramp_payment_quote(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, OnRampPaymentQuote)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.on_ramp_payment_quote
  }

  /// Describes a session for an on-ramp purchase
  pub fn on_ramp_payment_session(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, OnRampPaymentSession)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.on_ramp_payment_session
  }

  /// Represents a list of on-ramp providers
  pub fn on_ramp_providers(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, OnRampProviders)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.on_ramp_providers
  }

  /// Order information
  pub fn order_info(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, OrderInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.order_info
  }

  /// Represents outline of an image
  pub fn outline(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Outline)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.outline
  }

  /// Describes a passkey
  pub fn passkey(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Passkey)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.passkey
  }

  /// Contains a list of passkeys
  pub fn passkeys(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Passkeys)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.passkeys
  }

  /// Contains information about a Telegram Passport authorization form that was requested
  pub fn passport_authorization_form(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, PassportAuthorizationForm)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.passport_authorization_form
  }

  /// Contains information about saved Telegram Passport elements
  pub fn passport_elements(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, PassportElements)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.passport_elements
  }

  /// Contains information about Telegram Passport elements and corresponding errors
  pub fn passport_elements_with_errors(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, PassportElementsWithErrors)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.passport_elements_with_errors
  }

  /// Represents the current state of 2-step verification
  pub fn password_state(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, PasswordState)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.password_state
  }

  /// Contains information about an invoice payment form
  pub fn payment_form(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, PaymentForm)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.payment_form
  }

  /// Contains information about a successful payment
  pub fn payment_receipt(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, PaymentReceipt)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.payment_receipt
  }

  /// Contains the result of a payment request
  pub fn payment_result(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, PaymentResult)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.payment_result
  }

  /// Contains information about a phone number
  pub fn phone_number_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, PhoneNumberInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.phone_number_info
  }

  /// Contains properties of a poll option and describes actions that can be done with the option right now
  pub fn poll_option_properties(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, PollOptionProperties)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.poll_option_properties
  }

  /// A detailed statistics about poll votes
  pub fn poll_vote_statistics(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, PollVoteStatistics)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.poll_vote_statistics
  }

  /// Represents a list of poll voters
  pub fn poll_voters(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, PollVoters)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.poll_voters
  }

  /// Contains information about features, available to Premium users
  pub fn premium_features(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, PremiumFeatures)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.premium_features
  }

  /// Contains information about a Telegram Premium gift code
  pub fn premium_gift_code_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, PremiumGiftCodeInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.premium_gift_code_info
  }

  /// Contains a list of options for gifting Telegram Premium to a user
  pub fn premium_gift_payment_options(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, PremiumGiftPaymentOptions)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.premium_gift_payment_options
  }

  /// Contains a list of options for creating of Telegram Premium giveaway or manual distribution of Telegram Premium among chat members
  pub fn premium_giveaway_payment_options(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, PremiumGiveawayPaymentOptions)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.premium_giveaway_payment_options
  }

  /// Contains information about a limit, increased for Premium users
  pub fn premium_limit(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, PremiumLimit)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.premium_limit
  }

  /// Contains state of Telegram Premium subscription and promotion videos for Premium features
  pub fn premium_state(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, PremiumState)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.premium_state
  }

  /// Represents a ready to send inline message. Use sendInlineQueryResultMessage to send the message
  pub fn prepared_inline_message(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, PreparedInlineMessage)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.prepared_inline_message
  }

  /// Represents an inline message that can be sent via the bot
  pub fn prepared_inline_message_id(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, PreparedInlineMessageId)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.prepared_inline_message_id
  }

  /// Contains a list of backdrops, which can be used as a background of a theme of the screen for sharing a link to a user or a chat
  pub fn profile_share_theme_backdrops(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ProfileShareThemeBackdrops)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.profile_share_theme_backdrops
  }

  /// Represents a list of public forwards and reposts as a story of a message or a story
  pub fn public_forwards(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, PublicForwards)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.public_forwards
  }

  /// Contains information about public post search limits
  pub fn public_post_search_limits(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, PublicPostSearchLimits)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.public_post_search_limits
  }

  /// Contains a globally unique push receiver identifier, which can be used to identify which account has received a push notification
  pub fn push_receiver_id(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, PushReceiverId)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.push_receiver_id
  }

  /// Describes a message that can be used for quick reply
  pub fn quick_reply_message(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, QuickReplyMessage)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.quick_reply_message
  }

  /// Contains a list of quick reply messages
  pub fn quick_reply_messages(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, QuickReplyMessages)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.quick_reply_messages
  }

  /// Contains privacy settings for message read date in private chats. Read dates are always shown to the users that can see online status of the current user regardless of this setting
  pub fn read_date_privacy_settings(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ReadDatePrivacySettings)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.read_date_privacy_settings
  }

  /// Represents a gift received by a user or a chat
  pub fn received_gift(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, ReceivedGift)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.received_gift
  }

  /// Represents a list of gifts received by a user or a chat
  pub fn received_gifts(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, ReceivedGifts)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.received_gifts
  }

  /// Contains a list of recommended chat folders
  pub fn recommended_chat_folders(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, RecommendedChatFolders)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.recommended_chat_folders
  }

  /// Contains information about the current recovery email address
  pub fn recovery_email_address(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, RecoveryEmailAddress)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.recovery_email_address
  }

  /// Describes a message with rich formatting
  pub fn rich_message(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, RichMessage)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.rich_message
  }

  /// Represents an RTMP URL
  pub fn rtmp_url(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, RtmpUrl)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.rtmp_url
  }

  /// Contains a list of tags used in Saved Messages
  pub fn saved_messages_tags(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, SavedMessagesTags)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.saved_messages_tags
  }

  /// Contains information about notification settings for several chats
  pub fn scope_notification_settings(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ScopeNotificationSettings)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.scope_notification_settings
  }

  /// Contains a value representing a number of seconds
  pub fn seconds(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Seconds)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.seconds
  }

  /// Represents a secret chat
  pub fn secret_chat(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, SecretChat)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.secret_chat
  }

  /// Contains information about one session in a Telegram application used by the current user. Sessions must be shown to the user in the returned order
  pub fn session(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Session)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.session
  }

  /// Contains a list of sessions
  pub fn sessions(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Sessions)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.sessions
  }

  /// Contains a list of sponsored chats
  pub fn sponsored_chats(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, SponsoredChats)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.sponsored_chats
  }

  /// Contains a list of sponsored messages
  pub fn sponsored_messages(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, SponsoredMessages)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.sponsored_messages
  }

  /// Describes state of the stake dice
  pub fn stake_dice_state(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, StakeDiceState)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.stake_dice_state
  }

  /// Describes a possibly non-integer Telegram Star amount
  pub fn star_amount(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, StarAmount)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.star_amount
  }

  /// Contains a number of Telegram Stars
  pub fn star_count(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, StarCount)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.star_count
  }

  /// Contains a list of options for creating of Telegram Star giveaway
  pub fn star_giveaway_payment_options(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, StarGiveawayPaymentOptions)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.star_giveaway_payment_options
  }

  /// Contains a list of options for buying Telegram Stars
  pub fn star_payment_options(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, StarPaymentOptions)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.star_payment_options
  }

  /// A detailed statistics about Telegram Stars earned by a user or a chat
  pub fn star_revenue_statistics(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, StarRevenueStatistics)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.star_revenue_statistics
  }

  /// Represents a list of Telegram Star subscriptions
  pub fn star_subscriptions(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, StarSubscriptions)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.star_subscriptions
  }

  /// Represents a list of Telegram Star transactions
  pub fn star_transactions(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, StarTransactions)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.star_transactions
  }

  /// Describes a sticker
  pub fn sticker(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Sticker)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.sticker
  }

  /// Represents a sticker set
  pub fn sticker_set(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, StickerSet)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.sticker_set
  }

  /// Represents a list of sticker sets
  pub fn sticker_sets(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, StickerSets)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.sticker_sets
  }

  /// Represents a list of stickers
  pub fn stickers(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Stickers)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.stickers
  }

  /// Contains the exact storage usage statistics split by chats and file type
  pub fn storage_statistics(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, StorageStatistics)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.storage_statistics
  }

  /// Contains approximate storage usage statistics, excluding files of unknown file type
  pub fn storage_statistics_fast(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, StorageStatisticsFast)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.storage_statistics_fast
  }

  /// Represents a list of stories
  pub fn stories(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Stories)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.stories
  }

  /// Represents a story
  pub fn story(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Story)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.story
  }

  /// Describes album of stories
  pub fn story_album(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, StoryAlbum)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.story_album
  }

  /// Represents a list of story albums
  pub fn story_albums(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, StoryAlbums)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.story_albums
  }

  /// Represents a list of interactions with a story
  pub fn story_interactions(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, StoryInteractions)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.story_interactions
  }

  /// A detailed statistics about a story
  pub fn story_statistics(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, StoryStatistics)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.story_statistics
  }

  /// Represents a supergroup or channel with zero or more members (subscribers in the case of channels)
  pub fn supergroup(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Supergroup)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.supergroup
  }

  /// Contains full information about a supergroup or channel
  pub fn supergroup_full_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, SupergroupFullInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.supergroup_full_info
  }

  /// Contains a list of t.me URLs
  pub fn t_me_urls(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, TMeUrls)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.t_me_urls
  }

  /// Contains information about the availability of a temporary password, which can be used for payments and withdrawal of upgraded gifts
  pub fn temporary_password_state(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, TemporaryPasswordState)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.temporary_password_state
  }

  /// A simple object containing a sequence of bytes; for testing only
  pub fn test_bytes(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, TestBytes)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.test_bytes
  }

  /// A simple object containing a number; for testing only
  pub fn test_int(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, TestInt)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.test_int
  }

  /// A simple object containing a string; for testing only
  pub fn test_string(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, TestString)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.test_string
  }

  /// A simple object containing a vector of numbers; for testing only
  pub fn test_vector_int(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, TestVectorInt)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.test_vector_int
  }

  /// A simple object containing a vector of objects that hold a number; for testing only
  pub fn test_vector_int_object(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, TestVectorIntObject)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.test_vector_int_object
  }

  /// A simple object containing a vector of strings; for testing only
  pub fn test_vector_string(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, TestVectorString)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.test_vector_string
  }

  /// A simple object containing a vector of objects that hold a string; for testing only
  pub fn test_vector_string_object(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, TestVectorStringObject)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.test_vector_string_object
  }

  /// Contains some text
  pub fn text(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Text)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.text
  }

  /// Describes a style that can be used to compose a text
  pub fn text_composition_style(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, TextCompositionStyle)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.text_composition_style
  }

  /// Contains an example of text composition style usage
  pub fn text_composition_style_example(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, TextCompositionStyleExample)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.text_composition_style_example
  }

  /// Contains a list of text entities
  pub fn text_entities(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, TextEntities)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.text_entities
  }

  /// Contains a list of time zones
  pub fn time_zones(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, TimeZones)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.time_zones
  }

  /// Contains an HTTP URL for TON Center Streaming API
  pub fn ton_center_streaming_api_url(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, TonCenterStreamingApiUrl)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.ton_center_streaming_api_url
  }

  /// Represents a TON Connect challenge
  pub fn ton_connect_challenge(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, TonConnectChallenge)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.ton_connect_challenge
  }

  /// Represents a session with its pending TON Connect requests
  pub fn ton_connect_requests(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, TonConnectRequests)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.ton_connect_requests
  }

  /// Represents a TON Connect session. See https://github.com/ton-blockchain/ton-connect/tree/main/spec for full TON Connect protocol documentation
  pub fn ton_connect_session(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, TonConnectSession)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.ton_connect_session
  }

  /// Represents identifier of an event in a TON Connect session
  pub fn ton_connect_session_event_id(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, TonConnectSessionEventId)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.ton_connect_session_event_id
  }

  /// Contains a list of TON Connect sessions
  pub fn ton_connect_sessions(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, TonConnectSessions)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.ton_connect_sessions
  }

  /// Represents a list of TON NFTs
  pub fn ton_nfts(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, TonNfts)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.ton_nfts
  }

  /// Represents a list of TON Gram transactions
  pub fn ton_transactions(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, TonTransactions)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.ton_transactions
  }

  /// Describes a TON wallet card, which is shown on the profile page of a user to everyone
  pub fn ton_wallet_card(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, TonWalletCard)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.ton_wallet_card
  }

  /// Represents a challenge for TON wallet ownership proof
  pub fn ton_wallet_ownership_proof_challenge(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, TonWalletOwnershipProofChallenge)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.ton_wallet_ownership_proof_challenge
  }

  /// Contains the secret phrase of a TON wallet
  pub fn ton_wallet_secret_phrase(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, TonWalletSecretPhrase)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.ton_wallet_secret_phrase
  }

  /// Represents a transaction with the TON wallet of the current user
  pub fn ton_wallet_transaction(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, TonWalletTransaction)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.ton_wallet_transaction
  }

  /// Represents a list of TON Gram transactions
  pub fn ton_wallet_transactions(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, TonWalletTransactions)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.ton_wallet_transactions
  }

  /// Represents result of cryptocurrency transfer from the current user's TON wallet
  pub fn ton_wallet_transfer_result(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, TonWalletTransferResult)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.ton_wallet_transfer_result
  }

  /// Represents a list of trending sticker sets
  pub fn trending_sticker_sets(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, TrendingStickerSets)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.trending_sticker_sets
  }

  /// Contains a list of updates
  pub fn updates(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Updates)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.updates
  }

  /// Contains result of gift upgrading
  pub fn upgrade_gift_result(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpgradeGiftResult)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.upgrade_gift_result
  }

  /// Describes an upgraded gift that can be transferred to another owner or transferred to the TON blockchain as an NFT
  pub fn upgraded_gift(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, UpgradedGift)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.upgraded_gift
  }

  /// Contains a list of names of upgraded gift collections. The name of the collection of an upgraded gift is the part of upgradedGift.name before the last "-"
  pub fn upgraded_gift_collection_names(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpgradedGiftCollectionNames)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.upgraded_gift_collection_names
  }

  /// Contains information needed to deposit an upgraded gift from the TON wallet of the current user to a Telegram account. The gift must be transferred by an internal message from the wallet to nft_address with gram_amount attached and with payload as the message body
  pub fn upgraded_gift_deposit_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpgradedGiftDepositInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.upgraded_gift_deposit_info
  }

  /// Contains information about value of an upgraded gift
  pub fn upgraded_gift_value_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UpgradedGiftValueInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.upgraded_gift_value_info
  }

  /// Represents a user
  pub fn user(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, User)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.user
  }

  /// Contains full information about a user
  pub fn user_full_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send + Sync + Fn((Api, UserFullInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static,
    >,
  > {
    &self.listener.user_full_info
  }

  /// Contains an HTTPS URL, which can be used to get information about a user
  pub fn user_link(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, UserLink)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.user_link
  }

  /// A list of privacy rules. Rules are matched in the specified order. The first matched rule defines the privacy setting for a given user. If no rule matches, the action is not allowed
  pub fn user_privacy_setting_rules(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UserPrivacySettingRules)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.user_privacy_setting_rules
  }

  /// Contains custom information about the user
  pub fn user_support_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UserSupportInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.user_support_info
  }

  /// Contains TON wallet address of a user
  pub fn user_ton_wallet_address(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UserTonWalletAddress)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.user_ton_wallet_address
  }

  /// Represents a list of TON wallet addresses of users
  pub fn user_ton_wallet_addresses(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, UserTonWalletAddresses)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.user_ton_wallet_addresses
  }

  /// Represents a list of users
  pub fn users(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, Users)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.users
  }

  /// Contains a temporary identifier of validated order information, which is stored for one hour, and the available shipping options
  pub fn validated_order_info(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, ValidatedOrderInfo)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.validated_order_info
  }

  /// Contains a list of advertisements to be shown while a video from a message is watched
  pub fn video_message_advertisements(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, VideoMessageAdvertisements)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.video_message_advertisements
  }

  /// Describes balance of the current user in the bot https://t.me/walt
  pub fn wallet_bot_balance(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, WalletBotBalance)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.wallet_bot_balance
  }

  /// Contains information about a Web App
  pub fn web_app_info(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, WebAppInfo)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.web_app_info
  }

  /// Contains information about a Web App URL
  pub fn web_app_url(
    &self,
  ) -> &Option<
    Arc<dyn Send + Sync + Fn((Api, WebAppUrl)) -> LocalBoxFuture<'static, TGResult<()>> + 'static>,
  > {
    &self.listener.web_app_url
  }

  /// Describes an instant view page for a web page
  pub fn web_page_instant_view(
    &self,
  ) -> &Option<
    Arc<
      dyn Send
        + Sync
        + Fn((Api, WebPageInstantView)) -> LocalBoxFuture<'static, TGResult<()>>
        + 'static,
    >,
  > {
    &self.listener.web_page_instant_view
  }
}
