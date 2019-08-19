use rtdlib::types as td_types;
use rtdlib::types::{
  RObject,
  UserStatus,
  UserType,
  LinkState,
};

use crate::errors;
use crate::types::t_user::{
  TGUpdateUser,
  TGUpdateUserStatus,
  TGUser,
  TGUserStatusEmpty,
  TGUserStatusLastMonth,
  TGUserStatusLastWeek,
  TGUserStatusOffline,
  TGUserStatusOnline,
  TGUserStatusRecently,
  TGUserTypeBot,
  TGUserTypeDeleted,
  TGUserTypeRegular,
  TGUserTypeUnknown,
};
use crate::types::TGProfilePhoto;

impl TGUser {
  pub fn id(&self) -> i32 { self.td_origin().id().expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  pub fn first_name(&self) -> Option<String> { self.td_origin().first_name() }

  pub fn last_name(&self) -> Option<String> { self.td_origin().last_name() }

  pub fn username(&self) -> Option<String> { self.td_origin().username() }

  pub fn phone_number(&self) -> String { self.td_origin().phone_number().expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  pub fn status(&self) -> TGUserStatus { self.td_origin().status().map(|v| TGUserStatus::of(v)).expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  pub fn profile_photo(&self) -> Option<TGProfilePhoto> { self.td_origin().profile_photo().map(|v| TGProfilePhoto::from_json(v.to_json()).expect(&errors::data_fail_with_rtd(self.td_origin())[..])) }

  pub fn outgoing_link(&self) -> TGLinkState { self.td_origin().outgoing_link().map(|v| TGLinkState::of(v)).expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  pub fn incoming_link(&self) -> TGLinkState { self.td_origin().incoming_link().map(|v| TGLinkState::of(v)).expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  pub fn is_verified(&self) -> bool { self.td_origin().is_verified().expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  pub fn is_support(&self) -> bool { self.td_origin().is_support().expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  pub fn restriction_reason(&self) -> Option<String> { self.td_origin().restriction_reason().filter(|v| !v.is_empty()) }

  pub fn have_access(&self) -> bool { self.td_origin().have_access().expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  pub fn type_(&self) -> TGUserType { self.td_origin().type_().map(|v| TGUserType::of(v)).expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  pub fn language_code(&self) -> Option<String> { self.td_origin().language_code().filter(|v| !v.is_empty()) }

  pub fn is_bot    (&self) -> bool { self.type_().is_bot    () }
  pub fn is_deleted(&self) -> bool { self.type_().is_deleted() }
  pub fn is_regular(&self) -> bool { self.type_().is_regular() }
  pub fn is_unknown(&self) -> bool { self.type_().is_unknown() }

}

#[derive(Debug, Clone)]
pub enum TGUserType {
  Bot     (TGUserTypeBot     ),
  Deleted (TGUserTypeDeleted ),
  Regular (TGUserTypeRegular ),
  Unknown (TGUserTypeUnknown ),
}

impl TGUserType {
  pub(crate) fn of(td: Box<td_types::UserType>) -> Self {
    tuple_rtd_type_mapping!(
      UserType,
      TGUserType,
      RTDUserTypeType,
      (UserTypeBot     , Bot    , TGUserTypeBot    );
      (UserTypeDeleted , Deleted, TGUserTypeDeleted);
      (UserTypeRegular , Regular, TGUserTypeRegular);
      (UserTypeUnknown , Unknown, TGUserTypeUnknown);
    )(td)
  }

  pub fn is_bot    (&self) -> bool { tuple_enum_is!(TGUserType, Bot    )(self) }
  pub fn is_deleted(&self) -> bool { tuple_enum_is!(TGUserType, Deleted)(self) }
  pub fn is_regular(&self) -> bool { tuple_enum_is!(TGUserType, Regular)(self) }
  pub fn is_unknown(&self) -> bool { tuple_enum_is!(TGUserType, Unknown)(self) }

  pub fn on_bot    <F: FnOnce(&TGUserTypeBot    )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGUserType, Bot    , |t| fnc(t))(self); self }
  pub fn on_deleted<F: FnOnce(&TGUserTypeDeleted)>(&self, fnc: F) -> &Self { tuple_enum_on!(TGUserType, Deleted, |t| fnc(t))(self); self }
  pub fn on_regular<F: FnOnce(&TGUserTypeRegular)>(&self, fnc: F) -> &Self { tuple_enum_on!(TGUserType, Regular, |t| fnc(t))(self); self }
  pub fn on_unknown<F: FnOnce(&TGUserTypeUnknown)>(&self, fnc: F) -> &Self { tuple_enum_on!(TGUserType, Unknown, |t| fnc(t))(self); self }
}


/// Represents the relationship between user A and user B. For incoming_link, user A is the current user; for outgoing_link, user B is the current user.
#[derive(Debug, Clone)]
pub enum TGLinkState {
  /// /// The phone number of user A has been saved to the contact list of user B.
  Contact,
  /// The phone number of user A is known but that number has not been saved to the contact list of user B.
  KnowsPhoneNumber,
  /// The phone number of user A is not known to user B.
  None,
}

impl TGLinkState {
  pub(crate) fn of(td: Box<td_types::LinkState>) -> Self {
    rtd_type_mapping!(
      LinkState,
      TGLinkState,
      RTDLinkStateType,
      (LinkStateIsContact       , Contact          );
      (LinkStateKnowsPhoneNumber, KnowsPhoneNumber );
      (LinkStateNone            , None             );
    )(td)
  }

  pub fn is_contact           (&self) -> bool { enum_is!(TGLinkState, Contact         )(self) }
  pub fn is_knows_phone_number(&self) -> bool { enum_is!(TGLinkState, KnowsPhoneNumber)(self) }
  pub fn is_none              (&self) -> bool { enum_is!(TGLinkState, None            )(self) }

  pub fn on_contact           <F: FnOnce()>(&self, fnc: F) -> &Self { enum_on!(TGLinkState, Contact         , || fnc())(self); self }
  pub fn on_knows_phone_number<F: FnOnce()>(&self, fnc: F) -> &Self { enum_on!(TGLinkState, KnowsPhoneNumber, || fnc())(self); self }
  pub fn on_none              <F: FnOnce()>(&self, fnc: F) -> &Self { enum_on!(TGLinkState, None            , || fnc())(self); self }
}

#[derive(Debug, Clone)]
pub enum TGUserStatus {
  Empty     (TGUserStatusEmpty    ),
  LastMonth (TGUserStatusLastMonth),
  LastWeek  (TGUserStatusLastWeek ),
  Offline   (TGUserStatusOffline  ),
  Online    (TGUserStatusOnline   ),
  Recently  (TGUserStatusRecently ),
}

impl TGUserStatus {
  pub(crate) fn of(td: Box<td_types::UserStatus>) -> Self {
    tuple_rtd_type_mapping!(
      UserStatus,
      TGUserStatus,
      RTDUserStatusType,
      (UserStatusEmpty     , Empty    , TGUserStatusEmpty    );
      (UserStatusLastMonth , LastMonth, TGUserStatusLastMonth);
      (UserStatusLastWeek  , LastWeek , TGUserStatusLastWeek );
      (UserStatusOffline   , Offline  , TGUserStatusOffline  );
      (UserStatusOnline    , Online   , TGUserStatusOnline   );
      (UserStatusRecently  , Recently , TGUserStatusRecently );
    )(td)
  }

  pub fn is_empty      (&self) -> bool { tuple_enum_is!(TGUserStatus, Empty    )(self) }
  pub fn is_last_month (&self) -> bool { tuple_enum_is!(TGUserStatus, LastMonth)(self) }
  pub fn is_last_week  (&self) -> bool { tuple_enum_is!(TGUserStatus, LastWeek )(self) }
  pub fn is_offline    (&self) -> bool { tuple_enum_is!(TGUserStatus, Offline  )(self) }
  pub fn is_online     (&self) -> bool { tuple_enum_is!(TGUserStatus, Online   )(self) }
  pub fn is_recently   (&self) -> bool { tuple_enum_is!(TGUserStatus, Recently )(self) }

  pub fn on_empty      <F: FnOnce(&TGUserStatusEmpty    )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGUserStatus, Empty    , |t| fnc(t))(self); self }
  pub fn on_last_month <F: FnOnce(&TGUserStatusLastMonth)>(&self, fnc: F) -> &Self { tuple_enum_on!(TGUserStatus, LastMonth, |t| fnc(t))(self); self }
  pub fn on_last_week  <F: FnOnce(&TGUserStatusLastWeek )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGUserStatus, LastWeek , |t| fnc(t))(self); self }
  pub fn on_offline    <F: FnOnce(&TGUserStatusOffline  )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGUserStatus, Offline  , |t| fnc(t))(self); self }
  pub fn on_online     <F: FnOnce(&TGUserStatusOnline   )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGUserStatus, Online   , |t| fnc(t))(self); self }
  pub fn on_recently   <F: FnOnce(&TGUserStatusRecently )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGUserStatus, Recently , |t| fnc(t))(self); self }
}

