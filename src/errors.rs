use error_chain_mini::ChainedError;
use error_chain_mini::ErrorKind;

pub const TELEGRAM_DATA_FAIL: &'static str = "TELEGRAM DATA FAIL. IF YOU SEE THIS MESSAGE, PLEASE POST AN ISSUE TO https://github.com/fewensa/telegram-client/issues";

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
