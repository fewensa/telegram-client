// Generated from TDLib 1.8.69 td_api.tl. Match arms track TdType.
use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

use futures::channel::mpsc;
use rtdlib::types::{RObject, TdType};

static OBSERVER: LazyLock<Observer> = LazyLock::new(Observer::new);

struct Observer {
  channels: RwLock<HashMap<String, mpsc::Sender<TdType>>>,
}

impl Observer {
  fn new() -> Self {
    Self {
      channels: RwLock::new(HashMap::new()),
    }
  }

  fn notify(&self, payload: TdType) {
    let extra = match &payload {
      TdType::TestUseUpdate(value) => value.extra(),

      TdType::UpdateAccentColors(value) => value.extra(),

      TdType::UpdateActiveEmojiReactions(value) => value.extra(),

      TdType::UpdateActiveGiftAuctions(value) => value.extra(),

      TdType::UpdateActiveLiveLocationMessages(value) => value.extra(),

      TdType::UpdateActiveNotifications(value) => value.extra(),

      TdType::UpdateAgeVerificationParameters(value) => value.extra(),

      TdType::UpdateAllowedBotUsernamePremiumSuffixes(value) => value.extra(),

      TdType::UpdateAnimatedEmojiMessageClicked(value) => value.extra(),

      TdType::UpdateAnimationSearchParameters(value) => value.extra(),

      TdType::UpdateApplicationRecaptchaVerificationRequired(value) => value.extra(),

      TdType::UpdateApplicationVerificationRequired(value) => value.extra(),

      TdType::UpdateAttachmentMenuBots(value) => value.extra(),

      TdType::UpdateAuthorizationState(value) => value.extra(),

      TdType::UpdateAutosaveSettings(value) => value.extra(),

      TdType::UpdateAvailableMessageEffects(value) => value.extra(),

      TdType::UpdateBasicGroup(value) => value.extra(),

      TdType::UpdateBasicGroupFullInfo(value) => value.extra(),

      TdType::UpdateBusinessConnection(value) => value.extra(),

      TdType::UpdateBusinessMessageEdited(value) => value.extra(),

      TdType::UpdateBusinessMessagesDeleted(value) => value.extra(),

      TdType::UpdateCall(value) => value.extra(),

      TdType::UpdateChatAccentColors(value) => value.extra(),

      TdType::UpdateChatAction(value) => value.extra(),

      TdType::UpdateChatActionBar(value) => value.extra(),

      TdType::UpdateChatActiveStories(value) => value.extra(),

      TdType::UpdateChatAddedToList(value) => value.extra(),

      TdType::UpdateChatAvailableReactions(value) => value.extra(),

      TdType::UpdateChatBackground(value) => value.extra(),

      TdType::UpdateChatBlockList(value) => value.extra(),

      TdType::UpdateChatBoost(value) => value.extra(),

      TdType::UpdateChatBusinessBotManageBar(value) => value.extra(),

      TdType::UpdateChatDefaultDisableNotification(value) => value.extra(),

      TdType::UpdateChatDraftMessage(value) => value.extra(),

      TdType::UpdateChatEmojiStatus(value) => value.extra(),

      TdType::UpdateChatFolders(value) => value.extra(),

      TdType::UpdateChatHasProtectedContent(value) => value.extra(),

      TdType::UpdateChatHasScheduledMessages(value) => value.extra(),

      TdType::UpdateChatHasWelcomeMessages(value) => value.extra(),

      TdType::UpdateChatIsMarkedAsUnread(value) => value.extra(),

      TdType::UpdateChatIsTranslatable(value) => value.extra(),

      TdType::UpdateChatJoinResult(value) => value.extra(),

      TdType::UpdateChatLastMessage(value) => value.extra(),

      TdType::UpdateChatMember(value) => value.extra(),

      TdType::UpdateChatMessageAutoDeleteTime(value) => value.extra(),

      TdType::UpdateChatMessageSender(value) => value.extra(),

      TdType::UpdateChatNotificationSettings(value) => value.extra(),

      TdType::UpdateChatOnlineMemberCount(value) => value.extra(),

      TdType::UpdateChatPendingJoinRequests(value) => value.extra(),

      TdType::UpdateChatPermissions(value) => value.extra(),

      TdType::UpdateChatPhoto(value) => value.extra(),

      TdType::UpdateChatPosition(value) => value.extra(),

      TdType::UpdateChatReadInbox(value) => value.extra(),

      TdType::UpdateChatReadOutbox(value) => value.extra(),

      TdType::UpdateChatRemovedFromList(value) => value.extra(),

      TdType::UpdateChatReplyMarkup(value) => value.extra(),

      TdType::UpdateChatRevenueAmount(value) => value.extra(),

      TdType::UpdateChatTheme(value) => value.extra(),

      TdType::UpdateChatTitle(value) => value.extra(),

      TdType::UpdateChatUnreadMentionCount(value) => value.extra(),

      TdType::UpdateChatUnreadPollVoteCount(value) => value.extra(),

      TdType::UpdateChatUnreadReactionCount(value) => value.extra(),

      TdType::UpdateChatVideoChat(value) => value.extra(),

      TdType::UpdateChatViewAsTopics(value) => value.extra(),

      TdType::UpdateChatWelcomeMessages(value) => value.extra(),

      TdType::UpdateCommunity(value) => value.extra(),

      TdType::UpdateCommunityFullInfo(value) => value.extra(),

      TdType::UpdateConnectionState(value) => value.extra(),

      TdType::UpdateContactCloseBirthdays(value) => value.extra(),

      TdType::UpdateDefaultBackground(value) => value.extra(),

      TdType::UpdateDefaultPaidReactionType(value) => value.extra(),

      TdType::UpdateDefaultReactionType(value) => value.extra(),

      TdType::UpdateDeleteMessages(value) => value.extra(),

      TdType::UpdateDiceEmojis(value) => value.extra(),

      TdType::UpdateDirectMessagesChatTopic(value) => value.extra(),

      TdType::UpdateEmojiChatThemes(value) => value.extra(),

      TdType::UpdateFavoriteStickers(value) => value.extra(),

      TdType::UpdateFile(value) => value.extra(),

      TdType::UpdateFileAddedToDownloads(value) => value.extra(),

      TdType::UpdateFileDownload(value) => value.extra(),

      TdType::UpdateFileDownloads(value) => value.extra(),

      TdType::UpdateFileGenerationStart(value) => value.extra(),

      TdType::UpdateFileGenerationStop(value) => value.extra(),

      TdType::UpdateFileRemovedFromDownloads(value) => value.extra(),

      TdType::UpdateForumTopic(value) => value.extra(),

      TdType::UpdateForumTopicInfo(value) => value.extra(),

      TdType::UpdateFreezeState(value) => value.extra(),

      TdType::UpdateGiftAuctionState(value) => value.extra(),

      TdType::UpdateGramRevenueStatus(value) => value.extra(),

      TdType::UpdateGroupCall(value) => value.extra(),

      TdType::UpdateGroupCallMessageLevels(value) => value.extra(),

      TdType::UpdateGroupCallMessageSendFailed(value) => value.extra(),

      TdType::UpdateGroupCallMessagesDeleted(value) => value.extra(),

      TdType::UpdateGroupCallParticipant(value) => value.extra(),

      TdType::UpdateGroupCallParticipants(value) => value.extra(),

      TdType::UpdateGroupCallVerificationState(value) => value.extra(),

      TdType::UpdateHavePendingNotifications(value) => value.extra(),

      TdType::UpdateInstalledStickerSets(value) => value.extra(),

      TdType::UpdateLanguagePackStrings(value) => value.extra(),

      TdType::UpdateLiveStoryTopDonors(value) => value.extra(),

      TdType::UpdateManagedBot(value) => value.extra(),

      TdType::UpdateMessageContainsUnreadPollVotes(value) => value.extra(),

      TdType::UpdateMessageContent(value) => value.extra(),

      TdType::UpdateMessageContentOpened(value) => value.extra(),

      TdType::UpdateMessageEdited(value) => value.extra(),

      TdType::UpdateMessageEphemeralContent(value) => value.extra(),

      TdType::UpdateMessageFactCheck(value) => value.extra(),

      TdType::UpdateMessageInteractionInfo(value) => value.extra(),

      TdType::UpdateMessageIsPinned(value) => value.extra(),

      TdType::UpdateMessageLiveLocationViewed(value) => value.extra(),

      TdType::UpdateMessageMentionRead(value) => value.extra(),

      TdType::UpdateMessageReaction(value) => value.extra(),

      TdType::UpdateMessageReactions(value) => value.extra(),

      TdType::UpdateMessageSendAcknowledged(value) => value.extra(),

      TdType::UpdateMessageSendFailed(value) => value.extra(),

      TdType::UpdateMessageSendSucceeded(value) => value.extra(),

      TdType::UpdateMessageSuggestedPostInfo(value) => value.extra(),

      TdType::UpdateMessageUnreadReactions(value) => value.extra(),

      TdType::UpdateNewBusinessCallbackQuery(value) => value.extra(),

      TdType::UpdateNewBusinessMessage(value) => value.extra(),

      TdType::UpdateNewCallSignalingData(value) => value.extra(),

      TdType::UpdateNewCallbackQuery(value) => value.extra(),

      TdType::UpdateNewChat(value) => value.extra(),

      TdType::UpdateNewChatJoinRequest(value) => value.extra(),

      TdType::UpdateNewChosenInlineResult(value) => value.extra(),

      TdType::UpdateNewCustomEvent(value) => value.extra(),

      TdType::UpdateNewCustomQuery(value) => value.extra(),

      TdType::UpdateNewGroupCallMessage(value) => value.extra(),

      TdType::UpdateNewGroupCallPaidReaction(value) => value.extra(),

      TdType::UpdateNewGuestQuery(value) => value.extra(),

      TdType::UpdateNewInlineCallbackQuery(value) => value.extra(),

      TdType::UpdateNewInlineQuery(value) => value.extra(),

      TdType::UpdateNewMessage(value) => value.extra(),

      TdType::UpdateNewOauthRequest(value) => value.extra(),

      TdType::UpdateNewPreCheckoutQuery(value) => value.extra(),

      TdType::UpdateNewShippingQuery(value) => value.extra(),

      TdType::UpdateNotification(value) => value.extra(),

      TdType::UpdateNotificationGroup(value) => value.extra(),

      TdType::UpdateOption(value) => value.extra(),

      TdType::UpdateOwnedGramCount(value) => value.extra(),

      TdType::UpdateOwnedStarCount(value) => value.extra(),

      TdType::UpdatePaidMediaPurchased(value) => value.extra(),

      TdType::UpdatePendingMessage(value) => value.extra(),

      TdType::UpdatePoll(value) => value.extra(),

      TdType::UpdatePollAnswer(value) => value.extra(),

      TdType::UpdateProfileAccentColors(value) => value.extra(),

      TdType::UpdateQuickReplyShortcut(value) => value.extra(),

      TdType::UpdateQuickReplyShortcutDeleted(value) => value.extra(),

      TdType::UpdateQuickReplyShortcutMessages(value) => value.extra(),

      TdType::UpdateQuickReplyShortcuts(value) => value.extra(),

      TdType::UpdateReactionNotificationSettings(value) => value.extra(),

      TdType::UpdateRecentStickers(value) => value.extra(),

      TdType::UpdateSavedAnimations(value) => value.extra(),

      TdType::UpdateSavedMessagesTags(value) => value.extra(),

      TdType::UpdateSavedMessagesTopic(value) => value.extra(),

      TdType::UpdateSavedMessagesTopicCount(value) => value.extra(),

      TdType::UpdateSavedNotificationSounds(value) => value.extra(),

      TdType::UpdateScopeNotificationSettings(value) => value.extra(),

      TdType::UpdateSecretChat(value) => value.extra(),

      TdType::UpdateServiceNotification(value) => value.extra(),

      TdType::UpdateSpeechRecognitionTrial(value) => value.extra(),

      TdType::UpdateSpeedLimitNotification(value) => value.extra(),

      TdType::UpdateStakeDiceState(value) => value.extra(),

      TdType::UpdateStarRevenueStatus(value) => value.extra(),

      TdType::UpdateStickerSet(value) => value.extra(),

      TdType::UpdateStopMessageDraft(value) => value.extra(),

      TdType::UpdateStory(value) => value.extra(),

      TdType::UpdateStoryDeleted(value) => value.extra(),

      TdType::UpdateStoryListChatCount(value) => value.extra(),

      TdType::UpdateStoryPostFailed(value) => value.extra(),

      TdType::UpdateStoryPostSucceeded(value) => value.extra(),

      TdType::UpdateStoryStealthMode(value) => value.extra(),

      TdType::UpdateSuggestedActions(value) => value.extra(),

      TdType::UpdateSupergroup(value) => value.extra(),

      TdType::UpdateSupergroupFullInfo(value) => value.extra(),

      TdType::UpdateTermsOfService(value) => value.extra(),

      TdType::UpdateTextCompositionStyles(value) => value.extra(),

      TdType::UpdateTonWalletGaslessTransfersInfo(value) => value.extra(),

      TdType::UpdateTonWalletState(value) => value.extra(),

      TdType::UpdateTonWalletTonConnectSession(value) => value.extra(),

      TdType::UpdateTonWalletTonConnectSessionDisconnectRequired(value) => value.extra(),

      TdType::UpdateTopicMessageCount(value) => value.extra(),

      TdType::UpdateTrendingStickerSets(value) => value.extra(),

      TdType::UpdateTrustedMiniAppBots(value) => value.extra(),

      TdType::UpdateUnconfirmedSession(value) => value.extra(),

      TdType::UpdateUnreadChatCount(value) => value.extra(),

      TdType::UpdateUnreadMessageCount(value) => value.extra(),

      TdType::UpdateUser(value) => value.extra(),

      TdType::UpdateUserFullInfo(value) => value.extra(),

      TdType::UpdateUserPrivacySettingRules(value) => value.extra(),

      TdType::UpdateUserStatus(value) => value.extra(),

      TdType::UpdateUserSubscription(value) => value.extra(),

      TdType::UpdateVideoPublished(value) => value.extra(),

      TdType::UpdateWebAppMessageSent(value) => value.extra(),

      TdType::UpdateWebBrowserSettings(value) => value.extra(),

      TdType::UpdateWebProxyCarrierClosed(value) => value.extra(),

      TdType::UpdateWebProxyCarrierData(value) => value.extra(),

      TdType::UpdateWebProxyCarrierOpened(value) => value.extra(),

      TdType::AuthorizationState(value) => value.extra(),

      TdType::CanPostStoryResult(value) => value.extra(),

      TdType::CanSendGiftResult(value) => value.extra(),

      TdType::CanSendMessageToUserResult(value) => value.extra(),

      TdType::CanTransferOwnershipResult(value) => value.extra(),

      TdType::ChatJoinResult(value) => value.extra(),

      TdType::ChatStatistics(value) => value.extra(),

      TdType::CheckChatUsernameResult(value) => value.extra(),

      TdType::CheckStickerSetNameResult(value) => value.extra(),

      TdType::CraftGiftResult(value) => value.extra(),

      TdType::GiftResaleResult(value) => value.extra(),

      TdType::GiveawayInfo(value) => value.extra(),

      TdType::InternalLinkType(value) => value.extra(),

      TdType::InviteGroupCallParticipantResult(value) => value.extra(),

      TdType::JsonValue(value) => value.extra(),

      TdType::LanguagePackStringValue(value) => value.extra(),

      TdType::LogStream(value) => value.extra(),

      TdType::LoginUrlInfo(value) => value.extra(),

      TdType::MessageFileType(value) => value.extra(),

      TdType::MessageReadDate(value) => value.extra(),

      TdType::MessageSender(value) => value.extra(),

      TdType::OptionValue(value) => value.extra(),

      TdType::PassportElement(value) => value.extra(),

      TdType::ReportChatResult(value) => value.extra(),

      TdType::ReportSponsoredResult(value) => value.extra(),

      TdType::ReportStoryResult(value) => value.extra(),

      TdType::ResetPasswordResult(value) => value.extra(),

      TdType::StartLiveStoryResult(value) => value.extra(),

      TdType::StatisticalGraph(value) => value.extra(),

      TdType::Update(value) => value.extra(),

      TdType::WebBrowserType(value) => value.extra(),

      TdType::AccountTtl(value) => value.extra(),

      TdType::AddedProxies(value) => value.extra(),

      TdType::AddedProxy(value) => value.extra(),

      TdType::AddedReactions(value) => value.extra(),

      TdType::AnimatedEmoji(value) => value.extra(),

      TdType::Animation(value) => value.extra(),

      TdType::Animations(value) => value.extra(),

      TdType::ArchiveChatListSettings(value) => value.extra(),

      TdType::AttachmentMenuBot(value) => value.extra(),

      TdType::Audios(value) => value.extra(),

      TdType::AuthenticationCodeInfo(value) => value.extra(),

      TdType::AutoDownloadSettingsPresets(value) => value.extra(),

      TdType::AutosaveSettings(value) => value.extra(),

      TdType::AvailableGifts(value) => value.extra(),

      TdType::AvailableReactions(value) => value.extra(),

      TdType::Background(value) => value.extra(),

      TdType::Backgrounds(value) => value.extra(),

      TdType::BankCardInfo(value) => value.extra(),

      TdType::BasicGroup(value) => value.extra(),

      TdType::BasicGroupFullInfo(value) => value.extra(),

      TdType::BotAccessSettings(value) => value.extra(),

      TdType::BotCommands(value) => value.extra(),

      TdType::BotMediaPreview(value) => value.extra(),

      TdType::BotMediaPreviewInfo(value) => value.extra(),

      TdType::BotMediaPreviews(value) => value.extra(),

      TdType::BotMenuButton(value) => value.extra(),

      TdType::BusinessChatLink(value) => value.extra(),

      TdType::BusinessChatLinkInfo(value) => value.extra(),

      TdType::BusinessChatLinks(value) => value.extra(),

      TdType::BusinessConnectedBotInfo(value) => value.extra(),

      TdType::BusinessConnection(value) => value.extra(),

      TdType::BusinessFeatures(value) => value.extra(),

      TdType::BusinessMessage(value) => value.extra(),

      TdType::BusinessMessages(value) => value.extra(),

      TdType::CallId(value) => value.extra(),

      TdType::CallbackQueryAnswer(value) => value.extra(),

      TdType::Chat(value) => value.extra(),

      TdType::ChatActiveStories(value) => value.extra(),

      TdType::ChatAdministrators(value) => value.extra(),

      TdType::ChatBoostFeatures(value) => value.extra(),

      TdType::ChatBoostLevelFeatures(value) => value.extra(),

      TdType::ChatBoostLink(value) => value.extra(),

      TdType::ChatBoostLinkInfo(value) => value.extra(),

      TdType::ChatBoostSlots(value) => value.extra(),

      TdType::ChatBoostStatus(value) => value.extra(),

      TdType::ChatEvents(value) => value.extra(),

      TdType::ChatFolder(value) => value.extra(),

      TdType::ChatFolderIcon(value) => value.extra(),

      TdType::ChatFolderInfo(value) => value.extra(),

      TdType::ChatFolderInviteLink(value) => value.extra(),

      TdType::ChatFolderInviteLinkInfo(value) => value.extra(),

      TdType::ChatFolderInviteLinks(value) => value.extra(),

      TdType::ChatInviteLink(value) => value.extra(),

      TdType::ChatInviteLinkCounts(value) => value.extra(),

      TdType::ChatInviteLinkInfo(value) => value.extra(),

      TdType::ChatInviteLinkMembers(value) => value.extra(),

      TdType::ChatInviteLinks(value) => value.extra(),

      TdType::ChatJoinRequests(value) => value.extra(),

      TdType::ChatLists(value) => value.extra(),

      TdType::ChatMember(value) => value.extra(),

      TdType::ChatMembers(value) => value.extra(),

      TdType::ChatMessageSenders(value) => value.extra(),

      TdType::ChatPhotos(value) => value.extra(),

      TdType::ChatRevenueStatistics(value) => value.extra(),

      TdType::ChatRevenueTransactions(value) => value.extra(),

      TdType::Chats(value) => value.extra(),

      TdType::CollectibleItemInfo(value) => value.extra(),

      TdType::CommunityFullInfo(value) => value.extra(),

      TdType::CommunityId(value) => value.extra(),

      TdType::ConnectedAffiliateProgram(value) => value.extra(),

      TdType::ConnectedAffiliatePrograms(value) => value.extra(),

      TdType::ConnectedWebsites(value) => value.extra(),

      TdType::Count(value) => value.extra(),

      TdType::Countries(value) => value.extra(),

      TdType::CountryInfo(value) => value.extra(),

      TdType::CreatedBasicGroupChat(value) => value.extra(),

      TdType::Currencies(value) => value.extra(),

      TdType::CurrencyExchangeRates(value) => value.extra(),

      TdType::CurrentWeather(value) => value.extra(),

      TdType::CustomRequestResult(value) => value.extra(),

      TdType::Data(value) => value.extra(),

      TdType::DatabaseStatistics(value) => value.extra(),

      TdType::DeepLinkInfo(value) => value.extra(),

      TdType::DirectMessagesChatTopic(value) => value.extra(),

      TdType::EmailAddressAuthenticationCodeInfo(value) => value.extra(),

      TdType::EmojiCategories(value) => value.extra(),

      TdType::EmojiKeywords(value) => value.extra(),

      TdType::EmojiReaction(value) => value.extra(),

      TdType::EmojiStatusCustomEmojis(value) => value.extra(),

      TdType::EmojiStatuses(value) => value.extra(),

      TdType::Emojis(value) => value.extra(),

      TdType::Error(value) => value.extra(),

      TdType::FailedToAddMembers(value) => value.extra(),

      TdType::File(value) => value.extra(),

      TdType::FileDownloadedPrefixSize(value) => value.extra(),

      TdType::FixedText(value) => value.extra(),

      TdType::FormattedText(value) => value.extra(),

      TdType::ForumTopic(value) => value.extra(),

      TdType::ForumTopicInfo(value) => value.extra(),

      TdType::ForumTopics(value) => value.extra(),

      TdType::FoundAffiliatePrograms(value) => value.extra(),

      TdType::FoundChatBoosts(value) => value.extra(),

      TdType::FoundChatMessages(value) => value.extra(),

      TdType::FoundFileDownloads(value) => value.extra(),

      TdType::FoundMessages(value) => value.extra(),

      TdType::FoundPosition(value) => value.extra(),

      TdType::FoundPositions(value) => value.extra(),

      TdType::FoundPublicPosts(value) => value.extra(),

      TdType::FoundStories(value) => value.extra(),

      TdType::FoundUsers(value) => value.extra(),

      TdType::FoundWebApp(value) => value.extra(),

      TdType::GameHighScores(value) => value.extra(),

      TdType::GiftAuctionAcquiredGifts(value) => value.extra(),

      TdType::GiftAuctionState(value) => value.extra(),

      TdType::GiftChatThemes(value) => value.extra(),

      TdType::GiftCollection(value) => value.extra(),

      TdType::GiftCollections(value) => value.extra(),

      TdType::GiftUpgradePreview(value) => value.extra(),

      TdType::GiftUpgradeVariants(value) => value.extra(),

      TdType::GiftsForCrafting(value) => value.extra(),

      TdType::GiftsForResale(value) => value.extra(),

      TdType::GramRevenueStatistics(value) => value.extra(),

      TdType::GroupCall(value) => value.extra(),

      TdType::GroupCallId(value) => value.extra(),

      TdType::GroupCallInfo(value) => value.extra(),

      TdType::GroupCallParticipant(value) => value.extra(),

      TdType::GroupCallParticipants(value) => value.extra(),

      TdType::GroupCallStreams(value) => value.extra(),

      TdType::Hashtags(value) => value.extra(),

      TdType::HttpUrl(value) => value.extra(),

      TdType::ImportedContacts(value) => value.extra(),

      TdType::InlineMessageId(value) => value.extra(),

      TdType::InlineQueryResults(value) => value.extra(),

      TdType::KeyboardButton(value) => value.extra(),

      TdType::LanguagePackInfo(value) => value.extra(),

      TdType::LanguagePackStrings(value) => value.extra(),

      TdType::LinkPreview(value) => value.extra(),

      TdType::LiveStoryDonors(value) => value.extra(),

      TdType::LocalizationTargetInfo(value) => value.extra(),

      TdType::LogTags(value) => value.extra(),

      TdType::LogVerbosityLevel(value) => value.extra(),

      TdType::MainWebApp(value) => value.extra(),

      TdType::Message(value) => value.extra(),

      TdType::MessageAutoDeleteTime(value) => value.extra(),

      TdType::MessageCalendar(value) => value.extra(),

      TdType::MessageEffect(value) => value.extra(),

      TdType::MessageLink(value) => value.extra(),

      TdType::MessageLinkInfo(value) => value.extra(),

      TdType::MessagePositions(value) => value.extra(),

      TdType::MessageProperties(value) => value.extra(),

      TdType::MessageSenders(value) => value.extra(),

      TdType::MessageStatistics(value) => value.extra(),

      TdType::MessageThreadInfo(value) => value.extra(),

      TdType::MessageViewers(value) => value.extra(),

      TdType::Messages(value) => value.extra(),

      TdType::NetworkStatistics(value) => value.extra(),

      TdType::NewChatPrivacySettings(value) => value.extra(),

      TdType::NotificationSound(value) => value.extra(),

      TdType::NotificationSounds(value) => value.extra(),

      TdType::OauthLinkInfo(value) => value.extra(),

      TdType::Ok(value) => value.extra(),

      TdType::OnRampPaymentAvailability(value) => value.extra(),

      TdType::OnRampPaymentLimits(value) => value.extra(),

      TdType::OnRampPaymentQuote(value) => value.extra(),

      TdType::OnRampPaymentSession(value) => value.extra(),

      TdType::OnRampProviders(value) => value.extra(),

      TdType::OrderInfo(value) => value.extra(),

      TdType::Outline(value) => value.extra(),

      TdType::Passkey(value) => value.extra(),

      TdType::Passkeys(value) => value.extra(),

      TdType::PassportAuthorizationForm(value) => value.extra(),

      TdType::PassportElements(value) => value.extra(),

      TdType::PassportElementsWithErrors(value) => value.extra(),

      TdType::PasswordState(value) => value.extra(),

      TdType::PaymentForm(value) => value.extra(),

      TdType::PaymentReceipt(value) => value.extra(),

      TdType::PaymentResult(value) => value.extra(),

      TdType::PhoneNumberInfo(value) => value.extra(),

      TdType::PollOptionProperties(value) => value.extra(),

      TdType::PollVoteStatistics(value) => value.extra(),

      TdType::PollVoters(value) => value.extra(),

      TdType::PremiumFeatures(value) => value.extra(),

      TdType::PremiumGiftCodeInfo(value) => value.extra(),

      TdType::PremiumGiftPaymentOptions(value) => value.extra(),

      TdType::PremiumGiveawayPaymentOptions(value) => value.extra(),

      TdType::PremiumLimit(value) => value.extra(),

      TdType::PremiumState(value) => value.extra(),

      TdType::PreparedInlineMessage(value) => value.extra(),

      TdType::PreparedInlineMessageId(value) => value.extra(),

      TdType::ProfileShareThemeBackdrops(value) => value.extra(),

      TdType::PublicForwards(value) => value.extra(),

      TdType::PublicPostSearchLimits(value) => value.extra(),

      TdType::PushReceiverId(value) => value.extra(),

      TdType::QuickReplyMessage(value) => value.extra(),

      TdType::QuickReplyMessages(value) => value.extra(),

      TdType::ReadDatePrivacySettings(value) => value.extra(),

      TdType::ReceivedGift(value) => value.extra(),

      TdType::ReceivedGifts(value) => value.extra(),

      TdType::RecommendedChatFolders(value) => value.extra(),

      TdType::RecoveryEmailAddress(value) => value.extra(),

      TdType::RichMessage(value) => value.extra(),

      TdType::RtmpUrl(value) => value.extra(),

      TdType::SavedMessagesTags(value) => value.extra(),

      TdType::ScopeNotificationSettings(value) => value.extra(),

      TdType::Seconds(value) => value.extra(),

      TdType::SecretChat(value) => value.extra(),

      TdType::Session(value) => value.extra(),

      TdType::Sessions(value) => value.extra(),

      TdType::SponsoredChats(value) => value.extra(),

      TdType::SponsoredMessages(value) => value.extra(),

      TdType::StakeDiceState(value) => value.extra(),

      TdType::StarAmount(value) => value.extra(),

      TdType::StarCount(value) => value.extra(),

      TdType::StarGiveawayPaymentOptions(value) => value.extra(),

      TdType::StarPaymentOptions(value) => value.extra(),

      TdType::StarRevenueStatistics(value) => value.extra(),

      TdType::StarSubscriptions(value) => value.extra(),

      TdType::StarTransactions(value) => value.extra(),

      TdType::Sticker(value) => value.extra(),

      TdType::StickerSet(value) => value.extra(),

      TdType::StickerSets(value) => value.extra(),

      TdType::Stickers(value) => value.extra(),

      TdType::StorageStatistics(value) => value.extra(),

      TdType::StorageStatisticsFast(value) => value.extra(),

      TdType::Stories(value) => value.extra(),

      TdType::Story(value) => value.extra(),

      TdType::StoryAlbum(value) => value.extra(),

      TdType::StoryAlbums(value) => value.extra(),

      TdType::StoryInteractions(value) => value.extra(),

      TdType::StoryStatistics(value) => value.extra(),

      TdType::Supergroup(value) => value.extra(),

      TdType::SupergroupFullInfo(value) => value.extra(),

      TdType::TMeUrls(value) => value.extra(),

      TdType::TemporaryPasswordState(value) => value.extra(),

      TdType::TestBytes(value) => value.extra(),

      TdType::TestInt(value) => value.extra(),

      TdType::TestString(value) => value.extra(),

      TdType::TestVectorInt(value) => value.extra(),

      TdType::TestVectorIntObject(value) => value.extra(),

      TdType::TestVectorString(value) => value.extra(),

      TdType::TestVectorStringObject(value) => value.extra(),

      TdType::Text(value) => value.extra(),

      TdType::TextCompositionStyle(value) => value.extra(),

      TdType::TextCompositionStyleExample(value) => value.extra(),

      TdType::TextEntities(value) => value.extra(),

      TdType::TimeZones(value) => value.extra(),

      TdType::TonCenterStreamingApiUrl(value) => value.extra(),

      TdType::TonConnectChallenge(value) => value.extra(),

      TdType::TonConnectRequests(value) => value.extra(),

      TdType::TonConnectSession(value) => value.extra(),

      TdType::TonConnectSessionEventId(value) => value.extra(),

      TdType::TonConnectSessions(value) => value.extra(),

      TdType::TonNfts(value) => value.extra(),

      TdType::TonTransactions(value) => value.extra(),

      TdType::TonWalletCard(value) => value.extra(),

      TdType::TonWalletOwnershipProofChallenge(value) => value.extra(),

      TdType::TonWalletSecretPhrase(value) => value.extra(),

      TdType::TonWalletTransaction(value) => value.extra(),

      TdType::TonWalletTransactions(value) => value.extra(),

      TdType::TonWalletTransferResult(value) => value.extra(),

      TdType::TrendingStickerSets(value) => value.extra(),

      TdType::Updates(value) => value.extra(),

      TdType::UpgradeGiftResult(value) => value.extra(),

      TdType::UpgradedGift(value) => value.extra(),

      TdType::UpgradedGiftCollectionNames(value) => value.extra(),

      TdType::UpgradedGiftDepositInfo(value) => value.extra(),

      TdType::UpgradedGiftValueInfo(value) => value.extra(),

      TdType::User(value) => value.extra(),

      TdType::UserFullInfo(value) => value.extra(),

      TdType::UserLink(value) => value.extra(),

      TdType::UserPrivacySettingRules(value) => value.extra(),

      TdType::UserSupportInfo(value) => value.extra(),

      TdType::UserTonWalletAddress(value) => value.extra(),

      TdType::UserTonWalletAddresses(value) => value.extra(),

      TdType::Users(value) => value.extra(),

      TdType::ValidatedOrderInfo(value) => value.extra(),

      TdType::VideoMessageAdvertisements(value) => value.extra(),

      TdType::WalletBotBalance(value) => value.extra(),

      TdType::WebAppInfo(value) => value.extra(),

      TdType::WebAppUrl(value) => value.extra(),

      TdType::WebPageInstantView(value) => value.extra(),
    };
    if let Some(extra) = extra {
      let mut map = self.channels.write().unwrap();
      if let Some(sender) = map.get_mut(&extra) {
        sender.try_send(payload).unwrap();
      }
    }
  }

  fn subscribe(&self, extra: String) -> mpsc::Receiver<TdType> {
    let (sender, receiver) = mpsc::channel::<TdType>(1);
    if let Ok(mut map) = self.channels.write() {
      map.insert(extra, sender);
    }
    receiver
  }

  fn unsubscribe(&self, extra: &str) {
    if let Ok(mut map) = self.channels.write() {
      map.remove(extra);
    }
  }
}

pub fn notify(payload: TdType) {
  OBSERVER.notify(payload)
}

pub fn subscribe<T: AsRef<str>>(extra: T) -> mpsc::Receiver<TdType> {
  OBSERVER.subscribe(extra.as_ref().to_string())
}

pub fn unsubscribe<T: AsRef<str>>(extra: T) {
  OBSERVER.unsubscribe(extra.as_ref())
}
