
use rtdlib::types::*;
use crate::types::*;
use crate::api::TDFB;


#[doc(hidden)] pub struct _TGInputMessageAnimationBuilder { inner: TGInputMessageAnimation }

impl _TGInputMessageAnimationBuilder {

  pub fn build(&self) -> TGInputMessageAnimation { self.inner.clone() }

  ///  Duration of the animation, in seconds. 
  pub fn duration(&mut self, duration: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_duration(duration);
    self
  }
  ///  Width of the animation; may be replaced by the server. 
  pub fn width(&mut self, width: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_width(width);
    self
  }
  ///  Height of the animation; may be replaced by the server. 
  pub fn height(&mut self, height: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_height(height);
    self
  }
  

  
  // [animation] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _animation(&mut self, animation: Box<InputFile>) -> &mut Self {
    self.inner.td_origin_mut()._set_animation(animation);
    self
  }
  
  // [thumbnail] type is [InputThumbnail], is not support, need add manully.
  #[doc(hidden)] pub fn _thumbnail(&mut self, thumbnail: InputThumbnail) -> &mut Self {
    self.inner.td_origin_mut()._set_thumbnail(thumbnail);
    self
  }
  
  // [caption] type is [FormattedText], is not support, need add manully.
  #[doc(hidden)] pub fn _caption(&mut self, caption: FormattedText) -> &mut Self {
    self.inner.td_origin_mut()._set_caption(caption);
    self
  }
  
}

impl TGInputMessageAnimation {
  pub fn builder() -> _TGInputMessageAnimationBuilder {
    _TGInputMessageAnimationBuilder { inner: Self::new(InputMessageAnimation::_new()) }
  }
}

impl TDFB for TGInputMessageAnimation {}

impl AsRef<TGInputMessageAnimation> for TGInputMessageAnimation {
  fn as_ref(&self) -> &TGInputMessageAnimation { self }
}

impl AsRef<TGInputMessageAnimation> for _TGInputMessageAnimationBuilder {
  fn as_ref(&self) -> &TGInputMessageAnimation { &self.inner }
}


#[doc(hidden)] pub struct _TGInputMessageAudioBuilder { inner: TGInputMessageAudio }

impl _TGInputMessageAudioBuilder {

  pub fn build(&self) -> TGInputMessageAudio { self.inner.clone() }

  ///  Duration of the audio, in seconds; may be replaced by the server. 
  pub fn duration(&mut self, duration: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_duration(duration);
    self
  }
  ///  Title of the audio; 0-64 characters; may be replaced by the server. 
  pub fn title<S: AsRef<str>>(&mut self, title: S) -> &mut Self {
    self.inner.td_origin_mut()._set_title(title.as_ref().to_string());
    self
  }
  ///  Performer of the audio; 0-64 characters, may be replaced by the server. 
  pub fn performer<S: AsRef<str>>(&mut self, performer: S) -> &mut Self {
    self.inner.td_origin_mut()._set_performer(performer.as_ref().to_string());
    self
  }
  

  
  // [audio] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _audio(&mut self, audio: Box<InputFile>) -> &mut Self {
    self.inner.td_origin_mut()._set_audio(audio);
    self
  }
  
  // [album_cover_thumbnail] type is [InputThumbnail], is not support, need add manully.
  #[doc(hidden)] pub fn _album_cover_thumbnail(&mut self, album_cover_thumbnail: InputThumbnail) -> &mut Self {
    self.inner.td_origin_mut()._set_album_cover_thumbnail(album_cover_thumbnail);
    self
  }
  
  // [caption] type is [FormattedText], is not support, need add manully.
  #[doc(hidden)] pub fn _caption(&mut self, caption: FormattedText) -> &mut Self {
    self.inner.td_origin_mut()._set_caption(caption);
    self
  }
  
}

impl TGInputMessageAudio {
  pub fn builder() -> _TGInputMessageAudioBuilder {
    _TGInputMessageAudioBuilder { inner: Self::new(InputMessageAudio::_new()) }
  }
}

impl TDFB for TGInputMessageAudio {}

impl AsRef<TGInputMessageAudio> for TGInputMessageAudio {
  fn as_ref(&self) -> &TGInputMessageAudio { self }
}

impl AsRef<TGInputMessageAudio> for _TGInputMessageAudioBuilder {
  fn as_ref(&self) -> &TGInputMessageAudio { &self.inner }
}


#[doc(hidden)] pub struct _TGInputMessageContactBuilder { inner: TGInputMessageContact }

impl _TGInputMessageContactBuilder {

  pub fn build(&self) -> TGInputMessageContact { self.inner.clone() }

  

  
  // [contact] type is [Contact], is not support, need add manully.
  #[doc(hidden)] pub fn _contact(&mut self, contact: Contact) -> &mut Self {
    self.inner.td_origin_mut()._set_contact(contact);
    self
  }
  
}

impl TGInputMessageContact {
  pub fn builder() -> _TGInputMessageContactBuilder {
    _TGInputMessageContactBuilder { inner: Self::new(InputMessageContact::_new()) }
  }
}

impl TDFB for TGInputMessageContact {}

impl AsRef<TGInputMessageContact> for TGInputMessageContact {
  fn as_ref(&self) -> &TGInputMessageContact { self }
}

impl AsRef<TGInputMessageContact> for _TGInputMessageContactBuilder {
  fn as_ref(&self) -> &TGInputMessageContact { &self.inner }
}


#[doc(hidden)] pub struct _TGInputMessageDocumentBuilder { inner: TGInputMessageDocument }

impl _TGInputMessageDocumentBuilder {

  pub fn build(&self) -> TGInputMessageDocument { self.inner.clone() }

  

  
  // [document] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _document(&mut self, document: Box<InputFile>) -> &mut Self {
    self.inner.td_origin_mut()._set_document(document);
    self
  }
  
  // [thumbnail] type is [InputThumbnail], is not support, need add manully.
  #[doc(hidden)] pub fn _thumbnail(&mut self, thumbnail: InputThumbnail) -> &mut Self {
    self.inner.td_origin_mut()._set_thumbnail(thumbnail);
    self
  }
  
  // [caption] type is [FormattedText], is not support, need add manully.
  #[doc(hidden)] pub fn _caption(&mut self, caption: FormattedText) -> &mut Self {
    self.inner.td_origin_mut()._set_caption(caption);
    self
  }
  
}

impl TGInputMessageDocument {
  pub fn builder() -> _TGInputMessageDocumentBuilder {
    _TGInputMessageDocumentBuilder { inner: Self::new(InputMessageDocument::_new()) }
  }
}

impl TDFB for TGInputMessageDocument {}

impl AsRef<TGInputMessageDocument> for TGInputMessageDocument {
  fn as_ref(&self) -> &TGInputMessageDocument { self }
}

impl AsRef<TGInputMessageDocument> for _TGInputMessageDocumentBuilder {
  fn as_ref(&self) -> &TGInputMessageDocument { &self.inner }
}


#[doc(hidden)] pub struct _TGInputMessageForwardedBuilder { inner: TGInputMessageForwarded }

impl _TGInputMessageForwardedBuilder {

  pub fn build(&self) -> TGInputMessageForwarded { self.inner.clone() }

  ///  Identifier for the chat this forwarded message came from. 
  pub fn from_chat_id(&mut self, from_chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_from_chat_id(from_chat_id);
    self
  }
  ///  Identifier of the message to forward. 
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_message_id(message_id);
    self
  }
  ///  True, if a game message should be shared within a launched game; applies only to game messages. 
  pub fn in_game_share(&mut self, in_game_share: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_in_game_share(in_game_share);
    self
  }
  

  
}

impl TGInputMessageForwarded {
  pub fn builder() -> _TGInputMessageForwardedBuilder {
    _TGInputMessageForwardedBuilder { inner: Self::new(InputMessageForwarded::_new()) }
  }
}

impl TDFB for TGInputMessageForwarded {}

impl AsRef<TGInputMessageForwarded> for TGInputMessageForwarded {
  fn as_ref(&self) -> &TGInputMessageForwarded { self }
}

