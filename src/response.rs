//! TDLib answers an abstract result with a concrete constructor.
//!
//! `getOption` is declared `= OptionValue`, and the value TDLib actually
//! sends is `{"@type":"optionValueString", ...}`. `optionValueString` is a
//! constructor of `OptionValue`, not a `TdType` variant. `TdType` deserializes
//! `@type` of `OptionValue` (the class), which never appears on the wire, so
//! `from_json::<TdType>` rejects the response.
//!
//! Every function result that is an abstract class with more than one
//! constructor is listed here. `Update` is not: each `update*` constructor is
//! already a `TdType` variant so listeners can match it. A response-shaped
//! update (one that carries `@extra`) is wrapped as `TdType::Update` by
//! [`read_response`] for `testUseUpdate`.

use rtdlib::errors::RTDError;
use rtdlib::types::*;

/// Decode one TDLib object for listeners.
///
/// Abstract results become the class variant (`TdType::OptionValue`).
/// Updates stay on their concrete variant.
pub(crate) fn read_object(json: &str) -> Result<TdType, RTDError> {
  match from_json::<TdType>(json) {
    Ok(object) => Ok(object),
    Err(error) => lift_abstract(json).ok_or(error),
  }
}

/// Decode one TDLib object for an async caller waiting on `@extra`.
///
/// Same as [`read_object`], except an `update*` object that carries `@extra`
/// is returned as `TdType::Update`. Pushed updates have no `@extra` and stay
/// concrete, which is what listeners match.
pub(crate) fn read_response(json: &str) -> Result<TdType, RTDError> {
  let object = read_object(json)?;
  if !is_update_response(&object) {
    return Ok(object);
  }
  match from_json::<Update>(json) {
    Ok(update) => Ok(TdType::Update(update)),
    Err(_) => Ok(object),
  }
}

fn is_update_response(object: &TdType) -> bool {
  let name = object.td_name();
  object.extra().is_some() && name.starts_with("update") && name != "updates"
}

