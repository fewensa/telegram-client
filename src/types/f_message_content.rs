use rtdlib::types as td_types;

use crate::types::t_message_content::*;

pub enum TGMessageContent {
  Animation(TGMessageAnimation),
  Audio(TGMessageAudio),
  BasicGroupChatCreate(TGMessageBasicGroupChatCreate),
  Call(TGMessageCall),
  ChatAddMembers(TGMessageChatAddMembers),
  ChatChangePhoto(TGMessageChatChangePhoto),
  ChatChangeTitle(TGMessageChatChangeTitle),
  ChatDeleteMember(TGMessageChatDeleteMember),
  ChatDeletePhoto(TGMessageChatDeletePhoto),
  ChatJoinByLink(TGMessageChatJoinByLink),
  ChatSetTtl(TGMessageChatSetTtl),
  ChatUpgradeFrom(TGMessageChatUpgradeFrom),
  ChatUpgradeTo(TGMessageChatUpgradeTo),
  Contact(TGMessageContact),
  ContactRegistered(TGMessageContactRegistered),
  CustomServiceAction(TGMessageCustomServiceAction),
  Document(TGMessageDocument),
  ExpiredPhoto(TGMessageExpiredPhoto),
  ExpiredVideo(TGMessageExpiredVideo),
  Game(TGMessageGame),
  GameScore(TGMessageGameScore),
  Invoice(TGMessageInvoice),
  Location(TGMessageLocation),
  PassportDataReceived(TGMessagePassportDataReceived),
  PassportDataSent(TGMessagePassportDataSent),
  PaymentSuccessful(TGMessagePaymentSuccessful),
  PaymentSuccessfulBot(TGMessagePaymentSuccessfulBot),
  Photo(TGMessagePhoto),
  PinMessage(TGMessagePinMessage),
  Poll(TGMessagePoll),
  ScreenshotTaken(TGMessageScreenshotTaken),
  Sticker(TGMessageSticker),
  SupergroupChatCreate(TGMessageSupergroupChatCreate),
  Text(TGMessageText),
  Unsupported(TGMessageUnsupported),
  Venue(TGMessageVenue),
  Video(TGMessageVideo),
  VideoNote(TGMessageVideoNote),
  VoiceNote(TGMessageVoiceNote),
  WebsiteConnected(TGMessageWebsiteConnected),
}

