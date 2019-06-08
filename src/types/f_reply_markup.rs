use rtdlib::types as td_types;
use rtdlib::types::{InlineKeyboardButtonType, KeyboardButton, KeyboardButtonType, ReplyMarkup, RObject};

use crate::errors;
use crate::types::t_reply_markup::*;

#[derive(Debug, Clone)]
pub enum TGReplyMarkup {
  ForceReply(TGReplyMarkupForceReply),
  InlineKeyboard(TGReplyMarkupInlineKeyboard),
  RemoveKeyboard(TGReplyMarkupRemoveKeyboard),
  ShowKeyboard(TGReplyMarkupShowKeyboard),
}


impl TGReplyMarkup {
  pub(crate) fn of(td: Box<td_types::ReplyMarkup>) -> Self {
    tuple_rtd_type_mapping!(
      ReplyMarkup,
      TGReplyMarkup,
      RTDReplyMarkupType,
      (ReplyMarkupForceReply       ,     ForceReply          ,      TGReplyMarkupForceReply      );
      (ReplyMarkupInlineKeyboard   ,     InlineKeyboard      ,      TGReplyMarkupInlineKeyboard  );
      (ReplyMarkupRemoveKeyboard   ,     RemoveKeyboard      ,      TGReplyMarkupRemoveKeyboard  );
      (ReplyMarkupShowKeyboard     ,     ShowKeyboard        ,      TGReplyMarkupShowKeyboard    );
    )(td)
  }

  pub fn is_force_reply(&self) -> bool { tuple_enum_is!(TGReplyMarkup, ForceReply       )(self) }
  pub fn is_inline_keyboard(&self) -> bool { tuple_enum_is!(TGReplyMarkup, InlineKeyboard   )(self) }
  pub fn is_remove_keyboard(&self) -> bool { tuple_enum_is!(TGReplyMarkup, RemoveKeyboard   )(self) }
  pub fn is_show_keyboard(&self) -> bool { tuple_enum_is!(TGReplyMarkup, ShowKeyboard     )(self) }


  pub fn on_force_reply<F: FnOnce(&TGReplyMarkupForceReply)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGReplyMarkup, ForceReply         , |t| fnc(t))(self);
    self
  }
  pub fn on_inline_keyboard<F: FnOnce(&TGReplyMarkupInlineKeyboard)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGReplyMarkup, InlineKeyboard     , |t| fnc(t))(self);
    self
  }
  pub fn on_remove_keyboard<F: FnOnce(&TGReplyMarkupRemoveKeyboard)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGReplyMarkup, RemoveKeyboard     , |t| fnc(t))(self);
    self
  }
  pub fn on_show_keyboard<F: FnOnce(&TGReplyMarkupShowKeyboard)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGReplyMarkup, ShowKeyboard       , |t| fnc(t))(self);
    self
  }
}


impl TGReplyMarkupForceReply {
  pub fn is_personal(&self) -> bool { self.td_origin().is_personal().map_or(false, |v| v) }
}

impl TGReplyMarkupInlineKeyboard {
  pub fn rows(&self) -> Vec<Vec<TGInlineKeyboardButton>> {
    self.td_origin().rows().map_or(vec![], |v| v.iter()
      .map(|v| v.iter()
        .map(|v| TGInlineKeyboardButton::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL))
        .collect::<Vec<TGInlineKeyboardButton>>())
      .collect::<Vec<Vec<TGInlineKeyboardButton>>>(),
    )
  }
}

