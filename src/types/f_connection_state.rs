use crate::types::TGUpdateConnectionState;
use rtdlib::types as td_type;

impl TGUpdateConnectionState {
  pub fn on_state<F: FnOnce(TGConnectionState)>(&self, fnc: F) -> &Self {
    self.td_origin().state().clone().map(|val| {
      TGConnectionState::of(val).map(|state| fnc(state))
    });
    self
  }
}

pub enum TGConnectionState {
  WaitingForNetwork,
  ConnectingToProxy,
  Connecting,
  Updating,
  Ready,
}

impl TGConnectionState {
  fn of(state: Box<td_type::ConnectionState>) -> Option<Self> {
    match td_type::RTDConnectionStateType::of(state.td_name()) {
      Some(td_type::RTDConnectionStateType::ConnectionStateConnecting) => Some(TGConnectionState::Connecting),
      Some(td_type::RTDConnectionStateType::ConnectionStateConnectingToProxy) => Some(TGConnectionState::ConnectingToProxy),
      Some(td_type::RTDConnectionStateType::ConnectionStateReady) => Some(TGConnectionState::Ready),
      Some(td_type::RTDConnectionStateType::ConnectionStateUpdating) => Some(TGConnectionState::Updating),
      Some(td_type::RTDConnectionStateType::ConnectionStateWaitingForNetwork) => Some(TGConnectionState::WaitingForNetwork),
      None => None
    }
  }
}

