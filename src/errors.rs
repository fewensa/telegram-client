use error_chain_mini::ChainedError;
use error_chain_mini::ErrorKind;
use rtdlib::types::RObject;

const PLZ_POST_ISSUES: &'static str = "PLEASE POST AN ISSUE TO https://github.com/fewensa/telegram-client/issues";

const TELEGRAM_DATA_FAIL: &'static str = "TELEGRAM DATA FAIL. IF YOU SEE THIS MESSAGE,";

#[derive(ErrorKind)]
pub enum TGErrorKind {
  #[msg(short = "Json error", detailed = "inner: {:?}", _0)]
  JsonError(::serde_json::Error),
  RTDLibFromError,
  Other,
}

pub type TGError = ChainedError<TGErrorKind>;
pub type TGResult<T> = Result<T, TGError>;

//// todox: only development
//impl ErrorKind for TGErrorKind {
//  fn short(&self) -> &str {
//    match self {
//      _ => ""
//    }
//  }
//}

pub fn please_post_issues() -> &'static str {
  PLZ_POST_ISSUES
}

pub fn not_have_listener<S: AsRef<str>>(td_name: S) -> String {
  format!("NOT HAVE [{}] LISTENER, {} , OR YOU CAN USE `on_receive` TO HANDLE THIS EVENT.", td_name.as_ref(), PLZ_POST_ISSUES)
}

pub fn data_fail_with_json<S: AsRef<str>>(json: S) -> String {
  format!("{} {} \n INCLUDE THIS JSON => {} \n AND PANIC TRACE", TELEGRAM_DATA_FAIL, PLZ_POST_ISSUES, json.as_ref())
}

pub fn data_fail_with_rtd<ROBJ: RObject>(robj: ROBJ) -> String {
  data_fail_with_json(robj.to_json())
}