impl AsRef<TGInputMessageForwarded> for _TGInputMessageForwardedBuilder {
  fn as_ref(&self) -> &TGInputMessageForwarded { &self.inner }
}


#[doc(hidden)] pub struct _TGInputMessageGameBuilder { inner: TGInputMessageGame }

impl _TGInputMessageGameBuilder {

  pub fn build(&self) -> TGInputMessageGame { self.inner.clone() }

  ///  User identifier of the bot that owns the game. 
  pub fn bot_user_id(&mut self, bot_user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_bot_user_id(bot_user_id);
    self
  }
  ///  Short name of the game. 
  pub fn game_short_name<S: AsRef<str>>(&mut self, game_short_name: S) -> &mut Self {
    self.inner.td_origin_mut()._set_game_short_name(game_short_name.as_ref().to_string());
    self
  }
  

  
}

impl TGInputMessageGame {
  pub fn builder() -> _TGInputMessageGameBuilder {
    _TGInputMessageGameBuilder { inner: Self::new(InputMessageGame::_new()) }
  }
}

impl TDFB for TGInputMessageGame {}

impl AsRef<TGInputMessageGame> for TGInputMessageGame {
  fn as_ref(&self) -> &TGInputMessageGame { self }
}

impl AsRef<TGInputMessageGame> for _TGInputMessageGameBuilder {
  fn as_ref(&self) -> &TGInputMessageGame { &self.inner }
}


#[doc(hidden)] pub struct _TGInputMessageInvoiceBuilder { inner: TGInputMessageInvoice }

impl _TGInputMessageInvoiceBuilder {

  pub fn build(&self) -> TGInputMessageInvoice { self.inner.clone() }

  ///  Product title; 1-32 characters. 
  pub fn title<S: AsRef<str>>(&mut self, title: S) -> &mut Self {
    self.inner.td_origin_mut()._set_title(title.as_ref().to_string());
    self
  }
  ///  Product description; 0-255 characters. 
  pub fn description<S: AsRef<str>>(&mut self, description: S) -> &mut Self {
    self.inner.td_origin_mut()._set_description(description.as_ref().to_string());
    self
  }
  ///  Product photo URL; optional. 
  pub fn photo_url<S: AsRef<str>>(&mut self, photo_url: S) -> &mut Self {
    self.inner.td_origin_mut()._set_photo_url(photo_url.as_ref().to_string());
    self
  }
  ///  Product photo size. 
  pub fn photo_size(&mut self, photo_size: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_photo_size(photo_size);
    self
  }
  ///  Product photo width. 
  pub fn photo_width(&mut self, photo_width: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_photo_width(photo_width);
    self
  }
  ///  Product photo height. 
  pub fn photo_height(&mut self, photo_height: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_photo_height(photo_height);
    self
  }
  ///  The invoice payload. 
  pub fn payload<S: AsRef<str>>(&mut self, payload: S) -> &mut Self {
    self.inner.td_origin_mut()._set_payload(payload.as_ref().to_string());
    self
  }
  ///  Payment provider token. 
  pub fn provider_token<S: AsRef<str>>(&mut self, provider_token: S) -> &mut Self {
    self.inner.td_origin_mut()._set_provider_token(provider_token.as_ref().to_string());
    self
  }
  ///  JSON-encoded data about the invoice, which will be shared with the payment provider. 
  pub fn provider_data<S: AsRef<str>>(&mut self, provider_data: S) -> &mut Self {
    self.inner.td_origin_mut()._set_provider_data(provider_data.as_ref().to_string());
    self
  }
  ///  Unique invoice bot start_parameter for the generation of this invoice. 
  pub fn start_parameter<S: AsRef<str>>(&mut self, start_parameter: S) -> &mut Self {
    self.inner.td_origin_mut()._set_start_parameter(start_parameter.as_ref().to_string());
    self
  }
  

  
  // [invoice] type is [Invoice], is not support, need add manully.
  #[doc(hidden)] pub fn _invoice(&mut self, invoice: Invoice) -> &mut Self {
    self.inner.td_origin_mut()._set_invoice(invoice);
    self
  }
  
}

impl TGInputMessageInvoice {
  pub fn builder() -> _TGInputMessageInvoiceBuilder {
    _TGInputMessageInvoiceBuilder { inner: Self::new(InputMessageInvoice::_new()) }
  }
}

impl TDFB for TGInputMessageInvoice {}

impl AsRef<TGInputMessageInvoice> for TGInputMessageInvoice {
  fn as_ref(&self) -> &TGInputMessageInvoice { self }
}

impl AsRef<TGInputMessageInvoice> for _TGInputMessageInvoiceBuilder {
  fn as_ref(&self) -> &TGInputMessageInvoice { &self.inner }
}


#[doc(hidden)] pub struct _TGInputMessageLocationBuilder { inner: TGInputMessageLocation }

impl _TGInputMessageLocationBuilder {

  pub fn build(&self) -> TGInputMessageLocation { self.inner.clone() }

  ///  Period for which the location can be updated, in seconds; should bebetween 60 and 86400 for a live location and 0 otherwise. 
  pub fn live_period(&mut self, live_period: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_live_period(live_period);
    self
  }
  

  
  // [location] type is [Location], is not support, need add manully.
  #[doc(hidden)] pub fn _location(&mut self, location: Location) -> &mut Self {
    self.inner.td_origin_mut()._set_location(location);
    self
  }
  
}

impl TGInputMessageLocation {
  pub fn builder() -> _TGInputMessageLocationBuilder {
    _TGInputMessageLocationBuilder { inner: Self::new(InputMessageLocation::_new()) }
  }
}

impl TDFB for TGInputMessageLocation {}

impl AsRef<TGInputMessageLocation> for TGInputMessageLocation {
  fn as_ref(&self) -> &TGInputMessageLocation { self }
}

impl AsRef<TGInputMessageLocation> for _TGInputMessageLocationBuilder {
  fn as_ref(&self) -> &TGInputMessageLocation { &self.inner }
}


#[doc(hidden)] pub struct _TGInputMessagePhotoBuilder { inner: TGInputMessagePhoto }

impl _TGInputMessagePhotoBuilder {

  pub fn build(&self) -> TGInputMessagePhoto { self.inner.clone() }

  ///  File identifiers of the stickers added to the photo, if applicable. 
  pub fn added_sticker_file_ids(&mut self, added_sticker_file_ids: Vec<i32>) -> &mut Self {
    self.inner.td_origin_mut()._set_added_sticker_file_ids(added_sticker_file_ids);
    self
  }
  ///  Photo width. 
  pub fn width(&mut self, width: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_width(width);
    self
  }
  ///  Photo height. 
  pub fn height(&mut self, height: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_height(height);
    self
  }
  ///  Photo TTL (Time To Live), in seconds (0-60). A non-zero TTL can be specified only in private chats. 
  pub fn ttl(&mut self, ttl: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_ttl(ttl);
    self
  }
  

  
  // [photo] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _photo(&mut self, photo: Box<InputFile>) -> &mut Self {
    self.inner.td_origin_mut()._set_photo(photo);
    self
  }
  
  // [thumbnail] type is [InputThumbnail], is not support, need add manully.
  #[doc(hidden)] pub fn _thumbnail(&mut self, thumbnail: InputThumbnail) -> &mut Self {
    self.inner.td_origin_mut()._set_thumbnail(thumbnail);
    self
  }
  
  // [caption] type is [FormattedText], is not support, need add manully.
  #[doc(hidden)] pub fn _caption(&mut self, caption: FormattedText) -> &mut Self {
    self.inner.td_origin_mut()._set_caption(caption);
    self
  }
  
}

impl TGInputMessagePhoto {
  pub fn builder() -> _TGInputMessagePhotoBuilder {
    _TGInputMessagePhotoBuilder { inner: Self::new(InputMessagePhoto::_new()) }
  }
}

impl TDFB for TGInputMessagePhoto {}

