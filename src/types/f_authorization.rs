use rtdlib::types as td_types;
use rtdlib::types::{AuthenticationCodeType, AuthorizationState, RObject, RTDAuthenticationCodeTypeType, RTDAuthorizationStateType};

use crate::errors;
use crate::types::t_authorization::*;
use crate::types::TGTermsOfService;


impl TGUpdateAuthorizationState {
  pub fn authorization_state(&self) -> TGAuthorizationState { self.td_origin().authorization_state().map(|v| TGAuthorizationState::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn is_closed                 (&self) -> bool { self.authorization_state().is_closed                () }
  pub fn is_closing                (&self) -> bool { self.authorization_state().is_closing               () }
  pub fn is_logging_out            (&self) -> bool { self.authorization_state().is_logging_out           () }
  pub fn is_ready                  (&self) -> bool { self.authorization_state().is_ready                 () }
  pub fn is_wait_code              (&self) -> bool { self.authorization_state().is_wait_code             () }
  pub fn is_wait_encryption_key    (&self) -> bool { self.authorization_state().is_wait_encryption_key   () }
  pub fn is_wait_password          (&self) -> bool { self.authorization_state().is_wait_password         () }
  pub fn is_wait_phone_number      (&self) -> bool { self.authorization_state().is_wait_phone_number     () }
  pub fn is_wait_tdlib_parameters  (&self) -> bool { self.authorization_state().is_wait_tdlib_parameters () }

  pub fn on_closed                  <F: FnOnce(&TGAuthorizationStateClosed             )>(&self, fnc: F) -> &Self { self.authorization_state().on_closed               (fnc); self }
  pub fn on_closing                 <F: FnOnce(&TGAuthorizationStateClosing            )>(&self, fnc: F) -> &Self { self.authorization_state().on_closing              (fnc); self }
  pub fn on_logging_out             <F: FnOnce(&TGAuthorizationStateLoggingOut         )>(&self, fnc: F) -> &Self { self.authorization_state().on_logging_out          (fnc); self }
  pub fn on_ready                   <F: FnOnce(&TGAuthorizationStateReady              )>(&self, fnc: F) -> &Self { self.authorization_state().on_ready                (fnc); self }
  pub fn on_wait_code               <F: FnOnce(&TGAuthorizationStateWaitCode           )>(&self, fnc: F) -> &Self { self.authorization_state().on_wait_code            (fnc); self }
  pub fn on_wait_encryption_key     <F: FnOnce(&TGAuthorizationStateWaitEncryptionKey  )>(&self, fnc: F) -> &Self { self.authorization_state().on_wait_encryption_key  (fnc); self }
  pub fn on_wait_password           <F: FnOnce(&TGAuthorizationStateWaitPassword       )>(&self, fnc: F) -> &Self { self.authorization_state().on_wait_password        (fnc); self }
  pub fn on_wait_phone_number       <F: FnOnce(&TGAuthorizationStateWaitPhoneNumber    )>(&self, fnc: F) -> &Self { self.authorization_state().on_wait_phone_number    (fnc); self }
  pub fn on_wait_tdlib_parameters   <F: FnOnce(&TGAuthorizationStateWaitTdlibParameters)>(&self, fnc: F) -> &Self { self.authorization_state().on_wait_tdlib_parameters(fnc); self }
}

pub enum TGAuthorizationState {
  Closed               (TGAuthorizationStateClosed                ),
  Closing              (TGAuthorizationStateClosing               ),
  LoggingOut           (TGAuthorizationStateLoggingOut            ),
  Ready                (TGAuthorizationStateReady                 ),
  WaitCode             (TGAuthorizationStateWaitCode              ),
  WaitEncryptionKey    (TGAuthorizationStateWaitEncryptionKey     ),
  WaitPassword         (TGAuthorizationStateWaitPassword          ),
  WaitPhoneNumber      (TGAuthorizationStateWaitPhoneNumber       ),
  WaitTdlibParameters  (TGAuthorizationStateWaitTdlibParameters   ),
}

impl TGAuthorizationState {
  pub(crate) fn of(td: Box<td_types::AuthorizationState>) -> Self {
    tuple_rtd_type_mapping!(
      AuthorizationState,
      TGAuthorizationState,
      RTDAuthorizationStateType,
      (AuthorizationStateClosed                ,  Closed               , TGAuthorizationStateClosed               );
      (AuthorizationStateClosing               ,  Closing              , TGAuthorizationStateClosing              );
      (AuthorizationStateLoggingOut            ,  LoggingOut           , TGAuthorizationStateLoggingOut           );
      (AuthorizationStateReady                 ,  Ready                , TGAuthorizationStateReady                );
      (AuthorizationStateWaitCode              ,  WaitCode             , TGAuthorizationStateWaitCode             );
      (AuthorizationStateWaitEncryptionKey     ,  WaitEncryptionKey    , TGAuthorizationStateWaitEncryptionKey    );
      (AuthorizationStateWaitPassword          ,  WaitPassword         , TGAuthorizationStateWaitPassword         );
      (AuthorizationStateWaitPhoneNumber       ,  WaitPhoneNumber      , TGAuthorizationStateWaitPhoneNumber      );
      (AuthorizationStateWaitTdlibParameters   ,  WaitTdlibParameters  , TGAuthorizationStateWaitTdlibParameters  );
    )(td)
  }

  pub fn is_closed                 (&self) -> bool { tuple_enum_is!(TGAuthorizationState, Closed             )(self) }
  pub fn is_closing                (&self) -> bool { tuple_enum_is!(TGAuthorizationState, Closing            )(self) }
  pub fn is_logging_out            (&self) -> bool { tuple_enum_is!(TGAuthorizationState, LoggingOut         )(self) }
  pub fn is_ready                  (&self) -> bool { tuple_enum_is!(TGAuthorizationState, Ready              )(self) }
  pub fn is_wait_code              (&self) -> bool { tuple_enum_is!(TGAuthorizationState, WaitCode           )(self) }
  pub fn is_wait_encryption_key    (&self) -> bool { tuple_enum_is!(TGAuthorizationState, WaitEncryptionKey  )(self) }
  pub fn is_wait_password          (&self) -> bool { tuple_enum_is!(TGAuthorizationState, WaitPassword       )(self) }
  pub fn is_wait_phone_number      (&self) -> bool { tuple_enum_is!(TGAuthorizationState, WaitPhoneNumber    )(self) }
  pub fn is_wait_tdlib_parameters  (&self) -> bool { tuple_enum_is!(TGAuthorizationState, WaitTdlibParameters)(self) }

  pub fn on_closed                  <F: FnOnce(&TGAuthorizationStateClosed             )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGAuthorizationState, Closed             , |t| fnc(t))(self); self }
  pub fn on_closing                 <F: FnOnce(&TGAuthorizationStateClosing            )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGAuthorizationState, Closing            , |t| fnc(t))(self); self }
  pub fn on_logging_out             <F: FnOnce(&TGAuthorizationStateLoggingOut         )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGAuthorizationState, LoggingOut         , |t| fnc(t))(self); self }
  pub fn on_ready                   <F: FnOnce(&TGAuthorizationStateReady              )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGAuthorizationState, Ready              , |t| fnc(t))(self); self }
  pub fn on_wait_code               <F: FnOnce(&TGAuthorizationStateWaitCode           )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGAuthorizationState, WaitCode           , |t| fnc(t))(self); self }
  pub fn on_wait_encryption_key     <F: FnOnce(&TGAuthorizationStateWaitEncryptionKey  )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGAuthorizationState, WaitEncryptionKey  , |t| fnc(t))(self); self }
  pub fn on_wait_password           <F: FnOnce(&TGAuthorizationStateWaitPassword       )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGAuthorizationState, WaitPassword       , |t| fnc(t))(self); self }
  pub fn on_wait_phone_number       <F: FnOnce(&TGAuthorizationStateWaitPhoneNumber    )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGAuthorizationState, WaitPhoneNumber    , |t| fnc(t))(self); self }
  pub fn on_wait_tdlib_parameters   <F: FnOnce(&TGAuthorizationStateWaitTdlibParameters)>(&self, fnc: F) -> &Self { tuple_enum_on!(TGAuthorizationState, WaitTdlibParameters, |t| fnc(t))(self); self }
}

