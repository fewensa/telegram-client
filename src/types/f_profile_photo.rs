use crate::types::t_profile_photo::TGProfilePhoto;
use rtdlib::types::File;
use crate::errors;

impl TGProfilePhoto {


  pub fn id(&self) -> i64 { self.td_origin().id().map(|v| toolkit::number::as_i64(v).expect(&errors::data_fail_with_rtd(self.td_origin())[..])).expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  pub fn small(&self) -> Option<File> { self.td_origin().small() }

  pub fn big(&self) -> Option<File> { self.td_origin().big() }


}

