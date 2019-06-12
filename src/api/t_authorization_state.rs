
use rtdlib::types::*;
use crate::types::*;
use crate::api::TDFB;


#[doc(hidden)] pub struct _TGTdlibParametersBuilder { inner: TGTdlibParameters }

impl _TGTdlibParametersBuilder {

  pub fn build(&self) -> TGTdlibParameters { self.inner.clone() }

  ///  If set to true, the Telegram test environment will be used instead of the production environment. 
  pub fn use_test_dc(&mut self, use_test_dc: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_use_test_dc(use_test_dc);
    self
  }
  ///  The path to the directory for the persistent database; if empty, the current working directory will be used. 
  pub fn database_directory<S: AsRef<str>>(&mut self, database_directory: S) -> &mut Self {
    self.inner.td_origin_mut()._set_database_directory(database_directory.as_ref().to_string());
    self
  }
  ///  The path to the directory for storing files; if empty, database_directory will be used. 
  pub fn files_directory<S: AsRef<str>>(&mut self, files_directory: S) -> &mut Self {
    self.inner.td_origin_mut()._set_files_directory(files_directory.as_ref().to_string());
    self
  }
  ///  If set to true, information about downloaded and uploaded files will be saved between application restarts. 
  pub fn use_file_database(&mut self, use_file_database: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_use_file_database(use_file_database);
    self
  }
  ///  If set to true, the library will maintain a cache of users, basic groups, supergroups, channels and secret chats. Implies use_file_database. 
  pub fn use_chat_info_database(&mut self, use_chat_info_database: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_use_chat_info_database(use_chat_info_database);
    self
  }
  ///  If set to true, the library will maintain a cache of chats and messages. Implies use_chat_info_database. 
  pub fn use_message_database(&mut self, use_message_database: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_use_message_database(use_message_database);
    self
  }
  ///  If set to true, support for secret chats will be enabled. 
  pub fn use_secret_chats(&mut self, use_secret_chats: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_use_secret_chats(use_secret_chats);
    self
  }
  ///  Application identifier for Telegram API access, which can be obtained at https://my.telegram.org. 
  pub fn api_id(&mut self, api_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_api_id(api_id);
    self
  }
  ///  Application identifier hash for Telegram API access, which can be obtained at https://my.telegram.org. 
  pub fn api_hash<S: AsRef<str>>(&mut self, api_hash: S) -> &mut Self {
    self.inner.td_origin_mut()._set_api_hash(api_hash.as_ref().to_string());
    self
  }
  ///  IETF language tag of the user's operating system language; must be non-empty. 
  pub fn system_language_code<S: AsRef<str>>(&mut self, system_language_code: S) -> &mut Self {
    self.inner.td_origin_mut()._set_system_language_code(system_language_code.as_ref().to_string());
    self
  }
  ///  Model of the device the application is being run on; must be non-empty. 
  pub fn device_model<S: AsRef<str>>(&mut self, device_model: S) -> &mut Self {
    self.inner.td_origin_mut()._set_device_model(device_model.as_ref().to_string());
    self
  }
  ///  Version of the operating system the application is being run on; must be non-empty. 
  pub fn system_version<S: AsRef<str>>(&mut self, system_version: S) -> &mut Self {
    self.inner.td_origin_mut()._set_system_version(system_version.as_ref().to_string());
    self
  }
  ///  Application version; must be non-empty. 
  pub fn application_version<S: AsRef<str>>(&mut self, application_version: S) -> &mut Self {
    self.inner.td_origin_mut()._set_application_version(application_version.as_ref().to_string());
    self
  }
  ///  If set to true, old files will automatically be deleted. 
  pub fn enable_storage_optimizer(&mut self, enable_storage_optimizer: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_enable_storage_optimizer(enable_storage_optimizer);
    self
  }
  ///  If set to true, original file names will be ignored. Otherwise, downloaded files will be saved under names as close as possible to the original name. 
  pub fn ignore_file_names(&mut self, ignore_file_names: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_ignore_file_names(ignore_file_names);
    self
  }
  

  
}

///  Contains parameters for TDLib initialization.  
#[derive(Debug, Clone)]
pub struct TGTdlibParameters {
  inner: TdlibParameters
}

impl TDFB for TGTdlibParameters {}

impl AsRef<TGTdlibParameters> for TGTdlibParameters {
  fn as_ref(&self) -> &TGTdlibParameters { self }
}

impl AsRef<TGTdlibParameters> for _TGTdlibParametersBuilder {
  fn as_ref(&self) -> &TGTdlibParameters { &self.inner }
}

impl TGTdlibParameters {
  pub fn builder() -> _TGTdlibParametersBuilder {
    _TGTdlibParametersBuilder { inner: Self::new(TdlibParameters::_new()) }
  }

  pub fn new(inner: TdlibParameters) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &TdlibParameters { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut TdlibParameters { &mut self.inner }
}