#[allow(clippy::too_many_lines)]
fn lift_abstract(json: &str) -> Option<TdType> {
  let type_name = detect_td_type(json)?;
  match type_name.as_str() {
    "authorizationStateClosed"
    | "authorizationStateClosing"
    | "authorizationStateLoggingOut"
    | "authorizationStateReady"
    | "authorizationStateWaitCode"
    | "authorizationStateWaitEmailAddress"
    | "authorizationStateWaitEmailCode"
    | "authorizationStateWaitOtherDeviceConfirmation"
    | "authorizationStateWaitPassword"
    | "authorizationStateWaitPhoneNumber"
    | "authorizationStateWaitPremiumPurchase"
    | "authorizationStateWaitRegistration"
    | "authorizationStateWaitTdlibParameters" => from_json::<AuthorizationState>(json)
      .ok()
      .map(TdType::AuthorizationState),
    "canPostStoryResultActiveStoryLimitExceeded"
    | "canPostStoryResultBoostNeeded"
    | "canPostStoryResultLiveStoryIsActive"
    | "canPostStoryResultMonthlyLimitExceeded"
    | "canPostStoryResultOk"
    | "canPostStoryResultPremiumNeeded"
    | "canPostStoryResultWeeklyLimitExceeded" => from_json::<CanPostStoryResult>(json)
      .ok()
      .map(TdType::CanPostStoryResult),
    "canSendGiftResultFail" | "canSendGiftResultOk" => from_json::<CanSendGiftResult>(json)
      .ok()
      .map(TdType::CanSendGiftResult),
    "canSendMessageToUserResultOk"
    | "canSendMessageToUserResultUserHasPaidMessages"
    | "canSendMessageToUserResultUserIsDeleted"
    | "canSendMessageToUserResultUserRestrictsNewChats" => {
      from_json::<CanSendMessageToUserResult>(json)
        .ok()
        .map(TdType::CanSendMessageToUserResult)
    }
    "canTransferOwnershipResultOk"
    | "canTransferOwnershipResultPasswordNeeded"
    | "canTransferOwnershipResultPasswordTooFresh"
    | "canTransferOwnershipResultSessionTooFresh" => from_json::<CanTransferOwnershipResult>(json)
      .ok()
      .map(TdType::CanTransferOwnershipResult),
    "chatJoinResultDeclined"
    | "chatJoinResultGuardBotApprovalRequired"
    | "chatJoinResultRequestSent"
    | "chatJoinResultSuccess" => from_json::<ChatJoinResult>(json)
      .ok()
      .map(TdType::ChatJoinResult),
    "chatStatisticsChannel" | "chatStatisticsSupergroup" => from_json::<ChatStatistics>(json)
      .ok()
      .map(TdType::ChatStatistics),
    "checkChatUsernameResultOk"
    | "checkChatUsernameResultPublicChatsTooMany"
    | "checkChatUsernameResultPublicGroupsUnavailable"
    | "checkChatUsernameResultUsernameInvalid"
    | "checkChatUsernameResultUsernameOccupied"
    | "checkChatUsernameResultUsernamePurchasable" => from_json::<CheckChatUsernameResult>(json)
      .ok()
      .map(TdType::CheckChatUsernameResult),
    "checkStickerSetNameResultNameInvalid"
    | "checkStickerSetNameResultNameOccupied"
    | "checkStickerSetNameResultOk" => from_json::<CheckStickerSetNameResult>(json)
      .ok()
      .map(TdType::CheckStickerSetNameResult),
    "craftGiftResultFail"
    | "craftGiftResultInvalidGift"
    | "craftGiftResultSuccess"
    | "craftGiftResultTooEarly" => from_json::<CraftGiftResult>(json)
      .ok()
      .map(TdType::CraftGiftResult),
    "giftResaleResultOk" | "giftResaleResultPriceIncreased" => from_json::<GiftResaleResult>(json)
      .ok()
      .map(TdType::GiftResaleResult),
    "giveawayInfoCompleted" | "giveawayInfoOngoing" => from_json::<GiveawayInfo>(json)
      .ok()
      .map(TdType::GiveawayInfo),
    "internalLinkTypeAttachmentMenuBot"
    | "internalLinkTypeAuthenticationCode"
    | "internalLinkTypeBackground"
    | "internalLinkTypeBotAddToChannel"
    | "internalLinkTypeBotStart"
    | "internalLinkTypeBotStartInGroup"
    | "internalLinkTypeBusinessChat"
    | "internalLinkTypeCallsPage"
    | "internalLinkTypeChatAffiliateProgram"
    | "internalLinkTypeChatBoost"
    | "internalLinkTypeChatFolderInvite"
    | "internalLinkTypeChatInvite"
    | "internalLinkTypeChatSelection"
    | "internalLinkTypeContactsPage"
    | "internalLinkTypeDirectMessagesChat"
    | "internalLinkTypeGame"
    | "internalLinkTypeGiftAuction"
    | "internalLinkTypeGiftCollection"
    | "internalLinkTypeGroupCall"
    | "internalLinkTypeInstantView"
    | "internalLinkTypeInvoice"
    | "internalLinkTypeLanguagePack"
    | "internalLinkTypeLiveStory"
    | "internalLinkTypeMainWebApp"
    | "internalLinkTypeMessage"
    | "internalLinkTypeMessageDraft"
    | "internalLinkTypeMyProfilePage"
    | "internalLinkTypeNewChannelChat"
    | "internalLinkTypeNewGroupChat"
    | "internalLinkTypeNewPrivateChat"
    | "internalLinkTypeNewStory"
    | "internalLinkTypeOauth"
    | "internalLinkTypePassportDataRequest"
    | "internalLinkTypePhoneNumberConfirmation"
    | "internalLinkTypePremiumFeaturesPage"
    | "internalLinkTypePremiumGiftCode"
    | "internalLinkTypePremiumGiftPurchase"
    | "internalLinkTypeProxy"
    | "internalLinkTypePublicChat"
    | "internalLinkTypeQrCodeAuthentication"
    | "internalLinkTypeRequestManagedBot"
    | "internalLinkTypeRestorePurchases"
    | "internalLinkTypeSavedMessages"
    | "internalLinkTypeSearch"
    | "internalLinkTypeSettings"
    | "internalLinkTypeStarPurchase"
    | "internalLinkTypeStickerSet"
    | "internalLinkTypeStory"
    | "internalLinkTypeStoryAlbum"
    | "internalLinkTypeTextCompositionStyle"
    | "internalLinkTypeTheme"
    | "internalLinkTypeTonConnect"
    | "internalLinkTypeTonWalletTransfer"
    | "internalLinkTypeUnknownDeepLink"
    | "internalLinkTypeUpgradedGift"
    | "internalLinkTypeUserPhoneNumber"
    | "internalLinkTypeUserToken"
    | "internalLinkTypeVideoChat"
    | "internalLinkTypeWebApp" => from_json::<InternalLinkType>(json)
      .ok()
      .map(TdType::InternalLinkType),
    "inviteGroupCallParticipantResultSuccess"
    | "inviteGroupCallParticipantResultUserAlreadyParticipant"
    | "inviteGroupCallParticipantResultUserPrivacyRestricted"
    | "inviteGroupCallParticipantResultUserWasBanned" => {
      from_json::<InviteGroupCallParticipantResult>(json)
        .ok()
        .map(TdType::InviteGroupCallParticipantResult)
    }
    "jsonValueArray" | "jsonValueBoolean" | "jsonValueNull" | "jsonValueNumber"
    | "jsonValueObject" | "jsonValueString" => {
      from_json::<JsonValue>(json).ok().map(TdType::JsonValue)
    }
    "languagePackStringValueDeleted"
    | "languagePackStringValueOrdinary"
    | "languagePackStringValuePluralized" => from_json::<LanguagePackStringValue>(json)
      .ok()
      .map(TdType::LanguagePackStringValue),
    "logStreamDefault" | "logStreamEmpty" | "logStreamFile" => {
      from_json::<LogStream>(json).ok().map(TdType::LogStream)
    }
    "loginUrlInfoOpen" | "loginUrlInfoRequestConfirmation" => from_json::<LoginUrlInfo>(json)
      .ok()
      .map(TdType::LoginUrlInfo),
    "messageFileTypeGroup" | "messageFileTypePrivate" | "messageFileTypeUnknown" => {
      from_json::<MessageFileType>(json)
        .ok()
        .map(TdType::MessageFileType)
    }
    "messageReadDateMyPrivacyRestricted"
    | "messageReadDateRead"
    | "messageReadDateTooOld"
    | "messageReadDateUnread"
    | "messageReadDateUserPrivacyRestricted" => from_json::<MessageReadDate>(json)
      .ok()
      .map(TdType::MessageReadDate),
    "messageSenderChat" | "messageSenderUser" => from_json::<MessageSender>(json)
      .ok()
      .map(TdType::MessageSender),
    "optionValueBoolean" | "optionValueEmpty" | "optionValueInteger" | "optionValueString" => {
      from_json::<OptionValue>(json).ok().map(TdType::OptionValue)
    }
    "passportElementAddress"
    | "passportElementBankStatement"
    | "passportElementDriverLicense"
    | "passportElementEmailAddress"
    | "passportElementIdentityCard"
    | "passportElementInternalPassport"
    | "passportElementPassport"
    | "passportElementPassportRegistration"
    | "passportElementPersonalDetails"
    | "passportElementPhoneNumber"
    | "passportElementRentalAgreement"
    | "passportElementTemporaryRegistration"
    | "passportElementUtilityBill" => from_json::<PassportElement>(json)
      .ok()
      .map(TdType::PassportElement),
    "reportChatResultMessagesRequired"
    | "reportChatResultOk"
    | "reportChatResultOptionRequired"
    | "reportChatResultTextRequired" => from_json::<ReportChatResult>(json)
      .ok()
      .map(TdType::ReportChatResult),
    "reportSponsoredResultAdsHidden"
    | "reportSponsoredResultFailed"
    | "reportSponsoredResultOk"
    | "reportSponsoredResultOptionRequired"
    | "reportSponsoredResultPremiumRequired" => from_json::<ReportSponsoredResult>(json)
      .ok()
      .map(TdType::ReportSponsoredResult),
    "reportStoryResultOk" | "reportStoryResultOptionRequired" | "reportStoryResultTextRequired" => {
      from_json::<ReportStoryResult>(json)
        .ok()
        .map(TdType::ReportStoryResult)
    }
    "resetPasswordResultDeclined" | "resetPasswordResultOk" | "resetPasswordResultPending" => {
      from_json::<ResetPasswordResult>(json)
        .ok()
        .map(TdType::ResetPasswordResult)
    }
    "startLiveStoryResultFail" | "startLiveStoryResultOk" => {
      from_json::<StartLiveStoryResult>(json)
        .ok()
        .map(TdType::StartLiveStoryResult)
    }
    "statisticalGraphAsync" | "statisticalGraphData" | "statisticalGraphError" => {
      from_json::<StatisticalGraph>(json)
        .ok()
        .map(TdType::StatisticalGraph)
    }
    "webBrowserTypeExternal" | "webBrowserTypeInApp" => from_json::<WebBrowserType>(json)
      .ok()
      .map(TdType::WebBrowserType),
    _ => None,
  }
}