impl TGMessageContent {
  pub(crate) fn of(td: Box<td_types::MessageContent>) -> Self {
    rtd_type_mapping!(
      MessageContent,
      TGMessageContent,
      RTDMessageContentType,
      (MessageAnimation,                   Animation               ,    TGMessageAnimation);
      (MessageAudio,                       Audio                   ,    TGMessageAudio);
      (MessageBasicGroupChatCreate,        BasicGroupChatCreate    ,    TGMessageBasicGroupChatCreate);
      (MessageCall,                        Call                    ,    TGMessageCall);
      (MessageChatAddMembers,              ChatAddMembers          ,    TGMessageChatAddMembers);
      (MessageChatChangePhoto,             ChatChangePhoto         ,    TGMessageChatChangePhoto);
      (MessageChatChangeTitle,             ChatChangeTitle         ,    TGMessageChatChangeTitle);
      (MessageChatDeleteMember,            ChatDeleteMember        ,    TGMessageChatDeleteMember);
      (MessageChatDeletePhoto,             ChatDeletePhoto         ,    TGMessageChatDeletePhoto);
      (MessageChatJoinByLink,              ChatJoinByLink          ,    TGMessageChatJoinByLink);
      (MessageChatSetTtl,                  ChatSetTtl              ,    TGMessageChatSetTtl);
      (MessageChatUpgradeFrom,             ChatUpgradeFrom         ,    TGMessageChatUpgradeFrom);
      (MessageChatUpgradeTo,               ChatUpgradeTo           ,    TGMessageChatUpgradeTo);
      (MessageContact,                     Contact                 ,    TGMessageContact);
      (MessageContactRegistered,           ContactRegistered       ,    TGMessageContactRegistered);
      (MessageCustomServiceAction,         CustomServiceAction     ,    TGMessageCustomServiceAction);
      (MessageDocument,                    Document                ,    TGMessageDocument);
      (MessageExpiredPhoto,                ExpiredPhoto            ,    TGMessageExpiredPhoto);
      (MessageExpiredVideo,                ExpiredVideo            ,    TGMessageExpiredVideo);
      (MessageGame,                        Game                    ,    TGMessageGame);
      (MessageGameScore,                   GameScore               ,    TGMessageGameScore);
      (MessageInvoice,                     Invoice                 ,    TGMessageInvoice);
      (MessageLocation,                    Location                ,    TGMessageLocation);
      (MessagePassportDataReceived,        PassportDataReceived    ,    TGMessagePassportDataReceived);
      (MessagePassportDataSent,            PassportDataSent        ,    TGMessagePassportDataSent);
      (MessagePaymentSuccessful,           PaymentSuccessful       ,    TGMessagePaymentSuccessful);
      (MessagePaymentSuccessfulBot,        PaymentSuccessfulBot    ,    TGMessagePaymentSuccessfulBot);
      (MessagePhoto,                       Photo                   ,    TGMessagePhoto);
      (MessagePinMessage,                  PinMessage              ,    TGMessagePinMessage);
      (MessagePoll,                        Poll                    ,    TGMessagePoll);
      (MessageScreenshotTaken,             ScreenshotTaken         ,    TGMessageScreenshotTaken);
      (MessageSticker,                     Sticker                 ,    TGMessageSticker);
      (MessageSupergroupChatCreate,        SupergroupChatCreate    ,    TGMessageSupergroupChatCreate);
      (MessageText,                        Text                    ,    TGMessageText);
      (MessageUnsupported,                 Unsupported             ,    TGMessageUnsupported);
      (MessageVenue,                       Venue                   ,    TGMessageVenue);
      (MessageVideo,                       Video                   ,    TGMessageVideo);
      (MessageVideoNote,                   VideoNote               ,    TGMessageVideoNote);
      (MessageVoiceNote,                   VoiceNote               ,    TGMessageVoiceNote);
      (MessageWebsiteConnected,            WebsiteConnected        ,    TGMessageWebsiteConnected);
    )(td)
  }

