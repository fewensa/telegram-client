
use crate::api::*;


impl _TGSetTdlibParametersBuilder {
  pub fn parameters<S: AsRef<TGTdlibParameters>>(&mut self, parameters: S) -> &mut Self {
    self._parameters(parameters.as_ref().td_origin().clone())
  }
}

impl TGTdlibParameters {

}
