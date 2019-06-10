use rtdlib::types::RObject;

use crate::errors;
use crate::types::TGFile;
use crate::types::TGUpdateFile;

impl TGUpdateFile {
  pub fn file(&self) -> TGFile { self.td_origin().file().map(|v| TGFile::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }
}