impl AsRef<TGInputMessagePhoto> for TGInputMessagePhoto {
  fn as_ref(&self) -> &TGInputMessagePhoto { self }
}

impl AsRef<TGInputMessagePhoto> for _TGInputMessagePhotoBuilder {
  fn as_ref(&self) -> &TGInputMessagePhoto { &self.inner }
}


#[doc(hidden)] pub struct _TGInputMessagePollBuilder { inner: TGInputMessagePoll }

impl _TGInputMessagePollBuilder {

  pub fn build(&self) -> TGInputMessagePoll { self.inner.clone() }

  ///  Poll question, 1-255 characters. 
  pub fn question<S: AsRef<str>>(&mut self, question: S) -> &mut Self {
    self.inner.td_origin_mut()._set_question(question.as_ref().to_string());
    self
  }
  

  
  // [options] type is [Vec<String>], is not support, need add manully.
  #[doc(hidden)] pub fn _options(&mut self, options: Vec<String>) -> &mut Self {
    self.inner.td_origin_mut()._set_options(options);
    self
  }
  
}

impl TGInputMessagePoll {
  pub fn builder() -> _TGInputMessagePollBuilder {
    _TGInputMessagePollBuilder { inner: Self::new(InputMessagePoll::_new()) }
  }
}

impl TDFB for TGInputMessagePoll {}

impl AsRef<TGInputMessagePoll> for TGInputMessagePoll {
  fn as_ref(&self) -> &TGInputMessagePoll { self }
}

impl AsRef<TGInputMessagePoll> for _TGInputMessagePollBuilder {
  fn as_ref(&self) -> &TGInputMessagePoll { &self.inner }
}


#[doc(hidden)] pub struct _TGInputMessageStickerBuilder { inner: TGInputMessageSticker }

impl _TGInputMessageStickerBuilder {

  pub fn build(&self) -> TGInputMessageSticker { self.inner.clone() }

  ///  Sticker width. 
  pub fn width(&mut self, width: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_width(width);
    self
  }
  ///  Sticker height. 
  pub fn height(&mut self, height: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_height(height);
    self
  }
  

  
  // [sticker] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _sticker(&mut self, sticker: Box<InputFile>) -> &mut Self {
    self.inner.td_origin_mut()._set_sticker(sticker);
    self
  }
  
  // [thumbnail] type is [InputThumbnail], is not support, need add manully.
  #[doc(hidden)] pub fn _thumbnail(&mut self, thumbnail: InputThumbnail) -> &mut Self {
    self.inner.td_origin_mut()._set_thumbnail(thumbnail);
    self
  }
  
}

impl TGInputMessageSticker {
  pub fn builder() -> _TGInputMessageStickerBuilder {
    _TGInputMessageStickerBuilder { inner: Self::new(InputMessageSticker::_new()) }
  }
}

impl TDFB for TGInputMessageSticker {}

impl AsRef<TGInputMessageSticker> for TGInputMessageSticker {
  fn as_ref(&self) -> &TGInputMessageSticker { self }
}

impl AsRef<TGInputMessageSticker> for _TGInputMessageStickerBuilder {
  fn as_ref(&self) -> &TGInputMessageSticker { &self.inner }
}


#[doc(hidden)] pub struct _TGInputMessageTextBuilder { inner: TGInputMessageText }

impl _TGInputMessageTextBuilder {

  pub fn build(&self) -> TGInputMessageText { self.inner.clone() }

  ///  True, if rich web page previews for URLs in the message text should be disabled. 
  pub fn disable_web_page_preview(&mut self, disable_web_page_preview: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_disable_web_page_preview(disable_web_page_preview);
    self
  }
  ///  True, if a chat message draft should be deleted. 
  pub fn clear_draft(&mut self, clear_draft: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_clear_draft(clear_draft);
    self
  }
  

  
  // [text] type is [FormattedText], is not support, need add manully.
  #[doc(hidden)] pub fn _text(&mut self, text: FormattedText) -> &mut Self {
    self.inner.td_origin_mut()._set_text(text);
    self
  }
  
}

impl TGInputMessageText {
  pub fn builder() -> _TGInputMessageTextBuilder {
    _TGInputMessageTextBuilder { inner: Self::new(InputMessageText::_new()) }
  }
}

impl TDFB for TGInputMessageText {}

impl AsRef<TGInputMessageText> for TGInputMessageText {
  fn as_ref(&self) -> &TGInputMessageText { self }
}

impl AsRef<TGInputMessageText> for _TGInputMessageTextBuilder {
  fn as_ref(&self) -> &TGInputMessageText { &self.inner }
}


#[doc(hidden)] pub struct _TGInputMessageVenueBuilder { inner: TGInputMessageVenue }

impl _TGInputMessageVenueBuilder {

  pub fn build(&self) -> TGInputMessageVenue { self.inner.clone() }

  

  
  // [venue] type is [Venue], is not support, need add manully.
  #[doc(hidden)] pub fn _venue(&mut self, venue: Venue) -> &mut Self {
    self.inner.td_origin_mut()._set_venue(venue);
    self
  }
  
}

impl TGInputMessageVenue {
  pub fn builder() -> _TGInputMessageVenueBuilder {
    _TGInputMessageVenueBuilder { inner: Self::new(InputMessageVenue::_new()) }
  }
}

impl TDFB for TGInputMessageVenue {}

impl AsRef<TGInputMessageVenue> for TGInputMessageVenue {
  fn as_ref(&self) -> &TGInputMessageVenue { self }
}

impl AsRef<TGInputMessageVenue> for _TGInputMessageVenueBuilder {
  fn as_ref(&self) -> &TGInputMessageVenue { &self.inner }
}


#[doc(hidden)] pub struct _TGInputMessageVideoBuilder { inner: TGInputMessageVideo }

impl _TGInputMessageVideoBuilder {

  pub fn build(&self) -> TGInputMessageVideo { self.inner.clone() }

  ///  File identifiers of the stickers added to the video, if applicable. 
  pub fn added_sticker_file_ids(&mut self, added_sticker_file_ids: Vec<i32>) -> &mut Self {
    self.inner.td_origin_mut()._set_added_sticker_file_ids(added_sticker_file_ids);
    self
  }
  ///  Duration of the video, in seconds. 
  pub fn duration(&mut self, duration: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_duration(duration);
    self
  }
  ///  Video width. 
  pub fn width(&mut self, width: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_width(width);
    self
  }
  ///  Video height. 
  pub fn height(&mut self, height: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_height(height);
    self
  }
  ///  True, if the video should be tried to be streamed. 
  pub fn supports_streaming(&mut self, supports_streaming: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_supports_streaming(supports_streaming);
    self
  }
  ///  Video TTL (Time To Live), in seconds (0-60). A non-zero TTL can be specified only in private chats. 
  pub fn ttl(&mut self, ttl: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_ttl(ttl);
    self
  }
  

  
  // [video] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _video(&mut self, video: Box<InputFile>) -> &mut Self {
    self.inner.td_origin_mut()._set_video(video);
    self
  }
  
  // [thumbnail] type is [InputThumbnail], is not support, need add manully.
  #[doc(hidden)] pub fn _thumbnail(&mut self, thumbnail: InputThumbnail) -> &mut Self {
    self.inner.td_origin_mut()._set_thumbnail(thumbnail);
    self
  }
  
  // [caption] type is [FormattedText], is not support, need add manully.
  #[doc(hidden)] pub fn _caption(&mut self, caption: FormattedText) -> &mut Self {
    self.inner.td_origin_mut()._set_caption(caption);
    self
  }
  
}

impl TGInputMessageVideo {
  pub fn builder() -> _TGInputMessageVideoBuilder {
    _TGInputMessageVideoBuilder { inner: Self::new(InputMessageVideo::_new()) }
  }
}

impl TDFB for TGInputMessageVideo {}

impl AsRef<TGInputMessageVideo> for TGInputMessageVideo {
  fn as_ref(&self) -> &TGInputMessageVideo { self }
}

