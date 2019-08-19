use rtdlib::types::*;

use crate::api::*;
use crate::types::*;

impl _TGSendMessageBuilder {
  pub fn reply_markup<T: AsRef<TGReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
    self._reply_markup(reply_markup.as_ref().build());
    self
  }

  pub fn input_message_content<T: AsRef<TGInputMessageContent>>(&mut self, input_message_content: T) -> &Self {
    self._input_message_content(input_message_content.as_ref().build());
    self
  }
}


impl TGInputMessageContent {
  pub fn build(&self) -> Box<InputMessageContent> {
    match self {
      TGInputMessageContent::Animation(t) => Box::new(t.td_origin().clone()),
      TGInputMessageContent::Audio(t) => Box::new(t.td_origin().clone()),
      TGInputMessageContent::Contact(t) => Box::new(t.td_origin().clone()),
      TGInputMessageContent::Document(t) => Box::new(t.td_origin().clone()),
      TGInputMessageContent::Forwarded(t) => Box::new(t.td_origin().clone()),
      TGInputMessageContent::Game(t) => Box::new(t.td_origin().clone()),
      TGInputMessageContent::Invoice(t) => Box::new(t.td_origin().clone()),
      TGInputMessageContent::Location(t) => Box::new(t.td_origin().clone()),
      TGInputMessageContent::Photo(t) => Box::new(t.td_origin().clone()),
      TGInputMessageContent::Poll(t) => Box::new(t.td_origin().clone()),
      TGInputMessageContent::Sticker(t) => Box::new(t.td_origin().clone()),
      TGInputMessageContent::Text(t) => Box::new(t.td_origin().clone()),
      TGInputMessageContent::Venue(t) => Box::new(t.td_origin().clone()),
      TGInputMessageContent::Video(t) => Box::new(t.td_origin().clone()),
      TGInputMessageContent::VideoNote(t) => Box::new(t.td_origin().clone()),
      TGInputMessageContent::VoiceNote(t) => Box::new(t.td_origin().clone()),
    }
  }

  pub fn animation  <T: AsRef<TGInputMessageAnimation>>(animation: T)  ->  Self { TGInputMessageContent::Animation(animation.as_ref().clone())  }
  pub fn audio      <T: AsRef<TGInputMessageAudio>>(audio: T)          ->  Self { TGInputMessageContent::Audio(audio.as_ref().clone())          }
  pub fn contact    <T: AsRef<TGInputMessageContact>>(contact: T)      ->  Self { TGInputMessageContent::Contact(contact.as_ref().clone())      }
  pub fn document   <T: AsRef<TGInputMessageDocument>>(document: T)    ->  Self { TGInputMessageContent::Document(document.as_ref().clone())    }
  pub fn forwarded  <T: AsRef<TGInputMessageForwarded>>(forwarded: T)  ->  Self { TGInputMessageContent::Forwarded(forwarded.as_ref().clone())  }
  pub fn game       <T: AsRef<TGInputMessageGame>>(game: T)            ->  Self { TGInputMessageContent::Game(game.as_ref().clone())            }
  pub fn invoice    <T: AsRef<TGInputMessageInvoice>>(invoice: T)      ->  Self { TGInputMessageContent::Invoice(invoice.as_ref().clone())      }
  pub fn location   <T: AsRef<TGInputMessageLocation>>(location: T)    ->  Self { TGInputMessageContent::Location(location.as_ref().clone())    }
  pub fn photo      <T: AsRef<TGInputMessagePhoto>>(photo: T)          ->  Self { TGInputMessageContent::Photo(photo.as_ref().clone())          }
  pub fn poll       <T: AsRef<TGInputMessagePoll>>(poll: T)            ->  Self { TGInputMessageContent::Poll(poll.as_ref().clone())            }
  pub fn sticker    <T: AsRef<TGInputMessageSticker>>(sticker: T)      ->  Self { TGInputMessageContent::Sticker(sticker.as_ref().clone())      }
  pub fn text       <T: AsRef<TGInputMessageText>>(text: T)            ->  Self { TGInputMessageContent::Text(text.as_ref().clone())            }
  pub fn venue      <T: AsRef<TGInputMessageVenue>>(venue: T)          ->  Self { TGInputMessageContent::Venue(venue.as_ref().clone())          }
  pub fn video      <T: AsRef<TGInputMessageVideo>>(video: T)          ->  Self { TGInputMessageContent::Video(video.as_ref().clone())          }
  pub fn video_note <T: AsRef<TGInputMessageVideoNote>>(video_note: T) ->  Self { TGInputMessageContent::VideoNote(video_note.as_ref().clone()) }
  pub fn voice_note <T: AsRef<TGInputMessageVoiceNote>>(voice_note: T) ->  Self { TGInputMessageContent::VoiceNote(voice_note.as_ref().clone()) }
}

