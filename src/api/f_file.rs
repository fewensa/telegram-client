use crate::api::*;
use rtdlib::types as td_types;

impl _TGGetRemoteFileBuilder {

  pub fn file_type(&mut self, file_type: TGFileType) -> &mut Self {
    self._file_type(file_type.build())
  }


}


#[derive(Debug, Clone)]
pub enum TGFileType {
  Animation,
  Audio,
  Document,
  None,
  Photo,
  ProfilePhoto,
  Secret,
  SecretThumbnail,
  Secure,
  Sticker,
  Thumbnail,
  Unknown,
  Video,
  VideoNote,
  VoiceNote,
  Wallpaper,
}

impl TDFB for TGFileType {}

impl AsRef<TGFileType> for TGFileType {
  fn as_ref(&self) -> &TGFileType { self }
}

impl TGFileType {
  #[doc(hidden)]
  pub fn build(&self) -> Box<td_types::FileType> {
    match self {
      TGFileType::Animation => Box::new(td_types::FileTypeAnimation::_new()),
      TGFileType::Audio => Box::new(td_types::FileTypeAudio::_new()),
      TGFileType::Document => Box::new(td_types::FileTypeDocument::_new()),
      TGFileType::None => Box::new(td_types::FileTypeNone::_new()),
      TGFileType::Photo => Box::new(td_types::FileTypePhoto::_new()),
      TGFileType::ProfilePhoto => Box::new(td_types::FileTypeProfilePhoto::_new()),
      TGFileType::Secret => Box::new(td_types::FileTypeSecret::_new()),
      TGFileType::SecretThumbnail => Box::new(td_types::FileTypeSecretThumbnail::_new()),
      TGFileType::Secure => Box::new(td_types::FileTypeSecure::_new()),
      TGFileType::Sticker => Box::new(td_types::FileTypeSticker::_new()),
      TGFileType::Thumbnail => Box::new(td_types::FileTypeThumbnail::_new()),
      TGFileType::Unknown => Box::new(td_types::FileTypeUnknown::_new()),
      TGFileType::Video => Box::new(td_types::FileTypeVideo::_new()),
      TGFileType::VideoNote => Box::new(td_types::FileTypeVideoNote::_new()),
      TGFileType::VoiceNote => Box::new(td_types::FileTypeVoiceNote::_new()),
      TGFileType::Wallpaper => Box::new(td_types::FileTypeWallpaper::_new()),
    }
  }
}