impl TGInlineKeyboardButton {
  pub fn text(&self) -> String { self.td_origin().text().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn type_(&self) -> TGInlineKeyboardButtonType { self.td_origin().type_().map(|v| TGInlineKeyboardButtonType::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }


  pub fn is_buy(&self) -> bool { self.type_().is_buy() }
  pub fn is_callback(&self) -> bool { self.type_().is_callback() }
  pub fn is_callback_game(&self) -> bool { self.type_().is_callback_game() }
  pub fn is_switch_inline(&self) -> bool { self.type_().is_switch_inline() }
  pub fn is_url(&self) -> bool { self.type_().is_url() }
}

#[derive(Debug, Clone)]
pub enum TGInlineKeyboardButtonType {
  Buy(TGInlineKeyboardButtonTypeBuy),
  Callback(TGInlineKeyboardButtonTypeCallback),
  CallbackGame(TGInlineKeyboardButtonTypeCallbackGame),
  SwitchInline(TGInlineKeyboardButtonTypeSwitchInline),
  Url(TGInlineKeyboardButtonTypeUrl),
}

impl TGInlineKeyboardButtonType {
  fn of(td: Box<InlineKeyboardButtonType>) -> Self {
    tuple_rtd_type_mapping!(
      InlineKeyboardButtonType,
      TGInlineKeyboardButtonType,
      RTDInlineKeyboardButtonTypeType,
      (InlineKeyboardButtonTypeBuy          ,  Buy           ,   TGInlineKeyboardButtonTypeBuy          );
      (InlineKeyboardButtonTypeCallback     ,  Callback      ,   TGInlineKeyboardButtonTypeCallback     );
      (InlineKeyboardButtonTypeCallbackGame ,  CallbackGame  ,   TGInlineKeyboardButtonTypeCallbackGame );
      (InlineKeyboardButtonTypeSwitchInline ,  SwitchInline  ,   TGInlineKeyboardButtonTypeSwitchInline );
      (InlineKeyboardButtonTypeUrl          ,  Url           ,   TGInlineKeyboardButtonTypeUrl          );
    )(td)
  }

  pub fn is_buy(&self) -> bool { tuple_enum_is!(TGInlineKeyboardButtonType, Buy           )(self) }
  pub fn is_callback(&self) -> bool { tuple_enum_is!(TGInlineKeyboardButtonType, Callback      )(self) }
  pub fn is_callback_game(&self) -> bool { tuple_enum_is!(TGInlineKeyboardButtonType, CallbackGame  )(self) }
  pub fn is_switch_inline(&self) -> bool { tuple_enum_is!(TGInlineKeyboardButtonType, SwitchInline  )(self) }
  pub fn is_url(&self) -> bool { tuple_enum_is!(TGInlineKeyboardButtonType, Url           )(self) }

  pub fn on_buy<F: FnOnce(&TGInlineKeyboardButtonTypeBuy)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGInlineKeyboardButtonType, Buy            , |t| fnc(t))(self);
    self
  }
  pub fn on_callback<F: FnOnce(&TGInlineKeyboardButtonTypeCallback)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGInlineKeyboardButtonType, Callback       , |t| fnc(t))(self);
    self
  }
  pub fn on_callback_game<F: FnOnce(&TGInlineKeyboardButtonTypeCallbackGame)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGInlineKeyboardButtonType, CallbackGame   , |t| fnc(t))(self);
    self
  }
  pub fn on_switch_inline<F: FnOnce(&TGInlineKeyboardButtonTypeSwitchInline)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGInlineKeyboardButtonType, SwitchInline   , |t| fnc(t))(self);
    self
  }
  pub fn on_url<F: FnOnce(&TGInlineKeyboardButtonTypeUrl)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGInlineKeyboardButtonType, Url            , |t| fnc(t))(self);
    self
  }
}


impl TGInlineKeyboardButtonTypeBuy {}