impl AsRef<TGInputMessageContent> for TGInputMessageContent {
  fn as_ref(&self) -> &TGInputMessageContent { self }
}

impl _TGInputMessageAnimationBuilder {
  pub fn animation<T: AsRef<TGInputFile>>(&mut self, animation: T) -> &mut Self {
    self._animation(animation.as_ref().build());
    self
  }

  pub fn thumbnail<T: AsRef<TGInputThumbnail>>(&mut self, thumbnail: T) -> &mut Self {
    self._thumbnail(thumbnail.as_ref().td_origin().clone());
    self
  }

  pub fn caption<T: AsRef<TGFormattedText>>(&mut self, caption: T) -> &mut Self {
    self._caption(caption.as_ref().td_origin().clone());
    self
  }
}

impl TGInputFile {
  pub fn build(&self) -> Box<InputFile> {
    match self {
      TGInputFile::Generated(t) => Box::new(t.td_origin().clone()),
      TGInputFile::Id(t) => Box::new(t.td_origin().clone()),
      TGInputFile::Local(t) => Box::new(t.td_origin().clone()),
      TGInputFile::Remote(t) => Box::new(t.td_origin().clone()),
    }
  }

  pub fn generated<T: AsRef<TGInputFileGenerated>>(generated: T) -> Self { TGInputFile::Generated(generated.as_ref().clone()) }
  pub fn id<T: AsRef<TGInputFileId>>(id: T) -> Self { TGInputFile::Id(id.as_ref().clone()) }
  pub fn local<T: AsRef<TGInputFileLocal>>(local: T) -> Self { TGInputFile::Local(local.as_ref().clone()) }
  pub fn remote<T: AsRef<TGInputFileRemote>>(remote: T) -> Self { TGInputFile::Remote(remote.as_ref().clone()) }
}

impl AsRef<TGInputFile> for TGInputFile {
  fn as_ref(&self) -> &TGInputFile { self }
}

impl _TGInputFileGeneratedBuilder {}

impl _TGInputFileIdBuilder {}

impl _TGInputFileLocalBuilder {}

impl _TGInputFileRemoteBuilder {}


impl _TGFormattedTextBuilder {
  pub fn entities<T: AsRef<TGTextEntity>>(&mut self, entities: Vec<T>) -> &mut Self {
    self._entities(entities.iter().map(|v| v.as_ref().td_origin().clone()).collect());
    self
  }
}

impl _TGTextEntityBuilder {
  pub fn bold(&mut self) -> &Self {
    self._type_(Box::new(TextEntityTypeBold::_new()));
    self
  }
  pub fn bot_command(&mut self) -> &Self {
    self._type_(Box::new(TextEntityTypeBotCommand::_new()));
    self
  }
  pub fn cashtag(&mut self) -> &Self {
    self._type_(Box::new(TextEntityTypeCashtag::_new()));
    self
  }
  pub fn code(&mut self) -> &Self {
    self._type_(Box::new(TextEntityTypeCode::_new()));
    self
  }
  pub fn email_address(&mut self) -> &Self {
    self._type_(Box::new(TextEntityTypeEmailAddress::_new()));
    self
  }
  pub fn hashtag(&mut self) -> &Self {
    self._type_(Box::new(TextEntityTypeHashtag::_new()));
    self
  }
  pub fn italic(&mut self) -> &Self {
    self._type_(Box::new(TextEntityTypeItalic::_new()));
    self
  }
  pub fn mention(&mut self) -> &Self {
    self._type_(Box::new(TextEntityTypeMention::_new()));
    self
  }
  pub fn mention_name(&mut self, user_id: i32) -> &Self {
    self._type_(Box::new(TextEntityTypeMentionName::_new()._set_user_id(user_id).clone()));
    self
  }
  pub fn phone_number(&mut self) -> &Self {
    self._type_(Box::new(TextEntityTypePhoneNumber::_new()));
    self
  }
  pub fn pre(&mut self) -> &Self {
    self._type_(Box::new(TextEntityTypePre::_new()));
    self
  }
  pub fn pre_code<S: AsRef<str>>(&mut self, language: S) -> &Self {
    self._type_(Box::new(TextEntityTypePreCode::_new()._set_language(language.as_ref().to_string()).clone()));
    self
  }
  pub fn text_url<S: AsRef<str>>(&mut self, url: S) -> &Self {
    self._type_(Box::new(TextEntityTypeTextUrl::_new()._set_url(url.as_ref().to_string()).clone()));
    self
  }
  pub fn url(&mut self) -> &Self {
    self._type_(Box::new(TextEntityTypeUrl::_new()));
    self
  }
}


