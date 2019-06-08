use rtdlib::types as td_types;
use rtdlib::types::{InputMessageContent, RObject};

use crate::errors;
use crate::types::f_input_file::*;
use crate::types::t_input_file::*;
use crate::types::t_input_message::*;
use crate::types::TGInvoice;
use crate::types::TGTextEntityType;

#[derive(Debug, Clone)]
pub enum TGInputMessageContent {
  Animation(TGInputMessageAnimation),
  Audio(TGInputMessageAudio),
  Contact(TGInputMessageContact),
  Document(TGInputMessageDocument),
  Forwarded(TGInputMessageForwarded),
  Game(TGInputMessageGame),
  Invoice(TGInputMessageInvoice),
  Location(TGInputMessageLocation),
  Photo(TGInputMessagePhoto),
  Poll(TGInputMessagePoll),
  Sticker(TGInputMessageSticker),
  Text(TGInputMessageText),
  Venue(TGInputMessageVenue),
  Video(TGInputMessageVideo),
  VideoNote(TGInputMessageVideoNote),
  VoiceNote(TGInputMessageVoiceNote),
}


impl TGInputMessageContent {
  pub(crate) fn of(td: Box<InputMessageContent>) -> Self {
//    macro_rules! type_mapping {
//      ($(($td_type:ident, $tg_type:ident, $tg_clz:ident));*;) => {
//        match td_types::RTDInputMessageContentType::of(td.td_name()) {
//        $(
//          Some(td_types::RTDInputMessageContentType::$td_type) => TGInputMessageContent::$tg_type($tg_clz::from_json(td.to_json()).expect(errors::TELEGRAM_DATA_FAIL)),
//        )*
//          None => panic!(errors::TELEGRAM_DATA_FAIL)
//        }
//      };
//    }
    tuple_rtd_type_mapping!(
      InputMessageContent,
      TGInputMessageContent,
      RTDInputMessageContentType,
      (InputMessageAnimation,  Animation,  TGInputMessageAnimation);
      (InputMessageAudio    ,  Audio    ,  TGInputMessageAudio    );
      (InputMessageContact  ,  Contact  ,  TGInputMessageContact  );
      (InputMessageDocument ,  Document ,  TGInputMessageDocument );
      (InputMessageForwarded,  Forwarded,  TGInputMessageForwarded);
      (InputMessageGame     ,  Game     ,  TGInputMessageGame     );
      (InputMessageInvoice  ,  Invoice  ,  TGInputMessageInvoice  );
      (InputMessageLocation ,  Location ,  TGInputMessageLocation );
      (InputMessagePhoto    ,  Photo    ,  TGInputMessagePhoto    );
      (InputMessagePoll     ,  Poll     ,  TGInputMessagePoll     );
      (InputMessageSticker  ,  Sticker  ,  TGInputMessageSticker  );
      (InputMessageText     ,  Text     ,  TGInputMessageText     );
      (InputMessageVenue    ,  Venue    ,  TGInputMessageVenue    );
      (InputMessageVideo    ,  Video    ,  TGInputMessageVideo    );
      (InputMessageVideoNote,  VideoNote,  TGInputMessageVideoNote);
      (InputMessageVoiceNote,  VoiceNote,  TGInputMessageVoiceNote);
    )(td)
  }


//  animation
//  audio
//  contact
//  document
//  forwarded
//  game
//  invoice
//  location
//  photo
//  poll
//  sticker
//  text
//  venue
//  video
//  video_note
//  voice_note


