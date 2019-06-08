use rtdlib::types as td_types;
use rtdlib::types::{RObject, TextEntityType};

use crate::errors;
use crate::types::t_text_entity_type::*;

pub enum TGTextEntityType {
  Bold(TGTextEntityTypeBold),
  BotCommand(TGTextEntityTypeBotCommand),
  Cashtag(TGTextEntityTypeCashtag),
  Code(TGTextEntityTypeCode),
  EmailAddress(TGTextEntityTypeEmailAddress),
  Hashtag(TGTextEntityTypeHashtag),
  Italic(TGTextEntityTypeItalic),
  Mention(TGTextEntityTypeMention),
  MentionName(TGTextEntityTypeMentionName),
  PhoneNumber(TGTextEntityTypePhoneNumber),
  Pre(TGTextEntityTypePre),
  PreCode(TGTextEntityTypePreCode),
  TextUrl(TGTextEntityTypeTextUrl),
  Url(TGTextEntityTypeUrl),
}

impl TGTextEntityType {
  pub(crate) fn of(td: Box<td_types::TextEntityType>) -> Self {
    tuple_rtd_type_mapping!(
      TextEntityType,
      TGTextEntityType,
      RTDTextEntityTypeType,
      (TextEntityTypeBold,          Bold,         TGTextEntityTypeBold         );
      (TextEntityTypeBotCommand,    BotCommand,   TGTextEntityTypeBotCommand   );
      (TextEntityTypeCashtag,       Cashtag,      TGTextEntityTypeCashtag      );
      (TextEntityTypeCode,          Code,         TGTextEntityTypeCode         );
      (TextEntityTypeEmailAddress,  EmailAddress, TGTextEntityTypeEmailAddress );
      (TextEntityTypeHashtag,       Hashtag,      TGTextEntityTypeHashtag      );
      (TextEntityTypeItalic,        Italic,       TGTextEntityTypeItalic       );
      (TextEntityTypeMention,       Mention,      TGTextEntityTypeMention      );
      (TextEntityTypeMentionName,   MentionName,  TGTextEntityTypeMentionName  );
      (TextEntityTypePhoneNumber,   PhoneNumber,  TGTextEntityTypePhoneNumber  );
      (TextEntityTypePre,           Pre,          TGTextEntityTypePre          );
      (TextEntityTypePreCode,       PreCode,      TGTextEntityTypePreCode      );
      (TextEntityTypeTextUrl,       TextUrl,      TGTextEntityTypeTextUrl      );
      (TextEntityTypeUrl,           Url,          TGTextEntityTypeUrl          );
    )(td)
  }

  pub fn is_bold(&self) -> bool { tuple_enum_is!(TGTextEntityType, Bold        )(self) }
  pub fn is_bot_command(&self) -> bool { tuple_enum_is!(TGTextEntityType, BotCommand  )(self) }
  pub fn is_cashtag(&self) -> bool { tuple_enum_is!(TGTextEntityType, Cashtag     )(self) }
  pub fn is_code(&self) -> bool { tuple_enum_is!(TGTextEntityType, Code        )(self) }
  pub fn is_email_address(&self) -> bool { tuple_enum_is!(TGTextEntityType, EmailAddress)(self) }
  pub fn is_hashtag(&self) -> bool { tuple_enum_is!(TGTextEntityType, Hashtag     )(self) }
  pub fn is_italic(&self) -> bool { tuple_enum_is!(TGTextEntityType, Italic      )(self) }
  pub fn is_mention(&self) -> bool { tuple_enum_is!(TGTextEntityType, Mention     )(self) }
  pub fn is_mention_name(&self) -> bool { tuple_enum_is!(TGTextEntityType, MentionName )(self) }
  pub fn is_phone_number(&self) -> bool { tuple_enum_is!(TGTextEntityType, PhoneNumber )(self) }
  pub fn is_pre(&self) -> bool { tuple_enum_is!(TGTextEntityType, Pre         )(self) }
  pub fn is_pre_code(&self) -> bool { tuple_enum_is!(TGTextEntityType, PreCode     )(self) }
  pub fn is_text_url(&self) -> bool { tuple_enum_is!(TGTextEntityType, TextUrl     )(self) }
  pub fn is_url(&self) -> bool { tuple_enum_is!(TGTextEntityType, Url         )(self) }

  pub fn on_bold<F: FnOnce(&TGTextEntityTypeBold)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGTextEntityType, Bold        , |t| fnc(t))(self);
    self
  }
  pub fn on_bot_command<F: FnOnce(&TGTextEntityTypeBotCommand)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGTextEntityType, BotCommand  , |t| fnc(t))(self);
    self
  }
  pub fn on_cashtag<F: FnOnce(&TGTextEntityTypeCashtag)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGTextEntityType, Cashtag     , |t| fnc(t))(self);
    self
  }
  pub fn on_code<F: FnOnce(&TGTextEntityTypeCode)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGTextEntityType, Code        , |t| fnc(t))(self);
    self
  }
  pub fn on_email_address<F: FnOnce(&TGTextEntityTypeEmailAddress)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGTextEntityType, EmailAddress, |t| fnc(t))(self);
    self
  }
  pub fn on_hashtag<F: FnOnce(&TGTextEntityTypeHashtag)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGTextEntityType, Hashtag     , |t| fnc(t))(self);
    self
  }
  pub fn on_italic<F: FnOnce(&TGTextEntityTypeItalic)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGTextEntityType, Italic      , |t| fnc(t))(self);
    self
  }
  pub fn on_mention<F: FnOnce(&TGTextEntityTypeMention)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGTextEntityType, Mention     , |t| fnc(t))(self);
    self
  }
  pub fn on_mention_name<F: FnOnce(&TGTextEntityTypeMentionName)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGTextEntityType, MentionName , |t| fnc(t))(self);
    self
  }
  pub fn on_phone_number<F: FnOnce(&TGTextEntityTypePhoneNumber)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGTextEntityType, PhoneNumber , |t| fnc(t))(self);
    self
  }
  pub fn on_pre<F: FnOnce(&TGTextEntityTypePre)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGTextEntityType, Pre         , |t| fnc(t))(self);
    self
  }
  pub fn on_pre_code<F: FnOnce(&TGTextEntityTypePreCode)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGTextEntityType, PreCode     , |t| fnc(t))(self);
    self
  }
  pub fn on_text_url<F: FnOnce(&TGTextEntityTypeTextUrl)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGTextEntityType, TextUrl     , |t| fnc(t))(self);
    self
  }
  pub fn on_url<F: FnOnce(&TGTextEntityTypeUrl)>(&self, fnc: F) -> &Self {
    tuple_enum_on!(TGTextEntityType, Url         , |t| fnc(t))(self);
    self
  }
}

impl TGTextEntityTypeBold {}

impl TGTextEntityTypeBotCommand {}

impl TGTextEntityTypeCashtag {}

impl TGTextEntityTypeCode {}

impl TGTextEntityTypeEmailAddress {}

impl TGTextEntityTypeHashtag {}

impl TGTextEntityTypeItalic {}

impl TGTextEntityTypeMention {}

impl TGTextEntityTypeMentionName {
  pub fn user_id(&self) -> i32 { self.origin().user_id().expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGTextEntityTypePhoneNumber {}

impl TGTextEntityTypePre {}

impl TGTextEntityTypePreCode {
  pub fn language(&self) -> Option<String> { self.origin().language() }
}

impl TGTextEntityTypeTextUrl {
  pub fn url(&self) -> String { self.origin().url().expect(errors::TELEGRAM_DATA_FAIL) }
}

impl TGTextEntityTypeUrl {}