impl _TGInputMessageAudioBuilder {
  pub fn audio<T: AsRef<TGInputFile>>(&mut self, audio: T) -> &mut Self {
    self._audio(audio.as_ref().build());
    self
  }

  pub fn album_cover_thumbnail<T: AsRef<TGInputThumbnail>>(&mut self, album_cover_thumbnail: T) -> &mut Self {
    self._album_cover_thumbnail(album_cover_thumbnail.as_ref().td_origin().clone());
    self
  }

  pub fn caption<T: AsRef<TGFormattedText>>(&mut self, caption: T) -> &mut Self {
    self._caption(caption.as_ref().td_origin().clone());
    self
  }
}

impl _TGInputMessageContactBuilder {
  pub fn contact<T: AsRef<TGContact>>(&mut self, contact: T) -> &mut Self {
    self._contact(contact.as_ref().td_origin().clone());
    self
  }
}

impl _TGContactBuilder {}

impl _TGInputMessageDocumentBuilder {
  pub fn document<T: AsRef<TGInputFile>>(&mut self, document: T) -> &mut Self {
    self._document(document.as_ref().build());
    self
  }

  pub fn thumbnail<T: AsRef<TGInputThumbnail>>(&mut self, thumbnail: T) -> &mut Self {
    self._thumbnail(thumbnail.as_ref().td_origin().clone());
    self
  }

  pub fn caption<T: AsRef<TGFormattedText>>(&mut self, caption: T) -> &mut Self {
    self._caption(caption.as_ref().td_origin().clone());
    self
  }
}

impl _TGInputMessageForwardedBuilder {}

impl _TGInputMessageGameBuilder {}

impl _TGInputMessageInvoiceBuilder {
  pub fn invoice<T: AsRef<TGInvoice>>(&mut self, invoice: T) -> &mut Self {
    self._invoice(invoice.as_ref().td_origin().clone());
    self
  }
}

impl _TGInvoiceBuilder {
  pub fn price_parts<T: AsRef<TGLabeledPricePart>>(&mut self, price_parts: Vec<T>) -> &mut Self {
    self._price_parts(price_parts.iter().map(|v| v.as_ref().td_origin().clone()).collect());
    self
  }
}

impl _TGInputMessageLocationBuilder {
  pub fn location<T: AsRef<TGLocation>>(&mut self, location: T) -> &mut Self {
    self._location(location.as_ref().td_origin().clone());
    self
  }
}

impl _TGLocationBuilder {}

impl _TGInputMessagePhotoBuilder {
  pub fn photo<T: AsRef<TGInputFile>>(&mut self, photo: T) -> &mut Self {
    self._photo(photo.as_ref().build());
    self
  }

  pub fn thumbnail<T: AsRef<TGInputThumbnail>>(&mut self, thumbnail: T) -> &mut Self {
    self._thumbnail(thumbnail.as_ref().td_origin().clone());
    self
  }

  pub fn caption<T: AsRef<TGFormattedText>>(&mut self, caption: T) -> &mut Self {
    self._caption(caption.as_ref().td_origin().clone());
    self
  }
}

impl _TGInputMessagePollBuilder {
  pub fn options<T: AsRef<str>>(&mut self, options: Vec<T>) -> &mut Self {
    self._options(options.iter().map(|v| v.as_ref().to_string()).collect());
    self
  }
}

impl _TGInputMessageStickerBuilder {
  pub fn sticker<T: AsRef<TGInputFile>>(&mut self, sticker: T) -> &mut Self {
    self._sticker(sticker.as_ref().build());
    self
  }

  pub fn thumbnail<T: AsRef<TGInputThumbnail>>(&mut self, thumbnail: T) -> &mut Self {
    self._thumbnail(thumbnail.as_ref().td_origin().clone());
    self
  }
}

