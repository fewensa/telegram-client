use rstring_builder::StringBuilder;
use text_reader::TextReader;

pub fn fill_json_struct<S: AsRef<str>>(json: S) -> String {
  let json = json.as_ref();

  let mut builder = StringBuilder::new();
  let mut reader = TextReader::new(json);

  while reader.has_next() {
    match reader.next() {
      Some('"') => {
        builder.append('"');
        let mut detector = reader.detector();
        if detector.next_text("@type\"").no() {
          continue;
        }
        builder.append("@type\"");
        let mut fnbuilder = StringBuilder::new();
        let mut times = 0;

        while reader.has_next() {
          match reader.next() {
            Some(':') => {
              builder.append(':');
              continue;
            }
            Some('"') => {
              match times {
                0 => {
                  builder.append("\"");
                  times += 1;
                  continue;
                }
                1 => {
                  builder.append(fnbuilder.string())
                    .append("\",")
                    .append(r#""@struct":""#)
                    .append(toolkit::text::uppercase_first_char(fnbuilder.string()))
                    .append('"');
                  break;
                }
                _ => {}
              }
              continue;
            }
            Some(ch) => {
              fnbuilder.append(ch);
              continue;
            }
            None => {}
          }
        }
      }
      Some(ch) => {
        builder.append(ch);
        continue;
      }
      None => {}
    }
  }

  builder.string()
}



#[cfg(test)]
mod rdtest {
  use rtdlib::types::{UpdateAuthorizationState, RObject};

  #[test]
  fn test_fill_json_struct() {
    let json = r#"{"@type":"updateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters"}}"#;
    let json = super::fill_json_struct(json);
    assert_eq!(json, r#"{"@type":"updateAuthorizationState","@struct":"UpdateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters","@struct":"AuthorizationStateWaitTdlibParameters"}}"#);
    let state: UpdateAuthorizationState = serde_json::from_str(&json[..]).expect("Json fail");
    assert_eq!("updateAuthorizationState", state.td_name());
  }

}