impl TGInlineKeyboardButtonTypeCallback {
  pub fn data(&self) -> String { self.td_origin().data().expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGInlineKeyboardButtonTypeCallbackGame {}

impl TGInlineKeyboardButtonTypeSwitchInline {
  pub fn query(&self) -> String { self.td_origin().query().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn in_current_chat(&self) -> bool { self.td_origin().in_current_chat().map_or(false, |v| v) }
}

impl TGInlineKeyboardButtonTypeUrl {
  pub fn url(&self) -> String { self.td_origin().url().expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGReplyMarkupRemoveKeyboard {
  pub fn is_personal(&self) -> bool { self.td_origin().is_personal().map_or(false, |v| v) }
}

impl TGReplyMarkupShowKeyboard {
  pub fn rows(&self) -> Vec<Vec<TGKeyboardButton>> {
    self.td_origin().rows().map_or(vec![], |v| v.iter()
      .map(|v| v.iter()
        .map(|v| TGKeyboardButton::from_json(v.to_json()).expect(errors::TELEGRAM_DATA_FAIL))
        .collect::<Vec<TGKeyboardButton>>())
      .collect::<Vec<Vec<TGKeyboardButton>>>(),
    )
  }

  pub fn resize_keyboard(&self) -> bool { self.td_origin().resize_keyboard().map_or(false, |v| v) }

  pub fn one_time(&self) -> bool { self.td_origin().one_time().map_or(false, |v| v) }

  pub fn is_personal(&self) -> bool { self.td_origin().is_personal().map_or(false, |v| v) }
}


impl TGKeyboardButton {
  pub fn text(&self) -> String { self.td_origin().text().expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn type_(&self) -> TGKeyboardButtonType { self.td_origin().type_().map(|v| TGKeyboardButtonType::of(v)).expect(errors::TELEGRAM_DATA_FAIL) }

  pub fn is_request_location(&self) -> bool { self.type_().is_request_location() }
  pub fn is_request_phone_number(&self) -> bool { self.type_().is_request_phone_number() }
  pub fn is_text(&self) -> bool { self.type_().is_text() }
}


#[derive(Debug, Clone)]
pub enum TGKeyboardButtonType {
  RequestLocation(TGKeyboardButtonTypeRequestLocation),
  RequestPhoneNumber(TGKeyboardButtonTypeRequestPhoneNumber),
  Text(TGKeyboardButtonTypeText),
}

impl TGKeyboardButtonType {
  fn of(td: Box<KeyboardButtonType>) -> Self {
    tuple_rtd_type_mapping!(
      KeyboardButtonType,
      TGKeyboardButtonType,
      RTDKeyboardButtonTypeType,
      (KeyboardButtonTypeRequestLocation    ,  RequestLocation    ,   TGKeyboardButtonTypeRequestLocation    );
      (KeyboardButtonTypeRequestPhoneNumber ,  RequestPhoneNumber ,   TGKeyboardButtonTypeRequestPhoneNumber );
      (KeyboardButtonTypeText               ,  Text               ,   TGKeyboardButtonTypeText               );
    )(td)
  }

  pub fn is_request_location(&self) -> bool { tuple_enum_is!(TGKeyboardButtonType, RequestLocation    )(self) }
  pub fn is_request_phone_number(&self) -> bool { tuple_enum_is!(TGKeyboardButtonType, RequestPhoneNumber )(self) }
  pub fn is_text(&self) -> bool { tuple_enum_is!(TGKeyboardButtonType, Text               )(self) }


  pub fn on_request_location<F: FnOnce(&TGKeyboardButtonTypeRequestLocation)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGKeyboardButtonType, RequestLocation    , |t| fnc(t))(self);
    self
  }
  pub fn on_request_phone_number<F: FnOnce(&TGKeyboardButtonTypeRequestPhoneNumber)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGKeyboardButtonType, RequestPhoneNumber , |t| fnc(t))(self);
    self
  }
  pub fn on_text<F: FnOnce(&TGKeyboardButtonTypeText)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGKeyboardButtonType, Text               , |t| fnc(t))(self);
    self
  }
}


impl TGKeyboardButtonTypeRequestLocation {}

impl TGKeyboardButtonTypeRequestPhoneNumber {}

impl TGKeyboardButtonTypeText {}