impl _TGInputMessageTextBuilder {
  pub fn text<T: AsRef<TGFormattedText>>(&mut self, text: T) -> &mut Self {
    self._text(text.as_ref().td_origin().clone());
    self
  }
}

impl _TGInputMessageVenueBuilder {
  pub fn venue<T: AsRef<TGVenue>>(&mut self, venue: T) -> &mut Self {
    self._venue(venue.as_ref().td_origin().clone());
    self
  }
}

impl _TGVenueBuilder {
  pub fn location<T: AsRef<TGLocation>>(&mut self, location: T) -> &mut Self {
    self._location(location.as_ref().td_origin().clone());
    self
  }
}


impl _TGInputMessageVideoBuilder {
  pub fn video<T: AsRef<TGInputFile>>(&mut self, video: T) -> &mut Self {
    self._video(video.as_ref().build());
    self
  }

  pub fn thumbnail<T: AsRef<TGInputThumbnail>>(&mut self, thumbnail: T) -> &mut Self {
    self._thumbnail(thumbnail.as_ref().td_origin().clone());
    self
  }

  pub fn caption<T: AsRef<TGFormattedText>>(&mut self, caption: T) -> &mut Self {
    self._caption(caption.as_ref().td_origin().clone());
    self
  }
}

impl _TGInputMessageVideoNoteBuilder {
  pub fn video_note<T: AsRef<TGInputFile>>(&mut self, video_note: T) -> &mut Self {
    self._video_note(video_note.as_ref().build());
    self
  }

  pub fn thumbnail<T: AsRef<TGInputThumbnail>>(&mut self, thumbnail: T) -> &mut Self {
    self._thumbnail(thumbnail.as_ref().td_origin().clone());
    self
  }
}

impl _TGInputMessageVoiceNoteBuilder {
  pub fn voice_note<T: AsRef<TGInputFile>>(&mut self, voice_note: T) -> &mut Self {
    self._voice_note(voice_note.as_ref().build());
    self
  }

  pub fn caption<T: AsRef<TGFormattedText>>(&mut self, caption: T) -> &mut Self {
    self._caption(caption.as_ref().td_origin().clone());
    self
  }
}

impl TGReplyMarkup {
  pub fn build(&self) -> Box<ReplyMarkup> {
    match self {
      TGReplyMarkup::ForceReply(t) => Box::new(t.td_origin().clone()),
      TGReplyMarkup::InlineKeyboard(t) => Box::new(t.td_origin().clone()),
      TGReplyMarkup::RemoveKeyboard(t) => Box::new(t.td_origin().clone()),
      TGReplyMarkup::ShowKeyboard(t) => Box::new(t.td_origin().clone()),
    }
  }


  pub fn force_reply    <T: AsRef<TGReplyMarkupForceReply>>    (force_reply: T    )  -> Self { TGReplyMarkup::ForceReply(force_reply.as_ref().clone())         }
  pub fn inline_keyboard<T: AsRef<TGReplyMarkupInlineKeyboard>>(inline_keyboard: T)  -> Self { TGReplyMarkup::InlineKeyboard(inline_keyboard.as_ref().clone()) }
  pub fn remove_keyboard<T: AsRef<TGReplyMarkupRemoveKeyboard>>(remove_keyboard: T)  -> Self { TGReplyMarkup::RemoveKeyboard(remove_keyboard.as_ref().clone()) }
  pub fn show_keyboard  <T: AsRef<TGReplyMarkupShowKeyboard>>  (show_keyboard: T  )  -> Self { TGReplyMarkup::ShowKeyboard(show_keyboard.as_ref().clone())     }
}

impl AsRef<TGReplyMarkup> for TGReplyMarkup {
  fn as_ref(&self) -> &TGReplyMarkup { self }
}

impl _TGReplyMarkupForceReplyBuilder {}

impl _TGReplyMarkupInlineKeyboardBuilder {
  pub fn rows<T: AsRef<TGInlineKeyboardButton>>(&mut self, rows: Vec<Vec<T>>) -> &mut Self {
    self._rows(rows.iter().map(|v| v.iter().map(|v| v.as_ref().td_origin().clone()).collect::<Vec<InlineKeyboardButton>>()).collect::<Vec<Vec<InlineKeyboardButton>>>());
    self
  }
}

