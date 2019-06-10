
use crate::api::*;


impl TGSetTdlibParameters {
  pub fn parameters<S: AsRef<TGTdlibParameters>>(&mut self, parameters: S) -> &mut Self {
    self._parameters(parameters.as_ref().build())
  }
}

impl TGTdlibParameters {

}
