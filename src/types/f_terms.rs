use rtdlib::types as td_types;

use crate::errors;
use crate::types::t_terms::*;
use crate::types::TGFormattedText;
use rtdlib::types::RObject;


impl TGTermsOfService {

  pub fn text(&self) -> TGFormattedText { self.td_origin().text().map(|v| TGFormattedText::from_json(v.to_json()).expect(&errors::data_fail_with_rtd(self.td_origin())[..])).expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  pub fn min_user_age(&self) -> i32 { self.td_origin().min_user_age().map_or(0, |v| v) }

  pub fn show_popup(&self) -> bool { self.td_origin().show_popup().map_or(false, |v| v) }

}

impl TGUpdateTermsOfService {

  pub fn terms_of_service_id(&self) -> String { self.td_origin().terms_of_service_id().expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  pub fn terms_of_service(&self) -> TGTermsOfService { self.td_origin().terms_of_service().map(|v| TGTermsOfService::from_json(v.to_json()).expect(&errors::data_fail_with_rtd(self.td_origin())[..])).expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

}