impl _TGReplyMarkupRemoveKeyboardBuilder {}

impl _TGReplyMarkupShowKeyboardBuilder {
  pub fn rows<T: AsRef<TGKeyboardButton>>(&mut self, rows: Vec<Vec<T>>) -> &mut Self {
    self._rows(rows.iter().map(|v| v.iter().map(|v| v.as_ref().td_origin().clone()).collect::<Vec<KeyboardButton>>()).collect::<Vec<Vec<KeyboardButton>>>());
    self
  }
}


impl _TGInlineKeyboardButtonBuilder {
  pub fn type_<T: AsRef<TGInlineKeyboardButtonType>>(&mut self, type_: T) -> &mut Self {
    self._type_(type_.as_ref().build());
    self
  }
}

impl TGInlineKeyboardButtonType {
  pub fn build(&self) -> Box<InlineKeyboardButtonType> {
    match self {
      TGInlineKeyboardButtonType::Buy(t) => Box::new(t.td_origin().clone()),
      TGInlineKeyboardButtonType::Callback(t) => Box::new(t.td_origin().clone()),
      TGInlineKeyboardButtonType::CallbackGame(t) => Box::new(t.td_origin().clone()),
      TGInlineKeyboardButtonType::SwitchInline(t) => Box::new(t.td_origin().clone()),
      TGInlineKeyboardButtonType::Url(t) => Box::new(t.td_origin().clone()),
    }
  }


  pub fn buy<T: AsRef<TGInlineKeyboardButtonTypeBuy>>(buy: T) -> Self { TGInlineKeyboardButtonType::Buy(buy.as_ref().clone()) }
  pub fn callback<T: AsRef<TGInlineKeyboardButtonTypeCallback>>(callback: T) -> Self { TGInlineKeyboardButtonType::Callback(callback.as_ref().clone()) }
  pub fn callback_game<T: AsRef<TGInlineKeyboardButtonTypeCallbackGame>>(callback_game: T) -> Self { TGInlineKeyboardButtonType::CallbackGame(callback_game.as_ref().clone()) }
  pub fn switch_inline<T: AsRef<TGInlineKeyboardButtonTypeSwitchInline>>(switch_inline: T) -> Self { TGInlineKeyboardButtonType::SwitchInline(switch_inline.as_ref().clone()) }
  pub fn url<T: AsRef<TGInlineKeyboardButtonTypeUrl>>(url: T) -> Self { TGInlineKeyboardButtonType::Url(url.as_ref().clone()) }
}

impl AsRef<TGInlineKeyboardButtonType> for TGInlineKeyboardButtonType {
  fn as_ref(&self) -> &TGInlineKeyboardButtonType { self }
}

impl _TGInlineKeyboardButtonTypeBuyBuilder {}

impl _TGInlineKeyboardButtonTypeCallbackBuilder {}

impl _TGInlineKeyboardButtonTypeCallbackGameBuilder {}

impl _TGInlineKeyboardButtonTypeSwitchInlineBuilder {}

impl _TGInlineKeyboardButtonTypeUrlBuilder {}


impl _TGKeyboardButtonBuilder {
  pub fn type_<T: AsRef<TGKeyboardButtonType>>(&mut self, type_: T) -> &mut Self {
    self._type_(type_.as_ref().build());
    self
  }
}

impl TGKeyboardButtonType {
  pub fn build(&self) -> Box<KeyboardButtonType> {
    match self {
      TGKeyboardButtonType::RequestLocation(t) => { Box::new(t.td_origin().clone()) }
      TGKeyboardButtonType::RequestPhoneNumber(t) => { Box::new(t.td_origin().clone()) }
      TGKeyboardButtonType::Text(t) => { Box::new(t.td_origin().clone()) }
    }
  }


  pub fn request_location<T: AsRef<TGKeyboardButtonTypeRequestLocation>>(request_location: T) -> Self { TGKeyboardButtonType::RequestLocation(request_location.as_ref().clone()) }
  pub fn request_phone_number<T: AsRef<TGKeyboardButtonTypeRequestPhoneNumber>>(request_phone_number: T) -> Self { TGKeyboardButtonType::RequestPhoneNumber(request_phone_number.as_ref().clone()) }
  pub fn text<T: AsRef<TGKeyboardButtonTypeText>>(text: T) -> Self { TGKeyboardButtonType::Text(text.as_ref().clone()) }
}

