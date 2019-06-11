use rtdlib::types::*;

pub trait TDFB {}

impl <'a, T: TDFB> TDFB for &'a T {}
impl <'a, T: TDFB> TDFB for &'a mut T {}


///  Accepts an incoming call. 
#[derive(Debug, Clone)]
pub struct TGAcceptCall {
  ///  Call identifier. 
  call_id: Option<i32>,
  ///  Description of the call protocols supported by the client. 
  protocol: Option<CallProtocol>,
  
}

impl TDFB for TGAcceptCall {}

impl AsRef<TGAcceptCall> for TGAcceptCall {
  fn as_ref(&self) -> &TGAcceptCall { self }
}

impl TGAcceptCall {

  pub fn new() -> Self {
    Self {
      call_id: None,
      protocol: None,
      
    }
  }

  
  pub fn call_id(&mut self, call_id: i32) -> &mut Self { self.call_id = Some(call_id); self }
  


  
  // [protocol] type is [CallProtocol], is not support, need add manully.
  #[doc(hidden)] pub fn _protocol(&mut self, protocol: CallProtocol) -> &mut Self { self.protocol = Some(protocol); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> AcceptCall {
    AcceptCall::builder()
      .call_id(self.call_id.clone())
      .protocol(self.protocol.clone())
      
      .build()
  }
}


///  Accepts Telegram terms of services. 
#[derive(Debug, Clone)]
pub struct TGAcceptTermsOfService {
  ///  Terms of service identifier. 
  terms_of_service_id: Option<String>,
  
}

impl TDFB for TGAcceptTermsOfService {}

impl AsRef<TGAcceptTermsOfService> for TGAcceptTermsOfService {
  fn as_ref(&self) -> &TGAcceptTermsOfService { self }
}

impl TGAcceptTermsOfService {

  pub fn new() -> Self {
    Self {
      terms_of_service_id: None,
      
    }
  }

  
  pub fn terms_of_service_id<S: AsRef<str>>(&mut self, terms_of_service_id: S) -> &mut Self { self.terms_of_service_id = Some(terms_of_service_id.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> AcceptTermsOfService {
    AcceptTermsOfService::builder()
      .terms_of_service_id(self.terms_of_service_id.clone())
      
      .build()
  }
}


///  Adds a new member to a chat. Members can't be added to private or secret chats. Members will not be added until the chat state has been synchronized with the server. 
#[derive(Debug, Clone)]
pub struct TGAddChatMember {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  ///  Identifier of the user. 
  user_id: Option<i32>,
  ///  The number of earlier messages from the chat to be forwarded to the new member; up to 100. Ignored for supergroups and channels. 
  forward_limit: Option<i32>,
  
}

impl TDFB for TGAddChatMember {}

impl AsRef<TGAddChatMember> for TGAddChatMember {
  fn as_ref(&self) -> &TGAddChatMember { self }
}

impl TGAddChatMember {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      user_id: None,
      forward_limit: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn forward_limit(&mut self, forward_limit: i32) -> &mut Self { self.forward_limit = Some(forward_limit); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> AddChatMember {
    AddChatMember::builder()
      .chat_id(self.chat_id.clone())
      .user_id(self.user_id.clone())
      .forward_limit(self.forward_limit.clone())
      
      .build()
  }
}


///  Adds multiple new members to a chat. Currently this option is only available for supergroups and channels. This option can't be used to join a chat. Members can't be added to a channel if it has more than 200 members. Members will not be added until the chat state has been synchronized with the server. 
#[derive(Debug, Clone)]
pub struct TGAddChatMembers {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  ///  Identifiers of the users to be added to the chat. 
  user_ids: Option<Vec<i32>>,
  
}

impl TDFB for TGAddChatMembers {}

impl AsRef<TGAddChatMembers> for TGAddChatMembers {
  fn as_ref(&self) -> &TGAddChatMembers { self }
}

impl TGAddChatMembers {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      user_ids: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn user_ids(&mut self, user_ids: Vec<i32>) -> &mut Self { self.user_ids = Some(user_ids); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> AddChatMembers {
    AddChatMembers::builder()
      .chat_id(self.chat_id.clone())
      .user_ids(self.user_ids.clone())
      
      .build()
  }
}


///  Adds a custom server language pack to the list of installed language packs in current localization target. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGAddCustomServerLanguagePack {
  ///  Identifier of a language pack to be added; may be different from a name that is used in an "https://t.me/setlanguage/" link. 
  language_pack_id: Option<String>,
  
}

impl TDFB for TGAddCustomServerLanguagePack {}

impl AsRef<TGAddCustomServerLanguagePack> for TGAddCustomServerLanguagePack {
  fn as_ref(&self) -> &TGAddCustomServerLanguagePack { self }
}

impl TGAddCustomServerLanguagePack {

  pub fn new() -> Self {
    Self {
      language_pack_id: None,
      
    }
  }

  
  pub fn language_pack_id<S: AsRef<str>>(&mut self, language_pack_id: S) -> &mut Self { self.language_pack_id = Some(language_pack_id.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> AddCustomServerLanguagePack {
    AddCustomServerLanguagePack::builder()
      .language_pack_id(self.language_pack_id.clone())
      
      .build()
  }
}


///  Adds a new sticker to the list of favorite stickers. The new sticker is added to the top of the list. If the sticker was already in the list, it is removed from the list first. Only stickers belonging to a sticker set can be added to this list. 
#[derive(Debug, Clone)]
pub struct TGAddFavoriteSticker {
  ///  Sticker file to add. 
  sticker: Option<Box<InputFile>>,
  
}

impl TDFB for TGAddFavoriteSticker {}

impl AsRef<TGAddFavoriteSticker> for TGAddFavoriteSticker {
  fn as_ref(&self) -> &TGAddFavoriteSticker { self }
}

impl TGAddFavoriteSticker {

  pub fn new() -> Self {
    Self {
      sticker: None,
      
    }
  }

  


  
  // [sticker] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _sticker(&mut self, sticker: Box<InputFile>) -> &mut Self { self.sticker = Some(sticker); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> AddFavoriteSticker {
    AddFavoriteSticker::builder()
      .sticker(self.sticker.clone())
      
      .build()
  }
}


///  Adds a local message to a chat. The message is persistent across application restarts only if the message database is used. Returns the added message. 
#[derive(Debug, Clone)]
pub struct TGAddLocalMessage {
  ///  Target chat. 
  chat_id: Option<i64>,
  ///  Identifier of the user who will be shown as the sender of the message; may be 0 for channel posts. 
  sender_user_id: Option<i32>,
  ///  Identifier of the message to reply to or 0. 
  reply_to_message_id: Option<i64>,
  ///  Pass true to disable notification for the message. 
  disable_notification: Option<bool>,
  ///  The content of the message to be added. 
  input_message_content: Option<Box<InputMessageContent>>,
  
}

impl TDFB for TGAddLocalMessage {}

impl AsRef<TGAddLocalMessage> for TGAddLocalMessage {
  fn as_ref(&self) -> &TGAddLocalMessage { self }
}

impl TGAddLocalMessage {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      sender_user_id: None,
      reply_to_message_id: None,
      disable_notification: None,
      input_message_content: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn sender_user_id(&mut self, sender_user_id: i32) -> &mut Self { self.sender_user_id = Some(sender_user_id); self }
  
  pub fn reply_to_message_id(&mut self, reply_to_message_id: i64) -> &mut Self { self.reply_to_message_id = Some(reply_to_message_id); self }
  
  pub fn disable_notification(&mut self, disable_notification: bool) -> &mut Self { self.disable_notification = Some(disable_notification); self }
  


  
  // [input_message_content] type is [Box<InputMessageContent>], is not support, need add manully.
  #[doc(hidden)] pub fn _input_message_content(&mut self, input_message_content: Box<InputMessageContent>) -> &mut Self { self.input_message_content = Some(input_message_content); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> AddLocalMessage {
    AddLocalMessage::builder()
      .chat_id(self.chat_id.clone())
      .sender_user_id(self.sender_user_id.clone())
      .reply_to_message_id(self.reply_to_message_id.clone())
      .disable_notification(self.disable_notification.clone())
      .input_message_content(self.input_message_content.clone())
      
      .build()
  }
}


///  Adds a message to TDLib internal log. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGAddLogMessage {
  ///  Minimum verbosity level needed for the message to be logged, 0-1023. 
  verbosity_level: Option<i32>,
  ///  Text of a message to log. 
  text: Option<String>,
  
}

impl TDFB for TGAddLogMessage {}

impl AsRef<TGAddLogMessage> for TGAddLogMessage {
  fn as_ref(&self) -> &TGAddLogMessage { self }
}

impl TGAddLogMessage {

  pub fn new() -> Self {
    Self {
      verbosity_level: None,
      text: None,
      
    }
  }

  
  pub fn verbosity_level(&mut self, verbosity_level: i32) -> &mut Self { self.verbosity_level = Some(verbosity_level); self }
  
  pub fn text<S: AsRef<str>>(&mut self, text: S) -> &mut Self { self.text = Some(text.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> AddLogMessage {
    AddLogMessage::builder()
      .verbosity_level(self.verbosity_level.clone())
      .text(self.text.clone())
      
      .build()
  }
}


///  Adds the specified data to data usage statistics. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGAddNetworkStatistics {
  ///  The network statistics entry with the data to be added to statistics. 
  entry: Option<Box<NetworkStatisticsEntry>>,
  
}

impl TDFB for TGAddNetworkStatistics {}

impl AsRef<TGAddNetworkStatistics> for TGAddNetworkStatistics {
  fn as_ref(&self) -> &TGAddNetworkStatistics { self }
}

impl TGAddNetworkStatistics {

  pub fn new() -> Self {
    Self {
      entry: None,
      
    }
  }

  


  
  // [entry] type is [Box<NetworkStatisticsEntry>], is not support, need add manully.
  #[doc(hidden)] pub fn _entry(&mut self, entry: Box<NetworkStatisticsEntry>) -> &mut Self { self.entry = Some(entry); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> AddNetworkStatistics {
    AddNetworkStatistics::builder()
      .entry(self.entry.clone())
      
      .build()
  }
}


///  Adds a proxy server for network requests. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGAddProxy {
  ///  Proxy server IP address. 
  server: Option<String>,
  ///  Proxy server port. 
  port: Option<i32>,
  ///  True, if the proxy should be enabled. 
  enable: Option<bool>,
  ///  Proxy type. 
  type_: Option<Box<ProxyType>>,
  
}

impl TDFB for TGAddProxy {}

impl AsRef<TGAddProxy> for TGAddProxy {
  fn as_ref(&self) -> &TGAddProxy { self }
}

impl TGAddProxy {

  pub fn new() -> Self {
    Self {
      server: None,
      port: None,
      enable: None,
      type_: None,
      
    }
  }

  
  pub fn server<S: AsRef<str>>(&mut self, server: S) -> &mut Self { self.server = Some(server.as_ref().to_string()); self }
  
  pub fn port(&mut self, port: i32) -> &mut Self { self.port = Some(port); self }
  
  pub fn enable(&mut self, enable: bool) -> &mut Self { self.enable = Some(enable); self }
  


  
  // [type_] type is [Box<ProxyType>], is not support, need add manully.
  #[doc(hidden)] pub fn _type_(&mut self, type_: Box<ProxyType>) -> &mut Self { self.type_ = Some(type_); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> AddProxy {
    AddProxy::builder()
      .server(self.server.clone())
      .port(self.port.clone())
      .enable(self.enable.clone())
      .type_(self.type_.clone())
      
      .build()
  }
}


///  Adds a chat to the list of recently found chats. The chat is added to the beginning of the list. If the chat is already in the list, it will be removed from the list first. 
#[derive(Debug, Clone)]
pub struct TGAddRecentlyFoundChat {
  ///  Identifier of the chat to add. 
  chat_id: Option<i64>,
  
}

impl TDFB for TGAddRecentlyFoundChat {}

impl AsRef<TGAddRecentlyFoundChat> for TGAddRecentlyFoundChat {
  fn as_ref(&self) -> &TGAddRecentlyFoundChat { self }
}

impl TGAddRecentlyFoundChat {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> AddRecentlyFoundChat {
    AddRecentlyFoundChat::builder()
      .chat_id(self.chat_id.clone())
      
      .build()
  }
}


///  Manually adds a new sticker to the list of recently used stickers. The new sticker is added to the top of the list. If the sticker was already in the list, it is removed from the list first. Only stickers belonging to a sticker set can be added to this list. 
#[derive(Debug, Clone)]
pub struct TGAddRecentSticker {
  ///  Pass true to add the sticker to the list of stickers recently attached to photo or video files; pass false to add the sticker to the list of recently sent stickers. 
  is_attached: Option<bool>,
  ///  Sticker file to add. 
  sticker: Option<Box<InputFile>>,
  
}

impl TDFB for TGAddRecentSticker {}

impl AsRef<TGAddRecentSticker> for TGAddRecentSticker {
  fn as_ref(&self) -> &TGAddRecentSticker { self }
}

impl TGAddRecentSticker {

  pub fn new() -> Self {
    Self {
      is_attached: None,
      sticker: None,
      
    }
  }

  
  pub fn is_attached(&mut self, is_attached: bool) -> &mut Self { self.is_attached = Some(is_attached); self }
  


  
  // [sticker] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _sticker(&mut self, sticker: Box<InputFile>) -> &mut Self { self.sticker = Some(sticker); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> AddRecentSticker {
    AddRecentSticker::builder()
      .is_attached(self.is_attached.clone())
      .sticker(self.sticker.clone())
      
      .build()
  }
}


///  Manually adds a new animation to the list of saved animations. The new animation is added to the beginning of the list. If the animation was already in the list, it is removed first. Only non-secret video animations with MIME type "video/mp4" can be added to the list. 
#[derive(Debug, Clone)]
pub struct TGAddSavedAnimation {
  ///  The animation file to be added. Only animations known to the server (i.e. successfully sent via a message) can be added to the list. 
  animation: Option<Box<InputFile>>,
  
}

impl TDFB for TGAddSavedAnimation {}

impl AsRef<TGAddSavedAnimation> for TGAddSavedAnimation {
  fn as_ref(&self) -> &TGAddSavedAnimation { self }
}

impl TGAddSavedAnimation {

  pub fn new() -> Self {
    Self {
      animation: None,
      
    }
  }

  


  
  // [animation] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _animation(&mut self, animation: Box<InputFile>) -> &mut Self { self.animation = Some(animation); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> AddSavedAnimation {
    AddSavedAnimation::builder()
      .animation(self.animation.clone())
      
      .build()
  }
}


///  Adds a new sticker to a set; for bots only. Returns the sticker set. 
#[derive(Debug, Clone)]
pub struct TGAddStickerToSet {
  ///  Sticker set owner. 
  user_id: Option<i32>,
  ///  Sticker set name. 
  name: Option<String>,
  ///  Sticker to add to the set. 
  sticker: Option<InputSticker>,
  
}

impl TDFB for TGAddStickerToSet {}

impl AsRef<TGAddStickerToSet> for TGAddStickerToSet {
  fn as_ref(&self) -> &TGAddStickerToSet { self }
}

impl TGAddStickerToSet {

  pub fn new() -> Self {
    Self {
      user_id: None,
      name: None,
      sticker: None,
      
    }
  }

  
  pub fn user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn name<S: AsRef<str>>(&mut self, name: S) -> &mut Self { self.name = Some(name.as_ref().to_string()); self }
  


  
  // [sticker] type is [InputSticker], is not support, need add manully.
  #[doc(hidden)] pub fn _sticker(&mut self, sticker: InputSticker) -> &mut Self { self.sticker = Some(sticker); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> AddStickerToSet {
    AddStickerToSet::builder()
      .user_id(self.user_id.clone())
      .name(self.name.clone())
      .sticker(self.sticker.clone())
      
      .build()
  }
}


///  Sets the result of a callback query; for bots only. 
#[derive(Debug, Clone)]
pub struct TGAnswerCallbackQuery {
  ///  Identifier of the callback query. 
  callback_query_id: Option<i64>,
  ///  Text of the answer. 
  text: Option<String>,
  ///  If true, an alert should be shown to the user instead of a toast notification. 
  show_alert: Option<bool>,
  ///  URL to be opened. 
  url: Option<String>,
  ///  Time during which the result of the query can be cached, in seconds. 
  cache_time: Option<i32>,
  
}

impl TDFB for TGAnswerCallbackQuery {}

impl AsRef<TGAnswerCallbackQuery> for TGAnswerCallbackQuery {
  fn as_ref(&self) -> &TGAnswerCallbackQuery { self }
}

impl TGAnswerCallbackQuery {

  pub fn new() -> Self {
    Self {
      callback_query_id: None,
      text: None,
      show_alert: None,
      url: None,
      cache_time: None,
      
    }
  }

  
  pub fn callback_query_id(&mut self, callback_query_id: i64) -> &mut Self { self.callback_query_id = Some(callback_query_id); self }
  
  pub fn text<S: AsRef<str>>(&mut self, text: S) -> &mut Self { self.text = Some(text.as_ref().to_string()); self }
  
  pub fn show_alert(&mut self, show_alert: bool) -> &mut Self { self.show_alert = Some(show_alert); self }
  
  pub fn url<S: AsRef<str>>(&mut self, url: S) -> &mut Self { self.url = Some(url.as_ref().to_string()); self }
  
  pub fn cache_time(&mut self, cache_time: i32) -> &mut Self { self.cache_time = Some(cache_time); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> AnswerCallbackQuery {
    AnswerCallbackQuery::builder()
      .callback_query_id(self.callback_query_id.clone())
      .text(self.text.clone())
      .show_alert(self.show_alert.clone())
      .url(self.url.clone())
      .cache_time(self.cache_time.clone())
      
      .build()
  }
}


///  Answers a custom query; for bots only. 
#[derive(Debug, Clone)]
pub struct TGAnswerCustomQuery {
  ///  Identifier of a custom query. 
  custom_query_id: Option<i64>,
  ///  JSON-serialized answer to the query. 
  data: Option<String>,
  
}

impl TDFB for TGAnswerCustomQuery {}

impl AsRef<TGAnswerCustomQuery> for TGAnswerCustomQuery {
  fn as_ref(&self) -> &TGAnswerCustomQuery { self }
}

impl TGAnswerCustomQuery {

  pub fn new() -> Self {
    Self {
      custom_query_id: None,
      data: None,
      
    }
  }

  
  pub fn custom_query_id(&mut self, custom_query_id: i64) -> &mut Self { self.custom_query_id = Some(custom_query_id); self }
  
  pub fn data<S: AsRef<str>>(&mut self, data: S) -> &mut Self { self.data = Some(data.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> AnswerCustomQuery {
    AnswerCustomQuery::builder()
      .custom_query_id(self.custom_query_id.clone())
      .data(self.data.clone())
      
      .build()
  }
}


///  Sets the result of an inline query; for bots only. 
#[derive(Debug, Clone)]
pub struct TGAnswerInlineQuery {
  ///  Identifier of the inline query. 
  inline_query_id: Option<i64>,
  ///  True, if the result of the query can be cached for the specified user. 
  is_personal: Option<bool>,
  ///  The results of the query. 
  results: Option<Vec<Box<InputInlineQueryResult>>>,
  ///  Allowed time to cache the results of the query, in seconds. 
  cache_time: Option<i32>,
  ///  Offset for the next inline query; pass an empty string if there are no more results. 
  next_offset: Option<String>,
  ///  If non-empty, this text should be shown on the button that opens a private chat with the bot and sends a start message to the bot with the parameter switch_pm_parameter. 
  switch_pm_text: Option<String>,
  ///  The parameter for the bot start message. 
  switch_pm_parameter: Option<String>,
  
}

impl TDFB for TGAnswerInlineQuery {}

impl AsRef<TGAnswerInlineQuery> for TGAnswerInlineQuery {
  fn as_ref(&self) -> &TGAnswerInlineQuery { self }
}

impl TGAnswerInlineQuery {

  pub fn new() -> Self {
    Self {
      inline_query_id: None,
      is_personal: None,
      results: None,
      cache_time: None,
      next_offset: None,
      switch_pm_text: None,
      switch_pm_parameter: None,
      
    }
  }

  
  pub fn inline_query_id(&mut self, inline_query_id: i64) -> &mut Self { self.inline_query_id = Some(inline_query_id); self }
  
  pub fn is_personal(&mut self, is_personal: bool) -> &mut Self { self.is_personal = Some(is_personal); self }
  
  pub fn cache_time(&mut self, cache_time: i32) -> &mut Self { self.cache_time = Some(cache_time); self }
  
  pub fn next_offset<S: AsRef<str>>(&mut self, next_offset: S) -> &mut Self { self.next_offset = Some(next_offset.as_ref().to_string()); self }
  
  pub fn switch_pm_text<S: AsRef<str>>(&mut self, switch_pm_text: S) -> &mut Self { self.switch_pm_text = Some(switch_pm_text.as_ref().to_string()); self }
  
  pub fn switch_pm_parameter<S: AsRef<str>>(&mut self, switch_pm_parameter: S) -> &mut Self { self.switch_pm_parameter = Some(switch_pm_parameter.as_ref().to_string()); self }
  


  
  // [results] type is [Vec<Box<InputInlineQueryResult>>], is not support, need add manully.
  #[doc(hidden)] pub fn _results(&mut self, results: Vec<Box<InputInlineQueryResult>>) -> &mut Self { self.results = Some(results); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> AnswerInlineQuery {
    AnswerInlineQuery::builder()
      .inline_query_id(self.inline_query_id.clone())
      .is_personal(self.is_personal.clone())
      .results(self.results.clone())
      .cache_time(self.cache_time.clone())
      .next_offset(self.next_offset.clone())
      .switch_pm_text(self.switch_pm_text.clone())
      .switch_pm_parameter(self.switch_pm_parameter.clone())
      
      .build()
  }
}


///  Sets the result of a pre-checkout query; for bots only. 
#[derive(Debug, Clone)]
pub struct TGAnswerPreCheckoutQuery {
  ///  Identifier of the pre-checkout query. 
  pre_checkout_query_id: Option<i64>,
  ///  An error message, empty on success. 
  error_message: Option<String>,
  
}

impl TDFB for TGAnswerPreCheckoutQuery {}

impl AsRef<TGAnswerPreCheckoutQuery> for TGAnswerPreCheckoutQuery {
  fn as_ref(&self) -> &TGAnswerPreCheckoutQuery { self }
}

impl TGAnswerPreCheckoutQuery {

  pub fn new() -> Self {
    Self {
      pre_checkout_query_id: None,
      error_message: None,
      
    }
  }

  
  pub fn pre_checkout_query_id(&mut self, pre_checkout_query_id: i64) -> &mut Self { self.pre_checkout_query_id = Some(pre_checkout_query_id); self }
  
  pub fn error_message<S: AsRef<str>>(&mut self, error_message: S) -> &mut Self { self.error_message = Some(error_message.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> AnswerPreCheckoutQuery {
    AnswerPreCheckoutQuery::builder()
      .pre_checkout_query_id(self.pre_checkout_query_id.clone())
      .error_message(self.error_message.clone())
      
      .build()
  }
}


///  Sets the result of a shipping query; for bots only. 
#[derive(Debug, Clone)]
pub struct TGAnswerShippingQuery {
  ///  Identifier of the shipping query. 
  shipping_query_id: Option<i64>,
  ///  Available shipping options. 
  shipping_options: Option<Vec<ShippingOption>>,
  ///  An error message, empty on success. 
  error_message: Option<String>,
  
}

impl TDFB for TGAnswerShippingQuery {}

impl AsRef<TGAnswerShippingQuery> for TGAnswerShippingQuery {
  fn as_ref(&self) -> &TGAnswerShippingQuery { self }
}

impl TGAnswerShippingQuery {

  pub fn new() -> Self {
    Self {
      shipping_query_id: None,
      shipping_options: None,
      error_message: None,
      
    }
  }

  
  pub fn shipping_query_id(&mut self, shipping_query_id: i64) -> &mut Self { self.shipping_query_id = Some(shipping_query_id); self }
  
  pub fn error_message<S: AsRef<str>>(&mut self, error_message: S) -> &mut Self { self.error_message = Some(error_message.as_ref().to_string()); self }
  


  
  // [shipping_options] type is [Vec<ShippingOption>], is not support, need add manully.
  #[doc(hidden)] pub fn _shipping_options(&mut self, shipping_options: Vec<ShippingOption>) -> &mut Self { self.shipping_options = Some(shipping_options); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> AnswerShippingQuery {
    AnswerShippingQuery::builder()
      .shipping_query_id(self.shipping_query_id.clone())
      .shipping_options(self.shipping_options.clone())
      .error_message(self.error_message.clone())
      
      .build()
  }
}


///  Adds a user to the blacklist. 
#[derive(Debug, Clone)]
pub struct TGBlockUser {
  ///  User identifier. 
  user_id: Option<i32>,
  
}

impl TDFB for TGBlockUser {}

impl AsRef<TGBlockUser> for TGBlockUser {
  fn as_ref(&self) -> &TGBlockUser { self }
}

impl TGBlockUser {

  pub fn new() -> Self {
    Self {
      user_id: None,
      
    }
  }

  
  pub fn user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> BlockUser {
    BlockUser::builder()
      .user_id(self.user_id.clone())
      
      .build()
  }
}


///  Stops the downloading of a file. If a file has already been downloaded, does nothing. 
#[derive(Debug, Clone)]
pub struct TGCancelDownloadFile {
  ///  Identifier of a file to stop downloading. 
  file_id: Option<i32>,
  ///  Pass true to stop downloading only if it hasn't been started, i.e. request hasn't been sent to server. 
  only_if_pending: Option<bool>,
  
}

impl TDFB for TGCancelDownloadFile {}

impl AsRef<TGCancelDownloadFile> for TGCancelDownloadFile {
  fn as_ref(&self) -> &TGCancelDownloadFile { self }
}

impl TGCancelDownloadFile {

  pub fn new() -> Self {
    Self {
      file_id: None,
      only_if_pending: None,
      
    }
  }

  
  pub fn file_id(&mut self, file_id: i32) -> &mut Self { self.file_id = Some(file_id); self }
  
  pub fn only_if_pending(&mut self, only_if_pending: bool) -> &mut Self { self.only_if_pending = Some(only_if_pending); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> CancelDownloadFile {
    CancelDownloadFile::builder()
      .file_id(self.file_id.clone())
      .only_if_pending(self.only_if_pending.clone())
      
      .build()
  }
}


///  Stops the uploading of a file. Supported only for files uploaded by using  
#[derive(Debug, Clone)]
pub struct TGCancelUploadFile {
  ///  Identifier of the file to stop uploading. 
  file_id: Option<i32>,
  
}

impl TDFB for TGCancelUploadFile {}

impl AsRef<TGCancelUploadFile> for TGCancelUploadFile {
  fn as_ref(&self) -> &TGCancelUploadFile { self }
}

impl TGCancelUploadFile {

  pub fn new() -> Self {
    Self {
      file_id: None,
      
    }
  }

  
  pub fn file_id(&mut self, file_id: i32) -> &mut Self { self.file_id = Some(file_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> CancelUploadFile {
    CancelUploadFile::builder()
      .file_id(self.file_id.clone())
      
      .build()
  }
}


///  Reports to the server whether a chat is a spam chat or not. Can be used only if ChatReportSpamState.can_report_spam is true. After this request, ChatReportSpamState.can_report_spam becomes false forever. 
#[derive(Debug, Clone)]
pub struct TGChangeChatReportSpamState {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  ///  If true, the chat will be reported as spam; otherwise it will be marked as not spam. 
  is_spam_chat: Option<bool>,
  
}

impl TDFB for TGChangeChatReportSpamState {}

impl AsRef<TGChangeChatReportSpamState> for TGChangeChatReportSpamState {
  fn as_ref(&self) -> &TGChangeChatReportSpamState { self }
}

impl TGChangeChatReportSpamState {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      is_spam_chat: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn is_spam_chat(&mut self, is_spam_chat: bool) -> &mut Self { self.is_spam_chat = Some(is_spam_chat); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> ChangeChatReportSpamState {
    ChangeChatReportSpamState::builder()
      .chat_id(self.chat_id.clone())
      .is_spam_chat(self.is_spam_chat.clone())
      
      .build()
  }
}


///  Changes imported contacts using the list of current user contacts saved on the device. Imports newly added contacts and, if at least the file database is enabled, deletes recently deleted contacts. Query result depends on the result of the previous query, so only one query is possible at the same time. 
#[derive(Debug, Clone)]
pub struct TGChangeImportedContacts {
  ///  The new list of contacts, contact's vCard are ignored and are not imported. 
  contacts: Option<Vec<Contact>>,
  
}

impl TDFB for TGChangeImportedContacts {}

impl AsRef<TGChangeImportedContacts> for TGChangeImportedContacts {
  fn as_ref(&self) -> &TGChangeImportedContacts { self }
}

impl TGChangeImportedContacts {

  pub fn new() -> Self {
    Self {
      contacts: None,
      
    }
  }

  


  
  // [contacts] type is [Vec<Contact>], is not support, need add manully.
  #[doc(hidden)] pub fn _contacts(&mut self, contacts: Vec<Contact>) -> &mut Self { self.contacts = Some(contacts); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> ChangeImportedContacts {
    ChangeImportedContacts::builder()
      .contacts(self.contacts.clone())
      
      .build()
  }
}


///  Changes the phone number of the user and sends an authentication code to the user's new phone number. On success, returns information about the sent code. 
#[derive(Debug, Clone)]
pub struct TGChangePhoneNumber {
  ///  The new phone number of the user in international format. 
  phone_number: Option<String>,
  ///  Pass true if the code can be sent via flash call to the specified phone number. 
  allow_flash_call: Option<bool>,
  ///  Pass true if the phone number is used on the current device. Ignored if allow_flash_call is false. 
  is_current_phone_number: Option<bool>,
  
}

impl TDFB for TGChangePhoneNumber {}

impl AsRef<TGChangePhoneNumber> for TGChangePhoneNumber {
  fn as_ref(&self) -> &TGChangePhoneNumber { self }
}

impl TGChangePhoneNumber {

  pub fn new() -> Self {
    Self {
      phone_number: None,
      allow_flash_call: None,
      is_current_phone_number: None,
      
    }
  }

  
  pub fn phone_number<S: AsRef<str>>(&mut self, phone_number: S) -> &mut Self { self.phone_number = Some(phone_number.as_ref().to_string()); self }
  
  pub fn allow_flash_call(&mut self, allow_flash_call: bool) -> &mut Self { self.allow_flash_call = Some(allow_flash_call); self }
  
  pub fn is_current_phone_number(&mut self, is_current_phone_number: bool) -> &mut Self { self.is_current_phone_number = Some(is_current_phone_number); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> ChangePhoneNumber {
    ChangePhoneNumber::builder()
      .phone_number(self.phone_number.clone())
      .allow_flash_call(self.allow_flash_call.clone())
      .is_current_phone_number(self.is_current_phone_number.clone())
      
      .build()
  }
}


///  Installs/uninstalls or activates/archives a sticker set. 
#[derive(Debug, Clone)]
pub struct TGChangeStickerSet {
  ///  Identifier of the sticker set. 
  set_id: Option<i64>,
  ///  The new value of is_installed. 
  is_installed: Option<bool>,
  ///  The new value of is_archived. A sticker set can't be installed and archived simultaneously. 
  is_archived: Option<bool>,
  
}

impl TDFB for TGChangeStickerSet {}

impl AsRef<TGChangeStickerSet> for TGChangeStickerSet {
  fn as_ref(&self) -> &TGChangeStickerSet { self }
}

impl TGChangeStickerSet {

  pub fn new() -> Self {
    Self {
      set_id: None,
      is_installed: None,
      is_archived: None,
      
    }
  }

  
  pub fn set_id(&mut self, set_id: i64) -> &mut Self { self.set_id = Some(set_id); self }
  
  pub fn is_installed(&mut self, is_installed: bool) -> &mut Self { self.is_installed = Some(is_installed); self }
  
  pub fn is_archived(&mut self, is_archived: bool) -> &mut Self { self.is_archived = Some(is_archived); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> ChangeStickerSet {
    ChangeStickerSet::builder()
      .set_id(self.set_id.clone())
      .is_installed(self.is_installed.clone())
      .is_archived(self.is_archived.clone())
      
      .build()
  }
}


///  Checks the authentication token of a bot; to log in as a bot. Works only when the current authorization state is  
#[derive(Debug, Clone)]
pub struct TGCheckAuthenticationBotToken {
  ///  The bot token. 
  token: Option<String>,
  
}

impl TDFB for TGCheckAuthenticationBotToken {}

impl AsRef<TGCheckAuthenticationBotToken> for TGCheckAuthenticationBotToken {
  fn as_ref(&self) -> &TGCheckAuthenticationBotToken { self }
}

impl TGCheckAuthenticationBotToken {

  pub fn new() -> Self {
    Self {
      token: None,
      
    }
  }

  
  pub fn token<S: AsRef<str>>(&mut self, token: S) -> &mut Self { self.token = Some(token.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> CheckAuthenticationBotToken {
    CheckAuthenticationBotToken::builder()
      .token(self.token.clone())
      
      .build()
  }
}


///  Checks the authentication code. Works only when the current authorization state is  
#[derive(Debug, Clone)]
pub struct TGCheckAuthenticationCode {
  ///  The verification code received via SMS, Telegram message, phone call, or flash call. 
  code: Option<String>,
  ///  If the user is not yet registered, the first name of the user; 1-64 characters. You can also pass an empty string for unregistered user there to check verification code validness. In the latter case PHONE_NUMBER_UNOCCUPIED error will be returned for a valid code. 
  first_name: Option<String>,
  ///  If the user is not yet registered; the last name of the user; optional; 0-64 characters. 
  last_name: Option<String>,
  
}

impl TDFB for TGCheckAuthenticationCode {}

impl AsRef<TGCheckAuthenticationCode> for TGCheckAuthenticationCode {
  fn as_ref(&self) -> &TGCheckAuthenticationCode { self }
}

impl TGCheckAuthenticationCode {

  pub fn new() -> Self {
    Self {
      code: None,
      first_name: None,
      last_name: None,
      
    }
  }

  
  pub fn code<S: AsRef<str>>(&mut self, code: S) -> &mut Self { self.code = Some(code.as_ref().to_string()); self }
  
  pub fn first_name<S: AsRef<str>>(&mut self, first_name: S) -> &mut Self { self.first_name = Some(first_name.as_ref().to_string()); self }
  
  pub fn last_name<S: AsRef<str>>(&mut self, last_name: S) -> &mut Self { self.last_name = Some(last_name.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> CheckAuthenticationCode {
    CheckAuthenticationCode::builder()
      .code(self.code.clone())
      .first_name(self.first_name.clone())
      .last_name(self.last_name.clone())
      
      .build()
  }
}


///  Checks the authentication password for correctness. Works only when the current authorization state is  
#[derive(Debug, Clone)]
pub struct TGCheckAuthenticationPassword {
  ///  The password to check. 
  password: Option<String>,
  
}

impl TDFB for TGCheckAuthenticationPassword {}

impl AsRef<TGCheckAuthenticationPassword> for TGCheckAuthenticationPassword {
  fn as_ref(&self) -> &TGCheckAuthenticationPassword { self }
}

impl TGCheckAuthenticationPassword {

  pub fn new() -> Self {
    Self {
      password: None,
      
    }
  }

  
  pub fn password<S: AsRef<str>>(&mut self, password: S) -> &mut Self { self.password = Some(password.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> CheckAuthenticationPassword {
    CheckAuthenticationPassword::builder()
      .password(self.password.clone())
      
      .build()
  }
}


///  Checks the authentication code sent to confirm a new phone number of the user. 
#[derive(Debug, Clone)]
pub struct TGCheckChangePhoneNumberCode {
  ///  Verification code received by SMS, phone call or flash call. 
  code: Option<String>,
  
}

impl TDFB for TGCheckChangePhoneNumberCode {}

impl AsRef<TGCheckChangePhoneNumberCode> for TGCheckChangePhoneNumberCode {
  fn as_ref(&self) -> &TGCheckChangePhoneNumberCode { self }
}

impl TGCheckChangePhoneNumberCode {

  pub fn new() -> Self {
    Self {
      code: None,
      
    }
  }

  
  pub fn code<S: AsRef<str>>(&mut self, code: S) -> &mut Self { self.code = Some(code.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> CheckChangePhoneNumberCode {
    CheckChangePhoneNumberCode::builder()
      .code(self.code.clone())
      
      .build()
  }
}


///  Checks the validity of an invite link for a chat and returns information about the corresponding chat. 
#[derive(Debug, Clone)]
pub struct TGCheckChatInviteLink {
  ///  Invite link to be checked; should begin with "https://t.me/joinchat/", "https://telegram.me/joinchat/", or "https://telegram.dog/joinchat/". 
  invite_link: Option<String>,
  
}

impl TDFB for TGCheckChatInviteLink {}

impl AsRef<TGCheckChatInviteLink> for TGCheckChatInviteLink {
  fn as_ref(&self) -> &TGCheckChatInviteLink { self }
}

impl TGCheckChatInviteLink {

  pub fn new() -> Self {
    Self {
      invite_link: None,
      
    }
  }

  
  pub fn invite_link<S: AsRef<str>>(&mut self, invite_link: S) -> &mut Self { self.invite_link = Some(invite_link.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> CheckChatInviteLink {
    CheckChatInviteLink::builder()
      .invite_link(self.invite_link.clone())
      
      .build()
  }
}


///  Checks whether a username can be set for a chat. 
#[derive(Debug, Clone)]
pub struct TGCheckChatUsername {
  ///  Chat identifier; should be identifier of a supergroup chat, or a channel chat, or a private chat with self, or zero if chat is being created. 
  chat_id: Option<i64>,
  ///  Username to be checked. 
  username: Option<String>,
  
}

impl TDFB for TGCheckChatUsername {}

impl AsRef<TGCheckChatUsername> for TGCheckChatUsername {
  fn as_ref(&self) -> &TGCheckChatUsername { self }
}

impl TGCheckChatUsername {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      username: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn username<S: AsRef<str>>(&mut self, username: S) -> &mut Self { self.username = Some(username.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> CheckChatUsername {
    CheckChatUsername::builder()
      .chat_id(self.chat_id.clone())
      .username(self.username.clone())
      
      .build()
  }
}


///  Checks the database encryption key for correctness. Works only when the current authorization state is  
#[derive(Debug, Clone)]
pub struct TGCheckDatabaseEncryptionKey {
  ///  Encryption key to check or set up. 
  encryption_key: Option<String>,
  
}

impl TDFB for TGCheckDatabaseEncryptionKey {}

impl AsRef<TGCheckDatabaseEncryptionKey> for TGCheckDatabaseEncryptionKey {
  fn as_ref(&self) -> &TGCheckDatabaseEncryptionKey { self }
}

impl TGCheckDatabaseEncryptionKey {

  pub fn new() -> Self {
    Self {
      encryption_key: None,
      
    }
  }

  
  pub fn encryption_key<S: AsRef<str>>(&mut self, encryption_key: S) -> &mut Self { self.encryption_key = Some(encryption_key.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> CheckDatabaseEncryptionKey {
    CheckDatabaseEncryptionKey::builder()
      .encryption_key(self.encryption_key.clone())
      
      .build()
  }
}


///  Checks the email address verification code for Telegram Passport. 
#[derive(Debug, Clone)]
pub struct TGCheckEmailAddressVerificationCode {
  ///  Verification code. 
  code: Option<String>,
  
}

impl TDFB for TGCheckEmailAddressVerificationCode {}

impl AsRef<TGCheckEmailAddressVerificationCode> for TGCheckEmailAddressVerificationCode {
  fn as_ref(&self) -> &TGCheckEmailAddressVerificationCode { self }
}

impl TGCheckEmailAddressVerificationCode {

  pub fn new() -> Self {
    Self {
      code: None,
      
    }
  }

  
  pub fn code<S: AsRef<str>>(&mut self, code: S) -> &mut Self { self.code = Some(code.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> CheckEmailAddressVerificationCode {
    CheckEmailAddressVerificationCode::builder()
      .code(self.code.clone())
      
      .build()
  }
}


///  Checks phone number confirmation code. 
#[derive(Debug, Clone)]
pub struct TGCheckPhoneNumberConfirmationCode {
  ///  The phone number confirmation code. 
  code: Option<String>,
  
}

impl TDFB for TGCheckPhoneNumberConfirmationCode {}

impl AsRef<TGCheckPhoneNumberConfirmationCode> for TGCheckPhoneNumberConfirmationCode {
  fn as_ref(&self) -> &TGCheckPhoneNumberConfirmationCode { self }
}

impl TGCheckPhoneNumberConfirmationCode {

  pub fn new() -> Self {
    Self {
      code: None,
      
    }
  }

  
  pub fn code<S: AsRef<str>>(&mut self, code: S) -> &mut Self { self.code = Some(code.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> CheckPhoneNumberConfirmationCode {
    CheckPhoneNumberConfirmationCode::builder()
      .code(self.code.clone())
      
      .build()
  }
}


///  Checks the phone number verification code for Telegram Passport. 
#[derive(Debug, Clone)]
pub struct TGCheckPhoneNumberVerificationCode {
  ///  Verification code. 
  code: Option<String>,
  
}

impl TDFB for TGCheckPhoneNumberVerificationCode {}

impl AsRef<TGCheckPhoneNumberVerificationCode> for TGCheckPhoneNumberVerificationCode {
  fn as_ref(&self) -> &TGCheckPhoneNumberVerificationCode { self }
}

impl TGCheckPhoneNumberVerificationCode {

  pub fn new() -> Self {
    Self {
      code: None,
      
    }
  }

  
  pub fn code<S: AsRef<str>>(&mut self, code: S) -> &mut Self { self.code = Some(code.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> CheckPhoneNumberVerificationCode {
    CheckPhoneNumberVerificationCode::builder()
      .code(self.code.clone())
      
      .build()
  }
}


///  Checks the 2-step verification recovery email address verification code. 
#[derive(Debug, Clone)]
pub struct TGCheckRecoveryEmailAddressCode {
  ///  Verification code. 
  code: Option<String>,
  
}

impl TDFB for TGCheckRecoveryEmailAddressCode {}

impl AsRef<TGCheckRecoveryEmailAddressCode> for TGCheckRecoveryEmailAddressCode {
  fn as_ref(&self) -> &TGCheckRecoveryEmailAddressCode { self }
}

impl TGCheckRecoveryEmailAddressCode {

  pub fn new() -> Self {
    Self {
      code: None,
      
    }
  }

  
  pub fn code<S: AsRef<str>>(&mut self, code: S) -> &mut Self { self.code = Some(code.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> CheckRecoveryEmailAddressCode {
    CheckRecoveryEmailAddressCode::builder()
      .code(self.code.clone())
      
      .build()
  }
}


///  Removes potentially dangerous characters from the name of a file. The encoding of the file name is supposed to be UTF-8. Returns an empty string on failure. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGCleanFileName {
  ///  File name or path to the file. 
  file_name: Option<String>,
  
}

impl TDFB for TGCleanFileName {}

impl AsRef<TGCleanFileName> for TGCleanFileName {
  fn as_ref(&self) -> &TGCleanFileName { self }
}

impl TGCleanFileName {

  pub fn new() -> Self {
    Self {
      file_name: None,
      
    }
  }

  
  pub fn file_name<S: AsRef<str>>(&mut self, file_name: S) -> &mut Self { self.file_name = Some(file_name.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> CleanFileName {
    CleanFileName::builder()
      .file_name(self.file_name.clone())
      
      .build()
  }
}


///  Clears draft messages in all chats. 
#[derive(Debug, Clone)]
pub struct TGClearAllDraftMessages {
  ///  If true, local draft messages in secret chats will not be cleared. 
  exclude_secret_chats: Option<bool>,
  
}

impl TDFB for TGClearAllDraftMessages {}

impl AsRef<TGClearAllDraftMessages> for TGClearAllDraftMessages {
  fn as_ref(&self) -> &TGClearAllDraftMessages { self }
}

impl TGClearAllDraftMessages {

  pub fn new() -> Self {
    Self {
      exclude_secret_chats: None,
      
    }
  }

  
  pub fn exclude_secret_chats(&mut self, exclude_secret_chats: bool) -> &mut Self { self.exclude_secret_chats = Some(exclude_secret_chats); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> ClearAllDraftMessages {
    ClearAllDraftMessages::builder()
      .exclude_secret_chats(self.exclude_secret_chats.clone())
      
      .build()
  }
}


///  Clears all imported contacts, contact list remains unchanged. 
#[derive(Debug, Clone)]
pub struct TGClearImportedContacts {
  
}

impl TDFB for TGClearImportedContacts {}

impl AsRef<TGClearImportedContacts> for TGClearImportedContacts {
  fn as_ref(&self) -> &TGClearImportedContacts { self }
}

impl TGClearImportedContacts {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> ClearImportedContacts {
    ClearImportedContacts::builder()
      
      .build()
  }
}


///  Clears the list of recently found chats. 
#[derive(Debug, Clone)]
pub struct TGClearRecentlyFoundChats {
  
}

impl TDFB for TGClearRecentlyFoundChats {}

impl AsRef<TGClearRecentlyFoundChats> for TGClearRecentlyFoundChats {
  fn as_ref(&self) -> &TGClearRecentlyFoundChats { self }
}

impl TGClearRecentlyFoundChats {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> ClearRecentlyFoundChats {
    ClearRecentlyFoundChats::builder()
      
      .build()
  }
}


///  Clears the list of recently used stickers. 
#[derive(Debug, Clone)]
pub struct TGClearRecentStickers {
  ///  Pass true to clear the list of stickers recently attached to photo or video files; pass false to clear the list of recently sent stickers. 
  is_attached: Option<bool>,
  
}

impl TDFB for TGClearRecentStickers {}

impl AsRef<TGClearRecentStickers> for TGClearRecentStickers {
  fn as_ref(&self) -> &TGClearRecentStickers { self }
}

impl TGClearRecentStickers {

  pub fn new() -> Self {
    Self {
      is_attached: None,
      
    }
  }

  
  pub fn is_attached(&mut self, is_attached: bool) -> &mut Self { self.is_attached = Some(is_attached); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> ClearRecentStickers {
    ClearRecentStickers::builder()
      .is_attached(self.is_attached.clone())
      
      .build()
  }
}


///  Closes the TDLib instance. All databases will be flushed to disk and properly closed. After the close completes,  
#[derive(Debug, Clone)]
pub struct TGClose {
  
}

impl TDFB for TGClose {}

impl AsRef<TGClose> for TGClose {
  fn as_ref(&self) -> &TGClose { self }
}

impl TGClose {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> Close {
    Close::builder()
      
      .build()
  }
}


///  Informs TDLib that the chat is closed by the user. Many useful activities depend on the chat being opened or closed. 
#[derive(Debug, Clone)]
pub struct TGCloseChat {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  
}

impl TDFB for TGCloseChat {}

impl AsRef<TGCloseChat> for TGCloseChat {
  fn as_ref(&self) -> &TGCloseChat { self }
}

impl TGCloseChat {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> CloseChat {
    CloseChat::builder()
      .chat_id(self.chat_id.clone())
      
      .build()
  }
}


///  Closes a secret chat, effectively transfering its state to  
#[derive(Debug, Clone)]
pub struct TGCloseSecretChat {
  ///  Secret chat identifier. 
  secret_chat_id: Option<i32>,
  
}

impl TDFB for TGCloseSecretChat {}

impl AsRef<TGCloseSecretChat> for TGCloseSecretChat {
  fn as_ref(&self) -> &TGCloseSecretChat { self }
}

impl TGCloseSecretChat {

  pub fn new() -> Self {
    Self {
      secret_chat_id: None,
      
    }
  }

  
  pub fn secret_chat_id(&mut self, secret_chat_id: i32) -> &mut Self { self.secret_chat_id = Some(secret_chat_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> CloseSecretChat {
    CloseSecretChat::builder()
      .secret_chat_id(self.secret_chat_id.clone())
      
      .build()
  }
}


///  Returns an existing chat corresponding to a known basic group. 
#[derive(Debug, Clone)]
pub struct TGCreateBasicGroupChat {
  ///  Basic group identifier. 
  basic_group_id: Option<i32>,
  ///  If true, the chat will be created without network request. In this case all information about the chat except its type, title and photo can be incorrect. 
  force: Option<bool>,
  
}

impl TDFB for TGCreateBasicGroupChat {}

impl AsRef<TGCreateBasicGroupChat> for TGCreateBasicGroupChat {
  fn as_ref(&self) -> &TGCreateBasicGroupChat { self }
}

impl TGCreateBasicGroupChat {

  pub fn new() -> Self {
    Self {
      basic_group_id: None,
      force: None,
      
    }
  }

  
  pub fn basic_group_id(&mut self, basic_group_id: i32) -> &mut Self { self.basic_group_id = Some(basic_group_id); self }
  
  pub fn force(&mut self, force: bool) -> &mut Self { self.force = Some(force); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> CreateBasicGroupChat {
    CreateBasicGroupChat::builder()
      .basic_group_id(self.basic_group_id.clone())
      .force(self.force.clone())
      
      .build()
  }
}


///  Creates a new call. 
#[derive(Debug, Clone)]
pub struct TGCreateCall {
  ///  Identifier of the user to be called. 
  user_id: Option<i32>,
  ///  Description of the call protocols supported by the client. 
  protocol: Option<CallProtocol>,
  
}

impl TDFB for TGCreateCall {}

impl AsRef<TGCreateCall> for TGCreateCall {
  fn as_ref(&self) -> &TGCreateCall { self }
}

impl TGCreateCall {

  pub fn new() -> Self {
    Self {
      user_id: None,
      protocol: None,
      
    }
  }

  
  pub fn user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  


  
  // [protocol] type is [CallProtocol], is not support, need add manully.
  #[doc(hidden)] pub fn _protocol(&mut self, protocol: CallProtocol) -> &mut Self { self.protocol = Some(protocol); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> CreateCall {
    CreateCall::builder()
      .user_id(self.user_id.clone())
      .protocol(self.protocol.clone())
      
      .build()
  }
}


///  Creates a new basic group and sends a corresponding  
#[derive(Debug, Clone)]
pub struct TGCreateNewBasicGroupChat {
  ///  Identifiers of users to be added to the basic group. 
  user_ids: Option<Vec<i32>>,
  ///  Title of the new basic group; 1-128 characters. 
  title: Option<String>,
  
}

impl TDFB for TGCreateNewBasicGroupChat {}

impl AsRef<TGCreateNewBasicGroupChat> for TGCreateNewBasicGroupChat {
  fn as_ref(&self) -> &TGCreateNewBasicGroupChat { self }
}

impl TGCreateNewBasicGroupChat {

  pub fn new() -> Self {
    Self {
      user_ids: None,
      title: None,
      
    }
  }

  
  pub fn user_ids(&mut self, user_ids: Vec<i32>) -> &mut Self { self.user_ids = Some(user_ids); self }
  
  pub fn title<S: AsRef<str>>(&mut self, title: S) -> &mut Self { self.title = Some(title.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> CreateNewBasicGroupChat {
    CreateNewBasicGroupChat::builder()
      .user_ids(self.user_ids.clone())
      .title(self.title.clone())
      
      .build()
  }
}


///  Creates a new secret chat. Returns the newly created chat. 
#[derive(Debug, Clone)]
pub struct TGCreateNewSecretChat {
  ///  Identifier of the target user. 
  user_id: Option<i32>,
  
}

impl TDFB for TGCreateNewSecretChat {}

impl AsRef<TGCreateNewSecretChat> for TGCreateNewSecretChat {
  fn as_ref(&self) -> &TGCreateNewSecretChat { self }
}

impl TGCreateNewSecretChat {

  pub fn new() -> Self {
    Self {
      user_id: None,
      
    }
  }

  
  pub fn user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> CreateNewSecretChat {
    CreateNewSecretChat::builder()
      .user_id(self.user_id.clone())
      
      .build()
  }
}


///  Creates a new sticker set; for bots only. Returns the newly created sticker set. 
#[derive(Debug, Clone)]
pub struct TGCreateNewStickerSet {
  ///  Sticker set owner. 
  user_id: Option<i32>,
  ///  Sticker set title; 1-64 characters. 
  title: Option<String>,
  ///  Sticker set name. Can contain only English letters, digits and underscores. Must end with "by<bot username>" (<bot_username> is case insensitive); 1-64 characters. 
  name: Option<String>,
  ///  True, if stickers are masks. 
  is_masks: Option<bool>,
  ///  List of stickers to be added to the set. 
  stickers: Option<Vec<InputSticker>>,
  
}

impl TDFB for TGCreateNewStickerSet {}

impl AsRef<TGCreateNewStickerSet> for TGCreateNewStickerSet {
  fn as_ref(&self) -> &TGCreateNewStickerSet { self }
}

impl TGCreateNewStickerSet {

  pub fn new() -> Self {
    Self {
      user_id: None,
      title: None,
      name: None,
      is_masks: None,
      stickers: None,
      
    }
  }

  
  pub fn user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn title<S: AsRef<str>>(&mut self, title: S) -> &mut Self { self.title = Some(title.as_ref().to_string()); self }
  
  pub fn name<S: AsRef<str>>(&mut self, name: S) -> &mut Self { self.name = Some(name.as_ref().to_string()); self }
  
  pub fn is_masks(&mut self, is_masks: bool) -> &mut Self { self.is_masks = Some(is_masks); self }
  


  
  // [stickers] type is [Vec<InputSticker>], is not support, need add manully.
  #[doc(hidden)] pub fn _stickers(&mut self, stickers: Vec<InputSticker>) -> &mut Self { self.stickers = Some(stickers); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> CreateNewStickerSet {
    CreateNewStickerSet::builder()
      .user_id(self.user_id.clone())
      .title(self.title.clone())
      .name(self.name.clone())
      .is_masks(self.is_masks.clone())
      .stickers(self.stickers.clone())
      
      .build()
  }
}


///  Creates a new supergroup or channel and sends a corresponding  
#[derive(Debug, Clone)]
pub struct TGCreateNewSupergroupChat {
  ///  Title of the new chat; 1-128 characters. 
  title: Option<String>,
  ///  True, if a channel chat should be created. 
  is_channel: Option<bool>,
  ///  Chat description; 0-255 characters. 
  description: Option<String>,
  
}

impl TDFB for TGCreateNewSupergroupChat {}

impl AsRef<TGCreateNewSupergroupChat> for TGCreateNewSupergroupChat {
  fn as_ref(&self) -> &TGCreateNewSupergroupChat { self }
}

impl TGCreateNewSupergroupChat {

  pub fn new() -> Self {
    Self {
      title: None,
      is_channel: None,
      description: None,
      
    }
  }

  
  pub fn title<S: AsRef<str>>(&mut self, title: S) -> &mut Self { self.title = Some(title.as_ref().to_string()); self }
  
  pub fn is_channel(&mut self, is_channel: bool) -> &mut Self { self.is_channel = Some(is_channel); self }
  
  pub fn description<S: AsRef<str>>(&mut self, description: S) -> &mut Self { self.description = Some(description.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> CreateNewSupergroupChat {
    CreateNewSupergroupChat::builder()
      .title(self.title.clone())
      .is_channel(self.is_channel.clone())
      .description(self.description.clone())
      
      .build()
  }
}


///  Returns an existing chat corresponding to a given user. 
#[derive(Debug, Clone)]
pub struct TGCreatePrivateChat {
  ///  User identifier. 
  user_id: Option<i32>,
  ///  If true, the chat will be created without network request. In this case all information about the chat except its type, title and photo can be incorrect. 
  force: Option<bool>,
  
}

impl TDFB for TGCreatePrivateChat {}

impl AsRef<TGCreatePrivateChat> for TGCreatePrivateChat {
  fn as_ref(&self) -> &TGCreatePrivateChat { self }
}

impl TGCreatePrivateChat {

  pub fn new() -> Self {
    Self {
      user_id: None,
      force: None,
      
    }
  }

  
  pub fn user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn force(&mut self, force: bool) -> &mut Self { self.force = Some(force); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> CreatePrivateChat {
    CreatePrivateChat::builder()
      .user_id(self.user_id.clone())
      .force(self.force.clone())
      
      .build()
  }
}


///  Returns an existing chat corresponding to a known secret chat. 
#[derive(Debug, Clone)]
pub struct TGCreateSecretChat {
  ///  Secret chat identifier. 
  secret_chat_id: Option<i32>,
  
}

impl TDFB for TGCreateSecretChat {}

impl AsRef<TGCreateSecretChat> for TGCreateSecretChat {
  fn as_ref(&self) -> &TGCreateSecretChat { self }
}

impl TGCreateSecretChat {

  pub fn new() -> Self {
    Self {
      secret_chat_id: None,
      
    }
  }

  
  pub fn secret_chat_id(&mut self, secret_chat_id: i32) -> &mut Self { self.secret_chat_id = Some(secret_chat_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> CreateSecretChat {
    CreateSecretChat::builder()
      .secret_chat_id(self.secret_chat_id.clone())
      
      .build()
  }
}


///  Returns an existing chat corresponding to a known supergroup or channel. 
#[derive(Debug, Clone)]
pub struct TGCreateSupergroupChat {
  ///  Supergroup or channel identifier. 
  supergroup_id: Option<i32>,
  ///  If true, the chat will be created without network request. In this case all information about the chat except its type, title and photo can be incorrect. 
  force: Option<bool>,
  
}

impl TDFB for TGCreateSupergroupChat {}

impl AsRef<TGCreateSupergroupChat> for TGCreateSupergroupChat {
  fn as_ref(&self) -> &TGCreateSupergroupChat { self }
}

impl TGCreateSupergroupChat {

  pub fn new() -> Self {
    Self {
      supergroup_id: None,
      force: None,
      
    }
  }

  
  pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self { self.supergroup_id = Some(supergroup_id); self }
  
  pub fn force(&mut self, force: bool) -> &mut Self { self.force = Some(force); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> CreateSupergroupChat {
    CreateSupergroupChat::builder()
      .supergroup_id(self.supergroup_id.clone())
      .force(self.force.clone())
      
      .build()
  }
}


///  Creates a new temporary password for processing payments. 
#[derive(Debug, Clone)]
pub struct TGCreateTemporaryPassword {
  ///  Persistent user password. 
  password: Option<String>,
  ///  Time during which the temporary password will be valid, in seconds; should be between 60 and 86400. 
  valid_for: Option<i32>,
  
}

impl TDFB for TGCreateTemporaryPassword {}

impl AsRef<TGCreateTemporaryPassword> for TGCreateTemporaryPassword {
  fn as_ref(&self) -> &TGCreateTemporaryPassword { self }
}

impl TGCreateTemporaryPassword {

  pub fn new() -> Self {
    Self {
      password: None,
      valid_for: None,
      
    }
  }

  
  pub fn password<S: AsRef<str>>(&mut self, password: S) -> &mut Self { self.password = Some(password.as_ref().to_string()); self }
  
  pub fn valid_for(&mut self, valid_for: i32) -> &mut Self { self.valid_for = Some(valid_for); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> CreateTemporaryPassword {
    CreateTemporaryPassword::builder()
      .password(self.password.clone())
      .valid_for(self.valid_for.clone())
      
      .build()
  }
}


///  Deletes the account of the current user, deleting all information associated with the user from the server. The phone number of the account can be used to create a new account. Can be called before authorization when the current authorization state is  
#[derive(Debug, Clone)]
pub struct TGDeleteAccount {
  ///  The reason why the account was deleted; optional. 
  reason: Option<String>,
  
}

impl TDFB for TGDeleteAccount {}

impl AsRef<TGDeleteAccount> for TGDeleteAccount {
  fn as_ref(&self) -> &TGDeleteAccount { self }
}

impl TGDeleteAccount {

  pub fn new() -> Self {
    Self {
      reason: None,
      
    }
  }

  
  pub fn reason<S: AsRef<str>>(&mut self, reason: S) -> &mut Self { self.reason = Some(reason.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> DeleteAccount {
    DeleteAccount::builder()
      .reason(self.reason.clone())
      
      .build()
  }
}


///  Deletes all messages in the chat. Use Chat.can_be_deleted_only_for_self and Chat.can_be_deleted_for_all_users fields to find whether and how the method can be applied to the chat. 
#[derive(Debug, Clone)]
pub struct TGDeleteChatHistory {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  ///  Pass true if the chat should be removed from the chat list. 
  remove_from_chat_list: Option<bool>,
  ///  Pass true to try to delete chat history for all users. 
  revoke: Option<bool>,
  
}

impl TDFB for TGDeleteChatHistory {}

impl AsRef<TGDeleteChatHistory> for TGDeleteChatHistory {
  fn as_ref(&self) -> &TGDeleteChatHistory { self }
}

impl TGDeleteChatHistory {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      remove_from_chat_list: None,
      revoke: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn remove_from_chat_list(&mut self, remove_from_chat_list: bool) -> &mut Self { self.remove_from_chat_list = Some(remove_from_chat_list); self }
  
  pub fn revoke(&mut self, revoke: bool) -> &mut Self { self.revoke = Some(revoke); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> DeleteChatHistory {
    DeleteChatHistory::builder()
      .chat_id(self.chat_id.clone())
      .remove_from_chat_list(self.remove_from_chat_list.clone())
      .revoke(self.revoke.clone())
      
      .build()
  }
}


///  Deletes all messages sent by the specified user to a chat. Supported only in supergroups; requires can_delete_messages administrator privileges. 
#[derive(Debug, Clone)]
pub struct TGDeleteChatMessagesFromUser {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  ///  User identifier. 
  user_id: Option<i32>,
  
}

impl TDFB for TGDeleteChatMessagesFromUser {}

impl AsRef<TGDeleteChatMessagesFromUser> for TGDeleteChatMessagesFromUser {
  fn as_ref(&self) -> &TGDeleteChatMessagesFromUser { self }
}

impl TGDeleteChatMessagesFromUser {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      user_id: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> DeleteChatMessagesFromUser {
    DeleteChatMessagesFromUser::builder()
      .chat_id(self.chat_id.clone())
      .user_id(self.user_id.clone())
      
      .build()
  }
}


///  Deletes the default reply markup from a chat. Must be called after a one-time keyboard or a ForceReply reply markup has been used. UpdateChatReplyMarkup will be sent if the reply markup will be changed. 
#[derive(Debug, Clone)]
pub struct TGDeleteChatReplyMarkup {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  ///  The message identifier of the used keyboard. 
  message_id: Option<i64>,
  
}

impl TDFB for TGDeleteChatReplyMarkup {}

impl AsRef<TGDeleteChatReplyMarkup> for TGDeleteChatReplyMarkup {
  fn as_ref(&self) -> &TGDeleteChatReplyMarkup { self }
}

impl TGDeleteChatReplyMarkup {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      message_id: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> DeleteChatReplyMarkup {
    DeleteChatReplyMarkup::builder()
      .chat_id(self.chat_id.clone())
      .message_id(self.message_id.clone())
      
      .build()
  }
}


///  Deletes a file from the TDLib file cache. 
#[derive(Debug, Clone)]
pub struct TGDeleteFile {
  ///  Identifier of the file to delete. 
  file_id: Option<i32>,
  
}

impl TDFB for TGDeleteFile {}

impl AsRef<TGDeleteFile> for TGDeleteFile {
  fn as_ref(&self) -> &TGDeleteFile { self }
}

impl TGDeleteFile {

  pub fn new() -> Self {
    Self {
      file_id: None,
      
    }
  }

  
  pub fn file_id(&mut self, file_id: i32) -> &mut Self { self.file_id = Some(file_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> DeleteFile {
    DeleteFile::builder()
      .file_id(self.file_id.clone())
      
      .build()
  }
}


///  Deletes all information about a language pack in the current localization target. The language pack which is currently in use (including base language pack) or is being synchronized can't be deleted. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGDeleteLanguagePack {
  ///  Identifier of the language pack to delete. 
  language_pack_id: Option<String>,
  
}

impl TDFB for TGDeleteLanguagePack {}

impl AsRef<TGDeleteLanguagePack> for TGDeleteLanguagePack {
  fn as_ref(&self) -> &TGDeleteLanguagePack { self }
}

impl TGDeleteLanguagePack {

  pub fn new() -> Self {
    Self {
      language_pack_id: None,
      
    }
  }

  
  pub fn language_pack_id<S: AsRef<str>>(&mut self, language_pack_id: S) -> &mut Self { self.language_pack_id = Some(language_pack_id.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> DeleteLanguagePack {
    DeleteLanguagePack::builder()
      .language_pack_id(self.language_pack_id.clone())
      
      .build()
  }
}


///  Deletes messages. 
#[derive(Debug, Clone)]
pub struct TGDeleteMessages {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  ///  Identifiers of the messages to be deleted. 
  message_ids: Option<Vec<i64>>,
  ///  Pass true to try to delete messages for all chat members. Always true for supergroups, channels and secret chats. 
  revoke: Option<bool>,
  
}

impl TDFB for TGDeleteMessages {}

impl AsRef<TGDeleteMessages> for TGDeleteMessages {
  fn as_ref(&self) -> &TGDeleteMessages { self }
}

impl TGDeleteMessages {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      message_ids: None,
      revoke: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self { self.message_ids = Some(message_ids); self }
  
  pub fn revoke(&mut self, revoke: bool) -> &mut Self { self.revoke = Some(revoke); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> DeleteMessages {
    DeleteMessages::builder()
      .chat_id(self.chat_id.clone())
      .message_ids(self.message_ids.clone())
      .revoke(self.revoke.clone())
      
      .build()
  }
}


///  Deletes a Telegram Passport element. 
#[derive(Debug, Clone)]
pub struct TGDeletePassportElement {
  ///  Element type. 
  type_: Option<Box<PassportElementType>>,
  
}

impl TDFB for TGDeletePassportElement {}

impl AsRef<TGDeletePassportElement> for TGDeletePassportElement {
  fn as_ref(&self) -> &TGDeletePassportElement { self }
}

impl TGDeletePassportElement {

  pub fn new() -> Self {
    Self {
      type_: None,
      
    }
  }

  


  
  // [type_] type is [Box<PassportElementType>], is not support, need add manully.
  #[doc(hidden)] pub fn _type_(&mut self, type_: Box<PassportElementType>) -> &mut Self { self.type_ = Some(type_); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> DeletePassportElement {
    DeletePassportElement::builder()
      .type_(self.type_.clone())
      
      .build()
  }
}


///  Deletes a profile photo. If something changes,  
#[derive(Debug, Clone)]
pub struct TGDeleteProfilePhoto {
  ///  Identifier of the profile photo to delete. 
  profile_photo_id: Option<i64>,
  
}

impl TDFB for TGDeleteProfilePhoto {}

impl AsRef<TGDeleteProfilePhoto> for TGDeleteProfilePhoto {
  fn as_ref(&self) -> &TGDeleteProfilePhoto { self }
}

impl TGDeleteProfilePhoto {

  pub fn new() -> Self {
    Self {
      profile_photo_id: None,
      
    }
  }

  
  pub fn profile_photo_id(&mut self, profile_photo_id: i64) -> &mut Self { self.profile_photo_id = Some(profile_photo_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> DeleteProfilePhoto {
    DeleteProfilePhoto::builder()
      .profile_photo_id(self.profile_photo_id.clone())
      
      .build()
  }
}


///  Deletes saved credentials for all payment provider bots. 
#[derive(Debug, Clone)]
pub struct TGDeleteSavedCredentials {
  
}

impl TDFB for TGDeleteSavedCredentials {}

impl AsRef<TGDeleteSavedCredentials> for TGDeleteSavedCredentials {
  fn as_ref(&self) -> &TGDeleteSavedCredentials { self }
}

impl TGDeleteSavedCredentials {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> DeleteSavedCredentials {
    DeleteSavedCredentials::builder()
      
      .build()
  }
}


///  Deletes saved order info. 
#[derive(Debug, Clone)]
pub struct TGDeleteSavedOrderInfo {
  
}

impl TDFB for TGDeleteSavedOrderInfo {}

impl AsRef<TGDeleteSavedOrderInfo> for TGDeleteSavedOrderInfo {
  fn as_ref(&self) -> &TGDeleteSavedOrderInfo { self }
}

impl TGDeleteSavedOrderInfo {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> DeleteSavedOrderInfo {
    DeleteSavedOrderInfo::builder()
      
      .build()
  }
}


///  Deletes a supergroup or channel along with all messages in the corresponding chat. This will release the supergroup or channel username and remove all members; requires creator privileges in the supergroup or channel. Chats with more than 1000 members can't be deleted using this method. 
#[derive(Debug, Clone)]
pub struct TGDeleteSupergroup {
  ///  Identifier of the supergroup or channel. 
  supergroup_id: Option<i32>,
  
}

impl TDFB for TGDeleteSupergroup {}

impl AsRef<TGDeleteSupergroup> for TGDeleteSupergroup {
  fn as_ref(&self) -> &TGDeleteSupergroup { self }
}

impl TGDeleteSupergroup {

  pub fn new() -> Self {
    Self {
      supergroup_id: None,
      
    }
  }

  
  pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self { self.supergroup_id = Some(supergroup_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> DeleteSupergroup {
    DeleteSupergroup::builder()
      .supergroup_id(self.supergroup_id.clone())
      
      .build()
  }
}


///  Closes the TDLib instance, destroying all local data without a proper logout. The current user session will remain in the list of all active sessions. All local data will be destroyed. After the destruction completes  
#[derive(Debug, Clone)]
pub struct TGDestroy {
  
}

impl TDFB for TGDestroy {}

impl AsRef<TGDestroy> for TGDestroy {
  fn as_ref(&self) -> &TGDestroy { self }
}

impl TGDestroy {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> Destroy {
    Destroy::builder()
      
      .build()
  }
}


///  Disables the currently enabled proxy. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGDisableProxy {
  
}

impl TDFB for TGDisableProxy {}

impl AsRef<TGDisableProxy> for TGDisableProxy {
  fn as_ref(&self) -> &TGDisableProxy { self }
}

impl TGDisableProxy {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> DisableProxy {
    DisableProxy::builder()
      
      .build()
  }
}


///  Discards a call. 
#[derive(Debug, Clone)]
pub struct TGDiscardCall {
  ///  Call identifier. 
  call_id: Option<i32>,
  ///  True, if the user was disconnected. 
  is_disconnected: Option<bool>,
  ///  The call duration, in seconds. 
  duration: Option<i32>,
  ///  Identifier of the connection used during the call. 
  connection_id: Option<i64>,
  
}

impl TDFB for TGDiscardCall {}

impl AsRef<TGDiscardCall> for TGDiscardCall {
  fn as_ref(&self) -> &TGDiscardCall { self }
}

impl TGDiscardCall {

  pub fn new() -> Self {
    Self {
      call_id: None,
      is_disconnected: None,
      duration: None,
      connection_id: None,
      
    }
  }

  
  pub fn call_id(&mut self, call_id: i32) -> &mut Self { self.call_id = Some(call_id); self }
  
  pub fn is_disconnected(&mut self, is_disconnected: bool) -> &mut Self { self.is_disconnected = Some(is_disconnected); self }
  
  pub fn duration(&mut self, duration: i32) -> &mut Self { self.duration = Some(duration); self }
  
  pub fn connection_id(&mut self, connection_id: i64) -> &mut Self { self.connection_id = Some(connection_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> DiscardCall {
    DiscardCall::builder()
      .call_id(self.call_id.clone())
      .is_disconnected(self.is_disconnected.clone())
      .duration(self.duration.clone())
      .connection_id(self.connection_id.clone())
      
      .build()
  }
}


///  Disconnects all websites from the current user's Telegram account. 
#[derive(Debug, Clone)]
pub struct TGDisconnectAllWebsites {
  
}

impl TDFB for TGDisconnectAllWebsites {}

impl AsRef<TGDisconnectAllWebsites> for TGDisconnectAllWebsites {
  fn as_ref(&self) -> &TGDisconnectAllWebsites { self }
}

impl TGDisconnectAllWebsites {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> DisconnectAllWebsites {
    DisconnectAllWebsites::builder()
      
      .build()
  }
}


///  Disconnects website from the current user's Telegram account. 
#[derive(Debug, Clone)]
pub struct TGDisconnectWebsite {
  ///  Website identifier. 
  website_id: Option<i64>,
  
}

impl TDFB for TGDisconnectWebsite {}

impl AsRef<TGDisconnectWebsite> for TGDisconnectWebsite {
  fn as_ref(&self) -> &TGDisconnectWebsite { self }
}

impl TGDisconnectWebsite {

  pub fn new() -> Self {
    Self {
      website_id: None,
      
    }
  }

  
  pub fn website_id(&mut self, website_id: i64) -> &mut Self { self.website_id = Some(website_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> DisconnectWebsite {
    DisconnectWebsite::builder()
      .website_id(self.website_id.clone())
      
      .build()
  }
}


///  Downloads a file from the cloud. Download progress and completion of the download will be notified through  
#[derive(Debug, Clone)]
pub struct TGDownloadFile {
  ///  Identifier of the file to download. 
  file_id: Option<i32>,
  ///  Priority of the download (1-32). The higher the priority, the earlier the file will be downloaded. If the priorities of two files are equal, then the last one for which downloadFile was called will be downloaded first. 
  priority: Option<i32>,
  ///  The starting position from which the file should be downloaded. 
  offset: Option<i32>,
  ///  Number of bytes which should be downloaded starting from the "offset" position before the download will be automatically cancelled; use 0 to download without a limit. 
  limit: Option<i32>,
  ///  If false, this request returns file state just after the download has been started. If true, this request returns file state only after the download has succeeded, has failed, has been cancelled or a new downloadFile request with different offset/limit parameters was sent. 
  synchronous: Option<bool>,
  
}

impl TDFB for TGDownloadFile {}

impl AsRef<TGDownloadFile> for TGDownloadFile {
  fn as_ref(&self) -> &TGDownloadFile { self }
}

impl TGDownloadFile {

  pub fn new() -> Self {
    Self {
      file_id: None,
      priority: None,
      offset: None,
      limit: None,
      synchronous: None,
      
    }
  }

  
  pub fn file_id(&mut self, file_id: i32) -> &mut Self { self.file_id = Some(file_id); self }
  
  pub fn priority(&mut self, priority: i32) -> &mut Self { self.priority = Some(priority); self }
  
  pub fn offset(&mut self, offset: i32) -> &mut Self { self.offset = Some(offset); self }
  
  pub fn limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  
  pub fn synchronous(&mut self, synchronous: bool) -> &mut Self { self.synchronous = Some(synchronous); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> DownloadFile {
    DownloadFile::builder()
      .file_id(self.file_id.clone())
      .priority(self.priority.clone())
      .offset(self.offset.clone())
      .limit(self.limit.clone())
      .synchronous(self.synchronous.clone())
      
      .build()
  }
}


///  Edits information about a custom local language pack in the current localization target. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGEditCustomLanguagePackInfo {
  ///  New information about the custom local language pack. 
  info: Option<LanguagePackInfo>,
  
}

impl TDFB for TGEditCustomLanguagePackInfo {}

impl AsRef<TGEditCustomLanguagePackInfo> for TGEditCustomLanguagePackInfo {
  fn as_ref(&self) -> &TGEditCustomLanguagePackInfo { self }
}

impl TGEditCustomLanguagePackInfo {

  pub fn new() -> Self {
    Self {
      info: None,
      
    }
  }

  


  
  // [info] type is [LanguagePackInfo], is not support, need add manully.
  #[doc(hidden)] pub fn _info(&mut self, info: LanguagePackInfo) -> &mut Self { self.info = Some(info); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> EditCustomLanguagePackInfo {
    EditCustomLanguagePackInfo::builder()
      .info(self.info.clone())
      
      .build()
  }
}


///  Edits the caption of an inline message sent via a bot; for bots only. 
#[derive(Debug, Clone)]
pub struct TGEditInlineMessageCaption {
  ///  Inline message identifier. 
  inline_message_id: Option<String>,
  ///  The new message reply markup. 
  reply_markup: Option<Box<ReplyMarkup>>,
  ///  New message content caption; 0-GetOption("message_caption_length_max") characters. 
  caption: Option<FormattedText>,
  
}

impl TDFB for TGEditInlineMessageCaption {}

impl AsRef<TGEditInlineMessageCaption> for TGEditInlineMessageCaption {
  fn as_ref(&self) -> &TGEditInlineMessageCaption { self }
}

impl TGEditInlineMessageCaption {

  pub fn new() -> Self {
    Self {
      inline_message_id: None,
      reply_markup: None,
      caption: None,
      
    }
  }

  
  pub fn inline_message_id<S: AsRef<str>>(&mut self, inline_message_id: S) -> &mut Self { self.inline_message_id = Some(inline_message_id.as_ref().to_string()); self }
  


  
  // [reply_markup] type is [Box<ReplyMarkup>], is not support, need add manully.
  #[doc(hidden)] pub fn _reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  
  // [caption] type is [FormattedText], is not support, need add manully.
  #[doc(hidden)] pub fn _caption(&mut self, caption: FormattedText) -> &mut Self { self.caption = Some(caption); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> EditInlineMessageCaption {
    EditInlineMessageCaption::builder()
      .inline_message_id(self.inline_message_id.clone())
      .reply_markup(self.reply_markup.clone())
      .caption(self.caption.clone())
      
      .build()
  }
}


///  Edits the content of a live location in an inline message sent via a bot; for bots only. 
#[derive(Debug, Clone)]
pub struct TGEditInlineMessageLiveLocation {
  ///  Inline message identifier. 
  inline_message_id: Option<String>,
  ///  The new message reply markup. 
  reply_markup: Option<Box<ReplyMarkup>>,
  ///  New location content of the message; may be null. Pass null to stop sharing the live location. 
  location: Option<Location>,
  
}

impl TDFB for TGEditInlineMessageLiveLocation {}

impl AsRef<TGEditInlineMessageLiveLocation> for TGEditInlineMessageLiveLocation {
  fn as_ref(&self) -> &TGEditInlineMessageLiveLocation { self }
}

impl TGEditInlineMessageLiveLocation {

  pub fn new() -> Self {
    Self {
      inline_message_id: None,
      reply_markup: None,
      location: None,
      
    }
  }

  
  pub fn inline_message_id<S: AsRef<str>>(&mut self, inline_message_id: S) -> &mut Self { self.inline_message_id = Some(inline_message_id.as_ref().to_string()); self }
  


  
  // [reply_markup] type is [Box<ReplyMarkup>], is not support, need add manully.
  #[doc(hidden)] pub fn _reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  
  // [location] type is [Location], is not support, need add manully.
  #[doc(hidden)] pub fn _location(&mut self, location: Location) -> &mut Self { self.location = Some(location); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> EditInlineMessageLiveLocation {
    EditInlineMessageLiveLocation::builder()
      .inline_message_id(self.inline_message_id.clone())
      .reply_markup(self.reply_markup.clone())
      .location(self.location.clone())
      
      .build()
  }
}


///  Edits the content of a message with an animation, an audio, a document, a photo or a video in an inline message sent via a bot; for bots only. 
#[derive(Debug, Clone)]
pub struct TGEditInlineMessageMedia {
  ///  Inline message identifier. 
  inline_message_id: Option<String>,
  ///  The new message reply markup; for bots only. 
  reply_markup: Option<Box<ReplyMarkup>>,
  ///  New content of the message. Must be one of the following types: InputMessageAnimation, InputMessageAudio, InputMessageDocument, InputMessagePhoto or InputMessageVideo. 
  input_message_content: Option<Box<InputMessageContent>>,
  
}

impl TDFB for TGEditInlineMessageMedia {}

impl AsRef<TGEditInlineMessageMedia> for TGEditInlineMessageMedia {
  fn as_ref(&self) -> &TGEditInlineMessageMedia { self }
}

impl TGEditInlineMessageMedia {

  pub fn new() -> Self {
    Self {
      inline_message_id: None,
      reply_markup: None,
      input_message_content: None,
      
    }
  }

  
  pub fn inline_message_id<S: AsRef<str>>(&mut self, inline_message_id: S) -> &mut Self { self.inline_message_id = Some(inline_message_id.as_ref().to_string()); self }
  


  
  // [reply_markup] type is [Box<ReplyMarkup>], is not support, need add manully.
  #[doc(hidden)] pub fn _reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  
  // [input_message_content] type is [Box<InputMessageContent>], is not support, need add manully.
  #[doc(hidden)] pub fn _input_message_content(&mut self, input_message_content: Box<InputMessageContent>) -> &mut Self { self.input_message_content = Some(input_message_content); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> EditInlineMessageMedia {
    EditInlineMessageMedia::builder()
      .inline_message_id(self.inline_message_id.clone())
      .reply_markup(self.reply_markup.clone())
      .input_message_content(self.input_message_content.clone())
      
      .build()
  }
}


///  Edits the reply markup of an inline message sent via a bot; for bots only. 
#[derive(Debug, Clone)]
pub struct TGEditInlineMessageReplyMarkup {
  ///  Inline message identifier. 
  inline_message_id: Option<String>,
  ///  The new message reply markup. 
  reply_markup: Option<Box<ReplyMarkup>>,
  
}

impl TDFB for TGEditInlineMessageReplyMarkup {}

impl AsRef<TGEditInlineMessageReplyMarkup> for TGEditInlineMessageReplyMarkup {
  fn as_ref(&self) -> &TGEditInlineMessageReplyMarkup { self }
}

impl TGEditInlineMessageReplyMarkup {

  pub fn new() -> Self {
    Self {
      inline_message_id: None,
      reply_markup: None,
      
    }
  }

  
  pub fn inline_message_id<S: AsRef<str>>(&mut self, inline_message_id: S) -> &mut Self { self.inline_message_id = Some(inline_message_id.as_ref().to_string()); self }
  


  
  // [reply_markup] type is [Box<ReplyMarkup>], is not support, need add manully.
  #[doc(hidden)] pub fn _reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> EditInlineMessageReplyMarkup {
    EditInlineMessageReplyMarkup::builder()
      .inline_message_id(self.inline_message_id.clone())
      .reply_markup(self.reply_markup.clone())
      
      .build()
  }
}


///  Edits the text of an inline text or game message sent via a bot; for bots only. 
#[derive(Debug, Clone)]
pub struct TGEditInlineMessageText {
  ///  Inline message identifier. 
  inline_message_id: Option<String>,
  ///  The new message reply markup. 
  reply_markup: Option<Box<ReplyMarkup>>,
  ///  New text content of the message. Should be of type InputMessageText. 
  input_message_content: Option<Box<InputMessageContent>>,
  
}

impl TDFB for TGEditInlineMessageText {}

impl AsRef<TGEditInlineMessageText> for TGEditInlineMessageText {
  fn as_ref(&self) -> &TGEditInlineMessageText { self }
}

impl TGEditInlineMessageText {

  pub fn new() -> Self {
    Self {
      inline_message_id: None,
      reply_markup: None,
      input_message_content: None,
      
    }
  }

  
  pub fn inline_message_id<S: AsRef<str>>(&mut self, inline_message_id: S) -> &mut Self { self.inline_message_id = Some(inline_message_id.as_ref().to_string()); self }
  


  
  // [reply_markup] type is [Box<ReplyMarkup>], is not support, need add manully.
  #[doc(hidden)] pub fn _reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  
  // [input_message_content] type is [Box<InputMessageContent>], is not support, need add manully.
  #[doc(hidden)] pub fn _input_message_content(&mut self, input_message_content: Box<InputMessageContent>) -> &mut Self { self.input_message_content = Some(input_message_content); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> EditInlineMessageText {
    EditInlineMessageText::builder()
      .inline_message_id(self.inline_message_id.clone())
      .reply_markup(self.reply_markup.clone())
      .input_message_content(self.input_message_content.clone())
      
      .build()
  }
}


///  Edits the message content caption. Returns the edited message after the edit is completed on the server side. 
#[derive(Debug, Clone)]
pub struct TGEditMessageCaption {
  ///  The chat the message belongs to. 
  chat_id: Option<i64>,
  ///  Identifier of the message. 
  message_id: Option<i64>,
  ///  The new message reply markup; for bots only. 
  reply_markup: Option<Box<ReplyMarkup>>,
  ///  New message content caption; 0-GetOption("message_caption_length_max") characters. 
  caption: Option<FormattedText>,
  
}

impl TDFB for TGEditMessageCaption {}

impl AsRef<TGEditMessageCaption> for TGEditMessageCaption {
  fn as_ref(&self) -> &TGEditMessageCaption { self }
}

impl TGEditMessageCaption {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      message_id: None,
      reply_markup: None,
      caption: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  


  
  // [reply_markup] type is [Box<ReplyMarkup>], is not support, need add manully.
  #[doc(hidden)] pub fn _reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  
  // [caption] type is [FormattedText], is not support, need add manully.
  #[doc(hidden)] pub fn _caption(&mut self, caption: FormattedText) -> &mut Self { self.caption = Some(caption); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> EditMessageCaption {
    EditMessageCaption::builder()
      .chat_id(self.chat_id.clone())
      .message_id(self.message_id.clone())
      .reply_markup(self.reply_markup.clone())
      .caption(self.caption.clone())
      
      .build()
  }
}


///  Edits the message content of a live location. Messages can be edited for a limited period of time specified in the live location. Returns the edited message after the edit is completed on the server side. 
#[derive(Debug, Clone)]
pub struct TGEditMessageLiveLocation {
  ///  The chat the message belongs to. 
  chat_id: Option<i64>,
  ///  Identifier of the message. 
  message_id: Option<i64>,
  ///  The new message reply markup; for bots only. 
  reply_markup: Option<Box<ReplyMarkup>>,
  ///  New location content of the message; may be null. Pass null to stop sharing the live location. 
  location: Option<Location>,
  
}

impl TDFB for TGEditMessageLiveLocation {}

impl AsRef<TGEditMessageLiveLocation> for TGEditMessageLiveLocation {
  fn as_ref(&self) -> &TGEditMessageLiveLocation { self }
}

impl TGEditMessageLiveLocation {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      message_id: None,
      reply_markup: None,
      location: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  


  
  // [reply_markup] type is [Box<ReplyMarkup>], is not support, need add manully.
  #[doc(hidden)] pub fn _reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  
  // [location] type is [Location], is not support, need add manully.
  #[doc(hidden)] pub fn _location(&mut self, location: Location) -> &mut Self { self.location = Some(location); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> EditMessageLiveLocation {
    EditMessageLiveLocation::builder()
      .chat_id(self.chat_id.clone())
      .message_id(self.message_id.clone())
      .reply_markup(self.reply_markup.clone())
      .location(self.location.clone())
      
      .build()
  }
}


///  Edits the content of a message with an animation, an audio, a document, a photo or a video. The media in the message can't be replaced if the message was set to self-destruct. Media can't be replaced by self-destructing media. Media in an album can be edited only to contain a photo or a video. Returns the edited message after the edit is completed on the server side. 
#[derive(Debug, Clone)]
pub struct TGEditMessageMedia {
  ///  The chat the message belongs to. 
  chat_id: Option<i64>,
  ///  Identifier of the message. 
  message_id: Option<i64>,
  ///  The new message reply markup; for bots only. 
  reply_markup: Option<Box<ReplyMarkup>>,
  ///  New content of the message. Must be one of the following types: InputMessageAnimation, InputMessageAudio, InputMessageDocument, InputMessagePhoto or InputMessageVideo. 
  input_message_content: Option<Box<InputMessageContent>>,
  
}

impl TDFB for TGEditMessageMedia {}

impl AsRef<TGEditMessageMedia> for TGEditMessageMedia {
  fn as_ref(&self) -> &TGEditMessageMedia { self }
}

impl TGEditMessageMedia {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      message_id: None,
      reply_markup: None,
      input_message_content: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  


  
  // [reply_markup] type is [Box<ReplyMarkup>], is not support, need add manully.
  #[doc(hidden)] pub fn _reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  
  // [input_message_content] type is [Box<InputMessageContent>], is not support, need add manully.
  #[doc(hidden)] pub fn _input_message_content(&mut self, input_message_content: Box<InputMessageContent>) -> &mut Self { self.input_message_content = Some(input_message_content); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> EditMessageMedia {
    EditMessageMedia::builder()
      .chat_id(self.chat_id.clone())
      .message_id(self.message_id.clone())
      .reply_markup(self.reply_markup.clone())
      .input_message_content(self.input_message_content.clone())
      
      .build()
  }
}


///  Edits the message reply markup; for bots only. Returns the edited message after the edit is completed on the server side. 
#[derive(Debug, Clone)]
pub struct TGEditMessageReplyMarkup {
  ///  The chat the message belongs to. 
  chat_id: Option<i64>,
  ///  Identifier of the message. 
  message_id: Option<i64>,
  ///  The new message reply markup. 
  reply_markup: Option<Box<ReplyMarkup>>,
  
}

impl TDFB for TGEditMessageReplyMarkup {}

impl AsRef<TGEditMessageReplyMarkup> for TGEditMessageReplyMarkup {
  fn as_ref(&self) -> &TGEditMessageReplyMarkup { self }
}

impl TGEditMessageReplyMarkup {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      message_id: None,
      reply_markup: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  


  
  // [reply_markup] type is [Box<ReplyMarkup>], is not support, need add manully.
  #[doc(hidden)] pub fn _reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> EditMessageReplyMarkup {
    EditMessageReplyMarkup::builder()
      .chat_id(self.chat_id.clone())
      .message_id(self.message_id.clone())
      .reply_markup(self.reply_markup.clone())
      
      .build()
  }
}


///  Edits the text of a message (or a text of a game message). Returns the edited message after the edit is completed on the server side. 
#[derive(Debug, Clone)]
pub struct TGEditMessageText {
  ///  The chat the message belongs to. 
  chat_id: Option<i64>,
  ///  Identifier of the message. 
  message_id: Option<i64>,
  ///  The new message reply markup; for bots only. 
  reply_markup: Option<Box<ReplyMarkup>>,
  ///  New text content of the message. Should be of type InputMessageText. 
  input_message_content: Option<Box<InputMessageContent>>,
  
}

impl TDFB for TGEditMessageText {}

impl AsRef<TGEditMessageText> for TGEditMessageText {
  fn as_ref(&self) -> &TGEditMessageText { self }
}

impl TGEditMessageText {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      message_id: None,
      reply_markup: None,
      input_message_content: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  


  
  // [reply_markup] type is [Box<ReplyMarkup>], is not support, need add manully.
  #[doc(hidden)] pub fn _reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  
  // [input_message_content] type is [Box<InputMessageContent>], is not support, need add manully.
  #[doc(hidden)] pub fn _input_message_content(&mut self, input_message_content: Box<InputMessageContent>) -> &mut Self { self.input_message_content = Some(input_message_content); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> EditMessageText {
    EditMessageText::builder()
      .chat_id(self.chat_id.clone())
      .message_id(self.message_id.clone())
      .reply_markup(self.reply_markup.clone())
      .input_message_content(self.input_message_content.clone())
      
      .build()
  }
}


///  Edits an existing proxy server for network requests. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGEditProxy {
  ///  Proxy identifier. 
  proxy_id: Option<i32>,
  ///  Proxy server IP address. 
  server: Option<String>,
  ///  Proxy server port. 
  port: Option<i32>,
  ///  True, if the proxy should be enabled. 
  enable: Option<bool>,
  ///  Proxy type. 
  type_: Option<Box<ProxyType>>,
  
}

impl TDFB for TGEditProxy {}

impl AsRef<TGEditProxy> for TGEditProxy {
  fn as_ref(&self) -> &TGEditProxy { self }
}

impl TGEditProxy {

  pub fn new() -> Self {
    Self {
      proxy_id: None,
      server: None,
      port: None,
      enable: None,
      type_: None,
      
    }
  }

  
  pub fn proxy_id(&mut self, proxy_id: i32) -> &mut Self { self.proxy_id = Some(proxy_id); self }
  
  pub fn server<S: AsRef<str>>(&mut self, server: S) -> &mut Self { self.server = Some(server.as_ref().to_string()); self }
  
  pub fn port(&mut self, port: i32) -> &mut Self { self.port = Some(port); self }
  
  pub fn enable(&mut self, enable: bool) -> &mut Self { self.enable = Some(enable); self }
  


  
  // [type_] type is [Box<ProxyType>], is not support, need add manully.
  #[doc(hidden)] pub fn _type_(&mut self, type_: Box<ProxyType>) -> &mut Self { self.type_ = Some(type_); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> EditProxy {
    EditProxy::builder()
      .proxy_id(self.proxy_id.clone())
      .server(self.server.clone())
      .port(self.port.clone())
      .enable(self.enable.clone())
      .type_(self.type_.clone())
      
      .build()
  }
}


///  Enables a proxy. Only one proxy can be enabled at a time. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGEnableProxy {
  ///  Proxy identifier. 
  proxy_id: Option<i32>,
  
}

impl TDFB for TGEnableProxy {}

impl AsRef<TGEnableProxy> for TGEnableProxy {
  fn as_ref(&self) -> &TGEnableProxy { self }
}

impl TGEnableProxy {

  pub fn new() -> Self {
    Self {
      proxy_id: None,
      
    }
  }

  
  pub fn proxy_id(&mut self, proxy_id: i32) -> &mut Self { self.proxy_id = Some(proxy_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> EnableProxy {
    EnableProxy::builder()
      .proxy_id(self.proxy_id.clone())
      
      .build()
  }
}


///  Finishes the file generation. 
#[derive(Debug, Clone)]
pub struct TGFinishFileGeneration {
  ///  The identifier of the generation process. 
  generation_id: Option<i64>,
  ///  If set, means that file generation has failed and should be terminated. 
  error: Option<Error>,
  
}

impl TDFB for TGFinishFileGeneration {}

impl AsRef<TGFinishFileGeneration> for TGFinishFileGeneration {
  fn as_ref(&self) -> &TGFinishFileGeneration { self }
}

impl TGFinishFileGeneration {

  pub fn new() -> Self {
    Self {
      generation_id: None,
      error: None,
      
    }
  }

  
  pub fn generation_id(&mut self, generation_id: i64) -> &mut Self { self.generation_id = Some(generation_id); self }
  


  
  // [error] type is [Error], is not support, need add manully.
  #[doc(hidden)] pub fn _error(&mut self, error: Error) -> &mut Self { self.error = Some(error); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> FinishFileGeneration {
    FinishFileGeneration::builder()
      .generation_id(self.generation_id.clone())
      .error(self.error.clone())
      
      .build()
  }
}


///  Forwards previously sent messages. Returns the forwarded messages in the same order as the message identifiers passed in message_ids. If a message can't be forwarded, null will be returned instead of the message. 
#[derive(Debug, Clone)]
pub struct TGForwardMessages {
  ///  Identifier of the chat to which to forward messages. 
  chat_id: Option<i64>,
  ///  Identifier of the chat from which to forward messages. 
  from_chat_id: Option<i64>,
  ///  Identifiers of the messages to forward. 
  message_ids: Option<Vec<i64>>,
  ///  Pass true to disable notification for the message, doesn't work if messages are forwarded to a secret chat. 
  disable_notification: Option<bool>,
  ///  Pass true if the message is sent from the background. 
  from_background: Option<bool>,
  ///  True, if the messages should be grouped into an album after forwarding. For this to work, no more than 10 messages may be forwarded, and all of them must be photo or video messages. 
  as_album: Option<bool>,
  
}

impl TDFB for TGForwardMessages {}

impl AsRef<TGForwardMessages> for TGForwardMessages {
  fn as_ref(&self) -> &TGForwardMessages { self }
}

impl TGForwardMessages {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      from_chat_id: None,
      message_ids: None,
      disable_notification: None,
      from_background: None,
      as_album: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn from_chat_id(&mut self, from_chat_id: i64) -> &mut Self { self.from_chat_id = Some(from_chat_id); self }
  
  pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self { self.message_ids = Some(message_ids); self }
  
  pub fn disable_notification(&mut self, disable_notification: bool) -> &mut Self { self.disable_notification = Some(disable_notification); self }
  
  pub fn from_background(&mut self, from_background: bool) -> &mut Self { self.from_background = Some(from_background); self }
  
  pub fn as_album(&mut self, as_album: bool) -> &mut Self { self.as_album = Some(as_album); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> ForwardMessages {
    ForwardMessages::builder()
      .chat_id(self.chat_id.clone())
      .from_chat_id(self.from_chat_id.clone())
      .message_ids(self.message_ids.clone())
      .disable_notification(self.disable_notification.clone())
      .from_background(self.from_background.clone())
      .as_album(self.as_album.clone())
      
      .build()
  }
}


///  Generates a new invite link for a chat; the previously generated link is revoked. Available for basic groups, supergroups, and channels. In basic groups this can be called only by the group's creator; in supergroups and channels this requires appropriate administrator rights. 
#[derive(Debug, Clone)]
pub struct TGGenerateChatInviteLink {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  
}

impl TDFB for TGGenerateChatInviteLink {}

impl AsRef<TGGenerateChatInviteLink> for TGGenerateChatInviteLink {
  fn as_ref(&self) -> &TGGenerateChatInviteLink { self }
}

impl TGGenerateChatInviteLink {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GenerateChatInviteLink {
    GenerateChatInviteLink::builder()
      .chat_id(self.chat_id.clone())
      
      .build()
  }
}


///  Returns the period of inactivity after which the account of the current user will automatically be deleted. 
#[derive(Debug, Clone)]
pub struct TGGetAccountTtl {
  
}

impl TDFB for TGGetAccountTtl {}

impl AsRef<TGGetAccountTtl> for TGGetAccountTtl {
  fn as_ref(&self) -> &TGGetAccountTtl { self }
}

impl TGGetAccountTtl {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetAccountTtl {
    GetAccountTtl::builder()
      
      .build()
  }
}


///  Returns all active live locations that should be updated by the client. The list is persistent across application restarts only if the message database is used. 
#[derive(Debug, Clone)]
pub struct TGGetActiveLiveLocationMessages {
  
}

impl TDFB for TGGetActiveLiveLocationMessages {}

impl AsRef<TGGetActiveLiveLocationMessages> for TGGetActiveLiveLocationMessages {
  fn as_ref(&self) -> &TGGetActiveLiveLocationMessages { self }
}

impl TGGetActiveLiveLocationMessages {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetActiveLiveLocationMessages {
    GetActiveLiveLocationMessages::builder()
      
      .build()
  }
}


///  Returns all active sessions of the current user. 
#[derive(Debug, Clone)]
pub struct TGGetActiveSessions {
  
}

impl TDFB for TGGetActiveSessions {}

impl AsRef<TGGetActiveSessions> for TGGetActiveSessions {
  fn as_ref(&self) -> &TGGetActiveSessions { self }
}

impl TGGetActiveSessions {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetActiveSessions {
    GetActiveSessions::builder()
      
      .build()
  }
}


///  Returns all available Telegram Passport elements. 
#[derive(Debug, Clone)]
pub struct TGGetAllPassportElements {
  ///  Password of the current user. 
  password: Option<String>,
  
}

impl TDFB for TGGetAllPassportElements {}

impl AsRef<TGGetAllPassportElements> for TGGetAllPassportElements {
  fn as_ref(&self) -> &TGGetAllPassportElements { self }
}

impl TGGetAllPassportElements {

  pub fn new() -> Self {
    Self {
      password: None,
      
    }
  }

  
  pub fn password<S: AsRef<str>>(&mut self, password: S) -> &mut Self { self.password = Some(password.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetAllPassportElements {
    GetAllPassportElements::builder()
      .password(self.password.clone())
      
      .build()
  }
}


///  Returns application config, provided by the server. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGGetApplicationConfig {
  
}

impl TDFB for TGGetApplicationConfig {}

impl AsRef<TGGetApplicationConfig> for TGGetApplicationConfig {
  fn as_ref(&self) -> &TGGetApplicationConfig { self }
}

impl TGGetApplicationConfig {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetApplicationConfig {
    GetApplicationConfig::builder()
      
      .build()
  }
}


///  Returns a list of archived sticker sets. 
#[derive(Debug, Clone)]
pub struct TGGetArchivedStickerSets {
  ///  Pass true to return mask stickers sets; pass false to return ordinary sticker sets. 
  is_masks: Option<bool>,
  ///  Identifier of the sticker set from which to return the result. 
  offset_sticker_set_id: Option<i64>,
  ///  Maximum number of sticker sets to return. 
  limit: Option<i32>,
  
}

impl TDFB for TGGetArchivedStickerSets {}

impl AsRef<TGGetArchivedStickerSets> for TGGetArchivedStickerSets {
  fn as_ref(&self) -> &TGGetArchivedStickerSets { self }
}

impl TGGetArchivedStickerSets {

  pub fn new() -> Self {
    Self {
      is_masks: None,
      offset_sticker_set_id: None,
      limit: None,
      
    }
  }

  
  pub fn is_masks(&mut self, is_masks: bool) -> &mut Self { self.is_masks = Some(is_masks); self }
  
  pub fn offset_sticker_set_id(&mut self, offset_sticker_set_id: i64) -> &mut Self { self.offset_sticker_set_id = Some(offset_sticker_set_id); self }
  
  pub fn limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetArchivedStickerSets {
    GetArchivedStickerSets::builder()
      .is_masks(self.is_masks.clone())
      .offset_sticker_set_id(self.offset_sticker_set_id.clone())
      .limit(self.limit.clone())
      
      .build()
  }
}


///  Returns a list of sticker sets attached to a file. Currently only photos and videos can have attached sticker sets. 
#[derive(Debug, Clone)]
pub struct TGGetAttachedStickerSets {
  ///  File identifier. 
  file_id: Option<i32>,
  
}

impl TDFB for TGGetAttachedStickerSets {}

impl AsRef<TGGetAttachedStickerSets> for TGGetAttachedStickerSets {
  fn as_ref(&self) -> &TGGetAttachedStickerSets { self }
}

impl TGGetAttachedStickerSets {

  pub fn new() -> Self {
    Self {
      file_id: None,
      
    }
  }

  
  pub fn file_id(&mut self, file_id: i32) -> &mut Self { self.file_id = Some(file_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetAttachedStickerSets {
    GetAttachedStickerSets::builder()
      .file_id(self.file_id.clone())
      
      .build()
  }
}


///  Returns the current authorization state; this is an offline request. For informational purposes only. Use  
#[derive(Debug, Clone)]
pub struct TGGetAuthorizationState {
  
}

impl TDFB for TGGetAuthorizationState {}

impl AsRef<TGGetAuthorizationState> for TGGetAuthorizationState {
  fn as_ref(&self) -> &TGGetAuthorizationState { self }
}

impl TGGetAuthorizationState {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetAuthorizationState {
    GetAuthorizationState::builder()
      
      .build()
  }
}


///  Returns information about a basic group by its identifier. This is an offline request if the current user is not a bot. 
#[derive(Debug, Clone)]
pub struct TGGetBasicGroup {
  ///  Basic group identifier. 
  basic_group_id: Option<i32>,
  
}

impl TDFB for TGGetBasicGroup {}

impl AsRef<TGGetBasicGroup> for TGGetBasicGroup {
  fn as_ref(&self) -> &TGGetBasicGroup { self }
}

impl TGGetBasicGroup {

  pub fn new() -> Self {
    Self {
      basic_group_id: None,
      
    }
  }

  
  pub fn basic_group_id(&mut self, basic_group_id: i32) -> &mut Self { self.basic_group_id = Some(basic_group_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetBasicGroup {
    GetBasicGroup::builder()
      .basic_group_id(self.basic_group_id.clone())
      
      .build()
  }
}


///  Returns full information about a basic group by its identifier. 
#[derive(Debug, Clone)]
pub struct TGGetBasicGroupFullInfo {
  ///  Basic group identifier. 
  basic_group_id: Option<i32>,
  
}

impl TDFB for TGGetBasicGroupFullInfo {}

impl AsRef<TGGetBasicGroupFullInfo> for TGGetBasicGroupFullInfo {
  fn as_ref(&self) -> &TGGetBasicGroupFullInfo { self }
}

impl TGGetBasicGroupFullInfo {

  pub fn new() -> Self {
    Self {
      basic_group_id: None,
      
    }
  }

  
  pub fn basic_group_id(&mut self, basic_group_id: i32) -> &mut Self { self.basic_group_id = Some(basic_group_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetBasicGroupFullInfo {
    GetBasicGroupFullInfo::builder()
      .basic_group_id(self.basic_group_id.clone())
      
      .build()
  }
}


///  Returns users that were blocked by the current user. 
#[derive(Debug, Clone)]
pub struct TGGetBlockedUsers {
  ///  Number of users to skip in the result; must be non-negative. 
  offset: Option<i32>,
  ///  Maximum number of users to return; up to 100. 
  limit: Option<i32>,
  
}

impl TDFB for TGGetBlockedUsers {}

impl AsRef<TGGetBlockedUsers> for TGGetBlockedUsers {
  fn as_ref(&self) -> &TGGetBlockedUsers { self }
}

impl TGGetBlockedUsers {

  pub fn new() -> Self {
    Self {
      offset: None,
      limit: None,
      
    }
  }

  
  pub fn offset(&mut self, offset: i32) -> &mut Self { self.offset = Some(offset); self }
  
  pub fn limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetBlockedUsers {
    GetBlockedUsers::builder()
      .offset(self.offset.clone())
      .limit(self.limit.clone())
      
      .build()
  }
}


///  Sends a callback query to a bot and returns an answer. Returns an error with code 502 if the bot fails to answer the query before the query timeout expires. 
#[derive(Debug, Clone)]
pub struct TGGetCallbackQueryAnswer {
  ///  Identifier of the chat with the message. 
  chat_id: Option<i64>,
  ///  Identifier of the message from which the query originated. 
  message_id: Option<i64>,
  ///  Query payload. 
  payload: Option<Box<CallbackQueryPayload>>,
  
}

impl TDFB for TGGetCallbackQueryAnswer {}

impl AsRef<TGGetCallbackQueryAnswer> for TGGetCallbackQueryAnswer {
  fn as_ref(&self) -> &TGGetCallbackQueryAnswer { self }
}

impl TGGetCallbackQueryAnswer {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      message_id: None,
      payload: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  


  
  // [payload] type is [Box<CallbackQueryPayload>], is not support, need add manully.
  #[doc(hidden)] pub fn _payload(&mut self, payload: Box<CallbackQueryPayload>) -> &mut Self { self.payload = Some(payload); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> GetCallbackQueryAnswer {
    GetCallbackQueryAnswer::builder()
      .chat_id(self.chat_id.clone())
      .message_id(self.message_id.clone())
      .payload(self.payload.clone())
      
      .build()
  }
}


///  Returns information about a chat by its identifier, this is an offline request if the current user is not a bot. 
#[derive(Debug, Clone)]
pub struct TGGetChat {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  
}

impl TDFB for TGGetChat {}

impl AsRef<TGGetChat> for TGGetChat {
  fn as_ref(&self) -> &TGGetChat { self }
}

impl TGGetChat {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetChat {
    GetChat::builder()
      .chat_id(self.chat_id.clone())
      
      .build()
  }
}


///  Returns a list of users who are administrators of the chat. 
#[derive(Debug, Clone)]
pub struct TGGetChatAdministrators {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  
}

impl TDFB for TGGetChatAdministrators {}

impl AsRef<TGGetChatAdministrators> for TGGetChatAdministrators {
  fn as_ref(&self) -> &TGGetChatAdministrators { self }
}

impl TGGetChatAdministrators {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetChatAdministrators {
    GetChatAdministrators::builder()
      .chat_id(self.chat_id.clone())
      
      .build()
  }
}


///  Returns a list of service actions taken by chat members and administrators in the last 48 hours. Available only in supergroups and channels. Requires administrator rights. Returns results in reverse chronological order (i. e., in order of decreasing event_id). 
#[derive(Debug, Clone)]
pub struct TGGetChatEventLog {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  ///  Search query by which to filter events. 
  query: Option<String>,
  ///  Identifier of an event from which to return results. Use 0 to get results from the latest events. 
  from_event_id: Option<i64>,
  ///  Maximum number of events to return; up to 100. 
  limit: Option<i32>,
  ///  The types of events to return. By default, all types will be returned. 
  filters: Option<ChatEventLogFilters>,
  ///  User identifiers by which to filter events. By default, events relating to all users will be returned. 
  user_ids: Option<Vec<i32>>,
  
}

impl TDFB for TGGetChatEventLog {}

impl AsRef<TGGetChatEventLog> for TGGetChatEventLog {
  fn as_ref(&self) -> &TGGetChatEventLog { self }
}

impl TGGetChatEventLog {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      query: None,
      from_event_id: None,
      limit: None,
      filters: None,
      user_ids: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn query<S: AsRef<str>>(&mut self, query: S) -> &mut Self { self.query = Some(query.as_ref().to_string()); self }
  
  pub fn from_event_id(&mut self, from_event_id: i64) -> &mut Self { self.from_event_id = Some(from_event_id); self }
  
  pub fn limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  
  pub fn user_ids(&mut self, user_ids: Vec<i32>) -> &mut Self { self.user_ids = Some(user_ids); self }
  


  
  // [filters] type is [ChatEventLogFilters], is not support, need add manully.
  #[doc(hidden)] pub fn _filters(&mut self, filters: ChatEventLogFilters) -> &mut Self { self.filters = Some(filters); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> GetChatEventLog {
    GetChatEventLog::builder()
      .chat_id(self.chat_id.clone())
      .query(self.query.clone())
      .from_event_id(self.from_event_id.clone())
      .limit(self.limit.clone())
      .filters(self.filters.clone())
      .user_ids(self.user_ids.clone())
      
      .build()
  }
}


///  Returns messages in a chat. The messages are returned in a reverse chronological order (i.e., in order of decreasing message_id). For optimal performance the number of returned messages is chosen by the library. This is an offline request if only_local is true. 
#[derive(Debug, Clone)]
pub struct TGGetChatHistory {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  ///  Identifier of the message starting from which history must be fetched; use 0 to get results from the last message. 
  from_message_id: Option<i64>,
  ///  Specify 0 to get results from exactly the from_message_id or a negative offset up to 99 to get additionally some newer messages. 
  offset: Option<i32>,
  ///  The maximum number of messages to be returned; must be positive and can't be greater than 100. If the offset is negative, the limit must be greater or equal to -offset. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached. 
  limit: Option<i32>,
  ///  If true, returns only messages that are available locally without sending network requests. 
  only_local: Option<bool>,
  
}

impl TDFB for TGGetChatHistory {}

impl AsRef<TGGetChatHistory> for TGGetChatHistory {
  fn as_ref(&self) -> &TGGetChatHistory { self }
}

impl TGGetChatHistory {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      from_message_id: None,
      offset: None,
      limit: None,
      only_local: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn from_message_id(&mut self, from_message_id: i64) -> &mut Self { self.from_message_id = Some(from_message_id); self }
  
  pub fn offset(&mut self, offset: i32) -> &mut Self { self.offset = Some(offset); self }
  
  pub fn limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  
  pub fn only_local(&mut self, only_local: bool) -> &mut Self { self.only_local = Some(only_local); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetChatHistory {
    GetChatHistory::builder()
      .chat_id(self.chat_id.clone())
      .from_message_id(self.from_message_id.clone())
      .offset(self.offset.clone())
      .limit(self.limit.clone())
      .only_local(self.only_local.clone())
      
      .build()
  }
}


///  Returns information about a single member of a chat. 
#[derive(Debug, Clone)]
pub struct TGGetChatMember {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  ///  User identifier. 
  user_id: Option<i32>,
  
}

impl TDFB for TGGetChatMember {}

impl AsRef<TGGetChatMember> for TGGetChatMember {
  fn as_ref(&self) -> &TGGetChatMember { self }
}

impl TGGetChatMember {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      user_id: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetChatMember {
    GetChatMember::builder()
      .chat_id(self.chat_id.clone())
      .user_id(self.user_id.clone())
      
      .build()
  }
}


///  Returns the last message sent in a chat no later than the specified date. 
#[derive(Debug, Clone)]
pub struct TGGetChatMessageByDate {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  ///  Point in time (Unix timestamp) relative to which to search for messages. 
  date: Option<i32>,
  
}

impl TDFB for TGGetChatMessageByDate {}

impl AsRef<TGGetChatMessageByDate> for TGGetChatMessageByDate {
  fn as_ref(&self) -> &TGGetChatMessageByDate { self }
}

impl TGGetChatMessageByDate {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      date: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn date(&mut self, date: i32) -> &mut Self { self.date = Some(date); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetChatMessageByDate {
    GetChatMessageByDate::builder()
      .chat_id(self.chat_id.clone())
      .date(self.date.clone())
      
      .build()
  }
}


///  Returns approximate number of messages of the specified type in the chat. 
#[derive(Debug, Clone)]
pub struct TGGetChatMessageCount {
  ///  Identifier of the chat in which to count messages. 
  chat_id: Option<i64>,
  ///  Filter for message content; searchMessagesFilterEmpty is unsupported in this function. 
  filter: Option<Box<SearchMessagesFilter>>,
  ///  If true, returns count that is available locally without sending network requests, returning -1 if the number of messages is unknown. 
  return_local: Option<bool>,
  
}

impl TDFB for TGGetChatMessageCount {}

impl AsRef<TGGetChatMessageCount> for TGGetChatMessageCount {
  fn as_ref(&self) -> &TGGetChatMessageCount { self }
}

impl TGGetChatMessageCount {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      filter: None,
      return_local: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn return_local(&mut self, return_local: bool) -> &mut Self { self.return_local = Some(return_local); self }
  


  
  // [filter] type is [Box<SearchMessagesFilter>], is not support, need add manully.
  #[doc(hidden)] pub fn _filter(&mut self, filter: Box<SearchMessagesFilter>) -> &mut Self { self.filter = Some(filter); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> GetChatMessageCount {
    GetChatMessageCount::builder()
      .chat_id(self.chat_id.clone())
      .filter(self.filter.clone())
      .return_local(self.return_local.clone())
      
      .build()
  }
}


///  Returns list of chats with non-default notification settings. 
#[derive(Debug, Clone)]
pub struct TGGetChatNotificationSettingsExceptions {
  ///  If specified, only chats from the specified scope will be returned. 
  scope: Option<Box<NotificationSettingsScope>>,
  ///  If true, also chats with non-default sound will be returned. 
  compare_sound: Option<bool>,
  
}

impl TDFB for TGGetChatNotificationSettingsExceptions {}

impl AsRef<TGGetChatNotificationSettingsExceptions> for TGGetChatNotificationSettingsExceptions {
  fn as_ref(&self) -> &TGGetChatNotificationSettingsExceptions { self }
}

impl TGGetChatNotificationSettingsExceptions {

  pub fn new() -> Self {
    Self {
      scope: None,
      compare_sound: None,
      
    }
  }

  
  pub fn compare_sound(&mut self, compare_sound: bool) -> &mut Self { self.compare_sound = Some(compare_sound); self }
  


  
  // [scope] type is [Box<NotificationSettingsScope>], is not support, need add manully.
  #[doc(hidden)] pub fn _scope(&mut self, scope: Box<NotificationSettingsScope>) -> &mut Self { self.scope = Some(scope); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> GetChatNotificationSettingsExceptions {
    GetChatNotificationSettingsExceptions::builder()
      .scope(self.scope.clone())
      .compare_sound(self.compare_sound.clone())
      
      .build()
  }
}


///  Returns information about a pinned chat message. 
#[derive(Debug, Clone)]
pub struct TGGetChatPinnedMessage {
  ///  Identifier of the chat the message belongs to. 
  chat_id: Option<i64>,
  
}

impl TDFB for TGGetChatPinnedMessage {}

impl AsRef<TGGetChatPinnedMessage> for TGGetChatPinnedMessage {
  fn as_ref(&self) -> &TGGetChatPinnedMessage { self }
}

impl TGGetChatPinnedMessage {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetChatPinnedMessage {
    GetChatPinnedMessage::builder()
      .chat_id(self.chat_id.clone())
      
      .build()
  }
}


///  Returns information on whether the current chat can be reported as spam. 
#[derive(Debug, Clone)]
pub struct TGGetChatReportSpamState {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  
}

impl TDFB for TGGetChatReportSpamState {}

impl AsRef<TGGetChatReportSpamState> for TGGetChatReportSpamState {
  fn as_ref(&self) -> &TGGetChatReportSpamState { self }
}

impl TGGetChatReportSpamState {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetChatReportSpamState {
    GetChatReportSpamState::builder()
      .chat_id(self.chat_id.clone())
      
      .build()
  }
}


///  Returns an ordered list of chats. Chats are sorted by the pair (order, chat_id) in decreasing order. (For example, to get a list of chats from the beginning, the offset_order should be equal to a biggest signed 64-bit number 9223372036854775807 == 2^63 - 1). For optimal performance the number of returned chats is chosen by the library. 
#[derive(Debug, Clone)]
pub struct TGGetChats {
  ///  Chat order to return chats from. 
  offset_order: Option<i64>,
  ///  Chat identifier to return chats from. 
  offset_chat_id: Option<i64>,
  ///  The maximum number of chats to be returned. It is possible that fewer chats than the limit are returned even if the end of the list is not reached. 
  limit: Option<i32>,
  
}

impl TDFB for TGGetChats {}

impl AsRef<TGGetChats> for TGGetChats {
  fn as_ref(&self) -> &TGGetChats { self }
}

impl TGGetChats {

  pub fn new() -> Self {
    Self {
      offset_order: None,
      offset_chat_id: None,
      limit: None,
      
    }
  }

  
  pub fn offset_order(&mut self, offset_order: i64) -> &mut Self { self.offset_order = Some(offset_order); self }
  
  pub fn offset_chat_id(&mut self, offset_chat_id: i64) -> &mut Self { self.offset_chat_id = Some(offset_chat_id); self }
  
  pub fn limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetChats {
    GetChats::builder()
      .offset_order(self.offset_order.clone())
      .offset_chat_id(self.offset_chat_id.clone())
      .limit(self.limit.clone())
      
      .build()
  }
}


///  Returns URL with the chat statistics. Currently this method can be used only for channels. 
#[derive(Debug, Clone)]
pub struct TGGetChatStatisticsUrl {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  ///  Parameters from "tg://statsrefresh?params=******" link. 
  parameters: Option<String>,
  ///  Pass true if a URL with the dark theme must be returned. 
  is_dark: Option<bool>,
  
}

impl TDFB for TGGetChatStatisticsUrl {}

impl AsRef<TGGetChatStatisticsUrl> for TGGetChatStatisticsUrl {
  fn as_ref(&self) -> &TGGetChatStatisticsUrl { self }
}

impl TGGetChatStatisticsUrl {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      parameters: None,
      is_dark: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn parameters<S: AsRef<str>>(&mut self, parameters: S) -> &mut Self { self.parameters = Some(parameters.as_ref().to_string()); self }
  
  pub fn is_dark(&mut self, is_dark: bool) -> &mut Self { self.is_dark = Some(is_dark); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetChatStatisticsUrl {
    GetChatStatisticsUrl::builder()
      .chat_id(self.chat_id.clone())
      .parameters(self.parameters.clone())
      .is_dark(self.is_dark.clone())
      
      .build()
  }
}


///  Returns all website where the current user used Telegram to log in. 
#[derive(Debug, Clone)]
pub struct TGGetConnectedWebsites {
  
}

impl TDFB for TGGetConnectedWebsites {}

impl AsRef<TGGetConnectedWebsites> for TGGetConnectedWebsites {
  fn as_ref(&self) -> &TGGetConnectedWebsites { self }
}

impl TGGetConnectedWebsites {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetConnectedWebsites {
    GetConnectedWebsites::builder()
      
      .build()
  }
}


///  Returns all user contacts. 
#[derive(Debug, Clone)]
pub struct TGGetContacts {
  
}

impl TDFB for TGGetContacts {}

impl AsRef<TGGetContacts> for TGGetContacts {
  fn as_ref(&self) -> &TGGetContacts { self }
}

impl TGGetContacts {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetContacts {
    GetContacts::builder()
      
      .build()
  }
}


///  Uses current user IP to found his country. Returns two-letter ISO 3166-1 alpha-2 country code. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGGetCountryCode {
  
}

impl TDFB for TGGetCountryCode {}

impl AsRef<TGGetCountryCode> for TGGetCountryCode {
  fn as_ref(&self) -> &TGGetCountryCode { self }
}

impl TGGetCountryCode {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetCountryCode {
    GetCountryCode::builder()
      
      .build()
  }
}


///  Returns a list of public chats created by the user. 
#[derive(Debug, Clone)]
pub struct TGGetCreatedPublicChats {
  
}

impl TDFB for TGGetCreatedPublicChats {}

impl AsRef<TGGetCreatedPublicChats> for TGGetCreatedPublicChats {
  fn as_ref(&self) -> &TGGetCreatedPublicChats { self }
}

impl TGGetCreatedPublicChats {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetCreatedPublicChats {
    GetCreatedPublicChats::builder()
      
      .build()
  }
}


///  Returns all updates needed to restore current TDLib state, i.e. all actual UpdateAuthorizationState/UpdateUser/UpdateNewChat and others. This is especially usefull if TDLib is run in a separate process. This is an offline method. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGGetCurrentState {
  
}

impl TDFB for TGGetCurrentState {}

impl AsRef<TGGetCurrentState> for TGGetCurrentState {
  fn as_ref(&self) -> &TGGetCurrentState { self }
}

impl TGGetCurrentState {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetCurrentState {
    GetCurrentState::builder()
      
      .build()
  }
}


///  Returns database statistics. 
#[derive(Debug, Clone)]
pub struct TGGetDatabaseStatistics {
  
}

impl TDFB for TGGetDatabaseStatistics {}

impl AsRef<TGGetDatabaseStatistics> for TGGetDatabaseStatistics {
  fn as_ref(&self) -> &TGGetDatabaseStatistics { self }
}

impl TGGetDatabaseStatistics {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetDatabaseStatistics {
    GetDatabaseStatistics::builder()
      
      .build()
  }
}


///  Returns information about a tg:// deep link. Use "tg://need_update_for_some_feature" or "tg:some_unsupported_feature" for testing. Returns a 404 error for unknown links. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGGetDeepLinkInfo {
  ///  The link. 
  link: Option<String>,
  
}

impl TDFB for TGGetDeepLinkInfo {}

impl AsRef<TGGetDeepLinkInfo> for TGGetDeepLinkInfo {
  fn as_ref(&self) -> &TGGetDeepLinkInfo { self }
}

impl TGGetDeepLinkInfo {

  pub fn new() -> Self {
    Self {
      link: None,
      
    }
  }

  
  pub fn link<S: AsRef<str>>(&mut self, link: S) -> &mut Self { self.link = Some(link.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetDeepLinkInfo {
    GetDeepLinkInfo::builder()
      .link(self.link.clone())
      
      .build()
  }
}


///  Returns favorite stickers. 
#[derive(Debug, Clone)]
pub struct TGGetFavoriteStickers {
  
}

impl TDFB for TGGetFavoriteStickers {}

impl AsRef<TGGetFavoriteStickers> for TGGetFavoriteStickers {
  fn as_ref(&self) -> &TGGetFavoriteStickers { self }
}

impl TGGetFavoriteStickers {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetFavoriteStickers {
    GetFavoriteStickers::builder()
      
      .build()
  }
}


///  Returns information about a file; this is an offline request. 
#[derive(Debug, Clone)]
pub struct TGGetFile {
  ///  Identifier of the file to get. 
  file_id: Option<i32>,
  
}

impl TDFB for TGGetFile {}

impl AsRef<TGGetFile> for TGGetFile {
  fn as_ref(&self) -> &TGGetFile { self }
}

impl TGGetFile {

  pub fn new() -> Self {
    Self {
      file_id: None,
      
    }
  }

  
  pub fn file_id(&mut self, file_id: i32) -> &mut Self { self.file_id = Some(file_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetFile {
    GetFile::builder()
      .file_id(self.file_id.clone())
      
      .build()
  }
}


///  Returns file downloaded prefix size from a given offset. 
#[derive(Debug, Clone)]
pub struct TGGetFileDownloadedPrefixSize {
  ///  Identifier of the file. 
  file_id: Option<i32>,
  ///  Offset from which downloaded prefix size should be calculated. 
  offset: Option<i32>,
  
}

impl TDFB for TGGetFileDownloadedPrefixSize {}

impl AsRef<TGGetFileDownloadedPrefixSize> for TGGetFileDownloadedPrefixSize {
  fn as_ref(&self) -> &TGGetFileDownloadedPrefixSize { self }
}

impl TGGetFileDownloadedPrefixSize {

  pub fn new() -> Self {
    Self {
      file_id: None,
      offset: None,
      
    }
  }

  
  pub fn file_id(&mut self, file_id: i32) -> &mut Self { self.file_id = Some(file_id); self }
  
  pub fn offset(&mut self, offset: i32) -> &mut Self { self.offset = Some(offset); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetFileDownloadedPrefixSize {
    GetFileDownloadedPrefixSize::builder()
      .file_id(self.file_id.clone())
      .offset(self.offset.clone())
      
      .build()
  }
}


///  Returns the extension of a file, guessed by its MIME type. Returns an empty string on failure. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGGetFileExtension {
  ///  The MIME type of the file. 
  mime_type: Option<String>,
  
}

impl TDFB for TGGetFileExtension {}

impl AsRef<TGGetFileExtension> for TGGetFileExtension {
  fn as_ref(&self) -> &TGGetFileExtension { self }
}

impl TGGetFileExtension {

  pub fn new() -> Self {
    Self {
      mime_type: None,
      
    }
  }

  
  pub fn mime_type<S: AsRef<str>>(&mut self, mime_type: S) -> &mut Self { self.mime_type = Some(mime_type.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetFileExtension {
    GetFileExtension::builder()
      .mime_type(self.mime_type.clone())
      
      .build()
  }
}


///  Returns the MIME type of a file, guessed by its extension. Returns an empty string on failure. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGGetFileMimeType {
  ///  The name of the file or path to the file. 
  file_name: Option<String>,
  
}

impl TDFB for TGGetFileMimeType {}

impl AsRef<TGGetFileMimeType> for TGGetFileMimeType {
  fn as_ref(&self) -> &TGGetFileMimeType { self }
}

impl TGGetFileMimeType {

  pub fn new() -> Self {
    Self {
      file_name: None,
      
    }
  }

  
  pub fn file_name<S: AsRef<str>>(&mut self, file_name: S) -> &mut Self { self.file_name = Some(file_name.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetFileMimeType {
    GetFileMimeType::builder()
      .file_name(self.file_name.clone())
      
      .build()
  }
}


///  Returns the high scores for a game and some part of the high score table in the range of the specified user; for bots only. 
#[derive(Debug, Clone)]
pub struct TGGetGameHighScores {
  ///  The chat that contains the message with the game. 
  chat_id: Option<i64>,
  ///  Identifier of the message. 
  message_id: Option<i64>,
  ///  User identifier. 
  user_id: Option<i32>,
  
}

impl TDFB for TGGetGameHighScores {}

impl AsRef<TGGetGameHighScores> for TGGetGameHighScores {
  fn as_ref(&self) -> &TGGetGameHighScores { self }
}

impl TGGetGameHighScores {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      message_id: None,
      user_id: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetGameHighScores {
    GetGameHighScores::builder()
      .chat_id(self.chat_id.clone())
      .message_id(self.message_id.clone())
      .user_id(self.user_id.clone())
      
      .build()
  }
}


///  Returns a list of common group chats with a given user. Chats are sorted by their type and creation date. 
#[derive(Debug, Clone)]
pub struct TGGetGroupsInCommon {
  ///  User identifier. 
  user_id: Option<i32>,
  ///  Chat identifier starting from which to return chats; use 0 for the first request. 
  offset_chat_id: Option<i64>,
  ///  Maximum number of chats to be returned; up to 100. 
  limit: Option<i32>,
  
}

impl TDFB for TGGetGroupsInCommon {}

impl AsRef<TGGetGroupsInCommon> for TGGetGroupsInCommon {
  fn as_ref(&self) -> &TGGetGroupsInCommon { self }
}

impl TGGetGroupsInCommon {

  pub fn new() -> Self {
    Self {
      user_id: None,
      offset_chat_id: None,
      limit: None,
      
    }
  }

  
  pub fn user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn offset_chat_id(&mut self, offset_chat_id: i64) -> &mut Self { self.offset_chat_id = Some(offset_chat_id); self }
  
  pub fn limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetGroupsInCommon {
    GetGroupsInCommon::builder()
      .user_id(self.user_id.clone())
      .offset_chat_id(self.offset_chat_id.clone())
      .limit(self.limit.clone())
      
      .build()
  }
}


///  Returns the total number of imported contacts. 
#[derive(Debug, Clone)]
pub struct TGGetImportedContactCount {
  
}

impl TDFB for TGGetImportedContactCount {}

impl AsRef<TGGetImportedContactCount> for TGGetImportedContactCount {
  fn as_ref(&self) -> &TGGetImportedContactCount { self }
}

impl TGGetImportedContactCount {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetImportedContactCount {
    GetImportedContactCount::builder()
      
      .build()
  }
}


///  Returns game high scores and some part of the high score table in the range of the specified user; for bots only. 
#[derive(Debug, Clone)]
pub struct TGGetInlineGameHighScores {
  ///  Inline message identifier. 
  inline_message_id: Option<String>,
  ///  User identifier. 
  user_id: Option<i32>,
  
}

impl TDFB for TGGetInlineGameHighScores {}

impl AsRef<TGGetInlineGameHighScores> for TGGetInlineGameHighScores {
  fn as_ref(&self) -> &TGGetInlineGameHighScores { self }
}

impl TGGetInlineGameHighScores {

  pub fn new() -> Self {
    Self {
      inline_message_id: None,
      user_id: None,
      
    }
  }

  
  pub fn inline_message_id<S: AsRef<str>>(&mut self, inline_message_id: S) -> &mut Self { self.inline_message_id = Some(inline_message_id.as_ref().to_string()); self }
  
  pub fn user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetInlineGameHighScores {
    GetInlineGameHighScores::builder()
      .inline_message_id(self.inline_message_id.clone())
      .user_id(self.user_id.clone())
      
      .build()
  }
}


///  Sends an inline query to a bot and returns its results. Returns an error with code 502 if the bot fails to answer the query before the query timeout expires. 
#[derive(Debug, Clone)]
pub struct TGGetInlineQueryResults {
  ///  The identifier of the target bot. 
  bot_user_id: Option<i32>,
  ///  Identifier of the chat, where the query was sent. 
  chat_id: Option<i64>,
  ///  Location of the user, only if needed. 
  user_location: Option<Location>,
  ///  Text of the query. 
  query: Option<String>,
  ///  Offset of the first entry to return. 
  offset: Option<String>,
  
}

impl TDFB for TGGetInlineQueryResults {}

impl AsRef<TGGetInlineQueryResults> for TGGetInlineQueryResults {
  fn as_ref(&self) -> &TGGetInlineQueryResults { self }
}

impl TGGetInlineQueryResults {

  pub fn new() -> Self {
    Self {
      bot_user_id: None,
      chat_id: None,
      user_location: None,
      query: None,
      offset: None,
      
    }
  }

  
  pub fn bot_user_id(&mut self, bot_user_id: i32) -> &mut Self { self.bot_user_id = Some(bot_user_id); self }
  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn query<S: AsRef<str>>(&mut self, query: S) -> &mut Self { self.query = Some(query.as_ref().to_string()); self }
  
  pub fn offset<S: AsRef<str>>(&mut self, offset: S) -> &mut Self { self.offset = Some(offset.as_ref().to_string()); self }
  


  
  // [user_location] type is [Location], is not support, need add manully.
  #[doc(hidden)] pub fn _user_location(&mut self, user_location: Location) -> &mut Self { self.user_location = Some(user_location); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> GetInlineQueryResults {
    GetInlineQueryResults::builder()
      .bot_user_id(self.bot_user_id.clone())
      .chat_id(self.chat_id.clone())
      .user_location(self.user_location.clone())
      .query(self.query.clone())
      .offset(self.offset.clone())
      
      .build()
  }
}


///  Returns a list of installed sticker sets. 
#[derive(Debug, Clone)]
pub struct TGGetInstalledStickerSets {
  ///  Pass true to return mask sticker sets; pass false to return ordinary sticker sets. 
  is_masks: Option<bool>,
  
}

impl TDFB for TGGetInstalledStickerSets {}

impl AsRef<TGGetInstalledStickerSets> for TGGetInstalledStickerSets {
  fn as_ref(&self) -> &TGGetInstalledStickerSets { self }
}

impl TGGetInstalledStickerSets {

  pub fn new() -> Self {
    Self {
      is_masks: None,
      
    }
  }

  
  pub fn is_masks(&mut self, is_masks: bool) -> &mut Self { self.is_masks = Some(is_masks); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetInstalledStickerSets {
    GetInstalledStickerSets::builder()
      .is_masks(self.is_masks.clone())
      
      .build()
  }
}


///  Returns the default text for invitation messages to be used as a placeholder when the current user invites friends to Telegram. 
#[derive(Debug, Clone)]
pub struct TGGetInviteText {
  
}

impl TDFB for TGGetInviteText {}

impl AsRef<TGGetInviteText> for TGGetInviteText {
  fn as_ref(&self) -> &TGGetInviteText { self }
}

impl TGGetInviteText {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetInviteText {
    GetInviteText::builder()
      
      .build()
  }
}


///  Returns information about a language pack. Returned language pack identifier may be different from a provided one. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGGetLanguagePackInfo {
  ///  Language pack identifier. 
  language_pack_id: Option<String>,
  
}

impl TDFB for TGGetLanguagePackInfo {}

impl AsRef<TGGetLanguagePackInfo> for TGGetLanguagePackInfo {
  fn as_ref(&self) -> &TGGetLanguagePackInfo { self }
}

impl TGGetLanguagePackInfo {

  pub fn new() -> Self {
    Self {
      language_pack_id: None,
      
    }
  }

  
  pub fn language_pack_id<S: AsRef<str>>(&mut self, language_pack_id: S) -> &mut Self { self.language_pack_id = Some(language_pack_id.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetLanguagePackInfo {
    GetLanguagePackInfo::builder()
      .language_pack_id(self.language_pack_id.clone())
      
      .build()
  }
}


///  Returns a string stored in the local database from the specified localization target and language pack by its key. Returns a 404 error if the string is not found. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGGetLanguagePackString {
  ///  Path to the language pack database in which strings are stored. 
  language_pack_database_path: Option<String>,
  ///  Localization target to which the language pack belongs. 
  localization_target: Option<String>,
  ///  Language pack identifier. 
  language_pack_id: Option<String>,
  ///  Language pack key of the string to be returned. 
  key: Option<String>,
  
}

impl TDFB for TGGetLanguagePackString {}

impl AsRef<TGGetLanguagePackString> for TGGetLanguagePackString {
  fn as_ref(&self) -> &TGGetLanguagePackString { self }
}

impl TGGetLanguagePackString {

  pub fn new() -> Self {
    Self {
      language_pack_database_path: None,
      localization_target: None,
      language_pack_id: None,
      key: None,
      
    }
  }

  
  pub fn language_pack_database_path<S: AsRef<str>>(&mut self, language_pack_database_path: S) -> &mut Self { self.language_pack_database_path = Some(language_pack_database_path.as_ref().to_string()); self }
  
  pub fn localization_target<S: AsRef<str>>(&mut self, localization_target: S) -> &mut Self { self.localization_target = Some(localization_target.as_ref().to_string()); self }
  
  pub fn language_pack_id<S: AsRef<str>>(&mut self, language_pack_id: S) -> &mut Self { self.language_pack_id = Some(language_pack_id.as_ref().to_string()); self }
  
  pub fn key<S: AsRef<str>>(&mut self, key: S) -> &mut Self { self.key = Some(key.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetLanguagePackString {
    GetLanguagePackString::builder()
      .language_pack_database_path(self.language_pack_database_path.clone())
      .localization_target(self.localization_target.clone())
      .language_pack_id(self.language_pack_id.clone())
      .key(self.key.clone())
      
      .build()
  }
}


///  Returns strings from a language pack in the current localization target by their keys. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGGetLanguagePackStrings {
  ///  Language pack identifier of the strings to be returned. 
  language_pack_id: Option<String>,
  ///  Language pack keys of the strings to be returned; leave empty to request all available strings. 
  keys: Option<Vec<String>>,
  
}

impl TDFB for TGGetLanguagePackStrings {}

impl AsRef<TGGetLanguagePackStrings> for TGGetLanguagePackStrings {
  fn as_ref(&self) -> &TGGetLanguagePackStrings { self }
}

impl TGGetLanguagePackStrings {

  pub fn new() -> Self {
    Self {
      language_pack_id: None,
      keys: None,
      
    }
  }

  
  pub fn language_pack_id<S: AsRef<str>>(&mut self, language_pack_id: S) -> &mut Self { self.language_pack_id = Some(language_pack_id.as_ref().to_string()); self }
  


  
  // [keys] type is [Vec<String>], is not support, need add manully.
  #[doc(hidden)] pub fn _keys(&mut self, keys: Vec<String>) -> &mut Self { self.keys = Some(keys); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> GetLanguagePackStrings {
    GetLanguagePackStrings::builder()
      .language_pack_id(self.language_pack_id.clone())
      .keys(self.keys.clone())
      
      .build()
  }
}


///  Returns information about the current localization target. This is an offline request if only_local is true. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGGetLocalizationTargetInfo {
  ///  If true, returns only locally available information without sending network requests. 
  only_local: Option<bool>,
  
}

impl TDFB for TGGetLocalizationTargetInfo {}

impl AsRef<TGGetLocalizationTargetInfo> for TGGetLocalizationTargetInfo {
  fn as_ref(&self) -> &TGGetLocalizationTargetInfo { self }
}

impl TGGetLocalizationTargetInfo {

  pub fn new() -> Self {
    Self {
      only_local: None,
      
    }
  }

  
  pub fn only_local(&mut self, only_local: bool) -> &mut Self { self.only_local = Some(only_local); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetLocalizationTargetInfo {
    GetLocalizationTargetInfo::builder()
      .only_local(self.only_local.clone())
      
      .build()
  }
}


///  Returns information about currently used log stream for internal logging of TDLib. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGGetLogStream {
  
}

impl TDFB for TGGetLogStream {}

impl AsRef<TGGetLogStream> for TGGetLogStream {
  fn as_ref(&self) -> &TGGetLogStream { self }
}

impl TGGetLogStream {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetLogStream {
    GetLogStream::builder()
      
      .build()
  }
}


///  Returns list of available TDLib internal log tags, for example, ["actor", "binlog", "connections", "notifications", "proxy"]. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGGetLogTags {
  
}

impl TDFB for TGGetLogTags {}

impl AsRef<TGGetLogTags> for TGGetLogTags {
  fn as_ref(&self) -> &TGGetLogTags { self }
}

impl TGGetLogTags {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetLogTags {
    GetLogTags::builder()
      
      .build()
  }
}


///  Returns current verbosity level for a specified TDLib internal log tag. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGGetLogTagVerbosityLevel {
  ///  Logging tag to change verbosity level. 
  tag: Option<String>,
  
}

impl TDFB for TGGetLogTagVerbosityLevel {}

impl AsRef<TGGetLogTagVerbosityLevel> for TGGetLogTagVerbosityLevel {
  fn as_ref(&self) -> &TGGetLogTagVerbosityLevel { self }
}

impl TGGetLogTagVerbosityLevel {

  pub fn new() -> Self {
    Self {
      tag: None,
      
    }
  }

  
  pub fn tag<S: AsRef<str>>(&mut self, tag: S) -> &mut Self { self.tag = Some(tag.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetLogTagVerbosityLevel {
    GetLogTagVerbosityLevel::builder()
      .tag(self.tag.clone())
      
      .build()
  }
}


///  Returns current verbosity level of the internal logging of TDLib. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGGetLogVerbosityLevel {
  
}

impl TDFB for TGGetLogVerbosityLevel {}

impl AsRef<TGGetLogVerbosityLevel> for TGGetLogVerbosityLevel {
  fn as_ref(&self) -> &TGGetLogVerbosityLevel { self }
}

impl TGGetLogVerbosityLevel {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetLogVerbosityLevel {
    GetLogVerbosityLevel::builder()
      
      .build()
  }
}


///  Returns information about a file with a map thumbnail in PNG format. Only map thumbnail files with size less than 1MB can be downloaded. 
#[derive(Debug, Clone)]
pub struct TGGetMapThumbnailFile {
  ///  Location of the map center. 
  location: Option<Location>,
  ///  Map zoom level; 13-20. 
  zoom: Option<i32>,
  ///  Map width in pixels before applying scale; 16-1024. 
  width: Option<i32>,
  ///  Map height in pixels before applying scale; 16-1024. 
  height: Option<i32>,
  ///  Map scale; 1-3. 
  scale: Option<i32>,
  ///  Identifier of a chat, in which the thumbnail will be shown. Use 0 if unknown. 
  chat_id: Option<i64>,
  
}

impl TDFB for TGGetMapThumbnailFile {}

impl AsRef<TGGetMapThumbnailFile> for TGGetMapThumbnailFile {
  fn as_ref(&self) -> &TGGetMapThumbnailFile { self }
}

impl TGGetMapThumbnailFile {

  pub fn new() -> Self {
    Self {
      location: None,
      zoom: None,
      width: None,
      height: None,
      scale: None,
      chat_id: None,
      
    }
  }

  
  pub fn zoom(&mut self, zoom: i32) -> &mut Self { self.zoom = Some(zoom); self }
  
  pub fn width(&mut self, width: i32) -> &mut Self { self.width = Some(width); self }
  
  pub fn height(&mut self, height: i32) -> &mut Self { self.height = Some(height); self }
  
  pub fn scale(&mut self, scale: i32) -> &mut Self { self.scale = Some(scale); self }
  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  


  
  // [location] type is [Location], is not support, need add manully.
  #[doc(hidden)] pub fn _location(&mut self, location: Location) -> &mut Self { self.location = Some(location); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> GetMapThumbnailFile {
    GetMapThumbnailFile::builder()
      .location(self.location.clone())
      .zoom(self.zoom.clone())
      .width(self.width.clone())
      .height(self.height.clone())
      .scale(self.scale.clone())
      .chat_id(self.chat_id.clone())
      
      .build()
  }
}


///  Returns the current user. 
#[derive(Debug, Clone)]
pub struct TGGetMe {
  
}

impl TDFB for TGGetMe {}

impl AsRef<TGGetMe> for TGGetMe {
  fn as_ref(&self) -> &TGGetMe { self }
}

impl TGGetMe {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetMe {
    GetMe::builder()
      
      .build()
  }
}


///  Returns information about a message. 
#[derive(Debug, Clone)]
pub struct TGGetMessage {
  ///  Identifier of the chat the message belongs to. 
  chat_id: Option<i64>,
  ///  Identifier of the message to get. 
  message_id: Option<i64>,
  
}

impl TDFB for TGGetMessage {}

impl AsRef<TGGetMessage> for TGGetMessage {
  fn as_ref(&self) -> &TGGetMessage { self }
}

impl TGGetMessage {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      message_id: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetMessage {
    GetMessage::builder()
      .chat_id(self.chat_id.clone())
      .message_id(self.message_id.clone())
      
      .build()
  }
}


///  Returns a private HTTPS link to a message in a chat. Available only for already sent messages in supergroups and channels. The link will work only for members of the chat. 
#[derive(Debug, Clone)]
pub struct TGGetMessageLink {
  ///  Identifier of the chat to which the message belongs. 
  chat_id: Option<i64>,
  ///  Identifier of the message. 
  message_id: Option<i64>,
  
}

impl TDFB for TGGetMessageLink {}

impl AsRef<TGGetMessageLink> for TGGetMessageLink {
  fn as_ref(&self) -> &TGGetMessageLink { self }
}

impl TGGetMessageLink {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      message_id: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetMessageLink {
    GetMessageLink::builder()
      .chat_id(self.chat_id.clone())
      .message_id(self.message_id.clone())
      
      .build()
  }
}


///  Returns information about a message, if it is available locally without sending network request. This is an offline request. 
#[derive(Debug, Clone)]
pub struct TGGetMessageLocally {
  ///  Identifier of the chat the message belongs to. 
  chat_id: Option<i64>,
  ///  Identifier of the message to get. 
  message_id: Option<i64>,
  
}

impl TDFB for TGGetMessageLocally {}

impl AsRef<TGGetMessageLocally> for TGGetMessageLocally {
  fn as_ref(&self) -> &TGGetMessageLocally { self }
}

impl TGGetMessageLocally {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      message_id: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetMessageLocally {
    GetMessageLocally::builder()
      .chat_id(self.chat_id.clone())
      .message_id(self.message_id.clone())
      
      .build()
  }
}


///  Returns information about messages. If a message is not found, returns null on the corresponding position of the result. 
#[derive(Debug, Clone)]
pub struct TGGetMessages {
  ///  Identifier of the chat the messages belong to. 
  chat_id: Option<i64>,
  ///  Identifiers of the messages to get. 
  message_ids: Option<Vec<i64>>,
  
}

impl TDFB for TGGetMessages {}

impl AsRef<TGGetMessages> for TGGetMessages {
  fn as_ref(&self) -> &TGGetMessages { self }
}

impl TGGetMessages {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      message_ids: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self { self.message_ids = Some(message_ids); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetMessages {
    GetMessages::builder()
      .chat_id(self.chat_id.clone())
      .message_ids(self.message_ids.clone())
      
      .build()
  }
}


///  Returns network data usage statistics. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGGetNetworkStatistics {
  ///  If true, returns only data for the current library launch. 
  only_current: Option<bool>,
  
}

impl TDFB for TGGetNetworkStatistics {}

impl AsRef<TGGetNetworkStatistics> for TGGetNetworkStatistics {
  fn as_ref(&self) -> &TGGetNetworkStatistics { self }
}

impl TGGetNetworkStatistics {

  pub fn new() -> Self {
    Self {
      only_current: None,
      
    }
  }

  
  pub fn only_current(&mut self, only_current: bool) -> &mut Self { self.only_current = Some(only_current); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetNetworkStatistics {
    GetNetworkStatistics::builder()
      .only_current(self.only_current.clone())
      
      .build()
  }
}


///  Returns the value of an option by its name. (Check the list of available options on  
#[derive(Debug, Clone)]
pub struct TGGetOption {
  ///  The name of the option. 
  name: Option<String>,
  
}

impl TDFB for TGGetOption {}

impl AsRef<TGGetOption> for TGGetOption {
  fn as_ref(&self) -> &TGGetOption { self }
}

impl TGGetOption {

  pub fn new() -> Self {
    Self {
      name: None,
      
    }
  }

  
  pub fn name<S: AsRef<str>>(&mut self, name: S) -> &mut Self { self.name = Some(name.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetOption {
    GetOption::builder()
      .name(self.name.clone())
      
      .build()
  }
}


///  Returns a Telegram Passport authorization form for sharing data with a service. 
#[derive(Debug, Clone)]
pub struct TGGetPassportAuthorizationForm {
  ///  User identifier of the service's bot. 
  bot_user_id: Option<i32>,
  ///  Telegram Passport element types requested by the service. 
  scope: Option<String>,
  ///  Service's public_key. 
  public_key: Option<String>,
  ///  Authorization form nonce provided by the service. 
  nonce: Option<String>,
  
}

impl TDFB for TGGetPassportAuthorizationForm {}

impl AsRef<TGGetPassportAuthorizationForm> for TGGetPassportAuthorizationForm {
  fn as_ref(&self) -> &TGGetPassportAuthorizationForm { self }
}

impl TGGetPassportAuthorizationForm {

  pub fn new() -> Self {
    Self {
      bot_user_id: None,
      scope: None,
      public_key: None,
      nonce: None,
      
    }
  }

  
  pub fn bot_user_id(&mut self, bot_user_id: i32) -> &mut Self { self.bot_user_id = Some(bot_user_id); self }
  
  pub fn scope<S: AsRef<str>>(&mut self, scope: S) -> &mut Self { self.scope = Some(scope.as_ref().to_string()); self }
  
  pub fn public_key<S: AsRef<str>>(&mut self, public_key: S) -> &mut Self { self.public_key = Some(public_key.as_ref().to_string()); self }
  
  pub fn nonce<S: AsRef<str>>(&mut self, nonce: S) -> &mut Self { self.nonce = Some(nonce.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetPassportAuthorizationForm {
    GetPassportAuthorizationForm::builder()
      .bot_user_id(self.bot_user_id.clone())
      .scope(self.scope.clone())
      .public_key(self.public_key.clone())
      .nonce(self.nonce.clone())
      
      .build()
  }
}


///  Returns already available Telegram Passport elements suitable for completing a Telegram Passport authorization form. Result can be received only once for each authorization form. 
#[derive(Debug, Clone)]
pub struct TGGetPassportAuthorizationFormAvailableElements {
  ///  Authorization form identifier. 
  autorization_form_id: Option<i32>,
  ///  Password of the current user. 
  password: Option<String>,
  
}

impl TDFB for TGGetPassportAuthorizationFormAvailableElements {}

impl AsRef<TGGetPassportAuthorizationFormAvailableElements> for TGGetPassportAuthorizationFormAvailableElements {
  fn as_ref(&self) -> &TGGetPassportAuthorizationFormAvailableElements { self }
}

impl TGGetPassportAuthorizationFormAvailableElements {

  pub fn new() -> Self {
    Self {
      autorization_form_id: None,
      password: None,
      
    }
  }

  
  pub fn autorization_form_id(&mut self, autorization_form_id: i32) -> &mut Self { self.autorization_form_id = Some(autorization_form_id); self }
  
  pub fn password<S: AsRef<str>>(&mut self, password: S) -> &mut Self { self.password = Some(password.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetPassportAuthorizationFormAvailableElements {
    GetPassportAuthorizationFormAvailableElements::builder()
      .autorization_form_id(self.autorization_form_id.clone())
      .password(self.password.clone())
      
      .build()
  }
}


///  Returns one of the available Telegram Passport elements. 
#[derive(Debug, Clone)]
pub struct TGGetPassportElement {
  ///  Telegram Passport element type. 
  type_: Option<Box<PassportElementType>>,
  ///  Password of the current user. 
  password: Option<String>,
  
}

impl TDFB for TGGetPassportElement {}

impl AsRef<TGGetPassportElement> for TGGetPassportElement {
  fn as_ref(&self) -> &TGGetPassportElement { self }
}

impl TGGetPassportElement {

  pub fn new() -> Self {
    Self {
      type_: None,
      password: None,
      
    }
  }

  
  pub fn password<S: AsRef<str>>(&mut self, password: S) -> &mut Self { self.password = Some(password.as_ref().to_string()); self }
  


  
  // [type_] type is [Box<PassportElementType>], is not support, need add manully.
  #[doc(hidden)] pub fn _type_(&mut self, type_: Box<PassportElementType>) -> &mut Self { self.type_ = Some(type_); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> GetPassportElement {
    GetPassportElement::builder()
      .type_(self.type_.clone())
      .password(self.password.clone())
      
      .build()
  }
}


///  Returns the current state of 2-step verification. 
#[derive(Debug, Clone)]
pub struct TGGetPasswordState {
  
}

impl TDFB for TGGetPasswordState {}

impl AsRef<TGGetPasswordState> for TGGetPasswordState {
  fn as_ref(&self) -> &TGGetPasswordState { self }
}

impl TGGetPasswordState {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetPasswordState {
    GetPasswordState::builder()
      
      .build()
  }
}


///  Returns an invoice payment form. This method should be called when the user presses inlineKeyboardButtonBuy. 
#[derive(Debug, Clone)]
pub struct TGGetPaymentForm {
  ///  Chat identifier of the Invoice message. 
  chat_id: Option<i64>,
  ///  Message identifier. 
  message_id: Option<i64>,
  
}

impl TDFB for TGGetPaymentForm {}

impl AsRef<TGGetPaymentForm> for TGGetPaymentForm {
  fn as_ref(&self) -> &TGGetPaymentForm { self }
}

impl TGGetPaymentForm {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      message_id: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetPaymentForm {
    GetPaymentForm::builder()
      .chat_id(self.chat_id.clone())
      .message_id(self.message_id.clone())
      
      .build()
  }
}


///  Returns information about a successful payment. 
#[derive(Debug, Clone)]
pub struct TGGetPaymentReceipt {
  ///  Chat identifier of the PaymentSuccessful message. 
  chat_id: Option<i64>,
  ///  Message identifier. 
  message_id: Option<i64>,
  
}

impl TDFB for TGGetPaymentReceipt {}

impl AsRef<TGGetPaymentReceipt> for TGGetPaymentReceipt {
  fn as_ref(&self) -> &TGGetPaymentReceipt { self }
}

impl TGGetPaymentReceipt {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      message_id: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetPaymentReceipt {
    GetPaymentReceipt::builder()
      .chat_id(self.chat_id.clone())
      .message_id(self.message_id.clone())
      
      .build()
  }
}


///  Returns an IETF language tag of the language preferred in the country, which should be used to fill native fields in Telegram Passport personal details. Returns a 404 error if unknown. 
#[derive(Debug, Clone)]
pub struct TGGetPreferredCountryLanguage {
  ///  A two-letter ISO 3166-1 alpha-2 country code. 
  country_code: Option<String>,
  
}

impl TDFB for TGGetPreferredCountryLanguage {}

impl AsRef<TGGetPreferredCountryLanguage> for TGGetPreferredCountryLanguage {
  fn as_ref(&self) -> &TGGetPreferredCountryLanguage { self }
}

impl TGGetPreferredCountryLanguage {

  pub fn new() -> Self {
    Self {
      country_code: None,
      
    }
  }

  
  pub fn country_code<S: AsRef<str>>(&mut self, country_code: S) -> &mut Self { self.country_code = Some(country_code.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetPreferredCountryLanguage {
    GetPreferredCountryLanguage::builder()
      .country_code(self.country_code.clone())
      
      .build()
  }
}


///  Returns list of proxies that are currently set up. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGGetProxies {
  
}

impl TDFB for TGGetProxies {}

impl AsRef<TGGetProxies> for TGGetProxies {
  fn as_ref(&self) -> &TGGetProxies { self }
}

impl TGGetProxies {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetProxies {
    GetProxies::builder()
      
      .build()
  }
}


///  Returns an HTTPS link, which can be used to add a proxy. Available only for SOCKS5 and MTProto proxies. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGGetProxyLink {
  ///  Proxy identifier. 
  proxy_id: Option<i32>,
  
}

impl TDFB for TGGetProxyLink {}

impl AsRef<TGGetProxyLink> for TGGetProxyLink {
  fn as_ref(&self) -> &TGGetProxyLink { self }
}

impl TGGetProxyLink {

  pub fn new() -> Self {
    Self {
      proxy_id: None,
      
    }
  }

  
  pub fn proxy_id(&mut self, proxy_id: i32) -> &mut Self { self.proxy_id = Some(proxy_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetProxyLink {
    GetProxyLink::builder()
      .proxy_id(self.proxy_id.clone())
      
      .build()
  }
}


///  Returns a public HTTPS link to a message. Available only for messages in public supergroups and channels. 
#[derive(Debug, Clone)]
pub struct TGGetPublicMessageLink {
  ///  Identifier of the chat to which the message belongs. 
  chat_id: Option<i64>,
  ///  Identifier of the message. 
  message_id: Option<i64>,
  ///  Pass true if a link for a whole media album should be returned. 
  for_album: Option<bool>,
  
}

impl TDFB for TGGetPublicMessageLink {}

impl AsRef<TGGetPublicMessageLink> for TGGetPublicMessageLink {
  fn as_ref(&self) -> &TGGetPublicMessageLink { self }
}

impl TGGetPublicMessageLink {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      message_id: None,
      for_album: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn for_album(&mut self, for_album: bool) -> &mut Self { self.for_album = Some(for_album); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetPublicMessageLink {
    GetPublicMessageLink::builder()
      .chat_id(self.chat_id.clone())
      .message_id(self.message_id.clone())
      .for_album(self.for_album.clone())
      
      .build()
  }
}


///  Returns a globally unique push notification subscription identifier for identification of an account, which has received a push notification. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGGetPushReceiverId {
  ///  JSON-encoded push notification payload. 
  payload: Option<String>,
  
}

impl TDFB for TGGetPushReceiverId {}

impl AsRef<TGGetPushReceiverId> for TGGetPushReceiverId {
  fn as_ref(&self) -> &TGGetPushReceiverId { self }
}

impl TGGetPushReceiverId {

  pub fn new() -> Self {
    Self {
      payload: None,
      
    }
  }

  
  pub fn payload<S: AsRef<str>>(&mut self, payload: S) -> &mut Self { self.payload = Some(payload.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetPushReceiverId {
    GetPushReceiverId::builder()
      .payload(self.payload.clone())
      
      .build()
  }
}


///  Returns up to 20 recently used inline bots in the order of their last usage. 
#[derive(Debug, Clone)]
pub struct TGGetRecentInlineBots {
  
}

impl TDFB for TGGetRecentInlineBots {}

impl AsRef<TGGetRecentInlineBots> for TGGetRecentInlineBots {
  fn as_ref(&self) -> &TGGetRecentInlineBots { self }
}

impl TGGetRecentInlineBots {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetRecentInlineBots {
    GetRecentInlineBots::builder()
      
      .build()
  }
}


///  Returns t.me URLs recently visited by a newly registered user. 
#[derive(Debug, Clone)]
pub struct TGGetRecentlyVisitedTMeUrls {
  ///  Google Play referrer to identify the user. 
  referrer: Option<String>,
  
}

impl TDFB for TGGetRecentlyVisitedTMeUrls {}

impl AsRef<TGGetRecentlyVisitedTMeUrls> for TGGetRecentlyVisitedTMeUrls {
  fn as_ref(&self) -> &TGGetRecentlyVisitedTMeUrls { self }
}

impl TGGetRecentlyVisitedTMeUrls {

  pub fn new() -> Self {
    Self {
      referrer: None,
      
    }
  }

  
  pub fn referrer<S: AsRef<str>>(&mut self, referrer: S) -> &mut Self { self.referrer = Some(referrer.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetRecentlyVisitedTMeUrls {
    GetRecentlyVisitedTMeUrls::builder()
      .referrer(self.referrer.clone())
      
      .build()
  }
}


///  Returns a list of recently used stickers. 
#[derive(Debug, Clone)]
pub struct TGGetRecentStickers {
  ///  Pass true to return stickers and masks that were recently attached to photos or video files; pass false to return recently sent stickers. 
  is_attached: Option<bool>,
  
}

impl TDFB for TGGetRecentStickers {}

impl AsRef<TGGetRecentStickers> for TGGetRecentStickers {
  fn as_ref(&self) -> &TGGetRecentStickers { self }
}

impl TGGetRecentStickers {

  pub fn new() -> Self {
    Self {
      is_attached: None,
      
    }
  }

  
  pub fn is_attached(&mut self, is_attached: bool) -> &mut Self { self.is_attached = Some(is_attached); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetRecentStickers {
    GetRecentStickers::builder()
      .is_attached(self.is_attached.clone())
      
      .build()
  }
}


///  Returns a 2-step verification recovery email address that was previously set up. This method can be used to verify a password provided by the user. 
#[derive(Debug, Clone)]
pub struct TGGetRecoveryEmailAddress {
  ///  The password for the current user. 
  password: Option<String>,
  
}

impl TDFB for TGGetRecoveryEmailAddress {}

impl AsRef<TGGetRecoveryEmailAddress> for TGGetRecoveryEmailAddress {
  fn as_ref(&self) -> &TGGetRecoveryEmailAddress { self }
}

impl TGGetRecoveryEmailAddress {

  pub fn new() -> Self {
    Self {
      password: None,
      
    }
  }

  
  pub fn password<S: AsRef<str>>(&mut self, password: S) -> &mut Self { self.password = Some(password.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetRecoveryEmailAddress {
    GetRecoveryEmailAddress::builder()
      .password(self.password.clone())
      
      .build()
  }
}


///  Returns information about a file by its remote ID; this is an offline request. Can be used to register a URL as a file for further uploading, or sending as a message. 
#[derive(Debug, Clone)]
pub struct TGGetRemoteFile {
  ///  Remote identifier of the file to get. 
  remote_file_id: Option<String>,
  ///  File type, if known. 
  file_type: Option<Box<FileType>>,
  
}

impl TDFB for TGGetRemoteFile {}

impl AsRef<TGGetRemoteFile> for TGGetRemoteFile {
  fn as_ref(&self) -> &TGGetRemoteFile { self }
}

impl TGGetRemoteFile {

  pub fn new() -> Self {
    Self {
      remote_file_id: None,
      file_type: None,
      
    }
  }

  
  pub fn remote_file_id<S: AsRef<str>>(&mut self, remote_file_id: S) -> &mut Self { self.remote_file_id = Some(remote_file_id.as_ref().to_string()); self }
  


  
  // [file_type] type is [Box<FileType>], is not support, need add manully.
  #[doc(hidden)] pub fn _file_type(&mut self, file_type: Box<FileType>) -> &mut Self { self.file_type = Some(file_type); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> GetRemoteFile {
    GetRemoteFile::builder()
      .remote_file_id(self.remote_file_id.clone())
      .file_type(self.file_type.clone())
      
      .build()
  }
}


///  Returns information about a message that is replied by given message. 
#[derive(Debug, Clone)]
pub struct TGGetRepliedMessage {
  ///  Identifier of the chat the message belongs to. 
  chat_id: Option<i64>,
  ///  Identifier of the message reply to which get. 
  message_id: Option<i64>,
  
}

impl TDFB for TGGetRepliedMessage {}

impl AsRef<TGGetRepliedMessage> for TGGetRepliedMessage {
  fn as_ref(&self) -> &TGGetRepliedMessage { self }
}

impl TGGetRepliedMessage {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      message_id: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetRepliedMessage {
    GetRepliedMessage::builder()
      .chat_id(self.chat_id.clone())
      .message_id(self.message_id.clone())
      
      .build()
  }
}


///  Returns saved animations. 
#[derive(Debug, Clone)]
pub struct TGGetSavedAnimations {
  
}

impl TDFB for TGGetSavedAnimations {}

impl AsRef<TGGetSavedAnimations> for TGGetSavedAnimations {
  fn as_ref(&self) -> &TGGetSavedAnimations { self }
}

impl TGGetSavedAnimations {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetSavedAnimations {
    GetSavedAnimations::builder()
      
      .build()
  }
}


///  Returns saved order info, if any. 
#[derive(Debug, Clone)]
pub struct TGGetSavedOrderInfo {
  
}

impl TDFB for TGGetSavedOrderInfo {}

impl AsRef<TGGetSavedOrderInfo> for TGGetSavedOrderInfo {
  fn as_ref(&self) -> &TGGetSavedOrderInfo { self }
}

impl TGGetSavedOrderInfo {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetSavedOrderInfo {
    GetSavedOrderInfo::builder()
      
      .build()
  }
}


///  Returns the notification settings for chats of a given type. 
#[derive(Debug, Clone)]
pub struct TGGetScopeNotificationSettings {
  ///  Types of chats for which to return the notification settings information. 
  scope: Option<Box<NotificationSettingsScope>>,
  
}

impl TDFB for TGGetScopeNotificationSettings {}

impl AsRef<TGGetScopeNotificationSettings> for TGGetScopeNotificationSettings {
  fn as_ref(&self) -> &TGGetScopeNotificationSettings { self }
}

impl TGGetScopeNotificationSettings {

  pub fn new() -> Self {
    Self {
      scope: None,
      
    }
  }

  


  
  // [scope] type is [Box<NotificationSettingsScope>], is not support, need add manully.
  #[doc(hidden)] pub fn _scope(&mut self, scope: Box<NotificationSettingsScope>) -> &mut Self { self.scope = Some(scope); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> GetScopeNotificationSettings {
    GetScopeNotificationSettings::builder()
      .scope(self.scope.clone())
      
      .build()
  }
}


///  Returns information about a secret chat by its identifier. This is an offline request. 
#[derive(Debug, Clone)]
pub struct TGGetSecretChat {
  ///  Secret chat identifier. 
  secret_chat_id: Option<i32>,
  
}

impl TDFB for TGGetSecretChat {}

impl AsRef<TGGetSecretChat> for TGGetSecretChat {
  fn as_ref(&self) -> &TGGetSecretChat { self }
}

impl TGGetSecretChat {

  pub fn new() -> Self {
    Self {
      secret_chat_id: None,
      
    }
  }

  
  pub fn secret_chat_id(&mut self, secret_chat_id: i32) -> &mut Self { self.secret_chat_id = Some(secret_chat_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetSecretChat {
    GetSecretChat::builder()
      .secret_chat_id(self.secret_chat_id.clone())
      
      .build()
  }
}


///  Returns emoji corresponding to a sticker. 
#[derive(Debug, Clone)]
pub struct TGGetStickerEmojis {
  ///  Sticker file identifier. 
  sticker: Option<Box<InputFile>>,
  
}

impl TDFB for TGGetStickerEmojis {}

impl AsRef<TGGetStickerEmojis> for TGGetStickerEmojis {
  fn as_ref(&self) -> &TGGetStickerEmojis { self }
}

impl TGGetStickerEmojis {

  pub fn new() -> Self {
    Self {
      sticker: None,
      
    }
  }

  


  
  // [sticker] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _sticker(&mut self, sticker: Box<InputFile>) -> &mut Self { self.sticker = Some(sticker); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> GetStickerEmojis {
    GetStickerEmojis::builder()
      .sticker(self.sticker.clone())
      
      .build()
  }
}


///  Returns stickers from the installed sticker sets that correspond to a given emoji. If the emoji is not empty, favorite and recently used stickers may also be returned. 
#[derive(Debug, Clone)]
pub struct TGGetStickers {
  ///  String representation of emoji. If empty, returns all known installed stickers. 
  emoji: Option<String>,
  ///  Maximum number of stickers to be returned. 
  limit: Option<i32>,
  
}

impl TDFB for TGGetStickers {}

impl AsRef<TGGetStickers> for TGGetStickers {
  fn as_ref(&self) -> &TGGetStickers { self }
}

impl TGGetStickers {

  pub fn new() -> Self {
    Self {
      emoji: None,
      limit: None,
      
    }
  }

  
  pub fn emoji<S: AsRef<str>>(&mut self, emoji: S) -> &mut Self { self.emoji = Some(emoji.as_ref().to_string()); self }
  
  pub fn limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetStickers {
    GetStickers::builder()
      .emoji(self.emoji.clone())
      .limit(self.limit.clone())
      
      .build()
  }
}


///  Returns information about a sticker set by its identifier. 
#[derive(Debug, Clone)]
pub struct TGGetStickerSet {
  ///  Identifier of the sticker set. 
  set_id: Option<i64>,
  
}

impl TDFB for TGGetStickerSet {}

impl AsRef<TGGetStickerSet> for TGGetStickerSet {
  fn as_ref(&self) -> &TGGetStickerSet { self }
}

impl TGGetStickerSet {

  pub fn new() -> Self {
    Self {
      set_id: None,
      
    }
  }

  
  pub fn set_id(&mut self, set_id: i64) -> &mut Self { self.set_id = Some(set_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetStickerSet {
    GetStickerSet::builder()
      .set_id(self.set_id.clone())
      
      .build()
  }
}


///  Returns storage usage statistics. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGGetStorageStatistics {
  ///  Maximum number of chats with the largest storage usage for which separate statistics should be returned. All other chats will be grouped in entries with chat_id == 0. If the chat info database is not used, the chat_limit is ignored and is always set to 0. 
  chat_limit: Option<i32>,
  
}

impl TDFB for TGGetStorageStatistics {}

impl AsRef<TGGetStorageStatistics> for TGGetStorageStatistics {
  fn as_ref(&self) -> &TGGetStorageStatistics { self }
}

impl TGGetStorageStatistics {

  pub fn new() -> Self {
    Self {
      chat_limit: None,
      
    }
  }

  
  pub fn chat_limit(&mut self, chat_limit: i32) -> &mut Self { self.chat_limit = Some(chat_limit); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetStorageStatistics {
    GetStorageStatistics::builder()
      .chat_limit(self.chat_limit.clone())
      
      .build()
  }
}


///  Quickly returns approximate storage usage statistics. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGGetStorageStatisticsFast {
  
}

impl TDFB for TGGetStorageStatisticsFast {}

impl AsRef<TGGetStorageStatisticsFast> for TGGetStorageStatisticsFast {
  fn as_ref(&self) -> &TGGetStorageStatisticsFast { self }
}

impl TGGetStorageStatisticsFast {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetStorageStatisticsFast {
    GetStorageStatisticsFast::builder()
      
      .build()
  }
}


///  Returns information about a supergroup or channel by its identifier. This is an offline request if the current user is not a bot. 
#[derive(Debug, Clone)]
pub struct TGGetSupergroup {
  ///  Supergroup or channel identifier. 
  supergroup_id: Option<i32>,
  
}

impl TDFB for TGGetSupergroup {}

impl AsRef<TGGetSupergroup> for TGGetSupergroup {
  fn as_ref(&self) -> &TGGetSupergroup { self }
}

impl TGGetSupergroup {

  pub fn new() -> Self {
    Self {
      supergroup_id: None,
      
    }
  }

  
  pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self { self.supergroup_id = Some(supergroup_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetSupergroup {
    GetSupergroup::builder()
      .supergroup_id(self.supergroup_id.clone())
      
      .build()
  }
}


///  Returns full information about a supergroup or channel by its identifier, cached for up to 1 minute. 
#[derive(Debug, Clone)]
pub struct TGGetSupergroupFullInfo {
  ///  Supergroup or channel identifier. 
  supergroup_id: Option<i32>,
  
}

impl TDFB for TGGetSupergroupFullInfo {}

impl AsRef<TGGetSupergroupFullInfo> for TGGetSupergroupFullInfo {
  fn as_ref(&self) -> &TGGetSupergroupFullInfo { self }
}

impl TGGetSupergroupFullInfo {

  pub fn new() -> Self {
    Self {
      supergroup_id: None,
      
    }
  }

  
  pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self { self.supergroup_id = Some(supergroup_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetSupergroupFullInfo {
    GetSupergroupFullInfo::builder()
      .supergroup_id(self.supergroup_id.clone())
      
      .build()
  }
}


///  Returns information about members or banned users in a supergroup or channel. Can be used only if SupergroupFullInfo.can_get_members == true; additionally, administrator privileges may be required for some filters. 
#[derive(Debug, Clone)]
pub struct TGGetSupergroupMembers {
  ///  Identifier of the supergroup or channel. 
  supergroup_id: Option<i32>,
  ///  The type of users to return. By default, supergroupMembersRecent. 
  filter: Option<Box<SupergroupMembersFilter>>,
  ///  Number of users to skip. 
  offset: Option<i32>,
  ///  The maximum number of users be returned; up to 200. 
  limit: Option<i32>,
  
}

impl TDFB for TGGetSupergroupMembers {}

impl AsRef<TGGetSupergroupMembers> for TGGetSupergroupMembers {
  fn as_ref(&self) -> &TGGetSupergroupMembers { self }
}

impl TGGetSupergroupMembers {

  pub fn new() -> Self {
    Self {
      supergroup_id: None,
      filter: None,
      offset: None,
      limit: None,
      
    }
  }

  
  pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self { self.supergroup_id = Some(supergroup_id); self }
  
  pub fn offset(&mut self, offset: i32) -> &mut Self { self.offset = Some(offset); self }
  
  pub fn limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  


  
  // [filter] type is [Box<SupergroupMembersFilter>], is not support, need add manully.
  #[doc(hidden)] pub fn _filter(&mut self, filter: Box<SupergroupMembersFilter>) -> &mut Self { self.filter = Some(filter); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> GetSupergroupMembers {
    GetSupergroupMembers::builder()
      .supergroup_id(self.supergroup_id.clone())
      .filter(self.filter.clone())
      .offset(self.offset.clone())
      .limit(self.limit.clone())
      
      .build()
  }
}


///  Returns a user that can be contacted to get support. 
#[derive(Debug, Clone)]
pub struct TGGetSupportUser {
  
}

impl TDFB for TGGetSupportUser {}

impl AsRef<TGGetSupportUser> for TGGetSupportUser {
  fn as_ref(&self) -> &TGGetSupportUser { self }
}

impl TGGetSupportUser {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetSupportUser {
    GetSupportUser::builder()
      
      .build()
  }
}


///  Returns information about the current temporary password. 
#[derive(Debug, Clone)]
pub struct TGGetTemporaryPasswordState {
  
}

impl TDFB for TGGetTemporaryPasswordState {}

impl AsRef<TGGetTemporaryPasswordState> for TGGetTemporaryPasswordState {
  fn as_ref(&self) -> &TGGetTemporaryPasswordState { self }
}

impl TGGetTemporaryPasswordState {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetTemporaryPasswordState {
    GetTemporaryPasswordState::builder()
      
      .build()
  }
}


///  Returns all entities (mentions, hashtags, cashtags, bot commands, URLs, and email addresses) contained in the text. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGGetTextEntities {
  ///  The text in which to look for entites. 
  text: Option<String>,
  
}

impl TDFB for TGGetTextEntities {}

impl AsRef<TGGetTextEntities> for TGGetTextEntities {
  fn as_ref(&self) -> &TGGetTextEntities { self }
}

impl TGGetTextEntities {

  pub fn new() -> Self {
    Self {
      text: None,
      
    }
  }

  
  pub fn text<S: AsRef<str>>(&mut self, text: S) -> &mut Self { self.text = Some(text.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetTextEntities {
    GetTextEntities::builder()
      .text(self.text.clone())
      
      .build()
  }
}


///  Returns a list of frequently used chats. Supported only if the chat info database is enabled. 
#[derive(Debug, Clone)]
pub struct TGGetTopChats {
  ///  Category of chats to be returned. 
  category: Option<Box<TopChatCategory>>,
  ///  Maximum number of chats to be returned; up to 30. 
  limit: Option<i32>,
  
}

impl TDFB for TGGetTopChats {}

impl AsRef<TGGetTopChats> for TGGetTopChats {
  fn as_ref(&self) -> &TGGetTopChats { self }
}

impl TGGetTopChats {

  pub fn new() -> Self {
    Self {
      category: None,
      limit: None,
      
    }
  }

  
  pub fn limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  


  
  // [category] type is [Box<TopChatCategory>], is not support, need add manully.
  #[doc(hidden)] pub fn _category(&mut self, category: Box<TopChatCategory>) -> &mut Self { self.category = Some(category); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> GetTopChats {
    GetTopChats::builder()
      .category(self.category.clone())
      .limit(self.limit.clone())
      
      .build()
  }
}


///  Returns a list of trending sticker sets. 
#[derive(Debug, Clone)]
pub struct TGGetTrendingStickerSets {
  
}

impl TDFB for TGGetTrendingStickerSets {}

impl AsRef<TGGetTrendingStickerSets> for TGGetTrendingStickerSets {
  fn as_ref(&self) -> &TGGetTrendingStickerSets { self }
}

impl TGGetTrendingStickerSets {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetTrendingStickerSets {
    GetTrendingStickerSets::builder()
      
      .build()
  }
}


///  Returns information about a user by their identifier. This is an offline request if the current user is not a bot. 
#[derive(Debug, Clone)]
pub struct TGGetUser {
  ///  User identifier. 
  user_id: Option<i32>,
  
}

impl TDFB for TGGetUser {}

impl AsRef<TGGetUser> for TGGetUser {
  fn as_ref(&self) -> &TGGetUser { self }
}

impl TGGetUser {

  pub fn new() -> Self {
    Self {
      user_id: None,
      
    }
  }

  
  pub fn user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetUser {
    GetUser::builder()
      .user_id(self.user_id.clone())
      
      .build()
  }
}


///  Returns full information about a user by their identifier. 
#[derive(Debug, Clone)]
pub struct TGGetUserFullInfo {
  ///  User identifier. 
  user_id: Option<i32>,
  
}

impl TDFB for TGGetUserFullInfo {}

impl AsRef<TGGetUserFullInfo> for TGGetUserFullInfo {
  fn as_ref(&self) -> &TGGetUserFullInfo { self }
}

impl TGGetUserFullInfo {

  pub fn new() -> Self {
    Self {
      user_id: None,
      
    }
  }

  
  pub fn user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetUserFullInfo {
    GetUserFullInfo::builder()
      .user_id(self.user_id.clone())
      
      .build()
  }
}


///  Returns the current privacy settings. 
#[derive(Debug, Clone)]
pub struct TGGetUserPrivacySettingRules {
  ///  The privacy setting. 
  setting: Option<Box<UserPrivacySetting>>,
  
}

impl TDFB for TGGetUserPrivacySettingRules {}

impl AsRef<TGGetUserPrivacySettingRules> for TGGetUserPrivacySettingRules {
  fn as_ref(&self) -> &TGGetUserPrivacySettingRules { self }
}

impl TGGetUserPrivacySettingRules {

  pub fn new() -> Self {
    Self {
      setting: None,
      
    }
  }

  


  
  // [setting] type is [Box<UserPrivacySetting>], is not support, need add manully.
  #[doc(hidden)] pub fn _setting(&mut self, setting: Box<UserPrivacySetting>) -> &mut Self { self.setting = Some(setting); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> GetUserPrivacySettingRules {
    GetUserPrivacySettingRules::builder()
      .setting(self.setting.clone())
      
      .build()
  }
}


///  Returns the profile photos of a user. The result of this query may be outdated: some photos might have been deleted already. 
#[derive(Debug, Clone)]
pub struct TGGetUserProfilePhotos {
  ///  User identifier. 
  user_id: Option<i32>,
  ///  The number of photos to skip; must be non-negative. 
  offset: Option<i32>,
  ///  Maximum number of photos to be returned; up to 100. 
  limit: Option<i32>,
  
}

impl TDFB for TGGetUserProfilePhotos {}

impl AsRef<TGGetUserProfilePhotos> for TGGetUserProfilePhotos {
  fn as_ref(&self) -> &TGGetUserProfilePhotos { self }
}

impl TGGetUserProfilePhotos {

  pub fn new() -> Self {
    Self {
      user_id: None,
      offset: None,
      limit: None,
      
    }
  }

  
  pub fn user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn offset(&mut self, offset: i32) -> &mut Self { self.offset = Some(offset); self }
  
  pub fn limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetUserProfilePhotos {
    GetUserProfilePhotos::builder()
      .user_id(self.user_id.clone())
      .offset(self.offset.clone())
      .limit(self.limit.clone())
      
      .build()
  }
}


///  Returns background wallpapers. 
#[derive(Debug, Clone)]
pub struct TGGetWallpapers {
  
}

impl TDFB for TGGetWallpapers {}

impl AsRef<TGGetWallpapers> for TGGetWallpapers {
  fn as_ref(&self) -> &TGGetWallpapers { self }
}

impl TGGetWallpapers {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetWallpapers {
    GetWallpapers::builder()
      
      .build()
  }
}


///  Returns an instant view version of a web page if available. Returns a 404 error if the web page has no instant view page. 
#[derive(Debug, Clone)]
pub struct TGGetWebPageInstantView {
  ///  The web page URL. 
  url: Option<String>,
  ///  If true, the full instant view for the web page will be returned. 
  force_full: Option<bool>,
  
}

impl TDFB for TGGetWebPageInstantView {}

impl AsRef<TGGetWebPageInstantView> for TGGetWebPageInstantView {
  fn as_ref(&self) -> &TGGetWebPageInstantView { self }
}

impl TGGetWebPageInstantView {

  pub fn new() -> Self {
    Self {
      url: None,
      force_full: None,
      
    }
  }

  
  pub fn url<S: AsRef<str>>(&mut self, url: S) -> &mut Self { self.url = Some(url.as_ref().to_string()); self }
  
  pub fn force_full(&mut self, force_full: bool) -> &mut Self { self.force_full = Some(force_full); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> GetWebPageInstantView {
    GetWebPageInstantView::builder()
      .url(self.url.clone())
      .force_full(self.force_full.clone())
      
      .build()
  }
}


///  Returns a web page preview by the text of the message. Do not call this function too often. Returns a 404 error if the web page has no preview. 
#[derive(Debug, Clone)]
pub struct TGGetWebPagePreview {
  ///  Message text with formatting. 
  text: Option<FormattedText>,
  
}

impl TDFB for TGGetWebPagePreview {}

impl AsRef<TGGetWebPagePreview> for TGGetWebPagePreview {
  fn as_ref(&self) -> &TGGetWebPagePreview { self }
}

impl TGGetWebPagePreview {

  pub fn new() -> Self {
    Self {
      text: None,
      
    }
  }

  


  
  // [text] type is [FormattedText], is not support, need add manully.
  #[doc(hidden)] pub fn _text(&mut self, text: FormattedText) -> &mut Self { self.text = Some(text); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> GetWebPagePreview {
    GetWebPagePreview::builder()
      .text(self.text.clone())
      
      .build()
  }
}


///  Adds new contacts or edits existing contacts; contacts' user identifiers are ignored. 
#[derive(Debug, Clone)]
pub struct TGImportContacts {
  ///  The list of contacts to import or edit, contact's vCard are ignored and are not imported. 
  contacts: Option<Vec<Contact>>,
  
}

impl TDFB for TGImportContacts {}

impl AsRef<TGImportContacts> for TGImportContacts {
  fn as_ref(&self) -> &TGImportContacts { self }
}

impl TGImportContacts {

  pub fn new() -> Self {
    Self {
      contacts: None,
      
    }
  }

  


  
  // [contacts] type is [Vec<Contact>], is not support, need add manully.
  #[doc(hidden)] pub fn _contacts(&mut self, contacts: Vec<Contact>) -> &mut Self { self.contacts = Some(contacts); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> ImportContacts {
    ImportContacts::builder()
      .contacts(self.contacts.clone())
      
      .build()
  }
}


///  Adds current user as a new member to a chat. Private and secret chats can't be joined using this method. 
#[derive(Debug, Clone)]
pub struct TGJoinChat {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  
}

impl TDFB for TGJoinChat {}

impl AsRef<TGJoinChat> for TGJoinChat {
  fn as_ref(&self) -> &TGJoinChat { self }
}

impl TGJoinChat {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> JoinChat {
    JoinChat::builder()
      .chat_id(self.chat_id.clone())
      
      .build()
  }
}


///  Uses an invite link to add the current user to the chat if possible. The new member will not be added until the chat state has been synchronized with the server. 
#[derive(Debug, Clone)]
pub struct TGJoinChatByInviteLink {
  ///  Invite link to import; should begin with "https://t.me/joinchat/", "https://telegram.me/joinchat/", or "https://telegram.dog/joinchat/". 
  invite_link: Option<String>,
  
}

impl TDFB for TGJoinChatByInviteLink {}

impl AsRef<TGJoinChatByInviteLink> for TGJoinChatByInviteLink {
  fn as_ref(&self) -> &TGJoinChatByInviteLink { self }
}

impl TGJoinChatByInviteLink {

  pub fn new() -> Self {
    Self {
      invite_link: None,
      
    }
  }

  
  pub fn invite_link<S: AsRef<str>>(&mut self, invite_link: S) -> &mut Self { self.invite_link = Some(invite_link.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> JoinChatByInviteLink {
    JoinChatByInviteLink::builder()
      .invite_link(self.invite_link.clone())
      
      .build()
  }
}


///  Removes current user from chat members. Private and secret chats can't be left using this method. 
#[derive(Debug, Clone)]
pub struct TGLeaveChat {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  
}

impl TDFB for TGLeaveChat {}

impl AsRef<TGLeaveChat> for TGLeaveChat {
  fn as_ref(&self) -> &TGLeaveChat { self }
}

impl TGLeaveChat {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> LeaveChat {
    LeaveChat::builder()
      .chat_id(self.chat_id.clone())
      
      .build()
  }
}


///  Closes the TDLib instance after a proper logout. Requires an available network connection. All local data will be destroyed. After the logout completes,  
#[derive(Debug, Clone)]
pub struct TGLogOut {
  
}

impl TDFB for TGLogOut {}

impl AsRef<TGLogOut> for TGLogOut {
  fn as_ref(&self) -> &TGLogOut { self }
}

impl TGLogOut {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> LogOut {
    LogOut::builder()
      
      .build()
  }
}


///  Informs TDLib that the chat is opened by the user. Many useful activities depend on the chat being opened or closed (e.g., in supergroups and channels all updates are received only for opened chats). 
#[derive(Debug, Clone)]
pub struct TGOpenChat {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  
}

impl TDFB for TGOpenChat {}

impl AsRef<TGOpenChat> for TGOpenChat {
  fn as_ref(&self) -> &TGOpenChat { self }
}

impl TGOpenChat {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> OpenChat {
    OpenChat::builder()
      .chat_id(self.chat_id.clone())
      
      .build()
  }
}


///  Informs TDLib that the message content has been opened (e.g., the user has opened a photo, video, document, location or venue, or has listened to an audio file or voice note message). An  
#[derive(Debug, Clone)]
pub struct TGOpenMessageContent {
  ///  Chat identifier of the message. 
  chat_id: Option<i64>,
  ///  Identifier of the message with the opened content. 
  message_id: Option<i64>,
  
}

impl TDFB for TGOpenMessageContent {}

impl AsRef<TGOpenMessageContent> for TGOpenMessageContent {
  fn as_ref(&self) -> &TGOpenMessageContent { self }
}

impl TGOpenMessageContent {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      message_id: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> OpenMessageContent {
    OpenMessageContent::builder()
      .chat_id(self.chat_id.clone())
      .message_id(self.message_id.clone())
      
      .build()
  }
}


///  Optimizes storage usage, i.e. deletes some files and returns new storage usage statistics. Secret thumbnails can't be deleted. 
#[derive(Debug, Clone)]
pub struct TGOptimizeStorage {
  ///  Limit on the total size of files after deletion. Pass -1 to use the default limit. 
  size: Option<i64>,
  ///  Limit on the time that has passed since the last time a file was accessed (or creation time for some filesystems). Pass -1 to use the default limit. 
  ttl: Option<i32>,
  ///  Limit on the total count of files after deletion. Pass -1 to use the default limit. 
  count: Option<i32>,
  ///  The amount of time after the creation of a file during which it can't be deleted, in seconds. Pass -1 to use the default value. 
  immunity_delay: Option<i32>,
  ///  If not empty, only files with the given type(s) are considered. By default, all types except thumbnails, profile photos, stickers and wallpapers are deleted. 
  file_types: Option<Vec<Box<FileType>>>,
  ///  If not empty, only files from the given chats are considered. Use 0 as chat identifier to delete files not belonging to any chat (e.g., profile photos). 
  chat_ids: Option<Vec<i64>>,
  ///  If not empty, files from the given chats are excluded. Use 0 as chat identifier to exclude all files not belonging to any chat (e.g., profile photos). 
  exclude_chat_ids: Option<Vec<i64>>,
  ///  Same as in getStorageStatistics. Affects only returned statistics. 
  chat_limit: Option<i32>,
  
}

impl TDFB for TGOptimizeStorage {}

impl AsRef<TGOptimizeStorage> for TGOptimizeStorage {
  fn as_ref(&self) -> &TGOptimizeStorage { self }
}

impl TGOptimizeStorage {

  pub fn new() -> Self {
    Self {
      size: None,
      ttl: None,
      count: None,
      immunity_delay: None,
      file_types: None,
      chat_ids: None,
      exclude_chat_ids: None,
      chat_limit: None,
      
    }
  }

  
  pub fn size(&mut self, size: i64) -> &mut Self { self.size = Some(size); self }
  
  pub fn ttl(&mut self, ttl: i32) -> &mut Self { self.ttl = Some(ttl); self }
  
  pub fn count(&mut self, count: i32) -> &mut Self { self.count = Some(count); self }
  
  pub fn immunity_delay(&mut self, immunity_delay: i32) -> &mut Self { self.immunity_delay = Some(immunity_delay); self }
  
  pub fn chat_ids(&mut self, chat_ids: Vec<i64>) -> &mut Self { self.chat_ids = Some(chat_ids); self }
  
  pub fn exclude_chat_ids(&mut self, exclude_chat_ids: Vec<i64>) -> &mut Self { self.exclude_chat_ids = Some(exclude_chat_ids); self }
  
  pub fn chat_limit(&mut self, chat_limit: i32) -> &mut Self { self.chat_limit = Some(chat_limit); self }
  


  
  // [file_types] type is [Vec<Box<FileType>>], is not support, need add manully.
  #[doc(hidden)] pub fn _file_types(&mut self, file_types: Vec<Box<FileType>>) -> &mut Self { self.file_types = Some(file_types); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> OptimizeStorage {
    OptimizeStorage::builder()
      .size(self.size.clone())
      .ttl(self.ttl.clone())
      .count(self.count.clone())
      .immunity_delay(self.immunity_delay.clone())
      .file_types(self.file_types.clone())
      .chat_ids(self.chat_ids.clone())
      .exclude_chat_ids(self.exclude_chat_ids.clone())
      .chat_limit(self.chat_limit.clone())
      
      .build()
  }
}


///  Parses Bold, Italic, Code, Pre, PreCode and TextUrl entities contained in the text. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGParseTextEntities {
  ///  The text which should be parsed. 
  text: Option<String>,
  ///  Text parse mode. 
  parse_mode: Option<Box<TextParseMode>>,
  
}

impl TDFB for TGParseTextEntities {}

impl AsRef<TGParseTextEntities> for TGParseTextEntities {
  fn as_ref(&self) -> &TGParseTextEntities { self }
}

impl TGParseTextEntities {

  pub fn new() -> Self {
    Self {
      text: None,
      parse_mode: None,
      
    }
  }

  
  pub fn text<S: AsRef<str>>(&mut self, text: S) -> &mut Self { self.text = Some(text.as_ref().to_string()); self }
  


  
  // [parse_mode] type is [Box<TextParseMode>], is not support, need add manully.
  #[doc(hidden)] pub fn _parse_mode(&mut self, parse_mode: Box<TextParseMode>) -> &mut Self { self.parse_mode = Some(parse_mode); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> ParseTextEntities {
    ParseTextEntities::builder()
      .text(self.text.clone())
      .parse_mode(self.parse_mode.clone())
      
      .build()
  }
}


///  Pins a message in a chat; requires appropriate administrator rights in the group or channel. 
#[derive(Debug, Clone)]
pub struct TGPinChatMessage {
  ///  Identifier of the chat. 
  chat_id: Option<i64>,
  ///  Identifier of the new pinned message. 
  message_id: Option<i64>,
  ///  True, if there should be no notification about the pinned message. 
  disable_notification: Option<bool>,
  
}

impl TDFB for TGPinChatMessage {}

impl AsRef<TGPinChatMessage> for TGPinChatMessage {
  fn as_ref(&self) -> &TGPinChatMessage { self }
}

impl TGPinChatMessage {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      message_id: None,
      disable_notification: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn disable_notification(&mut self, disable_notification: bool) -> &mut Self { self.disable_notification = Some(disable_notification); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> PinChatMessage {
    PinChatMessage::builder()
      .chat_id(self.chat_id.clone())
      .message_id(self.message_id.clone())
      .disable_notification(self.disable_notification.clone())
      
      .build()
  }
}


///  Computes time needed to receive a response from a Telegram server through a proxy. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGPingProxy {
  ///  Proxy identifier. Use 0 to ping a Telegram server without a proxy. 
  proxy_id: Option<i32>,
  
}

impl TDFB for TGPingProxy {}

impl AsRef<TGPingProxy> for TGPingProxy {
  fn as_ref(&self) -> &TGPingProxy { self }
}

impl TGPingProxy {

  pub fn new() -> Self {
    Self {
      proxy_id: None,
      
    }
  }

  
  pub fn proxy_id(&mut self, proxy_id: i32) -> &mut Self { self.proxy_id = Some(proxy_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> PingProxy {
    PingProxy::builder()
      .proxy_id(self.proxy_id.clone())
      
      .build()
  }
}


///  Handles a push notification. Returns error with code 406 if the push notification is not supported and connection to the server is required to fetch new data. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGProcessPushNotification {
  ///  JSON-encoded push notification payload with all fields sent by the server, and "google.sent_time" and "google.notification.sound" fields added. 
  payload: Option<String>,
  
}

impl TDFB for TGProcessPushNotification {}

impl AsRef<TGProcessPushNotification> for TGProcessPushNotification {
  fn as_ref(&self) -> &TGProcessPushNotification { self }
}

impl TGProcessPushNotification {

  pub fn new() -> Self {
    Self {
      payload: None,
      
    }
  }

  
  pub fn payload<S: AsRef<str>>(&mut self, payload: S) -> &mut Self { self.payload = Some(payload.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> ProcessPushNotification {
    ProcessPushNotification::builder()
      .payload(self.payload.clone())
      
      .build()
  }
}


///  Marks all mentions in a chat as read. 
#[derive(Debug, Clone)]
pub struct TGReadAllChatMentions {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  
}

impl TDFB for TGReadAllChatMentions {}

impl AsRef<TGReadAllChatMentions> for TGReadAllChatMentions {
  fn as_ref(&self) -> &TGReadAllChatMentions { self }
}

impl TGReadAllChatMentions {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> ReadAllChatMentions {
    ReadAllChatMentions::builder()
      .chat_id(self.chat_id.clone())
      
      .build()
  }
}


///  Reads a part of a file from the TDLib file cache and returns read bytes. This method is intended to be used only if the client has no direct access to TDLib's file system, because it is usually slower than a direct read from the file. 
#[derive(Debug, Clone)]
pub struct TGReadFilePart {
  ///  Identifier of the file. The file must be located in the TDLib file cache. 
  file_id: Option<i32>,
  ///  The offset from which to read the file. 
  offset: Option<i32>,
  ///  Number of bytes to read. An error will be returned if there are not enough bytes available in the file from the specified position. Pass 0 to read all available data from the specified position. 
  count: Option<i32>,
  
}

impl TDFB for TGReadFilePart {}

impl AsRef<TGReadFilePart> for TGReadFilePart {
  fn as_ref(&self) -> &TGReadFilePart { self }
}

impl TGReadFilePart {

  pub fn new() -> Self {
    Self {
      file_id: None,
      offset: None,
      count: None,
      
    }
  }

  
  pub fn file_id(&mut self, file_id: i32) -> &mut Self { self.file_id = Some(file_id); self }
  
  pub fn offset(&mut self, offset: i32) -> &mut Self { self.offset = Some(offset); self }
  
  pub fn count(&mut self, count: i32) -> &mut Self { self.count = Some(count); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> ReadFilePart {
    ReadFilePart::builder()
      .file_id(self.file_id.clone())
      .offset(self.offset.clone())
      .count(self.count.clone())
      
      .build()
  }
}


///  Recovers the password with a password recovery code sent to an email address that was previously set up. Works only when the current authorization state is  
#[derive(Debug, Clone)]
pub struct TGRecoverAuthenticationPassword {
  ///  Recovery code to check. 
  recovery_code: Option<String>,
  
}

impl TDFB for TGRecoverAuthenticationPassword {}

impl AsRef<TGRecoverAuthenticationPassword> for TGRecoverAuthenticationPassword {
  fn as_ref(&self) -> &TGRecoverAuthenticationPassword { self }
}

impl TGRecoverAuthenticationPassword {

  pub fn new() -> Self {
    Self {
      recovery_code: None,
      
    }
  }

  
  pub fn recovery_code<S: AsRef<str>>(&mut self, recovery_code: S) -> &mut Self { self.recovery_code = Some(recovery_code.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> RecoverAuthenticationPassword {
    RecoverAuthenticationPassword::builder()
      .recovery_code(self.recovery_code.clone())
      
      .build()
  }
}


///  Recovers the password using a recovery code sent to an email address that was previously set up. 
#[derive(Debug, Clone)]
pub struct TGRecoverPassword {
  ///  Recovery code to check. 
  recovery_code: Option<String>,
  
}

impl TDFB for TGRecoverPassword {}

impl AsRef<TGRecoverPassword> for TGRecoverPassword {
  fn as_ref(&self) -> &TGRecoverPassword { self }
}

impl TGRecoverPassword {

  pub fn new() -> Self {
    Self {
      recovery_code: None,
      
    }
  }

  
  pub fn recovery_code<S: AsRef<str>>(&mut self, recovery_code: S) -> &mut Self { self.recovery_code = Some(recovery_code.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> RecoverPassword {
    RecoverPassword::builder()
      .recovery_code(self.recovery_code.clone())
      
      .build()
  }
}


///  Registers the currently used device for receiving push notifications. Returns a globally unique identifier of the push notification subscription. 
#[derive(Debug, Clone)]
pub struct TGRegisterDevice {
  ///  Device token. 
  device_token: Option<Box<DeviceToken>>,
  ///  List of user identifiers of other users currently using the client. 
  other_user_ids: Option<Vec<i32>>,
  
}

impl TDFB for TGRegisterDevice {}

impl AsRef<TGRegisterDevice> for TGRegisterDevice {
  fn as_ref(&self) -> &TGRegisterDevice { self }
}

impl TGRegisterDevice {

  pub fn new() -> Self {
    Self {
      device_token: None,
      other_user_ids: None,
      
    }
  }

  
  pub fn other_user_ids(&mut self, other_user_ids: Vec<i32>) -> &mut Self { self.other_user_ids = Some(other_user_ids); self }
  


  
  // [device_token] type is [Box<DeviceToken>], is not support, need add manully.
  #[doc(hidden)] pub fn _device_token(&mut self, device_token: Box<DeviceToken>) -> &mut Self { self.device_token = Some(device_token); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> RegisterDevice {
    RegisterDevice::builder()
      .device_token(self.device_token.clone())
      .other_user_ids(self.other_user_ids.clone())
      
      .build()
  }
}


///  Removes users from the contact list. 
#[derive(Debug, Clone)]
pub struct TGRemoveContacts {
  ///  Identifiers of users to be deleted. 
  user_ids: Option<Vec<i32>>,
  
}

impl TDFB for TGRemoveContacts {}

impl AsRef<TGRemoveContacts> for TGRemoveContacts {
  fn as_ref(&self) -> &TGRemoveContacts { self }
}

impl TGRemoveContacts {

  pub fn new() -> Self {
    Self {
      user_ids: None,
      
    }
  }

  
  pub fn user_ids(&mut self, user_ids: Vec<i32>) -> &mut Self { self.user_ids = Some(user_ids); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> RemoveContacts {
    RemoveContacts::builder()
      .user_ids(self.user_ids.clone())
      
      .build()
  }
}


///  Removes a sticker from the list of favorite stickers. 
#[derive(Debug, Clone)]
pub struct TGRemoveFavoriteSticker {
  ///  Sticker file to delete from the list. 
  sticker: Option<Box<InputFile>>,
  
}

impl TDFB for TGRemoveFavoriteSticker {}

impl AsRef<TGRemoveFavoriteSticker> for TGRemoveFavoriteSticker {
  fn as_ref(&self) -> &TGRemoveFavoriteSticker { self }
}

impl TGRemoveFavoriteSticker {

  pub fn new() -> Self {
    Self {
      sticker: None,
      
    }
  }

  


  
  // [sticker] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _sticker(&mut self, sticker: Box<InputFile>) -> &mut Self { self.sticker = Some(sticker); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> RemoveFavoriteSticker {
    RemoveFavoriteSticker::builder()
      .sticker(self.sticker.clone())
      
      .build()
  }
}


///  Removes an active notification from notification list. Needs to be called only if the notification is removed by the current user. 
#[derive(Debug, Clone)]
pub struct TGRemoveNotification {
  ///  Identifier of notification group to which the notification belongs. 
  notification_group_id: Option<i32>,
  ///  Identifier of removed notification. 
  notification_id: Option<i32>,
  
}

impl TDFB for TGRemoveNotification {}

impl AsRef<TGRemoveNotification> for TGRemoveNotification {
  fn as_ref(&self) -> &TGRemoveNotification { self }
}

impl TGRemoveNotification {

  pub fn new() -> Self {
    Self {
      notification_group_id: None,
      notification_id: None,
      
    }
  }

  
  pub fn notification_group_id(&mut self, notification_group_id: i32) -> &mut Self { self.notification_group_id = Some(notification_group_id); self }
  
  pub fn notification_id(&mut self, notification_id: i32) -> &mut Self { self.notification_id = Some(notification_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> RemoveNotification {
    RemoveNotification::builder()
      .notification_group_id(self.notification_group_id.clone())
      .notification_id(self.notification_id.clone())
      
      .build()
  }
}


///  Removes a group of active notifications. Needs to be called only if the notification group is removed by the current user. 
#[derive(Debug, Clone)]
pub struct TGRemoveNotificationGroup {
  ///  Notification group identifier. 
  notification_group_id: Option<i32>,
  ///  Maximum identifier of removed notifications. 
  max_notification_id: Option<i32>,
  
}

impl TDFB for TGRemoveNotificationGroup {}

impl AsRef<TGRemoveNotificationGroup> for TGRemoveNotificationGroup {
  fn as_ref(&self) -> &TGRemoveNotificationGroup { self }
}

impl TGRemoveNotificationGroup {

  pub fn new() -> Self {
    Self {
      notification_group_id: None,
      max_notification_id: None,
      
    }
  }

  
  pub fn notification_group_id(&mut self, notification_group_id: i32) -> &mut Self { self.notification_group_id = Some(notification_group_id); self }
  
  pub fn max_notification_id(&mut self, max_notification_id: i32) -> &mut Self { self.max_notification_id = Some(max_notification_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> RemoveNotificationGroup {
    RemoveNotificationGroup::builder()
      .notification_group_id(self.notification_group_id.clone())
      .max_notification_id(self.max_notification_id.clone())
      
      .build()
  }
}


///  Removes a proxy server. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGRemoveProxy {
  ///  Proxy identifier. 
  proxy_id: Option<i32>,
  
}

impl TDFB for TGRemoveProxy {}

impl AsRef<TGRemoveProxy> for TGRemoveProxy {
  fn as_ref(&self) -> &TGRemoveProxy { self }
}

impl TGRemoveProxy {

  pub fn new() -> Self {
    Self {
      proxy_id: None,
      
    }
  }

  
  pub fn proxy_id(&mut self, proxy_id: i32) -> &mut Self { self.proxy_id = Some(proxy_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> RemoveProxy {
    RemoveProxy::builder()
      .proxy_id(self.proxy_id.clone())
      
      .build()
  }
}


///  Removes a hashtag from the list of recently used hashtags. 
#[derive(Debug, Clone)]
pub struct TGRemoveRecentHashtag {
  ///  Hashtag to delete. 
  hashtag: Option<String>,
  
}

impl TDFB for TGRemoveRecentHashtag {}

impl AsRef<TGRemoveRecentHashtag> for TGRemoveRecentHashtag {
  fn as_ref(&self) -> &TGRemoveRecentHashtag { self }
}

impl TGRemoveRecentHashtag {

  pub fn new() -> Self {
    Self {
      hashtag: None,
      
    }
  }

  
  pub fn hashtag<S: AsRef<str>>(&mut self, hashtag: S) -> &mut Self { self.hashtag = Some(hashtag.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> RemoveRecentHashtag {
    RemoveRecentHashtag::builder()
      .hashtag(self.hashtag.clone())
      
      .build()
  }
}


///  Removes a chat from the list of recently found chats. 
#[derive(Debug, Clone)]
pub struct TGRemoveRecentlyFoundChat {
  ///  Identifier of the chat to be removed. 
  chat_id: Option<i64>,
  
}

impl TDFB for TGRemoveRecentlyFoundChat {}

impl AsRef<TGRemoveRecentlyFoundChat> for TGRemoveRecentlyFoundChat {
  fn as_ref(&self) -> &TGRemoveRecentlyFoundChat { self }
}

impl TGRemoveRecentlyFoundChat {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> RemoveRecentlyFoundChat {
    RemoveRecentlyFoundChat::builder()
      .chat_id(self.chat_id.clone())
      
      .build()
  }
}


///  Removes a sticker from the list of recently used stickers. 
#[derive(Debug, Clone)]
pub struct TGRemoveRecentSticker {
  ///  Pass true to remove the sticker from the list of stickers recently attached to photo or video files; pass false to remove the sticker from the list of recently sent stickers. 
  is_attached: Option<bool>,
  ///  Sticker file to delete. 
  sticker: Option<Box<InputFile>>,
  
}

impl TDFB for TGRemoveRecentSticker {}

impl AsRef<TGRemoveRecentSticker> for TGRemoveRecentSticker {
  fn as_ref(&self) -> &TGRemoveRecentSticker { self }
}

impl TGRemoveRecentSticker {

  pub fn new() -> Self {
    Self {
      is_attached: None,
      sticker: None,
      
    }
  }

  
  pub fn is_attached(&mut self, is_attached: bool) -> &mut Self { self.is_attached = Some(is_attached); self }
  


  
  // [sticker] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _sticker(&mut self, sticker: Box<InputFile>) -> &mut Self { self.sticker = Some(sticker); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> RemoveRecentSticker {
    RemoveRecentSticker::builder()
      .is_attached(self.is_attached.clone())
      .sticker(self.sticker.clone())
      
      .build()
  }
}


///  Removes an animation from the list of saved animations. 
#[derive(Debug, Clone)]
pub struct TGRemoveSavedAnimation {
  ///  Animation file to be removed. 
  animation: Option<Box<InputFile>>,
  
}

impl TDFB for TGRemoveSavedAnimation {}

impl AsRef<TGRemoveSavedAnimation> for TGRemoveSavedAnimation {
  fn as_ref(&self) -> &TGRemoveSavedAnimation { self }
}

impl TGRemoveSavedAnimation {

  pub fn new() -> Self {
    Self {
      animation: None,
      
    }
  }

  


  
  // [animation] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _animation(&mut self, animation: Box<InputFile>) -> &mut Self { self.animation = Some(animation); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> RemoveSavedAnimation {
    RemoveSavedAnimation::builder()
      .animation(self.animation.clone())
      
      .build()
  }
}


///  Removes a sticker from the set to which it belongs; for bots only. The sticker set must have been created by the bot. 
#[derive(Debug, Clone)]
pub struct TGRemoveStickerFromSet {
  ///  Sticker. 
  sticker: Option<Box<InputFile>>,
  
}

impl TDFB for TGRemoveStickerFromSet {}

impl AsRef<TGRemoveStickerFromSet> for TGRemoveStickerFromSet {
  fn as_ref(&self) -> &TGRemoveStickerFromSet { self }
}

impl TGRemoveStickerFromSet {

  pub fn new() -> Self {
    Self {
      sticker: None,
      
    }
  }

  


  
  // [sticker] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _sticker(&mut self, sticker: Box<InputFile>) -> &mut Self { self.sticker = Some(sticker); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> RemoveStickerFromSet {
    RemoveStickerFromSet::builder()
      .sticker(self.sticker.clone())
      
      .build()
  }
}


///  Removes a chat from the list of frequently used chats. Supported only if the chat info database is enabled. 
#[derive(Debug, Clone)]
pub struct TGRemoveTopChat {
  ///  Category of frequently used chats. 
  category: Option<Box<TopChatCategory>>,
  ///  Chat identifier. 
  chat_id: Option<i64>,
  
}

impl TDFB for TGRemoveTopChat {}

impl AsRef<TGRemoveTopChat> for TGRemoveTopChat {
  fn as_ref(&self) -> &TGRemoveTopChat { self }
}

impl TGRemoveTopChat {

  pub fn new() -> Self {
    Self {
      category: None,
      chat_id: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  


  
  // [category] type is [Box<TopChatCategory>], is not support, need add manully.
  #[doc(hidden)] pub fn _category(&mut self, category: Box<TopChatCategory>) -> &mut Self { self.category = Some(category); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> RemoveTopChat {
    RemoveTopChat::builder()
      .category(self.category.clone())
      .chat_id(self.chat_id.clone())
      
      .build()
  }
}


///  Changes the order of installed sticker sets. 
#[derive(Debug, Clone)]
pub struct TGReorderInstalledStickerSets {
  ///  Pass true to change the order of mask sticker sets; pass false to change the order of ordinary sticker sets. 
  is_masks: Option<bool>,
  ///  Identifiers of installed sticker sets in the new correct order. 
  sticker_set_ids: Option<Vec<i64>>,
  
}

impl TDFB for TGReorderInstalledStickerSets {}

impl AsRef<TGReorderInstalledStickerSets> for TGReorderInstalledStickerSets {
  fn as_ref(&self) -> &TGReorderInstalledStickerSets { self }
}

impl TGReorderInstalledStickerSets {

  pub fn new() -> Self {
    Self {
      is_masks: None,
      sticker_set_ids: None,
      
    }
  }

  
  pub fn is_masks(&mut self, is_masks: bool) -> &mut Self { self.is_masks = Some(is_masks); self }
  
  pub fn sticker_set_ids(&mut self, sticker_set_ids: Vec<i64>) -> &mut Self { self.sticker_set_ids = Some(sticker_set_ids); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> ReorderInstalledStickerSets {
    ReorderInstalledStickerSets::builder()
      .is_masks(self.is_masks.clone())
      .sticker_set_ids(self.sticker_set_ids.clone())
      
      .build()
  }
}


///  Reports a chat to the Telegram moderators. Supported only for supergroups, channels, or private chats with bots, since other chats can't be checked by moderators. 
#[derive(Debug, Clone)]
pub struct TGReportChat {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  ///  The reason for reporting the chat. 
  reason: Option<Box<ChatReportReason>>,
  ///  Identifiers of reported messages, if any. 
  message_ids: Option<Vec<i64>>,
  
}

impl TDFB for TGReportChat {}

impl AsRef<TGReportChat> for TGReportChat {
  fn as_ref(&self) -> &TGReportChat { self }
}

impl TGReportChat {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      reason: None,
      message_ids: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self { self.message_ids = Some(message_ids); self }
  


  
  // [reason] type is [Box<ChatReportReason>], is not support, need add manully.
  #[doc(hidden)] pub fn _reason(&mut self, reason: Box<ChatReportReason>) -> &mut Self { self.reason = Some(reason); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> ReportChat {
    ReportChat::builder()
      .chat_id(self.chat_id.clone())
      .reason(self.reason.clone())
      .message_ids(self.message_ids.clone())
      
      .build()
  }
}


///  Reports some messages from a user in a supergroup as spam; requires administrator rights in the supergroup. 
#[derive(Debug, Clone)]
pub struct TGReportSupergroupSpam {
  ///  Supergroup identifier. 
  supergroup_id: Option<i32>,
  ///  User identifier. 
  user_id: Option<i32>,
  ///  Identifiers of messages sent in the supergroup by the user. This list must be non-empty. 
  message_ids: Option<Vec<i64>>,
  
}

impl TDFB for TGReportSupergroupSpam {}

impl AsRef<TGReportSupergroupSpam> for TGReportSupergroupSpam {
  fn as_ref(&self) -> &TGReportSupergroupSpam { self }
}

impl TGReportSupergroupSpam {

  pub fn new() -> Self {
    Self {
      supergroup_id: None,
      user_id: None,
      message_ids: None,
      
    }
  }

  
  pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self { self.supergroup_id = Some(supergroup_id); self }
  
  pub fn user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self { self.message_ids = Some(message_ids); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> ReportSupergroupSpam {
    ReportSupergroupSpam::builder()
      .supergroup_id(self.supergroup_id.clone())
      .user_id(self.user_id.clone())
      .message_ids(self.message_ids.clone())
      
      .build()
  }
}


///  Requests to send a password recovery code to an email address that was previously set up. Works only when the current authorization state is  
#[derive(Debug, Clone)]
pub struct TGRequestAuthenticationPasswordRecovery {
  
}

impl TDFB for TGRequestAuthenticationPasswordRecovery {}

impl AsRef<TGRequestAuthenticationPasswordRecovery> for TGRequestAuthenticationPasswordRecovery {
  fn as_ref(&self) -> &TGRequestAuthenticationPasswordRecovery { self }
}

impl TGRequestAuthenticationPasswordRecovery {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> RequestAuthenticationPasswordRecovery {
    RequestAuthenticationPasswordRecovery::builder()
      
      .build()
  }
}


///  Requests to send a password recovery code to an email address that was previously set up. 
#[derive(Debug, Clone)]
pub struct TGRequestPasswordRecovery {
  
}

impl TDFB for TGRequestPasswordRecovery {}

impl AsRef<TGRequestPasswordRecovery> for TGRequestPasswordRecovery {
  fn as_ref(&self) -> &TGRequestPasswordRecovery { self }
}

impl TGRequestPasswordRecovery {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> RequestPasswordRecovery {
    RequestPasswordRecovery::builder()
      
      .build()
  }
}


///  Re-sends an authentication code to the user. Works only when the current authorization state is  
#[derive(Debug, Clone)]
pub struct TGResendAuthenticationCode {
  
}

impl TDFB for TGResendAuthenticationCode {}

impl AsRef<TGResendAuthenticationCode> for TGResendAuthenticationCode {
  fn as_ref(&self) -> &TGResendAuthenticationCode { self }
}

impl TGResendAuthenticationCode {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> ResendAuthenticationCode {
    ResendAuthenticationCode::builder()
      
      .build()
  }
}


///  Re-sends the authentication code sent to confirm a new phone number for the user. Works only if the previously received  
#[derive(Debug, Clone)]
pub struct TGResendChangePhoneNumberCode {
  
}

impl TDFB for TGResendChangePhoneNumberCode {}

impl AsRef<TGResendChangePhoneNumberCode> for TGResendChangePhoneNumberCode {
  fn as_ref(&self) -> &TGResendChangePhoneNumberCode { self }
}

impl TGResendChangePhoneNumberCode {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> ResendChangePhoneNumberCode {
    ResendChangePhoneNumberCode::builder()
      
      .build()
  }
}


///  Re-sends the code to verify an email address to be added to a user's Telegram Passport. 
#[derive(Debug, Clone)]
pub struct TGResendEmailAddressVerificationCode {
  
}

impl TDFB for TGResendEmailAddressVerificationCode {}

impl AsRef<TGResendEmailAddressVerificationCode> for TGResendEmailAddressVerificationCode {
  fn as_ref(&self) -> &TGResendEmailAddressVerificationCode { self }
}

impl TGResendEmailAddressVerificationCode {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> ResendEmailAddressVerificationCode {
    ResendEmailAddressVerificationCode::builder()
      
      .build()
  }
}


///  Resends phone number confirmation code. 
#[derive(Debug, Clone)]
pub struct TGResendPhoneNumberConfirmationCode {
  
}

impl TDFB for TGResendPhoneNumberConfirmationCode {}

impl AsRef<TGResendPhoneNumberConfirmationCode> for TGResendPhoneNumberConfirmationCode {
  fn as_ref(&self) -> &TGResendPhoneNumberConfirmationCode { self }
}

impl TGResendPhoneNumberConfirmationCode {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> ResendPhoneNumberConfirmationCode {
    ResendPhoneNumberConfirmationCode::builder()
      
      .build()
  }
}


///  Re-sends the code to verify a phone number to be added to a user's Telegram Passport. 
#[derive(Debug, Clone)]
pub struct TGResendPhoneNumberVerificationCode {
  
}

impl TDFB for TGResendPhoneNumberVerificationCode {}

impl AsRef<TGResendPhoneNumberVerificationCode> for TGResendPhoneNumberVerificationCode {
  fn as_ref(&self) -> &TGResendPhoneNumberVerificationCode { self }
}

impl TGResendPhoneNumberVerificationCode {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> ResendPhoneNumberVerificationCode {
    ResendPhoneNumberVerificationCode::builder()
      
      .build()
  }
}


///  Resends the 2-step verification recovery email address verification code. 
#[derive(Debug, Clone)]
pub struct TGResendRecoveryEmailAddressCode {
  
}

impl TDFB for TGResendRecoveryEmailAddressCode {}

impl AsRef<TGResendRecoveryEmailAddressCode> for TGResendRecoveryEmailAddressCode {
  fn as_ref(&self) -> &TGResendRecoveryEmailAddressCode { self }
}

impl TGResendRecoveryEmailAddressCode {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> ResendRecoveryEmailAddressCode {
    ResendRecoveryEmailAddressCode::builder()
      
      .build()
  }
}


///  Resets all notification settings to their default values. By default, all chats are unmuted, the sound is set to "default" and message previews are shown. 
#[derive(Debug, Clone)]
pub struct TGResetAllNotificationSettings {
  
}

impl TDFB for TGResetAllNotificationSettings {}

impl AsRef<TGResetAllNotificationSettings> for TGResetAllNotificationSettings {
  fn as_ref(&self) -> &TGResetAllNotificationSettings { self }
}

impl TGResetAllNotificationSettings {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> ResetAllNotificationSettings {
    ResetAllNotificationSettings::builder()
      
      .build()
  }
}


///  Resets all network data usage statistics to zero. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGResetNetworkStatistics {
  
}

impl TDFB for TGResetNetworkStatistics {}

impl AsRef<TGResetNetworkStatistics> for TGResetNetworkStatistics {
  fn as_ref(&self) -> &TGResetNetworkStatistics { self }
}

impl TGResetNetworkStatistics {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> ResetNetworkStatistics {
    ResetNetworkStatistics::builder()
      
      .build()
  }
}


///  Searches for call messages. Returns the results in reverse chronological order (i. e., in order of decreasing message_id). For optimal performance the number of returned messages is chosen by the library. 
#[derive(Debug, Clone)]
pub struct TGSearchCallMessages {
  ///  Identifier of the message from which to search; use 0 to get results from the last message. 
  from_message_id: Option<i64>,
  ///  The maximum number of messages to be returned; up to 100. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached. 
  limit: Option<i32>,
  ///  If true, returns only messages with missed calls. 
  only_missed: Option<bool>,
  
}

impl TDFB for TGSearchCallMessages {}

impl AsRef<TGSearchCallMessages> for TGSearchCallMessages {
  fn as_ref(&self) -> &TGSearchCallMessages { self }
}

impl TGSearchCallMessages {

  pub fn new() -> Self {
    Self {
      from_message_id: None,
      limit: None,
      only_missed: None,
      
    }
  }

  
  pub fn from_message_id(&mut self, from_message_id: i64) -> &mut Self { self.from_message_id = Some(from_message_id); self }
  
  pub fn limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  
  pub fn only_missed(&mut self, only_missed: bool) -> &mut Self { self.only_missed = Some(only_missed); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SearchCallMessages {
    SearchCallMessages::builder()
      .from_message_id(self.from_message_id.clone())
      .limit(self.limit.clone())
      .only_missed(self.only_missed.clone())
      
      .build()
  }
}


///  Searches for a specified query in the first name, last name and username of the members of a specified chat. Requires administrator rights in channels. 
#[derive(Debug, Clone)]
pub struct TGSearchChatMembers {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  ///  Query to search for. 
  query: Option<String>,
  ///  The maximum number of users to be returned. 
  limit: Option<i32>,
  ///  The type of users to return. By default, chatMembersFilterMembers. 
  filter: Option<Box<ChatMembersFilter>>,
  
}

impl TDFB for TGSearchChatMembers {}

impl AsRef<TGSearchChatMembers> for TGSearchChatMembers {
  fn as_ref(&self) -> &TGSearchChatMembers { self }
}

impl TGSearchChatMembers {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      query: None,
      limit: None,
      filter: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn query<S: AsRef<str>>(&mut self, query: S) -> &mut Self { self.query = Some(query.as_ref().to_string()); self }
  
  pub fn limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  


  
  // [filter] type is [Box<ChatMembersFilter>], is not support, need add manully.
  #[doc(hidden)] pub fn _filter(&mut self, filter: Box<ChatMembersFilter>) -> &mut Self { self.filter = Some(filter); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> SearchChatMembers {
    SearchChatMembers::builder()
      .chat_id(self.chat_id.clone())
      .query(self.query.clone())
      .limit(self.limit.clone())
      .filter(self.filter.clone())
      
      .build()
  }
}


///  Searches for messages with given words in the chat. Returns the results in reverse chronological order, i.e. in order of decreasing message_id. Cannot be used in secret chats with a non-empty query ( 
#[derive(Debug, Clone)]
pub struct TGSearchChatMessages {
  ///  Identifier of the chat in which to search messages. 
  chat_id: Option<i64>,
  ///  Query to search for. 
  query: Option<String>,
  ///  If not 0, only messages sent by the specified user will be returned. Not supported in secret chats. 
  sender_user_id: Option<i32>,
  ///  Identifier of the message starting from which history must be fetched; use 0 to get results from the last message. 
  from_message_id: Option<i64>,
  ///  Specify 0 to get results from exactly the from_message_id or a negative offset to get the specified message and some newer messages. 
  offset: Option<i32>,
  ///  The maximum number of messages to be returned; must be positive and can't be greater than 100. If the offset is negative, the limit must be greater than -offset. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached. 
  limit: Option<i32>,
  ///  Filter for message content in the search results. 
  filter: Option<Box<SearchMessagesFilter>>,
  
}

impl TDFB for TGSearchChatMessages {}

impl AsRef<TGSearchChatMessages> for TGSearchChatMessages {
  fn as_ref(&self) -> &TGSearchChatMessages { self }
}

impl TGSearchChatMessages {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      query: None,
      sender_user_id: None,
      from_message_id: None,
      offset: None,
      limit: None,
      filter: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn query<S: AsRef<str>>(&mut self, query: S) -> &mut Self { self.query = Some(query.as_ref().to_string()); self }
  
  pub fn sender_user_id(&mut self, sender_user_id: i32) -> &mut Self { self.sender_user_id = Some(sender_user_id); self }
  
  pub fn from_message_id(&mut self, from_message_id: i64) -> &mut Self { self.from_message_id = Some(from_message_id); self }
  
  pub fn offset(&mut self, offset: i32) -> &mut Self { self.offset = Some(offset); self }
  
  pub fn limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  


  
  // [filter] type is [Box<SearchMessagesFilter>], is not support, need add manully.
  #[doc(hidden)] pub fn _filter(&mut self, filter: Box<SearchMessagesFilter>) -> &mut Self { self.filter = Some(filter); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> SearchChatMessages {
    SearchChatMessages::builder()
      .chat_id(self.chat_id.clone())
      .query(self.query.clone())
      .sender_user_id(self.sender_user_id.clone())
      .from_message_id(self.from_message_id.clone())
      .offset(self.offset.clone())
      .limit(self.limit.clone())
      .filter(self.filter.clone())
      
      .build()
  }
}


///  Returns information about the recent locations of chat members that were sent to the chat. Returns up to 1 location message per user. 
#[derive(Debug, Clone)]
pub struct TGSearchChatRecentLocationMessages {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  ///  Maximum number of messages to be returned. 
  limit: Option<i32>,
  
}

impl TDFB for TGSearchChatRecentLocationMessages {}

impl AsRef<TGSearchChatRecentLocationMessages> for TGSearchChatRecentLocationMessages {
  fn as_ref(&self) -> &TGSearchChatRecentLocationMessages { self }
}

impl TGSearchChatRecentLocationMessages {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      limit: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SearchChatRecentLocationMessages {
    SearchChatRecentLocationMessages::builder()
      .chat_id(self.chat_id.clone())
      .limit(self.limit.clone())
      
      .build()
  }
}


///  Searches for the specified query in the title and username of already known chats, this is an offline request. Returns chats in the order seen in the chat list. 
#[derive(Debug, Clone)]
pub struct TGSearchChats {
  ///  Query to search for. If the query is empty, returns up to 20 recently found chats. 
  query: Option<String>,
  ///  Maximum number of chats to be returned. 
  limit: Option<i32>,
  
}

impl TDFB for TGSearchChats {}

impl AsRef<TGSearchChats> for TGSearchChats {
  fn as_ref(&self) -> &TGSearchChats { self }
}

impl TGSearchChats {

  pub fn new() -> Self {
    Self {
      query: None,
      limit: None,
      
    }
  }

  
  pub fn query<S: AsRef<str>>(&mut self, query: S) -> &mut Self { self.query = Some(query.as_ref().to_string()); self }
  
  pub fn limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SearchChats {
    SearchChats::builder()
      .query(self.query.clone())
      .limit(self.limit.clone())
      
      .build()
  }
}


///  Searches for the specified query in the title and username of already known chats via request to the server. Returns chats in the order seen in the chat list. 
#[derive(Debug, Clone)]
pub struct TGSearchChatsOnServer {
  ///  Query to search for. 
  query: Option<String>,
  ///  Maximum number of chats to be returned. 
  limit: Option<i32>,
  
}

impl TDFB for TGSearchChatsOnServer {}

impl AsRef<TGSearchChatsOnServer> for TGSearchChatsOnServer {
  fn as_ref(&self) -> &TGSearchChatsOnServer { self }
}

impl TGSearchChatsOnServer {

  pub fn new() -> Self {
    Self {
      query: None,
      limit: None,
      
    }
  }

  
  pub fn query<S: AsRef<str>>(&mut self, query: S) -> &mut Self { self.query = Some(query.as_ref().to_string()); self }
  
  pub fn limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SearchChatsOnServer {
    SearchChatsOnServer::builder()
      .query(self.query.clone())
      .limit(self.limit.clone())
      
      .build()
  }
}


///  Searches for the specified query in the first names, last names and usernames of the known user contacts. 
#[derive(Debug, Clone)]
pub struct TGSearchContacts {
  ///  Query to search for; may be empty to return all contacts. 
  query: Option<String>,
  ///  Maximum number of users to be returned. 
  limit: Option<i32>,
  
}

impl TDFB for TGSearchContacts {}

impl AsRef<TGSearchContacts> for TGSearchContacts {
  fn as_ref(&self) -> &TGSearchContacts { self }
}

impl TGSearchContacts {

  pub fn new() -> Self {
    Self {
      query: None,
      limit: None,
      
    }
  }

  
  pub fn query<S: AsRef<str>>(&mut self, query: S) -> &mut Self { self.query = Some(query.as_ref().to_string()); self }
  
  pub fn limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SearchContacts {
    SearchContacts::builder()
      .query(self.query.clone())
      .limit(self.limit.clone())
      
      .build()
  }
}


///  Searches for recently used hashtags by their prefix. 
#[derive(Debug, Clone)]
pub struct TGSearchHashtags {
  ///  Hashtag prefix to search for. 
  prefix: Option<String>,
  ///  Maximum number of hashtags to be returned. 
  limit: Option<i32>,
  
}

impl TDFB for TGSearchHashtags {}

impl AsRef<TGSearchHashtags> for TGSearchHashtags {
  fn as_ref(&self) -> &TGSearchHashtags { self }
}

impl TGSearchHashtags {

  pub fn new() -> Self {
    Self {
      prefix: None,
      limit: None,
      
    }
  }

  
  pub fn prefix<S: AsRef<str>>(&mut self, prefix: S) -> &mut Self { self.prefix = Some(prefix.as_ref().to_string()); self }
  
  pub fn limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SearchHashtags {
    SearchHashtags::builder()
      .prefix(self.prefix.clone())
      .limit(self.limit.clone())
      
      .build()
  }
}


///  Searches for installed sticker sets by looking for specified query in their title and name. 
#[derive(Debug, Clone)]
pub struct TGSearchInstalledStickerSets {
  ///  Pass true to return mask sticker sets; pass false to return ordinary sticker sets. 
  is_masks: Option<bool>,
  ///  Query to search for. 
  query: Option<String>,
  ///  Maximum number of sticker sets to return. 
  limit: Option<i32>,
  
}

impl TDFB for TGSearchInstalledStickerSets {}

impl AsRef<TGSearchInstalledStickerSets> for TGSearchInstalledStickerSets {
  fn as_ref(&self) -> &TGSearchInstalledStickerSets { self }
}

impl TGSearchInstalledStickerSets {

  pub fn new() -> Self {
    Self {
      is_masks: None,
      query: None,
      limit: None,
      
    }
  }

  
  pub fn is_masks(&mut self, is_masks: bool) -> &mut Self { self.is_masks = Some(is_masks); self }
  
  pub fn query<S: AsRef<str>>(&mut self, query: S) -> &mut Self { self.query = Some(query.as_ref().to_string()); self }
  
  pub fn limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SearchInstalledStickerSets {
    SearchInstalledStickerSets::builder()
      .is_masks(self.is_masks.clone())
      .query(self.query.clone())
      .limit(self.limit.clone())
      
      .build()
  }
}


///  Searches for messages in all chats except secret chats. Returns the results in reverse chronological order (i.e., in order of decreasing (date, chat_id, message_id)). For optimal performance the number of returned messages is chosen by the library. 
#[derive(Debug, Clone)]
pub struct TGSearchMessages {
  ///  Query to search for. 
  query: Option<String>,
  ///  The date of the message starting from which the results should be fetched. Use 0 or any date in the future to get results from the last message. 
  offset_date: Option<i32>,
  ///  The chat identifier of the last found message, or 0 for the first request. 
  offset_chat_id: Option<i64>,
  ///  The message identifier of the last found message, or 0 for the first request. 
  offset_message_id: Option<i64>,
  ///  The maximum number of messages to be returned, up to 100. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached. 
  limit: Option<i32>,
  
}

impl TDFB for TGSearchMessages {}

impl AsRef<TGSearchMessages> for TGSearchMessages {
  fn as_ref(&self) -> &TGSearchMessages { self }
}

impl TGSearchMessages {

  pub fn new() -> Self {
    Self {
      query: None,
      offset_date: None,
      offset_chat_id: None,
      offset_message_id: None,
      limit: None,
      
    }
  }

  
  pub fn query<S: AsRef<str>>(&mut self, query: S) -> &mut Self { self.query = Some(query.as_ref().to_string()); self }
  
  pub fn offset_date(&mut self, offset_date: i32) -> &mut Self { self.offset_date = Some(offset_date); self }
  
  pub fn offset_chat_id(&mut self, offset_chat_id: i64) -> &mut Self { self.offset_chat_id = Some(offset_chat_id); self }
  
  pub fn offset_message_id(&mut self, offset_message_id: i64) -> &mut Self { self.offset_message_id = Some(offset_message_id); self }
  
  pub fn limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SearchMessages {
    SearchMessages::builder()
      .query(self.query.clone())
      .offset_date(self.offset_date.clone())
      .offset_chat_id(self.offset_chat_id.clone())
      .offset_message_id(self.offset_message_id.clone())
      .limit(self.limit.clone())
      
      .build()
  }
}


///  Searches a public chat by its username. Currently only private chats, supergroups and channels can be public. Returns the chat if found; otherwise an error is returned. 
#[derive(Debug, Clone)]
pub struct TGSearchPublicChat {
  ///  Username to be resolved. 
  username: Option<String>,
  
}

impl TDFB for TGSearchPublicChat {}

impl AsRef<TGSearchPublicChat> for TGSearchPublicChat {
  fn as_ref(&self) -> &TGSearchPublicChat { self }
}

impl TGSearchPublicChat {

  pub fn new() -> Self {
    Self {
      username: None,
      
    }
  }

  
  pub fn username<S: AsRef<str>>(&mut self, username: S) -> &mut Self { self.username = Some(username.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SearchPublicChat {
    SearchPublicChat::builder()
      .username(self.username.clone())
      
      .build()
  }
}


///  Searches public chats by looking for specified query in their username and title. Currently only private chats, supergroups and channels can be public. Returns a meaningful number of results. Returns nothing if the length of the searched username prefix is less than 5. Excludes private chats with contacts and chats from the chat list from the results. 
#[derive(Debug, Clone)]
pub struct TGSearchPublicChats {
  ///  Query to search for. 
  query: Option<String>,
  
}

impl TDFB for TGSearchPublicChats {}

impl AsRef<TGSearchPublicChats> for TGSearchPublicChats {
  fn as_ref(&self) -> &TGSearchPublicChats { self }
}

impl TGSearchPublicChats {

  pub fn new() -> Self {
    Self {
      query: None,
      
    }
  }

  
  pub fn query<S: AsRef<str>>(&mut self, query: S) -> &mut Self { self.query = Some(query.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SearchPublicChats {
    SearchPublicChats::builder()
      .query(self.query.clone())
      
      .build()
  }
}


///  Searches for messages in secret chats. Returns the results in reverse chronological order. For optimal performance the number of returned messages is chosen by the library. 
#[derive(Debug, Clone)]
pub struct TGSearchSecretMessages {
  ///  Identifier of the chat in which to search. Specify 0 to search in all secret chats. 
  chat_id: Option<i64>,
  ///  Query to search for. If empty, searchChatMessages should be used instead. 
  query: Option<String>,
  ///  The identifier from the result of a previous request, use 0 to get results from the last message. 
  from_search_id: Option<i64>,
  ///  Maximum number of messages to be returned; up to 100. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached. 
  limit: Option<i32>,
  ///  A filter for the content of messages in the search results. 
  filter: Option<Box<SearchMessagesFilter>>,
  
}

impl TDFB for TGSearchSecretMessages {}

impl AsRef<TGSearchSecretMessages> for TGSearchSecretMessages {
  fn as_ref(&self) -> &TGSearchSecretMessages { self }
}

impl TGSearchSecretMessages {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      query: None,
      from_search_id: None,
      limit: None,
      filter: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn query<S: AsRef<str>>(&mut self, query: S) -> &mut Self { self.query = Some(query.as_ref().to_string()); self }
  
  pub fn from_search_id(&mut self, from_search_id: i64) -> &mut Self { self.from_search_id = Some(from_search_id); self }
  
  pub fn limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  


  
  // [filter] type is [Box<SearchMessagesFilter>], is not support, need add manully.
  #[doc(hidden)] pub fn _filter(&mut self, filter: Box<SearchMessagesFilter>) -> &mut Self { self.filter = Some(filter); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> SearchSecretMessages {
    SearchSecretMessages::builder()
      .chat_id(self.chat_id.clone())
      .query(self.query.clone())
      .from_search_id(self.from_search_id.clone())
      .limit(self.limit.clone())
      .filter(self.filter.clone())
      
      .build()
  }
}


///  Searches for stickers from public sticker sets that correspond to a given emoji. 
#[derive(Debug, Clone)]
pub struct TGSearchStickers {
  ///  String representation of emoji; must be non-empty. 
  emoji: Option<String>,
  ///  Maximum number of stickers to be returned. 
  limit: Option<i32>,
  
}

impl TDFB for TGSearchStickers {}

impl AsRef<TGSearchStickers> for TGSearchStickers {
  fn as_ref(&self) -> &TGSearchStickers { self }
}

impl TGSearchStickers {

  pub fn new() -> Self {
    Self {
      emoji: None,
      limit: None,
      
    }
  }

  
  pub fn emoji<S: AsRef<str>>(&mut self, emoji: S) -> &mut Self { self.emoji = Some(emoji.as_ref().to_string()); self }
  
  pub fn limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SearchStickers {
    SearchStickers::builder()
      .emoji(self.emoji.clone())
      .limit(self.limit.clone())
      
      .build()
  }
}


///  Searches for a sticker set by its name. 
#[derive(Debug, Clone)]
pub struct TGSearchStickerSet {
  ///  Name of the sticker set. 
  name: Option<String>,
  
}

impl TDFB for TGSearchStickerSet {}

impl AsRef<TGSearchStickerSet> for TGSearchStickerSet {
  fn as_ref(&self) -> &TGSearchStickerSet { self }
}

impl TGSearchStickerSet {

  pub fn new() -> Self {
    Self {
      name: None,
      
    }
  }

  
  pub fn name<S: AsRef<str>>(&mut self, name: S) -> &mut Self { self.name = Some(name.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SearchStickerSet {
    SearchStickerSet::builder()
      .name(self.name.clone())
      
      .build()
  }
}


///  Searches for ordinary sticker sets by looking for specified query in their title and name. Excludes installed sticker sets from the results. 
#[derive(Debug, Clone)]
pub struct TGSearchStickerSets {
  ///  Query to search for. 
  query: Option<String>,
  
}

impl TDFB for TGSearchStickerSets {}

impl AsRef<TGSearchStickerSets> for TGSearchStickerSets {
  fn as_ref(&self) -> &TGSearchStickerSets { self }
}

impl TGSearchStickerSets {

  pub fn new() -> Self {
    Self {
      query: None,
      
    }
  }

  
  pub fn query<S: AsRef<str>>(&mut self, query: S) -> &mut Self { self.query = Some(query.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SearchStickerSets {
    SearchStickerSets::builder()
      .query(self.query.clone())
      
      .build()
  }
}


///  Invites a bot to a chat (if it is not yet a member) and sends it the /start command. Bots can't be invited to a private chat other than the chat with the bot. Bots can't be invited to channels (although they can be added as admins) and secret chats. Returns the sent message. 
#[derive(Debug, Clone)]
pub struct TGSendBotStartMessage {
  ///  Identifier of the bot. 
  bot_user_id: Option<i32>,
  ///  Identifier of the target chat. 
  chat_id: Option<i64>,
  ///  A hidden parameter sent to the bot for deep linking purposes (https://api.telegram.org/bots#deep-linking). 
  parameter: Option<String>,
  
}

impl TDFB for TGSendBotStartMessage {}

impl AsRef<TGSendBotStartMessage> for TGSendBotStartMessage {
  fn as_ref(&self) -> &TGSendBotStartMessage { self }
}

impl TGSendBotStartMessage {

  pub fn new() -> Self {
    Self {
      bot_user_id: None,
      chat_id: None,
      parameter: None,
      
    }
  }

  
  pub fn bot_user_id(&mut self, bot_user_id: i32) -> &mut Self { self.bot_user_id = Some(bot_user_id); self }
  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn parameter<S: AsRef<str>>(&mut self, parameter: S) -> &mut Self { self.parameter = Some(parameter.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SendBotStartMessage {
    SendBotStartMessage::builder()
      .bot_user_id(self.bot_user_id.clone())
      .chat_id(self.chat_id.clone())
      .parameter(self.parameter.clone())
      
      .build()
  }
}


///  Sends debug information for a call. 
#[derive(Debug, Clone)]
pub struct TGSendCallDebugInformation {
  ///  Call identifier. 
  call_id: Option<i32>,
  ///  Debug information in application-specific format. 
  debug_information: Option<String>,
  
}

impl TDFB for TGSendCallDebugInformation {}

impl AsRef<TGSendCallDebugInformation> for TGSendCallDebugInformation {
  fn as_ref(&self) -> &TGSendCallDebugInformation { self }
}

impl TGSendCallDebugInformation {

  pub fn new() -> Self {
    Self {
      call_id: None,
      debug_information: None,
      
    }
  }

  
  pub fn call_id(&mut self, call_id: i32) -> &mut Self { self.call_id = Some(call_id); self }
  
  pub fn debug_information<S: AsRef<str>>(&mut self, debug_information: S) -> &mut Self { self.debug_information = Some(debug_information.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SendCallDebugInformation {
    SendCallDebugInformation::builder()
      .call_id(self.call_id.clone())
      .debug_information(self.debug_information.clone())
      
      .build()
  }
}


///  Sends a call rating. 
#[derive(Debug, Clone)]
pub struct TGSendCallRating {
  ///  Call identifier. 
  call_id: Option<i32>,
  ///  Call rating; 1-5. 
  rating: Option<i32>,
  ///  An optional user comment if the rating is less than 5. 
  comment: Option<String>,
  
}

impl TDFB for TGSendCallRating {}

impl AsRef<TGSendCallRating> for TGSendCallRating {
  fn as_ref(&self) -> &TGSendCallRating { self }
}

impl TGSendCallRating {

  pub fn new() -> Self {
    Self {
      call_id: None,
      rating: None,
      comment: None,
      
    }
  }

  
  pub fn call_id(&mut self, call_id: i32) -> &mut Self { self.call_id = Some(call_id); self }
  
  pub fn rating(&mut self, rating: i32) -> &mut Self { self.rating = Some(rating); self }
  
  pub fn comment<S: AsRef<str>>(&mut self, comment: S) -> &mut Self { self.comment = Some(comment.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SendCallRating {
    SendCallRating::builder()
      .call_id(self.call_id.clone())
      .rating(self.rating.clone())
      .comment(self.comment.clone())
      
      .build()
  }
}


///  Sends a notification about user activity in a chat. 
#[derive(Debug, Clone)]
pub struct TGSendChatAction {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  ///  The action description. 
  action: Option<Box<ChatAction>>,
  
}

impl TDFB for TGSendChatAction {}

impl AsRef<TGSendChatAction> for TGSendChatAction {
  fn as_ref(&self) -> &TGSendChatAction { self }
}

impl TGSendChatAction {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      action: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  


  
  // [action] type is [Box<ChatAction>], is not support, need add manully.
  #[doc(hidden)] pub fn _action(&mut self, action: Box<ChatAction>) -> &mut Self { self.action = Some(action); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> SendChatAction {
    SendChatAction::builder()
      .chat_id(self.chat_id.clone())
      .action(self.action.clone())
      
      .build()
  }
}


///  Sends a notification about a screenshot taken in a chat. Supported only in private and secret chats. 
#[derive(Debug, Clone)]
pub struct TGSendChatScreenshotTakenNotification {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  
}

impl TDFB for TGSendChatScreenshotTakenNotification {}

impl AsRef<TGSendChatScreenshotTakenNotification> for TGSendChatScreenshotTakenNotification {
  fn as_ref(&self) -> &TGSendChatScreenshotTakenNotification { self }
}

impl TGSendChatScreenshotTakenNotification {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SendChatScreenshotTakenNotification {
    SendChatScreenshotTakenNotification::builder()
      .chat_id(self.chat_id.clone())
      
      .build()
  }
}


///  Changes the current TTL setting (sets a new self-destruct timer) in a secret chat and sends the corresponding message. 
#[derive(Debug, Clone)]
pub struct TGSendChatSetTtlMessage {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  ///  New TTL value, in seconds. 
  ttl: Option<i32>,
  
}

impl TDFB for TGSendChatSetTtlMessage {}

impl AsRef<TGSendChatSetTtlMessage> for TGSendChatSetTtlMessage {
  fn as_ref(&self) -> &TGSendChatSetTtlMessage { self }
}

impl TGSendChatSetTtlMessage {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      ttl: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn ttl(&mut self, ttl: i32) -> &mut Self { self.ttl = Some(ttl); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SendChatSetTtlMessage {
    SendChatSetTtlMessage::builder()
      .chat_id(self.chat_id.clone())
      .ttl(self.ttl.clone())
      
      .build()
  }
}


///  Sends a custom request; for bots only. 
#[derive(Debug, Clone)]
pub struct TGSendCustomRequest {
  ///  The method name. 
  method: Option<String>,
  ///  JSON-serialized method parameters. 
  parameters: Option<String>,
  
}

impl TDFB for TGSendCustomRequest {}

impl AsRef<TGSendCustomRequest> for TGSendCustomRequest {
  fn as_ref(&self) -> &TGSendCustomRequest { self }
}

impl TGSendCustomRequest {

  pub fn new() -> Self {
    Self {
      method: None,
      parameters: None,
      
    }
  }

  
  pub fn method<S: AsRef<str>>(&mut self, method: S) -> &mut Self { self.method = Some(method.as_ref().to_string()); self }
  
  pub fn parameters<S: AsRef<str>>(&mut self, parameters: S) -> &mut Self { self.parameters = Some(parameters.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SendCustomRequest {
    SendCustomRequest::builder()
      .method(self.method.clone())
      .parameters(self.parameters.clone())
      
      .build()
  }
}


///  Sends a code to verify an email address to be added to a user's Telegram Passport. 
#[derive(Debug, Clone)]
pub struct TGSendEmailAddressVerificationCode {
  ///  Email address. 
  email_address: Option<String>,
  
}

impl TDFB for TGSendEmailAddressVerificationCode {}

impl AsRef<TGSendEmailAddressVerificationCode> for TGSendEmailAddressVerificationCode {
  fn as_ref(&self) -> &TGSendEmailAddressVerificationCode { self }
}

impl TGSendEmailAddressVerificationCode {

  pub fn new() -> Self {
    Self {
      email_address: None,
      
    }
  }

  
  pub fn email_address<S: AsRef<str>>(&mut self, email_address: S) -> &mut Self { self.email_address = Some(email_address.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SendEmailAddressVerificationCode {
    SendEmailAddressVerificationCode::builder()
      .email_address(self.email_address.clone())
      
      .build()
  }
}


///  Sends the result of an inline query as a message. Returns the sent message. Always clears a chat draft message. 
#[derive(Debug, Clone)]
pub struct TGSendInlineQueryResultMessage {
  ///  Target chat. 
  chat_id: Option<i64>,
  ///  Identifier of a message to reply to or 0. 
  reply_to_message_id: Option<i64>,
  ///  Pass true to disable notification for the message. Not supported in secret chats. 
  disable_notification: Option<bool>,
  ///  Pass true if the message is sent from background. 
  from_background: Option<bool>,
  ///  Identifier of the inline query. 
  query_id: Option<i64>,
  ///  Identifier of the inline result. 
  result_id: Option<String>,
  ///  If true, there will be no mention of a bot, via which the message is sent. Can be used only for bots GetOption("animation_search_bot_username"), GetOption("photo_search_bot_username") and GetOption("venue_search_bot_username"). 
  hide_via_bot: Option<bool>,
  
}

impl TDFB for TGSendInlineQueryResultMessage {}

impl AsRef<TGSendInlineQueryResultMessage> for TGSendInlineQueryResultMessage {
  fn as_ref(&self) -> &TGSendInlineQueryResultMessage { self }
}

impl TGSendInlineQueryResultMessage {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      reply_to_message_id: None,
      disable_notification: None,
      from_background: None,
      query_id: None,
      result_id: None,
      hide_via_bot: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn reply_to_message_id(&mut self, reply_to_message_id: i64) -> &mut Self { self.reply_to_message_id = Some(reply_to_message_id); self }
  
  pub fn disable_notification(&mut self, disable_notification: bool) -> &mut Self { self.disable_notification = Some(disable_notification); self }
  
  pub fn from_background(&mut self, from_background: bool) -> &mut Self { self.from_background = Some(from_background); self }
  
  pub fn query_id(&mut self, query_id: i64) -> &mut Self { self.query_id = Some(query_id); self }
  
  pub fn result_id<S: AsRef<str>>(&mut self, result_id: S) -> &mut Self { self.result_id = Some(result_id.as_ref().to_string()); self }
  
  pub fn hide_via_bot(&mut self, hide_via_bot: bool) -> &mut Self { self.hide_via_bot = Some(hide_via_bot); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SendInlineQueryResultMessage {
    SendInlineQueryResultMessage::builder()
      .chat_id(self.chat_id.clone())
      .reply_to_message_id(self.reply_to_message_id.clone())
      .disable_notification(self.disable_notification.clone())
      .from_background(self.from_background.clone())
      .query_id(self.query_id.clone())
      .result_id(self.result_id.clone())
      .hide_via_bot(self.hide_via_bot.clone())
      
      .build()
  }
}


///  Sends a message. Returns the sent message. 
#[derive(Debug, Clone)]
pub struct TGSendMessage {
  ///  Target chat. 
  chat_id: Option<i64>,
  ///  Identifier of the message to reply to or 0. 
  reply_to_message_id: Option<i64>,
  ///  Pass true to disable notification for the message. Not supported in secret chats. 
  disable_notification: Option<bool>,
  ///  Pass true if the message is sent from the background. 
  from_background: Option<bool>,
  ///  Markup for replying to the message; for bots only. 
  reply_markup: Option<Box<ReplyMarkup>>,
  ///  The content of the message to be sent. 
  input_message_content: Option<Box<InputMessageContent>>,
  
}

impl TDFB for TGSendMessage {}

impl AsRef<TGSendMessage> for TGSendMessage {
  fn as_ref(&self) -> &TGSendMessage { self }
}

impl TGSendMessage {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      reply_to_message_id: None,
      disable_notification: None,
      from_background: None,
      reply_markup: None,
      input_message_content: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn reply_to_message_id(&mut self, reply_to_message_id: i64) -> &mut Self { self.reply_to_message_id = Some(reply_to_message_id); self }
  
  pub fn disable_notification(&mut self, disable_notification: bool) -> &mut Self { self.disable_notification = Some(disable_notification); self }
  
  pub fn from_background(&mut self, from_background: bool) -> &mut Self { self.from_background = Some(from_background); self }
  


  
  // [reply_markup] type is [Box<ReplyMarkup>], is not support, need add manully.
  #[doc(hidden)] pub fn _reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  
  // [input_message_content] type is [Box<InputMessageContent>], is not support, need add manully.
  #[doc(hidden)] pub fn _input_message_content(&mut self, input_message_content: Box<InputMessageContent>) -> &mut Self { self.input_message_content = Some(input_message_content); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> SendMessage {
    SendMessage::builder()
      .chat_id(self.chat_id.clone())
      .reply_to_message_id(self.reply_to_message_id.clone())
      .disable_notification(self.disable_notification.clone())
      .from_background(self.from_background.clone())
      .reply_markup(self.reply_markup.clone())
      .input_message_content(self.input_message_content.clone())
      
      .build()
  }
}


///  Sends messages grouped together into an album. Currently only photo and video messages can be grouped into an album. Returns sent messages. 
#[derive(Debug, Clone)]
pub struct TGSendMessageAlbum {
  ///  Target chat. 
  chat_id: Option<i64>,
  ///  Identifier of a message to reply to or 0. 
  reply_to_message_id: Option<i64>,
  ///  Pass true to disable notification for the messages. Not supported in secret chats. 
  disable_notification: Option<bool>,
  ///  Pass true if the messages are sent from the background. 
  from_background: Option<bool>,
  ///  Contents of messages to be sent. 
  input_message_contents: Option<Vec<Box<InputMessageContent>>>,
  
}

impl TDFB for TGSendMessageAlbum {}

impl AsRef<TGSendMessageAlbum> for TGSendMessageAlbum {
  fn as_ref(&self) -> &TGSendMessageAlbum { self }
}

impl TGSendMessageAlbum {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      reply_to_message_id: None,
      disable_notification: None,
      from_background: None,
      input_message_contents: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn reply_to_message_id(&mut self, reply_to_message_id: i64) -> &mut Self { self.reply_to_message_id = Some(reply_to_message_id); self }
  
  pub fn disable_notification(&mut self, disable_notification: bool) -> &mut Self { self.disable_notification = Some(disable_notification); self }
  
  pub fn from_background(&mut self, from_background: bool) -> &mut Self { self.from_background = Some(from_background); self }
  


  
  // [input_message_contents] type is [Vec<Box<InputMessageContent>>], is not support, need add manully.
  #[doc(hidden)] pub fn _input_message_contents(&mut self, input_message_contents: Vec<Box<InputMessageContent>>) -> &mut Self { self.input_message_contents = Some(input_message_contents); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> SendMessageAlbum {
    SendMessageAlbum::builder()
      .chat_id(self.chat_id.clone())
      .reply_to_message_id(self.reply_to_message_id.clone())
      .disable_notification(self.disable_notification.clone())
      .from_background(self.from_background.clone())
      .input_message_contents(self.input_message_contents.clone())
      
      .build()
  }
}


///  Sends a Telegram Passport authorization form, effectively sharing data with the service. This method must be called after  
#[derive(Debug, Clone)]
pub struct TGSendPassportAuthorizationForm {
  ///  Authorization form identifier. 
  autorization_form_id: Option<i32>,
  ///  Types of Telegram Passport elements chosen by user to complete the authorization form. 
  types: Option<Vec<Box<PassportElementType>>>,
  
}

impl TDFB for TGSendPassportAuthorizationForm {}

impl AsRef<TGSendPassportAuthorizationForm> for TGSendPassportAuthorizationForm {
  fn as_ref(&self) -> &TGSendPassportAuthorizationForm { self }
}

impl TGSendPassportAuthorizationForm {

  pub fn new() -> Self {
    Self {
      autorization_form_id: None,
      types: None,
      
    }
  }

  
  pub fn autorization_form_id(&mut self, autorization_form_id: i32) -> &mut Self { self.autorization_form_id = Some(autorization_form_id); self }
  


  
  // [types] type is [Vec<Box<PassportElementType>>], is not support, need add manully.
  #[doc(hidden)] pub fn _types(&mut self, types: Vec<Box<PassportElementType>>) -> &mut Self { self.types = Some(types); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> SendPassportAuthorizationForm {
    SendPassportAuthorizationForm::builder()
      .autorization_form_id(self.autorization_form_id.clone())
      .types(self.types.clone())
      
      .build()
  }
}


///  Sends a filled-out payment form to the bot for final verification. 
#[derive(Debug, Clone)]
pub struct TGSendPaymentForm {
  ///  Chat identifier of the Invoice message. 
  chat_id: Option<i64>,
  ///  Message identifier. 
  message_id: Option<i64>,
  ///  Identifier returned by ValidateOrderInfo, or an empty string. 
  order_info_id: Option<String>,
  ///  Identifier of a chosen shipping option, if applicable. 
  shipping_option_id: Option<String>,
  ///  The credentials chosen by user for payment. 
  credentials: Option<Box<InputCredentials>>,
  
}

impl TDFB for TGSendPaymentForm {}

impl AsRef<TGSendPaymentForm> for TGSendPaymentForm {
  fn as_ref(&self) -> &TGSendPaymentForm { self }
}

impl TGSendPaymentForm {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      message_id: None,
      order_info_id: None,
      shipping_option_id: None,
      credentials: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn order_info_id<S: AsRef<str>>(&mut self, order_info_id: S) -> &mut Self { self.order_info_id = Some(order_info_id.as_ref().to_string()); self }
  
  pub fn shipping_option_id<S: AsRef<str>>(&mut self, shipping_option_id: S) -> &mut Self { self.shipping_option_id = Some(shipping_option_id.as_ref().to_string()); self }
  


  
  // [credentials] type is [Box<InputCredentials>], is not support, need add manully.
  #[doc(hidden)] pub fn _credentials(&mut self, credentials: Box<InputCredentials>) -> &mut Self { self.credentials = Some(credentials); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> SendPaymentForm {
    SendPaymentForm::builder()
      .chat_id(self.chat_id.clone())
      .message_id(self.message_id.clone())
      .order_info_id(self.order_info_id.clone())
      .shipping_option_id(self.shipping_option_id.clone())
      .credentials(self.credentials.clone())
      
      .build()
  }
}


///  Sends phone number confirmation code. Should be called when user presses " 
#[derive(Debug, Clone)]
pub struct TGSendPhoneNumberConfirmationCode {
  ///  Value of the "hash" parameter from the link. 
  hash: Option<String>,
  ///  Value of the "phone" parameter from the link. 
  phone_number: Option<String>,
  ///  Pass true if the authentication code may be sent via flash call to the specified phone number. 
  allow_flash_call: Option<bool>,
  ///  Pass true if the phone number is used on the current device. Ignored if allow_flash_call is false. 
  is_current_phone_number: Option<bool>,
  
}

impl TDFB for TGSendPhoneNumberConfirmationCode {}

impl AsRef<TGSendPhoneNumberConfirmationCode> for TGSendPhoneNumberConfirmationCode {
  fn as_ref(&self) -> &TGSendPhoneNumberConfirmationCode { self }
}

impl TGSendPhoneNumberConfirmationCode {

  pub fn new() -> Self {
    Self {
      hash: None,
      phone_number: None,
      allow_flash_call: None,
      is_current_phone_number: None,
      
    }
  }

  
  pub fn hash<S: AsRef<str>>(&mut self, hash: S) -> &mut Self { self.hash = Some(hash.as_ref().to_string()); self }
  
  pub fn phone_number<S: AsRef<str>>(&mut self, phone_number: S) -> &mut Self { self.phone_number = Some(phone_number.as_ref().to_string()); self }
  
  pub fn allow_flash_call(&mut self, allow_flash_call: bool) -> &mut Self { self.allow_flash_call = Some(allow_flash_call); self }
  
  pub fn is_current_phone_number(&mut self, is_current_phone_number: bool) -> &mut Self { self.is_current_phone_number = Some(is_current_phone_number); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SendPhoneNumberConfirmationCode {
    SendPhoneNumberConfirmationCode::builder()
      .hash(self.hash.clone())
      .phone_number(self.phone_number.clone())
      .allow_flash_call(self.allow_flash_call.clone())
      .is_current_phone_number(self.is_current_phone_number.clone())
      
      .build()
  }
}


///  Sends a code to verify a phone number to be added to a user's Telegram Passport. 
#[derive(Debug, Clone)]
pub struct TGSendPhoneNumberVerificationCode {
  ///  The phone number of the user, in international format. 
  phone_number: Option<String>,
  ///  Pass true if the authentication code may be sent via flash call to the specified phone number. 
  allow_flash_call: Option<bool>,
  ///  Pass true if the phone number is used on the current device. Ignored if allow_flash_call is false. 
  is_current_phone_number: Option<bool>,
  
}

impl TDFB for TGSendPhoneNumberVerificationCode {}

impl AsRef<TGSendPhoneNumberVerificationCode> for TGSendPhoneNumberVerificationCode {
  fn as_ref(&self) -> &TGSendPhoneNumberVerificationCode { self }
}

impl TGSendPhoneNumberVerificationCode {

  pub fn new() -> Self {
    Self {
      phone_number: None,
      allow_flash_call: None,
      is_current_phone_number: None,
      
    }
  }

  
  pub fn phone_number<S: AsRef<str>>(&mut self, phone_number: S) -> &mut Self { self.phone_number = Some(phone_number.as_ref().to_string()); self }
  
  pub fn allow_flash_call(&mut self, allow_flash_call: bool) -> &mut Self { self.allow_flash_call = Some(allow_flash_call); self }
  
  pub fn is_current_phone_number(&mut self, is_current_phone_number: bool) -> &mut Self { self.is_current_phone_number = Some(is_current_phone_number); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SendPhoneNumberVerificationCode {
    SendPhoneNumberVerificationCode::builder()
      .phone_number(self.phone_number.clone())
      .allow_flash_call(self.allow_flash_call.clone())
      .is_current_phone_number(self.is_current_phone_number.clone())
      
      .build()
  }
}


///  Changes the period of inactivity after which the account of the current user will automatically be deleted. 
#[derive(Debug, Clone)]
pub struct TGSetAccountTtl {
  ///  New account TTL. 
  ttl: Option<AccountTtl>,
  
}

impl TDFB for TGSetAccountTtl {}

impl AsRef<TGSetAccountTtl> for TGSetAccountTtl {
  fn as_ref(&self) -> &TGSetAccountTtl { self }
}

impl TGSetAccountTtl {

  pub fn new() -> Self {
    Self {
      ttl: None,
      
    }
  }

  


  
  // [ttl] type is [AccountTtl], is not support, need add manully.
  #[doc(hidden)] pub fn _ttl(&mut self, ttl: AccountTtl) -> &mut Self { self.ttl = Some(ttl); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> SetAccountTtl {
    SetAccountTtl::builder()
      .ttl(self.ttl.clone())
      
      .build()
  }
}


///  Succeeds after a specified amount of time has passed. Can be called before authorization. Can be called before initialization. 
#[derive(Debug, Clone)]
pub struct TGSetAlarm {
  ///  Number of seconds before the function returns. 
  seconds: Option<f64>,
  
}

impl TDFB for TGSetAlarm {}

impl AsRef<TGSetAlarm> for TGSetAlarm {
  fn as_ref(&self) -> &TGSetAlarm { self }
}

impl TGSetAlarm {

  pub fn new() -> Self {
    Self {
      seconds: None,
      
    }
  }

  
  pub fn seconds(&mut self, seconds: f64) -> &mut Self { self.seconds = Some(seconds); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SetAlarm {
    SetAlarm::builder()
      .seconds(self.seconds.clone())
      
      .build()
  }
}


///  Sets the phone number of the user and sends an authentication code to the user. Works only when the current authorization state is  
#[derive(Debug, Clone)]
pub struct TGSetAuthenticationPhoneNumber {
  ///  The phone number of the user, in international format. 
  phone_number: Option<String>,
  ///  Pass true if the authentication code may be sent via flash call to the specified phone number. 
  allow_flash_call: Option<bool>,
  ///  Pass true if the phone number is used on the current device. Ignored if allow_flash_call is false. 
  is_current_phone_number: Option<bool>,
  
}

impl TDFB for TGSetAuthenticationPhoneNumber {}

impl AsRef<TGSetAuthenticationPhoneNumber> for TGSetAuthenticationPhoneNumber {
  fn as_ref(&self) -> &TGSetAuthenticationPhoneNumber { self }
}

impl TGSetAuthenticationPhoneNumber {

  pub fn new() -> Self {
    Self {
      phone_number: None,
      allow_flash_call: None,
      is_current_phone_number: None,
      
    }
  }

  
  pub fn phone_number<S: AsRef<str>>(&mut self, phone_number: S) -> &mut Self { self.phone_number = Some(phone_number.as_ref().to_string()); self }
  
  pub fn allow_flash_call(&mut self, allow_flash_call: bool) -> &mut Self { self.allow_flash_call = Some(allow_flash_call); self }
  
  pub fn is_current_phone_number(&mut self, is_current_phone_number: bool) -> &mut Self { self.is_current_phone_number = Some(is_current_phone_number); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SetAuthenticationPhoneNumber {
    SetAuthenticationPhoneNumber::builder()
      .phone_number(self.phone_number.clone())
      .allow_flash_call(self.allow_flash_call.clone())
      .is_current_phone_number(self.is_current_phone_number.clone())
      
      .build()
  }
}


///  Changes the bio of the current user. 
#[derive(Debug, Clone)]
pub struct TGSetBio {
  ///  The new value of the user bio; 0-70 characters without line feeds. 
  bio: Option<String>,
  
}

impl TDFB for TGSetBio {}

impl AsRef<TGSetBio> for TGSetBio {
  fn as_ref(&self) -> &TGSetBio { self }
}

impl TGSetBio {

  pub fn new() -> Self {
    Self {
      bio: None,
      
    }
  }

  
  pub fn bio<S: AsRef<str>>(&mut self, bio: S) -> &mut Self { self.bio = Some(bio.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SetBio {
    SetBio::builder()
      .bio(self.bio.clone())
      
      .build()
  }
}


///  Informs the server about the number of pending bot updates if they haven't been processed for a long time; for bots only. 
#[derive(Debug, Clone)]
pub struct TGSetBotUpdatesStatus {
  ///  The number of pending updates. 
  pending_update_count: Option<i32>,
  ///  The last error message. 
  error_message: Option<String>,
  
}

impl TDFB for TGSetBotUpdatesStatus {}

impl AsRef<TGSetBotUpdatesStatus> for TGSetBotUpdatesStatus {
  fn as_ref(&self) -> &TGSetBotUpdatesStatus { self }
}

impl TGSetBotUpdatesStatus {

  pub fn new() -> Self {
    Self {
      pending_update_count: None,
      error_message: None,
      
    }
  }

  
  pub fn pending_update_count(&mut self, pending_update_count: i32) -> &mut Self { self.pending_update_count = Some(pending_update_count); self }
  
  pub fn error_message<S: AsRef<str>>(&mut self, error_message: S) -> &mut Self { self.error_message = Some(error_message.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SetBotUpdatesStatus {
    SetBotUpdatesStatus::builder()
      .pending_update_count(self.pending_update_count.clone())
      .error_message(self.error_message.clone())
      
      .build()
  }
}


///  Changes client data associated with a chat. 
#[derive(Debug, Clone)]
pub struct TGSetChatClientData {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  ///  New value of client_data. 
  client_data: Option<String>,
  
}

impl TDFB for TGSetChatClientData {}

impl AsRef<TGSetChatClientData> for TGSetChatClientData {
  fn as_ref(&self) -> &TGSetChatClientData { self }
}

impl TGSetChatClientData {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      client_data: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn client_data<S: AsRef<str>>(&mut self, client_data: S) -> &mut Self { self.client_data = Some(client_data.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SetChatClientData {
    SetChatClientData::builder()
      .chat_id(self.chat_id.clone())
      .client_data(self.client_data.clone())
      
      .build()
  }
}


///  Changes the draft message in a chat. 
#[derive(Debug, Clone)]
pub struct TGSetChatDraftMessage {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  ///  New draft message; may be null. 
  draft_message: Option<DraftMessage>,
  
}

impl TDFB for TGSetChatDraftMessage {}

impl AsRef<TGSetChatDraftMessage> for TGSetChatDraftMessage {
  fn as_ref(&self) -> &TGSetChatDraftMessage { self }
}

impl TGSetChatDraftMessage {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      draft_message: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  


  
  // [draft_message] type is [DraftMessage], is not support, need add manully.
  #[doc(hidden)] pub fn _draft_message(&mut self, draft_message: DraftMessage) -> &mut Self { self.draft_message = Some(draft_message); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> SetChatDraftMessage {
    SetChatDraftMessage::builder()
      .chat_id(self.chat_id.clone())
      .draft_message(self.draft_message.clone())
      
      .build()
  }
}


///  Changes the status of a chat member, needs appropriate privileges. This function is currently not suitable for adding new members to the chat; instead, use  
#[derive(Debug, Clone)]
pub struct TGSetChatMemberStatus {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  ///  User identifier. 
  user_id: Option<i32>,
  ///  The new status of the member in the chat. 
  status: Option<Box<ChatMemberStatus>>,
  
}

impl TDFB for TGSetChatMemberStatus {}

impl AsRef<TGSetChatMemberStatus> for TGSetChatMemberStatus {
  fn as_ref(&self) -> &TGSetChatMemberStatus { self }
}

impl TGSetChatMemberStatus {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      user_id: None,
      status: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  


  
  // [status] type is [Box<ChatMemberStatus>], is not support, need add manully.
  #[doc(hidden)] pub fn _status(&mut self, status: Box<ChatMemberStatus>) -> &mut Self { self.status = Some(status); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> SetChatMemberStatus {
    SetChatMemberStatus::builder()
      .chat_id(self.chat_id.clone())
      .user_id(self.user_id.clone())
      .status(self.status.clone())
      
      .build()
  }
}


///  Changes the notification settings of a chat. 
#[derive(Debug, Clone)]
pub struct TGSetChatNotificationSettings {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  ///  New notification settings for the chat. 
  notification_settings: Option<ChatNotificationSettings>,
  
}

impl TDFB for TGSetChatNotificationSettings {}

impl AsRef<TGSetChatNotificationSettings> for TGSetChatNotificationSettings {
  fn as_ref(&self) -> &TGSetChatNotificationSettings { self }
}

impl TGSetChatNotificationSettings {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      notification_settings: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  


  
  // [notification_settings] type is [ChatNotificationSettings], is not support, need add manully.
  #[doc(hidden)] pub fn _notification_settings(&mut self, notification_settings: ChatNotificationSettings) -> &mut Self { self.notification_settings = Some(notification_settings); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> SetChatNotificationSettings {
    SetChatNotificationSettings::builder()
      .chat_id(self.chat_id.clone())
      .notification_settings(self.notification_settings.clone())
      
      .build()
  }
}


///  Changes the photo of a chat. Supported only for basic groups, supergroups and channels. Requires administrator rights in basic groups and the appropriate administrator rights in supergroups and channels. The photo will not be changed before request to the server has been completed. 
#[derive(Debug, Clone)]
pub struct TGSetChatPhoto {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  ///  New chat photo. You can use a zero InputFileId to delete the chat photo. Files that are accessible only by HTTP URL are not acceptable. 
  photo: Option<Box<InputFile>>,
  
}

impl TDFB for TGSetChatPhoto {}

impl AsRef<TGSetChatPhoto> for TGSetChatPhoto {
  fn as_ref(&self) -> &TGSetChatPhoto { self }
}

impl TGSetChatPhoto {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      photo: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  


  
  // [photo] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _photo(&mut self, photo: Box<InputFile>) -> &mut Self { self.photo = Some(photo); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> SetChatPhoto {
    SetChatPhoto::builder()
      .chat_id(self.chat_id.clone())
      .photo(self.photo.clone())
      
      .build()
  }
}


///  Changes the chat title. Supported only for basic groups, supergroups and channels. Requires administrator rights in basic groups and the appropriate administrator rights in supergroups and channels. The title will not be changed until the request to the server has been completed. 
#[derive(Debug, Clone)]
pub struct TGSetChatTitle {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  ///  New title of the chat; 1-128 characters. 
  title: Option<String>,
  
}

impl TDFB for TGSetChatTitle {}

impl AsRef<TGSetChatTitle> for TGSetChatTitle {
  fn as_ref(&self) -> &TGSetChatTitle { self }
}

impl TGSetChatTitle {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      title: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn title<S: AsRef<str>>(&mut self, title: S) -> &mut Self { self.title = Some(title.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SetChatTitle {
    SetChatTitle::builder()
      .chat_id(self.chat_id.clone())
      .title(self.title.clone())
      
      .build()
  }
}


///  Adds or changes a custom local language pack to the current localization target. 
#[derive(Debug, Clone)]
pub struct TGSetCustomLanguagePack {
  ///  Information about the language pack. Language pack ID must start with 'X', consist only of English letters, digits and hyphens, and must not exceed 64 characters. Can be called before authorization. 
  info: Option<LanguagePackInfo>,
  ///  Strings of the new language pack. 
  strings: Option<Vec<LanguagePackString>>,
  
}

impl TDFB for TGSetCustomLanguagePack {}

impl AsRef<TGSetCustomLanguagePack> for TGSetCustomLanguagePack {
  fn as_ref(&self) -> &TGSetCustomLanguagePack { self }
}

impl TGSetCustomLanguagePack {

  pub fn new() -> Self {
    Self {
      info: None,
      strings: None,
      
    }
  }

  


  
  // [info] type is [LanguagePackInfo], is not support, need add manully.
  #[doc(hidden)] pub fn _info(&mut self, info: LanguagePackInfo) -> &mut Self { self.info = Some(info); self }
  
  // [strings] type is [Vec<LanguagePackString>], is not support, need add manully.
  #[doc(hidden)] pub fn _strings(&mut self, strings: Vec<LanguagePackString>) -> &mut Self { self.strings = Some(strings); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> SetCustomLanguagePack {
    SetCustomLanguagePack::builder()
      .info(self.info.clone())
      .strings(self.strings.clone())
      
      .build()
  }
}


///  Adds, edits or deletes a string in a custom local language pack. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGSetCustomLanguagePackString {
  ///  Identifier of a previously added custom local language pack in the current localization target. 
  language_pack_id: Option<String>,
  ///  New language pack string. 
  new_string: Option<LanguagePackString>,
  
}

impl TDFB for TGSetCustomLanguagePackString {}

impl AsRef<TGSetCustomLanguagePackString> for TGSetCustomLanguagePackString {
  fn as_ref(&self) -> &TGSetCustomLanguagePackString { self }
}

impl TGSetCustomLanguagePackString {

  pub fn new() -> Self {
    Self {
      language_pack_id: None,
      new_string: None,
      
    }
  }

  
  pub fn language_pack_id<S: AsRef<str>>(&mut self, language_pack_id: S) -> &mut Self { self.language_pack_id = Some(language_pack_id.as_ref().to_string()); self }
  


  
  // [new_string] type is [LanguagePackString], is not support, need add manully.
  #[doc(hidden)] pub fn _new_string(&mut self, new_string: LanguagePackString) -> &mut Self { self.new_string = Some(new_string); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> SetCustomLanguagePackString {
    SetCustomLanguagePackString::builder()
      .language_pack_id(self.language_pack_id.clone())
      .new_string(self.new_string.clone())
      
      .build()
  }
}


///  Changes the database encryption key. Usually the encryption key is never changed and is stored in some OS keychain. 
#[derive(Debug, Clone)]
pub struct TGSetDatabaseEncryptionKey {
  ///  New encryption key. 
  new_encryption_key: Option<String>,
  
}

impl TDFB for TGSetDatabaseEncryptionKey {}

impl AsRef<TGSetDatabaseEncryptionKey> for TGSetDatabaseEncryptionKey {
  fn as_ref(&self) -> &TGSetDatabaseEncryptionKey { self }
}

impl TGSetDatabaseEncryptionKey {

  pub fn new() -> Self {
    Self {
      new_encryption_key: None,
      
    }
  }

  
  pub fn new_encryption_key<S: AsRef<str>>(&mut self, new_encryption_key: S) -> &mut Self { self.new_encryption_key = Some(new_encryption_key.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SetDatabaseEncryptionKey {
    SetDatabaseEncryptionKey::builder()
      .new_encryption_key(self.new_encryption_key.clone())
      
      .build()
  }
}


///  Informs TDLib on a file generation prograss. 
#[derive(Debug, Clone)]
pub struct TGSetFileGenerationProgress {
  ///  The identifier of the generation process. 
  generation_id: Option<i64>,
  ///  Expected size of the generated file, in bytes; 0 if unknown. 
  expected_size: Option<i32>,
  ///  The number of bytes already generated. 
  local_prefix_size: Option<i32>,
  
}

impl TDFB for TGSetFileGenerationProgress {}

impl AsRef<TGSetFileGenerationProgress> for TGSetFileGenerationProgress {
  fn as_ref(&self) -> &TGSetFileGenerationProgress { self }
}

impl TGSetFileGenerationProgress {

  pub fn new() -> Self {
    Self {
      generation_id: None,
      expected_size: None,
      local_prefix_size: None,
      
    }
  }

  
  pub fn generation_id(&mut self, generation_id: i64) -> &mut Self { self.generation_id = Some(generation_id); self }
  
  pub fn expected_size(&mut self, expected_size: i32) -> &mut Self { self.expected_size = Some(expected_size); self }
  
  pub fn local_prefix_size(&mut self, local_prefix_size: i32) -> &mut Self { self.local_prefix_size = Some(local_prefix_size); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SetFileGenerationProgress {
    SetFileGenerationProgress::builder()
      .generation_id(self.generation_id.clone())
      .expected_size(self.expected_size.clone())
      .local_prefix_size(self.local_prefix_size.clone())
      
      .build()
  }
}


///  Updates the game score of the specified user in the game; for bots only. 
#[derive(Debug, Clone)]
pub struct TGSetGameScore {
  ///  The chat to which the message with the game belongs. 
  chat_id: Option<i64>,
  ///  Identifier of the message. 
  message_id: Option<i64>,
  ///  True, if the message should be edited. 
  edit_message: Option<bool>,
  ///  User identifier. 
  user_id: Option<i32>,
  ///  The new score. 
  score: Option<i32>,
  ///  Pass true to update the score even if it decreases. If the score is 0, the user will be deleted from the high score table. 
  force: Option<bool>,
  
}

impl TDFB for TGSetGameScore {}

impl AsRef<TGSetGameScore> for TGSetGameScore {
  fn as_ref(&self) -> &TGSetGameScore { self }
}

impl TGSetGameScore {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      message_id: None,
      edit_message: None,
      user_id: None,
      score: None,
      force: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn edit_message(&mut self, edit_message: bool) -> &mut Self { self.edit_message = Some(edit_message); self }
  
  pub fn user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn score(&mut self, score: i32) -> &mut Self { self.score = Some(score); self }
  
  pub fn force(&mut self, force: bool) -> &mut Self { self.force = Some(force); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SetGameScore {
    SetGameScore::builder()
      .chat_id(self.chat_id.clone())
      .message_id(self.message_id.clone())
      .edit_message(self.edit_message.clone())
      .user_id(self.user_id.clone())
      .score(self.score.clone())
      .force(self.force.clone())
      
      .build()
  }
}


///  Updates the game score of the specified user in a game; for bots only. 
#[derive(Debug, Clone)]
pub struct TGSetInlineGameScore {
  ///  Inline message identifier. 
  inline_message_id: Option<String>,
  ///  True, if the message should be edited. 
  edit_message: Option<bool>,
  ///  User identifier. 
  user_id: Option<i32>,
  ///  The new score. 
  score: Option<i32>,
  ///  Pass true to update the score even if it decreases. If the score is 0, the user will be deleted from the high score table. 
  force: Option<bool>,
  
}

impl TDFB for TGSetInlineGameScore {}

impl AsRef<TGSetInlineGameScore> for TGSetInlineGameScore {
  fn as_ref(&self) -> &TGSetInlineGameScore { self }
}

impl TGSetInlineGameScore {

  pub fn new() -> Self {
    Self {
      inline_message_id: None,
      edit_message: None,
      user_id: None,
      score: None,
      force: None,
      
    }
  }

  
  pub fn inline_message_id<S: AsRef<str>>(&mut self, inline_message_id: S) -> &mut Self { self.inline_message_id = Some(inline_message_id.as_ref().to_string()); self }
  
  pub fn edit_message(&mut self, edit_message: bool) -> &mut Self { self.edit_message = Some(edit_message); self }
  
  pub fn user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn score(&mut self, score: i32) -> &mut Self { self.score = Some(score); self }
  
  pub fn force(&mut self, force: bool) -> &mut Self { self.force = Some(force); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SetInlineGameScore {
    SetInlineGameScore::builder()
      .inline_message_id(self.inline_message_id.clone())
      .edit_message(self.edit_message.clone())
      .user_id(self.user_id.clone())
      .score(self.score.clone())
      .force(self.force.clone())
      
      .build()
  }
}


///  Sets new log stream for internal logging of TDLib. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGSetLogStream {
  ///  New log stream. 
  log_stream: Option<Box<LogStream>>,
  
}

impl TDFB for TGSetLogStream {}

impl AsRef<TGSetLogStream> for TGSetLogStream {
  fn as_ref(&self) -> &TGSetLogStream { self }
}

impl TGSetLogStream {

  pub fn new() -> Self {
    Self {
      log_stream: None,
      
    }
  }

  


  
  // [log_stream] type is [Box<LogStream>], is not support, need add manully.
  #[doc(hidden)] pub fn _log_stream(&mut self, log_stream: Box<LogStream>) -> &mut Self { self.log_stream = Some(log_stream); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> SetLogStream {
    SetLogStream::builder()
      .log_stream(self.log_stream.clone())
      
      .build()
  }
}


///  Sets the verbosity level for a specified TDLib internal log tag. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGSetLogTagVerbosityLevel {
  ///  Logging tag to change verbosity level. 
  tag: Option<String>,
  ///  New verbosity level; 1-1024. 
  new_verbosity_level: Option<i32>,
  
}

impl TDFB for TGSetLogTagVerbosityLevel {}

impl AsRef<TGSetLogTagVerbosityLevel> for TGSetLogTagVerbosityLevel {
  fn as_ref(&self) -> &TGSetLogTagVerbosityLevel { self }
}

impl TGSetLogTagVerbosityLevel {

  pub fn new() -> Self {
    Self {
      tag: None,
      new_verbosity_level: None,
      
    }
  }

  
  pub fn tag<S: AsRef<str>>(&mut self, tag: S) -> &mut Self { self.tag = Some(tag.as_ref().to_string()); self }
  
  pub fn new_verbosity_level(&mut self, new_verbosity_level: i32) -> &mut Self { self.new_verbosity_level = Some(new_verbosity_level); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SetLogTagVerbosityLevel {
    SetLogTagVerbosityLevel::builder()
      .tag(self.tag.clone())
      .new_verbosity_level(self.new_verbosity_level.clone())
      
      .build()
  }
}


///  Sets the verbosity level of the internal logging of TDLib. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGSetLogVerbosityLevel {
  ///  New value of the verbosity level for logging. Value 0 corresponds to fatal errors, value 1 corresponds to errors, value 2 corresponds to warnings and debug warnings, value 3 corresponds to informational, value 4 corresponds to debug, value 5 corresponds to verbose debug, value greater than 5 and up to 1023 can be used to enable even more logging. 
  new_verbosity_level: Option<i32>,
  
}

impl TDFB for TGSetLogVerbosityLevel {}

impl AsRef<TGSetLogVerbosityLevel> for TGSetLogVerbosityLevel {
  fn as_ref(&self) -> &TGSetLogVerbosityLevel { self }
}

impl TGSetLogVerbosityLevel {

  pub fn new() -> Self {
    Self {
      new_verbosity_level: None,
      
    }
  }

  
  pub fn new_verbosity_level(&mut self, new_verbosity_level: i32) -> &mut Self { self.new_verbosity_level = Some(new_verbosity_level); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SetLogVerbosityLevel {
    SetLogVerbosityLevel::builder()
      .new_verbosity_level(self.new_verbosity_level.clone())
      
      .build()
  }
}


///  Changes the first and last name of the current user. If something changes,  
#[derive(Debug, Clone)]
pub struct TGSetName {
  ///  The new value of the first name for the user; 1-64 characters. 
  first_name: Option<String>,
  ///  The new value of the optional last name for the user; 0-64 characters. 
  last_name: Option<String>,
  
}

impl TDFB for TGSetName {}

impl AsRef<TGSetName> for TGSetName {
  fn as_ref(&self) -> &TGSetName { self }
}

impl TGSetName {

  pub fn new() -> Self {
    Self {
      first_name: None,
      last_name: None,
      
    }
  }

  
  pub fn first_name<S: AsRef<str>>(&mut self, first_name: S) -> &mut Self { self.first_name = Some(first_name.as_ref().to_string()); self }
  
  pub fn last_name<S: AsRef<str>>(&mut self, last_name: S) -> &mut Self { self.last_name = Some(last_name.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SetName {
    SetName::builder()
      .first_name(self.first_name.clone())
      .last_name(self.last_name.clone())
      
      .build()
  }
}


///  Sets the current network type. Can be called before authorization. Calling this method forces all network connections to reopen, mitigating the delay in switching between different networks, so it should be called whenever the network is changed, even if the network type remains the same. Network type is used to check whether the library can use the network at all and also for collecting detailed network data usage statistics. 
#[derive(Debug, Clone)]
pub struct TGSetNetworkType {
  ///  The new network type. By default, networkTypeOther. 
  type_: Option<Box<NetworkType>>,
  
}

impl TDFB for TGSetNetworkType {}

impl AsRef<TGSetNetworkType> for TGSetNetworkType {
  fn as_ref(&self) -> &TGSetNetworkType { self }
}

impl TGSetNetworkType {

  pub fn new() -> Self {
    Self {
      type_: None,
      
    }
  }

  


  
  // [type_] type is [Box<NetworkType>], is not support, need add manully.
  #[doc(hidden)] pub fn _type_(&mut self, type_: Box<NetworkType>) -> &mut Self { self.type_ = Some(type_); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> SetNetworkType {
    SetNetworkType::builder()
      .type_(self.type_.clone())
      
      .build()
  }
}


///  Sets the value of an option. (Check the list of available options on  
#[derive(Debug, Clone)]
pub struct TGSetOption {
  ///  The name of the option. 
  name: Option<String>,
  ///  The new value of the option. 
  value: Option<Box<OptionValue>>,
  
}

impl TDFB for TGSetOption {}

impl AsRef<TGSetOption> for TGSetOption {
  fn as_ref(&self) -> &TGSetOption { self }
}

impl TGSetOption {

  pub fn new() -> Self {
    Self {
      name: None,
      value: None,
      
    }
  }

  
  pub fn name<S: AsRef<str>>(&mut self, name: S) -> &mut Self { self.name = Some(name.as_ref().to_string()); self }
  


  
  // [value] type is [Box<OptionValue>], is not support, need add manully.
  #[doc(hidden)] pub fn _value(&mut self, value: Box<OptionValue>) -> &mut Self { self.value = Some(value); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> SetOption {
    SetOption::builder()
      .name(self.name.clone())
      .value(self.value.clone())
      
      .build()
  }
}


///  Adds an element to the user's Telegram Passport. May return an error with a message "PHONE_VERIFICATION_NEEDED" or "EMAIL_VERIFICATION_NEEDED" if the chosen phone number or the chosen email address must be verified first. 
#[derive(Debug, Clone)]
pub struct TGSetPassportElement {
  ///  Input Telegram Passport element. 
  element: Option<Box<InputPassportElement>>,
  ///  Password of the current user. 
  password: Option<String>,
  
}

impl TDFB for TGSetPassportElement {}

impl AsRef<TGSetPassportElement> for TGSetPassportElement {
  fn as_ref(&self) -> &TGSetPassportElement { self }
}

impl TGSetPassportElement {

  pub fn new() -> Self {
    Self {
      element: None,
      password: None,
      
    }
  }

  
  pub fn password<S: AsRef<str>>(&mut self, password: S) -> &mut Self { self.password = Some(password.as_ref().to_string()); self }
  


  
  // [element] type is [Box<InputPassportElement>], is not support, need add manully.
  #[doc(hidden)] pub fn _element(&mut self, element: Box<InputPassportElement>) -> &mut Self { self.element = Some(element); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> SetPassportElement {
    SetPassportElement::builder()
      .element(self.element.clone())
      .password(self.password.clone())
      
      .build()
  }
}


///  Informs the user that some of the elements in their Telegram Passport contain errors; for bots only. The user will not be able to resend the elements, until the errors are fixed. 
#[derive(Debug, Clone)]
pub struct TGSetPassportElementErrors {
  ///  User identifier. 
  user_id: Option<i32>,
  ///  The errors. 
  errors: Option<Vec<InputPassportElementError>>,
  
}

impl TDFB for TGSetPassportElementErrors {}

impl AsRef<TGSetPassportElementErrors> for TGSetPassportElementErrors {
  fn as_ref(&self) -> &TGSetPassportElementErrors { self }
}

impl TGSetPassportElementErrors {

  pub fn new() -> Self {
    Self {
      user_id: None,
      errors: None,
      
    }
  }

  
  pub fn user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  


  
  // [errors] type is [Vec<InputPassportElementError>], is not support, need add manully.
  #[doc(hidden)] pub fn _errors(&mut self, errors: Vec<InputPassportElementError>) -> &mut Self { self.errors = Some(errors); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> SetPassportElementErrors {
    SetPassportElementErrors::builder()
      .user_id(self.user_id.clone())
      .errors(self.errors.clone())
      
      .build()
  }
}


///  Changes the password for the user. If a new recovery email address is specified, then the change will not be applied until the new recovery email address is confirmed. 
#[derive(Debug, Clone)]
pub struct TGSetPassword {
  ///  Previous password of the user. 
  old_password: Option<String>,
  ///  New password of the user; may be empty to remove the password. 
  new_password: Option<String>,
  ///  New password hint; may be empty. 
  new_hint: Option<String>,
  ///  Pass true if the recovery email address should be changed. 
  set_recovery_email_address: Option<bool>,
  ///  New recovery email address; may be empty. 
  new_recovery_email_address: Option<String>,
  
}

impl TDFB for TGSetPassword {}

impl AsRef<TGSetPassword> for TGSetPassword {
  fn as_ref(&self) -> &TGSetPassword { self }
}

impl TGSetPassword {

  pub fn new() -> Self {
    Self {
      old_password: None,
      new_password: None,
      new_hint: None,
      set_recovery_email_address: None,
      new_recovery_email_address: None,
      
    }
  }

  
  pub fn old_password<S: AsRef<str>>(&mut self, old_password: S) -> &mut Self { self.old_password = Some(old_password.as_ref().to_string()); self }
  
  pub fn new_password<S: AsRef<str>>(&mut self, new_password: S) -> &mut Self { self.new_password = Some(new_password.as_ref().to_string()); self }
  
  pub fn new_hint<S: AsRef<str>>(&mut self, new_hint: S) -> &mut Self { self.new_hint = Some(new_hint.as_ref().to_string()); self }
  
  pub fn set_recovery_email_address(&mut self, set_recovery_email_address: bool) -> &mut Self { self.set_recovery_email_address = Some(set_recovery_email_address); self }
  
  pub fn new_recovery_email_address<S: AsRef<str>>(&mut self, new_recovery_email_address: S) -> &mut Self { self.new_recovery_email_address = Some(new_recovery_email_address.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SetPassword {
    SetPassword::builder()
      .old_password(self.old_password.clone())
      .new_password(self.new_password.clone())
      .new_hint(self.new_hint.clone())
      .set_recovery_email_address(self.set_recovery_email_address.clone())
      .new_recovery_email_address(self.new_recovery_email_address.clone())
      
      .build()
  }
}


///  Changes the order of pinned chats. 
#[derive(Debug, Clone)]
pub struct TGSetPinnedChats {
  ///  The new list of pinned chats. 
  chat_ids: Option<Vec<i64>>,
  
}

impl TDFB for TGSetPinnedChats {}

impl AsRef<TGSetPinnedChats> for TGSetPinnedChats {
  fn as_ref(&self) -> &TGSetPinnedChats { self }
}

impl TGSetPinnedChats {

  pub fn new() -> Self {
    Self {
      chat_ids: None,
      
    }
  }

  
  pub fn chat_ids(&mut self, chat_ids: Vec<i64>) -> &mut Self { self.chat_ids = Some(chat_ids); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SetPinnedChats {
    SetPinnedChats::builder()
      .chat_ids(self.chat_ids.clone())
      
      .build()
  }
}


///  Changes user answer to a poll. 
#[derive(Debug, Clone)]
pub struct TGSetPollAnswer {
  ///  Identifier of the chat to which the poll belongs. 
  chat_id: Option<i64>,
  ///  Identifier of the message containing the poll. 
  message_id: Option<i64>,
  ///  0-based identifiers of options, chosen by the user. Currently user can't choose more than 1 option. 
  option_ids: Option<Vec<i32>>,
  
}

impl TDFB for TGSetPollAnswer {}

impl AsRef<TGSetPollAnswer> for TGSetPollAnswer {
  fn as_ref(&self) -> &TGSetPollAnswer { self }
}

impl TGSetPollAnswer {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      message_id: None,
      option_ids: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn option_ids(&mut self, option_ids: Vec<i32>) -> &mut Self { self.option_ids = Some(option_ids); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SetPollAnswer {
    SetPollAnswer::builder()
      .chat_id(self.chat_id.clone())
      .message_id(self.message_id.clone())
      .option_ids(self.option_ids.clone())
      
      .build()
  }
}


///  Uploads a new profile photo for the current user. If something changes,  
#[derive(Debug, Clone)]
pub struct TGSetProfilePhoto {
  ///  Profile photo to set. inputFileId and inputFileRemote may still be unsupported. 
  photo: Option<Box<InputFile>>,
  
}

impl TDFB for TGSetProfilePhoto {}

impl AsRef<TGSetProfilePhoto> for TGSetProfilePhoto {
  fn as_ref(&self) -> &TGSetProfilePhoto { self }
}

impl TGSetProfilePhoto {

  pub fn new() -> Self {
    Self {
      photo: None,
      
    }
  }

  


  
  // [photo] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _photo(&mut self, photo: Box<InputFile>) -> &mut Self { self.photo = Some(photo); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> SetProfilePhoto {
    SetProfilePhoto::builder()
      .photo(self.photo.clone())
      
      .build()
  }
}


///  Changes the 2-step verification recovery email address of the user. If a new recovery email address is specified, then the change will not be applied until the new recovery email address is confirmed If new_recovery_email_address is the same as the email address that is currently set up, this call succeeds immediately and aborts all other requests waiting for an email confirmation. 
#[derive(Debug, Clone)]
pub struct TGSetRecoveryEmailAddress {
  ///  Password of the current user. 
  password: Option<String>,
  ///  New recovery email address. 
  new_recovery_email_address: Option<String>,
  
}

impl TDFB for TGSetRecoveryEmailAddress {}

impl AsRef<TGSetRecoveryEmailAddress> for TGSetRecoveryEmailAddress {
  fn as_ref(&self) -> &TGSetRecoveryEmailAddress { self }
}

impl TGSetRecoveryEmailAddress {

  pub fn new() -> Self {
    Self {
      password: None,
      new_recovery_email_address: None,
      
    }
  }

  
  pub fn password<S: AsRef<str>>(&mut self, password: S) -> &mut Self { self.password = Some(password.as_ref().to_string()); self }
  
  pub fn new_recovery_email_address<S: AsRef<str>>(&mut self, new_recovery_email_address: S) -> &mut Self { self.new_recovery_email_address = Some(new_recovery_email_address.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SetRecoveryEmailAddress {
    SetRecoveryEmailAddress::builder()
      .password(self.password.clone())
      .new_recovery_email_address(self.new_recovery_email_address.clone())
      
      .build()
  }
}


///  Changes notification settings for chats of a given type. 
#[derive(Debug, Clone)]
pub struct TGSetScopeNotificationSettings {
  ///  Types of chats for which to change the notification settings. 
  scope: Option<Box<NotificationSettingsScope>>,
  ///  The new notification settings for the given scope. 
  notification_settings: Option<ScopeNotificationSettings>,
  
}

impl TDFB for TGSetScopeNotificationSettings {}

impl AsRef<TGSetScopeNotificationSettings> for TGSetScopeNotificationSettings {
  fn as_ref(&self) -> &TGSetScopeNotificationSettings { self }
}

impl TGSetScopeNotificationSettings {

  pub fn new() -> Self {
    Self {
      scope: None,
      notification_settings: None,
      
    }
  }

  


  
  // [scope] type is [Box<NotificationSettingsScope>], is not support, need add manully.
  #[doc(hidden)] pub fn _scope(&mut self, scope: Box<NotificationSettingsScope>) -> &mut Self { self.scope = Some(scope); self }
  
  // [notification_settings] type is [ScopeNotificationSettings], is not support, need add manully.
  #[doc(hidden)] pub fn _notification_settings(&mut self, notification_settings: ScopeNotificationSettings) -> &mut Self { self.notification_settings = Some(notification_settings); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> SetScopeNotificationSettings {
    SetScopeNotificationSettings::builder()
      .scope(self.scope.clone())
      .notification_settings(self.notification_settings.clone())
      
      .build()
  }
}


///  Changes the position of a sticker in the set to which it belongs; for bots only. The sticker set must have been created by the bot. 
#[derive(Debug, Clone)]
pub struct TGSetStickerPositionInSet {
  ///  Sticker. 
  sticker: Option<Box<InputFile>>,
  ///  New position of the sticker in the set, zero-based. 
  position: Option<i32>,
  
}

impl TDFB for TGSetStickerPositionInSet {}

impl AsRef<TGSetStickerPositionInSet> for TGSetStickerPositionInSet {
  fn as_ref(&self) -> &TGSetStickerPositionInSet { self }
}

impl TGSetStickerPositionInSet {

  pub fn new() -> Self {
    Self {
      sticker: None,
      position: None,
      
    }
  }

  
  pub fn position(&mut self, position: i32) -> &mut Self { self.position = Some(position); self }
  


  
  // [sticker] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _sticker(&mut self, sticker: Box<InputFile>) -> &mut Self { self.sticker = Some(sticker); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> SetStickerPositionInSet {
    SetStickerPositionInSet::builder()
      .sticker(self.sticker.clone())
      .position(self.position.clone())
      
      .build()
  }
}


///  Changes information about a supergroup or channel; requires appropriate administrator rights. 
#[derive(Debug, Clone)]
pub struct TGSetSupergroupDescription {
  ///  Identifier of the supergroup or channel. 
  supergroup_id: Option<i32>,
  ///  New supergroup or channel description; 0-255 characters. 
  description: Option<String>,
  
}

impl TDFB for TGSetSupergroupDescription {}

impl AsRef<TGSetSupergroupDescription> for TGSetSupergroupDescription {
  fn as_ref(&self) -> &TGSetSupergroupDescription { self }
}

impl TGSetSupergroupDescription {

  pub fn new() -> Self {
    Self {
      supergroup_id: None,
      description: None,
      
    }
  }

  
  pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self { self.supergroup_id = Some(supergroup_id); self }
  
  pub fn description<S: AsRef<str>>(&mut self, description: S) -> &mut Self { self.description = Some(description.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SetSupergroupDescription {
    SetSupergroupDescription::builder()
      .supergroup_id(self.supergroup_id.clone())
      .description(self.description.clone())
      
      .build()
  }
}


///  Changes the sticker set of a supergroup; requires appropriate rights in the supergroup. 
#[derive(Debug, Clone)]
pub struct TGSetSupergroupStickerSet {
  ///  Identifier of the supergroup. 
  supergroup_id: Option<i32>,
  ///  New value of the supergroup sticker set identifier. Use 0 to remove the supergroup sticker set. 
  sticker_set_id: Option<i64>,
  
}

impl TDFB for TGSetSupergroupStickerSet {}

impl AsRef<TGSetSupergroupStickerSet> for TGSetSupergroupStickerSet {
  fn as_ref(&self) -> &TGSetSupergroupStickerSet { self }
}

impl TGSetSupergroupStickerSet {

  pub fn new() -> Self {
    Self {
      supergroup_id: None,
      sticker_set_id: None,
      
    }
  }

  
  pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self { self.supergroup_id = Some(supergroup_id); self }
  
  pub fn sticker_set_id(&mut self, sticker_set_id: i64) -> &mut Self { self.sticker_set_id = Some(sticker_set_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SetSupergroupStickerSet {
    SetSupergroupStickerSet::builder()
      .supergroup_id(self.supergroup_id.clone())
      .sticker_set_id(self.sticker_set_id.clone())
      
      .build()
  }
}


///  Changes the username of a supergroup or channel, requires creator privileges in the supergroup or channel. 
#[derive(Debug, Clone)]
pub struct TGSetSupergroupUsername {
  ///  Identifier of the supergroup or channel. 
  supergroup_id: Option<i32>,
  ///  New value of the username. Use an empty string to remove the username. 
  username: Option<String>,
  
}

impl TDFB for TGSetSupergroupUsername {}

impl AsRef<TGSetSupergroupUsername> for TGSetSupergroupUsername {
  fn as_ref(&self) -> &TGSetSupergroupUsername { self }
}

impl TGSetSupergroupUsername {

  pub fn new() -> Self {
    Self {
      supergroup_id: None,
      username: None,
      
    }
  }

  
  pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self { self.supergroup_id = Some(supergroup_id); self }
  
  pub fn username<S: AsRef<str>>(&mut self, username: S) -> &mut Self { self.username = Some(username.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SetSupergroupUsername {
    SetSupergroupUsername::builder()
      .supergroup_id(self.supergroup_id.clone())
      .username(self.username.clone())
      
      .build()
  }
}


///  Sets the parameters for TDLib initialization. Works only when the current authorization state is  
#[derive(Debug, Clone)]
pub struct TGSetTdlibParameters {
  ///  Parameters. 
  parameters: Option<TdlibParameters>,
  
}

impl TDFB for TGSetTdlibParameters {}

impl AsRef<TGSetTdlibParameters> for TGSetTdlibParameters {
  fn as_ref(&self) -> &TGSetTdlibParameters { self }
}

impl TGSetTdlibParameters {

  pub fn new() -> Self {
    Self {
      parameters: None,
      
    }
  }

  


  
  // [parameters] type is [TdlibParameters], is not support, need add manully.
  #[doc(hidden)] pub fn _parameters(&mut self, parameters: TdlibParameters) -> &mut Self { self.parameters = Some(parameters); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> SetTdlibParameters {
    SetTdlibParameters::builder()
      .parameters(self.parameters.clone())
      
      .build()
  }
}


///  Changes the username of the current user. If something changes,  
#[derive(Debug, Clone)]
pub struct TGSetUsername {
  ///  The new value of the username. Use an empty string to remove the username. 
  username: Option<String>,
  
}

impl TDFB for TGSetUsername {}

impl AsRef<TGSetUsername> for TGSetUsername {
  fn as_ref(&self) -> &TGSetUsername { self }
}

impl TGSetUsername {

  pub fn new() -> Self {
    Self {
      username: None,
      
    }
  }

  
  pub fn username<S: AsRef<str>>(&mut self, username: S) -> &mut Self { self.username = Some(username.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SetUsername {
    SetUsername::builder()
      .username(self.username.clone())
      
      .build()
  }
}


///  Changes user privacy settings. 
#[derive(Debug, Clone)]
pub struct TGSetUserPrivacySettingRules {
  ///  The privacy setting. 
  setting: Option<Box<UserPrivacySetting>>,
  ///  The new privacy rules. 
  rules: Option<UserPrivacySettingRules>,
  
}

impl TDFB for TGSetUserPrivacySettingRules {}

impl AsRef<TGSetUserPrivacySettingRules> for TGSetUserPrivacySettingRules {
  fn as_ref(&self) -> &TGSetUserPrivacySettingRules { self }
}

impl TGSetUserPrivacySettingRules {

  pub fn new() -> Self {
    Self {
      setting: None,
      rules: None,
      
    }
  }

  


  
  // [setting] type is [Box<UserPrivacySetting>], is not support, need add manully.
  #[doc(hidden)] pub fn _setting(&mut self, setting: Box<UserPrivacySetting>) -> &mut Self { self.setting = Some(setting); self }
  
  // [rules] type is [UserPrivacySettingRules], is not support, need add manully.
  #[doc(hidden)] pub fn _rules(&mut self, rules: UserPrivacySettingRules) -> &mut Self { self.rules = Some(rules); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> SetUserPrivacySettingRules {
    SetUserPrivacySettingRules::builder()
      .setting(self.setting.clone())
      .rules(self.rules.clone())
      
      .build()
  }
}


///  Stops a poll. A poll in a message can be stopped when the message has can_be_edited flag set. 
#[derive(Debug, Clone)]
pub struct TGStopPoll {
  ///  Identifier of the chat to which the poll belongs. 
  chat_id: Option<i64>,
  ///  Identifier of the message containing the poll. 
  message_id: Option<i64>,
  ///  The new message reply markup; for bots only. 
  reply_markup: Option<Box<ReplyMarkup>>,
  
}

impl TDFB for TGStopPoll {}

impl AsRef<TGStopPoll> for TGStopPoll {
  fn as_ref(&self) -> &TGStopPoll { self }
}

impl TGStopPoll {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      message_id: None,
      reply_markup: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  


  
  // [reply_markup] type is [Box<ReplyMarkup>], is not support, need add manully.
  #[doc(hidden)] pub fn _reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> StopPoll {
    StopPoll::builder()
      .chat_id(self.chat_id.clone())
      .message_id(self.message_id.clone())
      .reply_markup(self.reply_markup.clone())
      
      .build()
  }
}


///  Fetches the latest versions of all strings from a language pack in the current localization target from the server. This method doesn't need to be called explicitly for the current used/base language packs. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGSynchronizeLanguagePack {
  ///  Language pack identifier. 
  language_pack_id: Option<String>,
  
}

impl TDFB for TGSynchronizeLanguagePack {}

impl AsRef<TGSynchronizeLanguagePack> for TGSynchronizeLanguagePack {
  fn as_ref(&self) -> &TGSynchronizeLanguagePack { self }
}

impl TGSynchronizeLanguagePack {

  pub fn new() -> Self {
    Self {
      language_pack_id: None,
      
    }
  }

  
  pub fn language_pack_id<S: AsRef<str>>(&mut self, language_pack_id: S) -> &mut Self { self.language_pack_id = Some(language_pack_id.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> SynchronizeLanguagePack {
    SynchronizeLanguagePack::builder()
      .language_pack_id(self.language_pack_id.clone())
      
      .build()
  }
}


///  Terminates all other sessions of the current user. 
#[derive(Debug, Clone)]
pub struct TGTerminateAllOtherSessions {
  
}

impl TDFB for TGTerminateAllOtherSessions {}

impl AsRef<TGTerminateAllOtherSessions> for TGTerminateAllOtherSessions {
  fn as_ref(&self) -> &TGTerminateAllOtherSessions { self }
}

impl TGTerminateAllOtherSessions {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> TerminateAllOtherSessions {
    TerminateAllOtherSessions::builder()
      
      .build()
  }
}


///  Terminates a session of the current user. 
#[derive(Debug, Clone)]
pub struct TGTerminateSession {
  ///  Session identifier. 
  session_id: Option<i64>,
  
}

impl TDFB for TGTerminateSession {}

impl AsRef<TGTerminateSession> for TGTerminateSession {
  fn as_ref(&self) -> &TGTerminateSession { self }
}

impl TGTerminateSession {

  pub fn new() -> Self {
    Self {
      session_id: None,
      
    }
  }

  
  pub fn session_id(&mut self, session_id: i64) -> &mut Self { self.session_id = Some(session_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> TerminateSession {
    TerminateSession::builder()
      .session_id(self.session_id.clone())
      
      .build()
  }
}


///  Returns the received bytes; for testing only. This is an offline method. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGTestCallBytes {
  ///  Bytes to return. 
  x: Option<String>,
  
}

impl TDFB for TGTestCallBytes {}

impl AsRef<TGTestCallBytes> for TGTestCallBytes {
  fn as_ref(&self) -> &TGTestCallBytes { self }
}

impl TGTestCallBytes {

  pub fn new() -> Self {
    Self {
      x: None,
      
    }
  }

  
  pub fn x<S: AsRef<str>>(&mut self, x: S) -> &mut Self { self.x = Some(x.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> TestCallBytes {
    TestCallBytes::builder()
      .x(self.x.clone())
      
      .build()
  }
}


///  Does nothing; for testing only. This is an offline method. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGTestCallEmpty {
  
}

impl TDFB for TGTestCallEmpty {}

impl AsRef<TGTestCallEmpty> for TGTestCallEmpty {
  fn as_ref(&self) -> &TGTestCallEmpty { self }
}

impl TGTestCallEmpty {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> TestCallEmpty {
    TestCallEmpty::builder()
      
      .build()
  }
}


///  Returns the received string; for testing only. This is an offline method. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGTestCallString {
  ///  String to return. 
  x: Option<String>,
  
}

impl TDFB for TGTestCallString {}

impl AsRef<TGTestCallString> for TGTestCallString {
  fn as_ref(&self) -> &TGTestCallString { self }
}

impl TGTestCallString {

  pub fn new() -> Self {
    Self {
      x: None,
      
    }
  }

  
  pub fn x<S: AsRef<str>>(&mut self, x: S) -> &mut Self { self.x = Some(x.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> TestCallString {
    TestCallString::builder()
      .x(self.x.clone())
      
      .build()
  }
}


///  Returns the received vector of numbers; for testing only. This is an offline method. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGTestCallVectorInt {
  ///  Vector of numbers to return. 
  x: Option<Vec<i32>>,
  
}

impl TDFB for TGTestCallVectorInt {}

impl AsRef<TGTestCallVectorInt> for TGTestCallVectorInt {
  fn as_ref(&self) -> &TGTestCallVectorInt { self }
}

impl TGTestCallVectorInt {

  pub fn new() -> Self {
    Self {
      x: None,
      
    }
  }

  
  pub fn x(&mut self, x: Vec<i32>) -> &mut Self { self.x = Some(x); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> TestCallVectorInt {
    TestCallVectorInt::builder()
      .x(self.x.clone())
      
      .build()
  }
}


///  Returns the received vector of objects containing a number; for testing only. This is an offline method. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGTestCallVectorIntObject {
  ///  Vector of objects to return. 
  x: Option<Vec<TestInt>>,
  
}

impl TDFB for TGTestCallVectorIntObject {}

impl AsRef<TGTestCallVectorIntObject> for TGTestCallVectorIntObject {
  fn as_ref(&self) -> &TGTestCallVectorIntObject { self }
}

impl TGTestCallVectorIntObject {

  pub fn new() -> Self {
    Self {
      x: None,
      
    }
  }

  


  
  // [x] type is [Vec<TestInt>], is not support, need add manully.
  #[doc(hidden)] pub fn _x(&mut self, x: Vec<TestInt>) -> &mut Self { self.x = Some(x); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> TestCallVectorIntObject {
    TestCallVectorIntObject::builder()
      .x(self.x.clone())
      
      .build()
  }
}


///  Returns the received vector of strings; for testing only. This is an offline method. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGTestCallVectorString {
  ///  Vector of strings to return. 
  x: Option<Vec<String>>,
  
}

impl TDFB for TGTestCallVectorString {}

impl AsRef<TGTestCallVectorString> for TGTestCallVectorString {
  fn as_ref(&self) -> &TGTestCallVectorString { self }
}

impl TGTestCallVectorString {

  pub fn new() -> Self {
    Self {
      x: None,
      
    }
  }

  


  
  // [x] type is [Vec<String>], is not support, need add manully.
  #[doc(hidden)] pub fn _x(&mut self, x: Vec<String>) -> &mut Self { self.x = Some(x); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> TestCallVectorString {
    TestCallVectorString::builder()
      .x(self.x.clone())
      
      .build()
  }
}


///  Returns the received vector of objects containing a string; for testing only. This is an offline method. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGTestCallVectorStringObject {
  ///  Vector of objects to return. 
  x: Option<Vec<TestString>>,
  
}

impl TDFB for TGTestCallVectorStringObject {}

impl AsRef<TGTestCallVectorStringObject> for TGTestCallVectorStringObject {
  fn as_ref(&self) -> &TGTestCallVectorStringObject { self }
}

impl TGTestCallVectorStringObject {

  pub fn new() -> Self {
    Self {
      x: None,
      
    }
  }

  


  
  // [x] type is [Vec<TestString>], is not support, need add manully.
  #[doc(hidden)] pub fn _x(&mut self, x: Vec<TestString>) -> &mut Self { self.x = Some(x); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> TestCallVectorStringObject {
    TestCallVectorStringObject::builder()
      .x(self.x.clone())
      
      .build()
  }
}


///  Forces an updates.getDifference call to the Telegram servers; for testing only. 
#[derive(Debug, Clone)]
pub struct TGTestGetDifference {
  
}

impl TDFB for TGTestGetDifference {}

impl AsRef<TGTestGetDifference> for TGTestGetDifference {
  fn as_ref(&self) -> &TGTestGetDifference { self }
}

impl TGTestGetDifference {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> TestGetDifference {
    TestGetDifference::builder()
      
      .build()
  }
}


///  Sends a simple network request to the Telegram servers; for testing only. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGTestNetwork {
  
}

impl TDFB for TGTestNetwork {}

impl AsRef<TGTestNetwork> for TGTestNetwork {
  fn as_ref(&self) -> &TGTestNetwork { self }
}

impl TGTestNetwork {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> TestNetwork {
    TestNetwork::builder()
      
      .build()
  }
}


///  Returns the squared received number; for testing only. This is an offline method. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGTestSquareInt {
  ///  Number to square. 
  x: Option<i32>,
  
}

impl TDFB for TGTestSquareInt {}

impl AsRef<TGTestSquareInt> for TGTestSquareInt {
  fn as_ref(&self) -> &TGTestSquareInt { self }
}

impl TGTestSquareInt {

  pub fn new() -> Self {
    Self {
      x: None,
      
    }
  }

  
  pub fn x(&mut self, x: i32) -> &mut Self { self.x = Some(x); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> TestSquareInt {
    TestSquareInt::builder()
      .x(self.x.clone())
      
      .build()
  }
}


///  Does nothing and ensures that the Error object is used; for testing only. This is an offline method. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGTestUseError {
  
}

impl TDFB for TGTestUseError {}

impl AsRef<TGTestUseError> for TGTestUseError {
  fn as_ref(&self) -> &TGTestUseError { self }
}

impl TGTestUseError {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> TestUseError {
    TestUseError::builder()
      
      .build()
  }
}


///  Does nothing and ensures that the  
#[derive(Debug, Clone)]
pub struct TGTestUseUpdate {
  
}

impl TDFB for TGTestUseUpdate {}

impl AsRef<TGTestUseUpdate> for TGTestUseUpdate {
  fn as_ref(&self) -> &TGTestUseUpdate { self }
}

impl TGTestUseUpdate {

  pub fn new() -> Self {
    Self {
      
    }
  }

  


  

  #[doc(hidden)]
  pub fn build(&self) -> TestUseUpdate {
    TestUseUpdate::builder()
      
      .build()
  }
}


///  Toggles the "All members are admins" setting in basic groups; requires creator privileges in the group. 
#[derive(Debug, Clone)]
pub struct TGToggleBasicGroupAdministrators {
  ///  Identifier of the basic group. 
  basic_group_id: Option<i32>,
  ///  New value of everyone_is_administrator. 
  everyone_is_administrator: Option<bool>,
  
}

impl TDFB for TGToggleBasicGroupAdministrators {}

impl AsRef<TGToggleBasicGroupAdministrators> for TGToggleBasicGroupAdministrators {
  fn as_ref(&self) -> &TGToggleBasicGroupAdministrators { self }
}

impl TGToggleBasicGroupAdministrators {

  pub fn new() -> Self {
    Self {
      basic_group_id: None,
      everyone_is_administrator: None,
      
    }
  }

  
  pub fn basic_group_id(&mut self, basic_group_id: i32) -> &mut Self { self.basic_group_id = Some(basic_group_id); self }
  
  pub fn everyone_is_administrator(&mut self, everyone_is_administrator: bool) -> &mut Self { self.everyone_is_administrator = Some(everyone_is_administrator); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> ToggleBasicGroupAdministrators {
    ToggleBasicGroupAdministrators::builder()
      .basic_group_id(self.basic_group_id.clone())
      .everyone_is_administrator(self.everyone_is_administrator.clone())
      
      .build()
  }
}


///  Changes the value of the default disable_notification parameter, used when a message is sent to a chat. 
#[derive(Debug, Clone)]
pub struct TGToggleChatDefaultDisableNotification {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  ///  New value of default_disable_notification. 
  default_disable_notification: Option<bool>,
  
}

impl TDFB for TGToggleChatDefaultDisableNotification {}

impl AsRef<TGToggleChatDefaultDisableNotification> for TGToggleChatDefaultDisableNotification {
  fn as_ref(&self) -> &TGToggleChatDefaultDisableNotification { self }
}

impl TGToggleChatDefaultDisableNotification {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      default_disable_notification: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn default_disable_notification(&mut self, default_disable_notification: bool) -> &mut Self { self.default_disable_notification = Some(default_disable_notification); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> ToggleChatDefaultDisableNotification {
    ToggleChatDefaultDisableNotification::builder()
      .chat_id(self.chat_id.clone())
      .default_disable_notification(self.default_disable_notification.clone())
      
      .build()
  }
}


///  Changes the marked as unread state of a chat. 
#[derive(Debug, Clone)]
pub struct TGToggleChatIsMarkedAsUnread {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  ///  New value of is_marked_as_unread. 
  is_marked_as_unread: Option<bool>,
  
}

impl TDFB for TGToggleChatIsMarkedAsUnread {}

impl AsRef<TGToggleChatIsMarkedAsUnread> for TGToggleChatIsMarkedAsUnread {
  fn as_ref(&self) -> &TGToggleChatIsMarkedAsUnread { self }
}

impl TGToggleChatIsMarkedAsUnread {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      is_marked_as_unread: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn is_marked_as_unread(&mut self, is_marked_as_unread: bool) -> &mut Self { self.is_marked_as_unread = Some(is_marked_as_unread); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> ToggleChatIsMarkedAsUnread {
    ToggleChatIsMarkedAsUnread::builder()
      .chat_id(self.chat_id.clone())
      .is_marked_as_unread(self.is_marked_as_unread.clone())
      
      .build()
  }
}


///  Changes the pinned state of a chat. You can pin up to GetOption("pinned_chat_count_max") non-secret chats and the same number of secret chats. 
#[derive(Debug, Clone)]
pub struct TGToggleChatIsPinned {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  ///  New value of is_pinned. 
  is_pinned: Option<bool>,
  
}

impl TDFB for TGToggleChatIsPinned {}

impl AsRef<TGToggleChatIsPinned> for TGToggleChatIsPinned {
  fn as_ref(&self) -> &TGToggleChatIsPinned { self }
}

impl TGToggleChatIsPinned {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      is_pinned: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self { self.is_pinned = Some(is_pinned); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> ToggleChatIsPinned {
    ToggleChatIsPinned::builder()
      .chat_id(self.chat_id.clone())
      .is_pinned(self.is_pinned.clone())
      
      .build()
  }
}


///  Toggles whether all members of a supergroup can add new members; requires appropriate administrator rights in the supergroup. 
#[derive(Debug, Clone)]
pub struct TGToggleSupergroupInvites {
  ///  Identifier of the supergroup. 
  supergroup_id: Option<i32>,
  ///  New value of anyone_can_invite. 
  anyone_can_invite: Option<bool>,
  
}

impl TDFB for TGToggleSupergroupInvites {}

impl AsRef<TGToggleSupergroupInvites> for TGToggleSupergroupInvites {
  fn as_ref(&self) -> &TGToggleSupergroupInvites { self }
}

impl TGToggleSupergroupInvites {

  pub fn new() -> Self {
    Self {
      supergroup_id: None,
      anyone_can_invite: None,
      
    }
  }

  
  pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self { self.supergroup_id = Some(supergroup_id); self }
  
  pub fn anyone_can_invite(&mut self, anyone_can_invite: bool) -> &mut Self { self.anyone_can_invite = Some(anyone_can_invite); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> ToggleSupergroupInvites {
    ToggleSupergroupInvites::builder()
      .supergroup_id(self.supergroup_id.clone())
      .anyone_can_invite(self.anyone_can_invite.clone())
      
      .build()
  }
}


///  Toggles whether the message history of a supergroup is available to new members; requires appropriate administrator rights in the supergroup. 
#[derive(Debug, Clone)]
pub struct TGToggleSupergroupIsAllHistoryAvailable {
  ///  The identifier of the supergroup. 
  supergroup_id: Option<i32>,
  ///  The new value of is_all_history_available. 
  is_all_history_available: Option<bool>,
  
}

impl TDFB for TGToggleSupergroupIsAllHistoryAvailable {}

impl AsRef<TGToggleSupergroupIsAllHistoryAvailable> for TGToggleSupergroupIsAllHistoryAvailable {
  fn as_ref(&self) -> &TGToggleSupergroupIsAllHistoryAvailable { self }
}

impl TGToggleSupergroupIsAllHistoryAvailable {

  pub fn new() -> Self {
    Self {
      supergroup_id: None,
      is_all_history_available: None,
      
    }
  }

  
  pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self { self.supergroup_id = Some(supergroup_id); self }
  
  pub fn is_all_history_available(&mut self, is_all_history_available: bool) -> &mut Self { self.is_all_history_available = Some(is_all_history_available); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> ToggleSupergroupIsAllHistoryAvailable {
    ToggleSupergroupIsAllHistoryAvailable::builder()
      .supergroup_id(self.supergroup_id.clone())
      .is_all_history_available(self.is_all_history_available.clone())
      
      .build()
  }
}


///  Toggles sender signatures messages sent in a channel; requires appropriate administrator rights in the channel. 
#[derive(Debug, Clone)]
pub struct TGToggleSupergroupSignMessages {
  ///  Identifier of the channel. 
  supergroup_id: Option<i32>,
  ///  New value of sign_messages. 
  sign_messages: Option<bool>,
  
}

impl TDFB for TGToggleSupergroupSignMessages {}

impl AsRef<TGToggleSupergroupSignMessages> for TGToggleSupergroupSignMessages {
  fn as_ref(&self) -> &TGToggleSupergroupSignMessages { self }
}

impl TGToggleSupergroupSignMessages {

  pub fn new() -> Self {
    Self {
      supergroup_id: None,
      sign_messages: None,
      
    }
  }

  
  pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self { self.supergroup_id = Some(supergroup_id); self }
  
  pub fn sign_messages(&mut self, sign_messages: bool) -> &mut Self { self.sign_messages = Some(sign_messages); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> ToggleSupergroupSignMessages {
    ToggleSupergroupSignMessages::builder()
      .supergroup_id(self.supergroup_id.clone())
      .sign_messages(self.sign_messages.clone())
      
      .build()
  }
}


///  Removes a user from the blacklist. 
#[derive(Debug, Clone)]
pub struct TGUnblockUser {
  ///  User identifier. 
  user_id: Option<i32>,
  
}

impl TDFB for TGUnblockUser {}

impl AsRef<TGUnblockUser> for TGUnblockUser {
  fn as_ref(&self) -> &TGUnblockUser { self }
}

impl TGUnblockUser {

  pub fn new() -> Self {
    Self {
      user_id: None,
      
    }
  }

  
  pub fn user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> UnblockUser {
    UnblockUser::builder()
      .user_id(self.user_id.clone())
      
      .build()
  }
}


///  Removes the pinned message from a chat; requires appropriate administrator rights in the group or channel. 
#[derive(Debug, Clone)]
pub struct TGUnpinChatMessage {
  ///  Identifier of the chat. 
  chat_id: Option<i64>,
  
}

impl TDFB for TGUnpinChatMessage {}

impl AsRef<TGUnpinChatMessage> for TGUnpinChatMessage {
  fn as_ref(&self) -> &TGUnpinChatMessage { self }
}

impl TGUnpinChatMessage {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> UnpinChatMessage {
    UnpinChatMessage::builder()
      .chat_id(self.chat_id.clone())
      
      .build()
  }
}


///  Creates a new supergroup from an existing basic group and sends a corresponding  
#[derive(Debug, Clone)]
pub struct TGUpgradeBasicGroupChatToSupergroupChat {
  ///  Identifier of the chat to upgrade. 
  chat_id: Option<i64>,
  
}

impl TDFB for TGUpgradeBasicGroupChatToSupergroupChat {}

impl AsRef<TGUpgradeBasicGroupChatToSupergroupChat> for TGUpgradeBasicGroupChatToSupergroupChat {
  fn as_ref(&self) -> &TGUpgradeBasicGroupChatToSupergroupChat { self }
}

impl TGUpgradeBasicGroupChatToSupergroupChat {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> UpgradeBasicGroupChatToSupergroupChat {
    UpgradeBasicGroupChatToSupergroupChat::builder()
      .chat_id(self.chat_id.clone())
      
      .build()
  }
}


///  Asynchronously uploads a file to the cloud without sending it in a message.  
#[derive(Debug, Clone)]
pub struct TGUploadFile {
  ///  File to upload. 
  file: Option<Box<InputFile>>,
  ///  File type. 
  file_type: Option<Box<FileType>>,
  ///  Priority of the upload (1-32). The higher the priority, the earlier the file will be uploaded. If the priorities of two files are equal, then the first one for which uploadFile was called will be uploaded first. 
  priority: Option<i32>,
  
}

impl TDFB for TGUploadFile {}

impl AsRef<TGUploadFile> for TGUploadFile {
  fn as_ref(&self) -> &TGUploadFile { self }
}

impl TGUploadFile {

  pub fn new() -> Self {
    Self {
      file: None,
      file_type: None,
      priority: None,
      
    }
  }

  
  pub fn priority(&mut self, priority: i32) -> &mut Self { self.priority = Some(priority); self }
  


  
  // [file] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _file(&mut self, file: Box<InputFile>) -> &mut Self { self.file = Some(file); self }
  
  // [file_type] type is [Box<FileType>], is not support, need add manully.
  #[doc(hidden)] pub fn _file_type(&mut self, file_type: Box<FileType>) -> &mut Self { self.file_type = Some(file_type); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> UploadFile {
    UploadFile::builder()
      .file(self.file.clone())
      .file_type(self.file_type.clone())
      .priority(self.priority.clone())
      
      .build()
  }
}


///  Uploads a PNG image with a sticker; for bots only; returns the uploaded file. 
#[derive(Debug, Clone)]
pub struct TGUploadStickerFile {
  ///  Sticker file owner. 
  user_id: Option<i32>,
  ///  PNG image with the sticker; must be up to 512 kB in size and fit in 512x512 square. 
  png_sticker: Option<Box<InputFile>>,
  
}

impl TDFB for TGUploadStickerFile {}

impl AsRef<TGUploadStickerFile> for TGUploadStickerFile {
  fn as_ref(&self) -> &TGUploadStickerFile { self }
}

impl TGUploadStickerFile {

  pub fn new() -> Self {
    Self {
      user_id: None,
      png_sticker: None,
      
    }
  }

  
  pub fn user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  


  
  // [png_sticker] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _png_sticker(&mut self, png_sticker: Box<InputFile>) -> &mut Self { self.png_sticker = Some(png_sticker); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> UploadStickerFile {
    UploadStickerFile::builder()
      .user_id(self.user_id.clone())
      .png_sticker(self.png_sticker.clone())
      
      .build()
  }
}


///  Validates the order information provided by a user and returns the available shipping options for a flexible invoice. 
#[derive(Debug, Clone)]
pub struct TGValidateOrderInfo {
  ///  Chat identifier of the Invoice message. 
  chat_id: Option<i64>,
  ///  Message identifier. 
  message_id: Option<i64>,
  ///  The order information, provided by the user. 
  order_info: Option<OrderInfo>,
  ///  True, if the order information can be saved. 
  allow_save: Option<bool>,
  
}

impl TDFB for TGValidateOrderInfo {}

impl AsRef<TGValidateOrderInfo> for TGValidateOrderInfo {
  fn as_ref(&self) -> &TGValidateOrderInfo { self }
}

impl TGValidateOrderInfo {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      message_id: None,
      order_info: None,
      allow_save: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn allow_save(&mut self, allow_save: bool) -> &mut Self { self.allow_save = Some(allow_save); self }
  


  
  // [order_info] type is [OrderInfo], is not support, need add manully.
  #[doc(hidden)] pub fn _order_info(&mut self, order_info: OrderInfo) -> &mut Self { self.order_info = Some(order_info); self }
  

  #[doc(hidden)]
  pub fn build(&self) -> ValidateOrderInfo {
    ValidateOrderInfo::builder()
      .chat_id(self.chat_id.clone())
      .message_id(self.message_id.clone())
      .order_info(self.order_info.clone())
      .allow_save(self.allow_save.clone())
      
      .build()
  }
}


///  Informs TDLib that messages are being viewed by the user. Many useful activities depend on whether the messages are currently being viewed or not (e.g., marking messages as read, incrementing a view counter, updating a view counter, removing deleted messages in supergroups and channels). 
#[derive(Debug, Clone)]
pub struct TGViewMessages {
  ///  Chat identifier. 
  chat_id: Option<i64>,
  ///  The identifiers of the messages being viewed. 
  message_ids: Option<Vec<i64>>,
  ///  True, if messages in closed chats should be marked as read. 
  force_read: Option<bool>,
  
}

impl TDFB for TGViewMessages {}

impl AsRef<TGViewMessages> for TGViewMessages {
  fn as_ref(&self) -> &TGViewMessages { self }
}

impl TGViewMessages {

  pub fn new() -> Self {
    Self {
      chat_id: None,
      message_ids: None,
      force_read: None,
      
    }
  }

  
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self { self.message_ids = Some(message_ids); self }
  
  pub fn force_read(&mut self, force_read: bool) -> &mut Self { self.force_read = Some(force_read); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> ViewMessages {
    ViewMessages::builder()
      .chat_id(self.chat_id.clone())
      .message_ids(self.message_ids.clone())
      .force_read(self.force_read.clone())
      
      .build()
  }
}


///  Informs the server that some trending sticker sets have been viewed by the user. 
#[derive(Debug, Clone)]
pub struct TGViewTrendingStickerSets {
  ///  Identifiers of viewed trending sticker sets. 
  sticker_set_ids: Option<Vec<i64>>,
  
}

impl TDFB for TGViewTrendingStickerSets {}

impl AsRef<TGViewTrendingStickerSets> for TGViewTrendingStickerSets {
  fn as_ref(&self) -> &TGViewTrendingStickerSets { self }
}

impl TGViewTrendingStickerSets {

  pub fn new() -> Self {
    Self {
      sticker_set_ids: None,
      
    }
  }

  
  pub fn sticker_set_ids(&mut self, sticker_set_ids: Vec<i64>) -> &mut Self { self.sticker_set_ids = Some(sticker_set_ids); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> ViewTrendingStickerSets {
    ViewTrendingStickerSets::builder()
      .sticker_set_ids(self.sticker_set_ids.clone())
      
      .build()
  }
}


///  Writes a part of a generated file. This method is intended to be used only if the client has no direct access to TDLib's file system, because it is usually slower than a direct write to the destination file. 
#[derive(Debug, Clone)]
pub struct TGWriteGeneratedFilePart {
  ///  The identifier of the generation process. 
  generation_id: Option<i64>,
  ///  The offset from which to write the data to the file. 
  offset: Option<i32>,
  ///  The data to write. 
  data: Option<String>,
  
}

impl TDFB for TGWriteGeneratedFilePart {}

impl AsRef<TGWriteGeneratedFilePart> for TGWriteGeneratedFilePart {
  fn as_ref(&self) -> &TGWriteGeneratedFilePart { self }
}

impl TGWriteGeneratedFilePart {

  pub fn new() -> Self {
    Self {
      generation_id: None,
      offset: None,
      data: None,
      
    }
  }

  
  pub fn generation_id(&mut self, generation_id: i64) -> &mut Self { self.generation_id = Some(generation_id); self }
  
  pub fn offset(&mut self, offset: i32) -> &mut Self { self.offset = Some(offset); self }
  
  pub fn data<S: AsRef<str>>(&mut self, data: S) -> &mut Self { self.data = Some(data.as_ref().to_string()); self }
  


  

  #[doc(hidden)]
  pub fn build(&self) -> WriteGeneratedFilePart {
    WriteGeneratedFilePart::builder()
      .generation_id(self.generation_id.clone())
      .offset(self.offset.clone())
      .data(self.data.clone())
      
      .build()
  }
}

