use rtdlib::types as td_types;

use crate::errors;
use crate::types::t_input_file::*;
use rtdlib::types::InputFile;


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

  pub fn is_generated(&self) -> bool {
    tuple_enum_is!(TGInputFile, Generated)(self)
  }

  pub fn is_id(&self) -> bool {
    tuple_enum_is!(TGInputFile, Id)(self)
  }

  pub fn is_local(&self) -> bool {
    tuple_enum_is!(TGInputFile, Local)(self)
  }

  pub fn is_remote(&self) -> bool {
    tuple_enum_is!(TGInputFile, Remote)(self)
  }

  pub fn on_generated<F: FnOnce(&TGInputFileGenerated)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGInputFile, Generated, |t| fnc(t))(self);
    self
  }

  pub fn on_id<F: FnOnce(&TGInputFileId)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGInputFile, Id, |t| fnc(t))(self);
    self
  }

  pub fn on_local<F: FnOnce(&TGInputFileLocal)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGInputFile, Local, |t| fnc(t))(self);
    self
  }

  pub fn on_remove<F: FnOnce(&TGInputFileRemote)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGInputFile, Remote, |t| fnc(t))(self);
    self
  }
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