impl AsRef<TGInputMessageVideo> for _TGInputMessageVideoBuilder {
  fn as_ref(&self) -> &TGInputMessageVideo { &self.inner }
}


#[doc(hidden)] pub struct _TGInputMessageVideoNoteBuilder { inner: TGInputMessageVideoNote }

impl _TGInputMessageVideoNoteBuilder {

  pub fn build(&self) -> TGInputMessageVideoNote { self.inner.clone() }

  ///  Duration of the video, in seconds. 
  pub fn duration(&mut self, duration: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_duration(duration);
    self
  }
  ///  Video width and height; must be positive and not greater than 640. 
  pub fn length(&mut self, length: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_length(length);
    self
  }
  

  
  // [video_note] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _video_note(&mut self, video_note: Box<InputFile>) -> &mut Self {
    self.inner.td_origin_mut()._set_video_note(video_note);
    self
  }
  
  // [thumbnail] type is [InputThumbnail], is not support, need add manully.
  #[doc(hidden)] pub fn _thumbnail(&mut self, thumbnail: InputThumbnail) -> &mut Self {
    self.inner.td_origin_mut()._set_thumbnail(thumbnail);
    self
  }
  
}

impl TGInputMessageVideoNote {
  pub fn builder() -> _TGInputMessageVideoNoteBuilder {
    _TGInputMessageVideoNoteBuilder { inner: Self::new(InputMessageVideoNote::_new()) }
  }
}

impl TDFB for TGInputMessageVideoNote {}

impl AsRef<TGInputMessageVideoNote> for TGInputMessageVideoNote {
  fn as_ref(&self) -> &TGInputMessageVideoNote { self }
}

impl AsRef<TGInputMessageVideoNote> for _TGInputMessageVideoNoteBuilder {
  fn as_ref(&self) -> &TGInputMessageVideoNote { &self.inner }
}


#[doc(hidden)] pub struct _TGInputMessageVoiceNoteBuilder { inner: TGInputMessageVoiceNote }

impl _TGInputMessageVoiceNoteBuilder {

  pub fn build(&self) -> TGInputMessageVoiceNote { self.inner.clone() }

  ///  Duration of the voice note, in seconds. 
  pub fn duration(&mut self, duration: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_duration(duration);
    self
  }
  ///  Waveform representation of the voice note, in 5-bit format. 
  pub fn waveform<S: AsRef<str>>(&mut self, waveform: S) -> &mut Self {
    self.inner.td_origin_mut()._set_waveform(waveform.as_ref().to_string());
    self
  }
  

  
  // [voice_note] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _voice_note(&mut self, voice_note: Box<InputFile>) -> &mut Self {
    self.inner.td_origin_mut()._set_voice_note(voice_note);
    self
  }
  
  // [caption] type is [FormattedText], is not support, need add manully.
  #[doc(hidden)] pub fn _caption(&mut self, caption: FormattedText) -> &mut Self {
    self.inner.td_origin_mut()._set_caption(caption);
    self
  }
  
}

impl TGInputMessageVoiceNote {
  pub fn builder() -> _TGInputMessageVoiceNoteBuilder {
    _TGInputMessageVoiceNoteBuilder { inner: Self::new(InputMessageVoiceNote::_new()) }
  }
}

impl TDFB for TGInputMessageVoiceNote {}

impl AsRef<TGInputMessageVoiceNote> for TGInputMessageVoiceNote {
  fn as_ref(&self) -> &TGInputMessageVoiceNote { self }
}

impl AsRef<TGInputMessageVoiceNote> for _TGInputMessageVoiceNoteBuilder {
  fn as_ref(&self) -> &TGInputMessageVoiceNote { &self.inner }
}


#[doc(hidden)] pub struct _TGInputThumbnailBuilder { inner: TGInputThumbnail }

impl _TGInputThumbnailBuilder {

  pub fn build(&self) -> TGInputThumbnail { self.inner.clone() }

  ///  Thumbnail width, usually shouldn't exceed 320. Use 0 if unknown. 
  pub fn width(&mut self, width: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_width(width);
    self
  }
  ///  Thumbnail height, usually shouldn't exceed 320. Use 0 if unknown. 
  pub fn height(&mut self, height: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_height(height);
    self
  }
  

  
  // [thumbnail] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _thumbnail(&mut self, thumbnail: Box<InputFile>) -> &mut Self {
    self.inner.td_origin_mut()._set_thumbnail(thumbnail);
    self
  }
  
}

impl TGInputThumbnail {
  pub fn builder() -> _TGInputThumbnailBuilder {
    _TGInputThumbnailBuilder { inner: Self::new(InputThumbnail::_new()) }
  }
}

impl TDFB for TGInputThumbnail {}

impl AsRef<TGInputThumbnail> for TGInputThumbnail {
  fn as_ref(&self) -> &TGInputThumbnail { self }
}

impl AsRef<TGInputThumbnail> for _TGInputThumbnailBuilder {
  fn as_ref(&self) -> &TGInputThumbnail { &self.inner }
}


#[doc(hidden)] pub struct _TGFormattedTextBuilder { inner: TGFormattedText }

impl _TGFormattedTextBuilder {

  pub fn build(&self) -> TGFormattedText { self.inner.clone() }

  ///  The text. 
  pub fn text<S: AsRef<str>>(&mut self, text: S) -> &mut Self {
    self.inner.td_origin_mut()._set_text(text.as_ref().to_string());
    self
  }
  

  
  // [entities] type is [Vec<TextEntity>], is not support, need add manully.
  #[doc(hidden)] pub fn _entities(&mut self, entities: Vec<TextEntity>) -> &mut Self {
    self.inner.td_origin_mut()._set_entities(entities);
    self
  }
  
}

impl TGFormattedText {
  pub fn builder() -> _TGFormattedTextBuilder {
    _TGFormattedTextBuilder { inner: Self::new(FormattedText::_new()) }
  }
}

impl TDFB for TGFormattedText {}

impl AsRef<TGFormattedText> for TGFormattedText {
  fn as_ref(&self) -> &TGFormattedText { self }
}

impl AsRef<TGFormattedText> for _TGFormattedTextBuilder {
  fn as_ref(&self) -> &TGFormattedText { &self.inner }
}


#[doc(hidden)] pub struct _TGInputFileGeneratedBuilder { inner: TGInputFileGenerated }

impl _TGInputFileGeneratedBuilder {

  pub fn build(&self) -> TGInputFileGenerated { self.inner.clone() }

  ///  Local path to a file from which the file is generated; may be empty if there is no such file. 
  pub fn original_path<S: AsRef<str>>(&mut self, original_path: S) -> &mut Self {
    self.inner.td_origin_mut()._set_original_path(original_path.as_ref().to_string());
    self
  }
  ///  String specifying the conversion applied to the original file; should be persistent across application restarts. Conversions beginning with '#' are reserved for internal TDLib usage. 
  pub fn conversion<S: AsRef<str>>(&mut self, conversion: S) -> &mut Self {
    self.inner.td_origin_mut()._set_conversion(conversion.as_ref().to_string());
    self
  }
  ///  Expected size of the generated file; 0 if unknown. 
  pub fn expected_size(&mut self, expected_size: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_expected_size(expected_size);
    self
  }
  

  
}

impl TGInputFileGenerated {
  pub fn builder() -> _TGInputFileGeneratedBuilder {
    _TGInputFileGeneratedBuilder { inner: Self::new(InputFileGenerated::_new()) }
  }
}

impl TDFB for TGInputFileGenerated {}

impl AsRef<TGInputFileGenerated> for TGInputFileGenerated {
  fn as_ref(&self) -> &TGInputFileGenerated { self }
}

impl AsRef<TGInputFileGenerated> for _TGInputFileGeneratedBuilder {
  fn as_ref(&self) -> &TGInputFileGenerated { &self.inner }
}


#[doc(hidden)] pub struct _TGInputFileIdBuilder { inner: TGInputFileId }

impl _TGInputFileIdBuilder {

  pub fn build(&self) -> TGInputFileId { self.inner.clone() }

