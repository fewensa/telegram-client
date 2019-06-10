use rtdlib::types as td_types;
use rtdlib::types::{CallDiscardReason, MaskPoint, MessageCall, MessageContent, PassportElementType, PollOption, RObject, TextEntityType};

use crate::errors;
use crate::types::f_text_entity_type::TGTextEntityType;
use crate::types::t_input_message::TGTextEntity;
use crate::types::t_message_content::*;

#[derive(Debug, Clone)]
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
    tuple_rtd_type_mapping!(
      MessageContent,
      TGMessageContent,
      RTDMessageContentType,
      (MessageAnimation              ,     Animation                ,      TGMessageAnimation               );
      (MessageAudio                  ,     Audio                    ,      TGMessageAudio                   );
      (MessageBasicGroupChatCreate   ,     BasicGroupChatCreate     ,      TGMessageBasicGroupChatCreate    );
      (MessageCall                   ,     Call                     ,      TGMessageCall                    );
      (MessageChatAddMembers         ,     ChatAddMembers           ,      TGMessageChatAddMembers          );
      (MessageChatChangePhoto        ,     ChatChangePhoto          ,      TGMessageChatChangePhoto         );
      (MessageChatChangeTitle        ,     ChatChangeTitle          ,      TGMessageChatChangeTitle         );
      (MessageChatDeleteMember       ,     ChatDeleteMember         ,      TGMessageChatDeleteMember        );
      (MessageChatDeletePhoto        ,     ChatDeletePhoto          ,      TGMessageChatDeletePhoto         );
      (MessageChatJoinByLink         ,     ChatJoinByLink           ,      TGMessageChatJoinByLink          );
      (MessageChatSetTtl             ,     ChatSetTtl               ,      TGMessageChatSetTtl              );
      (MessageChatUpgradeFrom        ,     ChatUpgradeFrom          ,      TGMessageChatUpgradeFrom         );
      (MessageChatUpgradeTo          ,     ChatUpgradeTo            ,      TGMessageChatUpgradeTo           );
      (MessageContact                ,     Contact                  ,      TGMessageContact                 );
      (MessageContactRegistered      ,     ContactRegistered        ,      TGMessageContactRegistered       );
      (MessageCustomServiceAction    ,     CustomServiceAction      ,      TGMessageCustomServiceAction     );
      (MessageDocument               ,     Document                 ,      TGMessageDocument                );
      (MessageExpiredPhoto           ,     ExpiredPhoto             ,      TGMessageExpiredPhoto            );
      (MessageExpiredVideo           ,     ExpiredVideo             ,      TGMessageExpiredVideo            );
      (MessageGame                   ,     Game                     ,      TGMessageGame                    );
      (MessageGameScore              ,     GameScore                ,      TGMessageGameScore               );
      (MessageInvoice                ,     Invoice                  ,      TGMessageInvoice                 );
      (MessageLocation               ,     Location                 ,      TGMessageLocation                );
      (MessagePassportDataReceived   ,     PassportDataReceived     ,      TGMessagePassportDataReceived    );
      (MessagePassportDataSent       ,     PassportDataSent         ,      TGMessagePassportDataSent        );
      (MessagePaymentSuccessful      ,     PaymentSuccessful        ,      TGMessagePaymentSuccessful       );
      (MessagePaymentSuccessfulBot   ,     PaymentSuccessfulBot     ,      TGMessagePaymentSuccessfulBot    );
      (MessagePhoto                  ,     Photo                    ,      TGMessagePhoto                   );
      (MessagePinMessage             ,     PinMessage               ,      TGMessagePinMessage              );
      (MessagePoll                   ,     Poll                     ,      TGMessagePoll                    );
      (MessageScreenshotTaken        ,     ScreenshotTaken          ,      TGMessageScreenshotTaken         );
      (MessageSticker                ,     Sticker                  ,      TGMessageSticker                 );
      (MessageSupergroupChatCreate   ,     SupergroupChatCreate     ,      TGMessageSupergroupChatCreate    );
      (MessageText                   ,     Text                     ,      TGMessageText                    );
      (MessageUnsupported            ,     Unsupported              ,      TGMessageUnsupported             );
      (MessageVenue                  ,     Venue                    ,      TGMessageVenue                   );
      (MessageVideo                  ,     Video                    ,      TGMessageVideo                   );
      (MessageVideoNote              ,     VideoNote                ,      TGMessageVideoNote               );
      (MessageVoiceNote              ,     VoiceNote                ,      TGMessageVoiceNote               );
      (MessageWebsiteConnected       ,     WebsiteConnected         ,      TGMessageWebsiteConnected        );
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


impl TGMessageAnimation {
  pub fn animation(&self) -> TGAnimation { self.td_origin().animation().map(|v| TGAnimation::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn caption(&self) -> Option<TGFormattedText> { self.td_origin().caption().map(|v| TGFormattedText::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn is_secret(&self) -> bool { self.td_origin().is_secret().map_or(false, |v| v) }
}

impl TGAnimation {
  pub fn duration(&self) -> Option<i32> { self.td_origin().duration() }

  pub fn width(&self) -> i32 { self.td_origin().width().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn height(&self) -> i32 { self.td_origin().height().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn file_name(&self) -> String { self.td_origin().file_name().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn mime_type(&self) -> String { self.td_origin().mime_type().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn thumbnail(&self) -> Option<TGPhotoSize> { self.td_origin().thumbnail().map(|v| TGPhotoSize::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn animation(&self) -> TGFile { self.td_origin().animation().map(|v| TGFile::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGFormattedText {
  pub fn text(&self) -> String { self.td_origin().text().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn entities(&self) -> Vec<TGTextEntity> {
    self.td_origin()
      .entities()
      .map_or(vec![], |v| v.iter()
        .map(|v| TGTextEntity::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL))
        .collect::<Vec<TGTextEntity>>(),
      )
  }
}

impl TGFile {
  pub fn id(&self) -> i32 { self.td_origin().id().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn size(&self) -> i32 { self.td_origin().size().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn expected_size(&self) -> Option<i32> { self.td_origin().expected_size() }

  pub fn local(&self) -> Option<TGLocalFile> { self.td_origin().local().map(|v| TGLocalFile::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn remote(&self) -> Option<TGRemoteFile> { self.td_origin().remote().map(|v| TGRemoteFile::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }
}


impl TGLocalFile {
  pub fn path(&self) -> Option<String> { self.td_origin().path() }

  pub fn can_be_downloaded(&self) -> bool { self.td_origin().can_be_downloaded().map_or(false, |v| v) }

  pub fn can_be_deleted(&self) -> bool { self.td_origin().can_be_deleted().map_or(false, |v| v) }

  pub fn is_downloading_active(&self) -> bool { self.td_origin().is_downloading_active().map_or(false, |v| v) }

  pub fn is_downloading_completed(&self) -> bool { self.td_origin().is_downloading_completed().map_or(false, |v| v) }

  pub fn download_offset(&self) -> Option<i32> { self.td_origin().download_offset() }

  pub fn downloaded_prefix_size(&self) -> Option<i32> { self.td_origin().downloaded_prefix_size() }

  pub fn downloaded_size(&self) -> Option<i32> { self.td_origin().downloaded_size() }
}


impl TGRemoteFile {
  pub fn id(&self) -> Option<String> { self.td_origin().id() }

  pub fn is_uploading_active(&self) -> bool { self.td_origin().is_uploading_active().map_or(false, |v| v) }

  pub fn is_uploading_completed(&self) -> bool { self.td_origin().is_uploading_completed().map_or(false, |v| v) }

  pub fn uploaded_size(&self) -> i32 { self.td_origin().uploaded_size().expect(errors::TELEGRAM_DATA_FAIL) }
}


impl TGMessageAudio {
  pub fn audio(&self) -> TGAudio { self.td_origin().audio().map(|v| TGAudio::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn caption(&self) -> Option<TGFormattedText> { self.td_origin().caption().map(|v| TGFormattedText::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }
}


impl TGAudio {
  pub fn duration(&self) -> Option<i32> { self.td_origin().duration() }

  pub fn title(&self) -> Option<String> { self.td_origin().title() }

  pub fn performer(&self) -> Option<String> { self.td_origin().performer() }

  pub fn file_name(&self) -> Option<String> { self.td_origin().file_name() }

  pub fn mime_type(&self) -> Option<String> { self.td_origin().mime_type() }

  pub fn album_cover_thumbnail(&self) -> Option<TGPhotoSize> { self.td_origin().album_cover_thumbnail().map(|v| TGPhotoSize::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn audio(&self) -> TGFile { self.td_origin().audio().map(|v| TGFile::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }
}


impl TGMessageBasicGroupChatCreate {
  pub fn title(&self) -> String { self.td_origin().title().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn member_user_ids(&self) -> Vec<i32> { self.td_origin().member_user_ids().map_or(vec![], |v| v) }
}

impl TGMessageCall {
  pub fn discard_reason(&self) -> TGCallDiscardReason { self.td_origin().discard_reason().map(|v| TGCallDiscardReason::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn duration(&self) -> Option<i32> { self.td_origin().duration() }


  pub fn is_declined(&self) -> bool { self.discard_reason().is_declined() }
  pub fn is_disconnected(&self) -> bool { self.discard_reason().is_disconnected() }
  pub fn is_empty(&self) -> bool { self.discard_reason().is_empty() }
  pub fn is_hung_up(&self) -> bool { self.discard_reason().is_hung_up() }
  pub fn is_missed(&self) -> bool { self.discard_reason().is_missed() }
}

#[derive(Debug, Clone)]
pub enum TGCallDiscardReason {
  Declined(TGCallDiscardReasonDeclined),
  Disconnected(TGCallDiscardReasonDisconnected),
  Empty(TGCallDiscardReasonEmpty),
  HungUp(TGCallDiscardReasonHungUp),
  Missed(TGCallDiscardReasonMissed),
}

impl TGCallDiscardReason {
  fn of(td: Box<CallDiscardReason>) -> Self {
    tuple_rtd_type_mapping!(
      CallDiscardReason,
      TGCallDiscardReason,
      RTDCallDiscardReasonType,
      (CallDiscardReasonDeclined      , Declined      , TGCallDiscardReasonDeclined     );
      (CallDiscardReasonDisconnected  , Disconnected  , TGCallDiscardReasonDisconnected );
      (CallDiscardReasonEmpty         , Empty         , TGCallDiscardReasonEmpty        );
      (CallDiscardReasonHungUp        , HungUp        , TGCallDiscardReasonHungUp       );
      (CallDiscardReasonMissed        , Missed        , TGCallDiscardReasonMissed       );
    )(td)
  }


  pub fn is_declined(&self) -> bool { tuple_enum_is!(TGCallDiscardReason, Declined       )(self) }
  pub fn is_disconnected(&self) -> bool { tuple_enum_is!(TGCallDiscardReason, Disconnected   )(self) }
  pub fn is_empty(&self) -> bool { tuple_enum_is!(TGCallDiscardReason, Empty          )(self) }
  pub fn is_hung_up(&self) -> bool { tuple_enum_is!(TGCallDiscardReason, HungUp         )(self) }
  pub fn is_missed(&self) -> bool { tuple_enum_is!(TGCallDiscardReason, Missed         )(self) }


  pub fn on_declined<F: FnOnce(&TGCallDiscardReasonDeclined)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGCallDiscardReason, Declined      , |t| fnc(t))(self);
    self
  }
  pub fn on_disconnected<F: FnOnce(&TGCallDiscardReasonDisconnected)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGCallDiscardReason, Disconnected  , |t| fnc(t))(self);
    self
  }
  pub fn on_empty<F: FnOnce(&TGCallDiscardReasonEmpty)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGCallDiscardReason, Empty         , |t| fnc(t))(self);
    self
  }
  pub fn on_hung_up<F: FnOnce(&TGCallDiscardReasonHungUp)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGCallDiscardReason, HungUp        , |t| fnc(t))(self);
    self
  }
  pub fn on_missed<F: FnOnce(&TGCallDiscardReasonMissed)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGCallDiscardReason, Missed        , |t| fnc(t))(self);
    self
  }
}


impl TGMessageChatAddMembers {
  pub fn member_user_ids(&self) -> Vec<i32> { self.td_origin().member_user_ids().map_or(vec![], |v| v) }
}

impl TGMessageChatChangePhoto {
  pub fn photo(&self) -> TGPhoto { self.td_origin().photo().map(|v| TGPhoto::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGPhoto {
  pub fn has_stickers(&self) -> bool { self.td_origin().has_stickers().map_or(false, |v| v) }

  pub fn sizes(&self) -> Vec<TGPhotoSize> {
    self.td_origin()
      .sizes()
      .map_or(vec![], |v| v.iter()
        .map(|v| TGPhotoSize::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).collect::<Vec<TGPhotoSize>>())
  }
}

impl TGMessageChatChangeTitle {
  pub fn title(&self) -> String { self.td_origin().title().expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGMessageChatDeleteMember {
  pub fn user_id(&self) -> i32 { self.td_origin().user_id().expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGMessageChatDeletePhoto {}

impl TGMessageChatJoinByLink {}

impl TGMessageChatSetTtl {
  pub fn ttl(&self) -> i32 { self.td_origin().ttl().expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGMessageChatUpgradeFrom {
  pub fn title(&self) -> String { self.td_origin().title().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn basic_group_id(&self) -> i32 { self.td_origin().basic_group_id().expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGMessageChatUpgradeTo {
  pub fn supergroup_id(&self) -> i32 { self.td_origin().supergroup_id().expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGMessageContact {
  pub fn contact(&self) -> TGContact { self.td_origin().contact().map(|v| TGContact::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGContact {
  pub fn phone_number(&self) -> String { self.td_origin().phone_number().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn first_name(&self) -> Option<String> { self.td_origin().first_name() }

  pub fn last_name(&self) -> Option<String> { self.td_origin().last_name() }

  pub fn vcard(&self) -> Option<String> { self.td_origin().vcard() }

  pub fn user_id(&self) -> i32 { self.td_origin().user_id().expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGMessageContactRegistered {}

impl TGMessageCustomServiceAction {
  pub fn text(&self) -> String { self.td_origin().text().expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGMessageDocument {
  pub fn document(&self) -> TGDocument { self.td_origin().document().map(|v| TGDocument::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn caption(&self) -> Option<TGFormattedText> { self.td_origin().caption().map(|v| TGFormattedText::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }
}

impl TGDocument {
  pub fn file_name(&self) -> String { self.td_origin().file_name().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn mime_type(&self) -> Option<String> { self.td_origin().mime_type() }

  pub fn thumbnail(&self) -> Option<TGPhotoSize> { self.td_origin().thumbnail().map(|v| TGPhotoSize::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn document(&self) -> TGFile { self.td_origin().document().map(|v| TGFile::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGMessageExpiredPhoto {}

impl TGMessageExpiredVideo {}

impl TGMessageGame {}

impl TGGame {
  pub fn id(&self) -> i64 { self.td_origin().id().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn short_name(&self) -> Option<String> { self.td_origin().short_name() }

  pub fn title(&self) -> Option<String> { self.td_origin().title() }

  pub fn text(&self) -> Option<TGFormattedText> { self.td_origin().text().map(|v| TGFormattedText::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn description(&self) -> Option<String> { self.td_origin().description() }

  pub fn photo(&self) -> Option<TGPhoto> { self.td_origin().photo().map(|v| TGPhoto::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn animation(&self) -> Option<TGAnimation> { self.td_origin().animation().map(|v| TGAnimation::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }
}


impl TGMessageGameScore {
  pub fn game_message_id(&self) -> i64 { self.td_origin().game_message_id().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn game_id(&self) -> i64 { self.td_origin().game_id().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn score(&self) -> i32 { self.td_origin().score().expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGMessageInvoice {
  pub fn title(&self) -> Option<String> { self.td_origin().title() }

  pub fn description(&self) -> Option<String> { self.td_origin().description() }

  pub fn photo(&self) -> Option<TGPhoto> { self.td_origin().photo().map(|v| TGPhoto::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn currency(&self) -> Option<String> { self.td_origin().currency() }

  pub fn total_amount(&self) -> Option<i64> { self.td_origin().total_amount() }

  pub fn start_parameter(&self) -> Option<String> { self.td_origin().start_parameter() }

  pub fn is_test(&self) -> bool { self.td_origin().is_test().map_or(false, |v| v) }

  pub fn need_shipping_address(&self) -> bool { self.td_origin().need_shipping_address().map_or(false, |v| v) }

  pub fn receipt_message_id(&self) -> Option<i64> { self.td_origin().receipt_message_id() }
}

impl TGMessageLocation {
  pub fn location(&self) -> TGLocation { self.td_origin().location().map(|v| TGLocation::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn live_period(&self) -> Option<i32> { self.td_origin().live_period() }

  pub fn expires_in(&self) -> Option<i32> { self.td_origin().expires_in() }
}

impl TGLocation {
  pub fn latitude(&self) -> f64 { self.td_origin().latitude().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn longitude(&self) -> f64 { self.td_origin().longitude().expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGMessagePassportDataReceived {
  pub fn elements(&self) -> Vec<TGEncryptedPassportElement> {
    self.td_origin().elements().map_or(vec![], |v| v.iter()
      .map(|v| TGEncryptedPassportElement::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL))
      .collect::<Vec<TGEncryptedPassportElement>>(),
    )
  }

  pub fn credentials(&self) -> Option<TGEncryptedCredentials> { self.td_origin().credentials().map(|v| TGEncryptedCredentials::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }
}

impl TGEncryptedPassportElement {
  pub fn type_(&self) -> TGPassportElementType { self.td_origin().type_().map(|v| TGPassportElementType::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn data(&self) -> String { self.td_origin().data().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn front_side(&self) -> Option<TGDatedFile> { self.td_origin().front_side().map(|v| TGDatedFile::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn reverse_side(&self) -> Option<TGDatedFile> { self.td_origin().reverse_side().map(|v| TGDatedFile::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn selfie(&self) -> Option<TGDatedFile> { self.td_origin().selfie().map(|v| TGDatedFile::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn translation(&self) -> Option<Vec<TGDatedFile>> {
    self.td_origin().translation().map(|v| v.iter()
      .map(|v| TGDatedFile::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL))
      .collect::<Vec<TGDatedFile>>())
  }

  pub fn files(&self) -> Option<Vec<TGDatedFile>> {
    self.td_origin().files().map(|v| v.iter()
      .map(|v| TGDatedFile::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL))
      .collect::<Vec<TGDatedFile>>())
  }

  pub fn value(&self) -> Option<String> { self.td_origin().value() }

  pub fn hash(&self) -> Option<String> { self.td_origin().hash() }


  pub fn is_address(&self) -> bool { self.type_().is_address() }
  pub fn is_bank_statement(&self) -> bool { self.type_().is_bank_statement() }
  pub fn is_driver_license(&self) -> bool { self.type_().is_driver_license() }
  pub fn is_email_address(&self) -> bool { self.type_().is_email_address() }
  pub fn is_identity_card(&self) -> bool { self.type_().is_identity_card() }
  pub fn is_internal_passport(&self) -> bool { self.type_().is_internal_passport() }
  pub fn is_passport(&self) -> bool { self.type_().is_passport() }
  pub fn is_passport_registration(&self) -> bool { self.type_().is_passport_registration() }
  pub fn is_personal_details(&self) -> bool { self.type_().is_personal_details() }
  pub fn is_phone_number(&self) -> bool { self.type_().is_phone_number() }
  pub fn is_rental_agreement(&self) -> bool { self.type_().is_rental_agreement() }
  pub fn is_temporary_registration(&self) -> bool { self.type_().is_temporary_registration() }
  pub fn is_utility_bill(&self) -> bool { self.type_().is_utility_bill() }
}

#[derive(Debug, Clone)]
pub enum TGPassportElementType {
  Address(TGPassportElementTypeAddress),
  BankStatement(TGPassportElementTypeBankStatement),
  DriverLicense(TGPassportElementTypeDriverLicense),
  EmailAddress(TGPassportElementTypeEmailAddress),
  IdentityCard(TGPassportElementTypeIdentityCard),
  InternalPassport(TGPassportElementTypeInternalPassport),
  Passport(TGPassportElementTypePassport),
  PassportRegistration(TGPassportElementTypePassportRegistration),
  PersonalDetails(TGPassportElementTypePersonalDetails),
  PhoneNumber(TGPassportElementTypePhoneNumber),
  RentalAgreement(TGPassportElementTypeRentalAgreement),
  TemporaryRegistration(TGPassportElementTypeTemporaryRegistration),
  UtilityBill(TGPassportElementTypeUtilityBill),
}


impl TGPassportElementType {
  fn of(td: Box<PassportElementType>) -> Self {
    tuple_rtd_type_mapping!(
      PassportElementType,
      TGPassportElementType,
      RTDPassportElementTypeType,
      (PassportElementTypeAddress                ,     Address                 ,      TGPassportElementTypeAddress                   );
      (PassportElementTypeBankStatement          ,     BankStatement           ,      TGPassportElementTypeBankStatement             );
      (PassportElementTypeDriverLicense          ,     DriverLicense           ,      TGPassportElementTypeDriverLicense             );
      (PassportElementTypeEmailAddress           ,     EmailAddress            ,      TGPassportElementTypeEmailAddress              );
      (PassportElementTypeIdentityCard           ,     IdentityCard            ,      TGPassportElementTypeIdentityCard              );
      (PassportElementTypeInternalPassport       ,     InternalPassport        ,      TGPassportElementTypeInternalPassport          );
      (PassportElementTypePassport               ,     Passport                ,      TGPassportElementTypePassport                  );
      (PassportElementTypePassportRegistration   ,     PassportRegistration    ,      TGPassportElementTypePassportRegistration      );
      (PassportElementTypePersonalDetails        ,     PersonalDetails         ,      TGPassportElementTypePersonalDetails           );
      (PassportElementTypePhoneNumber            ,     PhoneNumber             ,      TGPassportElementTypePhoneNumber               );
      (PassportElementTypeRentalAgreement        ,     RentalAgreement         ,      TGPassportElementTypeRentalAgreement           );
      (PassportElementTypeTemporaryRegistration  ,     TemporaryRegistration   ,      TGPassportElementTypeTemporaryRegistration     );
      (PassportElementTypeUtilityBill            ,     UtilityBill             ,      TGPassportElementTypeUtilityBill               );
    )(td)
  }


  pub fn is_address(&self) -> bool { tuple_enum_is!(TGPassportElementType, Address               )(self) }
  pub fn is_bank_statement(&self) -> bool { tuple_enum_is!(TGPassportElementType, BankStatement         )(self) }
  pub fn is_driver_license(&self) -> bool { tuple_enum_is!(TGPassportElementType, DriverLicense         )(self) }
  pub fn is_email_address(&self) -> bool { tuple_enum_is!(TGPassportElementType, EmailAddress          )(self) }
  pub fn is_identity_card(&self) -> bool { tuple_enum_is!(TGPassportElementType, IdentityCard          )(self) }
  pub fn is_internal_passport(&self) -> bool { tuple_enum_is!(TGPassportElementType, InternalPassport      )(self) }
  pub fn is_passport(&self) -> bool { tuple_enum_is!(TGPassportElementType, Passport              )(self) }
  pub fn is_passport_registration(&self) -> bool { tuple_enum_is!(TGPassportElementType, PassportRegistration  )(self) }
  pub fn is_personal_details(&self) -> bool { tuple_enum_is!(TGPassportElementType, PersonalDetails       )(self) }
  pub fn is_phone_number(&self) -> bool { tuple_enum_is!(TGPassportElementType, PhoneNumber           )(self) }
  pub fn is_rental_agreement(&self) -> bool { tuple_enum_is!(TGPassportElementType, RentalAgreement       )(self) }
  pub fn is_temporary_registration(&self) -> bool { tuple_enum_is!(TGPassportElementType, TemporaryRegistration )(self) }
  pub fn is_utility_bill(&self) -> bool { tuple_enum_is!(TGPassportElementType, UtilityBill           )(self) }


  pub fn on_address<F: FnOnce(&TGPassportElementTypeAddress)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGPassportElementType, Address                , |t| fnc(t))(self);
    self
  }
  pub fn on_bank_statement<F: FnOnce(&TGPassportElementTypeBankStatement)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGPassportElementType, BankStatement          , |t| fnc(t))(self);
    self
  }
  pub fn on_driver_license<F: FnOnce(&TGPassportElementTypeDriverLicense)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGPassportElementType, DriverLicense          , |t| fnc(t))(self);
    self
  }
  pub fn on_email_address<F: FnOnce(&TGPassportElementTypeEmailAddress)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGPassportElementType, EmailAddress           , |t| fnc(t))(self);
    self
  }
  pub fn on_identity_card<F: FnOnce(&TGPassportElementTypeIdentityCard)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGPassportElementType, IdentityCard           , |t| fnc(t))(self);
    self
  }
  pub fn on_internal_passport<F: FnOnce(&TGPassportElementTypeInternalPassport)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGPassportElementType, InternalPassport       , |t| fnc(t))(self);
    self
  }
  pub fn on_passport<F: FnOnce(&TGPassportElementTypePassport)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGPassportElementType, Passport               , |t| fnc(t))(self);
    self
  }
  pub fn on_passport_registration<F: FnOnce(&TGPassportElementTypePassportRegistration)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGPassportElementType, PassportRegistration   , |t| fnc(t))(self);
    self
  }
  pub fn on_personal_details<F: FnOnce(&TGPassportElementTypePersonalDetails)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGPassportElementType, PersonalDetails        , |t| fnc(t))(self);
    self
  }
  pub fn on_phone_number<F: FnOnce(&TGPassportElementTypePhoneNumber)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGPassportElementType, PhoneNumber            , |t| fnc(t))(self);
    self
  }
  pub fn on_rental_agreement<F: FnOnce(&TGPassportElementTypeRentalAgreement)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGPassportElementType, RentalAgreement        , |t| fnc(t))(self);
    self
  }
  pub fn on_temporary_registration<F: FnOnce(&TGPassportElementTypeTemporaryRegistration)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGPassportElementType, TemporaryRegistration  , |t| fnc(t))(self);
    self
  }
  pub fn on_utility_bill<F: FnOnce(&TGPassportElementTypeUtilityBill)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGPassportElementType, UtilityBill            , |t| fnc(t))(self);
    self
  }
}


impl TGPassportElementTypeAddress {}

impl TGPassportElementTypeBankStatement {}

impl TGPassportElementTypeDriverLicense {}

impl TGPassportElementTypeEmailAddress {}

impl TGPassportElementTypeIdentityCard {}

impl TGPassportElementTypeInternalPassport {}

impl TGPassportElementTypePassport {}

impl TGPassportElementTypePassportRegistration {}

impl TGPassportElementTypePersonalDetails {}

impl TGPassportElementTypePhoneNumber {}

impl TGPassportElementTypeRentalAgreement {}

impl TGPassportElementTypeTemporaryRegistration {}

impl TGPassportElementTypeUtilityBill {}


impl TGEncryptedCredentials {
  pub fn data(&self) -> String { self.td_origin().data().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn hash(&self) -> String { self.td_origin().hash().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn secret(&self) -> Option<String> { self.td_origin().secret() }
}

impl TGDatedFile {
  pub fn file(&self) -> TGFile { self.td_origin().file().map(|v| TGFile::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn date(&self) -> i32 { self.td_origin().date().expect(errors::TELEGRAM_DATA_FAIL) }
}


impl TGMessagePassportDataSent {
  pub fn types(&self) -> Vec<TGPassportElementType> {
    self.td_origin().types().map_or(vec![], |v| v.iter()
      .map(|v| TGPassportElementType::of(v.clone()))
      .collect::<Vec<TGPassportElementType>>(),
    )
  }
}

impl TGMessagePaymentSuccessful {
  pub fn invoice_message_id(&self) -> Option<i64> { self.td_origin().invoice_message_id() }

  pub fn currency(&self) -> Option<String> { self.td_origin().currency() }

  pub fn total_amount(&self) -> Option<i64> { self.td_origin().total_amount() }
}

impl TGMessagePaymentSuccessfulBot {
  pub fn invoice_message_id(&self) -> i64 { self.td_origin().invoice_message_id().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn currency(&self) -> Option<String> { self.td_origin().currency() }

  pub fn total_amount(&self) -> Option<i64> { self.td_origin().total_amount() }

  pub fn invoice_payload(&self) -> Option<String> { self.td_origin().invoice_payload() }

  pub fn shipping_option_id(&self) -> Option<String> { self.td_origin().shipping_option_id() }

  pub fn order_info(&self) -> Option<TGOrderInfo> { self.td_origin().order_info().map(|v| TGOrderInfo::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn telegram_payment_charge_id(&self) -> Option<String> { self.td_origin().telegram_payment_charge_id() }

  pub fn provider_payment_charge_id(&self) -> Option<String> { self.td_origin().provider_payment_charge_id() }
}


impl TGOrderInfo {
  pub fn name(&self) -> String { self.td_origin().name().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn phone_number(&self) -> String { self.td_origin().phone_number().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn email_address(&self) -> Option<String> { self.td_origin().email_address() }

  pub fn shipping_address(&self) -> Option<TGAddress> { self.td_origin().shipping_address().map(|v| TGAddress::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }
}


impl TGAddress {
  pub fn country_code(&self) -> Option<String> { self.td_origin().country_code() }

  pub fn state(&self) -> Option<String> { self.td_origin().state() }

  pub fn city(&self) -> Option<String> { self.td_origin().city() }

  pub fn street_line1(&self) -> Option<String> { self.td_origin().street_line1() }

  pub fn street_line2(&self) -> Option<String> { self.td_origin().street_line2() }

  pub fn postal_code(&self) -> Option<String> { self.td_origin().postal_code() }
}


impl TGMessagePhoto {
  pub fn photo(&self) -> TGPhoto { self.td_origin().photo().map(|v| TGPhoto::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn caption(&self) -> Option<TGFormattedText> { self.td_origin().caption().map(|v| TGFormattedText::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn is_secret(&self) -> bool { self.td_origin().is_secret().map_or(false, |v| v) }
}

impl TGMessagePinMessage {
  pub fn message_id(&self) -> i64 { self.td_origin().message_id().expect(errors::TELEGRAM_DATA_FAIL) }
}


impl TGMessagePoll {
  pub fn poll(&self) -> TGPoll { self.td_origin().poll().map(|v| TGPoll::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }
}


impl TGPoll {
  pub fn id(&self) -> i64 { self.td_origin().id().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn question(&self) -> String { self.td_origin().question().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn options(&self) -> Vec<TGPollOption> {
    self.td_origin().options().map_or(vec![], |v| v.iter()
      .map(|v| TGPollOption::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL))
      .collect::<Vec<TGPollOption>>())
  }

  pub fn total_voter_count(&self) -> i32 { self.td_origin().total_voter_count().map_or(0, |v| v) }

  pub fn is_closed(&self) -> bool { self.td_origin().is_closed().map_or(false, |v| v) }
}

impl TGPollOption {
  pub fn text(&self) -> String { self.td_origin().text().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn voter_count(&self) -> i32 { self.td_origin().voter_count().map_or(0, |v| v) }

  pub fn vote_percentage(&self) -> i32 { self.td_origin().vote_percentage().map_or(0, |v| v) }

  pub fn is_chosen(&self) -> bool { self.td_origin().is_chosen().map_or(false, |v| v) }

  pub fn is_being_chosen(&self) -> bool { self.td_origin().is_being_chosen().map_or(false, |v| v) }
}


impl TGMessageScreenshotTaken {}

impl TGMessageSticker {}


impl TGSticker {
  pub fn set_id(&self) -> i64 { self.td_origin().set_id().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn width(&self) -> i32 { self.td_origin().width().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn height(&self) -> i32 { self.td_origin().height().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn emoji(&self) -> Option<String> { self.td_origin().emoji() }

  pub fn is_mask(&self) -> bool { self.td_origin().is_mask().map_or(false, |v| v) }

  pub fn mask_position(&self) -> Option<TGMaskPosition> { self.td_origin().mask_position().map(|v| TGMaskPosition::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn thumbnail(&self) -> Option<TGPhotoSize> { self.td_origin().thumbnail().map(|v| TGPhotoSize::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn sticker(&self) -> TGFile { self.td_origin().sticker().map(|v| TGFile::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGMaskPosition {
  pub fn point(&self) -> TGMaskPoint { self.td_origin().point().map(|v| TGMaskPoint::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn x_shift(&self) -> f64 { self.td_origin().x_shift().map_or(0 as f64, |v| v) }

  pub fn y_shift(&self) -> f64 { self.td_origin().y_shift().map_or(0 as f64, |v| v) }

  pub fn scale(&self) -> f64 { self.td_origin().scale().map_or(0 as f64, |v| v) }


  pub fn is_chin(&self) -> bool { self.point().is_chin() }
  pub fn is_eyes(&self) -> bool { self.point().is_eyes() }
  pub fn is_forehead(&self) -> bool { self.point().is_forehead() }
  pub fn is_mouth(&self) -> bool { self.point().is_mouth() }
}


#[derive(Debug, Clone)]
pub enum TGMaskPoint {
  Chin(TGMaskPointChin),
  Eyes(TGMaskPointEyes),
  Forehead(TGMaskPointForehead),
  Mouth(TGMaskPointMouth),
}

impl TGMaskPoint {
  fn of(td: Box<MaskPoint>) -> Self {
    tuple_rtd_type_mapping!(
      MaskPoint,
      TGMaskPoint,
      RTDMaskPointType,
      (MaskPointChin      ,     Chin          ,      TGMaskPointChin        );
      (MaskPointEyes      ,     Eyes          ,      TGMaskPointEyes        );
      (MaskPointForehead  ,     Forehead      ,      TGMaskPointForehead    );
      (MaskPointMouth     ,     Mouth         ,      TGMaskPointMouth       );
    )(td)
  }

  pub fn is_chin(&self) -> bool { tuple_enum_is!(TGMaskPoint  ,     Chin       )(self) }
  pub fn is_eyes(&self) -> bool { tuple_enum_is!(TGMaskPoint  ,     Eyes       )(self) }
  pub fn is_forehead(&self) -> bool { tuple_enum_is!(TGMaskPoint  ,     Forehead   )(self) }
  pub fn is_mouth(&self) -> bool { tuple_enum_is!(TGMaskPoint  ,     Mouth      )(self) }


  pub fn on_chin<F: FnOnce(&TGMaskPointChin)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMaskPoint  ,     Chin        , |t| fnc(t))(self);
    self
  }
  pub fn on_eyes<F: FnOnce(&TGMaskPointEyes)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMaskPoint  ,     Eyes        , |t| fnc(t))(self);
    self
  }
  pub fn on_forehead<F: FnOnce(&TGMaskPointForehead)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMaskPoint  ,     Forehead    , |t| fnc(t))(self);
    self
  }
  pub fn on_mouth<F: FnOnce(&TGMaskPointMouth)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGMaskPoint  ,     Mouth       , |t| fnc(t))(self);
    self
  }
}

impl TGMaskPointChin {}

impl TGMaskPointEyes {}

impl TGMaskPointForehead {}

impl TGMaskPointMouth {}


impl TGMessageSupergroupChatCreate {
  pub fn title(&self) -> String { self.td_origin().title().expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGMessageText {
  pub fn text(&self) -> TGFormattedText { self.td_origin().text().map(|v| TGFormattedText::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn web_page(&self) -> Option<TGWebPage> { self.td_origin().web_page().map(|v| TGWebPage::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }
}


impl TGWebPage {
  pub fn url(&self) -> String { self.td_origin().url().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn display_url(&self) -> String { self.td_origin().display_url().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn type_(&self) -> String { self.td_origin().type_().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn site_name(&self) -> String { self.td_origin().site_name().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn title(&self) -> String { self.td_origin().title().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn description(&self) -> Option<String> { self.td_origin().description() }

  pub fn photo(&self) -> Option<TGPhoto> { self.td_origin().photo().map(|v| TGPhoto::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn embed_url(&self) -> String { self.td_origin().embed_url().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn embed_type(&self) -> String { self.td_origin().embed_type().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn embed_width(&self) -> i32 { self.td_origin().embed_width().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn embed_height(&self) -> i32 { self.td_origin().embed_height().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn duration(&self) -> i32 { self.td_origin().duration().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn author(&self) -> Option<String> { self.td_origin().author() }

  pub fn animation(&self) -> Option<TGAnimation> { self.td_origin().animation().map(|v| TGAnimation::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn audio(&self) -> Option<TGAudio> { self.td_origin().audio().map(|v| TGAudio::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn document(&self) -> Option<TGDocument> { self.td_origin().document().map(|v| TGDocument::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn sticker(&self) -> Option<TGSticker> { self.td_origin().sticker().map(|v| TGSticker::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn video(&self) -> Option<TGVideo> { self.td_origin().video().map(|v| TGVideo::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn video_note(&self) -> Option<TGVideoNote> { self.td_origin().video_note().map(|v| TGVideoNote::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn voice_note(&self) -> Option<TGVoiceNote> { self.td_origin().voice_note().map(|v| TGVoiceNote::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn instant_view_version(&self) -> i32 { self.td_origin().instant_view_version().map_or(0, |v| v) }
}

impl TGVideo {
  pub fn duration(&self) -> i32 { self.td_origin().duration().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn width(&self) -> i32 { self.td_origin().width().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn height(&self) -> i32 { self.td_origin().height().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn file_name(&self) -> String { self.td_origin().file_name().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn mime_type(&self) -> String { self.td_origin().mime_type().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn has_stickers(&self) -> bool { self.td_origin().has_stickers().map_or(false, |v| v) }

  pub fn supports_streaming(&self) -> bool { self.td_origin().supports_streaming().map_or(false, |v| v) }

  pub fn thumbnail(&self) -> Option<TGPhotoSize> { self.td_origin().thumbnail().map(|v| TGPhotoSize::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn video(&self) -> TGFile { self.td_origin().video().map(|v| TGFile::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGVideoNote {
  pub fn duration(&self) -> i32 { self.td_origin().duration().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn length(&self) -> i32 { self.td_origin().length().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn thumbnail(&self) -> Option<TGPhotoSize> { self.td_origin().thumbnail().map(|v| TGPhotoSize::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn video(&self) -> TGFile { self.td_origin().video().map(|v| TGFile::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGVoiceNote {
  pub fn duration(&self) -> i32 { self.td_origin().duration().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn waveform(&self) -> String { self.td_origin().waveform().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn mime_type(&self) -> String { self.td_origin().mime_type().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn voice(&self) -> TGFile { self.td_origin().voice().map(|v| TGFile::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }
}


impl TGMessageUnsupported {}

impl TGMessageVenue {
  pub fn venue(&self) -> TGVenue { self.td_origin().venue().map(|v| TGVenue::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGVenue {
  pub fn location(&self) -> TGLocation { self.td_origin().location().map(|v| TGLocation::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn title(&self) -> Option<String> { self.td_origin().title() }

  pub fn address(&self) -> Option<String> { self.td_origin().address() }

  pub fn provider(&self) -> Option<String> { self.td_origin().provider() }

  pub fn id(&self) -> Option<String> { self.td_origin().id() }

  pub fn type_(&self) -> Option<String> { self.td_origin().type_() }
}

impl TGMessageVideo {
  pub fn video(&self) -> TGVideo { self.td_origin().video().map(|v| TGVideo::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn caption(&self) -> Option<TGFormattedText> { self.td_origin().caption().map(|v| TGFormattedText::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn is_secret(&self) -> bool { self.td_origin().is_secret().map_or(false, |v| v) }
}

impl TGMessageVideoNote {
  pub fn video_note(&self) -> TGVideoNote { self.td_origin().video_note().map(|v| TGVideoNote::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn is_viewed(&self) -> bool { self.td_origin().is_viewed().map_or(false, |v| v) }

  pub fn is_secret(&self) -> bool { self.td_origin().is_secret().map_or(false, |v| v) }
}


impl TGMessageVoiceNote {
  pub fn voice_note(&self) -> TGVoiceNote { self.td_origin().voice_note().map(|v| TGVoiceNote::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn caption(&self) -> Option<TGFormattedText> { self.td_origin().caption().map(|v| TGFormattedText::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn is_listened(&self) -> bool { self.td_origin().is_listened().map_or(false, |v| v) }
}

impl TGMessageWebsiteConnected {
  pub fn domain_name(&self) -> String { self.td_origin().domain_name().expect(errors::TELEGRAM_DATA_FAIL) }
}