impl TGAuthorizationStateClosed {}

impl TGAuthorizationStateClosing {}

impl TGAuthorizationStateLoggingOut {}

impl TGAuthorizationStateReady {}

impl TGAuthorizationStateWaitCode {
  pub fn is_registered(&self) -> bool { self.td_origin().is_registered().map_or(false, |v| v) }

  pub fn terms_of_service(&self) -> Option<TGTermsOfService> { self.td_origin().terms_of_service().map(|v| TGTermsOfService::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn code_info(&self) -> TGAuthenticationCodeInfo { self.td_origin().code_info().map(|v| TGAuthenticationCodeInfo::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL)).expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGAuthorizationStateWaitEncryptionKey {
  pub fn is_encrypted(&self) -> bool { self.td_origin().is_encrypted().map_or(false, |v| v) }
}

impl TGAuthorizationStateWaitPassword {
  pub fn password_hint(&self) -> Option<String> { self.td_origin().password_hint() }

  pub fn has_recovery_email_address(&self) -> bool { self.td_origin().has_recovery_email_address().map_or(false, |v| v) }

  pub fn recovery_email_address_pattern(&self) -> Option<String> { self.td_origin().recovery_email_address_pattern() }
}

impl TGAuthorizationStateWaitPhoneNumber {}

impl TGAuthorizationStateWaitTdlibParameters {}


impl TGAuthenticationCodeInfo {

  pub fn phone_number(&self) -> String { self.td_origin().phone_number().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn type_(&self) -> TGAuthenticationCodeType { self.td_origin().type_().map(|v| TGAuthenticationCodeType::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn next_type(&self) -> Option<TGAuthenticationCodeType> { self.td_origin().next_type().map(|v| TGAuthenticationCodeType::of(v)) }

  pub fn timeout(&self) -> i32 { self.td_origin().timeout().map_or(0, |v| v) }

}

pub enum TGAuthenticationCodeType {
  Call             (TGAuthenticationCodeTypeCall             ),
  FlashCall        (TGAuthenticationCodeTypeFlashCall        ),
  Sms              (TGAuthenticationCodeTypeSms              ),
  TelegramMessage  (TGAuthenticationCodeTypeTelegramMessage  ),
}

impl TGAuthenticationCodeType {
  pub(crate) fn of(td: Box<td_types::AuthenticationCodeType>) -> Self {
    tuple_rtd_type_mapping!(
      AuthenticationCodeType,
      TGAuthenticationCodeType,
      RTDAuthenticationCodeTypeType,
      (AuthenticationCodeTypeCall              ,  Call            , TGAuthenticationCodeTypeCall             );
      (AuthenticationCodeTypeFlashCall         ,  FlashCall       , TGAuthenticationCodeTypeFlashCall        );
      (AuthenticationCodeTypeSms               ,  Sms             , TGAuthenticationCodeTypeSms              );
      (AuthenticationCodeTypeTelegramMessage   ,  TelegramMessage , TGAuthenticationCodeTypeTelegramMessage  );
    )(td)
  }

  pub fn is_call             (&self) -> bool { tuple_enum_is!(TGAuthenticationCodeType, Call           )(self) }
  pub fn is_flash_call       (&self) -> bool { tuple_enum_is!(TGAuthenticationCodeType, FlashCall      )(self) }
  pub fn is_sms              (&self) -> bool { tuple_enum_is!(TGAuthenticationCodeType, Sms            )(self) }
  pub fn is_telegram_message (&self) -> bool { tuple_enum_is!(TGAuthenticationCodeType, TelegramMessage)(self) }

  pub fn on_call              <F: FnOnce(&TGAuthenticationCodeTypeCall           )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGAuthenticationCodeType, Call           , |t| fnc(t))(self); self }
  pub fn on_flash_call        <F: FnOnce(&TGAuthenticationCodeTypeFlashCall      )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGAuthenticationCodeType, FlashCall      , |t| fnc(t))(self); self }
  pub fn on_sms               <F: FnOnce(&TGAuthenticationCodeTypeSms            )>(&self, fnc: F) -> &Self { tuple_enum_on!(TGAuthenticationCodeType, Sms            , |t| fnc(t))(self); self }
  pub fn on_telegram_message  <F: FnOnce(&TGAuthenticationCodeTypeTelegramMessage)>(&self, fnc: F) -> &Self { tuple_enum_on!(TGAuthenticationCodeType, TelegramMessage, |t| fnc(t))(self); self }

}

impl TGAuthenticationCodeTypeCall {
  pub fn length(&self) -> i32 { self.td_origin().length().map_or(0, |v| v) }
}

impl TGAuthenticationCodeTypeFlashCall {
  pub fn pattern(&self) -> String { self.td_origin().pattern().expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGAuthenticationCodeTypeSms {
  pub fn length(&self) -> i32 { self.td_origin().length().map_or(0, |v| v) }
}

impl TGAuthenticationCodeTypeTelegramMessage {
  pub fn length(&self) -> i32 { self.td_origin().length().map_or(0, |v| v) }
}

