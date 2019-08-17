use rtdlib::types as td_types;
use rtdlib::types::RObject;

use crate::errors;
use crate::types::t_user::TGUser;
use crate::types::t_user::TGUserTypeBot;
use crate::types::t_user::TGUpdateUser;
use crate::types::t_user::TGUpdateUserStatus;
use crate::types::TGProfilePhoto;

impl TGUser {
  pub fn id(&self) -> i32 { self.td_origin().id().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn first_name(&self) -> Option<String> { self.td_origin().first_name() }

  pub fn last_name(&self) -> Option<String> { self.td_origin().last_name() }

  pub fn username(&self) -> Option<String> { self.td_origin().username() }

  pub fn phone_number(&self) -> String { self.td_origin().phone_number().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn status(&self) -> TGUserStatus { self.td_origin().status().map(|v| TGUserStatus::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn profile_photo(&self) -> Option<TGProfilePhoto> { self.td_origin().profile_photo().map(|v| TGProfilePhoto::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn outgoing_link(&self) -> TGLinkState { self.td_origin().outgoing_link().map(|v| TGLinkState::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn incoming_link(&self) -> TGLinkState { self.td_origin().incoming_link().map(|v| TGLinkState::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn is_verified(&self) -> bool { self.td_origin().is_verified().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn is_support(&self) -> bool { self.td_origin().is_support().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn restriction_reason(&self) -> Option<String> { self.td_origin().restriction_reason().filter(|v| !v.is_empty()) }

  pub fn have_access(&self) -> bool { self.td_origin().have_access().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn type_(&self) -> TGUserType { self.td_origin().type_().map(|v| TGUserType::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn language_code(&self) -> Option<String> { self.td_origin().language_code().filter(|v| !v.is_empty()) }

  pub fn is_bot(&self) -> bool {
    match self.type_() {
      TGUserType::Bot(_) => true,
      _ => false
    }
  }

  pub fn is_deleted(&self) -> bool {
    match self.type_() {
      TGUserType::Deleted => true,
      _ => false
    }
  }

  pub fn is_regular(&self) -> bool {
    match self.type_() {
      TGUserType::Regular => true,
      _ => false
    }
  }
}

#[derive(Debug, Clone)]
pub enum TGUserType {
  Bot(TGUserTypeBot),
  Deleted,
  Regular,
  Unknown,
}

impl TGUserType {
  fn of(td: Box<td_types::UserType>) -> Self {
    match td_types::RTDUserTypeType::of(td.td_name()) {
      Some(td_types::RTDUserTypeType::UserTypeBot) => {
        TGUserType::Bot(TGUserTypeBot::from_json(td.to_json()).expect(errors::TELEGRAM_DATA_FAIL))
      }
      Some(td_types::RTDUserTypeType::UserTypeDeleted) => TGUserType::Deleted,
      Some(td_types::RTDUserTypeType::UserTypeRegular) => TGUserType::Regular,
      Some(td_types::RTDUserTypeType::UserTypeUnknown) => TGUserType::Unknown,
      None => panic!(errors::TELEGRAM_DATA_FAIL)
    }
  }
}


/// Represents the relationship between user A and user B. For incoming_link, user A is the current user; for outgoing_link, user B is the current user.
#[derive(Debug, Clone)]
pub enum TGLinkState {
  /// /// The phone number of user A has been saved to the contact list of user B.
  IsContact,
  /// The phone number of user A is known but that number has not been saved to the contact list of user B.
  KnowsPhoneNumber,
  /// The phone number of user A is not known to user B.
  None,
}

impl TGLinkState {
  fn of(td: Box<td_types::LinkState>) -> Self {
    match td_types::RTDLinkStateType::of(td.td_name()) {
      Some(td_types::RTDLinkStateType::LinkStateIsContact) => TGLinkState::IsContact,
      Some(td_types::RTDLinkStateType::LinkStateKnowsPhoneNumber) => TGLinkState::KnowsPhoneNumber,
      Some(td_types::RTDLinkStateType::LinkStateNone) => TGLinkState::None,
      None => panic!(errors::TELEGRAM_DATA_FAIL)
    }
  }
}

/// This class is an abstract base class. Describes the last time the user was online.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum TGUserStatus {
  /// The user status was never changed.
  Empty,
  /// The user is offline, but was online last month.
  LastMonth,
  /// The user is offline, but was online last week.
  LastWeek,
  /// The user is offline.
  ///
  /// Point in time (Unix timestamp) when the user's online status will expire.
  Offline(i32),
  /// The user is online.
  ///
  /// Point in time (Unix timestamp) when the user was last online.
  Online(i32),
  /// The user was online recently.
  Recently,
}

impl TGUserStatus {
  pub(crate) fn of(td: Box<td_types::UserStatus>) -> Self {
    match td_types::RTDUserStatusType::of(td.td_name()) {
      Some(td_types::RTDUserStatusType::UserStatusEmpty) => TGUserStatus::Empty,
      Some(td_types::RTDUserStatusType::UserStatusLastMonth) => TGUserStatus::LastMonth,
      Some(td_types::RTDUserStatusType::UserStatusLastWeek) => TGUserStatus::LastWeek,
      Some(td_types::RTDUserStatusType::UserStatusOffline) => {
        td_types::UserStatusOffline::from_json(td.to_json())
          .map(|v| TGUserStatus::Offline(v.was_online().expect(errors::TELEGRAM_DATA_FAIL)))
          .expect(errors::TELEGRAM_DATA_FAIL)
      }
      Some(td_types::RTDUserStatusType::UserStatusOnline) => {
        td_types::UserStatusOnline::from_json(td.to_json())
          .map(|v| TGUserStatus::Online(v.expires().expect(errors::TELEGRAM_DATA_FAIL)))
          .expect(errors::TELEGRAM_DATA_FAIL)
      }
      Some(td_types::RTDUserStatusType::UserStatusRecently) => TGUserStatus::Recently,
      None => panic!(errors::TELEGRAM_DATA_FAIL)
    }
  }
}



impl TGUserTypeBot {
  /// True, if the bot can be invited to basic group and supergroup chats.
  pub fn can_join_groups(&self) -> bool { self.td_origin().can_join_groups().map_or(false, |v| v) }

  /// True, if the bot can read all messages in basic group or supergroup chats and not just those addressed to the bot. In private and channel chats a bot can always read all messages.
  pub fn can_read_all_group_messages(&self) -> bool { self.td_origin().can_read_all_group_messages().map_or(false, |v| v) }

  /// True, if the bot supports inline queries.
  pub fn is_inline(&self) -> bool { self.td_origin().is_inline().map_or(false, |v| v) }

  /// Placeholder for inline queries (displayed on the client input field).
  pub fn inline_query_placeholder(&self) -> Option<String> { self.td_origin().inline_query_placeholder().clone() }

  /// True, if the location of the user should be sent with every inline query to this bot.
  pub fn need_location(&self) -> bool { self.td_origin().need_location().map_or(false, |v| v) }
}



impl TGUpdateUser {

  pub fn user(&self) -> TGUser { TGUser::from_json(self.td_origin().user().expect(errors::TELEGRAM_DATA_FAIL).to_json()).expect(errors::TELEGRAM_DATA_FAIL) }

}

impl TGUpdateUserStatus {

  pub fn user_id(&self) -> i32 { self.td_origin().user_id().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn status(&self) -> TGUserStatus {
    TGUserStatus::of(self.td_origin().status().expect(errors::TELEGRAM_DATA_FAIL))
  }

}



