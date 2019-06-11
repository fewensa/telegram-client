
use rtdlib::types::*;
use crate::api::TDFB;


///  Contains parameters for TDLib initialization.  
#[derive(Debug, Clone)]
pub struct TGTdlibParameters {
  ///  If set to true, the Telegram test environment will be used instead of the production environment. 
  use_test_dc: Option<bool>,
  ///  The path to the directory for the persistent database; if empty, the current working directory will be used. 
  database_directory: Option<String>,
  ///  The path to the directory for storing files; if empty, database_directory will be used. 
  files_directory: Option<String>,
  ///  If set to true, information about downloaded and uploaded files will be saved between application restarts. 
  use_file_database: Option<bool>,
  ///  If set to true, the library will maintain a cache of users, basic groups, supergroups, channels and secret chats. Implies use_file_database. 
  use_chat_info_database: Option<bool>,
  ///  If set to true, the library will maintain a cache of chats and messages. Implies use_chat_info_database. 
  use_message_database: Option<bool>,
  ///  If set to true, support for secret chats will be enabled. 
  use_secret_chats: Option<bool>,
  ///  Application identifier for Telegram API access, which can be obtained at https://my.telegram.org. 
  api_id: Option<i32>,
  ///  Application identifier hash for Telegram API access, which can be obtained at https://my.telegram.org. 
  api_hash: Option<String>,
  ///  IETF language tag of the user's operating system language; must be non-empty. 
  system_language_code: Option<String>,
  ///  Model of the device the application is being run on; must be non-empty. 
  device_model: Option<String>,
  ///  Version of the operating system the application is being run on; must be non-empty. 
  system_version: Option<String>,
  ///  Application version; must be non-empty. 
  application_version: Option<String>,
  ///  If set to true, old files will automatically be deleted. 
  enable_storage_optimizer: Option<bool>,
  ///  If set to true, original file names will be ignored. Otherwise, downloaded files will be saved under names as close as possible to the original name. 
  ignore_file_names: Option<bool>,
  
}

impl TDFB for TGTdlibParameters {}

impl AsRef<TGTdlibParameters> for TGTdlibParameters {
  fn as_ref(&self) -> &TGTdlibParameters { self }
}

impl TGTdlibParameters {
  pub fn new() -> Self {
    Self {
      use_test_dc: None,
      database_directory: None,
      files_directory: None,
      use_file_database: None,
      use_chat_info_database: None,
      use_message_database: None,
      use_secret_chats: None,
      api_id: None,
      api_hash: None,
      system_language_code: None,
      device_model: None,
      system_version: None,
      application_version: None,
      enable_storage_optimizer: None,
      ignore_file_names: None,
      
    }
  }

  #[doc(hidden)]
  pub fn build(&self) -> TdlibParameters {
    TdlibParameters::builder()
      .use_test_dc(self.use_test_dc.clone())
      .database_directory(self.database_directory.clone())
      .files_directory(self.files_directory.clone())
      .use_file_database(self.use_file_database.clone())
      .use_chat_info_database(self.use_chat_info_database.clone())
      .use_message_database(self.use_message_database.clone())
      .use_secret_chats(self.use_secret_chats.clone())
      .api_id(self.api_id.clone())
      .api_hash(self.api_hash.clone())
      .system_language_code(self.system_language_code.clone())
      .device_model(self.device_model.clone())
      .system_version(self.system_version.clone())
      .application_version(self.application_version.clone())
      .enable_storage_optimizer(self.enable_storage_optimizer.clone())
      .ignore_file_names(self.ignore_file_names.clone())
      
      .build()
  }

  
  pub fn use_test_dc(&mut self, use_test_dc: bool) -> &mut Self {
    self.use_test_dc = Some(use_test_dc);
    self
  }
  
  pub fn database_directory<S: AsRef<str>>(&mut self, database_directory: S) -> &mut Self {
    self.database_directory = Some(database_directory.as_ref().to_string());
    self
  }
  
  pub fn files_directory<S: AsRef<str>>(&mut self, files_directory: S) -> &mut Self {
    self.files_directory = Some(files_directory.as_ref().to_string());
    self
  }
  
  pub fn use_file_database(&mut self, use_file_database: bool) -> &mut Self {
    self.use_file_database = Some(use_file_database);
    self
  }
  
  pub fn use_chat_info_database(&mut self, use_chat_info_database: bool) -> &mut Self {
    self.use_chat_info_database = Some(use_chat_info_database);
    self
  }
  
  pub fn use_message_database(&mut self, use_message_database: bool) -> &mut Self {
    self.use_message_database = Some(use_message_database);
    self
  }
  
  pub fn use_secret_chats(&mut self, use_secret_chats: bool) -> &mut Self {
    self.use_secret_chats = Some(use_secret_chats);
    self
  }
  
  pub fn api_id(&mut self, api_id: i32) -> &mut Self {
    self.api_id = Some(api_id);
    self
  }
  
  pub fn api_hash<S: AsRef<str>>(&mut self, api_hash: S) -> &mut Self {
    self.api_hash = Some(api_hash.as_ref().to_string());
    self
  }
  
  pub fn system_language_code<S: AsRef<str>>(&mut self, system_language_code: S) -> &mut Self {
    self.system_language_code = Some(system_language_code.as_ref().to_string());
    self
  }
  
  pub fn device_model<S: AsRef<str>>(&mut self, device_model: S) -> &mut Self {
    self.device_model = Some(device_model.as_ref().to_string());
    self
  }
  
  pub fn system_version<S: AsRef<str>>(&mut self, system_version: S) -> &mut Self {
    self.system_version = Some(system_version.as_ref().to_string());
    self
  }
  
  pub fn application_version<S: AsRef<str>>(&mut self, application_version: S) -> &mut Self {
    self.application_version = Some(application_version.as_ref().to_string());
    self
  }
  
  pub fn enable_storage_optimizer(&mut self, enable_storage_optimizer: bool) -> &mut Self {
    self.enable_storage_optimizer = Some(enable_storage_optimizer);
    self
  }
  
  pub fn ignore_file_names(&mut self, ignore_file_names: bool) -> &mut Self {
    self.ignore_file_names = Some(ignore_file_names);
    self
  }
  

}

