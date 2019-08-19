use rtdlib::types as td_types;
use rtdlib::types::RObject;

use crate::errors;
use crate::types::t_group::{TGSupergroupFullInfo, TGUpdateSupergroupFullInfo};

impl TGUpdateSupergroupFullInfo {
  pub fn supergroup_id(&self) -> i32 { self.td_origin().supergroup_id().expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }


  pub fn supergroup_full_info(&self) -> TGSupergroupFullInfo { TGSupergroupFullInfo::from_json(self.td_origin().supergroup_full_info().expect(&errors::data_fail_with_rtd(self.td_origin())[..]).to_json()).expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }
}


impl TGSupergroupFullInfo {
  pub fn description(&self) -> Option<String> { self.td_origin().description() }

  pub fn member_count(&self) -> i32 { self.td_origin().member_count().map_or(0, |v| v) }

  pub fn administrator_count(&self) -> i32 { self.td_origin().administrator_count().map_or(0, |v| v) }

  pub fn restricted_count(&self) -> i32 { self.td_origin().restricted_count().map_or(0, |v| v) }

  pub fn banned_count(&self) -> i32 { self.td_origin().banned_count().map_or(0, |v| v) }

  pub fn can_get_members(&self) -> bool { self.td_origin().can_get_members().map_or(false, |v| v) }

  pub fn can_set_username(&self) -> bool { self.td_origin().can_set_username().map_or(false, |v| v) }

  pub fn can_set_sticker_set(&self) -> bool { self.td_origin().can_set_sticker_set().map_or(false, |v| v) }

  pub fn can_view_statistics(&self) -> bool { self.td_origin().can_view_statistics().map_or(false, |v| v) }

  pub fn is_all_history_available(&self) -> bool { self.td_origin().is_all_history_available().map_or(false, |v| v) }

  pub fn sticker_set_id(&self) -> i64 { self.td_origin().sticker_set_id().map_or(0, |v| toolkit::number::as_i64(v).expect(&errors::data_fail_with_rtd(self.td_origin())[..])) }

  pub fn invite_link(&self) -> Option<String> { self.td_origin().invite_link() }

  pub fn upgraded_from_basic_group_id(&self) -> i32 { self.td_origin().upgraded_from_basic_group_id().map_or(0, |v| v) }

  pub fn upgraded_from_max_message_id(&self) -> i64 { self.td_origin().upgraded_from_max_message_id().map_or(0, |v| v) }
}