  pub fn is_animation(&self) -> bool { tuple_enum_is!(TGInputMessageContent, Animation)(self) }
  pub fn is_audio(&self) -> bool { tuple_enum_is!(TGInputMessageContent, Audio    )(self) }
  pub fn is_contact(&self) -> bool { tuple_enum_is!(TGInputMessageContent, Contact  )(self) }
  pub fn is_document(&self) -> bool { tuple_enum_is!(TGInputMessageContent, Document )(self) }
  pub fn is_forwarded(&self) -> bool { tuple_enum_is!(TGInputMessageContent, Forwarded)(self) }
  pub fn is_game(&self) -> bool { tuple_enum_is!(TGInputMessageContent, Game     )(self) }
  pub fn is_invoice(&self) -> bool { tuple_enum_is!(TGInputMessageContent, Invoice  )(self) }
  pub fn is_location(&self) -> bool { tuple_enum_is!(TGInputMessageContent, Location )(self) }
  pub fn is_photo(&self) -> bool { tuple_enum_is!(TGInputMessageContent, Photo    )(self) }
  pub fn is_poll(&self) -> bool { tuple_enum_is!(TGInputMessageContent, Poll     )(self) }
  pub fn is_sticker(&self) -> bool { tuple_enum_is!(TGInputMessageContent, Sticker  )(self) }
  pub fn is_text(&self) -> bool { tuple_enum_is!(TGInputMessageContent, Text     )(self) }
  pub fn is_venue(&self) -> bool { tuple_enum_is!(TGInputMessageContent, Venue    )(self) }
  pub fn is_video(&self) -> bool { tuple_enum_is!(TGInputMessageContent, Video    )(self) }
  pub fn is_video_note(&self) -> bool { tuple_enum_is!(TGInputMessageContent, VideoNote)(self) }
  pub fn is_voice_note(&self) -> bool { tuple_enum_is!(TGInputMessageContent, VoiceNote)(self) }


  pub fn on_animation<F: FnOnce(&TGInputMessageAnimation)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGInputMessageContent, Animation, |t| fnc(t))(self);
    self
  }
  pub fn on_audio<F: FnOnce(&TGInputMessageAudio)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGInputMessageContent, Audio    , |t| fnc(t))(self);
    self
  }
  pub fn on_contact<F: FnOnce(&TGInputMessageContact)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGInputMessageContent, Contact  , |t| fnc(t))(self);
    self
  }
  pub fn on_document<F: FnOnce(&TGInputMessageDocument)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGInputMessageContent, Document , |t| fnc(t))(self);
    self
  }
  pub fn on_forwarded<F: FnOnce(&TGInputMessageForwarded)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGInputMessageContent, Forwarded, |t| fnc(t))(self);
    self
  }
  pub fn on_game<F: FnOnce(&TGInputMessageGame)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGInputMessageContent, Game     , |t| fnc(t))(self);
    self
  }
  pub fn on_invoice<F: FnOnce(&TGInputMessageInvoice)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGInputMessageContent, Invoice  , |t| fnc(t))(self);
    self
  }
  pub fn on_location<F: FnOnce(&TGInputMessageLocation)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGInputMessageContent, Location , |t| fnc(t))(self);
    self
  }
  pub fn on_photo<F: FnOnce(&TGInputMessagePhoto)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGInputMessageContent, Photo    , |t| fnc(t))(self);
    self
  }
  pub fn on_poll<F: FnOnce(&TGInputMessagePoll)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGInputMessageContent, Poll     , |t| fnc(t))(self);
    self
  }
  pub fn on_sticker<F: FnOnce(&TGInputMessageSticker)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGInputMessageContent, Sticker  , |t| fnc(t))(self);
    self
  }
  pub fn on_text<F: FnOnce(&TGInputMessageText)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGInputMessageContent, Text     , |t| fnc(t))(self);
    self
  }
  pub fn on_venue<F: FnOnce(&TGInputMessageVenue)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGInputMessageContent, Venue    , |t| fnc(t))(self);
    self
  }
  pub fn on_video<F: FnOnce(&TGInputMessageVideo)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGInputMessageContent, Video    , |t| fnc(t))(self);
    self
  }
  pub fn on_video_note<F: FnOnce(&TGInputMessageVideoNote)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGInputMessageContent, VideoNote, |t| fnc(t))(self);
    self
  }
  pub fn on_voice_note<F: FnOnce(&TGInputMessageVoiceNote)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGInputMessageContent, VoiceNote, |t| fnc(t))(self);
    self
  }
}