  pub fn is_animation(&self) -> bool { tuple_enum_is!(TGMessageContent, Animation           )(self) }
  pub fn is_audio(&self) -> bool { tuple_enum_is!(TGMessageContent, Audio               )(self) }
  pub fn is_basic_group_chat_create(&self) -> bool { tuple_enum_is!(TGMessageContent, BasicGroupChatCreate)(self) }
  pub fn is_call(&self) -> bool { tuple_enum_is!(TGMessageContent, Call                )(self) }
  pub fn is_chat_add_members(&self) -> bool { tuple_enum_is!(TGMessageContent, ChatAddMembers      )(self) }
  pub fn is_chat_change_photo(&self) -> bool { tuple_enum_is!(TGMessageContent, ChatChangePhoto     )(self) }
  pub fn is_chat_change_title(&self) -> bool { tuple_enum_is!(TGMessageContent, ChatChangeTitle     )(self) }
  pub fn is_chat_delete_member(&self) -> bool { tuple_enum_is!(TGMessageContent, ChatDeleteMember    )(self) }
  pub fn is_chat_delete_photo(&self) -> bool { tuple_enum_is!(TGMessageContent, ChatDeletePhoto     )(self) }
  pub fn is_chat_join_by_link(&self) -> bool { tuple_enum_is!(TGMessageContent, ChatJoinByLink      )(self) }
  pub fn is_chat_set_ttl(&self) -> bool { tuple_enum_is!(TGMessageContent, ChatSetTtl          )(self) }
  pub fn is_chat_upgrade_from(&self) -> bool { tuple_enum_is!(TGMessageContent, ChatUpgradeFrom     )(self) }
  pub fn is_chat_upgrade_to(&self) -> bool { tuple_enum_is!(TGMessageContent, ChatUpgradeTo       )(self) }
  pub fn is_contact(&self) -> bool { tuple_enum_is!(TGMessageContent, Contact             )(self) }
  pub fn is_contact_registered(&self) -> bool { tuple_enum_is!(TGMessageContent, ContactRegistered   )(self) }
  pub fn is_custom_service_action(&self) -> bool { tuple_enum_is!(TGMessageContent, CustomServiceAction )(self) }
  pub fn is_document(&self) -> bool { tuple_enum_is!(TGMessageContent, Document            )(self) }
  pub fn is_expired_photo(&self) -> bool { tuple_enum_is!(TGMessageContent, ExpiredPhoto        )(self) }
  pub fn is_expired_video(&self) -> bool { tuple_enum_is!(TGMessageContent, ExpiredVideo        )(self) }
  pub fn is_game(&self) -> bool { tuple_enum_is!(TGMessageContent, Game                )(self) }
  pub fn is_game_score(&self) -> bool { tuple_enum_is!(TGMessageContent, GameScore           )(self) }
  pub fn is_invoice(&self) -> bool { tuple_enum_is!(TGMessageContent, Invoice             )(self) }
  pub fn is_location(&self) -> bool { tuple_enum_is!(TGMessageContent, Location            )(self) }
  pub fn is_passport_data_received(&self) -> bool { tuple_enum_is!(TGMessageContent, PassportDataReceived)(self) }
  pub fn is_passport_data_sent(&self) -> bool { tuple_enum_is!(TGMessageContent, PassportDataSent    )(self) }
  pub fn is_payment_successful(&self) -> bool { tuple_enum_is!(TGMessageContent, PaymentSuccessful   )(self) }
  pub fn is_payment_successful_bot(&self) -> bool { tuple_enum_is!(TGMessageContent, PaymentSuccessfulBot)(self) }
  pub fn is_photo(&self) -> bool { tuple_enum_is!(TGMessageContent, Photo               )(self) }
  pub fn is_pin_message(&self) -> bool { tuple_enum_is!(TGMessageContent, PinMessage          )(self) }
  pub fn is_poll(&self) -> bool { tuple_enum_is!(TGMessageContent, Poll                )(self) }
  pub fn is_screenshot_taken(&self) -> bool { tuple_enum_is!(TGMessageContent, ScreenshotTaken     )(self) }
  pub fn is_sticker(&self) -> bool { tuple_enum_is!(TGMessageContent, Sticker             )(self) }
  pub fn is_supergroup_chat_create(&self) -> bool { tuple_enum_is!(TGMessageContent, SupergroupChatCreate)(self) }
  pub fn is_text(&self) -> bool { tuple_enum_is!(TGMessageContent, Text                )(self) }
  pub fn is_unsupported(&self) -> bool { tuple_enum_is!(TGMessageContent, Unsupported         )(self) }
  pub fn is_venue(&self) -> bool { tuple_enum_is!(TGMessageContent, Venue               )(self) }
  pub fn is_video(&self) -> bool { tuple_enum_is!(TGMessageContent, Video               )(self) }
  pub fn is_video_note(&self) -> bool { tuple_enum_is!(TGMessageContent, VideoNote           )(self) }
  pub fn is_voice_note(&self) -> bool { tuple_enum_is!(TGMessageContent, VoiceNote           )(self) }
  pub fn is_website_connected(&self) -> bool { tuple_enum_is!(TGMessageContent, WebsiteConnected    )(self) }