  ///  Unique file identifier. 
  pub fn id(&mut self, id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_id(id);
    self
  }
  

  
}

impl TGInputFileId {
  pub fn builder() -> _TGInputFileIdBuilder {
    _TGInputFileIdBuilder { inner: Self::new(InputFileId::_new()) }
  }
}

impl TDFB for TGInputFileId {}

impl AsRef<TGInputFileId> for TGInputFileId {
  fn as_ref(&self) -> &TGInputFileId { self }
}

impl AsRef<TGInputFileId> for _TGInputFileIdBuilder {
  fn as_ref(&self) -> &TGInputFileId { &self.inner }
}


#[doc(hidden)] pub struct _TGInputFileLocalBuilder { inner: TGInputFileLocal }

impl _TGInputFileLocalBuilder {

  pub fn build(&self) -> TGInputFileLocal { self.inner.clone() }

  ///  Local path to the file. 
  pub fn path<S: AsRef<str>>(&mut self, path: S) -> &mut Self {
    self.inner.td_origin_mut()._set_path(path.as_ref().to_string());
    self
  }
  

  
}

impl TGInputFileLocal {
  pub fn builder() -> _TGInputFileLocalBuilder {
    _TGInputFileLocalBuilder { inner: Self::new(InputFileLocal::_new()) }
  }
}

impl TDFB for TGInputFileLocal {}

impl AsRef<TGInputFileLocal> for TGInputFileLocal {
  fn as_ref(&self) -> &TGInputFileLocal { self }
}

impl AsRef<TGInputFileLocal> for _TGInputFileLocalBuilder {
  fn as_ref(&self) -> &TGInputFileLocal { &self.inner }
}


#[doc(hidden)] pub struct _TGInputFileRemoteBuilder { inner: TGInputFileRemote }

impl _TGInputFileRemoteBuilder {

  pub fn build(&self) -> TGInputFileRemote { self.inner.clone() }

  ///  Remote file identifier. 
  pub fn id<S: AsRef<str>>(&mut self, id: S) -> &mut Self {
    self.inner.td_origin_mut()._set_id(id.as_ref().to_string());
    self
  }
  

  
}

impl TGInputFileRemote {
  pub fn builder() -> _TGInputFileRemoteBuilder {
    _TGInputFileRemoteBuilder { inner: Self::new(InputFileRemote::_new()) }
  }
}

impl TDFB for TGInputFileRemote {}

impl AsRef<TGInputFileRemote> for TGInputFileRemote {
  fn as_ref(&self) -> &TGInputFileRemote { self }
}

impl AsRef<TGInputFileRemote> for _TGInputFileRemoteBuilder {
  fn as_ref(&self) -> &TGInputFileRemote { &self.inner }
}


#[doc(hidden)] pub struct _TGTextEntityBuilder { inner: TGTextEntity }

impl _TGTextEntityBuilder {

  pub fn build(&self) -> TGTextEntity { self.inner.clone() }

  ///  Offset of the entity in UTF-16 code points. 
  pub fn offset(&mut self, offset: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_offset(offset);
    self
  }
  ///  Length of the entity, in UTF-16 code points. 
  pub fn length(&mut self, length: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_length(length);
    self
  }
  

  
  // [type_] type is [Box<TextEntityType>], is not support, need add manully.
  #[doc(hidden)] pub fn _type_(&mut self, type_: Box<TextEntityType>) -> &mut Self {
    self.inner.td_origin_mut()._set_type_(type_);
    self
  }
  
}

impl TGTextEntity {
  pub fn builder() -> _TGTextEntityBuilder {
    _TGTextEntityBuilder { inner: Self::new(TextEntity::_new()) }
  }
}

impl TDFB for TGTextEntity {}

impl AsRef<TGTextEntity> for TGTextEntity {
  fn as_ref(&self) -> &TGTextEntity { self }
}

impl AsRef<TGTextEntity> for _TGTextEntityBuilder {
  fn as_ref(&self) -> &TGTextEntity { &self.inner }
}


#[doc(hidden)] pub struct _TGContactBuilder { inner: TGContact }

impl _TGContactBuilder {

  pub fn build(&self) -> TGContact { self.inner.clone() }

  ///  Phone number of the user. 
  pub fn phone_number<S: AsRef<str>>(&mut self, phone_number: S) -> &mut Self {
    self.inner.td_origin_mut()._set_phone_number(phone_number.as_ref().to_string());
    self
  }
  ///  First name of the user; 1-255 characters in length. 
  pub fn first_name<S: AsRef<str>>(&mut self, first_name: S) -> &mut Self {
    self.inner.td_origin_mut()._set_first_name(first_name.as_ref().to_string());
    self
  }
  ///  Last name of the user. 
  pub fn last_name<S: AsRef<str>>(&mut self, last_name: S) -> &mut Self {
    self.inner.td_origin_mut()._set_last_name(last_name.as_ref().to_string());
    self
  }
  ///  Additional data about the user in a form of vCard; 0-2048 bytes in length. 
  pub fn vcard<S: AsRef<str>>(&mut self, vcard: S) -> &mut Self {
    self.inner.td_origin_mut()._set_vcard(vcard.as_ref().to_string());
    self
  }
  ///  Identifier of the user, if known; otherwise 0. 
  pub fn user_id(&mut self, user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_user_id(user_id);
    self
  }
  

  
}

impl TGContact {
  pub fn builder() -> _TGContactBuilder {
    _TGContactBuilder { inner: Self::new(Contact::_new()) }
  }
}

impl TDFB for TGContact {}

impl AsRef<TGContact> for TGContact {
  fn as_ref(&self) -> &TGContact { self }
}

impl AsRef<TGContact> for _TGContactBuilder {
  fn as_ref(&self) -> &TGContact { &self.inner }
}


#[doc(hidden)] pub struct _TGInvoiceBuilder { inner: TGInvoice }

impl _TGInvoiceBuilder {

  pub fn build(&self) -> TGInvoice { self.inner.clone() }

  ///  ISO 4217 currency code. 
  pub fn currency<S: AsRef<str>>(&mut self, currency: S) -> &mut Self {
    self.inner.td_origin_mut()._set_currency(currency.as_ref().to_string());
    self
  }
  ///  True, if the payment is a test payment. 
  pub fn is_test(&mut self, is_test: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_is_test(is_test);
    self
  }
  ///  True, if the user's name is needed for payment. 
  pub fn need_name(&mut self, need_name: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_need_name(need_name);
    self
  }
  ///  True, if the user's phone number is needed for payment. 
  pub fn need_phone_number(&mut self, need_phone_number: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_need_phone_number(need_phone_number);
    self
  }
  ///  True, if the user's email address is needed for payment. 
  pub fn need_email_address(&mut self, need_email_address: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_need_email_address(need_email_address);
    self
  }
  ///  True, if the user's shipping address is needed for payment. 
  pub fn need_shipping_address(&mut self, need_shipping_address: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_need_shipping_address(need_shipping_address);
    self
  }
  ///  True, if the user's phone number will be sent to the provider. 
  pub fn send_phone_number_to_provider(&mut self, send_phone_number_to_provider: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_send_phone_number_to_provider(send_phone_number_to_provider);
    self
  }
  ///  True, if the user's email address will be sent to the provider. 
  pub fn send_email_address_to_provider(&mut self, send_email_address_to_provider: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_send_email_address_to_provider(send_email_address_to_provider);
    self
  }
  ///  True, if the total price depends on the shipping method. 
  pub fn is_flexible(&mut self, is_flexible: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_is_flexible(is_flexible);
    self
  }
  

  
  // [price_parts] type is [Vec<LabeledPricePart>], is not support, need add manully.
  #[doc(hidden)] pub fn _price_parts(&mut self, price_parts: Vec<LabeledPricePart>) -> &mut Self {
    self.inner.td_origin_mut()._set_price_parts(price_parts);
    self
  }
  
}

impl TGInvoice {
  pub fn builder() -> _TGInvoiceBuilder {
    _TGInvoiceBuilder { inner: Self::new(Invoice::_new()) }
  }
}

