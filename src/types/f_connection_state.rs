use crate::types::TGUpdateConnectionState;
use rtdlib::types as td_type;

impl TGUpdateConnectionState {
  pub fn on_state<F: FnOnce(TGConnType)>(&self, fnc: F) -> &Self {
    self.origin().state().clone().map(|val| {
      let state = TGConnectState::of(val);
      fnc(state);
    });
    self
  }
}

pub enum TGConnectState {
  WaitingForNetwork,
  ConnectingToProxy,
  Connecting,
  Updating,
  Ready
}

impl TGConnectState {
  fn of(state: Box<td_type::ConnectionState>) -> Self {
    match td_type::RTDConnectionStateType::of(state.td_name()) {
      td_type::RTDConnectionStateType::ConnectionStateConnecting => TGConnectState::Connecting,
      td_type::RTDConnectionStateType::ConnectionStateConnectingToProxy => TGConnectState::ConnectingToProxy,
      td_type::RTDConnectionStateType::ConnectionStateReady => TGConnectState::Ready,
      td_type::RTDConnectionStateType::ConnectionStateUpdating => TGConnectState::Updating,
      td_type::RTDConnectionStateType::ConnectionStateWaitingForNetwork => TGConnectState::WaitingForNetwork,
    }
  }
}