  pub fn on_animation<F: FnOnce(&TGMessageAnimation)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, Animation           , |t| fnc(t))(self);
    self
  }
  pub fn on_audio<F: FnOnce(&TGMessageAudio)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, Audio               , |t| fnc(t))(self);
    self
  }
  pub fn on_basic_group_chat_create<F: FnOnce(&TGMessageBasicGroupChatCreate)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, BasicGroupChatCreate, |t| fnc(t))(self);
    self
  }
  pub fn on_call<F: FnOnce(&TGMessageCall)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, Call                , |t| fnc(t))(self);
    self
  }
  pub fn on_chat_add_members<F: FnOnce(&TGMessageChatAddMembers)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, ChatAddMembers      , |t| fnc(t))(self);
    self
  }
  pub fn on_chat_change_photo<F: FnOnce(&TGMessageChatChangePhoto)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, ChatChangePhoto     , |t| fnc(t))(self);
    self
  }
  pub fn on_chat_change_title<F: FnOnce(&TGMessageChatChangeTitle)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, ChatChangeTitle     , |t| fnc(t))(self);
    self
  }
  pub fn on_chat_delete_member<F: FnOnce(&TGMessageChatDeleteMember)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, ChatDeleteMember    , |t| fnc(t))(self);
    self
  }
  pub fn on_chat_delete_photo<F: FnOnce(&TGMessageChatDeletePhoto)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, ChatDeletePhoto     , |t| fnc(t))(self);
    self
  }
  pub fn on_chat_join_by_link<F: FnOnce(&TGMessageChatJoinByLink)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, ChatJoinByLink      , |t| fnc(t))(self);
    self
  }
  pub fn on_chat_set_ttl<F: FnOnce(&TGMessageChatSetTtl)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, ChatSetTtl          , |t| fnc(t))(self);
    self
  }
  pub fn on_chat_upgrade_from<F: FnOnce(&TGMessageChatUpgradeFrom)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, ChatUpgradeFrom     , |t| fnc(t))(self);
    self
  }
  pub fn on_chat_upgrade_to<F: FnOnce(&TGMessageChatUpgradeTo)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, ChatUpgradeTo       , |t| fnc(t))(self);
    self
  }
  pub fn on_contact<F: FnOnce(&TGMessageContact)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, Contact             , |t| fnc(t))(self);
    self
  }
  pub fn on_contact_registered<F: FnOnce(&TGMessageContactRegistered)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, ContactRegistered   , |t| fnc(t))(self);
    self
  }
  pub fn on_custom_service_action<F: FnOnce(&TGMessageCustomServiceAction)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, CustomServiceAction , |t| fnc(t))(self);
    self
  }
  pub fn on_document<F: FnOnce(&TGMessageDocument)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, Document            , |t| fnc(t))(self);
    self
  }
  pub fn on_expired_photo<F: FnOnce(&TGMessageExpiredPhoto)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, ExpiredPhoto        , |t| fnc(t))(self);
    self
  }
  pub fn on_expired_video<F: FnOnce(&TGMessageExpiredVideo)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, ExpiredVideo        , |t| fnc(t))(self);
    self
  }
  pub fn on_game<F: FnOnce(&TGMessageGame)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, Game                , |t| fnc(t))(self);
    self
  }
  pub fn on_game_score<F: FnOnce(&TGMessageGameScore)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, GameScore           , |t| fnc(t))(self);
    self
  }
  pub fn on_invoice<F: FnOnce(&TGMessageInvoice)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, Invoice             , |t| fnc(t))(self);
    self
  }
  pub fn on_location<F: FnOnce(&TGMessageLocation)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, Location            , |t| fnc(t))(self);
    self
  }
  pub fn on_passport_data_received<F: FnOnce(&TGMessagePassportDataReceived)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, PassportDataReceived, |t| fnc(t))(self);
    self
  }
  pub fn on_passport_data_sent<F: FnOnce(&TGMessagePassportDataSent)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, PassportDataSent    , |t| fnc(t))(self);
    self
  }
  pub fn on_payment_successful<F: FnOnce(&TGMessagePaymentSuccessful)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, PaymentSuccessful   , |t| fnc(t))(self);
    self
  }
  pub fn on_payment_successful_bot<F: FnOnce(&TGMessagePaymentSuccessfulBot)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, PaymentSuccessfulBot, |t| fnc(t))(self);
    self
  }
  pub fn on_photo<F: FnOnce(&TGMessagePhoto)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, Photo               , |t| fnc(t))(self);
    self
  }
  pub fn on_pin_message<F: FnOnce(&TGMessagePinMessage)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, PinMessage          , |t| fnc(t))(self);
    self
  }
  pub fn on_poll<F: FnOnce(&TGMessagePoll)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, Poll                , |t| fnc(t))(self);
    self
  }
  pub fn on_screenshot_taken<F: FnOnce(&TGMessageScreenshotTaken)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, ScreenshotTaken     , |t| fnc(t))(self);
    self
  }
  pub fn on_sticker<F: FnOnce(&TGMessageSticker)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, Sticker             , |t| fnc(t))(self);
    self
  }
  pub fn on_supergroup_chat_create<F: FnOnce(&TGMessageSupergroupChatCreate)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, SupergroupChatCreate, |t| fnc(t))(self);
    self
  }
  pub fn on_text<F: FnOnce(&TGMessageText)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, Text                , |t| fnc(t))(self);
    self
  }
  pub fn on_unsupported<F: FnOnce(&TGMessageUnsupported)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, Unsupported         , |t| fnc(t))(self);
    self
  }
  pub fn on_venue<F: FnOnce(&TGMessageVenue)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, Venue               , |t| fnc(t))(self);
    self
  }
  pub fn on_video<F: FnOnce(&TGMessageVideo)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, Video               , |t| fnc(t))(self);
    self
  }
  pub fn on_video_note<F: FnOnce(&TGMessageVideoNote)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, VideoNote           , |t| fnc(t))(self);
    self
  }
  pub fn on_voice_note<F: FnOnce(&TGMessageVoiceNote)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, VoiceNote           , |t| fnc(t))(self);
    self
  }
  pub fn on_website_connected<F: FnOnce(&TGMessageWebsiteConnected)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMessageContent, WebsiteConnected    , |t| fnc(t))(self);
    self
  }
}