impl TDFB for TGInvoice {}

impl AsRef<TGInvoice> for TGInvoice {
  fn as_ref(&self) -> &TGInvoice { self }
}

impl AsRef<TGInvoice> for _TGInvoiceBuilder {
  fn as_ref(&self) -> &TGInvoice { &self.inner }
}


#[doc(hidden)] pub struct _TGLabeledPricePartBuilder { inner: TGLabeledPricePart }

impl _TGLabeledPricePartBuilder {

  pub fn build(&self) -> TGLabeledPricePart { self.inner.clone() }

  ///  Label for this portion of the product price. 
  pub fn label<S: AsRef<str>>(&mut self, label: S) -> &mut Self {
    self.inner.td_origin_mut()._set_label(label.as_ref().to_string());
    self
  }
  ///  Currency amount in minimal quantity of the currency. 
  pub fn amount(&mut self, amount: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_amount(amount);
    self
  }
  

  
}

impl TGLabeledPricePart {
  pub fn builder() -> _TGLabeledPricePartBuilder {
    _TGLabeledPricePartBuilder { inner: Self::new(LabeledPricePart::_new()) }
  }
}

impl TDFB for TGLabeledPricePart {}

impl AsRef<TGLabeledPricePart> for TGLabeledPricePart {
  fn as_ref(&self) -> &TGLabeledPricePart { self }
}

impl AsRef<TGLabeledPricePart> for _TGLabeledPricePartBuilder {
  fn as_ref(&self) -> &TGLabeledPricePart { &self.inner }
}


#[doc(hidden)] pub struct _TGLocationBuilder { inner: TGLocation }

impl _TGLocationBuilder {

  pub fn build(&self) -> TGLocation { self.inner.clone() }

  ///  Latitude of the location in degrees; as defined by the sender. 
  pub fn latitude(&mut self, latitude: f64) -> &mut Self {
    self.inner.td_origin_mut()._set_latitude(latitude);
    self
  }
  ///  Longitude of the location, in degrees; as defined by the sender. 
  pub fn longitude(&mut self, longitude: f64) -> &mut Self {
    self.inner.td_origin_mut()._set_longitude(longitude);
    self
  }
  

  
}

impl TGLocation {
  pub fn builder() -> _TGLocationBuilder {
    _TGLocationBuilder { inner: Self::new(Location::_new()) }
  }
}

impl TDFB for TGLocation {}

impl AsRef<TGLocation> for TGLocation {
  fn as_ref(&self) -> &TGLocation { self }
}

impl AsRef<TGLocation> for _TGLocationBuilder {
  fn as_ref(&self) -> &TGLocation { &self.inner }
}


#[doc(hidden)] pub struct _TGVenueBuilder { inner: TGVenue }

impl _TGVenueBuilder {

  pub fn build(&self) -> TGVenue { self.inner.clone() }

  ///  Venue name; as defined by the sender. 
  pub fn title<S: AsRef<str>>(&mut self, title: S) -> &mut Self {
    self.inner.td_origin_mut()._set_title(title.as_ref().to_string());
    self
  }
  ///  Venue address; as defined by the sender. 
  pub fn address<S: AsRef<str>>(&mut self, address: S) -> &mut Self {
    self.inner.td_origin_mut()._set_address(address.as_ref().to_string());
    self
  }
  ///  Provider of the venue database; as defined by the sender. Currently only "foursquare" needs to be supported. 
  pub fn provider<S: AsRef<str>>(&mut self, provider: S) -> &mut Self {
    self.inner.td_origin_mut()._set_provider(provider.as_ref().to_string());
    self
  }
  ///  Identifier of the venue in the provider database; as defined by the sender. 
  pub fn id<S: AsRef<str>>(&mut self, id: S) -> &mut Self {
    self.inner.td_origin_mut()._set_id(id.as_ref().to_string());
    self
  }
  ///  Type of the venue in the provider database; as defined by the sender. 
  pub fn type_<S: AsRef<str>>(&mut self, type_: S) -> &mut Self {
    self.inner.td_origin_mut()._set_type_(type_.as_ref().to_string());
    self
  }
  

  
  // [location] type is [Location], is not support, need add manully.
  #[doc(hidden)] pub fn _location(&mut self, location: Location) -> &mut Self {
    self.inner.td_origin_mut()._set_location(location);
    self
  }
  
}

impl TGVenue {
  pub fn builder() -> _TGVenueBuilder {
    _TGVenueBuilder { inner: Self::new(Venue::_new()) }
  }
}

impl TDFB for TGVenue {}

impl AsRef<TGVenue> for TGVenue {
  fn as_ref(&self) -> &TGVenue { self }
}

impl AsRef<TGVenue> for _TGVenueBuilder {
  fn as_ref(&self) -> &TGVenue { &self.inner }
}


#[doc(hidden)] pub struct _TGReplyMarkupForceReplyBuilder { inner: TGReplyMarkupForceReply }

impl _TGReplyMarkupForceReplyBuilder {

  pub fn build(&self) -> TGReplyMarkupForceReply { self.inner.clone() }

  ///  True, if a forced reply must automatically be shown to the current user. For outgoing messages, specify true to show the forced reply only for the mentioned users and for the target user of a reply. 
  pub fn is_personal(&mut self, is_personal: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_is_personal(is_personal);
    self
  }
  

  
}

impl TGReplyMarkupForceReply {
  pub fn builder() -> _TGReplyMarkupForceReplyBuilder {
    _TGReplyMarkupForceReplyBuilder { inner: Self::new(ReplyMarkupForceReply::_new()) }
  }
}

impl TDFB for TGReplyMarkupForceReply {}

impl AsRef<TGReplyMarkupForceReply> for TGReplyMarkupForceReply {
  fn as_ref(&self) -> &TGReplyMarkupForceReply { self }
}

impl AsRef<TGReplyMarkupForceReply> for _TGReplyMarkupForceReplyBuilder {
  fn as_ref(&self) -> &TGReplyMarkupForceReply { &self.inner }
}


#[doc(hidden)] pub struct _TGReplyMarkupInlineKeyboardBuilder { inner: TGReplyMarkupInlineKeyboard }

impl _TGReplyMarkupInlineKeyboardBuilder {

  pub fn build(&self) -> TGReplyMarkupInlineKeyboard { self.inner.clone() }

  

  
  // [rows] type is [Vec<Vec<InlineKeyboardButton>>], is not support, need add manully.
  #[doc(hidden)] pub fn _rows(&mut self, rows: Vec<Vec<InlineKeyboardButton>>) -> &mut Self {
    self.inner.td_origin_mut()._set_rows(rows);
    self
  }
  
}

impl TGReplyMarkupInlineKeyboard {
  pub fn builder() -> _TGReplyMarkupInlineKeyboardBuilder {
    _TGReplyMarkupInlineKeyboardBuilder { inner: Self::new(ReplyMarkupInlineKeyboard::_new()) }
  }
}

impl TDFB for TGReplyMarkupInlineKeyboard {}

impl AsRef<TGReplyMarkupInlineKeyboard> for TGReplyMarkupInlineKeyboard {
  fn as_ref(&self) -> &TGReplyMarkupInlineKeyboard { self }
}

impl AsRef<TGReplyMarkupInlineKeyboard> for _TGReplyMarkupInlineKeyboardBuilder {
  fn as_ref(&self) -> &TGReplyMarkupInlineKeyboard { &self.inner }
}


#[doc(hidden)] pub struct _TGReplyMarkupRemoveKeyboardBuilder { inner: TGReplyMarkupRemoveKeyboard }

impl _TGReplyMarkupRemoveKeyboardBuilder {

  pub fn build(&self) -> TGReplyMarkupRemoveKeyboard { self.inner.clone() }

  ///  True, if the keyboard is removed only for the mentioned users or the target user of a reply. 
  pub fn is_personal(&mut self, is_personal: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_is_personal(is_personal);
    self
  }
  

  
}

