use crate::api::*;
use rtdlib::types as td_types;

impl TGGetRemoteFile {

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
      TGFileType::Animation => Box::new(td_types::FileTypeAnimation::builder().build()),
      TGFileType::Audio => Box::new(td_types::FileTypeAudio::builder().build()),
      TGFileType::Document => Box::new(td_types::FileTypeDocument::builder().build()),
      TGFileType::None => Box::new(td_types::FileTypeNone::builder().build()),
      TGFileType::Photo => Box::new(td_types::FileTypePhoto::builder().build()),
      TGFileType::ProfilePhoto => Box::new(td_types::FileTypeProfilePhoto::builder().build()),
      TGFileType::Secret => Box::new(td_types::FileTypeSecret::builder().build()),
      TGFileType::SecretThumbnail => Box::new(td_types::FileTypeSecretThumbnail::builder().build()),
      TGFileType::Secure => Box::new(td_types::FileTypeSecure::builder().build()),
      TGFileType::Sticker => Box::new(td_types::FileTypeSticker::builder().build()),
      TGFileType::Thumbnail => Box::new(td_types::FileTypeThumbnail::builder().build()),
      TGFileType::Unknown => Box::new(td_types::FileTypeUnknown::builder().build()),
      TGFileType::Video => Box::new(td_types::FileTypeVideo::builder().build()),
      TGFileType::VideoNote => Box::new(td_types::FileTypeVideoNote::builder().build()),
      TGFileType::VoiceNote => Box::new(td_types::FileTypeVoiceNote::builder().build()),
      TGFileType::Wallpaper => Box::new(td_types::FileTypeWallpaper::builder().build()),
    }
  }
}
