use crate::types::t_profile_photo::TGProfilePhoto;
use rtdlib::types::File;
use crate::errors;

impl TGProfilePhoto {


  pub fn id(&self) -> i64 { self.origin().id().map(|v| toolkit::number::as_i64(v).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn small(&self) -> Option<File> { self.origin().small() }

  pub fn big(&self) -> Option<File> { self.origin().big() }


}