impl TGReplyMarkupRemoveKeyboard {
  pub fn builder() -> _TGReplyMarkupRemoveKeyboardBuilder {
    _TGReplyMarkupRemoveKeyboardBuilder { inner: Self::new(ReplyMarkupRemoveKeyboard::_new()) }
  }
}

impl TDFB for TGReplyMarkupRemoveKeyboard {}

impl AsRef<TGReplyMarkupRemoveKeyboard> for TGReplyMarkupRemoveKeyboard {
  fn as_ref(&self) -> &TGReplyMarkupRemoveKeyboard { self }
}

impl AsRef<TGReplyMarkupRemoveKeyboard> for _TGReplyMarkupRemoveKeyboardBuilder {
  fn as_ref(&self) -> &TGReplyMarkupRemoveKeyboard { &self.inner }
}


#[doc(hidden)] pub struct _TGReplyMarkupShowKeyboardBuilder { inner: TGReplyMarkupShowKeyboard }

impl _TGReplyMarkupShowKeyboardBuilder {

  pub fn build(&self) -> TGReplyMarkupShowKeyboard { self.inner.clone() }

  ///  True, if the client needs to resize the keyboard vertically. 
  pub fn resize_keyboard(&mut self, resize_keyboard: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_resize_keyboard(resize_keyboard);
    self
  }
  ///  True, if the client needs to hide the keyboard after use. 
  pub fn one_time(&mut self, one_time: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_one_time(one_time);
    self
  }
  ///  True, if the keyboard must automatically be shown to the current user. For outgoing messages, specify true to show the keyboard only for the mentioned users and for the target user of a reply. 
  pub fn is_personal(&mut self, is_personal: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_is_personal(is_personal);
    self
  }
  

  
  // [rows] type is [Vec<Vec<KeyboardButton>>], is not support, need add manully.
  #[doc(hidden)] pub fn _rows(&mut self, rows: Vec<Vec<KeyboardButton>>) -> &mut Self {
    self.inner.td_origin_mut()._set_rows(rows);
    self
  }
  
}

impl TGReplyMarkupShowKeyboard {
  pub fn builder() -> _TGReplyMarkupShowKeyboardBuilder {
    _TGReplyMarkupShowKeyboardBuilder { inner: Self::new(ReplyMarkupShowKeyboard::_new()) }
  }
}

impl TDFB for TGReplyMarkupShowKeyboard {}

impl AsRef<TGReplyMarkupShowKeyboard> for TGReplyMarkupShowKeyboard {
  fn as_ref(&self) -> &TGReplyMarkupShowKeyboard { self }
}

impl AsRef<TGReplyMarkupShowKeyboard> for _TGReplyMarkupShowKeyboardBuilder {
  fn as_ref(&self) -> &TGReplyMarkupShowKeyboard { &self.inner }
}


#[doc(hidden)] pub struct _TGInlineKeyboardButtonBuilder { inner: TGInlineKeyboardButton }

impl _TGInlineKeyboardButtonBuilder {

  pub fn build(&self) -> TGInlineKeyboardButton { self.inner.clone() }

  ///  Text of the button. 
  pub fn text<S: AsRef<str>>(&mut self, text: S) -> &mut Self {
    self.inner.td_origin_mut()._set_text(text.as_ref().to_string());
    self
  }
  

  
  // [type_] type is [Box<InlineKeyboardButtonType>], is not support, need add manully.
  #[doc(hidden)] pub fn _type_(&mut self, type_: Box<InlineKeyboardButtonType>) -> &mut Self {
    self.inner.td_origin_mut()._set_type_(type_);
    self
  }
  
}

impl TGInlineKeyboardButton {
  pub fn builder() -> _TGInlineKeyboardButtonBuilder {
    _TGInlineKeyboardButtonBuilder { inner: Self::new(InlineKeyboardButton::_new()) }
  }
}

impl TDFB for TGInlineKeyboardButton {}

impl AsRef<TGInlineKeyboardButton> for TGInlineKeyboardButton {
  fn as_ref(&self) -> &TGInlineKeyboardButton { self }
}

impl AsRef<TGInlineKeyboardButton> for _TGInlineKeyboardButtonBuilder {
  fn as_ref(&self) -> &TGInlineKeyboardButton { &self.inner }
}


#[doc(hidden)] pub struct _TGInlineKeyboardButtonTypeBuyBuilder { inner: TGInlineKeyboardButtonTypeBuy }

impl _TGInlineKeyboardButtonTypeBuyBuilder {

  pub fn build(&self) -> TGInlineKeyboardButtonTypeBuy { self.inner.clone() }

  

  
}

impl TGInlineKeyboardButtonTypeBuy {
  pub fn builder() -> _TGInlineKeyboardButtonTypeBuyBuilder {
    _TGInlineKeyboardButtonTypeBuyBuilder { inner: Self::new(InlineKeyboardButtonTypeBuy::_new()) }
  }
}

impl TDFB for TGInlineKeyboardButtonTypeBuy {}

impl AsRef<TGInlineKeyboardButtonTypeBuy> for TGInlineKeyboardButtonTypeBuy {
  fn as_ref(&self) -> &TGInlineKeyboardButtonTypeBuy { self }
}

impl AsRef<TGInlineKeyboardButtonTypeBuy> for _TGInlineKeyboardButtonTypeBuyBuilder {
  fn as_ref(&self) -> &TGInlineKeyboardButtonTypeBuy { &self.inner }
}


#[doc(hidden)] pub struct _TGInlineKeyboardButtonTypeCallbackBuilder { inner: TGInlineKeyboardButtonTypeCallback }

impl _TGInlineKeyboardButtonTypeCallbackBuilder {

  pub fn build(&self) -> TGInlineKeyboardButtonTypeCallback { self.inner.clone() }

  ///  Data to be sent to the bot via a callback query. 
  pub fn data<S: AsRef<str>>(&mut self, data: S) -> &mut Self {
    self.inner.td_origin_mut()._set_data(data.as_ref().to_string());
    self
  }
  

  
}

impl TGInlineKeyboardButtonTypeCallback {
  pub fn builder() -> _TGInlineKeyboardButtonTypeCallbackBuilder {
    _TGInlineKeyboardButtonTypeCallbackBuilder { inner: Self::new(InlineKeyboardButtonTypeCallback::_new()) }
  }
}

impl TDFB for TGInlineKeyboardButtonTypeCallback {}

impl AsRef<TGInlineKeyboardButtonTypeCallback> for TGInlineKeyboardButtonTypeCallback {
  fn as_ref(&self) -> &TGInlineKeyboardButtonTypeCallback { self }
}

impl AsRef<TGInlineKeyboardButtonTypeCallback> for _TGInlineKeyboardButtonTypeCallbackBuilder {
  fn as_ref(&self) -> &TGInlineKeyboardButtonTypeCallback { &self.inner }
}


#[doc(hidden)] pub struct _TGInlineKeyboardButtonTypeCallbackGameBuilder { inner: TGInlineKeyboardButtonTypeCallbackGame }

impl _TGInlineKeyboardButtonTypeCallbackGameBuilder {

  pub fn build(&self) -> TGInlineKeyboardButtonTypeCallbackGame { self.inner.clone() }

  

  
}

impl TGInlineKeyboardButtonTypeCallbackGame {
  pub fn builder() -> _TGInlineKeyboardButtonTypeCallbackGameBuilder {
    _TGInlineKeyboardButtonTypeCallbackGameBuilder { inner: Self::new(InlineKeyboardButtonTypeCallbackGame::_new()) }
  }
}

impl TDFB for TGInlineKeyboardButtonTypeCallbackGame {}

impl AsRef<TGInlineKeyboardButtonTypeCallbackGame> for TGInlineKeyboardButtonTypeCallbackGame {
  fn as_ref(&self) -> &TGInlineKeyboardButtonTypeCallbackGame { self }
}

impl AsRef<TGInlineKeyboardButtonTypeCallbackGame> for _TGInlineKeyboardButtonTypeCallbackGameBuilder {
  fn as_ref(&self) -> &TGInlineKeyboardButtonTypeCallbackGame { &self.inner }
}


