use rtdlib::types as td_types;
use rtdlib::types::InputFile;
use rtdlib::types::RObject;

use crate::errors;
use crate::types::t_file::*;

impl TGUpdateFile {
  pub fn file(&self) -> TGFile { self.td_origin().file().map(|v| TGFile::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }
}



impl TGFile {
  pub fn id(&self) -> i32 { self.td_origin().id().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn size(&self) -> i32 { self.td_origin().size().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn expected_size(&self) -> Option<i32> { self.td_origin().expected_size() }

  pub fn local(&self) -> Option<TGLocalFile> { self.td_origin().local().map(|v| TGLocalFile::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn remote(&self) -> Option<TGRemoteFile> { self.td_origin().remote().map(|v| TGRemoteFile::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }
}


impl TGLocalFile {
  pub fn path(&self) -> Option<String> { self.td_origin().path().filter(|v| !v.is_empty()) }

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



#[derive(Debug, Clone)]
pub enum TGInputFile {
  Generated(TGInputFileGenerated),
  Id       (TGInputFileId       ),
  Local    (TGInputFileLocal    ),
  Remote   (TGInputFileRemote   ),
}


impl TGInputFile {
  pub(crate) fn of(td: Box<td_types::InputFile>) -> Self {
    tuple_rtd_type_mapping!(
      InputFile,
      TGInputFile,
      RTDInputFileType,
      (InputFileGenerated , Generated , TGInputFileGenerated   );
      (InputFileId        , Id        , TGInputFileId          );
      (InputFileLocal     , Local     , TGInputFileLocal       );
      (InputFileRemote    , Remote    , TGInputFileRemote      );
    )(td)
  }

  pub fn is_generated(&self) -> bool { tuple_enum_is!(TGInputFile, Generated)(self) }
  pub fn is_id       (&self) -> bool { tuple_enum_is!(TGInputFile, Id)       (self) }
  pub fn is_local    (&self) -> bool { tuple_enum_is!(TGInputFile, Local)    (self) }
  pub fn is_remote   (&self) -> bool { tuple_enum_is!(TGInputFile, Remote)   (self) }

  pub fn on_generated<F: FnOnce(&TGInputFileGenerated)>(&self, fnc: F) -> &Self { tuple_enum_on!(TGInputFile, Generated, |t| fnc(t))(self); self }
  pub fn on_id       <F: FnOnce(&TGInputFileId)>       (&self, fnc: F) -> &Self { tuple_enum_on!(TGInputFile, Id, |t| fnc(t))       (self); self }
  pub fn on_local    <F: FnOnce(&TGInputFileLocal)>    (&self, fnc: F) -> &Self { tuple_enum_on!(TGInputFile, Local, |t| fnc(t))    (self); self }
  pub fn on_remove   <F: FnOnce(&TGInputFileRemote)>   (&self, fnc: F) -> &Self { tuple_enum_on!(TGInputFile, Remote, |t| fnc(t))   (self); self }
}


impl TGInputFileGenerated {
  pub fn original_path(&self) -> Option<String> { self.td_origin().original_path() }

  pub fn conversion(&self) -> Option<String> { self.td_origin().conversion() }

  pub fn expected_size(&self) -> Option<i32> { self.td_origin().expected_size() }
}

impl TGInputFileId {
  pub fn id(&self) -> i32 { self.td_origin().id().expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGInputFileLocal {
  pub fn path(&self) -> String { self.td_origin().path().expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGInputFileRemote {
  pub fn id(&self) -> String { self.td_origin().id().expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGInputThumbnail {
  pub fn thumbnail(&self) -> TGInputFile { self.td_origin().thumbnail().map(|v| TGInputFile::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn width(&self) -> i32 { self.td_origin().width().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn height(&self) -> i32 { self.td_origin().height().expect(errors::TELEGRAM_DATA_FAIL) }
}




