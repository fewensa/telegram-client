use rtdlib::types as td_type;
use rtdlib::types as td_types;
use rtdlib::types::ConnectionState;

use crate::errors;
use crate::types::TGUpdateConnectionState;

impl TGUpdateConnectionState {
  pub fn state(&self) -> TGConnectionState { self.td_origin().state().map(|v| TGConnectionState::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }


  pub fn is_waiting_for_network(&self) -> bool { self.state().is_waiting_for_network() }
  pub fn is_connecting_to_proxy(&self) -> bool { self.state().is_connecting_to_proxy() }
  pub fn is_connecting(&self) -> bool { self.state().is_connecting() }
  pub fn is_updating(&self) -> bool { self.state().is_updating() }
  pub fn is_ready(&self) -> bool { self.state().is_ready() }

  pub fn on_state<F: FnOnce(&TGConnectionState)>(&self, fnc: F) -> &Self {
    fnc(&self.state());
    self
  }
}

#[derive(Debug, Clone)]
pub enum TGConnectionState {
  WaitingForNetwork,
  ConnectingToProxy,
  Connecting,
  Updating,
  Ready,
}

impl TGConnectionState {
  fn of(td: Box<td_type::ConnectionState>) -> Self {
    rtd_type_mapping!(
      ConnectionState,
      TGConnectionState,
      RTDConnectionStateType,
      (ConnectionStateConnecting           , WaitingForNetwork );
      (ConnectionStateConnectingToProxy    , ConnectingToProxy );
      (ConnectionStateReady                , Connecting        );
      (ConnectionStateUpdating             , Updating          );
      (ConnectionStateWaitingForNetwork    , Ready             );
    )(td)
  }

  pub fn is_waiting_for_network(&self) -> bool { enum_is!(TGConnectionState, WaitingForNetwork)(self) }
  pub fn is_connecting_to_proxy(&self) -> bool { enum_is!(TGConnectionState, ConnectingToProxy)(self) }
  pub fn is_connecting(&self) -> bool { enum_is!(TGConnectionState, Connecting       )(self) }
  pub fn is_updating(&self) -> bool { enum_is!(TGConnectionState, Updating         )(self) }
  pub fn is_ready(&self) -> bool { enum_is!(TGConnectionState, Ready            )(self) }
}