impl TGInputMessageAnimation {
  pub fn animation(&self) -> TGInputFile { self.td_origin().animation().map(|v| TGInputFile::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn thumbnail(&self) -> TGInputThumbnail {
    self.td_origin().thumbnail().map(|v| TGInputThumbnail::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL))
      .expect(errors::TELEGRAM_DATA_FAIL)
  }

  pub fn duration(&self) -> Option<i32> { self.td_origin().duration() }

  pub fn width(&self) -> i32 { self.td_origin().width().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn height(&self) -> i32 { self.td_origin().height().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn caption(&self) -> Option<TGFormattedText> { self.td_origin().caption().map(|v| TGFormattedText::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }
}

impl TGInputMessageAudio {
  pub fn audio(&self) -> TGInputFile { self.td_origin().audio().map(|v| TGInputFile::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn album_cover_thumbnail(&self) -> TGInputThumbnail {
    self.td_origin().album_cover_thumbnail().map(|v| TGInputThumbnail::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL))
      .expect(errors::TELEGRAM_DATA_FAIL)
  }

  pub fn duration(&self) -> Option<i32> { self.td_origin().duration() }

  pub fn title(&self) -> Option<String> { self.td_origin().title() }

  pub fn performer(&self) -> Option<String> { self.td_origin().performer() }

  pub fn caption(&self) -> Option<TGFormattedText> { self.td_origin().caption().map(|v| TGFormattedText::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }
}

impl TGInputMessageContact {
  pub fn contact(&self) -> Option<td_types::Contact> { self.td_origin().contact() }
}

impl TGInputMessageDocument {
  pub fn document(&self) -> TGInputFile { self.td_origin().document().map(|v| TGInputFile::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn thumbnail(&self) -> Option<TGInputThumbnail> { self.td_origin().thumbnail().map(|v| TGInputThumbnail::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn caption(&self) -> Option<TGFormattedText> { self.td_origin().caption().map(|v| TGFormattedText::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }
}

impl TGInputMessageForwarded {
  pub fn from_chat_id(&self) -> i64 { self.td_origin().from_chat_id().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn message_id(&self) -> i64 { self.td_origin().message_id().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn in_game_share(&self) -> bool { self.td_origin().in_game_share().map_or(false, |v| v) }
}

impl TGInputMessageGame {
  pub fn bot_user_id(&self) -> i32 { self.td_origin().bot_user_id().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn game_short_name(&self) -> String { self.td_origin().game_short_name().expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGInputMessageInvoice {
  pub fn invoice(&self) -> TGInvoice { self.td_origin().invoice().map(|v| TGInvoice::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn title(&self) -> Option<String> { self.td_origin().title() }

  pub fn description(&self) -> Option<String> { self.td_origin().description() }

  pub fn photo_url(&self) -> Option<String> { self.td_origin().photo_url() }

  pub fn photo_size(&self) -> Option<i32> { self.td_origin().photo_size() }

  pub fn photo_width(&self) -> Option<i32> { self.td_origin().photo_height() }

  pub fn photo_height(&self) -> Option<i32> { self.td_origin().photo_height() }

  pub fn payload(&self) -> Option<String> { self.td_origin().payload() }

  pub fn provider_token(&self) -> Option<String> { self.td_origin().provider_token() }

  pub fn provider_data(&self) -> Option<String> { self.td_origin().provider_data() }

  pub fn start_parameter(&self) -> Option<String> { self.td_origin().start_parameter() }
}

impl TGInputMessageLocation {
  pub fn location(&self) -> td_types::Location { self.td_origin().location().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn live_period(&self) -> Option<i32> { self.td_origin().live_period() }
}

impl TGInputMessagePhoto {
  pub fn photo(&self) -> TGInputFile { self.td_origin().photo().map(|v| TGInputFile::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn thumbnail(&self) -> TGInputThumbnail {
    self.td_origin().thumbnail().map(|v| TGInputThumbnail::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL))
      .expect(errors::TELEGRAM_DATA_FAIL)
  }

  pub fn added_sticker_file_ids(&self) -> Vec<i32> { self.td_origin().added_sticker_file_ids().map_or(vec![], |v| v) }

  pub fn width(&self) -> i32 { self.td_origin().width().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn height(&self) -> i32 { self.td_origin().height().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn caption(&self) -> Option<TGFormattedText> { self.td_origin().caption().map(|v| TGFormattedText::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn ttl(&self) -> Option<i32> { self.td_origin().ttl() }
}

impl TGInputMessagePoll {
  pub fn question(&self) -> String { self.td_origin().question().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn options(&self) -> Vec<String> { self.td_origin().options().map_or(vec![], |v| v) }
}

impl TGInputMessageSticker {
  pub fn sticker(&self) -> TGInputFile { self.td_origin().sticker().map(|v| TGInputFile::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn thumbnail(&self) -> TGInputThumbnail {
    self.td_origin().thumbnail().map(|v| TGInputThumbnail::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL))
      .expect(errors::TELEGRAM_DATA_FAIL)
  }

  pub fn width(&self) -> i32 { self.td_origin().width().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn height(&self) -> i32 { self.td_origin().height().expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGInputMessageText {
  pub fn text(&self) -> TGFormattedText { self.td_origin().text().map(|v| TGFormattedText::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn disable_web_page_preview(&self) -> bool { self.td_origin().disable_web_page_preview().map_or(false, |v| v) }

  pub fn clear_draft(&self) -> bool { self.td_origin().clear_draft().map_or(false, |v| v) }
}

impl TGInputMessageVenue {
  pub fn venue(&self) -> td_types::Venue { self.td_origin().venue().expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGInputMessageVideo {
  pub fn video(&self) -> TGInputFile { self.td_origin().video().map(|v| TGInputFile::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn thumbnail(&self) -> TGInputThumbnail { self.td_origin().thumbnail().map(|v| TGInputThumbnail::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn added_sticker_file_ids(&self) -> Vec<i32> { self.td_origin().added_sticker_file_ids().map_or(vec![], |v| v) }

  pub fn duration(&self) -> Option<i32> { self.td_origin().duration() }

  pub fn width(&self) -> i32 { self.td_origin().width().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn height(&self) -> i32 { self.td_origin().height().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn supports_streaming(&self) -> bool { self.td_origin().supports_streaming().map_or(false, |v| v) }

  pub fn caption(&self) -> Option<TGFormattedText> { self.td_origin().caption().map(|v| TGFormattedText::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn ttl(&self) -> Option<i32> { self.td_origin().ttl() }
}

impl TGInputMessageVideoNote {
  pub fn video_note(&self) -> TGInputFile { self.td_origin().video_note().map(|v| TGInputFile::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn thumbnail(&self) -> TGInputThumbnail { self.td_origin().thumbnail().map(|v| TGInputThumbnail::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn duration(&self) -> Option<i32> { self.td_origin().duration() }

  pub fn length(&self) -> i32 { self.td_origin().length().expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGInputMessageVoiceNote {
  pub fn voice_note(&self) -> TGInputFile { self.td_origin().voice_note().map(|v| TGInputFile::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn duration(&self) -> Option<i32> { self.td_origin().duration() }

  pub fn waveform(&self) -> Option<String> { self.td_origin().waveform() }

  pub fn caption(&self) -> Option<TGFormattedText> { self.td_origin().caption().map(|v| TGFormattedText::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }
}

impl TGFormattedText {
  pub fn text(&self) -> String { self.td_origin().text().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn entities(&self) -> Vec<TGTextEntity> {
    self.td_origin().entities().map(|v| v.iter()
      .map(|v| TGTextEntity::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL))
      .collect::<Vec<TGTextEntity>>()
    )
      .map_or(vec![], |v| v)
  }
}

impl TGTextEntity {
  pub fn offset(&self) -> i32 { self.td_origin().offset().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn length(&self) -> i32 { self.td_origin().length().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn type_(&self) -> TGTextEntityType { self.td_origin().type_().map(|v| TGTextEntityType::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }
}