impl TGUserStatusEmpty {}

impl TGUserStatusLastMonth {}

impl TGUserStatusLastWeek {}

impl TGUserStatusOffline {
  pub fn was_online(&self) -> i32 { self.td_origin().was_online().map_or(0, |v| v) }
}

impl TGUserStatusOnline {
  pub fn expires(&self) -> i32 { self.td_origin().expires().map_or(0, |v| v) }
}

impl TGUserStatusRecently {}



impl TGUserTypeBot {
  /// True, if the bot can be invited to basic group and supergroup chats.
  pub fn can_join_groups(&self) -> bool { self.td_origin().can_join_groups().map_or(false, |v| v) }

  /// True, if the bot can read all messages in basic group or supergroup chats and not just those addressed to the bot. In private and channel chats a bot can always read all messages.
  pub fn can_read_all_group_messages(&self) -> bool { self.td_origin().can_read_all_group_messages().map_or(false, |v| v) }

  /// True, if the bot supports inline queries.
  pub fn is_inline(&self) -> bool { self.td_origin().is_inline().map_or(false, |v| v) }

  /// Placeholder for inline queries (displayed on the client input field).
  pub fn inline_query_placeholder(&self) -> Option<String> { self.td_origin().inline_query_placeholder() }

  /// True, if the location of the user should be sent with every inline query to this bot.
  pub fn need_location(&self) -> bool { self.td_origin().need_location().map_or(false, |v| v) }
}



impl TGUpdateUser {

  pub fn user(&self) -> TGUser { TGUser::from_json(self.td_origin().user().expect(&errors::data_fail_with_rtd(self.td_origin())[..]).to_json()).expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

}

impl TGUpdateUserStatus {

  pub fn user_id(&self) -> i32 { self.td_origin().user_id().expect(&errors::data_fail_with_rtd(self.td_origin())[..]) }

  pub fn status(&self) -> TGUserStatus {
    TGUserStatus::of(self.td_origin().status().expect(&errors::data_fail_with_rtd(self.td_origin())[..]))
  }

}



