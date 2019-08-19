use crate::types::TGUpdateOption;
use rtdlib::types as td_type;
use crate::errors;

impl TGUpdateOption {

  pub fn name(&self) -> String {
    self.td_origin().name().expect(&errors::data_fail_with_rtd(self.td_origin())[..])
  }

  pub fn value(&self) -> TGOptionValue {
    TGOptionValue::new(self.td_origin().value())
  }

  pub fn on_name<S: AsRef<str>, F: FnOnce(&TGOptionValue)>(&self, name: S, fnc: F) {
    let value = TGOptionValue::new(self.td_origin().value());
    if &self.name()[..] == name.as_ref() && value.is_some() {
      fnc(&value)
    }
  }

}

pub struct TGOptionValue {
  value: Option<Box<td_type::OptionValue>>
}

macro_rules! option_value_as {
  ($value_class:ident, $retype:tt) => (
    fn ovas(value: &Option<Box<td_type::OptionValue>>) -> Option<$retype> {
      value.clone().filter(|v| v.td_type() == td_type::RTDType::$value_class)
        .map(|v| td_type::$value_class::from_json(v.to_json()))
        .filter(|v| v.is_some())
        .map(|v| v.map(|v| v.value().clone().map(|v| v)))
        .map_or(None, |v| v)
        .map_or(None, |v| v)
    }
  )
}

impl TGOptionValue {

  fn new(value: Option<Box<td_type::OptionValue>>) -> Self {
    Self { value }
  }

  fn is_some(&self) -> bool {
    self.value.is_some()
  }

  pub fn is_bool(&self) -> bool {
    self.value.clone().map(|v| v.td_type() == td_type::RTDType::OptionValueBoolean)
      .map_or(false, |v| v)
  }

  pub fn is_empty(&self) -> bool {
    self.value.clone().map(|v| v.td_type() == td_type::RTDType::OptionValueEmpty)
      .map_or(false, |v| v)
  }

  pub fn is_string(&self) -> bool {
    self.value.clone().map(|v| v.td_type() == td_type::RTDType::OptionValueString)
      .map_or(false, |v| v)
  }

  pub fn is_integer(&self) -> bool {
    self.value.clone().map(|v| v.td_type() == td_type::RTDType::OptionValueInteger)
      .map_or(false, |v| v)
  }

  pub fn as_string(&self) -> Option<String> {
//    self.value.clone().filter(|v| v.td_type() == RTDType::OptionValueString)
//      .map(|v| OptionValueString::from_json(v.to_json()))
//      .filter(|v| v.is_some())
//      .map(|v|
//        v.map(|v| v.value().clone().map(|v| v))
//      )
//      .map_or(None, |v| v)
//      .map_or(None, |v| v)
////    option_value_as!(OptionValueString, String);
////    option_value_as(self.value)

    option_value_as!(OptionValueString, String);
    ovas(&self.value)
  }

  pub fn as_integer(&self) -> Option<i32> {
    option_value_as!(OptionValueInteger, i32);
    ovas(&self.value)
  }

  pub fn as_bool(&self) -> Option<bool> {
    option_value_as!(OptionValueBoolean, bool);
    ovas(&self.value)
  }

}


mod tgupdatetest {
  use rtdlib::types::*;

  use crate::types::TGUpdateOption;

  #[test]
  fn test_serialize() {
    let json = r#"{"@type":"updateOption","name":"forwarded_message_count_max","value":{"@type":"optionValueInteger","value":100}}"#;
    let tgo = TGUpdateOption::from_json(json);
    assert!(tgo.is_some());
    let tgo = tgo.unwrap();
    println!("{}", tgo.to_json());
  }
}