#[doc(hidden)] pub struct _TGInlineKeyboardButtonTypeSwitchInlineBuilder { inner: TGInlineKeyboardButtonTypeSwitchInline }

impl _TGInlineKeyboardButtonTypeSwitchInlineBuilder {

  pub fn build(&self) -> TGInlineKeyboardButtonTypeSwitchInline { self.inner.clone() }

  ///  Inline query to be sent to the bot. 
  pub fn query<S: AsRef<str>>(&mut self, query: S) -> &mut Self {
    self.inner.td_origin_mut()._set_query(query.as_ref().to_string());
    self
  }
  ///  True, if the inline query should be sent from the current chat. 
  pub fn in_current_chat(&mut self, in_current_chat: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_in_current_chat(in_current_chat);
    self
  }
  

  
}

impl TGInlineKeyboardButtonTypeSwitchInline {
  pub fn builder() -> _TGInlineKeyboardButtonTypeSwitchInlineBuilder {
    _TGInlineKeyboardButtonTypeSwitchInlineBuilder { inner: Self::new(InlineKeyboardButtonTypeSwitchInline::_new()) }
  }
}

impl TDFB for TGInlineKeyboardButtonTypeSwitchInline {}

impl AsRef<TGInlineKeyboardButtonTypeSwitchInline> for TGInlineKeyboardButtonTypeSwitchInline {
  fn as_ref(&self) -> &TGInlineKeyboardButtonTypeSwitchInline { self }
}

impl AsRef<TGInlineKeyboardButtonTypeSwitchInline> for _TGInlineKeyboardButtonTypeSwitchInlineBuilder {
  fn as_ref(&self) -> &TGInlineKeyboardButtonTypeSwitchInline { &self.inner }
}


#[doc(hidden)] pub struct _TGInlineKeyboardButtonTypeUrlBuilder { inner: TGInlineKeyboardButtonTypeUrl }

impl _TGInlineKeyboardButtonTypeUrlBuilder {

  pub fn build(&self) -> TGInlineKeyboardButtonTypeUrl { self.inner.clone() }

  ///  HTTP or tg:// URL to open. 
  pub fn url<S: AsRef<str>>(&mut self, url: S) -> &mut Self {
    self.inner.td_origin_mut()._set_url(url.as_ref().to_string());
    self
  }
  

  
}

impl TGInlineKeyboardButtonTypeUrl {
  pub fn builder() -> _TGInlineKeyboardButtonTypeUrlBuilder {
    _TGInlineKeyboardButtonTypeUrlBuilder { inner: Self::new(InlineKeyboardButtonTypeUrl::_new()) }
  }
}

impl TDFB for TGInlineKeyboardButtonTypeUrl {}

impl AsRef<TGInlineKeyboardButtonTypeUrl> for TGInlineKeyboardButtonTypeUrl {
  fn as_ref(&self) -> &TGInlineKeyboardButtonTypeUrl { self }
}

impl AsRef<TGInlineKeyboardButtonTypeUrl> for _TGInlineKeyboardButtonTypeUrlBuilder {
  fn as_ref(&self) -> &TGInlineKeyboardButtonTypeUrl { &self.inner }
}


#[doc(hidden)] pub struct _TGKeyboardButtonBuilder { inner: TGKeyboardButton }

impl _TGKeyboardButtonBuilder {

  pub fn build(&self) -> TGKeyboardButton { self.inner.clone() }

  ///  Text of the button. 
  pub fn text<S: AsRef<str>>(&mut self, text: S) -> &mut Self {
    self.inner.td_origin_mut()._set_text(text.as_ref().to_string());
    self
  }
  

  
  // [type_] type is [Box<KeyboardButtonType>], is not support, need add manully.
  #[doc(hidden)] pub fn _type_(&mut self, type_: Box<KeyboardButtonType>) -> &mut Self {
    self.inner.td_origin_mut()._set_type_(type_);
    self
  }
  
}

impl TGKeyboardButton {
  pub fn builder() -> _TGKeyboardButtonBuilder {
    _TGKeyboardButtonBuilder { inner: Self::new(KeyboardButton::_new()) }
  }
}

impl TDFB for TGKeyboardButton {}

impl AsRef<TGKeyboardButton> for TGKeyboardButton {
  fn as_ref(&self) -> &TGKeyboardButton { self }
}

impl AsRef<TGKeyboardButton> for _TGKeyboardButtonBuilder {
  fn as_ref(&self) -> &TGKeyboardButton { &self.inner }
}


#[doc(hidden)] pub struct _TGKeyboardButtonTypeRequestLocationBuilder { inner: TGKeyboardButtonTypeRequestLocation }

impl _TGKeyboardButtonTypeRequestLocationBuilder {

  pub fn build(&self) -> TGKeyboardButtonTypeRequestLocation { self.inner.clone() }

  

  
}

impl TGKeyboardButtonTypeRequestLocation {
  pub fn builder() -> _TGKeyboardButtonTypeRequestLocationBuilder {
    _TGKeyboardButtonTypeRequestLocationBuilder { inner: Self::new(KeyboardButtonTypeRequestLocation::_new()) }
  }
}

impl TDFB for TGKeyboardButtonTypeRequestLocation {}

impl AsRef<TGKeyboardButtonTypeRequestLocation> for TGKeyboardButtonTypeRequestLocation {
  fn as_ref(&self) -> &TGKeyboardButtonTypeRequestLocation { self }
}

impl AsRef<TGKeyboardButtonTypeRequestLocation> for _TGKeyboardButtonTypeRequestLocationBuilder {
  fn as_ref(&self) -> &TGKeyboardButtonTypeRequestLocation { &self.inner }
}


#[doc(hidden)] pub struct _TGKeyboardButtonTypeRequestPhoneNumberBuilder { inner: TGKeyboardButtonTypeRequestPhoneNumber }

impl _TGKeyboardButtonTypeRequestPhoneNumberBuilder {

  pub fn build(&self) -> TGKeyboardButtonTypeRequestPhoneNumber { self.inner.clone() }

  

  
}

impl TGKeyboardButtonTypeRequestPhoneNumber {
  pub fn builder() -> _TGKeyboardButtonTypeRequestPhoneNumberBuilder {
    _TGKeyboardButtonTypeRequestPhoneNumberBuilder { inner: Self::new(KeyboardButtonTypeRequestPhoneNumber::_new()) }
  }
}

impl TDFB for TGKeyboardButtonTypeRequestPhoneNumber {}

impl AsRef<TGKeyboardButtonTypeRequestPhoneNumber> for TGKeyboardButtonTypeRequestPhoneNumber {
  fn as_ref(&self) -> &TGKeyboardButtonTypeRequestPhoneNumber { self }
}

impl AsRef<TGKeyboardButtonTypeRequestPhoneNumber> for _TGKeyboardButtonTypeRequestPhoneNumberBuilder {
  fn as_ref(&self) -> &TGKeyboardButtonTypeRequestPhoneNumber { &self.inner }
}


#[doc(hidden)] pub struct _TGKeyboardButtonTypeTextBuilder { inner: TGKeyboardButtonTypeText }

impl _TGKeyboardButtonTypeTextBuilder {

  pub fn build(&self) -> TGKeyboardButtonTypeText { self.inner.clone() }

  

  
}

impl TGKeyboardButtonTypeText {
  pub fn builder() -> _TGKeyboardButtonTypeTextBuilder {
    _TGKeyboardButtonTypeTextBuilder { inner: Self::new(KeyboardButtonTypeText::_new()) }
  }
}

impl TDFB for TGKeyboardButtonTypeText {}

impl AsRef<TGKeyboardButtonTypeText> for TGKeyboardButtonTypeText {
  fn as_ref(&self) -> &TGKeyboardButtonTypeText { self }
}

impl AsRef<TGKeyboardButtonTypeText> for _TGKeyboardButtonTypeTextBuilder {
  fn as_ref(&self) -> &TGKeyboardButtonTypeText { &self.inner }
}