impl TGMessageAnimation {}

impl TGMessageAudio {}

impl TGMessageBasicGroupChatCreate {}

impl TGMessageCall {}

impl TGMessageChatAddMembers {}

impl TGMessageChatChangePhoto {}

impl TGMessageChatChangeTitle {}

impl TGMessageChatDeleteMember {}

impl TGMessageChatDeletePhoto {}

impl TGMessageChatJoinByLink {}

impl TGMessageChatSetTtl {}

impl TGMessageChatUpgradeFrom {}

impl TGMessageChatUpgradeTo {}

impl TGMessageContact {}

impl TGMessageContactRegistered {}

impl TGMessageCustomServiceAction {}

impl TGMessageDocument {}

impl TGMessageExpiredPhoto {}

impl TGMessageExpiredVideo {}

impl TGMessageGame {}

impl TGMessageGameScore {}

impl TGMessageInvoice {}

impl TGMessageLocation {}

impl TGMessagePassportDataReceived {}

impl TGMessagePassportDataSent {}

impl TGMessagePaymentSuccessful {}

impl TGMessagePaymentSuccessfulBot {}

impl TGMessagePhoto {}

impl TGMessagePinMessage {}

impl TGMessagePoll {}

impl TGMessageScreenshotTaken {}

impl TGMessageSticker {}

impl TGMessageSupergroupChatCreate {}

impl TGMessageText {}

impl TGMessageUnsupported {}

impl TGMessageVenue {}

impl TGMessageVideo {}

impl TGMessageVideoNote {}

impl TGMessageVoiceNote {}

impl TGMessageWebsiteConnected {}
