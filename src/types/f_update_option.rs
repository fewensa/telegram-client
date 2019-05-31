use crate::types::TGUpdateOption;

impl TGUpdateOption {}


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


