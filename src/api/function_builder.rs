use rtdlib::types::*;

pub trait TDFB {}

impl <'a, T: TDFB> TDFB for &'a T {}
impl <'a, T: TDFB> TDFB for &'a mut T {}


#[doc(hidden)] pub struct _TGAcceptCallBuilder { inner: TGAcceptCall }

impl _TGAcceptCallBuilder {

  pub fn build(&self) -> TGAcceptCall { self.inner.clone() }

  ///  Call identifier. 
  pub fn call_id(&mut self, call_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_call_id(call_id);
    self
  }
  

  
  // [protocol] type is [CallProtocol], is not support, need add manully.
  #[doc(hidden)] pub fn _protocol(&mut self, protocol: CallProtocol) -> &mut Self {
    self.inner.td_origin_mut()._set_protocol(protocol);
    self
  }
  
}


///  Accepts an incoming call. 
#[derive(Debug, Clone)]
pub struct TGAcceptCall {
  inner: AcceptCall
}

impl TDFB for TGAcceptCall {}

impl AsRef<TGAcceptCall> for TGAcceptCall {
  fn as_ref(&self) -> &TGAcceptCall { self }
}

impl AsRef<TGAcceptCall> for _TGAcceptCallBuilder {
  fn as_ref(&self) -> &TGAcceptCall { &self.inner }
}

impl TGAcceptCall {

  pub fn builder() -> _TGAcceptCallBuilder {
    _TGAcceptCallBuilder { inner: Self::new(AcceptCall::_new()) }
  }

  pub fn new(inner: AcceptCall) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &AcceptCall { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut AcceptCall { &mut self.inner }

}


#[doc(hidden)] pub struct _TGAcceptTermsOfServiceBuilder { inner: TGAcceptTermsOfService }

impl _TGAcceptTermsOfServiceBuilder {

  pub fn build(&self) -> TGAcceptTermsOfService { self.inner.clone() }

  ///  Terms of service identifier. 
  pub fn terms_of_service_id<S: AsRef<str>>(&mut self, terms_of_service_id: S) -> &mut Self {
    self.inner.td_origin_mut()._set_terms_of_service_id(terms_of_service_id.as_ref().to_string());
    self
  }
  

  
}


///  Accepts Telegram terms of services. 
#[derive(Debug, Clone)]
pub struct TGAcceptTermsOfService {
  inner: AcceptTermsOfService
}

impl TDFB for TGAcceptTermsOfService {}

impl AsRef<TGAcceptTermsOfService> for TGAcceptTermsOfService {
  fn as_ref(&self) -> &TGAcceptTermsOfService { self }
}

impl AsRef<TGAcceptTermsOfService> for _TGAcceptTermsOfServiceBuilder {
  fn as_ref(&self) -> &TGAcceptTermsOfService { &self.inner }
}

impl TGAcceptTermsOfService {

  pub fn builder() -> _TGAcceptTermsOfServiceBuilder {
    _TGAcceptTermsOfServiceBuilder { inner: Self::new(AcceptTermsOfService::_new()) }
  }

  pub fn new(inner: AcceptTermsOfService) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &AcceptTermsOfService { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut AcceptTermsOfService { &mut self.inner }

}


#[doc(hidden)] pub struct _TGAddChatMemberBuilder { inner: TGAddChatMember }

impl _TGAddChatMemberBuilder {

  pub fn build(&self) -> TGAddChatMember { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Identifier of the user. 
  pub fn user_id(&mut self, user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_user_id(user_id);
    self
  }
  ///  The number of earlier messages from the chat to be forwarded to the new member; up to 100. Ignored for supergroups and channels. 
  pub fn forward_limit(&mut self, forward_limit: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_forward_limit(forward_limit);
    self
  }
  

  
}


///  Adds a new member to a chat. Members can't be added to private or secret chats. Members will not be added until the chat state has been synchronized with the server. 
#[derive(Debug, Clone)]
pub struct TGAddChatMember {
  inner: AddChatMember
}

impl TDFB for TGAddChatMember {}

impl AsRef<TGAddChatMember> for TGAddChatMember {
  fn as_ref(&self) -> &TGAddChatMember { self }
}

impl AsRef<TGAddChatMember> for _TGAddChatMemberBuilder {
  fn as_ref(&self) -> &TGAddChatMember { &self.inner }
}

impl TGAddChatMember {

  pub fn builder() -> _TGAddChatMemberBuilder {
    _TGAddChatMemberBuilder { inner: Self::new(AddChatMember::_new()) }
  }

  pub fn new(inner: AddChatMember) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &AddChatMember { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut AddChatMember { &mut self.inner }

}


#[doc(hidden)] pub struct _TGAddChatMembersBuilder { inner: TGAddChatMembers }

impl _TGAddChatMembersBuilder {

  pub fn build(&self) -> TGAddChatMembers { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Identifiers of the users to be added to the chat. 
  pub fn user_ids(&mut self, user_ids: Vec<i32>) -> &mut Self {
    self.inner.td_origin_mut()._set_user_ids(user_ids);
    self
  }
  

  
}


///  Adds multiple new members to a chat. Currently this option is only available for supergroups and channels. This option can't be used to join a chat. Members can't be added to a channel if it has more than 200 members. Members will not be added until the chat state has been synchronized with the server. 
#[derive(Debug, Clone)]
pub struct TGAddChatMembers {
  inner: AddChatMembers
}

impl TDFB for TGAddChatMembers {}

impl AsRef<TGAddChatMembers> for TGAddChatMembers {
  fn as_ref(&self) -> &TGAddChatMembers { self }
}

impl AsRef<TGAddChatMembers> for _TGAddChatMembersBuilder {
  fn as_ref(&self) -> &TGAddChatMembers { &self.inner }
}

impl TGAddChatMembers {

  pub fn builder() -> _TGAddChatMembersBuilder {
    _TGAddChatMembersBuilder { inner: Self::new(AddChatMembers::_new()) }
  }

  pub fn new(inner: AddChatMembers) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &AddChatMembers { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut AddChatMembers { &mut self.inner }

}


#[doc(hidden)] pub struct _TGAddCustomServerLanguagePackBuilder { inner: TGAddCustomServerLanguagePack }

impl _TGAddCustomServerLanguagePackBuilder {

  pub fn build(&self) -> TGAddCustomServerLanguagePack { self.inner.clone() }

  ///  Identifier of a language pack to be added; may be different from a name that is used in an "https://t.me/setlanguage/" link. 
  pub fn language_pack_id<S: AsRef<str>>(&mut self, language_pack_id: S) -> &mut Self {
    self.inner.td_origin_mut()._set_language_pack_id(language_pack_id.as_ref().to_string());
    self
  }
  

  
}


///  Adds a custom server language pack to the list of installed language packs in current localization target. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGAddCustomServerLanguagePack {
  inner: AddCustomServerLanguagePack
}

impl TDFB for TGAddCustomServerLanguagePack {}

impl AsRef<TGAddCustomServerLanguagePack> for TGAddCustomServerLanguagePack {
  fn as_ref(&self) -> &TGAddCustomServerLanguagePack { self }
}

impl AsRef<TGAddCustomServerLanguagePack> for _TGAddCustomServerLanguagePackBuilder {
  fn as_ref(&self) -> &TGAddCustomServerLanguagePack { &self.inner }
}

impl TGAddCustomServerLanguagePack {

  pub fn builder() -> _TGAddCustomServerLanguagePackBuilder {
    _TGAddCustomServerLanguagePackBuilder { inner: Self::new(AddCustomServerLanguagePack::_new()) }
  }

  pub fn new(inner: AddCustomServerLanguagePack) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &AddCustomServerLanguagePack { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut AddCustomServerLanguagePack { &mut self.inner }

}


#[doc(hidden)] pub struct _TGAddFavoriteStickerBuilder { inner: TGAddFavoriteSticker }

impl _TGAddFavoriteStickerBuilder {

  pub fn build(&self) -> TGAddFavoriteSticker { self.inner.clone() }

  

  
  // [sticker] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _sticker(&mut self, sticker: Box<InputFile>) -> &mut Self {
    self.inner.td_origin_mut()._set_sticker(sticker);
    self
  }
  
}


///  Adds a new sticker to the list of favorite stickers. The new sticker is added to the top of the list. If the sticker was already in the list, it is removed from the list first. Only stickers belonging to a sticker set can be added to this list. 
#[derive(Debug, Clone)]
pub struct TGAddFavoriteSticker {
  inner: AddFavoriteSticker
}

impl TDFB for TGAddFavoriteSticker {}

impl AsRef<TGAddFavoriteSticker> for TGAddFavoriteSticker {
  fn as_ref(&self) -> &TGAddFavoriteSticker { self }
}

impl AsRef<TGAddFavoriteSticker> for _TGAddFavoriteStickerBuilder {
  fn as_ref(&self) -> &TGAddFavoriteSticker { &self.inner }
}

impl TGAddFavoriteSticker {

  pub fn builder() -> _TGAddFavoriteStickerBuilder {
    _TGAddFavoriteStickerBuilder { inner: Self::new(AddFavoriteSticker::_new()) }
  }

  pub fn new(inner: AddFavoriteSticker) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &AddFavoriteSticker { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut AddFavoriteSticker { &mut self.inner }

}


#[doc(hidden)] pub struct _TGAddLocalMessageBuilder { inner: TGAddLocalMessage }

impl _TGAddLocalMessageBuilder {

  pub fn build(&self) -> TGAddLocalMessage { self.inner.clone() }

  ///  Target chat. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Identifier of the user who will be shown as the sender of the message; may be 0 for channel posts. 
  pub fn sender_user_id(&mut self, sender_user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_sender_user_id(sender_user_id);
    self
  }
  ///  Identifier of the message to reply to or 0. 
  pub fn reply_to_message_id(&mut self, reply_to_message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_reply_to_message_id(reply_to_message_id);
    self
  }
  ///  Pass true to disable notification for the message. 
  pub fn disable_notification(&mut self, disable_notification: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_disable_notification(disable_notification);
    self
  }
  

  
  // [input_message_content] type is [Box<InputMessageContent>], is not support, need add manully.
  #[doc(hidden)] pub fn _input_message_content(&mut self, input_message_content: Box<InputMessageContent>) -> &mut Self {
    self.inner.td_origin_mut()._set_input_message_content(input_message_content);
    self
  }
  
}


///  Adds a local message to a chat. The message is persistent across application restarts only if the message database is used. Returns the added message. 
#[derive(Debug, Clone)]
pub struct TGAddLocalMessage {
  inner: AddLocalMessage
}

impl TDFB for TGAddLocalMessage {}

impl AsRef<TGAddLocalMessage> for TGAddLocalMessage {
  fn as_ref(&self) -> &TGAddLocalMessage { self }
}

impl AsRef<TGAddLocalMessage> for _TGAddLocalMessageBuilder {
  fn as_ref(&self) -> &TGAddLocalMessage { &self.inner }
}

impl TGAddLocalMessage {

  pub fn builder() -> _TGAddLocalMessageBuilder {
    _TGAddLocalMessageBuilder { inner: Self::new(AddLocalMessage::_new()) }
  }

  pub fn new(inner: AddLocalMessage) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &AddLocalMessage { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut AddLocalMessage { &mut self.inner }

}


#[doc(hidden)] pub struct _TGAddLogMessageBuilder { inner: TGAddLogMessage }

impl _TGAddLogMessageBuilder {

  pub fn build(&self) -> TGAddLogMessage { self.inner.clone() }

  ///  Minimum verbosity level needed for the message to be logged, 0-1023. 
  pub fn verbosity_level(&mut self, verbosity_level: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_verbosity_level(verbosity_level);
    self
  }
  ///  Text of a message to log. 
  pub fn text<S: AsRef<str>>(&mut self, text: S) -> &mut Self {
    self.inner.td_origin_mut()._set_text(text.as_ref().to_string());
    self
  }
  

  
}


///  Adds a message to TDLib internal log. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGAddLogMessage {
  inner: AddLogMessage
}

impl TDFB for TGAddLogMessage {}

impl AsRef<TGAddLogMessage> for TGAddLogMessage {
  fn as_ref(&self) -> &TGAddLogMessage { self }
}

impl AsRef<TGAddLogMessage> for _TGAddLogMessageBuilder {
  fn as_ref(&self) -> &TGAddLogMessage { &self.inner }
}

impl TGAddLogMessage {

  pub fn builder() -> _TGAddLogMessageBuilder {
    _TGAddLogMessageBuilder { inner: Self::new(AddLogMessage::_new()) }
  }

  pub fn new(inner: AddLogMessage) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &AddLogMessage { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut AddLogMessage { &mut self.inner }

}


#[doc(hidden)] pub struct _TGAddNetworkStatisticsBuilder { inner: TGAddNetworkStatistics }

impl _TGAddNetworkStatisticsBuilder {

  pub fn build(&self) -> TGAddNetworkStatistics { self.inner.clone() }

  

  
  // [entry] type is [Box<NetworkStatisticsEntry>], is not support, need add manully.
  #[doc(hidden)] pub fn _entry(&mut self, entry: Box<NetworkStatisticsEntry>) -> &mut Self {
    self.inner.td_origin_mut()._set_entry(entry);
    self
  }
  
}


///  Adds the specified data to data usage statistics. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGAddNetworkStatistics {
  inner: AddNetworkStatistics
}

impl TDFB for TGAddNetworkStatistics {}

impl AsRef<TGAddNetworkStatistics> for TGAddNetworkStatistics {
  fn as_ref(&self) -> &TGAddNetworkStatistics { self }
}

impl AsRef<TGAddNetworkStatistics> for _TGAddNetworkStatisticsBuilder {
  fn as_ref(&self) -> &TGAddNetworkStatistics { &self.inner }
}

impl TGAddNetworkStatistics {

  pub fn builder() -> _TGAddNetworkStatisticsBuilder {
    _TGAddNetworkStatisticsBuilder { inner: Self::new(AddNetworkStatistics::_new()) }
  }

  pub fn new(inner: AddNetworkStatistics) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &AddNetworkStatistics { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut AddNetworkStatistics { &mut self.inner }

}


#[doc(hidden)] pub struct _TGAddProxyBuilder { inner: TGAddProxy }

impl _TGAddProxyBuilder {

  pub fn build(&self) -> TGAddProxy { self.inner.clone() }

  ///  Proxy server IP address. 
  pub fn server<S: AsRef<str>>(&mut self, server: S) -> &mut Self {
    self.inner.td_origin_mut()._set_server(server.as_ref().to_string());
    self
  }
  ///  Proxy server port. 
  pub fn port(&mut self, port: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_port(port);
    self
  }
  ///  True, if the proxy should be enabled. 
  pub fn enable(&mut self, enable: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_enable(enable);
    self
  }
  

  
  // [type_] type is [Box<ProxyType>], is not support, need add manully.
  #[doc(hidden)] pub fn _type_(&mut self, type_: Box<ProxyType>) -> &mut Self {
    self.inner.td_origin_mut()._set_type_(type_);
    self
  }
  
}


///  Adds a proxy server for network requests. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGAddProxy {
  inner: AddProxy
}

impl TDFB for TGAddProxy {}

impl AsRef<TGAddProxy> for TGAddProxy {
  fn as_ref(&self) -> &TGAddProxy { self }
}

impl AsRef<TGAddProxy> for _TGAddProxyBuilder {
  fn as_ref(&self) -> &TGAddProxy { &self.inner }
}

impl TGAddProxy {

  pub fn builder() -> _TGAddProxyBuilder {
    _TGAddProxyBuilder { inner: Self::new(AddProxy::_new()) }
  }

  pub fn new(inner: AddProxy) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &AddProxy { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut AddProxy { &mut self.inner }

}


#[doc(hidden)] pub struct _TGAddRecentlyFoundChatBuilder { inner: TGAddRecentlyFoundChat }

impl _TGAddRecentlyFoundChatBuilder {

  pub fn build(&self) -> TGAddRecentlyFoundChat { self.inner.clone() }

  ///  Identifier of the chat to add. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  

  
}


///  Adds a chat to the list of recently found chats. The chat is added to the beginning of the list. If the chat is already in the list, it will be removed from the list first. 
#[derive(Debug, Clone)]
pub struct TGAddRecentlyFoundChat {
  inner: AddRecentlyFoundChat
}

impl TDFB for TGAddRecentlyFoundChat {}

impl AsRef<TGAddRecentlyFoundChat> for TGAddRecentlyFoundChat {
  fn as_ref(&self) -> &TGAddRecentlyFoundChat { self }
}

impl AsRef<TGAddRecentlyFoundChat> for _TGAddRecentlyFoundChatBuilder {
  fn as_ref(&self) -> &TGAddRecentlyFoundChat { &self.inner }
}

impl TGAddRecentlyFoundChat {

  pub fn builder() -> _TGAddRecentlyFoundChatBuilder {
    _TGAddRecentlyFoundChatBuilder { inner: Self::new(AddRecentlyFoundChat::_new()) }
  }

  pub fn new(inner: AddRecentlyFoundChat) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &AddRecentlyFoundChat { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut AddRecentlyFoundChat { &mut self.inner }

}


#[doc(hidden)] pub struct _TGAddRecentStickerBuilder { inner: TGAddRecentSticker }

impl _TGAddRecentStickerBuilder {

  pub fn build(&self) -> TGAddRecentSticker { self.inner.clone() }

  ///  Pass true to add the sticker to the list of stickers recently attached to photo or video files; pass false to add the sticker to the list of recently sent stickers. 
  pub fn is_attached(&mut self, is_attached: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_is_attached(is_attached);
    self
  }
  

  
  // [sticker] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _sticker(&mut self, sticker: Box<InputFile>) -> &mut Self {
    self.inner.td_origin_mut()._set_sticker(sticker);
    self
  }
  
}


///  Manually adds a new sticker to the list of recently used stickers. The new sticker is added to the top of the list. If the sticker was already in the list, it is removed from the list first. Only stickers belonging to a sticker set can be added to this list. 
#[derive(Debug, Clone)]
pub struct TGAddRecentSticker {
  inner: AddRecentSticker
}

impl TDFB for TGAddRecentSticker {}

impl AsRef<TGAddRecentSticker> for TGAddRecentSticker {
  fn as_ref(&self) -> &TGAddRecentSticker { self }
}

impl AsRef<TGAddRecentSticker> for _TGAddRecentStickerBuilder {
  fn as_ref(&self) -> &TGAddRecentSticker { &self.inner }
}

impl TGAddRecentSticker {

  pub fn builder() -> _TGAddRecentStickerBuilder {
    _TGAddRecentStickerBuilder { inner: Self::new(AddRecentSticker::_new()) }
  }

  pub fn new(inner: AddRecentSticker) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &AddRecentSticker { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut AddRecentSticker { &mut self.inner }

}


#[doc(hidden)] pub struct _TGAddSavedAnimationBuilder { inner: TGAddSavedAnimation }

impl _TGAddSavedAnimationBuilder {

  pub fn build(&self) -> TGAddSavedAnimation { self.inner.clone() }

  

  
  // [animation] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _animation(&mut self, animation: Box<InputFile>) -> &mut Self {
    self.inner.td_origin_mut()._set_animation(animation);
    self
  }
  
}


///  Manually adds a new animation to the list of saved animations. The new animation is added to the beginning of the list. If the animation was already in the list, it is removed first. Only non-secret video animations with MIME type "video/mp4" can be added to the list. 
#[derive(Debug, Clone)]
pub struct TGAddSavedAnimation {
  inner: AddSavedAnimation
}

impl TDFB for TGAddSavedAnimation {}

impl AsRef<TGAddSavedAnimation> for TGAddSavedAnimation {
  fn as_ref(&self) -> &TGAddSavedAnimation { self }
}

impl AsRef<TGAddSavedAnimation> for _TGAddSavedAnimationBuilder {
  fn as_ref(&self) -> &TGAddSavedAnimation { &self.inner }
}

impl TGAddSavedAnimation {

  pub fn builder() -> _TGAddSavedAnimationBuilder {
    _TGAddSavedAnimationBuilder { inner: Self::new(AddSavedAnimation::_new()) }
  }

  pub fn new(inner: AddSavedAnimation) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &AddSavedAnimation { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut AddSavedAnimation { &mut self.inner }

}


#[doc(hidden)] pub struct _TGAddStickerToSetBuilder { inner: TGAddStickerToSet }

impl _TGAddStickerToSetBuilder {

  pub fn build(&self) -> TGAddStickerToSet { self.inner.clone() }

  ///  Sticker set owner. 
  pub fn user_id(&mut self, user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_user_id(user_id);
    self
  }
  ///  Sticker set name. 
  pub fn name<S: AsRef<str>>(&mut self, name: S) -> &mut Self {
    self.inner.td_origin_mut()._set_name(name.as_ref().to_string());
    self
  }
  

  
  // [sticker] type is [InputSticker], is not support, need add manully.
  #[doc(hidden)] pub fn _sticker(&mut self, sticker: InputSticker) -> &mut Self {
    self.inner.td_origin_mut()._set_sticker(sticker);
    self
  }
  
}


///  Adds a new sticker to a set; for bots only. Returns the sticker set. 
#[derive(Debug, Clone)]
pub struct TGAddStickerToSet {
  inner: AddStickerToSet
}

impl TDFB for TGAddStickerToSet {}

impl AsRef<TGAddStickerToSet> for TGAddStickerToSet {
  fn as_ref(&self) -> &TGAddStickerToSet { self }
}

impl AsRef<TGAddStickerToSet> for _TGAddStickerToSetBuilder {
  fn as_ref(&self) -> &TGAddStickerToSet { &self.inner }
}

impl TGAddStickerToSet {

  pub fn builder() -> _TGAddStickerToSetBuilder {
    _TGAddStickerToSetBuilder { inner: Self::new(AddStickerToSet::_new()) }
  }

  pub fn new(inner: AddStickerToSet) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &AddStickerToSet { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut AddStickerToSet { &mut self.inner }

}


#[doc(hidden)] pub struct _TGAnswerCallbackQueryBuilder { inner: TGAnswerCallbackQuery }

impl _TGAnswerCallbackQueryBuilder {

  pub fn build(&self) -> TGAnswerCallbackQuery { self.inner.clone() }

  ///  Identifier of the callback query. 
  pub fn callback_query_id(&mut self, callback_query_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_callback_query_id(callback_query_id);
    self
  }
  ///  Text of the answer. 
  pub fn text<S: AsRef<str>>(&mut self, text: S) -> &mut Self {
    self.inner.td_origin_mut()._set_text(text.as_ref().to_string());
    self
  }
  ///  If true, an alert should be shown to the user instead of a toast notification. 
  pub fn show_alert(&mut self, show_alert: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_show_alert(show_alert);
    self
  }
  ///  URL to be opened. 
  pub fn url<S: AsRef<str>>(&mut self, url: S) -> &mut Self {
    self.inner.td_origin_mut()._set_url(url.as_ref().to_string());
    self
  }
  ///  Time during which the result of the query can be cached, in seconds. 
  pub fn cache_time(&mut self, cache_time: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_cache_time(cache_time);
    self
  }
  

  
}


///  Sets the result of a callback query; for bots only. 
#[derive(Debug, Clone)]
pub struct TGAnswerCallbackQuery {
  inner: AnswerCallbackQuery
}

impl TDFB for TGAnswerCallbackQuery {}

impl AsRef<TGAnswerCallbackQuery> for TGAnswerCallbackQuery {
  fn as_ref(&self) -> &TGAnswerCallbackQuery { self }
}

impl AsRef<TGAnswerCallbackQuery> for _TGAnswerCallbackQueryBuilder {
  fn as_ref(&self) -> &TGAnswerCallbackQuery { &self.inner }
}

impl TGAnswerCallbackQuery {

  pub fn builder() -> _TGAnswerCallbackQueryBuilder {
    _TGAnswerCallbackQueryBuilder { inner: Self::new(AnswerCallbackQuery::_new()) }
  }

  pub fn new(inner: AnswerCallbackQuery) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &AnswerCallbackQuery { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut AnswerCallbackQuery { &mut self.inner }

}


#[doc(hidden)] pub struct _TGAnswerCustomQueryBuilder { inner: TGAnswerCustomQuery }

impl _TGAnswerCustomQueryBuilder {

  pub fn build(&self) -> TGAnswerCustomQuery { self.inner.clone() }

  ///  Identifier of a custom query. 
  pub fn custom_query_id(&mut self, custom_query_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_custom_query_id(custom_query_id);
    self
  }
  ///  JSON-serialized answer to the query. 
  pub fn data<S: AsRef<str>>(&mut self, data: S) -> &mut Self {
    self.inner.td_origin_mut()._set_data(data.as_ref().to_string());
    self
  }
  

  
}


///  Answers a custom query; for bots only. 
#[derive(Debug, Clone)]
pub struct TGAnswerCustomQuery {
  inner: AnswerCustomQuery
}

impl TDFB for TGAnswerCustomQuery {}

impl AsRef<TGAnswerCustomQuery> for TGAnswerCustomQuery {
  fn as_ref(&self) -> &TGAnswerCustomQuery { self }
}

impl AsRef<TGAnswerCustomQuery> for _TGAnswerCustomQueryBuilder {
  fn as_ref(&self) -> &TGAnswerCustomQuery { &self.inner }
}

impl TGAnswerCustomQuery {

  pub fn builder() -> _TGAnswerCustomQueryBuilder {
    _TGAnswerCustomQueryBuilder { inner: Self::new(AnswerCustomQuery::_new()) }
  }

  pub fn new(inner: AnswerCustomQuery) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &AnswerCustomQuery { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut AnswerCustomQuery { &mut self.inner }

}


#[doc(hidden)] pub struct _TGAnswerInlineQueryBuilder { inner: TGAnswerInlineQuery }

impl _TGAnswerInlineQueryBuilder {

  pub fn build(&self) -> TGAnswerInlineQuery { self.inner.clone() }

  ///  Identifier of the inline query. 
  pub fn inline_query_id(&mut self, inline_query_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_inline_query_id(inline_query_id);
    self
  }
  ///  True, if the result of the query can be cached for the specified user. 
  pub fn is_personal(&mut self, is_personal: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_is_personal(is_personal);
    self
  }
  ///  Allowed time to cache the results of the query, in seconds. 
  pub fn cache_time(&mut self, cache_time: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_cache_time(cache_time);
    self
  }
  ///  Offset for the next inline query; pass an empty string if there are no more results. 
  pub fn next_offset<S: AsRef<str>>(&mut self, next_offset: S) -> &mut Self {
    self.inner.td_origin_mut()._set_next_offset(next_offset.as_ref().to_string());
    self
  }
  ///  If non-empty, this text should be shown on the button that opens a private chat with the bot and sends a start message to the bot with the parameter switch_pm_parameter. 
  pub fn switch_pm_text<S: AsRef<str>>(&mut self, switch_pm_text: S) -> &mut Self {
    self.inner.td_origin_mut()._set_switch_pm_text(switch_pm_text.as_ref().to_string());
    self
  }
  ///  The parameter for the bot start message. 
  pub fn switch_pm_parameter<S: AsRef<str>>(&mut self, switch_pm_parameter: S) -> &mut Self {
    self.inner.td_origin_mut()._set_switch_pm_parameter(switch_pm_parameter.as_ref().to_string());
    self
  }
  

  
  // [results] type is [Vec<Box<InputInlineQueryResult>>], is not support, need add manully.
  #[doc(hidden)] pub fn _results(&mut self, results: Vec<Box<InputInlineQueryResult>>) -> &mut Self {
    self.inner.td_origin_mut()._set_results(results);
    self
  }
  
}


///  Sets the result of an inline query; for bots only. 
#[derive(Debug, Clone)]
pub struct TGAnswerInlineQuery {
  inner: AnswerInlineQuery
}

impl TDFB for TGAnswerInlineQuery {}

impl AsRef<TGAnswerInlineQuery> for TGAnswerInlineQuery {
  fn as_ref(&self) -> &TGAnswerInlineQuery { self }
}

impl AsRef<TGAnswerInlineQuery> for _TGAnswerInlineQueryBuilder {
  fn as_ref(&self) -> &TGAnswerInlineQuery { &self.inner }
}

impl TGAnswerInlineQuery {

  pub fn builder() -> _TGAnswerInlineQueryBuilder {
    _TGAnswerInlineQueryBuilder { inner: Self::new(AnswerInlineQuery::_new()) }
  }

  pub fn new(inner: AnswerInlineQuery) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &AnswerInlineQuery { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut AnswerInlineQuery { &mut self.inner }

}


#[doc(hidden)] pub struct _TGAnswerPreCheckoutQueryBuilder { inner: TGAnswerPreCheckoutQuery }

impl _TGAnswerPreCheckoutQueryBuilder {

  pub fn build(&self) -> TGAnswerPreCheckoutQuery { self.inner.clone() }

  ///  Identifier of the pre-checkout query. 
  pub fn pre_checkout_query_id(&mut self, pre_checkout_query_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_pre_checkout_query_id(pre_checkout_query_id);
    self
  }
  ///  An error message, empty on success. 
  pub fn error_message<S: AsRef<str>>(&mut self, error_message: S) -> &mut Self {
    self.inner.td_origin_mut()._set_error_message(error_message.as_ref().to_string());
    self
  }
  

  
}


///  Sets the result of a pre-checkout query; for bots only. 
#[derive(Debug, Clone)]
pub struct TGAnswerPreCheckoutQuery {
  inner: AnswerPreCheckoutQuery
}

impl TDFB for TGAnswerPreCheckoutQuery {}

impl AsRef<TGAnswerPreCheckoutQuery> for TGAnswerPreCheckoutQuery {
  fn as_ref(&self) -> &TGAnswerPreCheckoutQuery { self }
}

impl AsRef<TGAnswerPreCheckoutQuery> for _TGAnswerPreCheckoutQueryBuilder {
  fn as_ref(&self) -> &TGAnswerPreCheckoutQuery { &self.inner }
}

impl TGAnswerPreCheckoutQuery {

  pub fn builder() -> _TGAnswerPreCheckoutQueryBuilder {
    _TGAnswerPreCheckoutQueryBuilder { inner: Self::new(AnswerPreCheckoutQuery::_new()) }
  }

  pub fn new(inner: AnswerPreCheckoutQuery) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &AnswerPreCheckoutQuery { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut AnswerPreCheckoutQuery { &mut self.inner }

}


#[doc(hidden)] pub struct _TGAnswerShippingQueryBuilder { inner: TGAnswerShippingQuery }

impl _TGAnswerShippingQueryBuilder {

  pub fn build(&self) -> TGAnswerShippingQuery { self.inner.clone() }

  ///  Identifier of the shipping query. 
  pub fn shipping_query_id(&mut self, shipping_query_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_shipping_query_id(shipping_query_id);
    self
  }
  ///  An error message, empty on success. 
  pub fn error_message<S: AsRef<str>>(&mut self, error_message: S) -> &mut Self {
    self.inner.td_origin_mut()._set_error_message(error_message.as_ref().to_string());
    self
  }
  

  
  // [shipping_options] type is [Vec<ShippingOption>], is not support, need add manully.
  #[doc(hidden)] pub fn _shipping_options(&mut self, shipping_options: Vec<ShippingOption>) -> &mut Self {
    self.inner.td_origin_mut()._set_shipping_options(shipping_options);
    self
  }
  
}


///  Sets the result of a shipping query; for bots only. 
#[derive(Debug, Clone)]
pub struct TGAnswerShippingQuery {
  inner: AnswerShippingQuery
}

impl TDFB for TGAnswerShippingQuery {}

impl AsRef<TGAnswerShippingQuery> for TGAnswerShippingQuery {
  fn as_ref(&self) -> &TGAnswerShippingQuery { self }
}

impl AsRef<TGAnswerShippingQuery> for _TGAnswerShippingQueryBuilder {
  fn as_ref(&self) -> &TGAnswerShippingQuery { &self.inner }
}

impl TGAnswerShippingQuery {

  pub fn builder() -> _TGAnswerShippingQueryBuilder {
    _TGAnswerShippingQueryBuilder { inner: Self::new(AnswerShippingQuery::_new()) }
  }

  pub fn new(inner: AnswerShippingQuery) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &AnswerShippingQuery { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut AnswerShippingQuery { &mut self.inner }

}


#[doc(hidden)] pub struct _TGBlockUserBuilder { inner: TGBlockUser }

impl _TGBlockUserBuilder {

  pub fn build(&self) -> TGBlockUser { self.inner.clone() }

  ///  User identifier. 
  pub fn user_id(&mut self, user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_user_id(user_id);
    self
  }
  

  
}


///  Adds a user to the blacklist. 
#[derive(Debug, Clone)]
pub struct TGBlockUser {
  inner: BlockUser
}

impl TDFB for TGBlockUser {}

impl AsRef<TGBlockUser> for TGBlockUser {
  fn as_ref(&self) -> &TGBlockUser { self }
}

impl AsRef<TGBlockUser> for _TGBlockUserBuilder {
  fn as_ref(&self) -> &TGBlockUser { &self.inner }
}

impl TGBlockUser {

  pub fn builder() -> _TGBlockUserBuilder {
    _TGBlockUserBuilder { inner: Self::new(BlockUser::_new()) }
  }

  pub fn new(inner: BlockUser) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &BlockUser { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut BlockUser { &mut self.inner }

}


#[doc(hidden)] pub struct _TGCancelDownloadFileBuilder { inner: TGCancelDownloadFile }

impl _TGCancelDownloadFileBuilder {

  pub fn build(&self) -> TGCancelDownloadFile { self.inner.clone() }

  ///  Identifier of a file to stop downloading. 
  pub fn file_id(&mut self, file_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_file_id(file_id);
    self
  }
  ///  Pass true to stop downloading only if it hasn't been started, i.e. request hasn't been sent to server. 
  pub fn only_if_pending(&mut self, only_if_pending: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_only_if_pending(only_if_pending);
    self
  }
  

  
}


///  Stops the downloading of a file. If a file has already been downloaded, does nothing. 
#[derive(Debug, Clone)]
pub struct TGCancelDownloadFile {
  inner: CancelDownloadFile
}

impl TDFB for TGCancelDownloadFile {}

impl AsRef<TGCancelDownloadFile> for TGCancelDownloadFile {
  fn as_ref(&self) -> &TGCancelDownloadFile { self }
}

impl AsRef<TGCancelDownloadFile> for _TGCancelDownloadFileBuilder {
  fn as_ref(&self) -> &TGCancelDownloadFile { &self.inner }
}

impl TGCancelDownloadFile {

  pub fn builder() -> _TGCancelDownloadFileBuilder {
    _TGCancelDownloadFileBuilder { inner: Self::new(CancelDownloadFile::_new()) }
  }

  pub fn new(inner: CancelDownloadFile) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &CancelDownloadFile { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut CancelDownloadFile { &mut self.inner }

}


#[doc(hidden)] pub struct _TGCancelUploadFileBuilder { inner: TGCancelUploadFile }

impl _TGCancelUploadFileBuilder {

  pub fn build(&self) -> TGCancelUploadFile { self.inner.clone() }

  ///  Identifier of the file to stop uploading. 
  pub fn file_id(&mut self, file_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_file_id(file_id);
    self
  }
  

  
}


///  Stops the uploading of a file. Supported only for files uploaded by using  
#[derive(Debug, Clone)]
pub struct TGCancelUploadFile {
  inner: CancelUploadFile
}

impl TDFB for TGCancelUploadFile {}

impl AsRef<TGCancelUploadFile> for TGCancelUploadFile {
  fn as_ref(&self) -> &TGCancelUploadFile { self }
}

impl AsRef<TGCancelUploadFile> for _TGCancelUploadFileBuilder {
  fn as_ref(&self) -> &TGCancelUploadFile { &self.inner }
}

impl TGCancelUploadFile {

  pub fn builder() -> _TGCancelUploadFileBuilder {
    _TGCancelUploadFileBuilder { inner: Self::new(CancelUploadFile::_new()) }
  }

  pub fn new(inner: CancelUploadFile) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &CancelUploadFile { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut CancelUploadFile { &mut self.inner }

}


#[doc(hidden)] pub struct _TGChangeChatReportSpamStateBuilder { inner: TGChangeChatReportSpamState }

impl _TGChangeChatReportSpamStateBuilder {

  pub fn build(&self) -> TGChangeChatReportSpamState { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  If true, the chat will be reported as spam; otherwise it will be marked as not spam. 
  pub fn is_spam_chat(&mut self, is_spam_chat: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_is_spam_chat(is_spam_chat);
    self
  }
  

  
}


///  Reports to the server whether a chat is a spam chat or not. Can be used only if ChatReportSpamState.can_report_spam is true. After this request, ChatReportSpamState.can_report_spam becomes false forever. 
#[derive(Debug, Clone)]
pub struct TGChangeChatReportSpamState {
  inner: ChangeChatReportSpamState
}

impl TDFB for TGChangeChatReportSpamState {}

impl AsRef<TGChangeChatReportSpamState> for TGChangeChatReportSpamState {
  fn as_ref(&self) -> &TGChangeChatReportSpamState { self }
}

impl AsRef<TGChangeChatReportSpamState> for _TGChangeChatReportSpamStateBuilder {
  fn as_ref(&self) -> &TGChangeChatReportSpamState { &self.inner }
}

impl TGChangeChatReportSpamState {

  pub fn builder() -> _TGChangeChatReportSpamStateBuilder {
    _TGChangeChatReportSpamStateBuilder { inner: Self::new(ChangeChatReportSpamState::_new()) }
  }

  pub fn new(inner: ChangeChatReportSpamState) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ChangeChatReportSpamState { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ChangeChatReportSpamState { &mut self.inner }

}


#[doc(hidden)] pub struct _TGChangeImportedContactsBuilder { inner: TGChangeImportedContacts }

impl _TGChangeImportedContactsBuilder {

  pub fn build(&self) -> TGChangeImportedContacts { self.inner.clone() }

  

  
  // [contacts] type is [Vec<Contact>], is not support, need add manully.
  #[doc(hidden)] pub fn _contacts(&mut self, contacts: Vec<Contact>) -> &mut Self {
    self.inner.td_origin_mut()._set_contacts(contacts);
    self
  }
  
}


///  Changes imported contacts using the list of current user contacts saved on the device. Imports newly added contacts and, if at least the file database is enabled, deletes recently deleted contacts. Query result depends on the result of the previous query, so only one query is possible at the same time. 
#[derive(Debug, Clone)]
pub struct TGChangeImportedContacts {
  inner: ChangeImportedContacts
}

impl TDFB for TGChangeImportedContacts {}

impl AsRef<TGChangeImportedContacts> for TGChangeImportedContacts {
  fn as_ref(&self) -> &TGChangeImportedContacts { self }
}

impl AsRef<TGChangeImportedContacts> for _TGChangeImportedContactsBuilder {
  fn as_ref(&self) -> &TGChangeImportedContacts { &self.inner }
}

impl TGChangeImportedContacts {

  pub fn builder() -> _TGChangeImportedContactsBuilder {
    _TGChangeImportedContactsBuilder { inner: Self::new(ChangeImportedContacts::_new()) }
  }

  pub fn new(inner: ChangeImportedContacts) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ChangeImportedContacts { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ChangeImportedContacts { &mut self.inner }

}


#[doc(hidden)] pub struct _TGChangePhoneNumberBuilder { inner: TGChangePhoneNumber }

impl _TGChangePhoneNumberBuilder {

  pub fn build(&self) -> TGChangePhoneNumber { self.inner.clone() }

  ///  The new phone number of the user in international format. 
  pub fn phone_number<S: AsRef<str>>(&mut self, phone_number: S) -> &mut Self {
    self.inner.td_origin_mut()._set_phone_number(phone_number.as_ref().to_string());
    self
  }
  ///  Pass true if the code can be sent via flash call to the specified phone number. 
  pub fn allow_flash_call(&mut self, allow_flash_call: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_allow_flash_call(allow_flash_call);
    self
  }
  ///  Pass true if the phone number is used on the current device. Ignored if allow_flash_call is false. 
  pub fn is_current_phone_number(&mut self, is_current_phone_number: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_is_current_phone_number(is_current_phone_number);
    self
  }
  

  
}


///  Changes the phone number of the user and sends an authentication code to the user's new phone number. On success, returns information about the sent code. 
#[derive(Debug, Clone)]
pub struct TGChangePhoneNumber {
  inner: ChangePhoneNumber
}

impl TDFB for TGChangePhoneNumber {}

impl AsRef<TGChangePhoneNumber> for TGChangePhoneNumber {
  fn as_ref(&self) -> &TGChangePhoneNumber { self }
}

impl AsRef<TGChangePhoneNumber> for _TGChangePhoneNumberBuilder {
  fn as_ref(&self) -> &TGChangePhoneNumber { &self.inner }
}

impl TGChangePhoneNumber {

  pub fn builder() -> _TGChangePhoneNumberBuilder {
    _TGChangePhoneNumberBuilder { inner: Self::new(ChangePhoneNumber::_new()) }
  }

  pub fn new(inner: ChangePhoneNumber) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ChangePhoneNumber { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ChangePhoneNumber { &mut self.inner }

}


#[doc(hidden)] pub struct _TGChangeStickerSetBuilder { inner: TGChangeStickerSet }

impl _TGChangeStickerSetBuilder {

  pub fn build(&self) -> TGChangeStickerSet { self.inner.clone() }

  ///  Identifier of the sticker set. 
  pub fn set_id(&mut self, set_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_set_id(set_id);
    self
  }
  ///  The new value of is_installed. 
  pub fn is_installed(&mut self, is_installed: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_is_installed(is_installed);
    self
  }
  ///  The new value of is_archived. A sticker set can't be installed and archived simultaneously. 
  pub fn is_archived(&mut self, is_archived: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_is_archived(is_archived);
    self
  }
  

  
}


///  Installs/uninstalls or activates/archives a sticker set. 
#[derive(Debug, Clone)]
pub struct TGChangeStickerSet {
  inner: ChangeStickerSet
}

impl TDFB for TGChangeStickerSet {}

impl AsRef<TGChangeStickerSet> for TGChangeStickerSet {
  fn as_ref(&self) -> &TGChangeStickerSet { self }
}

impl AsRef<TGChangeStickerSet> for _TGChangeStickerSetBuilder {
  fn as_ref(&self) -> &TGChangeStickerSet { &self.inner }
}

impl TGChangeStickerSet {

  pub fn builder() -> _TGChangeStickerSetBuilder {
    _TGChangeStickerSetBuilder { inner: Self::new(ChangeStickerSet::_new()) }
  }

  pub fn new(inner: ChangeStickerSet) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ChangeStickerSet { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ChangeStickerSet { &mut self.inner }

}


#[doc(hidden)] pub struct _TGCheckAuthenticationBotTokenBuilder { inner: TGCheckAuthenticationBotToken }

impl _TGCheckAuthenticationBotTokenBuilder {

  pub fn build(&self) -> TGCheckAuthenticationBotToken { self.inner.clone() }

  ///  The bot token. 
  pub fn token<S: AsRef<str>>(&mut self, token: S) -> &mut Self {
    self.inner.td_origin_mut()._set_token(token.as_ref().to_string());
    self
  }
  

  
}


///  Checks the authentication token of a bot; to log in as a bot. Works only when the current authorization state is  
#[derive(Debug, Clone)]
pub struct TGCheckAuthenticationBotToken {
  inner: CheckAuthenticationBotToken
}

impl TDFB for TGCheckAuthenticationBotToken {}

impl AsRef<TGCheckAuthenticationBotToken> for TGCheckAuthenticationBotToken {
  fn as_ref(&self) -> &TGCheckAuthenticationBotToken { self }
}

impl AsRef<TGCheckAuthenticationBotToken> for _TGCheckAuthenticationBotTokenBuilder {
  fn as_ref(&self) -> &TGCheckAuthenticationBotToken { &self.inner }
}

impl TGCheckAuthenticationBotToken {

  pub fn builder() -> _TGCheckAuthenticationBotTokenBuilder {
    _TGCheckAuthenticationBotTokenBuilder { inner: Self::new(CheckAuthenticationBotToken::_new()) }
  }

  pub fn new(inner: CheckAuthenticationBotToken) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &CheckAuthenticationBotToken { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut CheckAuthenticationBotToken { &mut self.inner }

}


#[doc(hidden)] pub struct _TGCheckAuthenticationCodeBuilder { inner: TGCheckAuthenticationCode }

impl _TGCheckAuthenticationCodeBuilder {

  pub fn build(&self) -> TGCheckAuthenticationCode { self.inner.clone() }

  ///  The verification code received via SMS, Telegram message, phone call, or flash call. 
  pub fn code<S: AsRef<str>>(&mut self, code: S) -> &mut Self {
    self.inner.td_origin_mut()._set_code(code.as_ref().to_string());
    self
  }
  ///  If the user is not yet registered, the first name of the user; 1-64 characters. You can also pass an empty string for unregistered user there to check verification code validness. In the latter case PHONE_NUMBER_UNOCCUPIED error will be returned for a valid code. 
  pub fn first_name<S: AsRef<str>>(&mut self, first_name: S) -> &mut Self {
    self.inner.td_origin_mut()._set_first_name(first_name.as_ref().to_string());
    self
  }
  ///  If the user is not yet registered; the last name of the user; optional; 0-64 characters. 
  pub fn last_name<S: AsRef<str>>(&mut self, last_name: S) -> &mut Self {
    self.inner.td_origin_mut()._set_last_name(last_name.as_ref().to_string());
    self
  }
  

  
}


///  Checks the authentication code. Works only when the current authorization state is  
#[derive(Debug, Clone)]
pub struct TGCheckAuthenticationCode {
  inner: CheckAuthenticationCode
}

impl TDFB for TGCheckAuthenticationCode {}

impl AsRef<TGCheckAuthenticationCode> for TGCheckAuthenticationCode {
  fn as_ref(&self) -> &TGCheckAuthenticationCode { self }
}

impl AsRef<TGCheckAuthenticationCode> for _TGCheckAuthenticationCodeBuilder {
  fn as_ref(&self) -> &TGCheckAuthenticationCode { &self.inner }
}

impl TGCheckAuthenticationCode {

  pub fn builder() -> _TGCheckAuthenticationCodeBuilder {
    _TGCheckAuthenticationCodeBuilder { inner: Self::new(CheckAuthenticationCode::_new()) }
  }

  pub fn new(inner: CheckAuthenticationCode) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &CheckAuthenticationCode { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut CheckAuthenticationCode { &mut self.inner }

}


#[doc(hidden)] pub struct _TGCheckAuthenticationPasswordBuilder { inner: TGCheckAuthenticationPassword }

impl _TGCheckAuthenticationPasswordBuilder {

  pub fn build(&self) -> TGCheckAuthenticationPassword { self.inner.clone() }

  ///  The password to check. 
  pub fn password<S: AsRef<str>>(&mut self, password: S) -> &mut Self {
    self.inner.td_origin_mut()._set_password(password.as_ref().to_string());
    self
  }
  

  
}


///  Checks the authentication password for correctness. Works only when the current authorization state is  
#[derive(Debug, Clone)]
pub struct TGCheckAuthenticationPassword {
  inner: CheckAuthenticationPassword
}

impl TDFB for TGCheckAuthenticationPassword {}

impl AsRef<TGCheckAuthenticationPassword> for TGCheckAuthenticationPassword {
  fn as_ref(&self) -> &TGCheckAuthenticationPassword { self }
}

impl AsRef<TGCheckAuthenticationPassword> for _TGCheckAuthenticationPasswordBuilder {
  fn as_ref(&self) -> &TGCheckAuthenticationPassword { &self.inner }
}

impl TGCheckAuthenticationPassword {

  pub fn builder() -> _TGCheckAuthenticationPasswordBuilder {
    _TGCheckAuthenticationPasswordBuilder { inner: Self::new(CheckAuthenticationPassword::_new()) }
  }

  pub fn new(inner: CheckAuthenticationPassword) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &CheckAuthenticationPassword { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut CheckAuthenticationPassword { &mut self.inner }

}


#[doc(hidden)] pub struct _TGCheckChangePhoneNumberCodeBuilder { inner: TGCheckChangePhoneNumberCode }

impl _TGCheckChangePhoneNumberCodeBuilder {

  pub fn build(&self) -> TGCheckChangePhoneNumberCode { self.inner.clone() }

  ///  Verification code received by SMS, phone call or flash call. 
  pub fn code<S: AsRef<str>>(&mut self, code: S) -> &mut Self {
    self.inner.td_origin_mut()._set_code(code.as_ref().to_string());
    self
  }
  

  
}


///  Checks the authentication code sent to confirm a new phone number of the user. 
#[derive(Debug, Clone)]
pub struct TGCheckChangePhoneNumberCode {
  inner: CheckChangePhoneNumberCode
}

impl TDFB for TGCheckChangePhoneNumberCode {}

impl AsRef<TGCheckChangePhoneNumberCode> for TGCheckChangePhoneNumberCode {
  fn as_ref(&self) -> &TGCheckChangePhoneNumberCode { self }
}

impl AsRef<TGCheckChangePhoneNumberCode> for _TGCheckChangePhoneNumberCodeBuilder {
  fn as_ref(&self) -> &TGCheckChangePhoneNumberCode { &self.inner }
}

impl TGCheckChangePhoneNumberCode {

  pub fn builder() -> _TGCheckChangePhoneNumberCodeBuilder {
    _TGCheckChangePhoneNumberCodeBuilder { inner: Self::new(CheckChangePhoneNumberCode::_new()) }
  }

  pub fn new(inner: CheckChangePhoneNumberCode) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &CheckChangePhoneNumberCode { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut CheckChangePhoneNumberCode { &mut self.inner }

}


#[doc(hidden)] pub struct _TGCheckChatInviteLinkBuilder { inner: TGCheckChatInviteLink }

impl _TGCheckChatInviteLinkBuilder {

  pub fn build(&self) -> TGCheckChatInviteLink { self.inner.clone() }

  ///  Invite link to be checked; should begin with "https://t.me/joinchat/", "https://telegram.me/joinchat/", or "https://telegram.dog/joinchat/". 
  pub fn invite_link<S: AsRef<str>>(&mut self, invite_link: S) -> &mut Self {
    self.inner.td_origin_mut()._set_invite_link(invite_link.as_ref().to_string());
    self
  }
  

  
}


///  Checks the validity of an invite link for a chat and returns information about the corresponding chat. 
#[derive(Debug, Clone)]
pub struct TGCheckChatInviteLink {
  inner: CheckChatInviteLink
}

impl TDFB for TGCheckChatInviteLink {}

impl AsRef<TGCheckChatInviteLink> for TGCheckChatInviteLink {
  fn as_ref(&self) -> &TGCheckChatInviteLink { self }
}

impl AsRef<TGCheckChatInviteLink> for _TGCheckChatInviteLinkBuilder {
  fn as_ref(&self) -> &TGCheckChatInviteLink { &self.inner }
}

impl TGCheckChatInviteLink {

  pub fn builder() -> _TGCheckChatInviteLinkBuilder {
    _TGCheckChatInviteLinkBuilder { inner: Self::new(CheckChatInviteLink::_new()) }
  }

  pub fn new(inner: CheckChatInviteLink) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &CheckChatInviteLink { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut CheckChatInviteLink { &mut self.inner }

}


#[doc(hidden)] pub struct _TGCheckChatUsernameBuilder { inner: TGCheckChatUsername }

impl _TGCheckChatUsernameBuilder {

  pub fn build(&self) -> TGCheckChatUsername { self.inner.clone() }

  ///  Chat identifier; should be identifier of a supergroup chat, or a channel chat, or a private chat with self, or zero if chat is being created. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Username to be checked. 
  pub fn username<S: AsRef<str>>(&mut self, username: S) -> &mut Self {
    self.inner.td_origin_mut()._set_username(username.as_ref().to_string());
    self
  }
  

  
}


///  Checks whether a username can be set for a chat. 
#[derive(Debug, Clone)]
pub struct TGCheckChatUsername {
  inner: CheckChatUsername
}

impl TDFB for TGCheckChatUsername {}

impl AsRef<TGCheckChatUsername> for TGCheckChatUsername {
  fn as_ref(&self) -> &TGCheckChatUsername { self }
}

impl AsRef<TGCheckChatUsername> for _TGCheckChatUsernameBuilder {
  fn as_ref(&self) -> &TGCheckChatUsername { &self.inner }
}

impl TGCheckChatUsername {

  pub fn builder() -> _TGCheckChatUsernameBuilder {
    _TGCheckChatUsernameBuilder { inner: Self::new(CheckChatUsername::_new()) }
  }

  pub fn new(inner: CheckChatUsername) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &CheckChatUsername { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut CheckChatUsername { &mut self.inner }

}


#[doc(hidden)] pub struct _TGCheckDatabaseEncryptionKeyBuilder { inner: TGCheckDatabaseEncryptionKey }

impl _TGCheckDatabaseEncryptionKeyBuilder {

  pub fn build(&self) -> TGCheckDatabaseEncryptionKey { self.inner.clone() }

  ///  Encryption key to check or set up. 
  pub fn encryption_key<S: AsRef<str>>(&mut self, encryption_key: S) -> &mut Self {
    self.inner.td_origin_mut()._set_encryption_key(encryption_key.as_ref().to_string());
    self
  }
  

  
}


///  Checks the database encryption key for correctness. Works only when the current authorization state is  
#[derive(Debug, Clone)]
pub struct TGCheckDatabaseEncryptionKey {
  inner: CheckDatabaseEncryptionKey
}

impl TDFB for TGCheckDatabaseEncryptionKey {}

impl AsRef<TGCheckDatabaseEncryptionKey> for TGCheckDatabaseEncryptionKey {
  fn as_ref(&self) -> &TGCheckDatabaseEncryptionKey { self }
}

impl AsRef<TGCheckDatabaseEncryptionKey> for _TGCheckDatabaseEncryptionKeyBuilder {
  fn as_ref(&self) -> &TGCheckDatabaseEncryptionKey { &self.inner }
}

impl TGCheckDatabaseEncryptionKey {

  pub fn builder() -> _TGCheckDatabaseEncryptionKeyBuilder {
    _TGCheckDatabaseEncryptionKeyBuilder { inner: Self::new(CheckDatabaseEncryptionKey::_new()) }
  }

  pub fn new(inner: CheckDatabaseEncryptionKey) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &CheckDatabaseEncryptionKey { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut CheckDatabaseEncryptionKey { &mut self.inner }

}


#[doc(hidden)] pub struct _TGCheckEmailAddressVerificationCodeBuilder { inner: TGCheckEmailAddressVerificationCode }

impl _TGCheckEmailAddressVerificationCodeBuilder {

  pub fn build(&self) -> TGCheckEmailAddressVerificationCode { self.inner.clone() }

  ///  Verification code. 
  pub fn code<S: AsRef<str>>(&mut self, code: S) -> &mut Self {
    self.inner.td_origin_mut()._set_code(code.as_ref().to_string());
    self
  }
  

  
}


///  Checks the email address verification code for Telegram Passport. 
#[derive(Debug, Clone)]
pub struct TGCheckEmailAddressVerificationCode {
  inner: CheckEmailAddressVerificationCode
}

impl TDFB for TGCheckEmailAddressVerificationCode {}

impl AsRef<TGCheckEmailAddressVerificationCode> for TGCheckEmailAddressVerificationCode {
  fn as_ref(&self) -> &TGCheckEmailAddressVerificationCode { self }
}

impl AsRef<TGCheckEmailAddressVerificationCode> for _TGCheckEmailAddressVerificationCodeBuilder {
  fn as_ref(&self) -> &TGCheckEmailAddressVerificationCode { &self.inner }
}

impl TGCheckEmailAddressVerificationCode {

  pub fn builder() -> _TGCheckEmailAddressVerificationCodeBuilder {
    _TGCheckEmailAddressVerificationCodeBuilder { inner: Self::new(CheckEmailAddressVerificationCode::_new()) }
  }

  pub fn new(inner: CheckEmailAddressVerificationCode) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &CheckEmailAddressVerificationCode { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut CheckEmailAddressVerificationCode { &mut self.inner }

}


#[doc(hidden)] pub struct _TGCheckPhoneNumberConfirmationCodeBuilder { inner: TGCheckPhoneNumberConfirmationCode }

impl _TGCheckPhoneNumberConfirmationCodeBuilder {

  pub fn build(&self) -> TGCheckPhoneNumberConfirmationCode { self.inner.clone() }

  ///  The phone number confirmation code. 
  pub fn code<S: AsRef<str>>(&mut self, code: S) -> &mut Self {
    self.inner.td_origin_mut()._set_code(code.as_ref().to_string());
    self
  }
  

  
}


///  Checks phone number confirmation code. 
#[derive(Debug, Clone)]
pub struct TGCheckPhoneNumberConfirmationCode {
  inner: CheckPhoneNumberConfirmationCode
}

impl TDFB for TGCheckPhoneNumberConfirmationCode {}

impl AsRef<TGCheckPhoneNumberConfirmationCode> for TGCheckPhoneNumberConfirmationCode {
  fn as_ref(&self) -> &TGCheckPhoneNumberConfirmationCode { self }
}

impl AsRef<TGCheckPhoneNumberConfirmationCode> for _TGCheckPhoneNumberConfirmationCodeBuilder {
  fn as_ref(&self) -> &TGCheckPhoneNumberConfirmationCode { &self.inner }
}

impl TGCheckPhoneNumberConfirmationCode {

  pub fn builder() -> _TGCheckPhoneNumberConfirmationCodeBuilder {
    _TGCheckPhoneNumberConfirmationCodeBuilder { inner: Self::new(CheckPhoneNumberConfirmationCode::_new()) }
  }

  pub fn new(inner: CheckPhoneNumberConfirmationCode) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &CheckPhoneNumberConfirmationCode { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut CheckPhoneNumberConfirmationCode { &mut self.inner }

}


#[doc(hidden)] pub struct _TGCheckPhoneNumberVerificationCodeBuilder { inner: TGCheckPhoneNumberVerificationCode }

impl _TGCheckPhoneNumberVerificationCodeBuilder {

  pub fn build(&self) -> TGCheckPhoneNumberVerificationCode { self.inner.clone() }

  ///  Verification code. 
  pub fn code<S: AsRef<str>>(&mut self, code: S) -> &mut Self {
    self.inner.td_origin_mut()._set_code(code.as_ref().to_string());
    self
  }
  

  
}


///  Checks the phone number verification code for Telegram Passport. 
#[derive(Debug, Clone)]
pub struct TGCheckPhoneNumberVerificationCode {
  inner: CheckPhoneNumberVerificationCode
}

impl TDFB for TGCheckPhoneNumberVerificationCode {}

impl AsRef<TGCheckPhoneNumberVerificationCode> for TGCheckPhoneNumberVerificationCode {
  fn as_ref(&self) -> &TGCheckPhoneNumberVerificationCode { self }
}

impl AsRef<TGCheckPhoneNumberVerificationCode> for _TGCheckPhoneNumberVerificationCodeBuilder {
  fn as_ref(&self) -> &TGCheckPhoneNumberVerificationCode { &self.inner }
}

impl TGCheckPhoneNumberVerificationCode {

  pub fn builder() -> _TGCheckPhoneNumberVerificationCodeBuilder {
    _TGCheckPhoneNumberVerificationCodeBuilder { inner: Self::new(CheckPhoneNumberVerificationCode::_new()) }
  }

  pub fn new(inner: CheckPhoneNumberVerificationCode) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &CheckPhoneNumberVerificationCode { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut CheckPhoneNumberVerificationCode { &mut self.inner }

}


#[doc(hidden)] pub struct _TGCheckRecoveryEmailAddressCodeBuilder { inner: TGCheckRecoveryEmailAddressCode }

impl _TGCheckRecoveryEmailAddressCodeBuilder {

  pub fn build(&self) -> TGCheckRecoveryEmailAddressCode { self.inner.clone() }

  ///  Verification code. 
  pub fn code<S: AsRef<str>>(&mut self, code: S) -> &mut Self {
    self.inner.td_origin_mut()._set_code(code.as_ref().to_string());
    self
  }
  

  
}


///  Checks the 2-step verification recovery email address verification code. 
#[derive(Debug, Clone)]
pub struct TGCheckRecoveryEmailAddressCode {
  inner: CheckRecoveryEmailAddressCode
}

impl TDFB for TGCheckRecoveryEmailAddressCode {}

impl AsRef<TGCheckRecoveryEmailAddressCode> for TGCheckRecoveryEmailAddressCode {
  fn as_ref(&self) -> &TGCheckRecoveryEmailAddressCode { self }
}

impl AsRef<TGCheckRecoveryEmailAddressCode> for _TGCheckRecoveryEmailAddressCodeBuilder {
  fn as_ref(&self) -> &TGCheckRecoveryEmailAddressCode { &self.inner }
}

impl TGCheckRecoveryEmailAddressCode {

  pub fn builder() -> _TGCheckRecoveryEmailAddressCodeBuilder {
    _TGCheckRecoveryEmailAddressCodeBuilder { inner: Self::new(CheckRecoveryEmailAddressCode::_new()) }
  }

  pub fn new(inner: CheckRecoveryEmailAddressCode) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &CheckRecoveryEmailAddressCode { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut CheckRecoveryEmailAddressCode { &mut self.inner }

}


#[doc(hidden)] pub struct _TGCleanFileNameBuilder { inner: TGCleanFileName }

impl _TGCleanFileNameBuilder {

  pub fn build(&self) -> TGCleanFileName { self.inner.clone() }

  ///  File name or path to the file. 
  pub fn file_name<S: AsRef<str>>(&mut self, file_name: S) -> &mut Self {
    self.inner.td_origin_mut()._set_file_name(file_name.as_ref().to_string());
    self
  }
  

  
}


///  Removes potentially dangerous characters from the name of a file. The encoding of the file name is supposed to be UTF-8. Returns an empty string on failure. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGCleanFileName {
  inner: CleanFileName
}

impl TDFB for TGCleanFileName {}

impl AsRef<TGCleanFileName> for TGCleanFileName {
  fn as_ref(&self) -> &TGCleanFileName { self }
}

impl AsRef<TGCleanFileName> for _TGCleanFileNameBuilder {
  fn as_ref(&self) -> &TGCleanFileName { &self.inner }
}

impl TGCleanFileName {

  pub fn builder() -> _TGCleanFileNameBuilder {
    _TGCleanFileNameBuilder { inner: Self::new(CleanFileName::_new()) }
  }

  pub fn new(inner: CleanFileName) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &CleanFileName { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut CleanFileName { &mut self.inner }

}


#[doc(hidden)] pub struct _TGClearAllDraftMessagesBuilder { inner: TGClearAllDraftMessages }

impl _TGClearAllDraftMessagesBuilder {

  pub fn build(&self) -> TGClearAllDraftMessages { self.inner.clone() }

  ///  If true, local draft messages in secret chats will not be cleared. 
  pub fn exclude_secret_chats(&mut self, exclude_secret_chats: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_exclude_secret_chats(exclude_secret_chats);
    self
  }
  

  
}


///  Clears draft messages in all chats. 
#[derive(Debug, Clone)]
pub struct TGClearAllDraftMessages {
  inner: ClearAllDraftMessages
}

impl TDFB for TGClearAllDraftMessages {}

impl AsRef<TGClearAllDraftMessages> for TGClearAllDraftMessages {
  fn as_ref(&self) -> &TGClearAllDraftMessages { self }
}

impl AsRef<TGClearAllDraftMessages> for _TGClearAllDraftMessagesBuilder {
  fn as_ref(&self) -> &TGClearAllDraftMessages { &self.inner }
}

impl TGClearAllDraftMessages {

  pub fn builder() -> _TGClearAllDraftMessagesBuilder {
    _TGClearAllDraftMessagesBuilder { inner: Self::new(ClearAllDraftMessages::_new()) }
  }

  pub fn new(inner: ClearAllDraftMessages) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ClearAllDraftMessages { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ClearAllDraftMessages { &mut self.inner }

}


#[doc(hidden)] pub struct _TGClearImportedContactsBuilder { inner: TGClearImportedContacts }

impl _TGClearImportedContactsBuilder {

  pub fn build(&self) -> TGClearImportedContacts { self.inner.clone() }

  

  
}


///  Clears all imported contacts, contact list remains unchanged. 
#[derive(Debug, Clone)]
pub struct TGClearImportedContacts {
  inner: ClearImportedContacts
}

impl TDFB for TGClearImportedContacts {}

impl AsRef<TGClearImportedContacts> for TGClearImportedContacts {
  fn as_ref(&self) -> &TGClearImportedContacts { self }
}

impl AsRef<TGClearImportedContacts> for _TGClearImportedContactsBuilder {
  fn as_ref(&self) -> &TGClearImportedContacts { &self.inner }
}

impl TGClearImportedContacts {

  pub fn builder() -> _TGClearImportedContactsBuilder {
    _TGClearImportedContactsBuilder { inner: Self::new(ClearImportedContacts::_new()) }
  }

  pub fn new(inner: ClearImportedContacts) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ClearImportedContacts { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ClearImportedContacts { &mut self.inner }

}


#[doc(hidden)] pub struct _TGClearRecentlyFoundChatsBuilder { inner: TGClearRecentlyFoundChats }

impl _TGClearRecentlyFoundChatsBuilder {

  pub fn build(&self) -> TGClearRecentlyFoundChats { self.inner.clone() }

  

  
}


///  Clears the list of recently found chats. 
#[derive(Debug, Clone)]
pub struct TGClearRecentlyFoundChats {
  inner: ClearRecentlyFoundChats
}

impl TDFB for TGClearRecentlyFoundChats {}

impl AsRef<TGClearRecentlyFoundChats> for TGClearRecentlyFoundChats {
  fn as_ref(&self) -> &TGClearRecentlyFoundChats { self }
}

impl AsRef<TGClearRecentlyFoundChats> for _TGClearRecentlyFoundChatsBuilder {
  fn as_ref(&self) -> &TGClearRecentlyFoundChats { &self.inner }
}

impl TGClearRecentlyFoundChats {

  pub fn builder() -> _TGClearRecentlyFoundChatsBuilder {
    _TGClearRecentlyFoundChatsBuilder { inner: Self::new(ClearRecentlyFoundChats::_new()) }
  }

  pub fn new(inner: ClearRecentlyFoundChats) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ClearRecentlyFoundChats { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ClearRecentlyFoundChats { &mut self.inner }

}


#[doc(hidden)] pub struct _TGClearRecentStickersBuilder { inner: TGClearRecentStickers }

impl _TGClearRecentStickersBuilder {

  pub fn build(&self) -> TGClearRecentStickers { self.inner.clone() }

  ///  Pass true to clear the list of stickers recently attached to photo or video files; pass false to clear the list of recently sent stickers. 
  pub fn is_attached(&mut self, is_attached: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_is_attached(is_attached);
    self
  }
  

  
}


///  Clears the list of recently used stickers. 
#[derive(Debug, Clone)]
pub struct TGClearRecentStickers {
  inner: ClearRecentStickers
}

impl TDFB for TGClearRecentStickers {}

impl AsRef<TGClearRecentStickers> for TGClearRecentStickers {
  fn as_ref(&self) -> &TGClearRecentStickers { self }
}

impl AsRef<TGClearRecentStickers> for _TGClearRecentStickersBuilder {
  fn as_ref(&self) -> &TGClearRecentStickers { &self.inner }
}

impl TGClearRecentStickers {

  pub fn builder() -> _TGClearRecentStickersBuilder {
    _TGClearRecentStickersBuilder { inner: Self::new(ClearRecentStickers::_new()) }
  }

  pub fn new(inner: ClearRecentStickers) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ClearRecentStickers { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ClearRecentStickers { &mut self.inner }

}


#[doc(hidden)] pub struct _TGCloseBuilder { inner: TGClose }

impl _TGCloseBuilder {

  pub fn build(&self) -> TGClose { self.inner.clone() }

  

  
}


///  Closes the TDLib instance. All databases will be flushed to disk and properly closed. After the close completes,  
#[derive(Debug, Clone)]
pub struct TGClose {
  inner: Close
}

impl TDFB for TGClose {}

impl AsRef<TGClose> for TGClose {
  fn as_ref(&self) -> &TGClose { self }
}

impl AsRef<TGClose> for _TGCloseBuilder {
  fn as_ref(&self) -> &TGClose { &self.inner }
}

impl TGClose {

  pub fn builder() -> _TGCloseBuilder {
    _TGCloseBuilder { inner: Self::new(Close::_new()) }
  }

  pub fn new(inner: Close) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &Close { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut Close { &mut self.inner }

}


#[doc(hidden)] pub struct _TGCloseChatBuilder { inner: TGCloseChat }

impl _TGCloseChatBuilder {

  pub fn build(&self) -> TGCloseChat { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  

  
}


///  Informs TDLib that the chat is closed by the user. Many useful activities depend on the chat being opened or closed. 
#[derive(Debug, Clone)]
pub struct TGCloseChat {
  inner: CloseChat
}

impl TDFB for TGCloseChat {}

impl AsRef<TGCloseChat> for TGCloseChat {
  fn as_ref(&self) -> &TGCloseChat { self }
}

impl AsRef<TGCloseChat> for _TGCloseChatBuilder {
  fn as_ref(&self) -> &TGCloseChat { &self.inner }
}

impl TGCloseChat {

  pub fn builder() -> _TGCloseChatBuilder {
    _TGCloseChatBuilder { inner: Self::new(CloseChat::_new()) }
  }

  pub fn new(inner: CloseChat) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &CloseChat { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut CloseChat { &mut self.inner }

}


#[doc(hidden)] pub struct _TGCloseSecretChatBuilder { inner: TGCloseSecretChat }

impl _TGCloseSecretChatBuilder {

  pub fn build(&self) -> TGCloseSecretChat { self.inner.clone() }

  ///  Secret chat identifier. 
  pub fn secret_chat_id(&mut self, secret_chat_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_secret_chat_id(secret_chat_id);
    self
  }
  

  
}


///  Closes a secret chat, effectively transfering its state to  
#[derive(Debug, Clone)]
pub struct TGCloseSecretChat {
  inner: CloseSecretChat
}

impl TDFB for TGCloseSecretChat {}

impl AsRef<TGCloseSecretChat> for TGCloseSecretChat {
  fn as_ref(&self) -> &TGCloseSecretChat { self }
}

impl AsRef<TGCloseSecretChat> for _TGCloseSecretChatBuilder {
  fn as_ref(&self) -> &TGCloseSecretChat { &self.inner }
}

impl TGCloseSecretChat {

  pub fn builder() -> _TGCloseSecretChatBuilder {
    _TGCloseSecretChatBuilder { inner: Self::new(CloseSecretChat::_new()) }
  }

  pub fn new(inner: CloseSecretChat) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &CloseSecretChat { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut CloseSecretChat { &mut self.inner }

}


#[doc(hidden)] pub struct _TGCreateBasicGroupChatBuilder { inner: TGCreateBasicGroupChat }

impl _TGCreateBasicGroupChatBuilder {

  pub fn build(&self) -> TGCreateBasicGroupChat { self.inner.clone() }

  ///  Basic group identifier. 
  pub fn basic_group_id(&mut self, basic_group_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_basic_group_id(basic_group_id);
    self
  }
  ///  If true, the chat will be created without network request. In this case all information about the chat except its type, title and photo can be incorrect. 
  pub fn force(&mut self, force: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_force(force);
    self
  }
  

  
}


///  Returns an existing chat corresponding to a known basic group. 
#[derive(Debug, Clone)]
pub struct TGCreateBasicGroupChat {
  inner: CreateBasicGroupChat
}

impl TDFB for TGCreateBasicGroupChat {}

impl AsRef<TGCreateBasicGroupChat> for TGCreateBasicGroupChat {
  fn as_ref(&self) -> &TGCreateBasicGroupChat { self }
}

impl AsRef<TGCreateBasicGroupChat> for _TGCreateBasicGroupChatBuilder {
  fn as_ref(&self) -> &TGCreateBasicGroupChat { &self.inner }
}

impl TGCreateBasicGroupChat {

  pub fn builder() -> _TGCreateBasicGroupChatBuilder {
    _TGCreateBasicGroupChatBuilder { inner: Self::new(CreateBasicGroupChat::_new()) }
  }

  pub fn new(inner: CreateBasicGroupChat) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &CreateBasicGroupChat { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut CreateBasicGroupChat { &mut self.inner }

}


#[doc(hidden)] pub struct _TGCreateCallBuilder { inner: TGCreateCall }

impl _TGCreateCallBuilder {

  pub fn build(&self) -> TGCreateCall { self.inner.clone() }

  ///  Identifier of the user to be called. 
  pub fn user_id(&mut self, user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_user_id(user_id);
    self
  }
  

  
  // [protocol] type is [CallProtocol], is not support, need add manully.
  #[doc(hidden)] pub fn _protocol(&mut self, protocol: CallProtocol) -> &mut Self {
    self.inner.td_origin_mut()._set_protocol(protocol);
    self
  }
  
}


///  Creates a new call. 
#[derive(Debug, Clone)]
pub struct TGCreateCall {
  inner: CreateCall
}

impl TDFB for TGCreateCall {}

impl AsRef<TGCreateCall> for TGCreateCall {
  fn as_ref(&self) -> &TGCreateCall { self }
}

impl AsRef<TGCreateCall> for _TGCreateCallBuilder {
  fn as_ref(&self) -> &TGCreateCall { &self.inner }
}

impl TGCreateCall {

  pub fn builder() -> _TGCreateCallBuilder {
    _TGCreateCallBuilder { inner: Self::new(CreateCall::_new()) }
  }

  pub fn new(inner: CreateCall) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &CreateCall { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut CreateCall { &mut self.inner }

}


#[doc(hidden)] pub struct _TGCreateNewBasicGroupChatBuilder { inner: TGCreateNewBasicGroupChat }

impl _TGCreateNewBasicGroupChatBuilder {

  pub fn build(&self) -> TGCreateNewBasicGroupChat { self.inner.clone() }

  ///  Identifiers of users to be added to the basic group. 
  pub fn user_ids(&mut self, user_ids: Vec<i32>) -> &mut Self {
    self.inner.td_origin_mut()._set_user_ids(user_ids);
    self
  }
  ///  Title of the new basic group; 1-128 characters. 
  pub fn title<S: AsRef<str>>(&mut self, title: S) -> &mut Self {
    self.inner.td_origin_mut()._set_title(title.as_ref().to_string());
    self
  }
  

  
}


///  Creates a new basic group and sends a corresponding  
#[derive(Debug, Clone)]
pub struct TGCreateNewBasicGroupChat {
  inner: CreateNewBasicGroupChat
}

impl TDFB for TGCreateNewBasicGroupChat {}

impl AsRef<TGCreateNewBasicGroupChat> for TGCreateNewBasicGroupChat {
  fn as_ref(&self) -> &TGCreateNewBasicGroupChat { self }
}

impl AsRef<TGCreateNewBasicGroupChat> for _TGCreateNewBasicGroupChatBuilder {
  fn as_ref(&self) -> &TGCreateNewBasicGroupChat { &self.inner }
}

impl TGCreateNewBasicGroupChat {

  pub fn builder() -> _TGCreateNewBasicGroupChatBuilder {
    _TGCreateNewBasicGroupChatBuilder { inner: Self::new(CreateNewBasicGroupChat::_new()) }
  }

  pub fn new(inner: CreateNewBasicGroupChat) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &CreateNewBasicGroupChat { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut CreateNewBasicGroupChat { &mut self.inner }

}


#[doc(hidden)] pub struct _TGCreateNewSecretChatBuilder { inner: TGCreateNewSecretChat }

impl _TGCreateNewSecretChatBuilder {

  pub fn build(&self) -> TGCreateNewSecretChat { self.inner.clone() }

  ///  Identifier of the target user. 
  pub fn user_id(&mut self, user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_user_id(user_id);
    self
  }
  

  
}


///  Creates a new secret chat. Returns the newly created chat. 
#[derive(Debug, Clone)]
pub struct TGCreateNewSecretChat {
  inner: CreateNewSecretChat
}

impl TDFB for TGCreateNewSecretChat {}

impl AsRef<TGCreateNewSecretChat> for TGCreateNewSecretChat {
  fn as_ref(&self) -> &TGCreateNewSecretChat { self }
}

impl AsRef<TGCreateNewSecretChat> for _TGCreateNewSecretChatBuilder {
  fn as_ref(&self) -> &TGCreateNewSecretChat { &self.inner }
}

impl TGCreateNewSecretChat {

  pub fn builder() -> _TGCreateNewSecretChatBuilder {
    _TGCreateNewSecretChatBuilder { inner: Self::new(CreateNewSecretChat::_new()) }
  }

  pub fn new(inner: CreateNewSecretChat) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &CreateNewSecretChat { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut CreateNewSecretChat { &mut self.inner }

}


#[doc(hidden)] pub struct _TGCreateNewStickerSetBuilder { inner: TGCreateNewStickerSet }

impl _TGCreateNewStickerSetBuilder {

  pub fn build(&self) -> TGCreateNewStickerSet { self.inner.clone() }

  ///  Sticker set owner. 
  pub fn user_id(&mut self, user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_user_id(user_id);
    self
  }
  ///  Sticker set title; 1-64 characters. 
  pub fn title<S: AsRef<str>>(&mut self, title: S) -> &mut Self {
    self.inner.td_origin_mut()._set_title(title.as_ref().to_string());
    self
  }
  ///  Sticker set name. Can contain only English letters, digits and underscores. Must end with "by<bot username>" (<bot_username> is case insensitive); 1-64 characters. 
  pub fn name<S: AsRef<str>>(&mut self, name: S) -> &mut Self {
    self.inner.td_origin_mut()._set_name(name.as_ref().to_string());
    self
  }
  ///  True, if stickers are masks. 
  pub fn is_masks(&mut self, is_masks: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_is_masks(is_masks);
    self
  }
  

  
  // [stickers] type is [Vec<InputSticker>], is not support, need add manully.
  #[doc(hidden)] pub fn _stickers(&mut self, stickers: Vec<InputSticker>) -> &mut Self {
    self.inner.td_origin_mut()._set_stickers(stickers);
    self
  }
  
}


///  Creates a new sticker set; for bots only. Returns the newly created sticker set. 
#[derive(Debug, Clone)]
pub struct TGCreateNewStickerSet {
  inner: CreateNewStickerSet
}

impl TDFB for TGCreateNewStickerSet {}

impl AsRef<TGCreateNewStickerSet> for TGCreateNewStickerSet {
  fn as_ref(&self) -> &TGCreateNewStickerSet { self }
}

impl AsRef<TGCreateNewStickerSet> for _TGCreateNewStickerSetBuilder {
  fn as_ref(&self) -> &TGCreateNewStickerSet { &self.inner }
}

impl TGCreateNewStickerSet {

  pub fn builder() -> _TGCreateNewStickerSetBuilder {
    _TGCreateNewStickerSetBuilder { inner: Self::new(CreateNewStickerSet::_new()) }
  }

  pub fn new(inner: CreateNewStickerSet) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &CreateNewStickerSet { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut CreateNewStickerSet { &mut self.inner }

}


#[doc(hidden)] pub struct _TGCreateNewSupergroupChatBuilder { inner: TGCreateNewSupergroupChat }

impl _TGCreateNewSupergroupChatBuilder {

  pub fn build(&self) -> TGCreateNewSupergroupChat { self.inner.clone() }

  ///  Title of the new chat; 1-128 characters. 
  pub fn title<S: AsRef<str>>(&mut self, title: S) -> &mut Self {
    self.inner.td_origin_mut()._set_title(title.as_ref().to_string());
    self
  }
  ///  True, if a channel chat should be created. 
  pub fn is_channel(&mut self, is_channel: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_is_channel(is_channel);
    self
  }
  ///  Chat description; 0-255 characters. 
  pub fn description<S: AsRef<str>>(&mut self, description: S) -> &mut Self {
    self.inner.td_origin_mut()._set_description(description.as_ref().to_string());
    self
  }
  

  
}


///  Creates a new supergroup or channel and sends a corresponding  
#[derive(Debug, Clone)]
pub struct TGCreateNewSupergroupChat {
  inner: CreateNewSupergroupChat
}

impl TDFB for TGCreateNewSupergroupChat {}

impl AsRef<TGCreateNewSupergroupChat> for TGCreateNewSupergroupChat {
  fn as_ref(&self) -> &TGCreateNewSupergroupChat { self }
}

impl AsRef<TGCreateNewSupergroupChat> for _TGCreateNewSupergroupChatBuilder {
  fn as_ref(&self) -> &TGCreateNewSupergroupChat { &self.inner }
}

impl TGCreateNewSupergroupChat {

  pub fn builder() -> _TGCreateNewSupergroupChatBuilder {
    _TGCreateNewSupergroupChatBuilder { inner: Self::new(CreateNewSupergroupChat::_new()) }
  }

  pub fn new(inner: CreateNewSupergroupChat) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &CreateNewSupergroupChat { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut CreateNewSupergroupChat { &mut self.inner }

}


#[doc(hidden)] pub struct _TGCreatePrivateChatBuilder { inner: TGCreatePrivateChat }

impl _TGCreatePrivateChatBuilder {

  pub fn build(&self) -> TGCreatePrivateChat { self.inner.clone() }

  ///  User identifier. 
  pub fn user_id(&mut self, user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_user_id(user_id);
    self
  }
  ///  If true, the chat will be created without network request. In this case all information about the chat except its type, title and photo can be incorrect. 
  pub fn force(&mut self, force: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_force(force);
    self
  }
  

  
}


///  Returns an existing chat corresponding to a given user. 
#[derive(Debug, Clone)]
pub struct TGCreatePrivateChat {
  inner: CreatePrivateChat
}

impl TDFB for TGCreatePrivateChat {}

impl AsRef<TGCreatePrivateChat> for TGCreatePrivateChat {
  fn as_ref(&self) -> &TGCreatePrivateChat { self }
}

impl AsRef<TGCreatePrivateChat> for _TGCreatePrivateChatBuilder {
  fn as_ref(&self) -> &TGCreatePrivateChat { &self.inner }
}

impl TGCreatePrivateChat {

  pub fn builder() -> _TGCreatePrivateChatBuilder {
    _TGCreatePrivateChatBuilder { inner: Self::new(CreatePrivateChat::_new()) }
  }

  pub fn new(inner: CreatePrivateChat) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &CreatePrivateChat { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut CreatePrivateChat { &mut self.inner }

}


#[doc(hidden)] pub struct _TGCreateSecretChatBuilder { inner: TGCreateSecretChat }

impl _TGCreateSecretChatBuilder {

  pub fn build(&self) -> TGCreateSecretChat { self.inner.clone() }

  ///  Secret chat identifier. 
  pub fn secret_chat_id(&mut self, secret_chat_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_secret_chat_id(secret_chat_id);
    self
  }
  

  
}


///  Returns an existing chat corresponding to a known secret chat. 
#[derive(Debug, Clone)]
pub struct TGCreateSecretChat {
  inner: CreateSecretChat
}

impl TDFB for TGCreateSecretChat {}

impl AsRef<TGCreateSecretChat> for TGCreateSecretChat {
  fn as_ref(&self) -> &TGCreateSecretChat { self }
}

impl AsRef<TGCreateSecretChat> for _TGCreateSecretChatBuilder {
  fn as_ref(&self) -> &TGCreateSecretChat { &self.inner }
}

impl TGCreateSecretChat {

  pub fn builder() -> _TGCreateSecretChatBuilder {
    _TGCreateSecretChatBuilder { inner: Self::new(CreateSecretChat::_new()) }
  }

  pub fn new(inner: CreateSecretChat) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &CreateSecretChat { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut CreateSecretChat { &mut self.inner }

}


#[doc(hidden)] pub struct _TGCreateSupergroupChatBuilder { inner: TGCreateSupergroupChat }

impl _TGCreateSupergroupChatBuilder {

  pub fn build(&self) -> TGCreateSupergroupChat { self.inner.clone() }

  ///  Supergroup or channel identifier. 
  pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_supergroup_id(supergroup_id);
    self
  }
  ///  If true, the chat will be created without network request. In this case all information about the chat except its type, title and photo can be incorrect. 
  pub fn force(&mut self, force: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_force(force);
    self
  }
  

  
}


///  Returns an existing chat corresponding to a known supergroup or channel. 
#[derive(Debug, Clone)]
pub struct TGCreateSupergroupChat {
  inner: CreateSupergroupChat
}

impl TDFB for TGCreateSupergroupChat {}

impl AsRef<TGCreateSupergroupChat> for TGCreateSupergroupChat {
  fn as_ref(&self) -> &TGCreateSupergroupChat { self }
}

impl AsRef<TGCreateSupergroupChat> for _TGCreateSupergroupChatBuilder {
  fn as_ref(&self) -> &TGCreateSupergroupChat { &self.inner }
}

impl TGCreateSupergroupChat {

  pub fn builder() -> _TGCreateSupergroupChatBuilder {
    _TGCreateSupergroupChatBuilder { inner: Self::new(CreateSupergroupChat::_new()) }
  }

  pub fn new(inner: CreateSupergroupChat) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &CreateSupergroupChat { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut CreateSupergroupChat { &mut self.inner }

}


#[doc(hidden)] pub struct _TGCreateTemporaryPasswordBuilder { inner: TGCreateTemporaryPassword }

impl _TGCreateTemporaryPasswordBuilder {

  pub fn build(&self) -> TGCreateTemporaryPassword { self.inner.clone() }

  ///  Persistent user password. 
  pub fn password<S: AsRef<str>>(&mut self, password: S) -> &mut Self {
    self.inner.td_origin_mut()._set_password(password.as_ref().to_string());
    self
  }
  ///  Time during which the temporary password will be valid, in seconds; should be between 60 and 86400. 
  pub fn valid_for(&mut self, valid_for: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_valid_for(valid_for);
    self
  }
  

  
}


///  Creates a new temporary password for processing payments. 
#[derive(Debug, Clone)]
pub struct TGCreateTemporaryPassword {
  inner: CreateTemporaryPassword
}

impl TDFB for TGCreateTemporaryPassword {}

impl AsRef<TGCreateTemporaryPassword> for TGCreateTemporaryPassword {
  fn as_ref(&self) -> &TGCreateTemporaryPassword { self }
}

impl AsRef<TGCreateTemporaryPassword> for _TGCreateTemporaryPasswordBuilder {
  fn as_ref(&self) -> &TGCreateTemporaryPassword { &self.inner }
}

impl TGCreateTemporaryPassword {

  pub fn builder() -> _TGCreateTemporaryPasswordBuilder {
    _TGCreateTemporaryPasswordBuilder { inner: Self::new(CreateTemporaryPassword::_new()) }
  }

  pub fn new(inner: CreateTemporaryPassword) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &CreateTemporaryPassword { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut CreateTemporaryPassword { &mut self.inner }

}


#[doc(hidden)] pub struct _TGDeleteAccountBuilder { inner: TGDeleteAccount }

impl _TGDeleteAccountBuilder {

  pub fn build(&self) -> TGDeleteAccount { self.inner.clone() }

  ///  The reason why the account was deleted; optional. 
  pub fn reason<S: AsRef<str>>(&mut self, reason: S) -> &mut Self {
    self.inner.td_origin_mut()._set_reason(reason.as_ref().to_string());
    self
  }
  

  
}


///  Deletes the account of the current user, deleting all information associated with the user from the server. The phone number of the account can be used to create a new account. Can be called before authorization when the current authorization state is  
#[derive(Debug, Clone)]
pub struct TGDeleteAccount {
  inner: DeleteAccount
}

impl TDFB for TGDeleteAccount {}

impl AsRef<TGDeleteAccount> for TGDeleteAccount {
  fn as_ref(&self) -> &TGDeleteAccount { self }
}

impl AsRef<TGDeleteAccount> for _TGDeleteAccountBuilder {
  fn as_ref(&self) -> &TGDeleteAccount { &self.inner }
}

impl TGDeleteAccount {

  pub fn builder() -> _TGDeleteAccountBuilder {
    _TGDeleteAccountBuilder { inner: Self::new(DeleteAccount::_new()) }
  }

  pub fn new(inner: DeleteAccount) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &DeleteAccount { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut DeleteAccount { &mut self.inner }

}


#[doc(hidden)] pub struct _TGDeleteChatHistoryBuilder { inner: TGDeleteChatHistory }

impl _TGDeleteChatHistoryBuilder {

  pub fn build(&self) -> TGDeleteChatHistory { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Pass true if the chat should be removed from the chat list. 
  pub fn remove_from_chat_list(&mut self, remove_from_chat_list: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_remove_from_chat_list(remove_from_chat_list);
    self
  }
  ///  Pass true to try to delete chat history for all users. 
  pub fn revoke(&mut self, revoke: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_revoke(revoke);
    self
  }
  

  
}


///  Deletes all messages in the chat. Use Chat.can_be_deleted_only_for_self and Chat.can_be_deleted_for_all_users fields to find whether and how the method can be applied to the chat. 
#[derive(Debug, Clone)]
pub struct TGDeleteChatHistory {
  inner: DeleteChatHistory
}

impl TDFB for TGDeleteChatHistory {}

impl AsRef<TGDeleteChatHistory> for TGDeleteChatHistory {
  fn as_ref(&self) -> &TGDeleteChatHistory { self }
}

impl AsRef<TGDeleteChatHistory> for _TGDeleteChatHistoryBuilder {
  fn as_ref(&self) -> &TGDeleteChatHistory { &self.inner }
}

impl TGDeleteChatHistory {

  pub fn builder() -> _TGDeleteChatHistoryBuilder {
    _TGDeleteChatHistoryBuilder { inner: Self::new(DeleteChatHistory::_new()) }
  }

  pub fn new(inner: DeleteChatHistory) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &DeleteChatHistory { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut DeleteChatHistory { &mut self.inner }

}


#[doc(hidden)] pub struct _TGDeleteChatMessagesFromUserBuilder { inner: TGDeleteChatMessagesFromUser }

impl _TGDeleteChatMessagesFromUserBuilder {

  pub fn build(&self) -> TGDeleteChatMessagesFromUser { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  User identifier. 
  pub fn user_id(&mut self, user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_user_id(user_id);
    self
  }
  

  
}


///  Deletes all messages sent by the specified user to a chat. Supported only in supergroups; requires can_delete_messages administrator privileges. 
#[derive(Debug, Clone)]
pub struct TGDeleteChatMessagesFromUser {
  inner: DeleteChatMessagesFromUser
}

impl TDFB for TGDeleteChatMessagesFromUser {}

impl AsRef<TGDeleteChatMessagesFromUser> for TGDeleteChatMessagesFromUser {
  fn as_ref(&self) -> &TGDeleteChatMessagesFromUser { self }
}

impl AsRef<TGDeleteChatMessagesFromUser> for _TGDeleteChatMessagesFromUserBuilder {
  fn as_ref(&self) -> &TGDeleteChatMessagesFromUser { &self.inner }
}

impl TGDeleteChatMessagesFromUser {

  pub fn builder() -> _TGDeleteChatMessagesFromUserBuilder {
    _TGDeleteChatMessagesFromUserBuilder { inner: Self::new(DeleteChatMessagesFromUser::_new()) }
  }

  pub fn new(inner: DeleteChatMessagesFromUser) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &DeleteChatMessagesFromUser { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut DeleteChatMessagesFromUser { &mut self.inner }

}


#[doc(hidden)] pub struct _TGDeleteChatReplyMarkupBuilder { inner: TGDeleteChatReplyMarkup }

impl _TGDeleteChatReplyMarkupBuilder {

  pub fn build(&self) -> TGDeleteChatReplyMarkup { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  The message identifier of the used keyboard. 
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_message_id(message_id);
    self
  }
  

  
}


///  Deletes the default reply markup from a chat. Must be called after a one-time keyboard or a ForceReply reply markup has been used. UpdateChatReplyMarkup will be sent if the reply markup will be changed. 
#[derive(Debug, Clone)]
pub struct TGDeleteChatReplyMarkup {
  inner: DeleteChatReplyMarkup
}

impl TDFB for TGDeleteChatReplyMarkup {}

impl AsRef<TGDeleteChatReplyMarkup> for TGDeleteChatReplyMarkup {
  fn as_ref(&self) -> &TGDeleteChatReplyMarkup { self }
}

impl AsRef<TGDeleteChatReplyMarkup> for _TGDeleteChatReplyMarkupBuilder {
  fn as_ref(&self) -> &TGDeleteChatReplyMarkup { &self.inner }
}

impl TGDeleteChatReplyMarkup {

  pub fn builder() -> _TGDeleteChatReplyMarkupBuilder {
    _TGDeleteChatReplyMarkupBuilder { inner: Self::new(DeleteChatReplyMarkup::_new()) }
  }

  pub fn new(inner: DeleteChatReplyMarkup) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &DeleteChatReplyMarkup { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut DeleteChatReplyMarkup { &mut self.inner }

}


#[doc(hidden)] pub struct _TGDeleteFileBuilder { inner: TGDeleteFile }

impl _TGDeleteFileBuilder {

  pub fn build(&self) -> TGDeleteFile { self.inner.clone() }

  ///  Identifier of the file to delete. 
  pub fn file_id(&mut self, file_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_file_id(file_id);
    self
  }
  

  
}


///  Deletes a file from the TDLib file cache. 
#[derive(Debug, Clone)]
pub struct TGDeleteFile {
  inner: DeleteFile
}

impl TDFB for TGDeleteFile {}

impl AsRef<TGDeleteFile> for TGDeleteFile {
  fn as_ref(&self) -> &TGDeleteFile { self }
}

impl AsRef<TGDeleteFile> for _TGDeleteFileBuilder {
  fn as_ref(&self) -> &TGDeleteFile { &self.inner }
}

impl TGDeleteFile {

  pub fn builder() -> _TGDeleteFileBuilder {
    _TGDeleteFileBuilder { inner: Self::new(DeleteFile::_new()) }
  }

  pub fn new(inner: DeleteFile) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &DeleteFile { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut DeleteFile { &mut self.inner }

}


#[doc(hidden)] pub struct _TGDeleteLanguagePackBuilder { inner: TGDeleteLanguagePack }

impl _TGDeleteLanguagePackBuilder {

  pub fn build(&self) -> TGDeleteLanguagePack { self.inner.clone() }

  ///  Identifier of the language pack to delete. 
  pub fn language_pack_id<S: AsRef<str>>(&mut self, language_pack_id: S) -> &mut Self {
    self.inner.td_origin_mut()._set_language_pack_id(language_pack_id.as_ref().to_string());
    self
  }
  

  
}


///  Deletes all information about a language pack in the current localization target. The language pack which is currently in use (including base language pack) or is being synchronized can't be deleted. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGDeleteLanguagePack {
  inner: DeleteLanguagePack
}

impl TDFB for TGDeleteLanguagePack {}

impl AsRef<TGDeleteLanguagePack> for TGDeleteLanguagePack {
  fn as_ref(&self) -> &TGDeleteLanguagePack { self }
}

impl AsRef<TGDeleteLanguagePack> for _TGDeleteLanguagePackBuilder {
  fn as_ref(&self) -> &TGDeleteLanguagePack { &self.inner }
}

impl TGDeleteLanguagePack {

  pub fn builder() -> _TGDeleteLanguagePackBuilder {
    _TGDeleteLanguagePackBuilder { inner: Self::new(DeleteLanguagePack::_new()) }
  }

  pub fn new(inner: DeleteLanguagePack) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &DeleteLanguagePack { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut DeleteLanguagePack { &mut self.inner }

}


#[doc(hidden)] pub struct _TGDeleteMessagesBuilder { inner: TGDeleteMessages }

impl _TGDeleteMessagesBuilder {

  pub fn build(&self) -> TGDeleteMessages { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Identifiers of the messages to be deleted. 
  pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self {
    self.inner.td_origin_mut()._set_message_ids(message_ids);
    self
  }
  ///  Pass true to try to delete messages for all chat members. Always true for supergroups, channels and secret chats. 
  pub fn revoke(&mut self, revoke: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_revoke(revoke);
    self
  }
  

  
}


///  Deletes messages. 
#[derive(Debug, Clone)]
pub struct TGDeleteMessages {
  inner: DeleteMessages
}

impl TDFB for TGDeleteMessages {}

impl AsRef<TGDeleteMessages> for TGDeleteMessages {
  fn as_ref(&self) -> &TGDeleteMessages { self }
}

impl AsRef<TGDeleteMessages> for _TGDeleteMessagesBuilder {
  fn as_ref(&self) -> &TGDeleteMessages { &self.inner }
}

impl TGDeleteMessages {

  pub fn builder() -> _TGDeleteMessagesBuilder {
    _TGDeleteMessagesBuilder { inner: Self::new(DeleteMessages::_new()) }
  }

  pub fn new(inner: DeleteMessages) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &DeleteMessages { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut DeleteMessages { &mut self.inner }

}


#[doc(hidden)] pub struct _TGDeletePassportElementBuilder { inner: TGDeletePassportElement }

impl _TGDeletePassportElementBuilder {

  pub fn build(&self) -> TGDeletePassportElement { self.inner.clone() }

  

  
  // [type_] type is [Box<PassportElementType>], is not support, need add manully.
  #[doc(hidden)] pub fn _type_(&mut self, type_: Box<PassportElementType>) -> &mut Self {
    self.inner.td_origin_mut()._set_type_(type_);
    self
  }
  
}


///  Deletes a Telegram Passport element. 
#[derive(Debug, Clone)]
pub struct TGDeletePassportElement {
  inner: DeletePassportElement
}

impl TDFB for TGDeletePassportElement {}

impl AsRef<TGDeletePassportElement> for TGDeletePassportElement {
  fn as_ref(&self) -> &TGDeletePassportElement { self }
}

impl AsRef<TGDeletePassportElement> for _TGDeletePassportElementBuilder {
  fn as_ref(&self) -> &TGDeletePassportElement { &self.inner }
}

impl TGDeletePassportElement {

  pub fn builder() -> _TGDeletePassportElementBuilder {
    _TGDeletePassportElementBuilder { inner: Self::new(DeletePassportElement::_new()) }
  }

  pub fn new(inner: DeletePassportElement) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &DeletePassportElement { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut DeletePassportElement { &mut self.inner }

}


#[doc(hidden)] pub struct _TGDeleteProfilePhotoBuilder { inner: TGDeleteProfilePhoto }

impl _TGDeleteProfilePhotoBuilder {

  pub fn build(&self) -> TGDeleteProfilePhoto { self.inner.clone() }

  ///  Identifier of the profile photo to delete. 
  pub fn profile_photo_id(&mut self, profile_photo_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_profile_photo_id(profile_photo_id);
    self
  }
  

  
}


///  Deletes a profile photo. If something changes,  
#[derive(Debug, Clone)]
pub struct TGDeleteProfilePhoto {
  inner: DeleteProfilePhoto
}

impl TDFB for TGDeleteProfilePhoto {}

impl AsRef<TGDeleteProfilePhoto> for TGDeleteProfilePhoto {
  fn as_ref(&self) -> &TGDeleteProfilePhoto { self }
}

impl AsRef<TGDeleteProfilePhoto> for _TGDeleteProfilePhotoBuilder {
  fn as_ref(&self) -> &TGDeleteProfilePhoto { &self.inner }
}

impl TGDeleteProfilePhoto {

  pub fn builder() -> _TGDeleteProfilePhotoBuilder {
    _TGDeleteProfilePhotoBuilder { inner: Self::new(DeleteProfilePhoto::_new()) }
  }

  pub fn new(inner: DeleteProfilePhoto) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &DeleteProfilePhoto { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut DeleteProfilePhoto { &mut self.inner }

}


#[doc(hidden)] pub struct _TGDeleteSavedCredentialsBuilder { inner: TGDeleteSavedCredentials }

impl _TGDeleteSavedCredentialsBuilder {

  pub fn build(&self) -> TGDeleteSavedCredentials { self.inner.clone() }

  

  
}


///  Deletes saved credentials for all payment provider bots. 
#[derive(Debug, Clone)]
pub struct TGDeleteSavedCredentials {
  inner: DeleteSavedCredentials
}

impl TDFB for TGDeleteSavedCredentials {}

impl AsRef<TGDeleteSavedCredentials> for TGDeleteSavedCredentials {
  fn as_ref(&self) -> &TGDeleteSavedCredentials { self }
}

impl AsRef<TGDeleteSavedCredentials> for _TGDeleteSavedCredentialsBuilder {
  fn as_ref(&self) -> &TGDeleteSavedCredentials { &self.inner }
}

impl TGDeleteSavedCredentials {

  pub fn builder() -> _TGDeleteSavedCredentialsBuilder {
    _TGDeleteSavedCredentialsBuilder { inner: Self::new(DeleteSavedCredentials::_new()) }
  }

  pub fn new(inner: DeleteSavedCredentials) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &DeleteSavedCredentials { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut DeleteSavedCredentials { &mut self.inner }

}


#[doc(hidden)] pub struct _TGDeleteSavedOrderInfoBuilder { inner: TGDeleteSavedOrderInfo }

impl _TGDeleteSavedOrderInfoBuilder {

  pub fn build(&self) -> TGDeleteSavedOrderInfo { self.inner.clone() }

  

  
}


///  Deletes saved order info. 
#[derive(Debug, Clone)]
pub struct TGDeleteSavedOrderInfo {
  inner: DeleteSavedOrderInfo
}

impl TDFB for TGDeleteSavedOrderInfo {}

impl AsRef<TGDeleteSavedOrderInfo> for TGDeleteSavedOrderInfo {
  fn as_ref(&self) -> &TGDeleteSavedOrderInfo { self }
}

impl AsRef<TGDeleteSavedOrderInfo> for _TGDeleteSavedOrderInfoBuilder {
  fn as_ref(&self) -> &TGDeleteSavedOrderInfo { &self.inner }
}

impl TGDeleteSavedOrderInfo {

  pub fn builder() -> _TGDeleteSavedOrderInfoBuilder {
    _TGDeleteSavedOrderInfoBuilder { inner: Self::new(DeleteSavedOrderInfo::_new()) }
  }

  pub fn new(inner: DeleteSavedOrderInfo) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &DeleteSavedOrderInfo { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut DeleteSavedOrderInfo { &mut self.inner }

}


#[doc(hidden)] pub struct _TGDeleteSupergroupBuilder { inner: TGDeleteSupergroup }

impl _TGDeleteSupergroupBuilder {

  pub fn build(&self) -> TGDeleteSupergroup { self.inner.clone() }

  ///  Identifier of the supergroup or channel. 
  pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_supergroup_id(supergroup_id);
    self
  }
  

  
}


///  Deletes a supergroup or channel along with all messages in the corresponding chat. This will release the supergroup or channel username and remove all members; requires creator privileges in the supergroup or channel. Chats with more than 1000 members can't be deleted using this method. 
#[derive(Debug, Clone)]
pub struct TGDeleteSupergroup {
  inner: DeleteSupergroup
}

impl TDFB for TGDeleteSupergroup {}

impl AsRef<TGDeleteSupergroup> for TGDeleteSupergroup {
  fn as_ref(&self) -> &TGDeleteSupergroup { self }
}

impl AsRef<TGDeleteSupergroup> for _TGDeleteSupergroupBuilder {
  fn as_ref(&self) -> &TGDeleteSupergroup { &self.inner }
}

impl TGDeleteSupergroup {

  pub fn builder() -> _TGDeleteSupergroupBuilder {
    _TGDeleteSupergroupBuilder { inner: Self::new(DeleteSupergroup::_new()) }
  }

  pub fn new(inner: DeleteSupergroup) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &DeleteSupergroup { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut DeleteSupergroup { &mut self.inner }

}


#[doc(hidden)] pub struct _TGDestroyBuilder { inner: TGDestroy }

impl _TGDestroyBuilder {

  pub fn build(&self) -> TGDestroy { self.inner.clone() }

  

  
}


///  Closes the TDLib instance, destroying all local data without a proper logout. The current user session will remain in the list of all active sessions. All local data will be destroyed. After the destruction completes  
#[derive(Debug, Clone)]
pub struct TGDestroy {
  inner: Destroy
}

impl TDFB for TGDestroy {}

impl AsRef<TGDestroy> for TGDestroy {
  fn as_ref(&self) -> &TGDestroy { self }
}

impl AsRef<TGDestroy> for _TGDestroyBuilder {
  fn as_ref(&self) -> &TGDestroy { &self.inner }
}

impl TGDestroy {

  pub fn builder() -> _TGDestroyBuilder {
    _TGDestroyBuilder { inner: Self::new(Destroy::_new()) }
  }

  pub fn new(inner: Destroy) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &Destroy { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut Destroy { &mut self.inner }

}


#[doc(hidden)] pub struct _TGDisableProxyBuilder { inner: TGDisableProxy }

impl _TGDisableProxyBuilder {

  pub fn build(&self) -> TGDisableProxy { self.inner.clone() }

  

  
}


///  Disables the currently enabled proxy. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGDisableProxy {
  inner: DisableProxy
}

impl TDFB for TGDisableProxy {}

impl AsRef<TGDisableProxy> for TGDisableProxy {
  fn as_ref(&self) -> &TGDisableProxy { self }
}

impl AsRef<TGDisableProxy> for _TGDisableProxyBuilder {
  fn as_ref(&self) -> &TGDisableProxy { &self.inner }
}

impl TGDisableProxy {

  pub fn builder() -> _TGDisableProxyBuilder {
    _TGDisableProxyBuilder { inner: Self::new(DisableProxy::_new()) }
  }

  pub fn new(inner: DisableProxy) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &DisableProxy { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut DisableProxy { &mut self.inner }

}


#[doc(hidden)] pub struct _TGDiscardCallBuilder { inner: TGDiscardCall }

impl _TGDiscardCallBuilder {

  pub fn build(&self) -> TGDiscardCall { self.inner.clone() }

  ///  Call identifier. 
  pub fn call_id(&mut self, call_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_call_id(call_id);
    self
  }
  ///  True, if the user was disconnected. 
  pub fn is_disconnected(&mut self, is_disconnected: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_is_disconnected(is_disconnected);
    self
  }
  ///  The call duration, in seconds. 
  pub fn duration(&mut self, duration: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_duration(duration);
    self
  }
  ///  Identifier of the connection used during the call. 
  pub fn connection_id(&mut self, connection_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_connection_id(connection_id);
    self
  }
  

  
}


///  Discards a call. 
#[derive(Debug, Clone)]
pub struct TGDiscardCall {
  inner: DiscardCall
}

impl TDFB for TGDiscardCall {}

impl AsRef<TGDiscardCall> for TGDiscardCall {
  fn as_ref(&self) -> &TGDiscardCall { self }
}

impl AsRef<TGDiscardCall> for _TGDiscardCallBuilder {
  fn as_ref(&self) -> &TGDiscardCall { &self.inner }
}

impl TGDiscardCall {

  pub fn builder() -> _TGDiscardCallBuilder {
    _TGDiscardCallBuilder { inner: Self::new(DiscardCall::_new()) }
  }

  pub fn new(inner: DiscardCall) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &DiscardCall { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut DiscardCall { &mut self.inner }

}


#[doc(hidden)] pub struct _TGDisconnectAllWebsitesBuilder { inner: TGDisconnectAllWebsites }

impl _TGDisconnectAllWebsitesBuilder {

  pub fn build(&self) -> TGDisconnectAllWebsites { self.inner.clone() }

  

  
}


///  Disconnects all websites from the current user's Telegram account. 
#[derive(Debug, Clone)]
pub struct TGDisconnectAllWebsites {
  inner: DisconnectAllWebsites
}

impl TDFB for TGDisconnectAllWebsites {}

impl AsRef<TGDisconnectAllWebsites> for TGDisconnectAllWebsites {
  fn as_ref(&self) -> &TGDisconnectAllWebsites { self }
}

impl AsRef<TGDisconnectAllWebsites> for _TGDisconnectAllWebsitesBuilder {
  fn as_ref(&self) -> &TGDisconnectAllWebsites { &self.inner }
}

impl TGDisconnectAllWebsites {

  pub fn builder() -> _TGDisconnectAllWebsitesBuilder {
    _TGDisconnectAllWebsitesBuilder { inner: Self::new(DisconnectAllWebsites::_new()) }
  }

  pub fn new(inner: DisconnectAllWebsites) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &DisconnectAllWebsites { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut DisconnectAllWebsites { &mut self.inner }

}


#[doc(hidden)] pub struct _TGDisconnectWebsiteBuilder { inner: TGDisconnectWebsite }

impl _TGDisconnectWebsiteBuilder {

  pub fn build(&self) -> TGDisconnectWebsite { self.inner.clone() }

  ///  Website identifier. 
  pub fn website_id(&mut self, website_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_website_id(website_id);
    self
  }
  

  
}


///  Disconnects website from the current user's Telegram account. 
#[derive(Debug, Clone)]
pub struct TGDisconnectWebsite {
  inner: DisconnectWebsite
}

impl TDFB for TGDisconnectWebsite {}

impl AsRef<TGDisconnectWebsite> for TGDisconnectWebsite {
  fn as_ref(&self) -> &TGDisconnectWebsite { self }
}

impl AsRef<TGDisconnectWebsite> for _TGDisconnectWebsiteBuilder {
  fn as_ref(&self) -> &TGDisconnectWebsite { &self.inner }
}

impl TGDisconnectWebsite {

  pub fn builder() -> _TGDisconnectWebsiteBuilder {
    _TGDisconnectWebsiteBuilder { inner: Self::new(DisconnectWebsite::_new()) }
  }

  pub fn new(inner: DisconnectWebsite) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &DisconnectWebsite { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut DisconnectWebsite { &mut self.inner }

}


#[doc(hidden)] pub struct _TGDownloadFileBuilder { inner: TGDownloadFile }

impl _TGDownloadFileBuilder {

  pub fn build(&self) -> TGDownloadFile { self.inner.clone() }

  ///  Identifier of the file to download. 
  pub fn file_id(&mut self, file_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_file_id(file_id);
    self
  }
  ///  Priority of the download (1-32). The higher the priority, the earlier the file will be downloaded. If the priorities of two files are equal, then the last one for which downloadFile was called will be downloaded first. 
  pub fn priority(&mut self, priority: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_priority(priority);
    self
  }
  ///  The starting position from which the file should be downloaded. 
  pub fn offset(&mut self, offset: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_offset(offset);
    self
  }
  ///  Number of bytes which should be downloaded starting from the "offset" position before the download will be automatically cancelled; use 0 to download without a limit. 
  pub fn limit(&mut self, limit: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_limit(limit);
    self
  }
  ///  If false, this request returns file state just after the download has been started. If true, this request returns file state only after the download has succeeded, has failed, has been cancelled or a new downloadFile request with different offset/limit parameters was sent. 
  pub fn synchronous(&mut self, synchronous: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_synchronous(synchronous);
    self
  }
  

  
}


///  Downloads a file from the cloud. Download progress and completion of the download will be notified through  
#[derive(Debug, Clone)]
pub struct TGDownloadFile {
  inner: DownloadFile
}

impl TDFB for TGDownloadFile {}

impl AsRef<TGDownloadFile> for TGDownloadFile {
  fn as_ref(&self) -> &TGDownloadFile { self }
}

impl AsRef<TGDownloadFile> for _TGDownloadFileBuilder {
  fn as_ref(&self) -> &TGDownloadFile { &self.inner }
}

impl TGDownloadFile {

  pub fn builder() -> _TGDownloadFileBuilder {
    _TGDownloadFileBuilder { inner: Self::new(DownloadFile::_new()) }
  }

  pub fn new(inner: DownloadFile) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &DownloadFile { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut DownloadFile { &mut self.inner }

}


#[doc(hidden)] pub struct _TGEditCustomLanguagePackInfoBuilder { inner: TGEditCustomLanguagePackInfo }

impl _TGEditCustomLanguagePackInfoBuilder {

  pub fn build(&self) -> TGEditCustomLanguagePackInfo { self.inner.clone() }

  

  
  // [info] type is [LanguagePackInfo], is not support, need add manully.
  #[doc(hidden)] pub fn _info(&mut self, info: LanguagePackInfo) -> &mut Self {
    self.inner.td_origin_mut()._set_info(info);
    self
  }
  
}


///  Edits information about a custom local language pack in the current localization target. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGEditCustomLanguagePackInfo {
  inner: EditCustomLanguagePackInfo
}

impl TDFB for TGEditCustomLanguagePackInfo {}

impl AsRef<TGEditCustomLanguagePackInfo> for TGEditCustomLanguagePackInfo {
  fn as_ref(&self) -> &TGEditCustomLanguagePackInfo { self }
}

impl AsRef<TGEditCustomLanguagePackInfo> for _TGEditCustomLanguagePackInfoBuilder {
  fn as_ref(&self) -> &TGEditCustomLanguagePackInfo { &self.inner }
}

impl TGEditCustomLanguagePackInfo {

  pub fn builder() -> _TGEditCustomLanguagePackInfoBuilder {
    _TGEditCustomLanguagePackInfoBuilder { inner: Self::new(EditCustomLanguagePackInfo::_new()) }
  }

  pub fn new(inner: EditCustomLanguagePackInfo) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &EditCustomLanguagePackInfo { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut EditCustomLanguagePackInfo { &mut self.inner }

}


#[doc(hidden)] pub struct _TGEditInlineMessageCaptionBuilder { inner: TGEditInlineMessageCaption }

impl _TGEditInlineMessageCaptionBuilder {

  pub fn build(&self) -> TGEditInlineMessageCaption { self.inner.clone() }

  ///  Inline message identifier. 
  pub fn inline_message_id<S: AsRef<str>>(&mut self, inline_message_id: S) -> &mut Self {
    self.inner.td_origin_mut()._set_inline_message_id(inline_message_id.as_ref().to_string());
    self
  }
  

  
  // [reply_markup] type is [Box<ReplyMarkup>], is not support, need add manully.
  #[doc(hidden)] pub fn _reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self {
    self.inner.td_origin_mut()._set_reply_markup(reply_markup);
    self
  }
  
  // [caption] type is [FormattedText], is not support, need add manully.
  #[doc(hidden)] pub fn _caption(&mut self, caption: FormattedText) -> &mut Self {
    self.inner.td_origin_mut()._set_caption(caption);
    self
  }
  
}


///  Edits the caption of an inline message sent via a bot; for bots only. 
#[derive(Debug, Clone)]
pub struct TGEditInlineMessageCaption {
  inner: EditInlineMessageCaption
}

impl TDFB for TGEditInlineMessageCaption {}

impl AsRef<TGEditInlineMessageCaption> for TGEditInlineMessageCaption {
  fn as_ref(&self) -> &TGEditInlineMessageCaption { self }
}

impl AsRef<TGEditInlineMessageCaption> for _TGEditInlineMessageCaptionBuilder {
  fn as_ref(&self) -> &TGEditInlineMessageCaption { &self.inner }
}

impl TGEditInlineMessageCaption {

  pub fn builder() -> _TGEditInlineMessageCaptionBuilder {
    _TGEditInlineMessageCaptionBuilder { inner: Self::new(EditInlineMessageCaption::_new()) }
  }

  pub fn new(inner: EditInlineMessageCaption) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &EditInlineMessageCaption { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut EditInlineMessageCaption { &mut self.inner }

}


#[doc(hidden)] pub struct _TGEditInlineMessageLiveLocationBuilder { inner: TGEditInlineMessageLiveLocation }

impl _TGEditInlineMessageLiveLocationBuilder {

  pub fn build(&self) -> TGEditInlineMessageLiveLocation { self.inner.clone() }

  ///  Inline message identifier. 
  pub fn inline_message_id<S: AsRef<str>>(&mut self, inline_message_id: S) -> &mut Self {
    self.inner.td_origin_mut()._set_inline_message_id(inline_message_id.as_ref().to_string());
    self
  }
  

  
  // [reply_markup] type is [Box<ReplyMarkup>], is not support, need add manully.
  #[doc(hidden)] pub fn _reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self {
    self.inner.td_origin_mut()._set_reply_markup(reply_markup);
    self
  }
  
  // [location] type is [Location], is not support, need add manully.
  #[doc(hidden)] pub fn _location(&mut self, location: Location) -> &mut Self {
    self.inner.td_origin_mut()._set_location(location);
    self
  }
  
}


///  Edits the content of a live location in an inline message sent via a bot; for bots only. 
#[derive(Debug, Clone)]
pub struct TGEditInlineMessageLiveLocation {
  inner: EditInlineMessageLiveLocation
}

impl TDFB for TGEditInlineMessageLiveLocation {}

impl AsRef<TGEditInlineMessageLiveLocation> for TGEditInlineMessageLiveLocation {
  fn as_ref(&self) -> &TGEditInlineMessageLiveLocation { self }
}

impl AsRef<TGEditInlineMessageLiveLocation> for _TGEditInlineMessageLiveLocationBuilder {
  fn as_ref(&self) -> &TGEditInlineMessageLiveLocation { &self.inner }
}

impl TGEditInlineMessageLiveLocation {

  pub fn builder() -> _TGEditInlineMessageLiveLocationBuilder {
    _TGEditInlineMessageLiveLocationBuilder { inner: Self::new(EditInlineMessageLiveLocation::_new()) }
  }

  pub fn new(inner: EditInlineMessageLiveLocation) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &EditInlineMessageLiveLocation { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut EditInlineMessageLiveLocation { &mut self.inner }

}


#[doc(hidden)] pub struct _TGEditInlineMessageMediaBuilder { inner: TGEditInlineMessageMedia }

impl _TGEditInlineMessageMediaBuilder {

  pub fn build(&self) -> TGEditInlineMessageMedia { self.inner.clone() }

  ///  Inline message identifier. 
  pub fn inline_message_id<S: AsRef<str>>(&mut self, inline_message_id: S) -> &mut Self {
    self.inner.td_origin_mut()._set_inline_message_id(inline_message_id.as_ref().to_string());
    self
  }
  

  
  // [reply_markup] type is [Box<ReplyMarkup>], is not support, need add manully.
  #[doc(hidden)] pub fn _reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self {
    self.inner.td_origin_mut()._set_reply_markup(reply_markup);
    self
  }
  
  // [input_message_content] type is [Box<InputMessageContent>], is not support, need add manully.
  #[doc(hidden)] pub fn _input_message_content(&mut self, input_message_content: Box<InputMessageContent>) -> &mut Self {
    self.inner.td_origin_mut()._set_input_message_content(input_message_content);
    self
  }
  
}


///  Edits the content of a message with an animation, an audio, a document, a photo or a video in an inline message sent via a bot; for bots only. 
#[derive(Debug, Clone)]
pub struct TGEditInlineMessageMedia {
  inner: EditInlineMessageMedia
}

impl TDFB for TGEditInlineMessageMedia {}

impl AsRef<TGEditInlineMessageMedia> for TGEditInlineMessageMedia {
  fn as_ref(&self) -> &TGEditInlineMessageMedia { self }
}

impl AsRef<TGEditInlineMessageMedia> for _TGEditInlineMessageMediaBuilder {
  fn as_ref(&self) -> &TGEditInlineMessageMedia { &self.inner }
}

impl TGEditInlineMessageMedia {

  pub fn builder() -> _TGEditInlineMessageMediaBuilder {
    _TGEditInlineMessageMediaBuilder { inner: Self::new(EditInlineMessageMedia::_new()) }
  }

  pub fn new(inner: EditInlineMessageMedia) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &EditInlineMessageMedia { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut EditInlineMessageMedia { &mut self.inner }

}


#[doc(hidden)] pub struct _TGEditInlineMessageReplyMarkupBuilder { inner: TGEditInlineMessageReplyMarkup }

impl _TGEditInlineMessageReplyMarkupBuilder {

  pub fn build(&self) -> TGEditInlineMessageReplyMarkup { self.inner.clone() }

  ///  Inline message identifier. 
  pub fn inline_message_id<S: AsRef<str>>(&mut self, inline_message_id: S) -> &mut Self {
    self.inner.td_origin_mut()._set_inline_message_id(inline_message_id.as_ref().to_string());
    self
  }
  

  
  // [reply_markup] type is [Box<ReplyMarkup>], is not support, need add manully.
  #[doc(hidden)] pub fn _reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self {
    self.inner.td_origin_mut()._set_reply_markup(reply_markup);
    self
  }
  
}


///  Edits the reply markup of an inline message sent via a bot; for bots only. 
#[derive(Debug, Clone)]
pub struct TGEditInlineMessageReplyMarkup {
  inner: EditInlineMessageReplyMarkup
}

impl TDFB for TGEditInlineMessageReplyMarkup {}

impl AsRef<TGEditInlineMessageReplyMarkup> for TGEditInlineMessageReplyMarkup {
  fn as_ref(&self) -> &TGEditInlineMessageReplyMarkup { self }
}

impl AsRef<TGEditInlineMessageReplyMarkup> for _TGEditInlineMessageReplyMarkupBuilder {
  fn as_ref(&self) -> &TGEditInlineMessageReplyMarkup { &self.inner }
}

impl TGEditInlineMessageReplyMarkup {

  pub fn builder() -> _TGEditInlineMessageReplyMarkupBuilder {
    _TGEditInlineMessageReplyMarkupBuilder { inner: Self::new(EditInlineMessageReplyMarkup::_new()) }
  }

  pub fn new(inner: EditInlineMessageReplyMarkup) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &EditInlineMessageReplyMarkup { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut EditInlineMessageReplyMarkup { &mut self.inner }

}


#[doc(hidden)] pub struct _TGEditInlineMessageTextBuilder { inner: TGEditInlineMessageText }

impl _TGEditInlineMessageTextBuilder {

  pub fn build(&self) -> TGEditInlineMessageText { self.inner.clone() }

  ///  Inline message identifier. 
  pub fn inline_message_id<S: AsRef<str>>(&mut self, inline_message_id: S) -> &mut Self {
    self.inner.td_origin_mut()._set_inline_message_id(inline_message_id.as_ref().to_string());
    self
  }
  

  
  // [reply_markup] type is [Box<ReplyMarkup>], is not support, need add manully.
  #[doc(hidden)] pub fn _reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self {
    self.inner.td_origin_mut()._set_reply_markup(reply_markup);
    self
  }
  
  // [input_message_content] type is [Box<InputMessageContent>], is not support, need add manully.
  #[doc(hidden)] pub fn _input_message_content(&mut self, input_message_content: Box<InputMessageContent>) -> &mut Self {
    self.inner.td_origin_mut()._set_input_message_content(input_message_content);
    self
  }
  
}


///  Edits the text of an inline text or game message sent via a bot; for bots only. 
#[derive(Debug, Clone)]
pub struct TGEditInlineMessageText {
  inner: EditInlineMessageText
}

impl TDFB for TGEditInlineMessageText {}

impl AsRef<TGEditInlineMessageText> for TGEditInlineMessageText {
  fn as_ref(&self) -> &TGEditInlineMessageText { self }
}

impl AsRef<TGEditInlineMessageText> for _TGEditInlineMessageTextBuilder {
  fn as_ref(&self) -> &TGEditInlineMessageText { &self.inner }
}

impl TGEditInlineMessageText {

  pub fn builder() -> _TGEditInlineMessageTextBuilder {
    _TGEditInlineMessageTextBuilder { inner: Self::new(EditInlineMessageText::_new()) }
  }

  pub fn new(inner: EditInlineMessageText) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &EditInlineMessageText { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut EditInlineMessageText { &mut self.inner }

}


#[doc(hidden)] pub struct _TGEditMessageCaptionBuilder { inner: TGEditMessageCaption }

impl _TGEditMessageCaptionBuilder {

  pub fn build(&self) -> TGEditMessageCaption { self.inner.clone() }

  ///  The chat the message belongs to. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Identifier of the message. 
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_message_id(message_id);
    self
  }
  

  
  // [reply_markup] type is [Box<ReplyMarkup>], is not support, need add manully.
  #[doc(hidden)] pub fn _reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self {
    self.inner.td_origin_mut()._set_reply_markup(reply_markup);
    self
  }
  
  // [caption] type is [FormattedText], is not support, need add manully.
  #[doc(hidden)] pub fn _caption(&mut self, caption: FormattedText) -> &mut Self {
    self.inner.td_origin_mut()._set_caption(caption);
    self
  }
  
}


///  Edits the message content caption. Returns the edited message after the edit is completed on the server side. 
#[derive(Debug, Clone)]
pub struct TGEditMessageCaption {
  inner: EditMessageCaption
}

impl TDFB for TGEditMessageCaption {}

impl AsRef<TGEditMessageCaption> for TGEditMessageCaption {
  fn as_ref(&self) -> &TGEditMessageCaption { self }
}

impl AsRef<TGEditMessageCaption> for _TGEditMessageCaptionBuilder {
  fn as_ref(&self) -> &TGEditMessageCaption { &self.inner }
}

impl TGEditMessageCaption {

  pub fn builder() -> _TGEditMessageCaptionBuilder {
    _TGEditMessageCaptionBuilder { inner: Self::new(EditMessageCaption::_new()) }
  }

  pub fn new(inner: EditMessageCaption) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &EditMessageCaption { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut EditMessageCaption { &mut self.inner }

}


#[doc(hidden)] pub struct _TGEditMessageLiveLocationBuilder { inner: TGEditMessageLiveLocation }

impl _TGEditMessageLiveLocationBuilder {

  pub fn build(&self) -> TGEditMessageLiveLocation { self.inner.clone() }

  ///  The chat the message belongs to. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Identifier of the message. 
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_message_id(message_id);
    self
  }
  

  
  // [reply_markup] type is [Box<ReplyMarkup>], is not support, need add manully.
  #[doc(hidden)] pub fn _reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self {
    self.inner.td_origin_mut()._set_reply_markup(reply_markup);
    self
  }
  
  // [location] type is [Location], is not support, need add manully.
  #[doc(hidden)] pub fn _location(&mut self, location: Location) -> &mut Self {
    self.inner.td_origin_mut()._set_location(location);
    self
  }
  
}


///  Edits the message content of a live location. Messages can be edited for a limited period of time specified in the live location. Returns the edited message after the edit is completed on the server side. 
#[derive(Debug, Clone)]
pub struct TGEditMessageLiveLocation {
  inner: EditMessageLiveLocation
}

impl TDFB for TGEditMessageLiveLocation {}

impl AsRef<TGEditMessageLiveLocation> for TGEditMessageLiveLocation {
  fn as_ref(&self) -> &TGEditMessageLiveLocation { self }
}

impl AsRef<TGEditMessageLiveLocation> for _TGEditMessageLiveLocationBuilder {
  fn as_ref(&self) -> &TGEditMessageLiveLocation { &self.inner }
}

impl TGEditMessageLiveLocation {

  pub fn builder() -> _TGEditMessageLiveLocationBuilder {
    _TGEditMessageLiveLocationBuilder { inner: Self::new(EditMessageLiveLocation::_new()) }
  }

  pub fn new(inner: EditMessageLiveLocation) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &EditMessageLiveLocation { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut EditMessageLiveLocation { &mut self.inner }

}


#[doc(hidden)] pub struct _TGEditMessageMediaBuilder { inner: TGEditMessageMedia }

impl _TGEditMessageMediaBuilder {

  pub fn build(&self) -> TGEditMessageMedia { self.inner.clone() }

  ///  The chat the message belongs to. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Identifier of the message. 
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_message_id(message_id);
    self
  }
  

  
  // [reply_markup] type is [Box<ReplyMarkup>], is not support, need add manully.
  #[doc(hidden)] pub fn _reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self {
    self.inner.td_origin_mut()._set_reply_markup(reply_markup);
    self
  }
  
  // [input_message_content] type is [Box<InputMessageContent>], is not support, need add manully.
  #[doc(hidden)] pub fn _input_message_content(&mut self, input_message_content: Box<InputMessageContent>) -> &mut Self {
    self.inner.td_origin_mut()._set_input_message_content(input_message_content);
    self
  }
  
}


///  Edits the content of a message with an animation, an audio, a document, a photo or a video. The media in the message can't be replaced if the message was set to self-destruct. Media can't be replaced by self-destructing media. Media in an album can be edited only to contain a photo or a video. Returns the edited message after the edit is completed on the server side. 
#[derive(Debug, Clone)]
pub struct TGEditMessageMedia {
  inner: EditMessageMedia
}

impl TDFB for TGEditMessageMedia {}

impl AsRef<TGEditMessageMedia> for TGEditMessageMedia {
  fn as_ref(&self) -> &TGEditMessageMedia { self }
}

impl AsRef<TGEditMessageMedia> for _TGEditMessageMediaBuilder {
  fn as_ref(&self) -> &TGEditMessageMedia { &self.inner }
}

impl TGEditMessageMedia {

  pub fn builder() -> _TGEditMessageMediaBuilder {
    _TGEditMessageMediaBuilder { inner: Self::new(EditMessageMedia::_new()) }
  }

  pub fn new(inner: EditMessageMedia) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &EditMessageMedia { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut EditMessageMedia { &mut self.inner }

}


#[doc(hidden)] pub struct _TGEditMessageReplyMarkupBuilder { inner: TGEditMessageReplyMarkup }

impl _TGEditMessageReplyMarkupBuilder {

  pub fn build(&self) -> TGEditMessageReplyMarkup { self.inner.clone() }

  ///  The chat the message belongs to. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Identifier of the message. 
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_message_id(message_id);
    self
  }
  

  
  // [reply_markup] type is [Box<ReplyMarkup>], is not support, need add manully.
  #[doc(hidden)] pub fn _reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self {
    self.inner.td_origin_mut()._set_reply_markup(reply_markup);
    self
  }
  
}


///  Edits the message reply markup; for bots only. Returns the edited message after the edit is completed on the server side. 
#[derive(Debug, Clone)]
pub struct TGEditMessageReplyMarkup {
  inner: EditMessageReplyMarkup
}

impl TDFB for TGEditMessageReplyMarkup {}

impl AsRef<TGEditMessageReplyMarkup> for TGEditMessageReplyMarkup {
  fn as_ref(&self) -> &TGEditMessageReplyMarkup { self }
}

impl AsRef<TGEditMessageReplyMarkup> for _TGEditMessageReplyMarkupBuilder {
  fn as_ref(&self) -> &TGEditMessageReplyMarkup { &self.inner }
}

impl TGEditMessageReplyMarkup {

  pub fn builder() -> _TGEditMessageReplyMarkupBuilder {
    _TGEditMessageReplyMarkupBuilder { inner: Self::new(EditMessageReplyMarkup::_new()) }
  }

  pub fn new(inner: EditMessageReplyMarkup) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &EditMessageReplyMarkup { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut EditMessageReplyMarkup { &mut self.inner }

}


#[doc(hidden)] pub struct _TGEditMessageTextBuilder { inner: TGEditMessageText }

impl _TGEditMessageTextBuilder {

  pub fn build(&self) -> TGEditMessageText { self.inner.clone() }

  ///  The chat the message belongs to. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Identifier of the message. 
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_message_id(message_id);
    self
  }
  

  
  // [reply_markup] type is [Box<ReplyMarkup>], is not support, need add manully.
  #[doc(hidden)] pub fn _reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self {
    self.inner.td_origin_mut()._set_reply_markup(reply_markup);
    self
  }
  
  // [input_message_content] type is [Box<InputMessageContent>], is not support, need add manully.
  #[doc(hidden)] pub fn _input_message_content(&mut self, input_message_content: Box<InputMessageContent>) -> &mut Self {
    self.inner.td_origin_mut()._set_input_message_content(input_message_content);
    self
  }
  
}


///  Edits the text of a message (or a text of a game message). Returns the edited message after the edit is completed on the server side. 
#[derive(Debug, Clone)]
pub struct TGEditMessageText {
  inner: EditMessageText
}

impl TDFB for TGEditMessageText {}

impl AsRef<TGEditMessageText> for TGEditMessageText {
  fn as_ref(&self) -> &TGEditMessageText { self }
}

impl AsRef<TGEditMessageText> for _TGEditMessageTextBuilder {
  fn as_ref(&self) -> &TGEditMessageText { &self.inner }
}

impl TGEditMessageText {

  pub fn builder() -> _TGEditMessageTextBuilder {
    _TGEditMessageTextBuilder { inner: Self::new(EditMessageText::_new()) }
  }

  pub fn new(inner: EditMessageText) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &EditMessageText { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut EditMessageText { &mut self.inner }

}


#[doc(hidden)] pub struct _TGEditProxyBuilder { inner: TGEditProxy }

impl _TGEditProxyBuilder {

  pub fn build(&self) -> TGEditProxy { self.inner.clone() }

  ///  Proxy identifier. 
  pub fn proxy_id(&mut self, proxy_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_proxy_id(proxy_id);
    self
  }
  ///  Proxy server IP address. 
  pub fn server<S: AsRef<str>>(&mut self, server: S) -> &mut Self {
    self.inner.td_origin_mut()._set_server(server.as_ref().to_string());
    self
  }
  ///  Proxy server port. 
  pub fn port(&mut self, port: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_port(port);
    self
  }
  ///  True, if the proxy should be enabled. 
  pub fn enable(&mut self, enable: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_enable(enable);
    self
  }
  

  
  // [type_] type is [Box<ProxyType>], is not support, need add manully.
  #[doc(hidden)] pub fn _type_(&mut self, type_: Box<ProxyType>) -> &mut Self {
    self.inner.td_origin_mut()._set_type_(type_);
    self
  }
  
}


///  Edits an existing proxy server for network requests. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGEditProxy {
  inner: EditProxy
}

impl TDFB for TGEditProxy {}

impl AsRef<TGEditProxy> for TGEditProxy {
  fn as_ref(&self) -> &TGEditProxy { self }
}

impl AsRef<TGEditProxy> for _TGEditProxyBuilder {
  fn as_ref(&self) -> &TGEditProxy { &self.inner }
}

impl TGEditProxy {

  pub fn builder() -> _TGEditProxyBuilder {
    _TGEditProxyBuilder { inner: Self::new(EditProxy::_new()) }
  }

  pub fn new(inner: EditProxy) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &EditProxy { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut EditProxy { &mut self.inner }

}


#[doc(hidden)] pub struct _TGEnableProxyBuilder { inner: TGEnableProxy }

impl _TGEnableProxyBuilder {

  pub fn build(&self) -> TGEnableProxy { self.inner.clone() }

  ///  Proxy identifier. 
  pub fn proxy_id(&mut self, proxy_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_proxy_id(proxy_id);
    self
  }
  

  
}


///  Enables a proxy. Only one proxy can be enabled at a time. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGEnableProxy {
  inner: EnableProxy
}

impl TDFB for TGEnableProxy {}

impl AsRef<TGEnableProxy> for TGEnableProxy {
  fn as_ref(&self) -> &TGEnableProxy { self }
}

impl AsRef<TGEnableProxy> for _TGEnableProxyBuilder {
  fn as_ref(&self) -> &TGEnableProxy { &self.inner }
}

impl TGEnableProxy {

  pub fn builder() -> _TGEnableProxyBuilder {
    _TGEnableProxyBuilder { inner: Self::new(EnableProxy::_new()) }
  }

  pub fn new(inner: EnableProxy) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &EnableProxy { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut EnableProxy { &mut self.inner }

}


#[doc(hidden)] pub struct _TGFinishFileGenerationBuilder { inner: TGFinishFileGeneration }

impl _TGFinishFileGenerationBuilder {

  pub fn build(&self) -> TGFinishFileGeneration { self.inner.clone() }

  ///  The identifier of the generation process. 
  pub fn generation_id(&mut self, generation_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_generation_id(generation_id);
    self
  }
  

  
  // [error] type is [Error], is not support, need add manully.
  #[doc(hidden)] pub fn _error(&mut self, error: Error) -> &mut Self {
    self.inner.td_origin_mut()._set_error(error);
    self
  }
  
}


///  Finishes the file generation. 
#[derive(Debug, Clone)]
pub struct TGFinishFileGeneration {
  inner: FinishFileGeneration
}

impl TDFB for TGFinishFileGeneration {}

impl AsRef<TGFinishFileGeneration> for TGFinishFileGeneration {
  fn as_ref(&self) -> &TGFinishFileGeneration { self }
}

impl AsRef<TGFinishFileGeneration> for _TGFinishFileGenerationBuilder {
  fn as_ref(&self) -> &TGFinishFileGeneration { &self.inner }
}

impl TGFinishFileGeneration {

  pub fn builder() -> _TGFinishFileGenerationBuilder {
    _TGFinishFileGenerationBuilder { inner: Self::new(FinishFileGeneration::_new()) }
  }

  pub fn new(inner: FinishFileGeneration) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &FinishFileGeneration { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut FinishFileGeneration { &mut self.inner }

}


#[doc(hidden)] pub struct _TGForwardMessagesBuilder { inner: TGForwardMessages }

impl _TGForwardMessagesBuilder {

  pub fn build(&self) -> TGForwardMessages { self.inner.clone() }

  ///  Identifier of the chat to which to forward messages. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Identifier of the chat from which to forward messages. 
  pub fn from_chat_id(&mut self, from_chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_from_chat_id(from_chat_id);
    self
  }
  ///  Identifiers of the messages to forward. 
  pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self {
    self.inner.td_origin_mut()._set_message_ids(message_ids);
    self
  }
  ///  Pass true to disable notification for the message, doesn't work if messages are forwarded to a secret chat. 
  pub fn disable_notification(&mut self, disable_notification: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_disable_notification(disable_notification);
    self
  }
  ///  Pass true if the message is sent from the background. 
  pub fn from_background(&mut self, from_background: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_from_background(from_background);
    self
  }
  ///  True, if the messages should be grouped into an album after forwarding. For this to work, no more than 10 messages may be forwarded, and all of them must be photo or video messages. 
  pub fn as_album(&mut self, as_album: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_as_album(as_album);
    self
  }
  

  
}


///  Forwards previously sent messages. Returns the forwarded messages in the same order as the message identifiers passed in message_ids. If a message can't be forwarded, null will be returned instead of the message. 
#[derive(Debug, Clone)]
pub struct TGForwardMessages {
  inner: ForwardMessages
}

impl TDFB for TGForwardMessages {}

impl AsRef<TGForwardMessages> for TGForwardMessages {
  fn as_ref(&self) -> &TGForwardMessages { self }
}

impl AsRef<TGForwardMessages> for _TGForwardMessagesBuilder {
  fn as_ref(&self) -> &TGForwardMessages { &self.inner }
}

impl TGForwardMessages {

  pub fn builder() -> _TGForwardMessagesBuilder {
    _TGForwardMessagesBuilder { inner: Self::new(ForwardMessages::_new()) }
  }

  pub fn new(inner: ForwardMessages) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ForwardMessages { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ForwardMessages { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGenerateChatInviteLinkBuilder { inner: TGGenerateChatInviteLink }

impl _TGGenerateChatInviteLinkBuilder {

  pub fn build(&self) -> TGGenerateChatInviteLink { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  

  
}


///  Generates a new invite link for a chat; the previously generated link is revoked. Available for basic groups, supergroups, and channels. In basic groups this can be called only by the group's creator; in supergroups and channels this requires appropriate administrator rights. 
#[derive(Debug, Clone)]
pub struct TGGenerateChatInviteLink {
  inner: GenerateChatInviteLink
}

impl TDFB for TGGenerateChatInviteLink {}

impl AsRef<TGGenerateChatInviteLink> for TGGenerateChatInviteLink {
  fn as_ref(&self) -> &TGGenerateChatInviteLink { self }
}

impl AsRef<TGGenerateChatInviteLink> for _TGGenerateChatInviteLinkBuilder {
  fn as_ref(&self) -> &TGGenerateChatInviteLink { &self.inner }
}

impl TGGenerateChatInviteLink {

  pub fn builder() -> _TGGenerateChatInviteLinkBuilder {
    _TGGenerateChatInviteLinkBuilder { inner: Self::new(GenerateChatInviteLink::_new()) }
  }

  pub fn new(inner: GenerateChatInviteLink) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GenerateChatInviteLink { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GenerateChatInviteLink { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetAccountTtlBuilder { inner: TGGetAccountTtl }

impl _TGGetAccountTtlBuilder {

  pub fn build(&self) -> TGGetAccountTtl { self.inner.clone() }

  

  
}


///  Returns the period of inactivity after which the account of the current user will automatically be deleted. 
#[derive(Debug, Clone)]
pub struct TGGetAccountTtl {
  inner: GetAccountTtl
}

impl TDFB for TGGetAccountTtl {}

impl AsRef<TGGetAccountTtl> for TGGetAccountTtl {
  fn as_ref(&self) -> &TGGetAccountTtl { self }
}

impl AsRef<TGGetAccountTtl> for _TGGetAccountTtlBuilder {
  fn as_ref(&self) -> &TGGetAccountTtl { &self.inner }
}

impl TGGetAccountTtl {

  pub fn builder() -> _TGGetAccountTtlBuilder {
    _TGGetAccountTtlBuilder { inner: Self::new(GetAccountTtl::_new()) }
  }

  pub fn new(inner: GetAccountTtl) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetAccountTtl { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetAccountTtl { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetActiveLiveLocationMessagesBuilder { inner: TGGetActiveLiveLocationMessages }

impl _TGGetActiveLiveLocationMessagesBuilder {

  pub fn build(&self) -> TGGetActiveLiveLocationMessages { self.inner.clone() }

  

  
}


///  Returns all active live locations that should be updated by the client. The list is persistent across application restarts only if the message database is used. 
#[derive(Debug, Clone)]
pub struct TGGetActiveLiveLocationMessages {
  inner: GetActiveLiveLocationMessages
}

impl TDFB for TGGetActiveLiveLocationMessages {}

impl AsRef<TGGetActiveLiveLocationMessages> for TGGetActiveLiveLocationMessages {
  fn as_ref(&self) -> &TGGetActiveLiveLocationMessages { self }
}

impl AsRef<TGGetActiveLiveLocationMessages> for _TGGetActiveLiveLocationMessagesBuilder {
  fn as_ref(&self) -> &TGGetActiveLiveLocationMessages { &self.inner }
}

impl TGGetActiveLiveLocationMessages {

  pub fn builder() -> _TGGetActiveLiveLocationMessagesBuilder {
    _TGGetActiveLiveLocationMessagesBuilder { inner: Self::new(GetActiveLiveLocationMessages::_new()) }
  }

  pub fn new(inner: GetActiveLiveLocationMessages) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetActiveLiveLocationMessages { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetActiveLiveLocationMessages { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetActiveSessionsBuilder { inner: TGGetActiveSessions }

impl _TGGetActiveSessionsBuilder {

  pub fn build(&self) -> TGGetActiveSessions { self.inner.clone() }

  

  
}


///  Returns all active sessions of the current user. 
#[derive(Debug, Clone)]
pub struct TGGetActiveSessions {
  inner: GetActiveSessions
}

impl TDFB for TGGetActiveSessions {}

impl AsRef<TGGetActiveSessions> for TGGetActiveSessions {
  fn as_ref(&self) -> &TGGetActiveSessions { self }
}

impl AsRef<TGGetActiveSessions> for _TGGetActiveSessionsBuilder {
  fn as_ref(&self) -> &TGGetActiveSessions { &self.inner }
}

impl TGGetActiveSessions {

  pub fn builder() -> _TGGetActiveSessionsBuilder {
    _TGGetActiveSessionsBuilder { inner: Self::new(GetActiveSessions::_new()) }
  }

  pub fn new(inner: GetActiveSessions) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetActiveSessions { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetActiveSessions { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetAllPassportElementsBuilder { inner: TGGetAllPassportElements }

impl _TGGetAllPassportElementsBuilder {

  pub fn build(&self) -> TGGetAllPassportElements { self.inner.clone() }

  ///  Password of the current user. 
  pub fn password<S: AsRef<str>>(&mut self, password: S) -> &mut Self {
    self.inner.td_origin_mut()._set_password(password.as_ref().to_string());
    self
  }
  

  
}


///  Returns all available Telegram Passport elements. 
#[derive(Debug, Clone)]
pub struct TGGetAllPassportElements {
  inner: GetAllPassportElements
}

impl TDFB for TGGetAllPassportElements {}

impl AsRef<TGGetAllPassportElements> for TGGetAllPassportElements {
  fn as_ref(&self) -> &TGGetAllPassportElements { self }
}

impl AsRef<TGGetAllPassportElements> for _TGGetAllPassportElementsBuilder {
  fn as_ref(&self) -> &TGGetAllPassportElements { &self.inner }
}

impl TGGetAllPassportElements {

  pub fn builder() -> _TGGetAllPassportElementsBuilder {
    _TGGetAllPassportElementsBuilder { inner: Self::new(GetAllPassportElements::_new()) }
  }

  pub fn new(inner: GetAllPassportElements) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetAllPassportElements { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetAllPassportElements { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetApplicationConfigBuilder { inner: TGGetApplicationConfig }

impl _TGGetApplicationConfigBuilder {

  pub fn build(&self) -> TGGetApplicationConfig { self.inner.clone() }

  

  
}


///  Returns application config, provided by the server. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGGetApplicationConfig {
  inner: GetApplicationConfig
}

impl TDFB for TGGetApplicationConfig {}

impl AsRef<TGGetApplicationConfig> for TGGetApplicationConfig {
  fn as_ref(&self) -> &TGGetApplicationConfig { self }
}

impl AsRef<TGGetApplicationConfig> for _TGGetApplicationConfigBuilder {
  fn as_ref(&self) -> &TGGetApplicationConfig { &self.inner }
}

impl TGGetApplicationConfig {

  pub fn builder() -> _TGGetApplicationConfigBuilder {
    _TGGetApplicationConfigBuilder { inner: Self::new(GetApplicationConfig::_new()) }
  }

  pub fn new(inner: GetApplicationConfig) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetApplicationConfig { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetApplicationConfig { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetArchivedStickerSetsBuilder { inner: TGGetArchivedStickerSets }

impl _TGGetArchivedStickerSetsBuilder {

  pub fn build(&self) -> TGGetArchivedStickerSets { self.inner.clone() }

  ///  Pass true to return mask stickers sets; pass false to return ordinary sticker sets. 
  pub fn is_masks(&mut self, is_masks: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_is_masks(is_masks);
    self
  }
  ///  Identifier of the sticker set from which to return the result. 
  pub fn offset_sticker_set_id(&mut self, offset_sticker_set_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_offset_sticker_set_id(offset_sticker_set_id);
    self
  }
  ///  Maximum number of sticker sets to return. 
  pub fn limit(&mut self, limit: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_limit(limit);
    self
  }
  

  
}


///  Returns a list of archived sticker sets. 
#[derive(Debug, Clone)]
pub struct TGGetArchivedStickerSets {
  inner: GetArchivedStickerSets
}

impl TDFB for TGGetArchivedStickerSets {}

impl AsRef<TGGetArchivedStickerSets> for TGGetArchivedStickerSets {
  fn as_ref(&self) -> &TGGetArchivedStickerSets { self }
}

impl AsRef<TGGetArchivedStickerSets> for _TGGetArchivedStickerSetsBuilder {
  fn as_ref(&self) -> &TGGetArchivedStickerSets { &self.inner }
}

impl TGGetArchivedStickerSets {

  pub fn builder() -> _TGGetArchivedStickerSetsBuilder {
    _TGGetArchivedStickerSetsBuilder { inner: Self::new(GetArchivedStickerSets::_new()) }
  }

  pub fn new(inner: GetArchivedStickerSets) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetArchivedStickerSets { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetArchivedStickerSets { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetAttachedStickerSetsBuilder { inner: TGGetAttachedStickerSets }

impl _TGGetAttachedStickerSetsBuilder {

  pub fn build(&self) -> TGGetAttachedStickerSets { self.inner.clone() }

  ///  File identifier. 
  pub fn file_id(&mut self, file_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_file_id(file_id);
    self
  }
  

  
}


///  Returns a list of sticker sets attached to a file. Currently only photos and videos can have attached sticker sets. 
#[derive(Debug, Clone)]
pub struct TGGetAttachedStickerSets {
  inner: GetAttachedStickerSets
}

impl TDFB for TGGetAttachedStickerSets {}

impl AsRef<TGGetAttachedStickerSets> for TGGetAttachedStickerSets {
  fn as_ref(&self) -> &TGGetAttachedStickerSets { self }
}

impl AsRef<TGGetAttachedStickerSets> for _TGGetAttachedStickerSetsBuilder {
  fn as_ref(&self) -> &TGGetAttachedStickerSets { &self.inner }
}

impl TGGetAttachedStickerSets {

  pub fn builder() -> _TGGetAttachedStickerSetsBuilder {
    _TGGetAttachedStickerSetsBuilder { inner: Self::new(GetAttachedStickerSets::_new()) }
  }

  pub fn new(inner: GetAttachedStickerSets) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetAttachedStickerSets { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetAttachedStickerSets { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetAuthorizationStateBuilder { inner: TGGetAuthorizationState }

impl _TGGetAuthorizationStateBuilder {

  pub fn build(&self) -> TGGetAuthorizationState { self.inner.clone() }

  

  
}


///  Returns the current authorization state; this is an offline request. For informational purposes only. Use  
#[derive(Debug, Clone)]
pub struct TGGetAuthorizationState {
  inner: GetAuthorizationState
}

impl TDFB for TGGetAuthorizationState {}

impl AsRef<TGGetAuthorizationState> for TGGetAuthorizationState {
  fn as_ref(&self) -> &TGGetAuthorizationState { self }
}

impl AsRef<TGGetAuthorizationState> for _TGGetAuthorizationStateBuilder {
  fn as_ref(&self) -> &TGGetAuthorizationState { &self.inner }
}

impl TGGetAuthorizationState {

  pub fn builder() -> _TGGetAuthorizationStateBuilder {
    _TGGetAuthorizationStateBuilder { inner: Self::new(GetAuthorizationState::_new()) }
  }

  pub fn new(inner: GetAuthorizationState) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetAuthorizationState { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetAuthorizationState { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetBasicGroupBuilder { inner: TGGetBasicGroup }

impl _TGGetBasicGroupBuilder {

  pub fn build(&self) -> TGGetBasicGroup { self.inner.clone() }

  ///  Basic group identifier. 
  pub fn basic_group_id(&mut self, basic_group_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_basic_group_id(basic_group_id);
    self
  }
  

  
}


///  Returns information about a basic group by its identifier. This is an offline request if the current user is not a bot. 
#[derive(Debug, Clone)]
pub struct TGGetBasicGroup {
  inner: GetBasicGroup
}

impl TDFB for TGGetBasicGroup {}

impl AsRef<TGGetBasicGroup> for TGGetBasicGroup {
  fn as_ref(&self) -> &TGGetBasicGroup { self }
}

impl AsRef<TGGetBasicGroup> for _TGGetBasicGroupBuilder {
  fn as_ref(&self) -> &TGGetBasicGroup { &self.inner }
}

impl TGGetBasicGroup {

  pub fn builder() -> _TGGetBasicGroupBuilder {
    _TGGetBasicGroupBuilder { inner: Self::new(GetBasicGroup::_new()) }
  }

  pub fn new(inner: GetBasicGroup) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetBasicGroup { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetBasicGroup { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetBasicGroupFullInfoBuilder { inner: TGGetBasicGroupFullInfo }

impl _TGGetBasicGroupFullInfoBuilder {

  pub fn build(&self) -> TGGetBasicGroupFullInfo { self.inner.clone() }

  ///  Basic group identifier. 
  pub fn basic_group_id(&mut self, basic_group_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_basic_group_id(basic_group_id);
    self
  }
  

  
}


///  Returns full information about a basic group by its identifier. 
#[derive(Debug, Clone)]
pub struct TGGetBasicGroupFullInfo {
  inner: GetBasicGroupFullInfo
}

impl TDFB for TGGetBasicGroupFullInfo {}

impl AsRef<TGGetBasicGroupFullInfo> for TGGetBasicGroupFullInfo {
  fn as_ref(&self) -> &TGGetBasicGroupFullInfo { self }
}

impl AsRef<TGGetBasicGroupFullInfo> for _TGGetBasicGroupFullInfoBuilder {
  fn as_ref(&self) -> &TGGetBasicGroupFullInfo { &self.inner }
}

impl TGGetBasicGroupFullInfo {

  pub fn builder() -> _TGGetBasicGroupFullInfoBuilder {
    _TGGetBasicGroupFullInfoBuilder { inner: Self::new(GetBasicGroupFullInfo::_new()) }
  }

  pub fn new(inner: GetBasicGroupFullInfo) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetBasicGroupFullInfo { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetBasicGroupFullInfo { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetBlockedUsersBuilder { inner: TGGetBlockedUsers }

impl _TGGetBlockedUsersBuilder {

  pub fn build(&self) -> TGGetBlockedUsers { self.inner.clone() }

  ///  Number of users to skip in the result; must be non-negative. 
  pub fn offset(&mut self, offset: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_offset(offset);
    self
  }
  ///  Maximum number of users to return; up to 100. 
  pub fn limit(&mut self, limit: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_limit(limit);
    self
  }
  

  
}


///  Returns users that were blocked by the current user. 
#[derive(Debug, Clone)]
pub struct TGGetBlockedUsers {
  inner: GetBlockedUsers
}

impl TDFB for TGGetBlockedUsers {}

impl AsRef<TGGetBlockedUsers> for TGGetBlockedUsers {
  fn as_ref(&self) -> &TGGetBlockedUsers { self }
}

impl AsRef<TGGetBlockedUsers> for _TGGetBlockedUsersBuilder {
  fn as_ref(&self) -> &TGGetBlockedUsers { &self.inner }
}

impl TGGetBlockedUsers {

  pub fn builder() -> _TGGetBlockedUsersBuilder {
    _TGGetBlockedUsersBuilder { inner: Self::new(GetBlockedUsers::_new()) }
  }

  pub fn new(inner: GetBlockedUsers) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetBlockedUsers { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetBlockedUsers { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetCallbackQueryAnswerBuilder { inner: TGGetCallbackQueryAnswer }

impl _TGGetCallbackQueryAnswerBuilder {

  pub fn build(&self) -> TGGetCallbackQueryAnswer { self.inner.clone() }

  ///  Identifier of the chat with the message. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Identifier of the message from which the query originated. 
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_message_id(message_id);
    self
  }
  

  
  // [payload] type is [Box<CallbackQueryPayload>], is not support, need add manully.
  #[doc(hidden)] pub fn _payload(&mut self, payload: Box<CallbackQueryPayload>) -> &mut Self {
    self.inner.td_origin_mut()._set_payload(payload);
    self
  }
  
}


///  Sends a callback query to a bot and returns an answer. Returns an error with code 502 if the bot fails to answer the query before the query timeout expires. 
#[derive(Debug, Clone)]
pub struct TGGetCallbackQueryAnswer {
  inner: GetCallbackQueryAnswer
}

impl TDFB for TGGetCallbackQueryAnswer {}

impl AsRef<TGGetCallbackQueryAnswer> for TGGetCallbackQueryAnswer {
  fn as_ref(&self) -> &TGGetCallbackQueryAnswer { self }
}

impl AsRef<TGGetCallbackQueryAnswer> for _TGGetCallbackQueryAnswerBuilder {
  fn as_ref(&self) -> &TGGetCallbackQueryAnswer { &self.inner }
}

impl TGGetCallbackQueryAnswer {

  pub fn builder() -> _TGGetCallbackQueryAnswerBuilder {
    _TGGetCallbackQueryAnswerBuilder { inner: Self::new(GetCallbackQueryAnswer::_new()) }
  }

  pub fn new(inner: GetCallbackQueryAnswer) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetCallbackQueryAnswer { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetCallbackQueryAnswer { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetChatBuilder { inner: TGGetChat }

impl _TGGetChatBuilder {

  pub fn build(&self) -> TGGetChat { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  

  
}


///  Returns information about a chat by its identifier, this is an offline request if the current user is not a bot. 
#[derive(Debug, Clone)]
pub struct TGGetChat {
  inner: GetChat
}

impl TDFB for TGGetChat {}

impl AsRef<TGGetChat> for TGGetChat {
  fn as_ref(&self) -> &TGGetChat { self }
}

impl AsRef<TGGetChat> for _TGGetChatBuilder {
  fn as_ref(&self) -> &TGGetChat { &self.inner }
}

impl TGGetChat {

  pub fn builder() -> _TGGetChatBuilder {
    _TGGetChatBuilder { inner: Self::new(GetChat::_new()) }
  }

  pub fn new(inner: GetChat) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetChat { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetChat { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetChatAdministratorsBuilder { inner: TGGetChatAdministrators }

impl _TGGetChatAdministratorsBuilder {

  pub fn build(&self) -> TGGetChatAdministrators { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  

  
}


///  Returns a list of users who are administrators of the chat. 
#[derive(Debug, Clone)]
pub struct TGGetChatAdministrators {
  inner: GetChatAdministrators
}

impl TDFB for TGGetChatAdministrators {}

impl AsRef<TGGetChatAdministrators> for TGGetChatAdministrators {
  fn as_ref(&self) -> &TGGetChatAdministrators { self }
}

impl AsRef<TGGetChatAdministrators> for _TGGetChatAdministratorsBuilder {
  fn as_ref(&self) -> &TGGetChatAdministrators { &self.inner }
}

impl TGGetChatAdministrators {

  pub fn builder() -> _TGGetChatAdministratorsBuilder {
    _TGGetChatAdministratorsBuilder { inner: Self::new(GetChatAdministrators::_new()) }
  }

  pub fn new(inner: GetChatAdministrators) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetChatAdministrators { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetChatAdministrators { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetChatEventLogBuilder { inner: TGGetChatEventLog }

impl _TGGetChatEventLogBuilder {

  pub fn build(&self) -> TGGetChatEventLog { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Search query by which to filter events. 
  pub fn query<S: AsRef<str>>(&mut self, query: S) -> &mut Self {
    self.inner.td_origin_mut()._set_query(query.as_ref().to_string());
    self
  }
  ///  Identifier of an event from which to return results. Use 0 to get results from the latest events. 
  pub fn from_event_id(&mut self, from_event_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_from_event_id(from_event_id);
    self
  }
  ///  Maximum number of events to return; up to 100. 
  pub fn limit(&mut self, limit: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_limit(limit);
    self
  }
  ///  User identifiers by which to filter events. By default, events relating to all users will be returned. 
  pub fn user_ids(&mut self, user_ids: Vec<i32>) -> &mut Self {
    self.inner.td_origin_mut()._set_user_ids(user_ids);
    self
  }
  

  
  // [filters] type is [ChatEventLogFilters], is not support, need add manully.
  #[doc(hidden)] pub fn _filters(&mut self, filters: ChatEventLogFilters) -> &mut Self {
    self.inner.td_origin_mut()._set_filters(filters);
    self
  }
  
}


///  Returns a list of service actions taken by chat members and administrators in the last 48 hours. Available only in supergroups and channels. Requires administrator rights. Returns results in reverse chronological order (i. e., in order of decreasing event_id). 
#[derive(Debug, Clone)]
pub struct TGGetChatEventLog {
  inner: GetChatEventLog
}

impl TDFB for TGGetChatEventLog {}

impl AsRef<TGGetChatEventLog> for TGGetChatEventLog {
  fn as_ref(&self) -> &TGGetChatEventLog { self }
}

impl AsRef<TGGetChatEventLog> for _TGGetChatEventLogBuilder {
  fn as_ref(&self) -> &TGGetChatEventLog { &self.inner }
}

impl TGGetChatEventLog {

  pub fn builder() -> _TGGetChatEventLogBuilder {
    _TGGetChatEventLogBuilder { inner: Self::new(GetChatEventLog::_new()) }
  }

  pub fn new(inner: GetChatEventLog) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetChatEventLog { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetChatEventLog { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetChatHistoryBuilder { inner: TGGetChatHistory }

impl _TGGetChatHistoryBuilder {

  pub fn build(&self) -> TGGetChatHistory { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Identifier of the message starting from which history must be fetched; use 0 to get results from the last message. 
  pub fn from_message_id(&mut self, from_message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_from_message_id(from_message_id);
    self
  }
  ///  Specify 0 to get results from exactly the from_message_id or a negative offset up to 99 to get additionally some newer messages. 
  pub fn offset(&mut self, offset: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_offset(offset);
    self
  }
  ///  The maximum number of messages to be returned; must be positive and can't be greater than 100. If the offset is negative, the limit must be greater or equal to -offset. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached. 
  pub fn limit(&mut self, limit: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_limit(limit);
    self
  }
  ///  If true, returns only messages that are available locally without sending network requests. 
  pub fn only_local(&mut self, only_local: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_only_local(only_local);
    self
  }
  

  
}


///  Returns messages in a chat. The messages are returned in a reverse chronological order (i.e., in order of decreasing message_id). For optimal performance the number of returned messages is chosen by the library. This is an offline request if only_local is true. 
#[derive(Debug, Clone)]
pub struct TGGetChatHistory {
  inner: GetChatHistory
}

impl TDFB for TGGetChatHistory {}

impl AsRef<TGGetChatHistory> for TGGetChatHistory {
  fn as_ref(&self) -> &TGGetChatHistory { self }
}

impl AsRef<TGGetChatHistory> for _TGGetChatHistoryBuilder {
  fn as_ref(&self) -> &TGGetChatHistory { &self.inner }
}

impl TGGetChatHistory {

  pub fn builder() -> _TGGetChatHistoryBuilder {
    _TGGetChatHistoryBuilder { inner: Self::new(GetChatHistory::_new()) }
  }

  pub fn new(inner: GetChatHistory) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetChatHistory { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetChatHistory { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetChatMemberBuilder { inner: TGGetChatMember }

impl _TGGetChatMemberBuilder {

  pub fn build(&self) -> TGGetChatMember { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  User identifier. 
  pub fn user_id(&mut self, user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_user_id(user_id);
    self
  }
  

  
}


///  Returns information about a single member of a chat. 
#[derive(Debug, Clone)]
pub struct TGGetChatMember {
  inner: GetChatMember
}

impl TDFB for TGGetChatMember {}

impl AsRef<TGGetChatMember> for TGGetChatMember {
  fn as_ref(&self) -> &TGGetChatMember { self }
}

impl AsRef<TGGetChatMember> for _TGGetChatMemberBuilder {
  fn as_ref(&self) -> &TGGetChatMember { &self.inner }
}

impl TGGetChatMember {

  pub fn builder() -> _TGGetChatMemberBuilder {
    _TGGetChatMemberBuilder { inner: Self::new(GetChatMember::_new()) }
  }

  pub fn new(inner: GetChatMember) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetChatMember { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetChatMember { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetChatMessageByDateBuilder { inner: TGGetChatMessageByDate }

impl _TGGetChatMessageByDateBuilder {

  pub fn build(&self) -> TGGetChatMessageByDate { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Point in time (Unix timestamp) relative to which to search for messages. 
  pub fn date(&mut self, date: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_date(date);
    self
  }
  

  
}


///  Returns the last message sent in a chat no later than the specified date. 
#[derive(Debug, Clone)]
pub struct TGGetChatMessageByDate {
  inner: GetChatMessageByDate
}

impl TDFB for TGGetChatMessageByDate {}

impl AsRef<TGGetChatMessageByDate> for TGGetChatMessageByDate {
  fn as_ref(&self) -> &TGGetChatMessageByDate { self }
}

impl AsRef<TGGetChatMessageByDate> for _TGGetChatMessageByDateBuilder {
  fn as_ref(&self) -> &TGGetChatMessageByDate { &self.inner }
}

impl TGGetChatMessageByDate {

  pub fn builder() -> _TGGetChatMessageByDateBuilder {
    _TGGetChatMessageByDateBuilder { inner: Self::new(GetChatMessageByDate::_new()) }
  }

  pub fn new(inner: GetChatMessageByDate) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetChatMessageByDate { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetChatMessageByDate { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetChatMessageCountBuilder { inner: TGGetChatMessageCount }

impl _TGGetChatMessageCountBuilder {

  pub fn build(&self) -> TGGetChatMessageCount { self.inner.clone() }

  ///  Identifier of the chat in which to count messages. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  If true, returns count that is available locally without sending network requests, returning -1 if the number of messages is unknown. 
  pub fn return_local(&mut self, return_local: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_return_local(return_local);
    self
  }
  

  
  // [filter] type is [Box<SearchMessagesFilter>], is not support, need add manully.
  #[doc(hidden)] pub fn _filter(&mut self, filter: Box<SearchMessagesFilter>) -> &mut Self {
    self.inner.td_origin_mut()._set_filter(filter);
    self
  }
  
}


///  Returns approximate number of messages of the specified type in the chat. 
#[derive(Debug, Clone)]
pub struct TGGetChatMessageCount {
  inner: GetChatMessageCount
}

impl TDFB for TGGetChatMessageCount {}

impl AsRef<TGGetChatMessageCount> for TGGetChatMessageCount {
  fn as_ref(&self) -> &TGGetChatMessageCount { self }
}

impl AsRef<TGGetChatMessageCount> for _TGGetChatMessageCountBuilder {
  fn as_ref(&self) -> &TGGetChatMessageCount { &self.inner }
}

impl TGGetChatMessageCount {

  pub fn builder() -> _TGGetChatMessageCountBuilder {
    _TGGetChatMessageCountBuilder { inner: Self::new(GetChatMessageCount::_new()) }
  }

  pub fn new(inner: GetChatMessageCount) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetChatMessageCount { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetChatMessageCount { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetChatNotificationSettingsExceptionsBuilder { inner: TGGetChatNotificationSettingsExceptions }

impl _TGGetChatNotificationSettingsExceptionsBuilder {

  pub fn build(&self) -> TGGetChatNotificationSettingsExceptions { self.inner.clone() }

  ///  If true, also chats with non-default sound will be returned. 
  pub fn compare_sound(&mut self, compare_sound: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_compare_sound(compare_sound);
    self
  }
  

  
  // [scope] type is [Box<NotificationSettingsScope>], is not support, need add manully.
  #[doc(hidden)] pub fn _scope(&mut self, scope: Box<NotificationSettingsScope>) -> &mut Self {
    self.inner.td_origin_mut()._set_scope(scope);
    self
  }
  
}


///  Returns list of chats with non-default notification settings. 
#[derive(Debug, Clone)]
pub struct TGGetChatNotificationSettingsExceptions {
  inner: GetChatNotificationSettingsExceptions
}

impl TDFB for TGGetChatNotificationSettingsExceptions {}

impl AsRef<TGGetChatNotificationSettingsExceptions> for TGGetChatNotificationSettingsExceptions {
  fn as_ref(&self) -> &TGGetChatNotificationSettingsExceptions { self }
}

impl AsRef<TGGetChatNotificationSettingsExceptions> for _TGGetChatNotificationSettingsExceptionsBuilder {
  fn as_ref(&self) -> &TGGetChatNotificationSettingsExceptions { &self.inner }
}

impl TGGetChatNotificationSettingsExceptions {

  pub fn builder() -> _TGGetChatNotificationSettingsExceptionsBuilder {
    _TGGetChatNotificationSettingsExceptionsBuilder { inner: Self::new(GetChatNotificationSettingsExceptions::_new()) }
  }

  pub fn new(inner: GetChatNotificationSettingsExceptions) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetChatNotificationSettingsExceptions { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetChatNotificationSettingsExceptions { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetChatPinnedMessageBuilder { inner: TGGetChatPinnedMessage }

impl _TGGetChatPinnedMessageBuilder {

  pub fn build(&self) -> TGGetChatPinnedMessage { self.inner.clone() }

  ///  Identifier of the chat the message belongs to. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  

  
}


///  Returns information about a pinned chat message. 
#[derive(Debug, Clone)]
pub struct TGGetChatPinnedMessage {
  inner: GetChatPinnedMessage
}

impl TDFB for TGGetChatPinnedMessage {}

impl AsRef<TGGetChatPinnedMessage> for TGGetChatPinnedMessage {
  fn as_ref(&self) -> &TGGetChatPinnedMessage { self }
}

impl AsRef<TGGetChatPinnedMessage> for _TGGetChatPinnedMessageBuilder {
  fn as_ref(&self) -> &TGGetChatPinnedMessage { &self.inner }
}

impl TGGetChatPinnedMessage {

  pub fn builder() -> _TGGetChatPinnedMessageBuilder {
    _TGGetChatPinnedMessageBuilder { inner: Self::new(GetChatPinnedMessage::_new()) }
  }

  pub fn new(inner: GetChatPinnedMessage) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetChatPinnedMessage { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetChatPinnedMessage { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetChatReportSpamStateBuilder { inner: TGGetChatReportSpamState }

impl _TGGetChatReportSpamStateBuilder {

  pub fn build(&self) -> TGGetChatReportSpamState { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  

  
}


///  Returns information on whether the current chat can be reported as spam. 
#[derive(Debug, Clone)]
pub struct TGGetChatReportSpamState {
  inner: GetChatReportSpamState
}

impl TDFB for TGGetChatReportSpamState {}

impl AsRef<TGGetChatReportSpamState> for TGGetChatReportSpamState {
  fn as_ref(&self) -> &TGGetChatReportSpamState { self }
}

impl AsRef<TGGetChatReportSpamState> for _TGGetChatReportSpamStateBuilder {
  fn as_ref(&self) -> &TGGetChatReportSpamState { &self.inner }
}

impl TGGetChatReportSpamState {

  pub fn builder() -> _TGGetChatReportSpamStateBuilder {
    _TGGetChatReportSpamStateBuilder { inner: Self::new(GetChatReportSpamState::_new()) }
  }

  pub fn new(inner: GetChatReportSpamState) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetChatReportSpamState { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetChatReportSpamState { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetChatsBuilder { inner: TGGetChats }

impl _TGGetChatsBuilder {

  pub fn build(&self) -> TGGetChats { self.inner.clone() }

  ///  Chat order to return chats from. 
  pub fn offset_order(&mut self, offset_order: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_offset_order(offset_order);
    self
  }
  ///  Chat identifier to return chats from. 
  pub fn offset_chat_id(&mut self, offset_chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_offset_chat_id(offset_chat_id);
    self
  }
  ///  The maximum number of chats to be returned. It is possible that fewer chats than the limit are returned even if the end of the list is not reached. 
  pub fn limit(&mut self, limit: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_limit(limit);
    self
  }
  

  
}


///  Returns an ordered list of chats. Chats are sorted by the pair (order, chat_id) in decreasing order. (For example, to get a list of chats from the beginning, the offset_order should be equal to a biggest signed 64-bit number 9223372036854775807 == 2^63 - 1). For optimal performance the number of returned chats is chosen by the library. 
#[derive(Debug, Clone)]
pub struct TGGetChats {
  inner: GetChats
}

impl TDFB for TGGetChats {}

impl AsRef<TGGetChats> for TGGetChats {
  fn as_ref(&self) -> &TGGetChats { self }
}

impl AsRef<TGGetChats> for _TGGetChatsBuilder {
  fn as_ref(&self) -> &TGGetChats { &self.inner }
}

impl TGGetChats {

  pub fn builder() -> _TGGetChatsBuilder {
    _TGGetChatsBuilder { inner: Self::new(GetChats::_new()) }
  }

  pub fn new(inner: GetChats) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetChats { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetChats { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetChatStatisticsUrlBuilder { inner: TGGetChatStatisticsUrl }

impl _TGGetChatStatisticsUrlBuilder {

  pub fn build(&self) -> TGGetChatStatisticsUrl { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Parameters from "tg://statsrefresh?params=******" link. 
  pub fn parameters<S: AsRef<str>>(&mut self, parameters: S) -> &mut Self {
    self.inner.td_origin_mut()._set_parameters(parameters.as_ref().to_string());
    self
  }
  ///  Pass true if a URL with the dark theme must be returned. 
  pub fn is_dark(&mut self, is_dark: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_is_dark(is_dark);
    self
  }
  

  
}


///  Returns URL with the chat statistics. Currently this method can be used only for channels. 
#[derive(Debug, Clone)]
pub struct TGGetChatStatisticsUrl {
  inner: GetChatStatisticsUrl
}

impl TDFB for TGGetChatStatisticsUrl {}

impl AsRef<TGGetChatStatisticsUrl> for TGGetChatStatisticsUrl {
  fn as_ref(&self) -> &TGGetChatStatisticsUrl { self }
}

impl AsRef<TGGetChatStatisticsUrl> for _TGGetChatStatisticsUrlBuilder {
  fn as_ref(&self) -> &TGGetChatStatisticsUrl { &self.inner }
}

impl TGGetChatStatisticsUrl {

  pub fn builder() -> _TGGetChatStatisticsUrlBuilder {
    _TGGetChatStatisticsUrlBuilder { inner: Self::new(GetChatStatisticsUrl::_new()) }
  }

  pub fn new(inner: GetChatStatisticsUrl) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetChatStatisticsUrl { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetChatStatisticsUrl { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetConnectedWebsitesBuilder { inner: TGGetConnectedWebsites }

impl _TGGetConnectedWebsitesBuilder {

  pub fn build(&self) -> TGGetConnectedWebsites { self.inner.clone() }

  

  
}


///  Returns all website where the current user used Telegram to log in. 
#[derive(Debug, Clone)]
pub struct TGGetConnectedWebsites {
  inner: GetConnectedWebsites
}

impl TDFB for TGGetConnectedWebsites {}

impl AsRef<TGGetConnectedWebsites> for TGGetConnectedWebsites {
  fn as_ref(&self) -> &TGGetConnectedWebsites { self }
}

impl AsRef<TGGetConnectedWebsites> for _TGGetConnectedWebsitesBuilder {
  fn as_ref(&self) -> &TGGetConnectedWebsites { &self.inner }
}

impl TGGetConnectedWebsites {

  pub fn builder() -> _TGGetConnectedWebsitesBuilder {
    _TGGetConnectedWebsitesBuilder { inner: Self::new(GetConnectedWebsites::_new()) }
  }

  pub fn new(inner: GetConnectedWebsites) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetConnectedWebsites { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetConnectedWebsites { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetContactsBuilder { inner: TGGetContacts }

impl _TGGetContactsBuilder {

  pub fn build(&self) -> TGGetContacts { self.inner.clone() }

  

  
}


///  Returns all user contacts. 
#[derive(Debug, Clone)]
pub struct TGGetContacts {
  inner: GetContacts
}

impl TDFB for TGGetContacts {}

impl AsRef<TGGetContacts> for TGGetContacts {
  fn as_ref(&self) -> &TGGetContacts { self }
}

impl AsRef<TGGetContacts> for _TGGetContactsBuilder {
  fn as_ref(&self) -> &TGGetContacts { &self.inner }
}

impl TGGetContacts {

  pub fn builder() -> _TGGetContactsBuilder {
    _TGGetContactsBuilder { inner: Self::new(GetContacts::_new()) }
  }

  pub fn new(inner: GetContacts) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetContacts { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetContacts { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetCountryCodeBuilder { inner: TGGetCountryCode }

impl _TGGetCountryCodeBuilder {

  pub fn build(&self) -> TGGetCountryCode { self.inner.clone() }

  

  
}


///  Uses current user IP to found his country. Returns two-letter ISO 3166-1 alpha-2 country code. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGGetCountryCode {
  inner: GetCountryCode
}

impl TDFB for TGGetCountryCode {}

impl AsRef<TGGetCountryCode> for TGGetCountryCode {
  fn as_ref(&self) -> &TGGetCountryCode { self }
}

impl AsRef<TGGetCountryCode> for _TGGetCountryCodeBuilder {
  fn as_ref(&self) -> &TGGetCountryCode { &self.inner }
}

impl TGGetCountryCode {

  pub fn builder() -> _TGGetCountryCodeBuilder {
    _TGGetCountryCodeBuilder { inner: Self::new(GetCountryCode::_new()) }
  }

  pub fn new(inner: GetCountryCode) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetCountryCode { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetCountryCode { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetCreatedPublicChatsBuilder { inner: TGGetCreatedPublicChats }

impl _TGGetCreatedPublicChatsBuilder {

  pub fn build(&self) -> TGGetCreatedPublicChats { self.inner.clone() }

  

  
}


///  Returns a list of public chats created by the user. 
#[derive(Debug, Clone)]
pub struct TGGetCreatedPublicChats {
  inner: GetCreatedPublicChats
}

impl TDFB for TGGetCreatedPublicChats {}

impl AsRef<TGGetCreatedPublicChats> for TGGetCreatedPublicChats {
  fn as_ref(&self) -> &TGGetCreatedPublicChats { self }
}

impl AsRef<TGGetCreatedPublicChats> for _TGGetCreatedPublicChatsBuilder {
  fn as_ref(&self) -> &TGGetCreatedPublicChats { &self.inner }
}

impl TGGetCreatedPublicChats {

  pub fn builder() -> _TGGetCreatedPublicChatsBuilder {
    _TGGetCreatedPublicChatsBuilder { inner: Self::new(GetCreatedPublicChats::_new()) }
  }

  pub fn new(inner: GetCreatedPublicChats) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetCreatedPublicChats { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetCreatedPublicChats { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetCurrentStateBuilder { inner: TGGetCurrentState }

impl _TGGetCurrentStateBuilder {

  pub fn build(&self) -> TGGetCurrentState { self.inner.clone() }

  

  
}


///  Returns all updates needed to restore current TDLib state, i.e. all actual UpdateAuthorizationState/UpdateUser/UpdateNewChat and others. This is especially usefull if TDLib is run in a separate process. This is an offline method. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGGetCurrentState {
  inner: GetCurrentState
}

impl TDFB for TGGetCurrentState {}

impl AsRef<TGGetCurrentState> for TGGetCurrentState {
  fn as_ref(&self) -> &TGGetCurrentState { self }
}

impl AsRef<TGGetCurrentState> for _TGGetCurrentStateBuilder {
  fn as_ref(&self) -> &TGGetCurrentState { &self.inner }
}

impl TGGetCurrentState {

  pub fn builder() -> _TGGetCurrentStateBuilder {
    _TGGetCurrentStateBuilder { inner: Self::new(GetCurrentState::_new()) }
  }

  pub fn new(inner: GetCurrentState) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetCurrentState { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetCurrentState { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetDatabaseStatisticsBuilder { inner: TGGetDatabaseStatistics }

impl _TGGetDatabaseStatisticsBuilder {

  pub fn build(&self) -> TGGetDatabaseStatistics { self.inner.clone() }

  

  
}


///  Returns database statistics. 
#[derive(Debug, Clone)]
pub struct TGGetDatabaseStatistics {
  inner: GetDatabaseStatistics
}

impl TDFB for TGGetDatabaseStatistics {}

impl AsRef<TGGetDatabaseStatistics> for TGGetDatabaseStatistics {
  fn as_ref(&self) -> &TGGetDatabaseStatistics { self }
}

impl AsRef<TGGetDatabaseStatistics> for _TGGetDatabaseStatisticsBuilder {
  fn as_ref(&self) -> &TGGetDatabaseStatistics { &self.inner }
}

impl TGGetDatabaseStatistics {

  pub fn builder() -> _TGGetDatabaseStatisticsBuilder {
    _TGGetDatabaseStatisticsBuilder { inner: Self::new(GetDatabaseStatistics::_new()) }
  }

  pub fn new(inner: GetDatabaseStatistics) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetDatabaseStatistics { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetDatabaseStatistics { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetDeepLinkInfoBuilder { inner: TGGetDeepLinkInfo }

impl _TGGetDeepLinkInfoBuilder {

  pub fn build(&self) -> TGGetDeepLinkInfo { self.inner.clone() }

  ///  The link. 
  pub fn link<S: AsRef<str>>(&mut self, link: S) -> &mut Self {
    self.inner.td_origin_mut()._set_link(link.as_ref().to_string());
    self
  }
  

  
}


///  Returns information about a tg:// deep link. Use "tg://need_update_for_some_feature" or "tg:some_unsupported_feature" for testing. Returns a 404 error for unknown links. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGGetDeepLinkInfo {
  inner: GetDeepLinkInfo
}

impl TDFB for TGGetDeepLinkInfo {}

impl AsRef<TGGetDeepLinkInfo> for TGGetDeepLinkInfo {
  fn as_ref(&self) -> &TGGetDeepLinkInfo { self }
}

impl AsRef<TGGetDeepLinkInfo> for _TGGetDeepLinkInfoBuilder {
  fn as_ref(&self) -> &TGGetDeepLinkInfo { &self.inner }
}

impl TGGetDeepLinkInfo {

  pub fn builder() -> _TGGetDeepLinkInfoBuilder {
    _TGGetDeepLinkInfoBuilder { inner: Self::new(GetDeepLinkInfo::_new()) }
  }

  pub fn new(inner: GetDeepLinkInfo) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetDeepLinkInfo { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetDeepLinkInfo { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetFavoriteStickersBuilder { inner: TGGetFavoriteStickers }

impl _TGGetFavoriteStickersBuilder {

  pub fn build(&self) -> TGGetFavoriteStickers { self.inner.clone() }

  

  
}


///  Returns favorite stickers. 
#[derive(Debug, Clone)]
pub struct TGGetFavoriteStickers {
  inner: GetFavoriteStickers
}

impl TDFB for TGGetFavoriteStickers {}

impl AsRef<TGGetFavoriteStickers> for TGGetFavoriteStickers {
  fn as_ref(&self) -> &TGGetFavoriteStickers { self }
}

impl AsRef<TGGetFavoriteStickers> for _TGGetFavoriteStickersBuilder {
  fn as_ref(&self) -> &TGGetFavoriteStickers { &self.inner }
}

impl TGGetFavoriteStickers {

  pub fn builder() -> _TGGetFavoriteStickersBuilder {
    _TGGetFavoriteStickersBuilder { inner: Self::new(GetFavoriteStickers::_new()) }
  }

  pub fn new(inner: GetFavoriteStickers) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetFavoriteStickers { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetFavoriteStickers { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetFileBuilder { inner: TGGetFile }

impl _TGGetFileBuilder {

  pub fn build(&self) -> TGGetFile { self.inner.clone() }

  ///  Identifier of the file to get. 
  pub fn file_id(&mut self, file_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_file_id(file_id);
    self
  }
  

  
}


///  Returns information about a file; this is an offline request. 
#[derive(Debug, Clone)]
pub struct TGGetFile {
  inner: GetFile
}

impl TDFB for TGGetFile {}

impl AsRef<TGGetFile> for TGGetFile {
  fn as_ref(&self) -> &TGGetFile { self }
}

impl AsRef<TGGetFile> for _TGGetFileBuilder {
  fn as_ref(&self) -> &TGGetFile { &self.inner }
}

impl TGGetFile {

  pub fn builder() -> _TGGetFileBuilder {
    _TGGetFileBuilder { inner: Self::new(GetFile::_new()) }
  }

  pub fn new(inner: GetFile) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetFile { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetFile { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetFileDownloadedPrefixSizeBuilder { inner: TGGetFileDownloadedPrefixSize }

impl _TGGetFileDownloadedPrefixSizeBuilder {

  pub fn build(&self) -> TGGetFileDownloadedPrefixSize { self.inner.clone() }

  ///  Identifier of the file. 
  pub fn file_id(&mut self, file_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_file_id(file_id);
    self
  }
  ///  Offset from which downloaded prefix size should be calculated. 
  pub fn offset(&mut self, offset: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_offset(offset);
    self
  }
  

  
}


///  Returns file downloaded prefix size from a given offset. 
#[derive(Debug, Clone)]
pub struct TGGetFileDownloadedPrefixSize {
  inner: GetFileDownloadedPrefixSize
}

impl TDFB for TGGetFileDownloadedPrefixSize {}

impl AsRef<TGGetFileDownloadedPrefixSize> for TGGetFileDownloadedPrefixSize {
  fn as_ref(&self) -> &TGGetFileDownloadedPrefixSize { self }
}

impl AsRef<TGGetFileDownloadedPrefixSize> for _TGGetFileDownloadedPrefixSizeBuilder {
  fn as_ref(&self) -> &TGGetFileDownloadedPrefixSize { &self.inner }
}

impl TGGetFileDownloadedPrefixSize {

  pub fn builder() -> _TGGetFileDownloadedPrefixSizeBuilder {
    _TGGetFileDownloadedPrefixSizeBuilder { inner: Self::new(GetFileDownloadedPrefixSize::_new()) }
  }

  pub fn new(inner: GetFileDownloadedPrefixSize) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetFileDownloadedPrefixSize { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetFileDownloadedPrefixSize { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetFileExtensionBuilder { inner: TGGetFileExtension }

impl _TGGetFileExtensionBuilder {

  pub fn build(&self) -> TGGetFileExtension { self.inner.clone() }

  ///  The MIME type of the file. 
  pub fn mime_type<S: AsRef<str>>(&mut self, mime_type: S) -> &mut Self {
    self.inner.td_origin_mut()._set_mime_type(mime_type.as_ref().to_string());
    self
  }
  

  
}


///  Returns the extension of a file, guessed by its MIME type. Returns an empty string on failure. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGGetFileExtension {
  inner: GetFileExtension
}

impl TDFB for TGGetFileExtension {}

impl AsRef<TGGetFileExtension> for TGGetFileExtension {
  fn as_ref(&self) -> &TGGetFileExtension { self }
}

impl AsRef<TGGetFileExtension> for _TGGetFileExtensionBuilder {
  fn as_ref(&self) -> &TGGetFileExtension { &self.inner }
}

impl TGGetFileExtension {

  pub fn builder() -> _TGGetFileExtensionBuilder {
    _TGGetFileExtensionBuilder { inner: Self::new(GetFileExtension::_new()) }
  }

  pub fn new(inner: GetFileExtension) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetFileExtension { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetFileExtension { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetFileMimeTypeBuilder { inner: TGGetFileMimeType }

impl _TGGetFileMimeTypeBuilder {

  pub fn build(&self) -> TGGetFileMimeType { self.inner.clone() }

  ///  The name of the file or path to the file. 
  pub fn file_name<S: AsRef<str>>(&mut self, file_name: S) -> &mut Self {
    self.inner.td_origin_mut()._set_file_name(file_name.as_ref().to_string());
    self
  }
  

  
}


///  Returns the MIME type of a file, guessed by its extension. Returns an empty string on failure. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGGetFileMimeType {
  inner: GetFileMimeType
}

impl TDFB for TGGetFileMimeType {}

impl AsRef<TGGetFileMimeType> for TGGetFileMimeType {
  fn as_ref(&self) -> &TGGetFileMimeType { self }
}

impl AsRef<TGGetFileMimeType> for _TGGetFileMimeTypeBuilder {
  fn as_ref(&self) -> &TGGetFileMimeType { &self.inner }
}

impl TGGetFileMimeType {

  pub fn builder() -> _TGGetFileMimeTypeBuilder {
    _TGGetFileMimeTypeBuilder { inner: Self::new(GetFileMimeType::_new()) }
  }

  pub fn new(inner: GetFileMimeType) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetFileMimeType { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetFileMimeType { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetGameHighScoresBuilder { inner: TGGetGameHighScores }

impl _TGGetGameHighScoresBuilder {

  pub fn build(&self) -> TGGetGameHighScores { self.inner.clone() }

  ///  The chat that contains the message with the game. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Identifier of the message. 
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_message_id(message_id);
    self
  }
  ///  User identifier. 
  pub fn user_id(&mut self, user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_user_id(user_id);
    self
  }
  

  
}


///  Returns the high scores for a game and some part of the high score table in the range of the specified user; for bots only. 
#[derive(Debug, Clone)]
pub struct TGGetGameHighScores {
  inner: GetGameHighScores
}

impl TDFB for TGGetGameHighScores {}

impl AsRef<TGGetGameHighScores> for TGGetGameHighScores {
  fn as_ref(&self) -> &TGGetGameHighScores { self }
}

impl AsRef<TGGetGameHighScores> for _TGGetGameHighScoresBuilder {
  fn as_ref(&self) -> &TGGetGameHighScores { &self.inner }
}

impl TGGetGameHighScores {

  pub fn builder() -> _TGGetGameHighScoresBuilder {
    _TGGetGameHighScoresBuilder { inner: Self::new(GetGameHighScores::_new()) }
  }

  pub fn new(inner: GetGameHighScores) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetGameHighScores { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetGameHighScores { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetGroupsInCommonBuilder { inner: TGGetGroupsInCommon }

impl _TGGetGroupsInCommonBuilder {

  pub fn build(&self) -> TGGetGroupsInCommon { self.inner.clone() }

  ///  User identifier. 
  pub fn user_id(&mut self, user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_user_id(user_id);
    self
  }
  ///  Chat identifier starting from which to return chats; use 0 for the first request. 
  pub fn offset_chat_id(&mut self, offset_chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_offset_chat_id(offset_chat_id);
    self
  }
  ///  Maximum number of chats to be returned; up to 100. 
  pub fn limit(&mut self, limit: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_limit(limit);
    self
  }
  

  
}


///  Returns a list of common group chats with a given user. Chats are sorted by their type and creation date. 
#[derive(Debug, Clone)]
pub struct TGGetGroupsInCommon {
  inner: GetGroupsInCommon
}

impl TDFB for TGGetGroupsInCommon {}

impl AsRef<TGGetGroupsInCommon> for TGGetGroupsInCommon {
  fn as_ref(&self) -> &TGGetGroupsInCommon { self }
}

impl AsRef<TGGetGroupsInCommon> for _TGGetGroupsInCommonBuilder {
  fn as_ref(&self) -> &TGGetGroupsInCommon { &self.inner }
}

impl TGGetGroupsInCommon {

  pub fn builder() -> _TGGetGroupsInCommonBuilder {
    _TGGetGroupsInCommonBuilder { inner: Self::new(GetGroupsInCommon::_new()) }
  }

  pub fn new(inner: GetGroupsInCommon) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetGroupsInCommon { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetGroupsInCommon { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetImportedContactCountBuilder { inner: TGGetImportedContactCount }

impl _TGGetImportedContactCountBuilder {

  pub fn build(&self) -> TGGetImportedContactCount { self.inner.clone() }

  

  
}


///  Returns the total number of imported contacts. 
#[derive(Debug, Clone)]
pub struct TGGetImportedContactCount {
  inner: GetImportedContactCount
}

impl TDFB for TGGetImportedContactCount {}

impl AsRef<TGGetImportedContactCount> for TGGetImportedContactCount {
  fn as_ref(&self) -> &TGGetImportedContactCount { self }
}

impl AsRef<TGGetImportedContactCount> for _TGGetImportedContactCountBuilder {
  fn as_ref(&self) -> &TGGetImportedContactCount { &self.inner }
}

impl TGGetImportedContactCount {

  pub fn builder() -> _TGGetImportedContactCountBuilder {
    _TGGetImportedContactCountBuilder { inner: Self::new(GetImportedContactCount::_new()) }
  }

  pub fn new(inner: GetImportedContactCount) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetImportedContactCount { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetImportedContactCount { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetInlineGameHighScoresBuilder { inner: TGGetInlineGameHighScores }

impl _TGGetInlineGameHighScoresBuilder {

  pub fn build(&self) -> TGGetInlineGameHighScores { self.inner.clone() }

  ///  Inline message identifier. 
  pub fn inline_message_id<S: AsRef<str>>(&mut self, inline_message_id: S) -> &mut Self {
    self.inner.td_origin_mut()._set_inline_message_id(inline_message_id.as_ref().to_string());
    self
  }
  ///  User identifier. 
  pub fn user_id(&mut self, user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_user_id(user_id);
    self
  }
  

  
}


///  Returns game high scores and some part of the high score table in the range of the specified user; for bots only. 
#[derive(Debug, Clone)]
pub struct TGGetInlineGameHighScores {
  inner: GetInlineGameHighScores
}

impl TDFB for TGGetInlineGameHighScores {}

impl AsRef<TGGetInlineGameHighScores> for TGGetInlineGameHighScores {
  fn as_ref(&self) -> &TGGetInlineGameHighScores { self }
}

impl AsRef<TGGetInlineGameHighScores> for _TGGetInlineGameHighScoresBuilder {
  fn as_ref(&self) -> &TGGetInlineGameHighScores { &self.inner }
}

impl TGGetInlineGameHighScores {

  pub fn builder() -> _TGGetInlineGameHighScoresBuilder {
    _TGGetInlineGameHighScoresBuilder { inner: Self::new(GetInlineGameHighScores::_new()) }
  }

  pub fn new(inner: GetInlineGameHighScores) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetInlineGameHighScores { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetInlineGameHighScores { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetInlineQueryResultsBuilder { inner: TGGetInlineQueryResults }

impl _TGGetInlineQueryResultsBuilder {

  pub fn build(&self) -> TGGetInlineQueryResults { self.inner.clone() }

  ///  The identifier of the target bot. 
  pub fn bot_user_id(&mut self, bot_user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_bot_user_id(bot_user_id);
    self
  }
  ///  Identifier of the chat, where the query was sent. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Text of the query. 
  pub fn query<S: AsRef<str>>(&mut self, query: S) -> &mut Self {
    self.inner.td_origin_mut()._set_query(query.as_ref().to_string());
    self
  }
  ///  Offset of the first entry to return. 
  pub fn offset<S: AsRef<str>>(&mut self, offset: S) -> &mut Self {
    self.inner.td_origin_mut()._set_offset(offset.as_ref().to_string());
    self
  }
  

  
  // [user_location] type is [Location], is not support, need add manully.
  #[doc(hidden)] pub fn _user_location(&mut self, user_location: Location) -> &mut Self {
    self.inner.td_origin_mut()._set_user_location(user_location);
    self
  }
  
}


///  Sends an inline query to a bot and returns its results. Returns an error with code 502 if the bot fails to answer the query before the query timeout expires. 
#[derive(Debug, Clone)]
pub struct TGGetInlineQueryResults {
  inner: GetInlineQueryResults
}

impl TDFB for TGGetInlineQueryResults {}

impl AsRef<TGGetInlineQueryResults> for TGGetInlineQueryResults {
  fn as_ref(&self) -> &TGGetInlineQueryResults { self }
}

impl AsRef<TGGetInlineQueryResults> for _TGGetInlineQueryResultsBuilder {
  fn as_ref(&self) -> &TGGetInlineQueryResults { &self.inner }
}

impl TGGetInlineQueryResults {

  pub fn builder() -> _TGGetInlineQueryResultsBuilder {
    _TGGetInlineQueryResultsBuilder { inner: Self::new(GetInlineQueryResults::_new()) }
  }

  pub fn new(inner: GetInlineQueryResults) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetInlineQueryResults { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetInlineQueryResults { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetInstalledStickerSetsBuilder { inner: TGGetInstalledStickerSets }

impl _TGGetInstalledStickerSetsBuilder {

  pub fn build(&self) -> TGGetInstalledStickerSets { self.inner.clone() }

  ///  Pass true to return mask sticker sets; pass false to return ordinary sticker sets. 
  pub fn is_masks(&mut self, is_masks: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_is_masks(is_masks);
    self
  }
  

  
}


///  Returns a list of installed sticker sets. 
#[derive(Debug, Clone)]
pub struct TGGetInstalledStickerSets {
  inner: GetInstalledStickerSets
}

impl TDFB for TGGetInstalledStickerSets {}

impl AsRef<TGGetInstalledStickerSets> for TGGetInstalledStickerSets {
  fn as_ref(&self) -> &TGGetInstalledStickerSets { self }
}

impl AsRef<TGGetInstalledStickerSets> for _TGGetInstalledStickerSetsBuilder {
  fn as_ref(&self) -> &TGGetInstalledStickerSets { &self.inner }
}

impl TGGetInstalledStickerSets {

  pub fn builder() -> _TGGetInstalledStickerSetsBuilder {
    _TGGetInstalledStickerSetsBuilder { inner: Self::new(GetInstalledStickerSets::_new()) }
  }

  pub fn new(inner: GetInstalledStickerSets) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetInstalledStickerSets { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetInstalledStickerSets { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetInviteTextBuilder { inner: TGGetInviteText }

impl _TGGetInviteTextBuilder {

  pub fn build(&self) -> TGGetInviteText { self.inner.clone() }

  

  
}


///  Returns the default text for invitation messages to be used as a placeholder when the current user invites friends to Telegram. 
#[derive(Debug, Clone)]
pub struct TGGetInviteText {
  inner: GetInviteText
}

impl TDFB for TGGetInviteText {}

impl AsRef<TGGetInviteText> for TGGetInviteText {
  fn as_ref(&self) -> &TGGetInviteText { self }
}

impl AsRef<TGGetInviteText> for _TGGetInviteTextBuilder {
  fn as_ref(&self) -> &TGGetInviteText { &self.inner }
}

impl TGGetInviteText {

  pub fn builder() -> _TGGetInviteTextBuilder {
    _TGGetInviteTextBuilder { inner: Self::new(GetInviteText::_new()) }
  }

  pub fn new(inner: GetInviteText) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetInviteText { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetInviteText { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetLanguagePackInfoBuilder { inner: TGGetLanguagePackInfo }

impl _TGGetLanguagePackInfoBuilder {

  pub fn build(&self) -> TGGetLanguagePackInfo { self.inner.clone() }

  ///  Language pack identifier. 
  pub fn language_pack_id<S: AsRef<str>>(&mut self, language_pack_id: S) -> &mut Self {
    self.inner.td_origin_mut()._set_language_pack_id(language_pack_id.as_ref().to_string());
    self
  }
  

  
}


///  Returns information about a language pack. Returned language pack identifier may be different from a provided one. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGGetLanguagePackInfo {
  inner: GetLanguagePackInfo
}

impl TDFB for TGGetLanguagePackInfo {}

impl AsRef<TGGetLanguagePackInfo> for TGGetLanguagePackInfo {
  fn as_ref(&self) -> &TGGetLanguagePackInfo { self }
}

impl AsRef<TGGetLanguagePackInfo> for _TGGetLanguagePackInfoBuilder {
  fn as_ref(&self) -> &TGGetLanguagePackInfo { &self.inner }
}

impl TGGetLanguagePackInfo {

  pub fn builder() -> _TGGetLanguagePackInfoBuilder {
    _TGGetLanguagePackInfoBuilder { inner: Self::new(GetLanguagePackInfo::_new()) }
  }

  pub fn new(inner: GetLanguagePackInfo) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetLanguagePackInfo { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetLanguagePackInfo { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetLanguagePackStringBuilder { inner: TGGetLanguagePackString }

impl _TGGetLanguagePackStringBuilder {

  pub fn build(&self) -> TGGetLanguagePackString { self.inner.clone() }

  ///  Path to the language pack database in which strings are stored. 
  pub fn language_pack_database_path<S: AsRef<str>>(&mut self, language_pack_database_path: S) -> &mut Self {
    self.inner.td_origin_mut()._set_language_pack_database_path(language_pack_database_path.as_ref().to_string());
    self
  }
  ///  Localization target to which the language pack belongs. 
  pub fn localization_target<S: AsRef<str>>(&mut self, localization_target: S) -> &mut Self {
    self.inner.td_origin_mut()._set_localization_target(localization_target.as_ref().to_string());
    self
  }
  ///  Language pack identifier. 
  pub fn language_pack_id<S: AsRef<str>>(&mut self, language_pack_id: S) -> &mut Self {
    self.inner.td_origin_mut()._set_language_pack_id(language_pack_id.as_ref().to_string());
    self
  }
  ///  Language pack key of the string to be returned. 
  pub fn key<S: AsRef<str>>(&mut self, key: S) -> &mut Self {
    self.inner.td_origin_mut()._set_key(key.as_ref().to_string());
    self
  }
  

  
}


///  Returns a string stored in the local database from the specified localization target and language pack by its key. Returns a 404 error if the string is not found. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGGetLanguagePackString {
  inner: GetLanguagePackString
}

impl TDFB for TGGetLanguagePackString {}

impl AsRef<TGGetLanguagePackString> for TGGetLanguagePackString {
  fn as_ref(&self) -> &TGGetLanguagePackString { self }
}

impl AsRef<TGGetLanguagePackString> for _TGGetLanguagePackStringBuilder {
  fn as_ref(&self) -> &TGGetLanguagePackString { &self.inner }
}

impl TGGetLanguagePackString {

  pub fn builder() -> _TGGetLanguagePackStringBuilder {
    _TGGetLanguagePackStringBuilder { inner: Self::new(GetLanguagePackString::_new()) }
  }

  pub fn new(inner: GetLanguagePackString) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetLanguagePackString { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetLanguagePackString { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetLanguagePackStringsBuilder { inner: TGGetLanguagePackStrings }

impl _TGGetLanguagePackStringsBuilder {

  pub fn build(&self) -> TGGetLanguagePackStrings { self.inner.clone() }

  ///  Language pack identifier of the strings to be returned. 
  pub fn language_pack_id<S: AsRef<str>>(&mut self, language_pack_id: S) -> &mut Self {
    self.inner.td_origin_mut()._set_language_pack_id(language_pack_id.as_ref().to_string());
    self
  }
  

  
  // [keys] type is [Vec<String>], is not support, need add manully.
  #[doc(hidden)] pub fn _keys(&mut self, keys: Vec<String>) -> &mut Self {
    self.inner.td_origin_mut()._set_keys(keys);
    self
  }
  
}


///  Returns strings from a language pack in the current localization target by their keys. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGGetLanguagePackStrings {
  inner: GetLanguagePackStrings
}

impl TDFB for TGGetLanguagePackStrings {}

impl AsRef<TGGetLanguagePackStrings> for TGGetLanguagePackStrings {
  fn as_ref(&self) -> &TGGetLanguagePackStrings { self }
}

impl AsRef<TGGetLanguagePackStrings> for _TGGetLanguagePackStringsBuilder {
  fn as_ref(&self) -> &TGGetLanguagePackStrings { &self.inner }
}

impl TGGetLanguagePackStrings {

  pub fn builder() -> _TGGetLanguagePackStringsBuilder {
    _TGGetLanguagePackStringsBuilder { inner: Self::new(GetLanguagePackStrings::_new()) }
  }

  pub fn new(inner: GetLanguagePackStrings) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetLanguagePackStrings { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetLanguagePackStrings { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetLocalizationTargetInfoBuilder { inner: TGGetLocalizationTargetInfo }

impl _TGGetLocalizationTargetInfoBuilder {

  pub fn build(&self) -> TGGetLocalizationTargetInfo { self.inner.clone() }

  ///  If true, returns only locally available information without sending network requests. 
  pub fn only_local(&mut self, only_local: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_only_local(only_local);
    self
  }
  

  
}


///  Returns information about the current localization target. This is an offline request if only_local is true. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGGetLocalizationTargetInfo {
  inner: GetLocalizationTargetInfo
}

impl TDFB for TGGetLocalizationTargetInfo {}

impl AsRef<TGGetLocalizationTargetInfo> for TGGetLocalizationTargetInfo {
  fn as_ref(&self) -> &TGGetLocalizationTargetInfo { self }
}

impl AsRef<TGGetLocalizationTargetInfo> for _TGGetLocalizationTargetInfoBuilder {
  fn as_ref(&self) -> &TGGetLocalizationTargetInfo { &self.inner }
}

impl TGGetLocalizationTargetInfo {

  pub fn builder() -> _TGGetLocalizationTargetInfoBuilder {
    _TGGetLocalizationTargetInfoBuilder { inner: Self::new(GetLocalizationTargetInfo::_new()) }
  }

  pub fn new(inner: GetLocalizationTargetInfo) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetLocalizationTargetInfo { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetLocalizationTargetInfo { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetLogStreamBuilder { inner: TGGetLogStream }

impl _TGGetLogStreamBuilder {

  pub fn build(&self) -> TGGetLogStream { self.inner.clone() }

  

  
}


///  Returns information about currently used log stream for internal logging of TDLib. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGGetLogStream {
  inner: GetLogStream
}

impl TDFB for TGGetLogStream {}

impl AsRef<TGGetLogStream> for TGGetLogStream {
  fn as_ref(&self) -> &TGGetLogStream { self }
}

impl AsRef<TGGetLogStream> for _TGGetLogStreamBuilder {
  fn as_ref(&self) -> &TGGetLogStream { &self.inner }
}

impl TGGetLogStream {

  pub fn builder() -> _TGGetLogStreamBuilder {
    _TGGetLogStreamBuilder { inner: Self::new(GetLogStream::_new()) }
  }

  pub fn new(inner: GetLogStream) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetLogStream { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetLogStream { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetLogTagsBuilder { inner: TGGetLogTags }

impl _TGGetLogTagsBuilder {

  pub fn build(&self) -> TGGetLogTags { self.inner.clone() }

  

  
}


///  Returns list of available TDLib internal log tags, for example, ["actor", "binlog", "connections", "notifications", "proxy"]. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGGetLogTags {
  inner: GetLogTags
}

impl TDFB for TGGetLogTags {}

impl AsRef<TGGetLogTags> for TGGetLogTags {
  fn as_ref(&self) -> &TGGetLogTags { self }
}

impl AsRef<TGGetLogTags> for _TGGetLogTagsBuilder {
  fn as_ref(&self) -> &TGGetLogTags { &self.inner }
}

impl TGGetLogTags {

  pub fn builder() -> _TGGetLogTagsBuilder {
    _TGGetLogTagsBuilder { inner: Self::new(GetLogTags::_new()) }
  }

  pub fn new(inner: GetLogTags) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetLogTags { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetLogTags { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetLogTagVerbosityLevelBuilder { inner: TGGetLogTagVerbosityLevel }

impl _TGGetLogTagVerbosityLevelBuilder {

  pub fn build(&self) -> TGGetLogTagVerbosityLevel { self.inner.clone() }

  ///  Logging tag to change verbosity level. 
  pub fn tag<S: AsRef<str>>(&mut self, tag: S) -> &mut Self {
    self.inner.td_origin_mut()._set_tag(tag.as_ref().to_string());
    self
  }
  

  
}


///  Returns current verbosity level for a specified TDLib internal log tag. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGGetLogTagVerbosityLevel {
  inner: GetLogTagVerbosityLevel
}

impl TDFB for TGGetLogTagVerbosityLevel {}

impl AsRef<TGGetLogTagVerbosityLevel> for TGGetLogTagVerbosityLevel {
  fn as_ref(&self) -> &TGGetLogTagVerbosityLevel { self }
}

impl AsRef<TGGetLogTagVerbosityLevel> for _TGGetLogTagVerbosityLevelBuilder {
  fn as_ref(&self) -> &TGGetLogTagVerbosityLevel { &self.inner }
}

impl TGGetLogTagVerbosityLevel {

  pub fn builder() -> _TGGetLogTagVerbosityLevelBuilder {
    _TGGetLogTagVerbosityLevelBuilder { inner: Self::new(GetLogTagVerbosityLevel::_new()) }
  }

  pub fn new(inner: GetLogTagVerbosityLevel) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetLogTagVerbosityLevel { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetLogTagVerbosityLevel { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetLogVerbosityLevelBuilder { inner: TGGetLogVerbosityLevel }

impl _TGGetLogVerbosityLevelBuilder {

  pub fn build(&self) -> TGGetLogVerbosityLevel { self.inner.clone() }

  

  
}


///  Returns current verbosity level of the internal logging of TDLib. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGGetLogVerbosityLevel {
  inner: GetLogVerbosityLevel
}

impl TDFB for TGGetLogVerbosityLevel {}

impl AsRef<TGGetLogVerbosityLevel> for TGGetLogVerbosityLevel {
  fn as_ref(&self) -> &TGGetLogVerbosityLevel { self }
}

impl AsRef<TGGetLogVerbosityLevel> for _TGGetLogVerbosityLevelBuilder {
  fn as_ref(&self) -> &TGGetLogVerbosityLevel { &self.inner }
}

impl TGGetLogVerbosityLevel {

  pub fn builder() -> _TGGetLogVerbosityLevelBuilder {
    _TGGetLogVerbosityLevelBuilder { inner: Self::new(GetLogVerbosityLevel::_new()) }
  }

  pub fn new(inner: GetLogVerbosityLevel) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetLogVerbosityLevel { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetLogVerbosityLevel { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetMapThumbnailFileBuilder { inner: TGGetMapThumbnailFile }

impl _TGGetMapThumbnailFileBuilder {

  pub fn build(&self) -> TGGetMapThumbnailFile { self.inner.clone() }

  ///  Map zoom level; 13-20. 
  pub fn zoom(&mut self, zoom: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_zoom(zoom);
    self
  }
  ///  Map width in pixels before applying scale; 16-1024. 
  pub fn width(&mut self, width: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_width(width);
    self
  }
  ///  Map height in pixels before applying scale; 16-1024. 
  pub fn height(&mut self, height: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_height(height);
    self
  }
  ///  Map scale; 1-3. 
  pub fn scale(&mut self, scale: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_scale(scale);
    self
  }
  ///  Identifier of a chat, in which the thumbnail will be shown. Use 0 if unknown. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  

  
  // [location] type is [Location], is not support, need add manully.
  #[doc(hidden)] pub fn _location(&mut self, location: Location) -> &mut Self {
    self.inner.td_origin_mut()._set_location(location);
    self
  }
  
}


///  Returns information about a file with a map thumbnail in PNG format. Only map thumbnail files with size less than 1MB can be downloaded. 
#[derive(Debug, Clone)]
pub struct TGGetMapThumbnailFile {
  inner: GetMapThumbnailFile
}

impl TDFB for TGGetMapThumbnailFile {}

impl AsRef<TGGetMapThumbnailFile> for TGGetMapThumbnailFile {
  fn as_ref(&self) -> &TGGetMapThumbnailFile { self }
}

impl AsRef<TGGetMapThumbnailFile> for _TGGetMapThumbnailFileBuilder {
  fn as_ref(&self) -> &TGGetMapThumbnailFile { &self.inner }
}

impl TGGetMapThumbnailFile {

  pub fn builder() -> _TGGetMapThumbnailFileBuilder {
    _TGGetMapThumbnailFileBuilder { inner: Self::new(GetMapThumbnailFile::_new()) }
  }

  pub fn new(inner: GetMapThumbnailFile) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetMapThumbnailFile { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetMapThumbnailFile { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetMeBuilder { inner: TGGetMe }

impl _TGGetMeBuilder {

  pub fn build(&self) -> TGGetMe { self.inner.clone() }

  

  
}


///  Returns the current user. 
#[derive(Debug, Clone)]
pub struct TGGetMe {
  inner: GetMe
}

impl TDFB for TGGetMe {}

impl AsRef<TGGetMe> for TGGetMe {
  fn as_ref(&self) -> &TGGetMe { self }
}

impl AsRef<TGGetMe> for _TGGetMeBuilder {
  fn as_ref(&self) -> &TGGetMe { &self.inner }
}

impl TGGetMe {

  pub fn builder() -> _TGGetMeBuilder {
    _TGGetMeBuilder { inner: Self::new(GetMe::_new()) }
  }

  pub fn new(inner: GetMe) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetMe { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetMe { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetMessageBuilder { inner: TGGetMessage }

impl _TGGetMessageBuilder {

  pub fn build(&self) -> TGGetMessage { self.inner.clone() }

  ///  Identifier of the chat the message belongs to. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Identifier of the message to get. 
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_message_id(message_id);
    self
  }
  

  
}


///  Returns information about a message. 
#[derive(Debug, Clone)]
pub struct TGGetMessage {
  inner: GetMessage
}

impl TDFB for TGGetMessage {}

impl AsRef<TGGetMessage> for TGGetMessage {
  fn as_ref(&self) -> &TGGetMessage { self }
}

impl AsRef<TGGetMessage> for _TGGetMessageBuilder {
  fn as_ref(&self) -> &TGGetMessage { &self.inner }
}

impl TGGetMessage {

  pub fn builder() -> _TGGetMessageBuilder {
    _TGGetMessageBuilder { inner: Self::new(GetMessage::_new()) }
  }

  pub fn new(inner: GetMessage) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetMessage { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetMessage { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetMessageLinkBuilder { inner: TGGetMessageLink }

impl _TGGetMessageLinkBuilder {

  pub fn build(&self) -> TGGetMessageLink { self.inner.clone() }

  ///  Identifier of the chat to which the message belongs. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Identifier of the message. 
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_message_id(message_id);
    self
  }
  

  
}


///  Returns a private HTTPS link to a message in a chat. Available only for already sent messages in supergroups and channels. The link will work only for members of the chat. 
#[derive(Debug, Clone)]
pub struct TGGetMessageLink {
  inner: GetMessageLink
}

impl TDFB for TGGetMessageLink {}

impl AsRef<TGGetMessageLink> for TGGetMessageLink {
  fn as_ref(&self) -> &TGGetMessageLink { self }
}

impl AsRef<TGGetMessageLink> for _TGGetMessageLinkBuilder {
  fn as_ref(&self) -> &TGGetMessageLink { &self.inner }
}

impl TGGetMessageLink {

  pub fn builder() -> _TGGetMessageLinkBuilder {
    _TGGetMessageLinkBuilder { inner: Self::new(GetMessageLink::_new()) }
  }

  pub fn new(inner: GetMessageLink) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetMessageLink { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetMessageLink { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetMessageLocallyBuilder { inner: TGGetMessageLocally }

impl _TGGetMessageLocallyBuilder {

  pub fn build(&self) -> TGGetMessageLocally { self.inner.clone() }

  ///  Identifier of the chat the message belongs to. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Identifier of the message to get. 
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_message_id(message_id);
    self
  }
  

  
}


///  Returns information about a message, if it is available locally without sending network request. This is an offline request. 
#[derive(Debug, Clone)]
pub struct TGGetMessageLocally {
  inner: GetMessageLocally
}

impl TDFB for TGGetMessageLocally {}

impl AsRef<TGGetMessageLocally> for TGGetMessageLocally {
  fn as_ref(&self) -> &TGGetMessageLocally { self }
}

impl AsRef<TGGetMessageLocally> for _TGGetMessageLocallyBuilder {
  fn as_ref(&self) -> &TGGetMessageLocally { &self.inner }
}

impl TGGetMessageLocally {

  pub fn builder() -> _TGGetMessageLocallyBuilder {
    _TGGetMessageLocallyBuilder { inner: Self::new(GetMessageLocally::_new()) }
  }

  pub fn new(inner: GetMessageLocally) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetMessageLocally { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetMessageLocally { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetMessagesBuilder { inner: TGGetMessages }

impl _TGGetMessagesBuilder {

  pub fn build(&self) -> TGGetMessages { self.inner.clone() }

  ///  Identifier of the chat the messages belong to. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Identifiers of the messages to get. 
  pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self {
    self.inner.td_origin_mut()._set_message_ids(message_ids);
    self
  }
  

  
}


///  Returns information about messages. If a message is not found, returns null on the corresponding position of the result. 
#[derive(Debug, Clone)]
pub struct TGGetMessages {
  inner: GetMessages
}

impl TDFB for TGGetMessages {}

impl AsRef<TGGetMessages> for TGGetMessages {
  fn as_ref(&self) -> &TGGetMessages { self }
}

impl AsRef<TGGetMessages> for _TGGetMessagesBuilder {
  fn as_ref(&self) -> &TGGetMessages { &self.inner }
}

impl TGGetMessages {

  pub fn builder() -> _TGGetMessagesBuilder {
    _TGGetMessagesBuilder { inner: Self::new(GetMessages::_new()) }
  }

  pub fn new(inner: GetMessages) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetMessages { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetMessages { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetNetworkStatisticsBuilder { inner: TGGetNetworkStatistics }

impl _TGGetNetworkStatisticsBuilder {

  pub fn build(&self) -> TGGetNetworkStatistics { self.inner.clone() }

  ///  If true, returns only data for the current library launch. 
  pub fn only_current(&mut self, only_current: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_only_current(only_current);
    self
  }
  

  
}


///  Returns network data usage statistics. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGGetNetworkStatistics {
  inner: GetNetworkStatistics
}

impl TDFB for TGGetNetworkStatistics {}

impl AsRef<TGGetNetworkStatistics> for TGGetNetworkStatistics {
  fn as_ref(&self) -> &TGGetNetworkStatistics { self }
}

impl AsRef<TGGetNetworkStatistics> for _TGGetNetworkStatisticsBuilder {
  fn as_ref(&self) -> &TGGetNetworkStatistics { &self.inner }
}

impl TGGetNetworkStatistics {

  pub fn builder() -> _TGGetNetworkStatisticsBuilder {
    _TGGetNetworkStatisticsBuilder { inner: Self::new(GetNetworkStatistics::_new()) }
  }

  pub fn new(inner: GetNetworkStatistics) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetNetworkStatistics { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetNetworkStatistics { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetOptionBuilder { inner: TGGetOption }

impl _TGGetOptionBuilder {

  pub fn build(&self) -> TGGetOption { self.inner.clone() }

  ///  The name of the option. 
  pub fn name<S: AsRef<str>>(&mut self, name: S) -> &mut Self {
    self.inner.td_origin_mut()._set_name(name.as_ref().to_string());
    self
  }
  

  
}


///  Returns the value of an option by its name. (Check the list of available options on  
#[derive(Debug, Clone)]
pub struct TGGetOption {
  inner: GetOption
}

impl TDFB for TGGetOption {}

impl AsRef<TGGetOption> for TGGetOption {
  fn as_ref(&self) -> &TGGetOption { self }
}

impl AsRef<TGGetOption> for _TGGetOptionBuilder {
  fn as_ref(&self) -> &TGGetOption { &self.inner }
}

impl TGGetOption {

  pub fn builder() -> _TGGetOptionBuilder {
    _TGGetOptionBuilder { inner: Self::new(GetOption::_new()) }
  }

  pub fn new(inner: GetOption) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetOption { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetOption { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetPassportAuthorizationFormBuilder { inner: TGGetPassportAuthorizationForm }

impl _TGGetPassportAuthorizationFormBuilder {

  pub fn build(&self) -> TGGetPassportAuthorizationForm { self.inner.clone() }

  ///  User identifier of the service's bot. 
  pub fn bot_user_id(&mut self, bot_user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_bot_user_id(bot_user_id);
    self
  }
  ///  Telegram Passport element types requested by the service. 
  pub fn scope<S: AsRef<str>>(&mut self, scope: S) -> &mut Self {
    self.inner.td_origin_mut()._set_scope(scope.as_ref().to_string());
    self
  }
  ///  Service's public_key. 
  pub fn public_key<S: AsRef<str>>(&mut self, public_key: S) -> &mut Self {
    self.inner.td_origin_mut()._set_public_key(public_key.as_ref().to_string());
    self
  }
  ///  Authorization form nonce provided by the service. 
  pub fn nonce<S: AsRef<str>>(&mut self, nonce: S) -> &mut Self {
    self.inner.td_origin_mut()._set_nonce(nonce.as_ref().to_string());
    self
  }
  

  
}


///  Returns a Telegram Passport authorization form for sharing data with a service. 
#[derive(Debug, Clone)]
pub struct TGGetPassportAuthorizationForm {
  inner: GetPassportAuthorizationForm
}

impl TDFB for TGGetPassportAuthorizationForm {}

impl AsRef<TGGetPassportAuthorizationForm> for TGGetPassportAuthorizationForm {
  fn as_ref(&self) -> &TGGetPassportAuthorizationForm { self }
}

impl AsRef<TGGetPassportAuthorizationForm> for _TGGetPassportAuthorizationFormBuilder {
  fn as_ref(&self) -> &TGGetPassportAuthorizationForm { &self.inner }
}

impl TGGetPassportAuthorizationForm {

  pub fn builder() -> _TGGetPassportAuthorizationFormBuilder {
    _TGGetPassportAuthorizationFormBuilder { inner: Self::new(GetPassportAuthorizationForm::_new()) }
  }

  pub fn new(inner: GetPassportAuthorizationForm) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetPassportAuthorizationForm { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetPassportAuthorizationForm { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetPassportAuthorizationFormAvailableElementsBuilder { inner: TGGetPassportAuthorizationFormAvailableElements }

impl _TGGetPassportAuthorizationFormAvailableElementsBuilder {

  pub fn build(&self) -> TGGetPassportAuthorizationFormAvailableElements { self.inner.clone() }

  ///  Authorization form identifier. 
  pub fn autorization_form_id(&mut self, autorization_form_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_autorization_form_id(autorization_form_id);
    self
  }
  ///  Password of the current user. 
  pub fn password<S: AsRef<str>>(&mut self, password: S) -> &mut Self {
    self.inner.td_origin_mut()._set_password(password.as_ref().to_string());
    self
  }
  

  
}


///  Returns already available Telegram Passport elements suitable for completing a Telegram Passport authorization form. Result can be received only once for each authorization form. 
#[derive(Debug, Clone)]
pub struct TGGetPassportAuthorizationFormAvailableElements {
  inner: GetPassportAuthorizationFormAvailableElements
}

impl TDFB for TGGetPassportAuthorizationFormAvailableElements {}

impl AsRef<TGGetPassportAuthorizationFormAvailableElements> for TGGetPassportAuthorizationFormAvailableElements {
  fn as_ref(&self) -> &TGGetPassportAuthorizationFormAvailableElements { self }
}

impl AsRef<TGGetPassportAuthorizationFormAvailableElements> for _TGGetPassportAuthorizationFormAvailableElementsBuilder {
  fn as_ref(&self) -> &TGGetPassportAuthorizationFormAvailableElements { &self.inner }
}

impl TGGetPassportAuthorizationFormAvailableElements {

  pub fn builder() -> _TGGetPassportAuthorizationFormAvailableElementsBuilder {
    _TGGetPassportAuthorizationFormAvailableElementsBuilder { inner: Self::new(GetPassportAuthorizationFormAvailableElements::_new()) }
  }

  pub fn new(inner: GetPassportAuthorizationFormAvailableElements) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetPassportAuthorizationFormAvailableElements { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetPassportAuthorizationFormAvailableElements { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetPassportElementBuilder { inner: TGGetPassportElement }

impl _TGGetPassportElementBuilder {

  pub fn build(&self) -> TGGetPassportElement { self.inner.clone() }

  ///  Password of the current user. 
  pub fn password<S: AsRef<str>>(&mut self, password: S) -> &mut Self {
    self.inner.td_origin_mut()._set_password(password.as_ref().to_string());
    self
  }
  

  
  // [type_] type is [Box<PassportElementType>], is not support, need add manully.
  #[doc(hidden)] pub fn _type_(&mut self, type_: Box<PassportElementType>) -> &mut Self {
    self.inner.td_origin_mut()._set_type_(type_);
    self
  }
  
}


///  Returns one of the available Telegram Passport elements. 
#[derive(Debug, Clone)]
pub struct TGGetPassportElement {
  inner: GetPassportElement
}

impl TDFB for TGGetPassportElement {}

impl AsRef<TGGetPassportElement> for TGGetPassportElement {
  fn as_ref(&self) -> &TGGetPassportElement { self }
}

impl AsRef<TGGetPassportElement> for _TGGetPassportElementBuilder {
  fn as_ref(&self) -> &TGGetPassportElement { &self.inner }
}

impl TGGetPassportElement {

  pub fn builder() -> _TGGetPassportElementBuilder {
    _TGGetPassportElementBuilder { inner: Self::new(GetPassportElement::_new()) }
  }

  pub fn new(inner: GetPassportElement) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetPassportElement { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetPassportElement { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetPasswordStateBuilder { inner: TGGetPasswordState }

impl _TGGetPasswordStateBuilder {

  pub fn build(&self) -> TGGetPasswordState { self.inner.clone() }

  

  
}


///  Returns the current state of 2-step verification. 
#[derive(Debug, Clone)]
pub struct TGGetPasswordState {
  inner: GetPasswordState
}

impl TDFB for TGGetPasswordState {}

impl AsRef<TGGetPasswordState> for TGGetPasswordState {
  fn as_ref(&self) -> &TGGetPasswordState { self }
}

impl AsRef<TGGetPasswordState> for _TGGetPasswordStateBuilder {
  fn as_ref(&self) -> &TGGetPasswordState { &self.inner }
}

impl TGGetPasswordState {

  pub fn builder() -> _TGGetPasswordStateBuilder {
    _TGGetPasswordStateBuilder { inner: Self::new(GetPasswordState::_new()) }
  }

  pub fn new(inner: GetPasswordState) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetPasswordState { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetPasswordState { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetPaymentFormBuilder { inner: TGGetPaymentForm }

impl _TGGetPaymentFormBuilder {

  pub fn build(&self) -> TGGetPaymentForm { self.inner.clone() }

  ///  Chat identifier of the Invoice message. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Message identifier. 
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_message_id(message_id);
    self
  }
  

  
}


///  Returns an invoice payment form. This method should be called when the user presses inlineKeyboardButtonBuy. 
#[derive(Debug, Clone)]
pub struct TGGetPaymentForm {
  inner: GetPaymentForm
}

impl TDFB for TGGetPaymentForm {}

impl AsRef<TGGetPaymentForm> for TGGetPaymentForm {
  fn as_ref(&self) -> &TGGetPaymentForm { self }
}

impl AsRef<TGGetPaymentForm> for _TGGetPaymentFormBuilder {
  fn as_ref(&self) -> &TGGetPaymentForm { &self.inner }
}

impl TGGetPaymentForm {

  pub fn builder() -> _TGGetPaymentFormBuilder {
    _TGGetPaymentFormBuilder { inner: Self::new(GetPaymentForm::_new()) }
  }

  pub fn new(inner: GetPaymentForm) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetPaymentForm { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetPaymentForm { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetPaymentReceiptBuilder { inner: TGGetPaymentReceipt }

impl _TGGetPaymentReceiptBuilder {

  pub fn build(&self) -> TGGetPaymentReceipt { self.inner.clone() }

  ///  Chat identifier of the PaymentSuccessful message. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Message identifier. 
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_message_id(message_id);
    self
  }
  

  
}


///  Returns information about a successful payment. 
#[derive(Debug, Clone)]
pub struct TGGetPaymentReceipt {
  inner: GetPaymentReceipt
}

impl TDFB for TGGetPaymentReceipt {}

impl AsRef<TGGetPaymentReceipt> for TGGetPaymentReceipt {
  fn as_ref(&self) -> &TGGetPaymentReceipt { self }
}

impl AsRef<TGGetPaymentReceipt> for _TGGetPaymentReceiptBuilder {
  fn as_ref(&self) -> &TGGetPaymentReceipt { &self.inner }
}

impl TGGetPaymentReceipt {

  pub fn builder() -> _TGGetPaymentReceiptBuilder {
    _TGGetPaymentReceiptBuilder { inner: Self::new(GetPaymentReceipt::_new()) }
  }

  pub fn new(inner: GetPaymentReceipt) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetPaymentReceipt { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetPaymentReceipt { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetPreferredCountryLanguageBuilder { inner: TGGetPreferredCountryLanguage }

impl _TGGetPreferredCountryLanguageBuilder {

  pub fn build(&self) -> TGGetPreferredCountryLanguage { self.inner.clone() }

  ///  A two-letter ISO 3166-1 alpha-2 country code. 
  pub fn country_code<S: AsRef<str>>(&mut self, country_code: S) -> &mut Self {
    self.inner.td_origin_mut()._set_country_code(country_code.as_ref().to_string());
    self
  }
  

  
}


///  Returns an IETF language tag of the language preferred in the country, which should be used to fill native fields in Telegram Passport personal details. Returns a 404 error if unknown. 
#[derive(Debug, Clone)]
pub struct TGGetPreferredCountryLanguage {
  inner: GetPreferredCountryLanguage
}

impl TDFB for TGGetPreferredCountryLanguage {}

impl AsRef<TGGetPreferredCountryLanguage> for TGGetPreferredCountryLanguage {
  fn as_ref(&self) -> &TGGetPreferredCountryLanguage { self }
}

impl AsRef<TGGetPreferredCountryLanguage> for _TGGetPreferredCountryLanguageBuilder {
  fn as_ref(&self) -> &TGGetPreferredCountryLanguage { &self.inner }
}

impl TGGetPreferredCountryLanguage {

  pub fn builder() -> _TGGetPreferredCountryLanguageBuilder {
    _TGGetPreferredCountryLanguageBuilder { inner: Self::new(GetPreferredCountryLanguage::_new()) }
  }

  pub fn new(inner: GetPreferredCountryLanguage) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetPreferredCountryLanguage { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetPreferredCountryLanguage { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetProxiesBuilder { inner: TGGetProxies }

impl _TGGetProxiesBuilder {

  pub fn build(&self) -> TGGetProxies { self.inner.clone() }

  

  
}


///  Returns list of proxies that are currently set up. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGGetProxies {
  inner: GetProxies
}

impl TDFB for TGGetProxies {}

impl AsRef<TGGetProxies> for TGGetProxies {
  fn as_ref(&self) -> &TGGetProxies { self }
}

impl AsRef<TGGetProxies> for _TGGetProxiesBuilder {
  fn as_ref(&self) -> &TGGetProxies { &self.inner }
}

impl TGGetProxies {

  pub fn builder() -> _TGGetProxiesBuilder {
    _TGGetProxiesBuilder { inner: Self::new(GetProxies::_new()) }
  }

  pub fn new(inner: GetProxies) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetProxies { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetProxies { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetProxyLinkBuilder { inner: TGGetProxyLink }

impl _TGGetProxyLinkBuilder {

  pub fn build(&self) -> TGGetProxyLink { self.inner.clone() }

  ///  Proxy identifier. 
  pub fn proxy_id(&mut self, proxy_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_proxy_id(proxy_id);
    self
  }
  

  
}


///  Returns an HTTPS link, which can be used to add a proxy. Available only for SOCKS5 and MTProto proxies. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGGetProxyLink {
  inner: GetProxyLink
}

impl TDFB for TGGetProxyLink {}

impl AsRef<TGGetProxyLink> for TGGetProxyLink {
  fn as_ref(&self) -> &TGGetProxyLink { self }
}

impl AsRef<TGGetProxyLink> for _TGGetProxyLinkBuilder {
  fn as_ref(&self) -> &TGGetProxyLink { &self.inner }
}

impl TGGetProxyLink {

  pub fn builder() -> _TGGetProxyLinkBuilder {
    _TGGetProxyLinkBuilder { inner: Self::new(GetProxyLink::_new()) }
  }

  pub fn new(inner: GetProxyLink) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetProxyLink { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetProxyLink { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetPublicMessageLinkBuilder { inner: TGGetPublicMessageLink }

impl _TGGetPublicMessageLinkBuilder {

  pub fn build(&self) -> TGGetPublicMessageLink { self.inner.clone() }

  ///  Identifier of the chat to which the message belongs. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Identifier of the message. 
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_message_id(message_id);
    self
  }
  ///  Pass true if a link for a whole media album should be returned. 
  pub fn for_album(&mut self, for_album: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_for_album(for_album);
    self
  }
  

  
}


///  Returns a public HTTPS link to a message. Available only for messages in public supergroups and channels. 
#[derive(Debug, Clone)]
pub struct TGGetPublicMessageLink {
  inner: GetPublicMessageLink
}

impl TDFB for TGGetPublicMessageLink {}

impl AsRef<TGGetPublicMessageLink> for TGGetPublicMessageLink {
  fn as_ref(&self) -> &TGGetPublicMessageLink { self }
}

impl AsRef<TGGetPublicMessageLink> for _TGGetPublicMessageLinkBuilder {
  fn as_ref(&self) -> &TGGetPublicMessageLink { &self.inner }
}

impl TGGetPublicMessageLink {

  pub fn builder() -> _TGGetPublicMessageLinkBuilder {
    _TGGetPublicMessageLinkBuilder { inner: Self::new(GetPublicMessageLink::_new()) }
  }

  pub fn new(inner: GetPublicMessageLink) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetPublicMessageLink { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetPublicMessageLink { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetPushReceiverIdBuilder { inner: TGGetPushReceiverId }

impl _TGGetPushReceiverIdBuilder {

  pub fn build(&self) -> TGGetPushReceiverId { self.inner.clone() }

  ///  JSON-encoded push notification payload. 
  pub fn payload<S: AsRef<str>>(&mut self, payload: S) -> &mut Self {
    self.inner.td_origin_mut()._set_payload(payload.as_ref().to_string());
    self
  }
  

  
}


///  Returns a globally unique push notification subscription identifier for identification of an account, which has received a push notification. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGGetPushReceiverId {
  inner: GetPushReceiverId
}

impl TDFB for TGGetPushReceiverId {}

impl AsRef<TGGetPushReceiverId> for TGGetPushReceiverId {
  fn as_ref(&self) -> &TGGetPushReceiverId { self }
}

impl AsRef<TGGetPushReceiverId> for _TGGetPushReceiverIdBuilder {
  fn as_ref(&self) -> &TGGetPushReceiverId { &self.inner }
}

impl TGGetPushReceiverId {

  pub fn builder() -> _TGGetPushReceiverIdBuilder {
    _TGGetPushReceiverIdBuilder { inner: Self::new(GetPushReceiverId::_new()) }
  }

  pub fn new(inner: GetPushReceiverId) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetPushReceiverId { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetPushReceiverId { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetRecentInlineBotsBuilder { inner: TGGetRecentInlineBots }

impl _TGGetRecentInlineBotsBuilder {

  pub fn build(&self) -> TGGetRecentInlineBots { self.inner.clone() }

  

  
}


///  Returns up to 20 recently used inline bots in the order of their last usage. 
#[derive(Debug, Clone)]
pub struct TGGetRecentInlineBots {
  inner: GetRecentInlineBots
}

impl TDFB for TGGetRecentInlineBots {}

impl AsRef<TGGetRecentInlineBots> for TGGetRecentInlineBots {
  fn as_ref(&self) -> &TGGetRecentInlineBots { self }
}

impl AsRef<TGGetRecentInlineBots> for _TGGetRecentInlineBotsBuilder {
  fn as_ref(&self) -> &TGGetRecentInlineBots { &self.inner }
}

impl TGGetRecentInlineBots {

  pub fn builder() -> _TGGetRecentInlineBotsBuilder {
    _TGGetRecentInlineBotsBuilder { inner: Self::new(GetRecentInlineBots::_new()) }
  }

  pub fn new(inner: GetRecentInlineBots) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetRecentInlineBots { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetRecentInlineBots { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetRecentlyVisitedTMeUrlsBuilder { inner: TGGetRecentlyVisitedTMeUrls }

impl _TGGetRecentlyVisitedTMeUrlsBuilder {

  pub fn build(&self) -> TGGetRecentlyVisitedTMeUrls { self.inner.clone() }

  ///  Google Play referrer to identify the user. 
  pub fn referrer<S: AsRef<str>>(&mut self, referrer: S) -> &mut Self {
    self.inner.td_origin_mut()._set_referrer(referrer.as_ref().to_string());
    self
  }
  

  
}


///  Returns t.me URLs recently visited by a newly registered user. 
#[derive(Debug, Clone)]
pub struct TGGetRecentlyVisitedTMeUrls {
  inner: GetRecentlyVisitedTMeUrls
}

impl TDFB for TGGetRecentlyVisitedTMeUrls {}

impl AsRef<TGGetRecentlyVisitedTMeUrls> for TGGetRecentlyVisitedTMeUrls {
  fn as_ref(&self) -> &TGGetRecentlyVisitedTMeUrls { self }
}

impl AsRef<TGGetRecentlyVisitedTMeUrls> for _TGGetRecentlyVisitedTMeUrlsBuilder {
  fn as_ref(&self) -> &TGGetRecentlyVisitedTMeUrls { &self.inner }
}

impl TGGetRecentlyVisitedTMeUrls {

  pub fn builder() -> _TGGetRecentlyVisitedTMeUrlsBuilder {
    _TGGetRecentlyVisitedTMeUrlsBuilder { inner: Self::new(GetRecentlyVisitedTMeUrls::_new()) }
  }

  pub fn new(inner: GetRecentlyVisitedTMeUrls) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetRecentlyVisitedTMeUrls { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetRecentlyVisitedTMeUrls { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetRecentStickersBuilder { inner: TGGetRecentStickers }

impl _TGGetRecentStickersBuilder {

  pub fn build(&self) -> TGGetRecentStickers { self.inner.clone() }

  ///  Pass true to return stickers and masks that were recently attached to photos or video files; pass false to return recently sent stickers. 
  pub fn is_attached(&mut self, is_attached: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_is_attached(is_attached);
    self
  }
  

  
}


///  Returns a list of recently used stickers. 
#[derive(Debug, Clone)]
pub struct TGGetRecentStickers {
  inner: GetRecentStickers
}

impl TDFB for TGGetRecentStickers {}

impl AsRef<TGGetRecentStickers> for TGGetRecentStickers {
  fn as_ref(&self) -> &TGGetRecentStickers { self }
}

impl AsRef<TGGetRecentStickers> for _TGGetRecentStickersBuilder {
  fn as_ref(&self) -> &TGGetRecentStickers { &self.inner }
}

impl TGGetRecentStickers {

  pub fn builder() -> _TGGetRecentStickersBuilder {
    _TGGetRecentStickersBuilder { inner: Self::new(GetRecentStickers::_new()) }
  }

  pub fn new(inner: GetRecentStickers) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetRecentStickers { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetRecentStickers { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetRecoveryEmailAddressBuilder { inner: TGGetRecoveryEmailAddress }

impl _TGGetRecoveryEmailAddressBuilder {

  pub fn build(&self) -> TGGetRecoveryEmailAddress { self.inner.clone() }

  ///  The password for the current user. 
  pub fn password<S: AsRef<str>>(&mut self, password: S) -> &mut Self {
    self.inner.td_origin_mut()._set_password(password.as_ref().to_string());
    self
  }
  

  
}


///  Returns a 2-step verification recovery email address that was previously set up. This method can be used to verify a password provided by the user. 
#[derive(Debug, Clone)]
pub struct TGGetRecoveryEmailAddress {
  inner: GetRecoveryEmailAddress
}

impl TDFB for TGGetRecoveryEmailAddress {}

impl AsRef<TGGetRecoveryEmailAddress> for TGGetRecoveryEmailAddress {
  fn as_ref(&self) -> &TGGetRecoveryEmailAddress { self }
}

impl AsRef<TGGetRecoveryEmailAddress> for _TGGetRecoveryEmailAddressBuilder {
  fn as_ref(&self) -> &TGGetRecoveryEmailAddress { &self.inner }
}

impl TGGetRecoveryEmailAddress {

  pub fn builder() -> _TGGetRecoveryEmailAddressBuilder {
    _TGGetRecoveryEmailAddressBuilder { inner: Self::new(GetRecoveryEmailAddress::_new()) }
  }

  pub fn new(inner: GetRecoveryEmailAddress) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetRecoveryEmailAddress { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetRecoveryEmailAddress { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetRemoteFileBuilder { inner: TGGetRemoteFile }

impl _TGGetRemoteFileBuilder {

  pub fn build(&self) -> TGGetRemoteFile { self.inner.clone() }

  ///  Remote identifier of the file to get. 
  pub fn remote_file_id<S: AsRef<str>>(&mut self, remote_file_id: S) -> &mut Self {
    self.inner.td_origin_mut()._set_remote_file_id(remote_file_id.as_ref().to_string());
    self
  }
  

  
  // [file_type] type is [Box<FileType>], is not support, need add manully.
  #[doc(hidden)] pub fn _file_type(&mut self, file_type: Box<FileType>) -> &mut Self {
    self.inner.td_origin_mut()._set_file_type(file_type);
    self
  }
  
}


///  Returns information about a file by its remote ID; this is an offline request. Can be used to register a URL as a file for further uploading, or sending as a message. 
#[derive(Debug, Clone)]
pub struct TGGetRemoteFile {
  inner: GetRemoteFile
}

impl TDFB for TGGetRemoteFile {}

impl AsRef<TGGetRemoteFile> for TGGetRemoteFile {
  fn as_ref(&self) -> &TGGetRemoteFile { self }
}

impl AsRef<TGGetRemoteFile> for _TGGetRemoteFileBuilder {
  fn as_ref(&self) -> &TGGetRemoteFile { &self.inner }
}

impl TGGetRemoteFile {

  pub fn builder() -> _TGGetRemoteFileBuilder {
    _TGGetRemoteFileBuilder { inner: Self::new(GetRemoteFile::_new()) }
  }

  pub fn new(inner: GetRemoteFile) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetRemoteFile { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetRemoteFile { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetRepliedMessageBuilder { inner: TGGetRepliedMessage }

impl _TGGetRepliedMessageBuilder {

  pub fn build(&self) -> TGGetRepliedMessage { self.inner.clone() }

  ///  Identifier of the chat the message belongs to. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Identifier of the message reply to which get. 
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_message_id(message_id);
    self
  }
  

  
}


///  Returns information about a message that is replied by given message. 
#[derive(Debug, Clone)]
pub struct TGGetRepliedMessage {
  inner: GetRepliedMessage
}

impl TDFB for TGGetRepliedMessage {}

impl AsRef<TGGetRepliedMessage> for TGGetRepliedMessage {
  fn as_ref(&self) -> &TGGetRepliedMessage { self }
}

impl AsRef<TGGetRepliedMessage> for _TGGetRepliedMessageBuilder {
  fn as_ref(&self) -> &TGGetRepliedMessage { &self.inner }
}

impl TGGetRepliedMessage {

  pub fn builder() -> _TGGetRepliedMessageBuilder {
    _TGGetRepliedMessageBuilder { inner: Self::new(GetRepliedMessage::_new()) }
  }

  pub fn new(inner: GetRepliedMessage) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetRepliedMessage { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetRepliedMessage { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetSavedAnimationsBuilder { inner: TGGetSavedAnimations }

impl _TGGetSavedAnimationsBuilder {

  pub fn build(&self) -> TGGetSavedAnimations { self.inner.clone() }

  

  
}


///  Returns saved animations. 
#[derive(Debug, Clone)]
pub struct TGGetSavedAnimations {
  inner: GetSavedAnimations
}

impl TDFB for TGGetSavedAnimations {}

impl AsRef<TGGetSavedAnimations> for TGGetSavedAnimations {
  fn as_ref(&self) -> &TGGetSavedAnimations { self }
}

impl AsRef<TGGetSavedAnimations> for _TGGetSavedAnimationsBuilder {
  fn as_ref(&self) -> &TGGetSavedAnimations { &self.inner }
}

impl TGGetSavedAnimations {

  pub fn builder() -> _TGGetSavedAnimationsBuilder {
    _TGGetSavedAnimationsBuilder { inner: Self::new(GetSavedAnimations::_new()) }
  }

  pub fn new(inner: GetSavedAnimations) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetSavedAnimations { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetSavedAnimations { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetSavedOrderInfoBuilder { inner: TGGetSavedOrderInfo }

impl _TGGetSavedOrderInfoBuilder {

  pub fn build(&self) -> TGGetSavedOrderInfo { self.inner.clone() }

  

  
}


///  Returns saved order info, if any. 
#[derive(Debug, Clone)]
pub struct TGGetSavedOrderInfo {
  inner: GetSavedOrderInfo
}

impl TDFB for TGGetSavedOrderInfo {}

impl AsRef<TGGetSavedOrderInfo> for TGGetSavedOrderInfo {
  fn as_ref(&self) -> &TGGetSavedOrderInfo { self }
}

impl AsRef<TGGetSavedOrderInfo> for _TGGetSavedOrderInfoBuilder {
  fn as_ref(&self) -> &TGGetSavedOrderInfo { &self.inner }
}

impl TGGetSavedOrderInfo {

  pub fn builder() -> _TGGetSavedOrderInfoBuilder {
    _TGGetSavedOrderInfoBuilder { inner: Self::new(GetSavedOrderInfo::_new()) }
  }

  pub fn new(inner: GetSavedOrderInfo) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetSavedOrderInfo { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetSavedOrderInfo { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetScopeNotificationSettingsBuilder { inner: TGGetScopeNotificationSettings }

impl _TGGetScopeNotificationSettingsBuilder {

  pub fn build(&self) -> TGGetScopeNotificationSettings { self.inner.clone() }

  

  
  // [scope] type is [Box<NotificationSettingsScope>], is not support, need add manully.
  #[doc(hidden)] pub fn _scope(&mut self, scope: Box<NotificationSettingsScope>) -> &mut Self {
    self.inner.td_origin_mut()._set_scope(scope);
    self
  }
  
}


///  Returns the notification settings for chats of a given type. 
#[derive(Debug, Clone)]
pub struct TGGetScopeNotificationSettings {
  inner: GetScopeNotificationSettings
}

impl TDFB for TGGetScopeNotificationSettings {}

impl AsRef<TGGetScopeNotificationSettings> for TGGetScopeNotificationSettings {
  fn as_ref(&self) -> &TGGetScopeNotificationSettings { self }
}

impl AsRef<TGGetScopeNotificationSettings> for _TGGetScopeNotificationSettingsBuilder {
  fn as_ref(&self) -> &TGGetScopeNotificationSettings { &self.inner }
}

impl TGGetScopeNotificationSettings {

  pub fn builder() -> _TGGetScopeNotificationSettingsBuilder {
    _TGGetScopeNotificationSettingsBuilder { inner: Self::new(GetScopeNotificationSettings::_new()) }
  }

  pub fn new(inner: GetScopeNotificationSettings) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetScopeNotificationSettings { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetScopeNotificationSettings { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetSecretChatBuilder { inner: TGGetSecretChat }

impl _TGGetSecretChatBuilder {

  pub fn build(&self) -> TGGetSecretChat { self.inner.clone() }

  ///  Secret chat identifier. 
  pub fn secret_chat_id(&mut self, secret_chat_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_secret_chat_id(secret_chat_id);
    self
  }
  

  
}


///  Returns information about a secret chat by its identifier. This is an offline request. 
#[derive(Debug, Clone)]
pub struct TGGetSecretChat {
  inner: GetSecretChat
}

impl TDFB for TGGetSecretChat {}

impl AsRef<TGGetSecretChat> for TGGetSecretChat {
  fn as_ref(&self) -> &TGGetSecretChat { self }
}

impl AsRef<TGGetSecretChat> for _TGGetSecretChatBuilder {
  fn as_ref(&self) -> &TGGetSecretChat { &self.inner }
}

impl TGGetSecretChat {

  pub fn builder() -> _TGGetSecretChatBuilder {
    _TGGetSecretChatBuilder { inner: Self::new(GetSecretChat::_new()) }
  }

  pub fn new(inner: GetSecretChat) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetSecretChat { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetSecretChat { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetStickerEmojisBuilder { inner: TGGetStickerEmojis }

impl _TGGetStickerEmojisBuilder {

  pub fn build(&self) -> TGGetStickerEmojis { self.inner.clone() }

  

  
  // [sticker] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _sticker(&mut self, sticker: Box<InputFile>) -> &mut Self {
    self.inner.td_origin_mut()._set_sticker(sticker);
    self
  }
  
}


///  Returns emoji corresponding to a sticker. 
#[derive(Debug, Clone)]
pub struct TGGetStickerEmojis {
  inner: GetStickerEmojis
}

impl TDFB for TGGetStickerEmojis {}

impl AsRef<TGGetStickerEmojis> for TGGetStickerEmojis {
  fn as_ref(&self) -> &TGGetStickerEmojis { self }
}

impl AsRef<TGGetStickerEmojis> for _TGGetStickerEmojisBuilder {
  fn as_ref(&self) -> &TGGetStickerEmojis { &self.inner }
}

impl TGGetStickerEmojis {

  pub fn builder() -> _TGGetStickerEmojisBuilder {
    _TGGetStickerEmojisBuilder { inner: Self::new(GetStickerEmojis::_new()) }
  }

  pub fn new(inner: GetStickerEmojis) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetStickerEmojis { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetStickerEmojis { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetStickersBuilder { inner: TGGetStickers }

impl _TGGetStickersBuilder {

  pub fn build(&self) -> TGGetStickers { self.inner.clone() }

  ///  String representation of emoji. If empty, returns all known installed stickers. 
  pub fn emoji<S: AsRef<str>>(&mut self, emoji: S) -> &mut Self {
    self.inner.td_origin_mut()._set_emoji(emoji.as_ref().to_string());
    self
  }
  ///  Maximum number of stickers to be returned. 
  pub fn limit(&mut self, limit: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_limit(limit);
    self
  }
  

  
}


///  Returns stickers from the installed sticker sets that correspond to a given emoji. If the emoji is not empty, favorite and recently used stickers may also be returned. 
#[derive(Debug, Clone)]
pub struct TGGetStickers {
  inner: GetStickers
}

impl TDFB for TGGetStickers {}

impl AsRef<TGGetStickers> for TGGetStickers {
  fn as_ref(&self) -> &TGGetStickers { self }
}

impl AsRef<TGGetStickers> for _TGGetStickersBuilder {
  fn as_ref(&self) -> &TGGetStickers { &self.inner }
}

impl TGGetStickers {

  pub fn builder() -> _TGGetStickersBuilder {
    _TGGetStickersBuilder { inner: Self::new(GetStickers::_new()) }
  }

  pub fn new(inner: GetStickers) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetStickers { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetStickers { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetStickerSetBuilder { inner: TGGetStickerSet }

impl _TGGetStickerSetBuilder {

  pub fn build(&self) -> TGGetStickerSet { self.inner.clone() }

  ///  Identifier of the sticker set. 
  pub fn set_id(&mut self, set_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_set_id(set_id);
    self
  }
  

  
}


///  Returns information about a sticker set by its identifier. 
#[derive(Debug, Clone)]
pub struct TGGetStickerSet {
  inner: GetStickerSet
}

impl TDFB for TGGetStickerSet {}

impl AsRef<TGGetStickerSet> for TGGetStickerSet {
  fn as_ref(&self) -> &TGGetStickerSet { self }
}

impl AsRef<TGGetStickerSet> for _TGGetStickerSetBuilder {
  fn as_ref(&self) -> &TGGetStickerSet { &self.inner }
}

impl TGGetStickerSet {

  pub fn builder() -> _TGGetStickerSetBuilder {
    _TGGetStickerSetBuilder { inner: Self::new(GetStickerSet::_new()) }
  }

  pub fn new(inner: GetStickerSet) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetStickerSet { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetStickerSet { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetStorageStatisticsBuilder { inner: TGGetStorageStatistics }

impl _TGGetStorageStatisticsBuilder {

  pub fn build(&self) -> TGGetStorageStatistics { self.inner.clone() }

  ///  Maximum number of chats with the largest storage usage for which separate statistics should be returned. All other chats will be grouped in entries with chat_id == 0. If the chat info database is not used, the chat_limit is ignored and is always set to 0. 
  pub fn chat_limit(&mut self, chat_limit: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_limit(chat_limit);
    self
  }
  

  
}


///  Returns storage usage statistics. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGGetStorageStatistics {
  inner: GetStorageStatistics
}

impl TDFB for TGGetStorageStatistics {}

impl AsRef<TGGetStorageStatistics> for TGGetStorageStatistics {
  fn as_ref(&self) -> &TGGetStorageStatistics { self }
}

impl AsRef<TGGetStorageStatistics> for _TGGetStorageStatisticsBuilder {
  fn as_ref(&self) -> &TGGetStorageStatistics { &self.inner }
}

impl TGGetStorageStatistics {

  pub fn builder() -> _TGGetStorageStatisticsBuilder {
    _TGGetStorageStatisticsBuilder { inner: Self::new(GetStorageStatistics::_new()) }
  }

  pub fn new(inner: GetStorageStatistics) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetStorageStatistics { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetStorageStatistics { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetStorageStatisticsFastBuilder { inner: TGGetStorageStatisticsFast }

impl _TGGetStorageStatisticsFastBuilder {

  pub fn build(&self) -> TGGetStorageStatisticsFast { self.inner.clone() }

  

  
}


///  Quickly returns approximate storage usage statistics. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGGetStorageStatisticsFast {
  inner: GetStorageStatisticsFast
}

impl TDFB for TGGetStorageStatisticsFast {}

impl AsRef<TGGetStorageStatisticsFast> for TGGetStorageStatisticsFast {
  fn as_ref(&self) -> &TGGetStorageStatisticsFast { self }
}

impl AsRef<TGGetStorageStatisticsFast> for _TGGetStorageStatisticsFastBuilder {
  fn as_ref(&self) -> &TGGetStorageStatisticsFast { &self.inner }
}

impl TGGetStorageStatisticsFast {

  pub fn builder() -> _TGGetStorageStatisticsFastBuilder {
    _TGGetStorageStatisticsFastBuilder { inner: Self::new(GetStorageStatisticsFast::_new()) }
  }

  pub fn new(inner: GetStorageStatisticsFast) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetStorageStatisticsFast { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetStorageStatisticsFast { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetSupergroupBuilder { inner: TGGetSupergroup }

impl _TGGetSupergroupBuilder {

  pub fn build(&self) -> TGGetSupergroup { self.inner.clone() }

  ///  Supergroup or channel identifier. 
  pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_supergroup_id(supergroup_id);
    self
  }
  

  
}


///  Returns information about a supergroup or channel by its identifier. This is an offline request if the current user is not a bot. 
#[derive(Debug, Clone)]
pub struct TGGetSupergroup {
  inner: GetSupergroup
}

impl TDFB for TGGetSupergroup {}

impl AsRef<TGGetSupergroup> for TGGetSupergroup {
  fn as_ref(&self) -> &TGGetSupergroup { self }
}

impl AsRef<TGGetSupergroup> for _TGGetSupergroupBuilder {
  fn as_ref(&self) -> &TGGetSupergroup { &self.inner }
}

impl TGGetSupergroup {

  pub fn builder() -> _TGGetSupergroupBuilder {
    _TGGetSupergroupBuilder { inner: Self::new(GetSupergroup::_new()) }
  }

  pub fn new(inner: GetSupergroup) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetSupergroup { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetSupergroup { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetSupergroupFullInfoBuilder { inner: TGGetSupergroupFullInfo }

impl _TGGetSupergroupFullInfoBuilder {

  pub fn build(&self) -> TGGetSupergroupFullInfo { self.inner.clone() }

  ///  Supergroup or channel identifier. 
  pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_supergroup_id(supergroup_id);
    self
  }
  

  
}


///  Returns full information about a supergroup or channel by its identifier, cached for up to 1 minute. 
#[derive(Debug, Clone)]
pub struct TGGetSupergroupFullInfo {
  inner: GetSupergroupFullInfo
}

impl TDFB for TGGetSupergroupFullInfo {}

impl AsRef<TGGetSupergroupFullInfo> for TGGetSupergroupFullInfo {
  fn as_ref(&self) -> &TGGetSupergroupFullInfo { self }
}

impl AsRef<TGGetSupergroupFullInfo> for _TGGetSupergroupFullInfoBuilder {
  fn as_ref(&self) -> &TGGetSupergroupFullInfo { &self.inner }
}

impl TGGetSupergroupFullInfo {

  pub fn builder() -> _TGGetSupergroupFullInfoBuilder {
    _TGGetSupergroupFullInfoBuilder { inner: Self::new(GetSupergroupFullInfo::_new()) }
  }

  pub fn new(inner: GetSupergroupFullInfo) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetSupergroupFullInfo { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetSupergroupFullInfo { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetSupergroupMembersBuilder { inner: TGGetSupergroupMembers }

impl _TGGetSupergroupMembersBuilder {

  pub fn build(&self) -> TGGetSupergroupMembers { self.inner.clone() }

  ///  Identifier of the supergroup or channel. 
  pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_supergroup_id(supergroup_id);
    self
  }
  ///  Number of users to skip. 
  pub fn offset(&mut self, offset: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_offset(offset);
    self
  }
  ///  The maximum number of users be returned; up to 200. 
  pub fn limit(&mut self, limit: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_limit(limit);
    self
  }
  

  
  // [filter] type is [Box<SupergroupMembersFilter>], is not support, need add manully.
  #[doc(hidden)] pub fn _filter(&mut self, filter: Box<SupergroupMembersFilter>) -> &mut Self {
    self.inner.td_origin_mut()._set_filter(filter);
    self
  }
  
}


///  Returns information about members or banned users in a supergroup or channel. Can be used only if SupergroupFullInfo.can_get_members == true; additionally, administrator privileges may be required for some filters. 
#[derive(Debug, Clone)]
pub struct TGGetSupergroupMembers {
  inner: GetSupergroupMembers
}

impl TDFB for TGGetSupergroupMembers {}

impl AsRef<TGGetSupergroupMembers> for TGGetSupergroupMembers {
  fn as_ref(&self) -> &TGGetSupergroupMembers { self }
}

impl AsRef<TGGetSupergroupMembers> for _TGGetSupergroupMembersBuilder {
  fn as_ref(&self) -> &TGGetSupergroupMembers { &self.inner }
}

impl TGGetSupergroupMembers {

  pub fn builder() -> _TGGetSupergroupMembersBuilder {
    _TGGetSupergroupMembersBuilder { inner: Self::new(GetSupergroupMembers::_new()) }
  }

  pub fn new(inner: GetSupergroupMembers) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetSupergroupMembers { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetSupergroupMembers { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetSupportUserBuilder { inner: TGGetSupportUser }

impl _TGGetSupportUserBuilder {

  pub fn build(&self) -> TGGetSupportUser { self.inner.clone() }

  

  
}


///  Returns a user that can be contacted to get support. 
#[derive(Debug, Clone)]
pub struct TGGetSupportUser {
  inner: GetSupportUser
}

impl TDFB for TGGetSupportUser {}

impl AsRef<TGGetSupportUser> for TGGetSupportUser {
  fn as_ref(&self) -> &TGGetSupportUser { self }
}

impl AsRef<TGGetSupportUser> for _TGGetSupportUserBuilder {
  fn as_ref(&self) -> &TGGetSupportUser { &self.inner }
}

impl TGGetSupportUser {

  pub fn builder() -> _TGGetSupportUserBuilder {
    _TGGetSupportUserBuilder { inner: Self::new(GetSupportUser::_new()) }
  }

  pub fn new(inner: GetSupportUser) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetSupportUser { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetSupportUser { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetTemporaryPasswordStateBuilder { inner: TGGetTemporaryPasswordState }

impl _TGGetTemporaryPasswordStateBuilder {

  pub fn build(&self) -> TGGetTemporaryPasswordState { self.inner.clone() }

  

  
}


///  Returns information about the current temporary password. 
#[derive(Debug, Clone)]
pub struct TGGetTemporaryPasswordState {
  inner: GetTemporaryPasswordState
}

impl TDFB for TGGetTemporaryPasswordState {}

impl AsRef<TGGetTemporaryPasswordState> for TGGetTemporaryPasswordState {
  fn as_ref(&self) -> &TGGetTemporaryPasswordState { self }
}

impl AsRef<TGGetTemporaryPasswordState> for _TGGetTemporaryPasswordStateBuilder {
  fn as_ref(&self) -> &TGGetTemporaryPasswordState { &self.inner }
}

impl TGGetTemporaryPasswordState {

  pub fn builder() -> _TGGetTemporaryPasswordStateBuilder {
    _TGGetTemporaryPasswordStateBuilder { inner: Self::new(GetTemporaryPasswordState::_new()) }
  }

  pub fn new(inner: GetTemporaryPasswordState) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetTemporaryPasswordState { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetTemporaryPasswordState { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetTextEntitiesBuilder { inner: TGGetTextEntities }

impl _TGGetTextEntitiesBuilder {

  pub fn build(&self) -> TGGetTextEntities { self.inner.clone() }

  ///  The text in which to look for entites. 
  pub fn text<S: AsRef<str>>(&mut self, text: S) -> &mut Self {
    self.inner.td_origin_mut()._set_text(text.as_ref().to_string());
    self
  }
  

  
}


///  Returns all entities (mentions, hashtags, cashtags, bot commands, URLs, and email addresses) contained in the text. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGGetTextEntities {
  inner: GetTextEntities
}

impl TDFB for TGGetTextEntities {}

impl AsRef<TGGetTextEntities> for TGGetTextEntities {
  fn as_ref(&self) -> &TGGetTextEntities { self }
}

impl AsRef<TGGetTextEntities> for _TGGetTextEntitiesBuilder {
  fn as_ref(&self) -> &TGGetTextEntities { &self.inner }
}

impl TGGetTextEntities {

  pub fn builder() -> _TGGetTextEntitiesBuilder {
    _TGGetTextEntitiesBuilder { inner: Self::new(GetTextEntities::_new()) }
  }

  pub fn new(inner: GetTextEntities) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetTextEntities { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetTextEntities { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetTopChatsBuilder { inner: TGGetTopChats }

impl _TGGetTopChatsBuilder {

  pub fn build(&self) -> TGGetTopChats { self.inner.clone() }

  ///  Maximum number of chats to be returned; up to 30. 
  pub fn limit(&mut self, limit: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_limit(limit);
    self
  }
  

  
  // [category] type is [Box<TopChatCategory>], is not support, need add manully.
  #[doc(hidden)] pub fn _category(&mut self, category: Box<TopChatCategory>) -> &mut Self {
    self.inner.td_origin_mut()._set_category(category);
    self
  }
  
}


///  Returns a list of frequently used chats. Supported only if the chat info database is enabled. 
#[derive(Debug, Clone)]
pub struct TGGetTopChats {
  inner: GetTopChats
}

impl TDFB for TGGetTopChats {}

impl AsRef<TGGetTopChats> for TGGetTopChats {
  fn as_ref(&self) -> &TGGetTopChats { self }
}

impl AsRef<TGGetTopChats> for _TGGetTopChatsBuilder {
  fn as_ref(&self) -> &TGGetTopChats { &self.inner }
}

impl TGGetTopChats {

  pub fn builder() -> _TGGetTopChatsBuilder {
    _TGGetTopChatsBuilder { inner: Self::new(GetTopChats::_new()) }
  }

  pub fn new(inner: GetTopChats) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetTopChats { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetTopChats { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetTrendingStickerSetsBuilder { inner: TGGetTrendingStickerSets }

impl _TGGetTrendingStickerSetsBuilder {

  pub fn build(&self) -> TGGetTrendingStickerSets { self.inner.clone() }

  

  
}


///  Returns a list of trending sticker sets. 
#[derive(Debug, Clone)]
pub struct TGGetTrendingStickerSets {
  inner: GetTrendingStickerSets
}

impl TDFB for TGGetTrendingStickerSets {}

impl AsRef<TGGetTrendingStickerSets> for TGGetTrendingStickerSets {
  fn as_ref(&self) -> &TGGetTrendingStickerSets { self }
}

impl AsRef<TGGetTrendingStickerSets> for _TGGetTrendingStickerSetsBuilder {
  fn as_ref(&self) -> &TGGetTrendingStickerSets { &self.inner }
}

impl TGGetTrendingStickerSets {

  pub fn builder() -> _TGGetTrendingStickerSetsBuilder {
    _TGGetTrendingStickerSetsBuilder { inner: Self::new(GetTrendingStickerSets::_new()) }
  }

  pub fn new(inner: GetTrendingStickerSets) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetTrendingStickerSets { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetTrendingStickerSets { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetUserBuilder { inner: TGGetUser }

impl _TGGetUserBuilder {

  pub fn build(&self) -> TGGetUser { self.inner.clone() }

  ///  User identifier. 
  pub fn user_id(&mut self, user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_user_id(user_id);
    self
  }
  

  
}


///  Returns information about a user by their identifier. This is an offline request if the current user is not a bot. 
#[derive(Debug, Clone)]
pub struct TGGetUser {
  inner: GetUser
}

impl TDFB for TGGetUser {}

impl AsRef<TGGetUser> for TGGetUser {
  fn as_ref(&self) -> &TGGetUser { self }
}

impl AsRef<TGGetUser> for _TGGetUserBuilder {
  fn as_ref(&self) -> &TGGetUser { &self.inner }
}

impl TGGetUser {

  pub fn builder() -> _TGGetUserBuilder {
    _TGGetUserBuilder { inner: Self::new(GetUser::_new()) }
  }

  pub fn new(inner: GetUser) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetUser { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetUser { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetUserFullInfoBuilder { inner: TGGetUserFullInfo }

impl _TGGetUserFullInfoBuilder {

  pub fn build(&self) -> TGGetUserFullInfo { self.inner.clone() }

  ///  User identifier. 
  pub fn user_id(&mut self, user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_user_id(user_id);
    self
  }
  

  
}


///  Returns full information about a user by their identifier. 
#[derive(Debug, Clone)]
pub struct TGGetUserFullInfo {
  inner: GetUserFullInfo
}

impl TDFB for TGGetUserFullInfo {}

impl AsRef<TGGetUserFullInfo> for TGGetUserFullInfo {
  fn as_ref(&self) -> &TGGetUserFullInfo { self }
}

impl AsRef<TGGetUserFullInfo> for _TGGetUserFullInfoBuilder {
  fn as_ref(&self) -> &TGGetUserFullInfo { &self.inner }
}

impl TGGetUserFullInfo {

  pub fn builder() -> _TGGetUserFullInfoBuilder {
    _TGGetUserFullInfoBuilder { inner: Self::new(GetUserFullInfo::_new()) }
  }

  pub fn new(inner: GetUserFullInfo) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetUserFullInfo { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetUserFullInfo { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetUserPrivacySettingRulesBuilder { inner: TGGetUserPrivacySettingRules }

impl _TGGetUserPrivacySettingRulesBuilder {

  pub fn build(&self) -> TGGetUserPrivacySettingRules { self.inner.clone() }

  

  
  // [setting] type is [Box<UserPrivacySetting>], is not support, need add manully.
  #[doc(hidden)] pub fn _setting(&mut self, setting: Box<UserPrivacySetting>) -> &mut Self {
    self.inner.td_origin_mut()._set_setting(setting);
    self
  }
  
}


///  Returns the current privacy settings. 
#[derive(Debug, Clone)]
pub struct TGGetUserPrivacySettingRules {
  inner: GetUserPrivacySettingRules
}

impl TDFB for TGGetUserPrivacySettingRules {}

impl AsRef<TGGetUserPrivacySettingRules> for TGGetUserPrivacySettingRules {
  fn as_ref(&self) -> &TGGetUserPrivacySettingRules { self }
}

impl AsRef<TGGetUserPrivacySettingRules> for _TGGetUserPrivacySettingRulesBuilder {
  fn as_ref(&self) -> &TGGetUserPrivacySettingRules { &self.inner }
}

impl TGGetUserPrivacySettingRules {

  pub fn builder() -> _TGGetUserPrivacySettingRulesBuilder {
    _TGGetUserPrivacySettingRulesBuilder { inner: Self::new(GetUserPrivacySettingRules::_new()) }
  }

  pub fn new(inner: GetUserPrivacySettingRules) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetUserPrivacySettingRules { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetUserPrivacySettingRules { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetUserProfilePhotosBuilder { inner: TGGetUserProfilePhotos }

impl _TGGetUserProfilePhotosBuilder {

  pub fn build(&self) -> TGGetUserProfilePhotos { self.inner.clone() }

  ///  User identifier. 
  pub fn user_id(&mut self, user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_user_id(user_id);
    self
  }
  ///  The number of photos to skip; must be non-negative. 
  pub fn offset(&mut self, offset: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_offset(offset);
    self
  }
  ///  Maximum number of photos to be returned; up to 100. 
  pub fn limit(&mut self, limit: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_limit(limit);
    self
  }
  

  
}


///  Returns the profile photos of a user. The result of this query may be outdated: some photos might have been deleted already. 
#[derive(Debug, Clone)]
pub struct TGGetUserProfilePhotos {
  inner: GetUserProfilePhotos
}

impl TDFB for TGGetUserProfilePhotos {}

impl AsRef<TGGetUserProfilePhotos> for TGGetUserProfilePhotos {
  fn as_ref(&self) -> &TGGetUserProfilePhotos { self }
}

impl AsRef<TGGetUserProfilePhotos> for _TGGetUserProfilePhotosBuilder {
  fn as_ref(&self) -> &TGGetUserProfilePhotos { &self.inner }
}

impl TGGetUserProfilePhotos {

  pub fn builder() -> _TGGetUserProfilePhotosBuilder {
    _TGGetUserProfilePhotosBuilder { inner: Self::new(GetUserProfilePhotos::_new()) }
  }

  pub fn new(inner: GetUserProfilePhotos) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetUserProfilePhotos { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetUserProfilePhotos { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetWallpapersBuilder { inner: TGGetWallpapers }

impl _TGGetWallpapersBuilder {

  pub fn build(&self) -> TGGetWallpapers { self.inner.clone() }

  

  
}


///  Returns background wallpapers. 
#[derive(Debug, Clone)]
pub struct TGGetWallpapers {
  inner: GetWallpapers
}

impl TDFB for TGGetWallpapers {}

impl AsRef<TGGetWallpapers> for TGGetWallpapers {
  fn as_ref(&self) -> &TGGetWallpapers { self }
}

impl AsRef<TGGetWallpapers> for _TGGetWallpapersBuilder {
  fn as_ref(&self) -> &TGGetWallpapers { &self.inner }
}

impl TGGetWallpapers {

  pub fn builder() -> _TGGetWallpapersBuilder {
    _TGGetWallpapersBuilder { inner: Self::new(GetWallpapers::_new()) }
  }

  pub fn new(inner: GetWallpapers) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetWallpapers { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetWallpapers { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetWebPageInstantViewBuilder { inner: TGGetWebPageInstantView }

impl _TGGetWebPageInstantViewBuilder {

  pub fn build(&self) -> TGGetWebPageInstantView { self.inner.clone() }

  ///  The web page URL. 
  pub fn url<S: AsRef<str>>(&mut self, url: S) -> &mut Self {
    self.inner.td_origin_mut()._set_url(url.as_ref().to_string());
    self
  }
  ///  If true, the full instant view for the web page will be returned. 
  pub fn force_full(&mut self, force_full: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_force_full(force_full);
    self
  }
  

  
}


///  Returns an instant view version of a web page if available. Returns a 404 error if the web page has no instant view page. 
#[derive(Debug, Clone)]
pub struct TGGetWebPageInstantView {
  inner: GetWebPageInstantView
}

impl TDFB for TGGetWebPageInstantView {}

impl AsRef<TGGetWebPageInstantView> for TGGetWebPageInstantView {
  fn as_ref(&self) -> &TGGetWebPageInstantView { self }
}

impl AsRef<TGGetWebPageInstantView> for _TGGetWebPageInstantViewBuilder {
  fn as_ref(&self) -> &TGGetWebPageInstantView { &self.inner }
}

impl TGGetWebPageInstantView {

  pub fn builder() -> _TGGetWebPageInstantViewBuilder {
    _TGGetWebPageInstantViewBuilder { inner: Self::new(GetWebPageInstantView::_new()) }
  }

  pub fn new(inner: GetWebPageInstantView) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetWebPageInstantView { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetWebPageInstantView { &mut self.inner }

}


#[doc(hidden)] pub struct _TGGetWebPagePreviewBuilder { inner: TGGetWebPagePreview }

impl _TGGetWebPagePreviewBuilder {

  pub fn build(&self) -> TGGetWebPagePreview { self.inner.clone() }

  

  
  // [text] type is [FormattedText], is not support, need add manully.
  #[doc(hidden)] pub fn _text(&mut self, text: FormattedText) -> &mut Self {
    self.inner.td_origin_mut()._set_text(text);
    self
  }
  
}


///  Returns a web page preview by the text of the message. Do not call this function too often. Returns a 404 error if the web page has no preview. 
#[derive(Debug, Clone)]
pub struct TGGetWebPagePreview {
  inner: GetWebPagePreview
}

impl TDFB for TGGetWebPagePreview {}

impl AsRef<TGGetWebPagePreview> for TGGetWebPagePreview {
  fn as_ref(&self) -> &TGGetWebPagePreview { self }
}

impl AsRef<TGGetWebPagePreview> for _TGGetWebPagePreviewBuilder {
  fn as_ref(&self) -> &TGGetWebPagePreview { &self.inner }
}

impl TGGetWebPagePreview {

  pub fn builder() -> _TGGetWebPagePreviewBuilder {
    _TGGetWebPagePreviewBuilder { inner: Self::new(GetWebPagePreview::_new()) }
  }

  pub fn new(inner: GetWebPagePreview) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &GetWebPagePreview { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut GetWebPagePreview { &mut self.inner }

}


#[doc(hidden)] pub struct _TGImportContactsBuilder { inner: TGImportContacts }

impl _TGImportContactsBuilder {

  pub fn build(&self) -> TGImportContacts { self.inner.clone() }

  

  
  // [contacts] type is [Vec<Contact>], is not support, need add manully.
  #[doc(hidden)] pub fn _contacts(&mut self, contacts: Vec<Contact>) -> &mut Self {
    self.inner.td_origin_mut()._set_contacts(contacts);
    self
  }
  
}


///  Adds new contacts or edits existing contacts; contacts' user identifiers are ignored. 
#[derive(Debug, Clone)]
pub struct TGImportContacts {
  inner: ImportContacts
}

impl TDFB for TGImportContacts {}

impl AsRef<TGImportContacts> for TGImportContacts {
  fn as_ref(&self) -> &TGImportContacts { self }
}

impl AsRef<TGImportContacts> for _TGImportContactsBuilder {
  fn as_ref(&self) -> &TGImportContacts { &self.inner }
}

impl TGImportContacts {

  pub fn builder() -> _TGImportContactsBuilder {
    _TGImportContactsBuilder { inner: Self::new(ImportContacts::_new()) }
  }

  pub fn new(inner: ImportContacts) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ImportContacts { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ImportContacts { &mut self.inner }

}


#[doc(hidden)] pub struct _TGJoinChatBuilder { inner: TGJoinChat }

impl _TGJoinChatBuilder {

  pub fn build(&self) -> TGJoinChat { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  

  
}


///  Adds current user as a new member to a chat. Private and secret chats can't be joined using this method. 
#[derive(Debug, Clone)]
pub struct TGJoinChat {
  inner: JoinChat
}

impl TDFB for TGJoinChat {}

impl AsRef<TGJoinChat> for TGJoinChat {
  fn as_ref(&self) -> &TGJoinChat { self }
}

impl AsRef<TGJoinChat> for _TGJoinChatBuilder {
  fn as_ref(&self) -> &TGJoinChat { &self.inner }
}

impl TGJoinChat {

  pub fn builder() -> _TGJoinChatBuilder {
    _TGJoinChatBuilder { inner: Self::new(JoinChat::_new()) }
  }

  pub fn new(inner: JoinChat) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &JoinChat { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut JoinChat { &mut self.inner }

}


#[doc(hidden)] pub struct _TGJoinChatByInviteLinkBuilder { inner: TGJoinChatByInviteLink }

impl _TGJoinChatByInviteLinkBuilder {

  pub fn build(&self) -> TGJoinChatByInviteLink { self.inner.clone() }

  ///  Invite link to import; should begin with "https://t.me/joinchat/", "https://telegram.me/joinchat/", or "https://telegram.dog/joinchat/". 
  pub fn invite_link<S: AsRef<str>>(&mut self, invite_link: S) -> &mut Self {
    self.inner.td_origin_mut()._set_invite_link(invite_link.as_ref().to_string());
    self
  }
  

  
}


///  Uses an invite link to add the current user to the chat if possible. The new member will not be added until the chat state has been synchronized with the server. 
#[derive(Debug, Clone)]
pub struct TGJoinChatByInviteLink {
  inner: JoinChatByInviteLink
}

impl TDFB for TGJoinChatByInviteLink {}

impl AsRef<TGJoinChatByInviteLink> for TGJoinChatByInviteLink {
  fn as_ref(&self) -> &TGJoinChatByInviteLink { self }
}

impl AsRef<TGJoinChatByInviteLink> for _TGJoinChatByInviteLinkBuilder {
  fn as_ref(&self) -> &TGJoinChatByInviteLink { &self.inner }
}

impl TGJoinChatByInviteLink {

  pub fn builder() -> _TGJoinChatByInviteLinkBuilder {
    _TGJoinChatByInviteLinkBuilder { inner: Self::new(JoinChatByInviteLink::_new()) }
  }

  pub fn new(inner: JoinChatByInviteLink) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &JoinChatByInviteLink { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut JoinChatByInviteLink { &mut self.inner }

}


#[doc(hidden)] pub struct _TGLeaveChatBuilder { inner: TGLeaveChat }

impl _TGLeaveChatBuilder {

  pub fn build(&self) -> TGLeaveChat { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  

  
}


///  Removes current user from chat members. Private and secret chats can't be left using this method. 
#[derive(Debug, Clone)]
pub struct TGLeaveChat {
  inner: LeaveChat
}

impl TDFB for TGLeaveChat {}

impl AsRef<TGLeaveChat> for TGLeaveChat {
  fn as_ref(&self) -> &TGLeaveChat { self }
}

impl AsRef<TGLeaveChat> for _TGLeaveChatBuilder {
  fn as_ref(&self) -> &TGLeaveChat { &self.inner }
}

impl TGLeaveChat {

  pub fn builder() -> _TGLeaveChatBuilder {
    _TGLeaveChatBuilder { inner: Self::new(LeaveChat::_new()) }
  }

  pub fn new(inner: LeaveChat) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &LeaveChat { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut LeaveChat { &mut self.inner }

}


#[doc(hidden)] pub struct _TGLogOutBuilder { inner: TGLogOut }

impl _TGLogOutBuilder {

  pub fn build(&self) -> TGLogOut { self.inner.clone() }

  

  
}


///  Closes the TDLib instance after a proper logout. Requires an available network connection. All local data will be destroyed. After the logout completes,  
#[derive(Debug, Clone)]
pub struct TGLogOut {
  inner: LogOut
}

impl TDFB for TGLogOut {}

impl AsRef<TGLogOut> for TGLogOut {
  fn as_ref(&self) -> &TGLogOut { self }
}

impl AsRef<TGLogOut> for _TGLogOutBuilder {
  fn as_ref(&self) -> &TGLogOut { &self.inner }
}

impl TGLogOut {

  pub fn builder() -> _TGLogOutBuilder {
    _TGLogOutBuilder { inner: Self::new(LogOut::_new()) }
  }

  pub fn new(inner: LogOut) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &LogOut { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut LogOut { &mut self.inner }

}


#[doc(hidden)] pub struct _TGOpenChatBuilder { inner: TGOpenChat }

impl _TGOpenChatBuilder {

  pub fn build(&self) -> TGOpenChat { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  

  
}


///  Informs TDLib that the chat is opened by the user. Many useful activities depend on the chat being opened or closed (e.g., in supergroups and channels all updates are received only for opened chats). 
#[derive(Debug, Clone)]
pub struct TGOpenChat {
  inner: OpenChat
}

impl TDFB for TGOpenChat {}

impl AsRef<TGOpenChat> for TGOpenChat {
  fn as_ref(&self) -> &TGOpenChat { self }
}

impl AsRef<TGOpenChat> for _TGOpenChatBuilder {
  fn as_ref(&self) -> &TGOpenChat { &self.inner }
}

impl TGOpenChat {

  pub fn builder() -> _TGOpenChatBuilder {
    _TGOpenChatBuilder { inner: Self::new(OpenChat::_new()) }
  }

  pub fn new(inner: OpenChat) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &OpenChat { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut OpenChat { &mut self.inner }

}


#[doc(hidden)] pub struct _TGOpenMessageContentBuilder { inner: TGOpenMessageContent }

impl _TGOpenMessageContentBuilder {

  pub fn build(&self) -> TGOpenMessageContent { self.inner.clone() }

  ///  Chat identifier of the message. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Identifier of the message with the opened content. 
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_message_id(message_id);
    self
  }
  

  
}


///  Informs TDLib that the message content has been opened (e.g., the user has opened a photo, video, document, location or venue, or has listened to an audio file or voice note message). An  
#[derive(Debug, Clone)]
pub struct TGOpenMessageContent {
  inner: OpenMessageContent
}

impl TDFB for TGOpenMessageContent {}

impl AsRef<TGOpenMessageContent> for TGOpenMessageContent {
  fn as_ref(&self) -> &TGOpenMessageContent { self }
}

impl AsRef<TGOpenMessageContent> for _TGOpenMessageContentBuilder {
  fn as_ref(&self) -> &TGOpenMessageContent { &self.inner }
}

impl TGOpenMessageContent {

  pub fn builder() -> _TGOpenMessageContentBuilder {
    _TGOpenMessageContentBuilder { inner: Self::new(OpenMessageContent::_new()) }
  }

  pub fn new(inner: OpenMessageContent) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &OpenMessageContent { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut OpenMessageContent { &mut self.inner }

}


#[doc(hidden)] pub struct _TGOptimizeStorageBuilder { inner: TGOptimizeStorage }

impl _TGOptimizeStorageBuilder {

  pub fn build(&self) -> TGOptimizeStorage { self.inner.clone() }

  ///  Limit on the total size of files after deletion. Pass -1 to use the default limit. 
  pub fn size(&mut self, size: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_size(size);
    self
  }
  ///  Limit on the time that has passed since the last time a file was accessed (or creation time for some filesystems). Pass -1 to use the default limit. 
  pub fn ttl(&mut self, ttl: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_ttl(ttl);
    self
  }
  ///  Limit on the total count of files after deletion. Pass -1 to use the default limit. 
  pub fn count(&mut self, count: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_count(count);
    self
  }
  ///  The amount of time after the creation of a file during which it can't be deleted, in seconds. Pass -1 to use the default value. 
  pub fn immunity_delay(&mut self, immunity_delay: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_immunity_delay(immunity_delay);
    self
  }
  ///  If not empty, only files from the given chats are considered. Use 0 as chat identifier to delete files not belonging to any chat (e.g., profile photos). 
  pub fn chat_ids(&mut self, chat_ids: Vec<i64>) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_ids(chat_ids);
    self
  }
  ///  If not empty, files from the given chats are excluded. Use 0 as chat identifier to exclude all files not belonging to any chat (e.g., profile photos). 
  pub fn exclude_chat_ids(&mut self, exclude_chat_ids: Vec<i64>) -> &mut Self {
    self.inner.td_origin_mut()._set_exclude_chat_ids(exclude_chat_ids);
    self
  }
  ///  Same as in getStorageStatistics. Affects only returned statistics. 
  pub fn chat_limit(&mut self, chat_limit: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_limit(chat_limit);
    self
  }
  

  
  // [file_types] type is [Vec<Box<FileType>>], is not support, need add manully.
  #[doc(hidden)] pub fn _file_types(&mut self, file_types: Vec<Box<FileType>>) -> &mut Self {
    self.inner.td_origin_mut()._set_file_types(file_types);
    self
  }
  
}


///  Optimizes storage usage, i.e. deletes some files and returns new storage usage statistics. Secret thumbnails can't be deleted. 
#[derive(Debug, Clone)]
pub struct TGOptimizeStorage {
  inner: OptimizeStorage
}

impl TDFB for TGOptimizeStorage {}

impl AsRef<TGOptimizeStorage> for TGOptimizeStorage {
  fn as_ref(&self) -> &TGOptimizeStorage { self }
}

impl AsRef<TGOptimizeStorage> for _TGOptimizeStorageBuilder {
  fn as_ref(&self) -> &TGOptimizeStorage { &self.inner }
}

impl TGOptimizeStorage {

  pub fn builder() -> _TGOptimizeStorageBuilder {
    _TGOptimizeStorageBuilder { inner: Self::new(OptimizeStorage::_new()) }
  }

  pub fn new(inner: OptimizeStorage) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &OptimizeStorage { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut OptimizeStorage { &mut self.inner }

}


#[doc(hidden)] pub struct _TGParseTextEntitiesBuilder { inner: TGParseTextEntities }

impl _TGParseTextEntitiesBuilder {

  pub fn build(&self) -> TGParseTextEntities { self.inner.clone() }

  ///  The text which should be parsed. 
  pub fn text<S: AsRef<str>>(&mut self, text: S) -> &mut Self {
    self.inner.td_origin_mut()._set_text(text.as_ref().to_string());
    self
  }
  

  
  // [parse_mode] type is [Box<TextParseMode>], is not support, need add manully.
  #[doc(hidden)] pub fn _parse_mode(&mut self, parse_mode: Box<TextParseMode>) -> &mut Self {
    self.inner.td_origin_mut()._set_parse_mode(parse_mode);
    self
  }
  
}


///  Parses Bold, Italic, Code, Pre, PreCode and TextUrl entities contained in the text. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGParseTextEntities {
  inner: ParseTextEntities
}

impl TDFB for TGParseTextEntities {}

impl AsRef<TGParseTextEntities> for TGParseTextEntities {
  fn as_ref(&self) -> &TGParseTextEntities { self }
}

impl AsRef<TGParseTextEntities> for _TGParseTextEntitiesBuilder {
  fn as_ref(&self) -> &TGParseTextEntities { &self.inner }
}

impl TGParseTextEntities {

  pub fn builder() -> _TGParseTextEntitiesBuilder {
    _TGParseTextEntitiesBuilder { inner: Self::new(ParseTextEntities::_new()) }
  }

  pub fn new(inner: ParseTextEntities) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ParseTextEntities { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ParseTextEntities { &mut self.inner }

}


#[doc(hidden)] pub struct _TGPinChatMessageBuilder { inner: TGPinChatMessage }

impl _TGPinChatMessageBuilder {

  pub fn build(&self) -> TGPinChatMessage { self.inner.clone() }

  ///  Identifier of the chat. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Identifier of the new pinned message. 
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_message_id(message_id);
    self
  }
  ///  True, if there should be no notification about the pinned message. 
  pub fn disable_notification(&mut self, disable_notification: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_disable_notification(disable_notification);
    self
  }
  

  
}


///  Pins a message in a chat; requires appropriate administrator rights in the group or channel. 
#[derive(Debug, Clone)]
pub struct TGPinChatMessage {
  inner: PinChatMessage
}

impl TDFB for TGPinChatMessage {}

impl AsRef<TGPinChatMessage> for TGPinChatMessage {
  fn as_ref(&self) -> &TGPinChatMessage { self }
}

impl AsRef<TGPinChatMessage> for _TGPinChatMessageBuilder {
  fn as_ref(&self) -> &TGPinChatMessage { &self.inner }
}

impl TGPinChatMessage {

  pub fn builder() -> _TGPinChatMessageBuilder {
    _TGPinChatMessageBuilder { inner: Self::new(PinChatMessage::_new()) }
  }

  pub fn new(inner: PinChatMessage) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &PinChatMessage { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut PinChatMessage { &mut self.inner }

}


#[doc(hidden)] pub struct _TGPingProxyBuilder { inner: TGPingProxy }

impl _TGPingProxyBuilder {

  pub fn build(&self) -> TGPingProxy { self.inner.clone() }

  ///  Proxy identifier. Use 0 to ping a Telegram server without a proxy. 
  pub fn proxy_id(&mut self, proxy_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_proxy_id(proxy_id);
    self
  }
  

  
}


///  Computes time needed to receive a response from a Telegram server through a proxy. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGPingProxy {
  inner: PingProxy
}

impl TDFB for TGPingProxy {}

impl AsRef<TGPingProxy> for TGPingProxy {
  fn as_ref(&self) -> &TGPingProxy { self }
}

impl AsRef<TGPingProxy> for _TGPingProxyBuilder {
  fn as_ref(&self) -> &TGPingProxy { &self.inner }
}

impl TGPingProxy {

  pub fn builder() -> _TGPingProxyBuilder {
    _TGPingProxyBuilder { inner: Self::new(PingProxy::_new()) }
  }

  pub fn new(inner: PingProxy) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &PingProxy { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut PingProxy { &mut self.inner }

}


#[doc(hidden)] pub struct _TGProcessPushNotificationBuilder { inner: TGProcessPushNotification }

impl _TGProcessPushNotificationBuilder {

  pub fn build(&self) -> TGProcessPushNotification { self.inner.clone() }

  ///  JSON-encoded push notification payload with all fields sent by the server, and "google.sent_time" and "google.notification.sound" fields added. 
  pub fn payload<S: AsRef<str>>(&mut self, payload: S) -> &mut Self {
    self.inner.td_origin_mut()._set_payload(payload.as_ref().to_string());
    self
  }
  

  
}


///  Handles a push notification. Returns error with code 406 if the push notification is not supported and connection to the server is required to fetch new data. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGProcessPushNotification {
  inner: ProcessPushNotification
}

impl TDFB for TGProcessPushNotification {}

impl AsRef<TGProcessPushNotification> for TGProcessPushNotification {
  fn as_ref(&self) -> &TGProcessPushNotification { self }
}

impl AsRef<TGProcessPushNotification> for _TGProcessPushNotificationBuilder {
  fn as_ref(&self) -> &TGProcessPushNotification { &self.inner }
}

impl TGProcessPushNotification {

  pub fn builder() -> _TGProcessPushNotificationBuilder {
    _TGProcessPushNotificationBuilder { inner: Self::new(ProcessPushNotification::_new()) }
  }

  pub fn new(inner: ProcessPushNotification) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ProcessPushNotification { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ProcessPushNotification { &mut self.inner }

}


#[doc(hidden)] pub struct _TGReadAllChatMentionsBuilder { inner: TGReadAllChatMentions }

impl _TGReadAllChatMentionsBuilder {

  pub fn build(&self) -> TGReadAllChatMentions { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  

  
}


///  Marks all mentions in a chat as read. 
#[derive(Debug, Clone)]
pub struct TGReadAllChatMentions {
  inner: ReadAllChatMentions
}

impl TDFB for TGReadAllChatMentions {}

impl AsRef<TGReadAllChatMentions> for TGReadAllChatMentions {
  fn as_ref(&self) -> &TGReadAllChatMentions { self }
}

impl AsRef<TGReadAllChatMentions> for _TGReadAllChatMentionsBuilder {
  fn as_ref(&self) -> &TGReadAllChatMentions { &self.inner }
}

impl TGReadAllChatMentions {

  pub fn builder() -> _TGReadAllChatMentionsBuilder {
    _TGReadAllChatMentionsBuilder { inner: Self::new(ReadAllChatMentions::_new()) }
  }

  pub fn new(inner: ReadAllChatMentions) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ReadAllChatMentions { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ReadAllChatMentions { &mut self.inner }

}


#[doc(hidden)] pub struct _TGReadFilePartBuilder { inner: TGReadFilePart }

impl _TGReadFilePartBuilder {

  pub fn build(&self) -> TGReadFilePart { self.inner.clone() }

  ///  Identifier of the file. The file must be located in the TDLib file cache. 
  pub fn file_id(&mut self, file_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_file_id(file_id);
    self
  }
  ///  The offset from which to read the file. 
  pub fn offset(&mut self, offset: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_offset(offset);
    self
  }
  ///  Number of bytes to read. An error will be returned if there are not enough bytes available in the file from the specified position. Pass 0 to read all available data from the specified position. 
  pub fn count(&mut self, count: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_count(count);
    self
  }
  

  
}


///  Reads a part of a file from the TDLib file cache and returns read bytes. This method is intended to be used only if the client has no direct access to TDLib's file system, because it is usually slower than a direct read from the file. 
#[derive(Debug, Clone)]
pub struct TGReadFilePart {
  inner: ReadFilePart
}

impl TDFB for TGReadFilePart {}

impl AsRef<TGReadFilePart> for TGReadFilePart {
  fn as_ref(&self) -> &TGReadFilePart { self }
}

impl AsRef<TGReadFilePart> for _TGReadFilePartBuilder {
  fn as_ref(&self) -> &TGReadFilePart { &self.inner }
}

impl TGReadFilePart {

  pub fn builder() -> _TGReadFilePartBuilder {
    _TGReadFilePartBuilder { inner: Self::new(ReadFilePart::_new()) }
  }

  pub fn new(inner: ReadFilePart) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ReadFilePart { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ReadFilePart { &mut self.inner }

}


#[doc(hidden)] pub struct _TGRecoverAuthenticationPasswordBuilder { inner: TGRecoverAuthenticationPassword }

impl _TGRecoverAuthenticationPasswordBuilder {

  pub fn build(&self) -> TGRecoverAuthenticationPassword { self.inner.clone() }

  ///  Recovery code to check. 
  pub fn recovery_code<S: AsRef<str>>(&mut self, recovery_code: S) -> &mut Self {
    self.inner.td_origin_mut()._set_recovery_code(recovery_code.as_ref().to_string());
    self
  }
  

  
}


///  Recovers the password with a password recovery code sent to an email address that was previously set up. Works only when the current authorization state is  
#[derive(Debug, Clone)]
pub struct TGRecoverAuthenticationPassword {
  inner: RecoverAuthenticationPassword
}

impl TDFB for TGRecoverAuthenticationPassword {}

impl AsRef<TGRecoverAuthenticationPassword> for TGRecoverAuthenticationPassword {
  fn as_ref(&self) -> &TGRecoverAuthenticationPassword { self }
}

impl AsRef<TGRecoverAuthenticationPassword> for _TGRecoverAuthenticationPasswordBuilder {
  fn as_ref(&self) -> &TGRecoverAuthenticationPassword { &self.inner }
}

impl TGRecoverAuthenticationPassword {

  pub fn builder() -> _TGRecoverAuthenticationPasswordBuilder {
    _TGRecoverAuthenticationPasswordBuilder { inner: Self::new(RecoverAuthenticationPassword::_new()) }
  }

  pub fn new(inner: RecoverAuthenticationPassword) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &RecoverAuthenticationPassword { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut RecoverAuthenticationPassword { &mut self.inner }

}


#[doc(hidden)] pub struct _TGRecoverPasswordBuilder { inner: TGRecoverPassword }

impl _TGRecoverPasswordBuilder {

  pub fn build(&self) -> TGRecoverPassword { self.inner.clone() }

  ///  Recovery code to check. 
  pub fn recovery_code<S: AsRef<str>>(&mut self, recovery_code: S) -> &mut Self {
    self.inner.td_origin_mut()._set_recovery_code(recovery_code.as_ref().to_string());
    self
  }
  

  
}


///  Recovers the password using a recovery code sent to an email address that was previously set up. 
#[derive(Debug, Clone)]
pub struct TGRecoverPassword {
  inner: RecoverPassword
}

impl TDFB for TGRecoverPassword {}

impl AsRef<TGRecoverPassword> for TGRecoverPassword {
  fn as_ref(&self) -> &TGRecoverPassword { self }
}

impl AsRef<TGRecoverPassword> for _TGRecoverPasswordBuilder {
  fn as_ref(&self) -> &TGRecoverPassword { &self.inner }
}

impl TGRecoverPassword {

  pub fn builder() -> _TGRecoverPasswordBuilder {
    _TGRecoverPasswordBuilder { inner: Self::new(RecoverPassword::_new()) }
  }

  pub fn new(inner: RecoverPassword) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &RecoverPassword { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut RecoverPassword { &mut self.inner }

}


#[doc(hidden)] pub struct _TGRegisterDeviceBuilder { inner: TGRegisterDevice }

impl _TGRegisterDeviceBuilder {

  pub fn build(&self) -> TGRegisterDevice { self.inner.clone() }

  ///  List of user identifiers of other users currently using the client. 
  pub fn other_user_ids(&mut self, other_user_ids: Vec<i32>) -> &mut Self {
    self.inner.td_origin_mut()._set_other_user_ids(other_user_ids);
    self
  }
  

  
  // [device_token] type is [Box<DeviceToken>], is not support, need add manully.
  #[doc(hidden)] pub fn _device_token(&mut self, device_token: Box<DeviceToken>) -> &mut Self {
    self.inner.td_origin_mut()._set_device_token(device_token);
    self
  }
  
}


///  Registers the currently used device for receiving push notifications. Returns a globally unique identifier of the push notification subscription. 
#[derive(Debug, Clone)]
pub struct TGRegisterDevice {
  inner: RegisterDevice
}

impl TDFB for TGRegisterDevice {}

impl AsRef<TGRegisterDevice> for TGRegisterDevice {
  fn as_ref(&self) -> &TGRegisterDevice { self }
}

impl AsRef<TGRegisterDevice> for _TGRegisterDeviceBuilder {
  fn as_ref(&self) -> &TGRegisterDevice { &self.inner }
}

impl TGRegisterDevice {

  pub fn builder() -> _TGRegisterDeviceBuilder {
    _TGRegisterDeviceBuilder { inner: Self::new(RegisterDevice::_new()) }
  }

  pub fn new(inner: RegisterDevice) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &RegisterDevice { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut RegisterDevice { &mut self.inner }

}


#[doc(hidden)] pub struct _TGRemoveContactsBuilder { inner: TGRemoveContacts }

impl _TGRemoveContactsBuilder {

  pub fn build(&self) -> TGRemoveContacts { self.inner.clone() }

  ///  Identifiers of users to be deleted. 
  pub fn user_ids(&mut self, user_ids: Vec<i32>) -> &mut Self {
    self.inner.td_origin_mut()._set_user_ids(user_ids);
    self
  }
  

  
}


///  Removes users from the contact list. 
#[derive(Debug, Clone)]
pub struct TGRemoveContacts {
  inner: RemoveContacts
}

impl TDFB for TGRemoveContacts {}

impl AsRef<TGRemoveContacts> for TGRemoveContacts {
  fn as_ref(&self) -> &TGRemoveContacts { self }
}

impl AsRef<TGRemoveContacts> for _TGRemoveContactsBuilder {
  fn as_ref(&self) -> &TGRemoveContacts { &self.inner }
}

impl TGRemoveContacts {

  pub fn builder() -> _TGRemoveContactsBuilder {
    _TGRemoveContactsBuilder { inner: Self::new(RemoveContacts::_new()) }
  }

  pub fn new(inner: RemoveContacts) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &RemoveContacts { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut RemoveContacts { &mut self.inner }

}


#[doc(hidden)] pub struct _TGRemoveFavoriteStickerBuilder { inner: TGRemoveFavoriteSticker }

impl _TGRemoveFavoriteStickerBuilder {

  pub fn build(&self) -> TGRemoveFavoriteSticker { self.inner.clone() }

  

  
  // [sticker] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _sticker(&mut self, sticker: Box<InputFile>) -> &mut Self {
    self.inner.td_origin_mut()._set_sticker(sticker);
    self
  }
  
}


///  Removes a sticker from the list of favorite stickers. 
#[derive(Debug, Clone)]
pub struct TGRemoveFavoriteSticker {
  inner: RemoveFavoriteSticker
}

impl TDFB for TGRemoveFavoriteSticker {}

impl AsRef<TGRemoveFavoriteSticker> for TGRemoveFavoriteSticker {
  fn as_ref(&self) -> &TGRemoveFavoriteSticker { self }
}

impl AsRef<TGRemoveFavoriteSticker> for _TGRemoveFavoriteStickerBuilder {
  fn as_ref(&self) -> &TGRemoveFavoriteSticker { &self.inner }
}

impl TGRemoveFavoriteSticker {

  pub fn builder() -> _TGRemoveFavoriteStickerBuilder {
    _TGRemoveFavoriteStickerBuilder { inner: Self::new(RemoveFavoriteSticker::_new()) }
  }

  pub fn new(inner: RemoveFavoriteSticker) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &RemoveFavoriteSticker { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut RemoveFavoriteSticker { &mut self.inner }

}


#[doc(hidden)] pub struct _TGRemoveNotificationBuilder { inner: TGRemoveNotification }

impl _TGRemoveNotificationBuilder {

  pub fn build(&self) -> TGRemoveNotification { self.inner.clone() }

  ///  Identifier of notification group to which the notification belongs. 
  pub fn notification_group_id(&mut self, notification_group_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_notification_group_id(notification_group_id);
    self
  }
  ///  Identifier of removed notification. 
  pub fn notification_id(&mut self, notification_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_notification_id(notification_id);
    self
  }
  

  
}


///  Removes an active notification from notification list. Needs to be called only if the notification is removed by the current user. 
#[derive(Debug, Clone)]
pub struct TGRemoveNotification {
  inner: RemoveNotification
}

impl TDFB for TGRemoveNotification {}

impl AsRef<TGRemoveNotification> for TGRemoveNotification {
  fn as_ref(&self) -> &TGRemoveNotification { self }
}

impl AsRef<TGRemoveNotification> for _TGRemoveNotificationBuilder {
  fn as_ref(&self) -> &TGRemoveNotification { &self.inner }
}

impl TGRemoveNotification {

  pub fn builder() -> _TGRemoveNotificationBuilder {
    _TGRemoveNotificationBuilder { inner: Self::new(RemoveNotification::_new()) }
  }

  pub fn new(inner: RemoveNotification) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &RemoveNotification { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut RemoveNotification { &mut self.inner }

}


#[doc(hidden)] pub struct _TGRemoveNotificationGroupBuilder { inner: TGRemoveNotificationGroup }

impl _TGRemoveNotificationGroupBuilder {

  pub fn build(&self) -> TGRemoveNotificationGroup { self.inner.clone() }

  ///  Notification group identifier. 
  pub fn notification_group_id(&mut self, notification_group_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_notification_group_id(notification_group_id);
    self
  }
  ///  Maximum identifier of removed notifications. 
  pub fn max_notification_id(&mut self, max_notification_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_max_notification_id(max_notification_id);
    self
  }
  

  
}


///  Removes a group of active notifications. Needs to be called only if the notification group is removed by the current user. 
#[derive(Debug, Clone)]
pub struct TGRemoveNotificationGroup {
  inner: RemoveNotificationGroup
}

impl TDFB for TGRemoveNotificationGroup {}

impl AsRef<TGRemoveNotificationGroup> for TGRemoveNotificationGroup {
  fn as_ref(&self) -> &TGRemoveNotificationGroup { self }
}

impl AsRef<TGRemoveNotificationGroup> for _TGRemoveNotificationGroupBuilder {
  fn as_ref(&self) -> &TGRemoveNotificationGroup { &self.inner }
}

impl TGRemoveNotificationGroup {

  pub fn builder() -> _TGRemoveNotificationGroupBuilder {
    _TGRemoveNotificationGroupBuilder { inner: Self::new(RemoveNotificationGroup::_new()) }
  }

  pub fn new(inner: RemoveNotificationGroup) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &RemoveNotificationGroup { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut RemoveNotificationGroup { &mut self.inner }

}


#[doc(hidden)] pub struct _TGRemoveProxyBuilder { inner: TGRemoveProxy }

impl _TGRemoveProxyBuilder {

  pub fn build(&self) -> TGRemoveProxy { self.inner.clone() }

  ///  Proxy identifier. 
  pub fn proxy_id(&mut self, proxy_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_proxy_id(proxy_id);
    self
  }
  

  
}


///  Removes a proxy server. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGRemoveProxy {
  inner: RemoveProxy
}

impl TDFB for TGRemoveProxy {}

impl AsRef<TGRemoveProxy> for TGRemoveProxy {
  fn as_ref(&self) -> &TGRemoveProxy { self }
}

impl AsRef<TGRemoveProxy> for _TGRemoveProxyBuilder {
  fn as_ref(&self) -> &TGRemoveProxy { &self.inner }
}

impl TGRemoveProxy {

  pub fn builder() -> _TGRemoveProxyBuilder {
    _TGRemoveProxyBuilder { inner: Self::new(RemoveProxy::_new()) }
  }

  pub fn new(inner: RemoveProxy) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &RemoveProxy { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut RemoveProxy { &mut self.inner }

}


#[doc(hidden)] pub struct _TGRemoveRecentHashtagBuilder { inner: TGRemoveRecentHashtag }

impl _TGRemoveRecentHashtagBuilder {

  pub fn build(&self) -> TGRemoveRecentHashtag { self.inner.clone() }

  ///  Hashtag to delete. 
  pub fn hashtag<S: AsRef<str>>(&mut self, hashtag: S) -> &mut Self {
    self.inner.td_origin_mut()._set_hashtag(hashtag.as_ref().to_string());
    self
  }
  

  
}


///  Removes a hashtag from the list of recently used hashtags. 
#[derive(Debug, Clone)]
pub struct TGRemoveRecentHashtag {
  inner: RemoveRecentHashtag
}

impl TDFB for TGRemoveRecentHashtag {}

impl AsRef<TGRemoveRecentHashtag> for TGRemoveRecentHashtag {
  fn as_ref(&self) -> &TGRemoveRecentHashtag { self }
}

impl AsRef<TGRemoveRecentHashtag> for _TGRemoveRecentHashtagBuilder {
  fn as_ref(&self) -> &TGRemoveRecentHashtag { &self.inner }
}

impl TGRemoveRecentHashtag {

  pub fn builder() -> _TGRemoveRecentHashtagBuilder {
    _TGRemoveRecentHashtagBuilder { inner: Self::new(RemoveRecentHashtag::_new()) }
  }

  pub fn new(inner: RemoveRecentHashtag) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &RemoveRecentHashtag { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut RemoveRecentHashtag { &mut self.inner }

}


#[doc(hidden)] pub struct _TGRemoveRecentlyFoundChatBuilder { inner: TGRemoveRecentlyFoundChat }

impl _TGRemoveRecentlyFoundChatBuilder {

  pub fn build(&self) -> TGRemoveRecentlyFoundChat { self.inner.clone() }

  ///  Identifier of the chat to be removed. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  

  
}


///  Removes a chat from the list of recently found chats. 
#[derive(Debug, Clone)]
pub struct TGRemoveRecentlyFoundChat {
  inner: RemoveRecentlyFoundChat
}

impl TDFB for TGRemoveRecentlyFoundChat {}

impl AsRef<TGRemoveRecentlyFoundChat> for TGRemoveRecentlyFoundChat {
  fn as_ref(&self) -> &TGRemoveRecentlyFoundChat { self }
}

impl AsRef<TGRemoveRecentlyFoundChat> for _TGRemoveRecentlyFoundChatBuilder {
  fn as_ref(&self) -> &TGRemoveRecentlyFoundChat { &self.inner }
}

impl TGRemoveRecentlyFoundChat {

  pub fn builder() -> _TGRemoveRecentlyFoundChatBuilder {
    _TGRemoveRecentlyFoundChatBuilder { inner: Self::new(RemoveRecentlyFoundChat::_new()) }
  }

  pub fn new(inner: RemoveRecentlyFoundChat) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &RemoveRecentlyFoundChat { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut RemoveRecentlyFoundChat { &mut self.inner }

}


#[doc(hidden)] pub struct _TGRemoveRecentStickerBuilder { inner: TGRemoveRecentSticker }

impl _TGRemoveRecentStickerBuilder {

  pub fn build(&self) -> TGRemoveRecentSticker { self.inner.clone() }

  ///  Pass true to remove the sticker from the list of stickers recently attached to photo or video files; pass false to remove the sticker from the list of recently sent stickers. 
  pub fn is_attached(&mut self, is_attached: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_is_attached(is_attached);
    self
  }
  

  
  // [sticker] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _sticker(&mut self, sticker: Box<InputFile>) -> &mut Self {
    self.inner.td_origin_mut()._set_sticker(sticker);
    self
  }
  
}


///  Removes a sticker from the list of recently used stickers. 
#[derive(Debug, Clone)]
pub struct TGRemoveRecentSticker {
  inner: RemoveRecentSticker
}

impl TDFB for TGRemoveRecentSticker {}

impl AsRef<TGRemoveRecentSticker> for TGRemoveRecentSticker {
  fn as_ref(&self) -> &TGRemoveRecentSticker { self }
}

impl AsRef<TGRemoveRecentSticker> for _TGRemoveRecentStickerBuilder {
  fn as_ref(&self) -> &TGRemoveRecentSticker { &self.inner }
}

impl TGRemoveRecentSticker {

  pub fn builder() -> _TGRemoveRecentStickerBuilder {
    _TGRemoveRecentStickerBuilder { inner: Self::new(RemoveRecentSticker::_new()) }
  }

  pub fn new(inner: RemoveRecentSticker) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &RemoveRecentSticker { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut RemoveRecentSticker { &mut self.inner }

}


#[doc(hidden)] pub struct _TGRemoveSavedAnimationBuilder { inner: TGRemoveSavedAnimation }

impl _TGRemoveSavedAnimationBuilder {

  pub fn build(&self) -> TGRemoveSavedAnimation { self.inner.clone() }

  

  
  // [animation] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _animation(&mut self, animation: Box<InputFile>) -> &mut Self {
    self.inner.td_origin_mut()._set_animation(animation);
    self
  }
  
}


///  Removes an animation from the list of saved animations. 
#[derive(Debug, Clone)]
pub struct TGRemoveSavedAnimation {
  inner: RemoveSavedAnimation
}

impl TDFB for TGRemoveSavedAnimation {}

impl AsRef<TGRemoveSavedAnimation> for TGRemoveSavedAnimation {
  fn as_ref(&self) -> &TGRemoveSavedAnimation { self }
}

impl AsRef<TGRemoveSavedAnimation> for _TGRemoveSavedAnimationBuilder {
  fn as_ref(&self) -> &TGRemoveSavedAnimation { &self.inner }
}

impl TGRemoveSavedAnimation {

  pub fn builder() -> _TGRemoveSavedAnimationBuilder {
    _TGRemoveSavedAnimationBuilder { inner: Self::new(RemoveSavedAnimation::_new()) }
  }

  pub fn new(inner: RemoveSavedAnimation) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &RemoveSavedAnimation { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut RemoveSavedAnimation { &mut self.inner }

}


#[doc(hidden)] pub struct _TGRemoveStickerFromSetBuilder { inner: TGRemoveStickerFromSet }

impl _TGRemoveStickerFromSetBuilder {

  pub fn build(&self) -> TGRemoveStickerFromSet { self.inner.clone() }

  

  
  // [sticker] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _sticker(&mut self, sticker: Box<InputFile>) -> &mut Self {
    self.inner.td_origin_mut()._set_sticker(sticker);
    self
  }
  
}


///  Removes a sticker from the set to which it belongs; for bots only. The sticker set must have been created by the bot. 
#[derive(Debug, Clone)]
pub struct TGRemoveStickerFromSet {
  inner: RemoveStickerFromSet
}

impl TDFB for TGRemoveStickerFromSet {}

impl AsRef<TGRemoveStickerFromSet> for TGRemoveStickerFromSet {
  fn as_ref(&self) -> &TGRemoveStickerFromSet { self }
}

impl AsRef<TGRemoveStickerFromSet> for _TGRemoveStickerFromSetBuilder {
  fn as_ref(&self) -> &TGRemoveStickerFromSet { &self.inner }
}

impl TGRemoveStickerFromSet {

  pub fn builder() -> _TGRemoveStickerFromSetBuilder {
    _TGRemoveStickerFromSetBuilder { inner: Self::new(RemoveStickerFromSet::_new()) }
  }

  pub fn new(inner: RemoveStickerFromSet) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &RemoveStickerFromSet { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut RemoveStickerFromSet { &mut self.inner }

}


#[doc(hidden)] pub struct _TGRemoveTopChatBuilder { inner: TGRemoveTopChat }

impl _TGRemoveTopChatBuilder {

  pub fn build(&self) -> TGRemoveTopChat { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  

  
  // [category] type is [Box<TopChatCategory>], is not support, need add manully.
  #[doc(hidden)] pub fn _category(&mut self, category: Box<TopChatCategory>) -> &mut Self {
    self.inner.td_origin_mut()._set_category(category);
    self
  }
  
}


///  Removes a chat from the list of frequently used chats. Supported only if the chat info database is enabled. 
#[derive(Debug, Clone)]
pub struct TGRemoveTopChat {
  inner: RemoveTopChat
}

impl TDFB for TGRemoveTopChat {}

impl AsRef<TGRemoveTopChat> for TGRemoveTopChat {
  fn as_ref(&self) -> &TGRemoveTopChat { self }
}

impl AsRef<TGRemoveTopChat> for _TGRemoveTopChatBuilder {
  fn as_ref(&self) -> &TGRemoveTopChat { &self.inner }
}

impl TGRemoveTopChat {

  pub fn builder() -> _TGRemoveTopChatBuilder {
    _TGRemoveTopChatBuilder { inner: Self::new(RemoveTopChat::_new()) }
  }

  pub fn new(inner: RemoveTopChat) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &RemoveTopChat { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut RemoveTopChat { &mut self.inner }

}


#[doc(hidden)] pub struct _TGReorderInstalledStickerSetsBuilder { inner: TGReorderInstalledStickerSets }

impl _TGReorderInstalledStickerSetsBuilder {

  pub fn build(&self) -> TGReorderInstalledStickerSets { self.inner.clone() }

  ///  Pass true to change the order of mask sticker sets; pass false to change the order of ordinary sticker sets. 
  pub fn is_masks(&mut self, is_masks: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_is_masks(is_masks);
    self
  }
  ///  Identifiers of installed sticker sets in the new correct order. 
  pub fn sticker_set_ids(&mut self, sticker_set_ids: Vec<i64>) -> &mut Self {
    self.inner.td_origin_mut()._set_sticker_set_ids(sticker_set_ids);
    self
  }
  

  
}


///  Changes the order of installed sticker sets. 
#[derive(Debug, Clone)]
pub struct TGReorderInstalledStickerSets {
  inner: ReorderInstalledStickerSets
}

impl TDFB for TGReorderInstalledStickerSets {}

impl AsRef<TGReorderInstalledStickerSets> for TGReorderInstalledStickerSets {
  fn as_ref(&self) -> &TGReorderInstalledStickerSets { self }
}

impl AsRef<TGReorderInstalledStickerSets> for _TGReorderInstalledStickerSetsBuilder {
  fn as_ref(&self) -> &TGReorderInstalledStickerSets { &self.inner }
}

impl TGReorderInstalledStickerSets {

  pub fn builder() -> _TGReorderInstalledStickerSetsBuilder {
    _TGReorderInstalledStickerSetsBuilder { inner: Self::new(ReorderInstalledStickerSets::_new()) }
  }

  pub fn new(inner: ReorderInstalledStickerSets) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ReorderInstalledStickerSets { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ReorderInstalledStickerSets { &mut self.inner }

}


#[doc(hidden)] pub struct _TGReportChatBuilder { inner: TGReportChat }

impl _TGReportChatBuilder {

  pub fn build(&self) -> TGReportChat { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Identifiers of reported messages, if any. 
  pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self {
    self.inner.td_origin_mut()._set_message_ids(message_ids);
    self
  }
  

  
  // [reason] type is [Box<ChatReportReason>], is not support, need add manully.
  #[doc(hidden)] pub fn _reason(&mut self, reason: Box<ChatReportReason>) -> &mut Self {
    self.inner.td_origin_mut()._set_reason(reason);
    self
  }
  
}


///  Reports a chat to the Telegram moderators. Supported only for supergroups, channels, or private chats with bots, since other chats can't be checked by moderators. 
#[derive(Debug, Clone)]
pub struct TGReportChat {
  inner: ReportChat
}

impl TDFB for TGReportChat {}

impl AsRef<TGReportChat> for TGReportChat {
  fn as_ref(&self) -> &TGReportChat { self }
}

impl AsRef<TGReportChat> for _TGReportChatBuilder {
  fn as_ref(&self) -> &TGReportChat { &self.inner }
}

impl TGReportChat {

  pub fn builder() -> _TGReportChatBuilder {
    _TGReportChatBuilder { inner: Self::new(ReportChat::_new()) }
  }

  pub fn new(inner: ReportChat) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ReportChat { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ReportChat { &mut self.inner }

}


#[doc(hidden)] pub struct _TGReportSupergroupSpamBuilder { inner: TGReportSupergroupSpam }

impl _TGReportSupergroupSpamBuilder {

  pub fn build(&self) -> TGReportSupergroupSpam { self.inner.clone() }

  ///  Supergroup identifier. 
  pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_supergroup_id(supergroup_id);
    self
  }
  ///  User identifier. 
  pub fn user_id(&mut self, user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_user_id(user_id);
    self
  }
  ///  Identifiers of messages sent in the supergroup by the user. This list must be non-empty. 
  pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self {
    self.inner.td_origin_mut()._set_message_ids(message_ids);
    self
  }
  

  
}


///  Reports some messages from a user in a supergroup as spam; requires administrator rights in the supergroup. 
#[derive(Debug, Clone)]
pub struct TGReportSupergroupSpam {
  inner: ReportSupergroupSpam
}

impl TDFB for TGReportSupergroupSpam {}

impl AsRef<TGReportSupergroupSpam> for TGReportSupergroupSpam {
  fn as_ref(&self) -> &TGReportSupergroupSpam { self }
}

impl AsRef<TGReportSupergroupSpam> for _TGReportSupergroupSpamBuilder {
  fn as_ref(&self) -> &TGReportSupergroupSpam { &self.inner }
}

impl TGReportSupergroupSpam {

  pub fn builder() -> _TGReportSupergroupSpamBuilder {
    _TGReportSupergroupSpamBuilder { inner: Self::new(ReportSupergroupSpam::_new()) }
  }

  pub fn new(inner: ReportSupergroupSpam) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ReportSupergroupSpam { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ReportSupergroupSpam { &mut self.inner }

}


#[doc(hidden)] pub struct _TGRequestAuthenticationPasswordRecoveryBuilder { inner: TGRequestAuthenticationPasswordRecovery }

impl _TGRequestAuthenticationPasswordRecoveryBuilder {

  pub fn build(&self) -> TGRequestAuthenticationPasswordRecovery { self.inner.clone() }

  

  
}


///  Requests to send a password recovery code to an email address that was previously set up. Works only when the current authorization state is  
#[derive(Debug, Clone)]
pub struct TGRequestAuthenticationPasswordRecovery {
  inner: RequestAuthenticationPasswordRecovery
}

impl TDFB for TGRequestAuthenticationPasswordRecovery {}

impl AsRef<TGRequestAuthenticationPasswordRecovery> for TGRequestAuthenticationPasswordRecovery {
  fn as_ref(&self) -> &TGRequestAuthenticationPasswordRecovery { self }
}

impl AsRef<TGRequestAuthenticationPasswordRecovery> for _TGRequestAuthenticationPasswordRecoveryBuilder {
  fn as_ref(&self) -> &TGRequestAuthenticationPasswordRecovery { &self.inner }
}

impl TGRequestAuthenticationPasswordRecovery {

  pub fn builder() -> _TGRequestAuthenticationPasswordRecoveryBuilder {
    _TGRequestAuthenticationPasswordRecoveryBuilder { inner: Self::new(RequestAuthenticationPasswordRecovery::_new()) }
  }

  pub fn new(inner: RequestAuthenticationPasswordRecovery) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &RequestAuthenticationPasswordRecovery { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut RequestAuthenticationPasswordRecovery { &mut self.inner }

}


#[doc(hidden)] pub struct _TGRequestPasswordRecoveryBuilder { inner: TGRequestPasswordRecovery }

impl _TGRequestPasswordRecoveryBuilder {

  pub fn build(&self) -> TGRequestPasswordRecovery { self.inner.clone() }

  

  
}


///  Requests to send a password recovery code to an email address that was previously set up. 
#[derive(Debug, Clone)]
pub struct TGRequestPasswordRecovery {
  inner: RequestPasswordRecovery
}

impl TDFB for TGRequestPasswordRecovery {}

impl AsRef<TGRequestPasswordRecovery> for TGRequestPasswordRecovery {
  fn as_ref(&self) -> &TGRequestPasswordRecovery { self }
}

impl AsRef<TGRequestPasswordRecovery> for _TGRequestPasswordRecoveryBuilder {
  fn as_ref(&self) -> &TGRequestPasswordRecovery { &self.inner }
}

impl TGRequestPasswordRecovery {

  pub fn builder() -> _TGRequestPasswordRecoveryBuilder {
    _TGRequestPasswordRecoveryBuilder { inner: Self::new(RequestPasswordRecovery::_new()) }
  }

  pub fn new(inner: RequestPasswordRecovery) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &RequestPasswordRecovery { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut RequestPasswordRecovery { &mut self.inner }

}


#[doc(hidden)] pub struct _TGResendAuthenticationCodeBuilder { inner: TGResendAuthenticationCode }

impl _TGResendAuthenticationCodeBuilder {

  pub fn build(&self) -> TGResendAuthenticationCode { self.inner.clone() }

  

  
}


///  Re-sends an authentication code to the user. Works only when the current authorization state is  
#[derive(Debug, Clone)]
pub struct TGResendAuthenticationCode {
  inner: ResendAuthenticationCode
}

impl TDFB for TGResendAuthenticationCode {}

impl AsRef<TGResendAuthenticationCode> for TGResendAuthenticationCode {
  fn as_ref(&self) -> &TGResendAuthenticationCode { self }
}

impl AsRef<TGResendAuthenticationCode> for _TGResendAuthenticationCodeBuilder {
  fn as_ref(&self) -> &TGResendAuthenticationCode { &self.inner }
}

impl TGResendAuthenticationCode {

  pub fn builder() -> _TGResendAuthenticationCodeBuilder {
    _TGResendAuthenticationCodeBuilder { inner: Self::new(ResendAuthenticationCode::_new()) }
  }

  pub fn new(inner: ResendAuthenticationCode) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ResendAuthenticationCode { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ResendAuthenticationCode { &mut self.inner }

}


#[doc(hidden)] pub struct _TGResendChangePhoneNumberCodeBuilder { inner: TGResendChangePhoneNumberCode }

impl _TGResendChangePhoneNumberCodeBuilder {

  pub fn build(&self) -> TGResendChangePhoneNumberCode { self.inner.clone() }

  

  
}


///  Re-sends the authentication code sent to confirm a new phone number for the user. Works only if the previously received  
#[derive(Debug, Clone)]
pub struct TGResendChangePhoneNumberCode {
  inner: ResendChangePhoneNumberCode
}

impl TDFB for TGResendChangePhoneNumberCode {}

impl AsRef<TGResendChangePhoneNumberCode> for TGResendChangePhoneNumberCode {
  fn as_ref(&self) -> &TGResendChangePhoneNumberCode { self }
}

impl AsRef<TGResendChangePhoneNumberCode> for _TGResendChangePhoneNumberCodeBuilder {
  fn as_ref(&self) -> &TGResendChangePhoneNumberCode { &self.inner }
}

impl TGResendChangePhoneNumberCode {

  pub fn builder() -> _TGResendChangePhoneNumberCodeBuilder {
    _TGResendChangePhoneNumberCodeBuilder { inner: Self::new(ResendChangePhoneNumberCode::_new()) }
  }

  pub fn new(inner: ResendChangePhoneNumberCode) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ResendChangePhoneNumberCode { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ResendChangePhoneNumberCode { &mut self.inner }

}


#[doc(hidden)] pub struct _TGResendEmailAddressVerificationCodeBuilder { inner: TGResendEmailAddressVerificationCode }

impl _TGResendEmailAddressVerificationCodeBuilder {

  pub fn build(&self) -> TGResendEmailAddressVerificationCode { self.inner.clone() }

  

  
}


///  Re-sends the code to verify an email address to be added to a user's Telegram Passport. 
#[derive(Debug, Clone)]
pub struct TGResendEmailAddressVerificationCode {
  inner: ResendEmailAddressVerificationCode
}

impl TDFB for TGResendEmailAddressVerificationCode {}

impl AsRef<TGResendEmailAddressVerificationCode> for TGResendEmailAddressVerificationCode {
  fn as_ref(&self) -> &TGResendEmailAddressVerificationCode { self }
}

impl AsRef<TGResendEmailAddressVerificationCode> for _TGResendEmailAddressVerificationCodeBuilder {
  fn as_ref(&self) -> &TGResendEmailAddressVerificationCode { &self.inner }
}

impl TGResendEmailAddressVerificationCode {

  pub fn builder() -> _TGResendEmailAddressVerificationCodeBuilder {
    _TGResendEmailAddressVerificationCodeBuilder { inner: Self::new(ResendEmailAddressVerificationCode::_new()) }
  }

  pub fn new(inner: ResendEmailAddressVerificationCode) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ResendEmailAddressVerificationCode { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ResendEmailAddressVerificationCode { &mut self.inner }

}


#[doc(hidden)] pub struct _TGResendPhoneNumberConfirmationCodeBuilder { inner: TGResendPhoneNumberConfirmationCode }

impl _TGResendPhoneNumberConfirmationCodeBuilder {

  pub fn build(&self) -> TGResendPhoneNumberConfirmationCode { self.inner.clone() }

  

  
}


///  Resends phone number confirmation code. 
#[derive(Debug, Clone)]
pub struct TGResendPhoneNumberConfirmationCode {
  inner: ResendPhoneNumberConfirmationCode
}

impl TDFB for TGResendPhoneNumberConfirmationCode {}

impl AsRef<TGResendPhoneNumberConfirmationCode> for TGResendPhoneNumberConfirmationCode {
  fn as_ref(&self) -> &TGResendPhoneNumberConfirmationCode { self }
}

impl AsRef<TGResendPhoneNumberConfirmationCode> for _TGResendPhoneNumberConfirmationCodeBuilder {
  fn as_ref(&self) -> &TGResendPhoneNumberConfirmationCode { &self.inner }
}

impl TGResendPhoneNumberConfirmationCode {

  pub fn builder() -> _TGResendPhoneNumberConfirmationCodeBuilder {
    _TGResendPhoneNumberConfirmationCodeBuilder { inner: Self::new(ResendPhoneNumberConfirmationCode::_new()) }
  }

  pub fn new(inner: ResendPhoneNumberConfirmationCode) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ResendPhoneNumberConfirmationCode { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ResendPhoneNumberConfirmationCode { &mut self.inner }

}


#[doc(hidden)] pub struct _TGResendPhoneNumberVerificationCodeBuilder { inner: TGResendPhoneNumberVerificationCode }

impl _TGResendPhoneNumberVerificationCodeBuilder {

  pub fn build(&self) -> TGResendPhoneNumberVerificationCode { self.inner.clone() }

  

  
}


///  Re-sends the code to verify a phone number to be added to a user's Telegram Passport. 
#[derive(Debug, Clone)]
pub struct TGResendPhoneNumberVerificationCode {
  inner: ResendPhoneNumberVerificationCode
}

impl TDFB for TGResendPhoneNumberVerificationCode {}

impl AsRef<TGResendPhoneNumberVerificationCode> for TGResendPhoneNumberVerificationCode {
  fn as_ref(&self) -> &TGResendPhoneNumberVerificationCode { self }
}

impl AsRef<TGResendPhoneNumberVerificationCode> for _TGResendPhoneNumberVerificationCodeBuilder {
  fn as_ref(&self) -> &TGResendPhoneNumberVerificationCode { &self.inner }
}

impl TGResendPhoneNumberVerificationCode {

  pub fn builder() -> _TGResendPhoneNumberVerificationCodeBuilder {
    _TGResendPhoneNumberVerificationCodeBuilder { inner: Self::new(ResendPhoneNumberVerificationCode::_new()) }
  }

  pub fn new(inner: ResendPhoneNumberVerificationCode) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ResendPhoneNumberVerificationCode { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ResendPhoneNumberVerificationCode { &mut self.inner }

}


#[doc(hidden)] pub struct _TGResendRecoveryEmailAddressCodeBuilder { inner: TGResendRecoveryEmailAddressCode }

impl _TGResendRecoveryEmailAddressCodeBuilder {

  pub fn build(&self) -> TGResendRecoveryEmailAddressCode { self.inner.clone() }

  

  
}


///  Resends the 2-step verification recovery email address verification code. 
#[derive(Debug, Clone)]
pub struct TGResendRecoveryEmailAddressCode {
  inner: ResendRecoveryEmailAddressCode
}

impl TDFB for TGResendRecoveryEmailAddressCode {}

impl AsRef<TGResendRecoveryEmailAddressCode> for TGResendRecoveryEmailAddressCode {
  fn as_ref(&self) -> &TGResendRecoveryEmailAddressCode { self }
}

impl AsRef<TGResendRecoveryEmailAddressCode> for _TGResendRecoveryEmailAddressCodeBuilder {
  fn as_ref(&self) -> &TGResendRecoveryEmailAddressCode { &self.inner }
}

impl TGResendRecoveryEmailAddressCode {

  pub fn builder() -> _TGResendRecoveryEmailAddressCodeBuilder {
    _TGResendRecoveryEmailAddressCodeBuilder { inner: Self::new(ResendRecoveryEmailAddressCode::_new()) }
  }

  pub fn new(inner: ResendRecoveryEmailAddressCode) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ResendRecoveryEmailAddressCode { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ResendRecoveryEmailAddressCode { &mut self.inner }

}


#[doc(hidden)] pub struct _TGResetAllNotificationSettingsBuilder { inner: TGResetAllNotificationSettings }

impl _TGResetAllNotificationSettingsBuilder {

  pub fn build(&self) -> TGResetAllNotificationSettings { self.inner.clone() }

  

  
}


///  Resets all notification settings to their default values. By default, all chats are unmuted, the sound is set to "default" and message previews are shown. 
#[derive(Debug, Clone)]
pub struct TGResetAllNotificationSettings {
  inner: ResetAllNotificationSettings
}

impl TDFB for TGResetAllNotificationSettings {}

impl AsRef<TGResetAllNotificationSettings> for TGResetAllNotificationSettings {
  fn as_ref(&self) -> &TGResetAllNotificationSettings { self }
}

impl AsRef<TGResetAllNotificationSettings> for _TGResetAllNotificationSettingsBuilder {
  fn as_ref(&self) -> &TGResetAllNotificationSettings { &self.inner }
}

impl TGResetAllNotificationSettings {

  pub fn builder() -> _TGResetAllNotificationSettingsBuilder {
    _TGResetAllNotificationSettingsBuilder { inner: Self::new(ResetAllNotificationSettings::_new()) }
  }

  pub fn new(inner: ResetAllNotificationSettings) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ResetAllNotificationSettings { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ResetAllNotificationSettings { &mut self.inner }

}


#[doc(hidden)] pub struct _TGResetNetworkStatisticsBuilder { inner: TGResetNetworkStatistics }

impl _TGResetNetworkStatisticsBuilder {

  pub fn build(&self) -> TGResetNetworkStatistics { self.inner.clone() }

  

  
}


///  Resets all network data usage statistics to zero. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGResetNetworkStatistics {
  inner: ResetNetworkStatistics
}

impl TDFB for TGResetNetworkStatistics {}

impl AsRef<TGResetNetworkStatistics> for TGResetNetworkStatistics {
  fn as_ref(&self) -> &TGResetNetworkStatistics { self }
}

impl AsRef<TGResetNetworkStatistics> for _TGResetNetworkStatisticsBuilder {
  fn as_ref(&self) -> &TGResetNetworkStatistics { &self.inner }
}

impl TGResetNetworkStatistics {

  pub fn builder() -> _TGResetNetworkStatisticsBuilder {
    _TGResetNetworkStatisticsBuilder { inner: Self::new(ResetNetworkStatistics::_new()) }
  }

  pub fn new(inner: ResetNetworkStatistics) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ResetNetworkStatistics { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ResetNetworkStatistics { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSearchCallMessagesBuilder { inner: TGSearchCallMessages }

impl _TGSearchCallMessagesBuilder {

  pub fn build(&self) -> TGSearchCallMessages { self.inner.clone() }

  ///  Identifier of the message from which to search; use 0 to get results from the last message. 
  pub fn from_message_id(&mut self, from_message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_from_message_id(from_message_id);
    self
  }
  ///  The maximum number of messages to be returned; up to 100. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached. 
  pub fn limit(&mut self, limit: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_limit(limit);
    self
  }
  ///  If true, returns only messages with missed calls. 
  pub fn only_missed(&mut self, only_missed: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_only_missed(only_missed);
    self
  }
  

  
}


///  Searches for call messages. Returns the results in reverse chronological order (i. e., in order of decreasing message_id). For optimal performance the number of returned messages is chosen by the library. 
#[derive(Debug, Clone)]
pub struct TGSearchCallMessages {
  inner: SearchCallMessages
}

impl TDFB for TGSearchCallMessages {}

impl AsRef<TGSearchCallMessages> for TGSearchCallMessages {
  fn as_ref(&self) -> &TGSearchCallMessages { self }
}

impl AsRef<TGSearchCallMessages> for _TGSearchCallMessagesBuilder {
  fn as_ref(&self) -> &TGSearchCallMessages { &self.inner }
}

impl TGSearchCallMessages {

  pub fn builder() -> _TGSearchCallMessagesBuilder {
    _TGSearchCallMessagesBuilder { inner: Self::new(SearchCallMessages::_new()) }
  }

  pub fn new(inner: SearchCallMessages) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SearchCallMessages { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SearchCallMessages { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSearchChatMembersBuilder { inner: TGSearchChatMembers }

impl _TGSearchChatMembersBuilder {

  pub fn build(&self) -> TGSearchChatMembers { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Query to search for. 
  pub fn query<S: AsRef<str>>(&mut self, query: S) -> &mut Self {
    self.inner.td_origin_mut()._set_query(query.as_ref().to_string());
    self
  }
  ///  The maximum number of users to be returned. 
  pub fn limit(&mut self, limit: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_limit(limit);
    self
  }
  

  
  // [filter] type is [Box<ChatMembersFilter>], is not support, need add manully.
  #[doc(hidden)] pub fn _filter(&mut self, filter: Box<ChatMembersFilter>) -> &mut Self {
    self.inner.td_origin_mut()._set_filter(filter);
    self
  }
  
}


///  Searches for a specified query in the first name, last name and username of the members of a specified chat. Requires administrator rights in channels. 
#[derive(Debug, Clone)]
pub struct TGSearchChatMembers {
  inner: SearchChatMembers
}

impl TDFB for TGSearchChatMembers {}

impl AsRef<TGSearchChatMembers> for TGSearchChatMembers {
  fn as_ref(&self) -> &TGSearchChatMembers { self }
}

impl AsRef<TGSearchChatMembers> for _TGSearchChatMembersBuilder {
  fn as_ref(&self) -> &TGSearchChatMembers { &self.inner }
}

impl TGSearchChatMembers {

  pub fn builder() -> _TGSearchChatMembersBuilder {
    _TGSearchChatMembersBuilder { inner: Self::new(SearchChatMembers::_new()) }
  }

  pub fn new(inner: SearchChatMembers) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SearchChatMembers { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SearchChatMembers { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSearchChatMessagesBuilder { inner: TGSearchChatMessages }

impl _TGSearchChatMessagesBuilder {

  pub fn build(&self) -> TGSearchChatMessages { self.inner.clone() }

  ///  Identifier of the chat in which to search messages. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Query to search for. 
  pub fn query<S: AsRef<str>>(&mut self, query: S) -> &mut Self {
    self.inner.td_origin_mut()._set_query(query.as_ref().to_string());
    self
  }
  ///  If not 0, only messages sent by the specified user will be returned. Not supported in secret chats. 
  pub fn sender_user_id(&mut self, sender_user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_sender_user_id(sender_user_id);
    self
  }
  ///  Identifier of the message starting from which history must be fetched; use 0 to get results from the last message. 
  pub fn from_message_id(&mut self, from_message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_from_message_id(from_message_id);
    self
  }
  ///  Specify 0 to get results from exactly the from_message_id or a negative offset to get the specified message and some newer messages. 
  pub fn offset(&mut self, offset: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_offset(offset);
    self
  }
  ///  The maximum number of messages to be returned; must be positive and can't be greater than 100. If the offset is negative, the limit must be greater than -offset. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached. 
  pub fn limit(&mut self, limit: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_limit(limit);
    self
  }
  

  
  // [filter] type is [Box<SearchMessagesFilter>], is not support, need add manully.
  #[doc(hidden)] pub fn _filter(&mut self, filter: Box<SearchMessagesFilter>) -> &mut Self {
    self.inner.td_origin_mut()._set_filter(filter);
    self
  }
  
}


///  Searches for messages with given words in the chat. Returns the results in reverse chronological order, i.e. in order of decreasing message_id. Cannot be used in secret chats with a non-empty query ( 
#[derive(Debug, Clone)]
pub struct TGSearchChatMessages {
  inner: SearchChatMessages
}

impl TDFB for TGSearchChatMessages {}

impl AsRef<TGSearchChatMessages> for TGSearchChatMessages {
  fn as_ref(&self) -> &TGSearchChatMessages { self }
}

impl AsRef<TGSearchChatMessages> for _TGSearchChatMessagesBuilder {
  fn as_ref(&self) -> &TGSearchChatMessages { &self.inner }
}

impl TGSearchChatMessages {

  pub fn builder() -> _TGSearchChatMessagesBuilder {
    _TGSearchChatMessagesBuilder { inner: Self::new(SearchChatMessages::_new()) }
  }

  pub fn new(inner: SearchChatMessages) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SearchChatMessages { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SearchChatMessages { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSearchChatRecentLocationMessagesBuilder { inner: TGSearchChatRecentLocationMessages }

impl _TGSearchChatRecentLocationMessagesBuilder {

  pub fn build(&self) -> TGSearchChatRecentLocationMessages { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Maximum number of messages to be returned. 
  pub fn limit(&mut self, limit: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_limit(limit);
    self
  }
  

  
}


///  Returns information about the recent locations of chat members that were sent to the chat. Returns up to 1 location message per user. 
#[derive(Debug, Clone)]
pub struct TGSearchChatRecentLocationMessages {
  inner: SearchChatRecentLocationMessages
}

impl TDFB for TGSearchChatRecentLocationMessages {}

impl AsRef<TGSearchChatRecentLocationMessages> for TGSearchChatRecentLocationMessages {
  fn as_ref(&self) -> &TGSearchChatRecentLocationMessages { self }
}

impl AsRef<TGSearchChatRecentLocationMessages> for _TGSearchChatRecentLocationMessagesBuilder {
  fn as_ref(&self) -> &TGSearchChatRecentLocationMessages { &self.inner }
}

impl TGSearchChatRecentLocationMessages {

  pub fn builder() -> _TGSearchChatRecentLocationMessagesBuilder {
    _TGSearchChatRecentLocationMessagesBuilder { inner: Self::new(SearchChatRecentLocationMessages::_new()) }
  }

  pub fn new(inner: SearchChatRecentLocationMessages) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SearchChatRecentLocationMessages { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SearchChatRecentLocationMessages { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSearchChatsBuilder { inner: TGSearchChats }

impl _TGSearchChatsBuilder {

  pub fn build(&self) -> TGSearchChats { self.inner.clone() }

  ///  Query to search for. If the query is empty, returns up to 20 recently found chats. 
  pub fn query<S: AsRef<str>>(&mut self, query: S) -> &mut Self {
    self.inner.td_origin_mut()._set_query(query.as_ref().to_string());
    self
  }
  ///  Maximum number of chats to be returned. 
  pub fn limit(&mut self, limit: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_limit(limit);
    self
  }
  

  
}


///  Searches for the specified query in the title and username of already known chats, this is an offline request. Returns chats in the order seen in the chat list. 
#[derive(Debug, Clone)]
pub struct TGSearchChats {
  inner: SearchChats
}

impl TDFB for TGSearchChats {}

impl AsRef<TGSearchChats> for TGSearchChats {
  fn as_ref(&self) -> &TGSearchChats { self }
}

impl AsRef<TGSearchChats> for _TGSearchChatsBuilder {
  fn as_ref(&self) -> &TGSearchChats { &self.inner }
}

impl TGSearchChats {

  pub fn builder() -> _TGSearchChatsBuilder {
    _TGSearchChatsBuilder { inner: Self::new(SearchChats::_new()) }
  }

  pub fn new(inner: SearchChats) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SearchChats { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SearchChats { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSearchChatsOnServerBuilder { inner: TGSearchChatsOnServer }

impl _TGSearchChatsOnServerBuilder {

  pub fn build(&self) -> TGSearchChatsOnServer { self.inner.clone() }

  ///  Query to search for. 
  pub fn query<S: AsRef<str>>(&mut self, query: S) -> &mut Self {
    self.inner.td_origin_mut()._set_query(query.as_ref().to_string());
    self
  }
  ///  Maximum number of chats to be returned. 
  pub fn limit(&mut self, limit: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_limit(limit);
    self
  }
  

  
}


///  Searches for the specified query in the title and username of already known chats via request to the server. Returns chats in the order seen in the chat list. 
#[derive(Debug, Clone)]
pub struct TGSearchChatsOnServer {
  inner: SearchChatsOnServer
}

impl TDFB for TGSearchChatsOnServer {}

impl AsRef<TGSearchChatsOnServer> for TGSearchChatsOnServer {
  fn as_ref(&self) -> &TGSearchChatsOnServer { self }
}

impl AsRef<TGSearchChatsOnServer> for _TGSearchChatsOnServerBuilder {
  fn as_ref(&self) -> &TGSearchChatsOnServer { &self.inner }
}

impl TGSearchChatsOnServer {

  pub fn builder() -> _TGSearchChatsOnServerBuilder {
    _TGSearchChatsOnServerBuilder { inner: Self::new(SearchChatsOnServer::_new()) }
  }

  pub fn new(inner: SearchChatsOnServer) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SearchChatsOnServer { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SearchChatsOnServer { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSearchContactsBuilder { inner: TGSearchContacts }

impl _TGSearchContactsBuilder {

  pub fn build(&self) -> TGSearchContacts { self.inner.clone() }

  ///  Query to search for; may be empty to return all contacts. 
  pub fn query<S: AsRef<str>>(&mut self, query: S) -> &mut Self {
    self.inner.td_origin_mut()._set_query(query.as_ref().to_string());
    self
  }
  ///  Maximum number of users to be returned. 
  pub fn limit(&mut self, limit: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_limit(limit);
    self
  }
  

  
}


///  Searches for the specified query in the first names, last names and usernames of the known user contacts. 
#[derive(Debug, Clone)]
pub struct TGSearchContacts {
  inner: SearchContacts
}

impl TDFB for TGSearchContacts {}

impl AsRef<TGSearchContacts> for TGSearchContacts {
  fn as_ref(&self) -> &TGSearchContacts { self }
}

impl AsRef<TGSearchContacts> for _TGSearchContactsBuilder {
  fn as_ref(&self) -> &TGSearchContacts { &self.inner }
}

impl TGSearchContacts {

  pub fn builder() -> _TGSearchContactsBuilder {
    _TGSearchContactsBuilder { inner: Self::new(SearchContacts::_new()) }
  }

  pub fn new(inner: SearchContacts) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SearchContacts { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SearchContacts { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSearchHashtagsBuilder { inner: TGSearchHashtags }

impl _TGSearchHashtagsBuilder {

  pub fn build(&self) -> TGSearchHashtags { self.inner.clone() }

  ///  Hashtag prefix to search for. 
  pub fn prefix<S: AsRef<str>>(&mut self, prefix: S) -> &mut Self {
    self.inner.td_origin_mut()._set_prefix(prefix.as_ref().to_string());
    self
  }
  ///  Maximum number of hashtags to be returned. 
  pub fn limit(&mut self, limit: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_limit(limit);
    self
  }
  

  
}


///  Searches for recently used hashtags by their prefix. 
#[derive(Debug, Clone)]
pub struct TGSearchHashtags {
  inner: SearchHashtags
}

impl TDFB for TGSearchHashtags {}

impl AsRef<TGSearchHashtags> for TGSearchHashtags {
  fn as_ref(&self) -> &TGSearchHashtags { self }
}

impl AsRef<TGSearchHashtags> for _TGSearchHashtagsBuilder {
  fn as_ref(&self) -> &TGSearchHashtags { &self.inner }
}

impl TGSearchHashtags {

  pub fn builder() -> _TGSearchHashtagsBuilder {
    _TGSearchHashtagsBuilder { inner: Self::new(SearchHashtags::_new()) }
  }

  pub fn new(inner: SearchHashtags) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SearchHashtags { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SearchHashtags { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSearchInstalledStickerSetsBuilder { inner: TGSearchInstalledStickerSets }

impl _TGSearchInstalledStickerSetsBuilder {

  pub fn build(&self) -> TGSearchInstalledStickerSets { self.inner.clone() }

  ///  Pass true to return mask sticker sets; pass false to return ordinary sticker sets. 
  pub fn is_masks(&mut self, is_masks: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_is_masks(is_masks);
    self
  }
  ///  Query to search for. 
  pub fn query<S: AsRef<str>>(&mut self, query: S) -> &mut Self {
    self.inner.td_origin_mut()._set_query(query.as_ref().to_string());
    self
  }
  ///  Maximum number of sticker sets to return. 
  pub fn limit(&mut self, limit: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_limit(limit);
    self
  }
  

  
}


///  Searches for installed sticker sets by looking for specified query in their title and name. 
#[derive(Debug, Clone)]
pub struct TGSearchInstalledStickerSets {
  inner: SearchInstalledStickerSets
}

impl TDFB for TGSearchInstalledStickerSets {}

impl AsRef<TGSearchInstalledStickerSets> for TGSearchInstalledStickerSets {
  fn as_ref(&self) -> &TGSearchInstalledStickerSets { self }
}

impl AsRef<TGSearchInstalledStickerSets> for _TGSearchInstalledStickerSetsBuilder {
  fn as_ref(&self) -> &TGSearchInstalledStickerSets { &self.inner }
}

impl TGSearchInstalledStickerSets {

  pub fn builder() -> _TGSearchInstalledStickerSetsBuilder {
    _TGSearchInstalledStickerSetsBuilder { inner: Self::new(SearchInstalledStickerSets::_new()) }
  }

  pub fn new(inner: SearchInstalledStickerSets) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SearchInstalledStickerSets { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SearchInstalledStickerSets { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSearchMessagesBuilder { inner: TGSearchMessages }

impl _TGSearchMessagesBuilder {

  pub fn build(&self) -> TGSearchMessages { self.inner.clone() }

  ///  Query to search for. 
  pub fn query<S: AsRef<str>>(&mut self, query: S) -> &mut Self {
    self.inner.td_origin_mut()._set_query(query.as_ref().to_string());
    self
  }
  ///  The date of the message starting from which the results should be fetched. Use 0 or any date in the future to get results from the last message. 
  pub fn offset_date(&mut self, offset_date: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_offset_date(offset_date);
    self
  }
  ///  The chat identifier of the last found message, or 0 for the first request. 
  pub fn offset_chat_id(&mut self, offset_chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_offset_chat_id(offset_chat_id);
    self
  }
  ///  The message identifier of the last found message, or 0 for the first request. 
  pub fn offset_message_id(&mut self, offset_message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_offset_message_id(offset_message_id);
    self
  }
  ///  The maximum number of messages to be returned, up to 100. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached. 
  pub fn limit(&mut self, limit: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_limit(limit);
    self
  }
  

  
}


///  Searches for messages in all chats except secret chats. Returns the results in reverse chronological order (i.e., in order of decreasing (date, chat_id, message_id)). For optimal performance the number of returned messages is chosen by the library. 
#[derive(Debug, Clone)]
pub struct TGSearchMessages {
  inner: SearchMessages
}

impl TDFB for TGSearchMessages {}

impl AsRef<TGSearchMessages> for TGSearchMessages {
  fn as_ref(&self) -> &TGSearchMessages { self }
}

impl AsRef<TGSearchMessages> for _TGSearchMessagesBuilder {
  fn as_ref(&self) -> &TGSearchMessages { &self.inner }
}

impl TGSearchMessages {

  pub fn builder() -> _TGSearchMessagesBuilder {
    _TGSearchMessagesBuilder { inner: Self::new(SearchMessages::_new()) }
  }

  pub fn new(inner: SearchMessages) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SearchMessages { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SearchMessages { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSearchPublicChatBuilder { inner: TGSearchPublicChat }

impl _TGSearchPublicChatBuilder {

  pub fn build(&self) -> TGSearchPublicChat { self.inner.clone() }

  ///  Username to be resolved. 
  pub fn username<S: AsRef<str>>(&mut self, username: S) -> &mut Self {
    self.inner.td_origin_mut()._set_username(username.as_ref().to_string());
    self
  }
  

  
}


///  Searches a public chat by its username. Currently only private chats, supergroups and channels can be public. Returns the chat if found; otherwise an error is returned. 
#[derive(Debug, Clone)]
pub struct TGSearchPublicChat {
  inner: SearchPublicChat
}

impl TDFB for TGSearchPublicChat {}

impl AsRef<TGSearchPublicChat> for TGSearchPublicChat {
  fn as_ref(&self) -> &TGSearchPublicChat { self }
}

impl AsRef<TGSearchPublicChat> for _TGSearchPublicChatBuilder {
  fn as_ref(&self) -> &TGSearchPublicChat { &self.inner }
}

impl TGSearchPublicChat {

  pub fn builder() -> _TGSearchPublicChatBuilder {
    _TGSearchPublicChatBuilder { inner: Self::new(SearchPublicChat::_new()) }
  }

  pub fn new(inner: SearchPublicChat) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SearchPublicChat { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SearchPublicChat { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSearchPublicChatsBuilder { inner: TGSearchPublicChats }

impl _TGSearchPublicChatsBuilder {

  pub fn build(&self) -> TGSearchPublicChats { self.inner.clone() }

  ///  Query to search for. 
  pub fn query<S: AsRef<str>>(&mut self, query: S) -> &mut Self {
    self.inner.td_origin_mut()._set_query(query.as_ref().to_string());
    self
  }
  

  
}


///  Searches public chats by looking for specified query in their username and title. Currently only private chats, supergroups and channels can be public. Returns a meaningful number of results. Returns nothing if the length of the searched username prefix is less than 5. Excludes private chats with contacts and chats from the chat list from the results. 
#[derive(Debug, Clone)]
pub struct TGSearchPublicChats {
  inner: SearchPublicChats
}

impl TDFB for TGSearchPublicChats {}

impl AsRef<TGSearchPublicChats> for TGSearchPublicChats {
  fn as_ref(&self) -> &TGSearchPublicChats { self }
}

impl AsRef<TGSearchPublicChats> for _TGSearchPublicChatsBuilder {
  fn as_ref(&self) -> &TGSearchPublicChats { &self.inner }
}

impl TGSearchPublicChats {

  pub fn builder() -> _TGSearchPublicChatsBuilder {
    _TGSearchPublicChatsBuilder { inner: Self::new(SearchPublicChats::_new()) }
  }

  pub fn new(inner: SearchPublicChats) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SearchPublicChats { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SearchPublicChats { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSearchSecretMessagesBuilder { inner: TGSearchSecretMessages }

impl _TGSearchSecretMessagesBuilder {

  pub fn build(&self) -> TGSearchSecretMessages { self.inner.clone() }

  ///  Identifier of the chat in which to search. Specify 0 to search in all secret chats. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Query to search for. If empty, searchChatMessages should be used instead. 
  pub fn query<S: AsRef<str>>(&mut self, query: S) -> &mut Self {
    self.inner.td_origin_mut()._set_query(query.as_ref().to_string());
    self
  }
  ///  The identifier from the result of a previous request, use 0 to get results from the last message. 
  pub fn from_search_id(&mut self, from_search_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_from_search_id(from_search_id);
    self
  }
  ///  Maximum number of messages to be returned; up to 100. Fewer messages may be returned than specified by the limit, even if the end of the message history has not been reached. 
  pub fn limit(&mut self, limit: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_limit(limit);
    self
  }
  

  
  // [filter] type is [Box<SearchMessagesFilter>], is not support, need add manully.
  #[doc(hidden)] pub fn _filter(&mut self, filter: Box<SearchMessagesFilter>) -> &mut Self {
    self.inner.td_origin_mut()._set_filter(filter);
    self
  }
  
}


///  Searches for messages in secret chats. Returns the results in reverse chronological order. For optimal performance the number of returned messages is chosen by the library. 
#[derive(Debug, Clone)]
pub struct TGSearchSecretMessages {
  inner: SearchSecretMessages
}

impl TDFB for TGSearchSecretMessages {}

impl AsRef<TGSearchSecretMessages> for TGSearchSecretMessages {
  fn as_ref(&self) -> &TGSearchSecretMessages { self }
}

impl AsRef<TGSearchSecretMessages> for _TGSearchSecretMessagesBuilder {
  fn as_ref(&self) -> &TGSearchSecretMessages { &self.inner }
}

impl TGSearchSecretMessages {

  pub fn builder() -> _TGSearchSecretMessagesBuilder {
    _TGSearchSecretMessagesBuilder { inner: Self::new(SearchSecretMessages::_new()) }
  }

  pub fn new(inner: SearchSecretMessages) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SearchSecretMessages { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SearchSecretMessages { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSearchStickersBuilder { inner: TGSearchStickers }

impl _TGSearchStickersBuilder {

  pub fn build(&self) -> TGSearchStickers { self.inner.clone() }

  ///  String representation of emoji; must be non-empty. 
  pub fn emoji<S: AsRef<str>>(&mut self, emoji: S) -> &mut Self {
    self.inner.td_origin_mut()._set_emoji(emoji.as_ref().to_string());
    self
  }
  ///  Maximum number of stickers to be returned. 
  pub fn limit(&mut self, limit: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_limit(limit);
    self
  }
  

  
}


///  Searches for stickers from public sticker sets that correspond to a given emoji. 
#[derive(Debug, Clone)]
pub struct TGSearchStickers {
  inner: SearchStickers
}

impl TDFB for TGSearchStickers {}

impl AsRef<TGSearchStickers> for TGSearchStickers {
  fn as_ref(&self) -> &TGSearchStickers { self }
}

impl AsRef<TGSearchStickers> for _TGSearchStickersBuilder {
  fn as_ref(&self) -> &TGSearchStickers { &self.inner }
}

impl TGSearchStickers {

  pub fn builder() -> _TGSearchStickersBuilder {
    _TGSearchStickersBuilder { inner: Self::new(SearchStickers::_new()) }
  }

  pub fn new(inner: SearchStickers) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SearchStickers { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SearchStickers { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSearchStickerSetBuilder { inner: TGSearchStickerSet }

impl _TGSearchStickerSetBuilder {

  pub fn build(&self) -> TGSearchStickerSet { self.inner.clone() }

  ///  Name of the sticker set. 
  pub fn name<S: AsRef<str>>(&mut self, name: S) -> &mut Self {
    self.inner.td_origin_mut()._set_name(name.as_ref().to_string());
    self
  }
  

  
}


///  Searches for a sticker set by its name. 
#[derive(Debug, Clone)]
pub struct TGSearchStickerSet {
  inner: SearchStickerSet
}

impl TDFB for TGSearchStickerSet {}

impl AsRef<TGSearchStickerSet> for TGSearchStickerSet {
  fn as_ref(&self) -> &TGSearchStickerSet { self }
}

impl AsRef<TGSearchStickerSet> for _TGSearchStickerSetBuilder {
  fn as_ref(&self) -> &TGSearchStickerSet { &self.inner }
}

impl TGSearchStickerSet {

  pub fn builder() -> _TGSearchStickerSetBuilder {
    _TGSearchStickerSetBuilder { inner: Self::new(SearchStickerSet::_new()) }
  }

  pub fn new(inner: SearchStickerSet) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SearchStickerSet { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SearchStickerSet { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSearchStickerSetsBuilder { inner: TGSearchStickerSets }

impl _TGSearchStickerSetsBuilder {

  pub fn build(&self) -> TGSearchStickerSets { self.inner.clone() }

  ///  Query to search for. 
  pub fn query<S: AsRef<str>>(&mut self, query: S) -> &mut Self {
    self.inner.td_origin_mut()._set_query(query.as_ref().to_string());
    self
  }
  

  
}


///  Searches for ordinary sticker sets by looking for specified query in their title and name. Excludes installed sticker sets from the results. 
#[derive(Debug, Clone)]
pub struct TGSearchStickerSets {
  inner: SearchStickerSets
}

impl TDFB for TGSearchStickerSets {}

impl AsRef<TGSearchStickerSets> for TGSearchStickerSets {
  fn as_ref(&self) -> &TGSearchStickerSets { self }
}

impl AsRef<TGSearchStickerSets> for _TGSearchStickerSetsBuilder {
  fn as_ref(&self) -> &TGSearchStickerSets { &self.inner }
}

impl TGSearchStickerSets {

  pub fn builder() -> _TGSearchStickerSetsBuilder {
    _TGSearchStickerSetsBuilder { inner: Self::new(SearchStickerSets::_new()) }
  }

  pub fn new(inner: SearchStickerSets) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SearchStickerSets { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SearchStickerSets { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSendBotStartMessageBuilder { inner: TGSendBotStartMessage }

impl _TGSendBotStartMessageBuilder {

  pub fn build(&self) -> TGSendBotStartMessage { self.inner.clone() }

  ///  Identifier of the bot. 
  pub fn bot_user_id(&mut self, bot_user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_bot_user_id(bot_user_id);
    self
  }
  ///  Identifier of the target chat. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  A hidden parameter sent to the bot for deep linking purposes (https://api.telegram.org/bots#deep-linking). 
  pub fn parameter<S: AsRef<str>>(&mut self, parameter: S) -> &mut Self {
    self.inner.td_origin_mut()._set_parameter(parameter.as_ref().to_string());
    self
  }
  

  
}


///  Invites a bot to a chat (if it is not yet a member) and sends it the /start command. Bots can't be invited to a private chat other than the chat with the bot. Bots can't be invited to channels (although they can be added as admins) and secret chats. Returns the sent message. 
#[derive(Debug, Clone)]
pub struct TGSendBotStartMessage {
  inner: SendBotStartMessage
}

impl TDFB for TGSendBotStartMessage {}

impl AsRef<TGSendBotStartMessage> for TGSendBotStartMessage {
  fn as_ref(&self) -> &TGSendBotStartMessage { self }
}

impl AsRef<TGSendBotStartMessage> for _TGSendBotStartMessageBuilder {
  fn as_ref(&self) -> &TGSendBotStartMessage { &self.inner }
}

impl TGSendBotStartMessage {

  pub fn builder() -> _TGSendBotStartMessageBuilder {
    _TGSendBotStartMessageBuilder { inner: Self::new(SendBotStartMessage::_new()) }
  }

  pub fn new(inner: SendBotStartMessage) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SendBotStartMessage { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SendBotStartMessage { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSendCallDebugInformationBuilder { inner: TGSendCallDebugInformation }

impl _TGSendCallDebugInformationBuilder {

  pub fn build(&self) -> TGSendCallDebugInformation { self.inner.clone() }

  ///  Call identifier. 
  pub fn call_id(&mut self, call_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_call_id(call_id);
    self
  }
  ///  Debug information in application-specific format. 
  pub fn debug_information<S: AsRef<str>>(&mut self, debug_information: S) -> &mut Self {
    self.inner.td_origin_mut()._set_debug_information(debug_information.as_ref().to_string());
    self
  }
  

  
}


///  Sends debug information for a call. 
#[derive(Debug, Clone)]
pub struct TGSendCallDebugInformation {
  inner: SendCallDebugInformation
}

impl TDFB for TGSendCallDebugInformation {}

impl AsRef<TGSendCallDebugInformation> for TGSendCallDebugInformation {
  fn as_ref(&self) -> &TGSendCallDebugInformation { self }
}

impl AsRef<TGSendCallDebugInformation> for _TGSendCallDebugInformationBuilder {
  fn as_ref(&self) -> &TGSendCallDebugInformation { &self.inner }
}

impl TGSendCallDebugInformation {

  pub fn builder() -> _TGSendCallDebugInformationBuilder {
    _TGSendCallDebugInformationBuilder { inner: Self::new(SendCallDebugInformation::_new()) }
  }

  pub fn new(inner: SendCallDebugInformation) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SendCallDebugInformation { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SendCallDebugInformation { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSendCallRatingBuilder { inner: TGSendCallRating }

impl _TGSendCallRatingBuilder {

  pub fn build(&self) -> TGSendCallRating { self.inner.clone() }

  ///  Call identifier. 
  pub fn call_id(&mut self, call_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_call_id(call_id);
    self
  }
  ///  Call rating; 1-5. 
  pub fn rating(&mut self, rating: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_rating(rating);
    self
  }
  ///  An optional user comment if the rating is less than 5. 
  pub fn comment<S: AsRef<str>>(&mut self, comment: S) -> &mut Self {
    self.inner.td_origin_mut()._set_comment(comment.as_ref().to_string());
    self
  }
  

  
}


///  Sends a call rating. 
#[derive(Debug, Clone)]
pub struct TGSendCallRating {
  inner: SendCallRating
}

impl TDFB for TGSendCallRating {}

impl AsRef<TGSendCallRating> for TGSendCallRating {
  fn as_ref(&self) -> &TGSendCallRating { self }
}

impl AsRef<TGSendCallRating> for _TGSendCallRatingBuilder {
  fn as_ref(&self) -> &TGSendCallRating { &self.inner }
}

impl TGSendCallRating {

  pub fn builder() -> _TGSendCallRatingBuilder {
    _TGSendCallRatingBuilder { inner: Self::new(SendCallRating::_new()) }
  }

  pub fn new(inner: SendCallRating) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SendCallRating { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SendCallRating { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSendChatActionBuilder { inner: TGSendChatAction }

impl _TGSendChatActionBuilder {

  pub fn build(&self) -> TGSendChatAction { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  

  
  // [action] type is [Box<ChatAction>], is not support, need add manully.
  #[doc(hidden)] pub fn _action(&mut self, action: Box<ChatAction>) -> &mut Self {
    self.inner.td_origin_mut()._set_action(action);
    self
  }
  
}


///  Sends a notification about user activity in a chat. 
#[derive(Debug, Clone)]
pub struct TGSendChatAction {
  inner: SendChatAction
}

impl TDFB for TGSendChatAction {}

impl AsRef<TGSendChatAction> for TGSendChatAction {
  fn as_ref(&self) -> &TGSendChatAction { self }
}

impl AsRef<TGSendChatAction> for _TGSendChatActionBuilder {
  fn as_ref(&self) -> &TGSendChatAction { &self.inner }
}

impl TGSendChatAction {

  pub fn builder() -> _TGSendChatActionBuilder {
    _TGSendChatActionBuilder { inner: Self::new(SendChatAction::_new()) }
  }

  pub fn new(inner: SendChatAction) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SendChatAction { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SendChatAction { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSendChatScreenshotTakenNotificationBuilder { inner: TGSendChatScreenshotTakenNotification }

impl _TGSendChatScreenshotTakenNotificationBuilder {

  pub fn build(&self) -> TGSendChatScreenshotTakenNotification { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  

  
}


///  Sends a notification about a screenshot taken in a chat. Supported only in private and secret chats. 
#[derive(Debug, Clone)]
pub struct TGSendChatScreenshotTakenNotification {
  inner: SendChatScreenshotTakenNotification
}

impl TDFB for TGSendChatScreenshotTakenNotification {}

impl AsRef<TGSendChatScreenshotTakenNotification> for TGSendChatScreenshotTakenNotification {
  fn as_ref(&self) -> &TGSendChatScreenshotTakenNotification { self }
}

impl AsRef<TGSendChatScreenshotTakenNotification> for _TGSendChatScreenshotTakenNotificationBuilder {
  fn as_ref(&self) -> &TGSendChatScreenshotTakenNotification { &self.inner }
}

impl TGSendChatScreenshotTakenNotification {

  pub fn builder() -> _TGSendChatScreenshotTakenNotificationBuilder {
    _TGSendChatScreenshotTakenNotificationBuilder { inner: Self::new(SendChatScreenshotTakenNotification::_new()) }
  }

  pub fn new(inner: SendChatScreenshotTakenNotification) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SendChatScreenshotTakenNotification { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SendChatScreenshotTakenNotification { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSendChatSetTtlMessageBuilder { inner: TGSendChatSetTtlMessage }

impl _TGSendChatSetTtlMessageBuilder {

  pub fn build(&self) -> TGSendChatSetTtlMessage { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  New TTL value, in seconds. 
  pub fn ttl(&mut self, ttl: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_ttl(ttl);
    self
  }
  

  
}


///  Changes the current TTL setting (sets a new self-destruct timer) in a secret chat and sends the corresponding message. 
#[derive(Debug, Clone)]
pub struct TGSendChatSetTtlMessage {
  inner: SendChatSetTtlMessage
}

impl TDFB for TGSendChatSetTtlMessage {}

impl AsRef<TGSendChatSetTtlMessage> for TGSendChatSetTtlMessage {
  fn as_ref(&self) -> &TGSendChatSetTtlMessage { self }
}

impl AsRef<TGSendChatSetTtlMessage> for _TGSendChatSetTtlMessageBuilder {
  fn as_ref(&self) -> &TGSendChatSetTtlMessage { &self.inner }
}

impl TGSendChatSetTtlMessage {

  pub fn builder() -> _TGSendChatSetTtlMessageBuilder {
    _TGSendChatSetTtlMessageBuilder { inner: Self::new(SendChatSetTtlMessage::_new()) }
  }

  pub fn new(inner: SendChatSetTtlMessage) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SendChatSetTtlMessage { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SendChatSetTtlMessage { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSendCustomRequestBuilder { inner: TGSendCustomRequest }

impl _TGSendCustomRequestBuilder {

  pub fn build(&self) -> TGSendCustomRequest { self.inner.clone() }

  ///  The method name. 
  pub fn method<S: AsRef<str>>(&mut self, method: S) -> &mut Self {
    self.inner.td_origin_mut()._set_method(method.as_ref().to_string());
    self
  }
  ///  JSON-serialized method parameters. 
  pub fn parameters<S: AsRef<str>>(&mut self, parameters: S) -> &mut Self {
    self.inner.td_origin_mut()._set_parameters(parameters.as_ref().to_string());
    self
  }
  

  
}


///  Sends a custom request; for bots only. 
#[derive(Debug, Clone)]
pub struct TGSendCustomRequest {
  inner: SendCustomRequest
}

impl TDFB for TGSendCustomRequest {}

impl AsRef<TGSendCustomRequest> for TGSendCustomRequest {
  fn as_ref(&self) -> &TGSendCustomRequest { self }
}

impl AsRef<TGSendCustomRequest> for _TGSendCustomRequestBuilder {
  fn as_ref(&self) -> &TGSendCustomRequest { &self.inner }
}

impl TGSendCustomRequest {

  pub fn builder() -> _TGSendCustomRequestBuilder {
    _TGSendCustomRequestBuilder { inner: Self::new(SendCustomRequest::_new()) }
  }

  pub fn new(inner: SendCustomRequest) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SendCustomRequest { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SendCustomRequest { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSendEmailAddressVerificationCodeBuilder { inner: TGSendEmailAddressVerificationCode }

impl _TGSendEmailAddressVerificationCodeBuilder {

  pub fn build(&self) -> TGSendEmailAddressVerificationCode { self.inner.clone() }

  ///  Email address. 
  pub fn email_address<S: AsRef<str>>(&mut self, email_address: S) -> &mut Self {
    self.inner.td_origin_mut()._set_email_address(email_address.as_ref().to_string());
    self
  }
  

  
}


///  Sends a code to verify an email address to be added to a user's Telegram Passport. 
#[derive(Debug, Clone)]
pub struct TGSendEmailAddressVerificationCode {
  inner: SendEmailAddressVerificationCode
}

impl TDFB for TGSendEmailAddressVerificationCode {}

impl AsRef<TGSendEmailAddressVerificationCode> for TGSendEmailAddressVerificationCode {
  fn as_ref(&self) -> &TGSendEmailAddressVerificationCode { self }
}

impl AsRef<TGSendEmailAddressVerificationCode> for _TGSendEmailAddressVerificationCodeBuilder {
  fn as_ref(&self) -> &TGSendEmailAddressVerificationCode { &self.inner }
}

impl TGSendEmailAddressVerificationCode {

  pub fn builder() -> _TGSendEmailAddressVerificationCodeBuilder {
    _TGSendEmailAddressVerificationCodeBuilder { inner: Self::new(SendEmailAddressVerificationCode::_new()) }
  }

  pub fn new(inner: SendEmailAddressVerificationCode) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SendEmailAddressVerificationCode { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SendEmailAddressVerificationCode { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSendInlineQueryResultMessageBuilder { inner: TGSendInlineQueryResultMessage }

impl _TGSendInlineQueryResultMessageBuilder {

  pub fn build(&self) -> TGSendInlineQueryResultMessage { self.inner.clone() }

  ///  Target chat. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Identifier of a message to reply to or 0. 
  pub fn reply_to_message_id(&mut self, reply_to_message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_reply_to_message_id(reply_to_message_id);
    self
  }
  ///  Pass true to disable notification for the message. Not supported in secret chats. 
  pub fn disable_notification(&mut self, disable_notification: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_disable_notification(disable_notification);
    self
  }
  ///  Pass true if the message is sent from background. 
  pub fn from_background(&mut self, from_background: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_from_background(from_background);
    self
  }
  ///  Identifier of the inline query. 
  pub fn query_id(&mut self, query_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_query_id(query_id);
    self
  }
  ///  Identifier of the inline result. 
  pub fn result_id<S: AsRef<str>>(&mut self, result_id: S) -> &mut Self {
    self.inner.td_origin_mut()._set_result_id(result_id.as_ref().to_string());
    self
  }
  ///  If true, there will be no mention of a bot, via which the message is sent. Can be used only for bots GetOption("animation_search_bot_username"), GetOption("photo_search_bot_username") and GetOption("venue_search_bot_username"). 
  pub fn hide_via_bot(&mut self, hide_via_bot: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_hide_via_bot(hide_via_bot);
    self
  }
  

  
}


///  Sends the result of an inline query as a message. Returns the sent message. Always clears a chat draft message. 
#[derive(Debug, Clone)]
pub struct TGSendInlineQueryResultMessage {
  inner: SendInlineQueryResultMessage
}

impl TDFB for TGSendInlineQueryResultMessage {}

impl AsRef<TGSendInlineQueryResultMessage> for TGSendInlineQueryResultMessage {
  fn as_ref(&self) -> &TGSendInlineQueryResultMessage { self }
}

impl AsRef<TGSendInlineQueryResultMessage> for _TGSendInlineQueryResultMessageBuilder {
  fn as_ref(&self) -> &TGSendInlineQueryResultMessage { &self.inner }
}

impl TGSendInlineQueryResultMessage {

  pub fn builder() -> _TGSendInlineQueryResultMessageBuilder {
    _TGSendInlineQueryResultMessageBuilder { inner: Self::new(SendInlineQueryResultMessage::_new()) }
  }

  pub fn new(inner: SendInlineQueryResultMessage) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SendInlineQueryResultMessage { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SendInlineQueryResultMessage { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSendMessageBuilder { inner: TGSendMessage }

impl _TGSendMessageBuilder {

  pub fn build(&self) -> TGSendMessage { self.inner.clone() }

  ///  Target chat. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Identifier of the message to reply to or 0. 
  pub fn reply_to_message_id(&mut self, reply_to_message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_reply_to_message_id(reply_to_message_id);
    self
  }
  ///  Pass true to disable notification for the message. Not supported in secret chats. 
  pub fn disable_notification(&mut self, disable_notification: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_disable_notification(disable_notification);
    self
  }
  ///  Pass true if the message is sent from the background. 
  pub fn from_background(&mut self, from_background: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_from_background(from_background);
    self
  }
  

  
  // [reply_markup] type is [Box<ReplyMarkup>], is not support, need add manully.
  #[doc(hidden)] pub fn _reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self {
    self.inner.td_origin_mut()._set_reply_markup(reply_markup);
    self
  }
  
  // [input_message_content] type is [Box<InputMessageContent>], is not support, need add manully.
  #[doc(hidden)] pub fn _input_message_content(&mut self, input_message_content: Box<InputMessageContent>) -> &mut Self {
    self.inner.td_origin_mut()._set_input_message_content(input_message_content);
    self
  }
  
}


///  Sends a message. Returns the sent message. 
#[derive(Debug, Clone)]
pub struct TGSendMessage {
  inner: SendMessage
}

impl TDFB for TGSendMessage {}

impl AsRef<TGSendMessage> for TGSendMessage {
  fn as_ref(&self) -> &TGSendMessage { self }
}

impl AsRef<TGSendMessage> for _TGSendMessageBuilder {
  fn as_ref(&self) -> &TGSendMessage { &self.inner }
}

impl TGSendMessage {

  pub fn builder() -> _TGSendMessageBuilder {
    _TGSendMessageBuilder { inner: Self::new(SendMessage::_new()) }
  }

  pub fn new(inner: SendMessage) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SendMessage { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SendMessage { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSendMessageAlbumBuilder { inner: TGSendMessageAlbum }

impl _TGSendMessageAlbumBuilder {

  pub fn build(&self) -> TGSendMessageAlbum { self.inner.clone() }

  ///  Target chat. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Identifier of a message to reply to or 0. 
  pub fn reply_to_message_id(&mut self, reply_to_message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_reply_to_message_id(reply_to_message_id);
    self
  }
  ///  Pass true to disable notification for the messages. Not supported in secret chats. 
  pub fn disable_notification(&mut self, disable_notification: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_disable_notification(disable_notification);
    self
  }
  ///  Pass true if the messages are sent from the background. 
  pub fn from_background(&mut self, from_background: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_from_background(from_background);
    self
  }
  

  
  // [input_message_contents] type is [Vec<Box<InputMessageContent>>], is not support, need add manully.
  #[doc(hidden)] pub fn _input_message_contents(&mut self, input_message_contents: Vec<Box<InputMessageContent>>) -> &mut Self {
    self.inner.td_origin_mut()._set_input_message_contents(input_message_contents);
    self
  }
  
}


///  Sends messages grouped together into an album. Currently only photo and video messages can be grouped into an album. Returns sent messages. 
#[derive(Debug, Clone)]
pub struct TGSendMessageAlbum {
  inner: SendMessageAlbum
}

impl TDFB for TGSendMessageAlbum {}

impl AsRef<TGSendMessageAlbum> for TGSendMessageAlbum {
  fn as_ref(&self) -> &TGSendMessageAlbum { self }
}

impl AsRef<TGSendMessageAlbum> for _TGSendMessageAlbumBuilder {
  fn as_ref(&self) -> &TGSendMessageAlbum { &self.inner }
}

impl TGSendMessageAlbum {

  pub fn builder() -> _TGSendMessageAlbumBuilder {
    _TGSendMessageAlbumBuilder { inner: Self::new(SendMessageAlbum::_new()) }
  }

  pub fn new(inner: SendMessageAlbum) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SendMessageAlbum { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SendMessageAlbum { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSendPassportAuthorizationFormBuilder { inner: TGSendPassportAuthorizationForm }

impl _TGSendPassportAuthorizationFormBuilder {

  pub fn build(&self) -> TGSendPassportAuthorizationForm { self.inner.clone() }

  ///  Authorization form identifier. 
  pub fn autorization_form_id(&mut self, autorization_form_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_autorization_form_id(autorization_form_id);
    self
  }
  

  
  // [types] type is [Vec<Box<PassportElementType>>], is not support, need add manully.
  #[doc(hidden)] pub fn _types(&mut self, types: Vec<Box<PassportElementType>>) -> &mut Self {
    self.inner.td_origin_mut()._set_types(types);
    self
  }
  
}


///  Sends a Telegram Passport authorization form, effectively sharing data with the service. This method must be called after  
#[derive(Debug, Clone)]
pub struct TGSendPassportAuthorizationForm {
  inner: SendPassportAuthorizationForm
}

impl TDFB for TGSendPassportAuthorizationForm {}

impl AsRef<TGSendPassportAuthorizationForm> for TGSendPassportAuthorizationForm {
  fn as_ref(&self) -> &TGSendPassportAuthorizationForm { self }
}

impl AsRef<TGSendPassportAuthorizationForm> for _TGSendPassportAuthorizationFormBuilder {
  fn as_ref(&self) -> &TGSendPassportAuthorizationForm { &self.inner }
}

impl TGSendPassportAuthorizationForm {

  pub fn builder() -> _TGSendPassportAuthorizationFormBuilder {
    _TGSendPassportAuthorizationFormBuilder { inner: Self::new(SendPassportAuthorizationForm::_new()) }
  }

  pub fn new(inner: SendPassportAuthorizationForm) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SendPassportAuthorizationForm { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SendPassportAuthorizationForm { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSendPaymentFormBuilder { inner: TGSendPaymentForm }

impl _TGSendPaymentFormBuilder {

  pub fn build(&self) -> TGSendPaymentForm { self.inner.clone() }

  ///  Chat identifier of the Invoice message. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Message identifier. 
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_message_id(message_id);
    self
  }
  ///  Identifier returned by ValidateOrderInfo, or an empty string. 
  pub fn order_info_id<S: AsRef<str>>(&mut self, order_info_id: S) -> &mut Self {
    self.inner.td_origin_mut()._set_order_info_id(order_info_id.as_ref().to_string());
    self
  }
  ///  Identifier of a chosen shipping option, if applicable. 
  pub fn shipping_option_id<S: AsRef<str>>(&mut self, shipping_option_id: S) -> &mut Self {
    self.inner.td_origin_mut()._set_shipping_option_id(shipping_option_id.as_ref().to_string());
    self
  }
  

  
  // [credentials] type is [Box<InputCredentials>], is not support, need add manully.
  #[doc(hidden)] pub fn _credentials(&mut self, credentials: Box<InputCredentials>) -> &mut Self {
    self.inner.td_origin_mut()._set_credentials(credentials);
    self
  }
  
}


///  Sends a filled-out payment form to the bot for final verification. 
#[derive(Debug, Clone)]
pub struct TGSendPaymentForm {
  inner: SendPaymentForm
}

impl TDFB for TGSendPaymentForm {}

impl AsRef<TGSendPaymentForm> for TGSendPaymentForm {
  fn as_ref(&self) -> &TGSendPaymentForm { self }
}

impl AsRef<TGSendPaymentForm> for _TGSendPaymentFormBuilder {
  fn as_ref(&self) -> &TGSendPaymentForm { &self.inner }
}

impl TGSendPaymentForm {

  pub fn builder() -> _TGSendPaymentFormBuilder {
    _TGSendPaymentFormBuilder { inner: Self::new(SendPaymentForm::_new()) }
  }

  pub fn new(inner: SendPaymentForm) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SendPaymentForm { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SendPaymentForm { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSendPhoneNumberConfirmationCodeBuilder { inner: TGSendPhoneNumberConfirmationCode }

impl _TGSendPhoneNumberConfirmationCodeBuilder {

  pub fn build(&self) -> TGSendPhoneNumberConfirmationCode { self.inner.clone() }

  ///  Value of the "hash" parameter from the link. 
  pub fn hash<S: AsRef<str>>(&mut self, hash: S) -> &mut Self {
    self.inner.td_origin_mut()._set_hash(hash.as_ref().to_string());
    self
  }
  ///  Value of the "phone" parameter from the link. 
  pub fn phone_number<S: AsRef<str>>(&mut self, phone_number: S) -> &mut Self {
    self.inner.td_origin_mut()._set_phone_number(phone_number.as_ref().to_string());
    self
  }
  ///  Pass true if the authentication code may be sent via flash call to the specified phone number. 
  pub fn allow_flash_call(&mut self, allow_flash_call: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_allow_flash_call(allow_flash_call);
    self
  }
  ///  Pass true if the phone number is used on the current device. Ignored if allow_flash_call is false. 
  pub fn is_current_phone_number(&mut self, is_current_phone_number: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_is_current_phone_number(is_current_phone_number);
    self
  }
  

  
}


///  Sends phone number confirmation code. Should be called when user presses " 
#[derive(Debug, Clone)]
pub struct TGSendPhoneNumberConfirmationCode {
  inner: SendPhoneNumberConfirmationCode
}

impl TDFB for TGSendPhoneNumberConfirmationCode {}

impl AsRef<TGSendPhoneNumberConfirmationCode> for TGSendPhoneNumberConfirmationCode {
  fn as_ref(&self) -> &TGSendPhoneNumberConfirmationCode { self }
}

impl AsRef<TGSendPhoneNumberConfirmationCode> for _TGSendPhoneNumberConfirmationCodeBuilder {
  fn as_ref(&self) -> &TGSendPhoneNumberConfirmationCode { &self.inner }
}

impl TGSendPhoneNumberConfirmationCode {

  pub fn builder() -> _TGSendPhoneNumberConfirmationCodeBuilder {
    _TGSendPhoneNumberConfirmationCodeBuilder { inner: Self::new(SendPhoneNumberConfirmationCode::_new()) }
  }

  pub fn new(inner: SendPhoneNumberConfirmationCode) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SendPhoneNumberConfirmationCode { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SendPhoneNumberConfirmationCode { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSendPhoneNumberVerificationCodeBuilder { inner: TGSendPhoneNumberVerificationCode }

impl _TGSendPhoneNumberVerificationCodeBuilder {

  pub fn build(&self) -> TGSendPhoneNumberVerificationCode { self.inner.clone() }

  ///  The phone number of the user, in international format. 
  pub fn phone_number<S: AsRef<str>>(&mut self, phone_number: S) -> &mut Self {
    self.inner.td_origin_mut()._set_phone_number(phone_number.as_ref().to_string());
    self
  }
  ///  Pass true if the authentication code may be sent via flash call to the specified phone number. 
  pub fn allow_flash_call(&mut self, allow_flash_call: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_allow_flash_call(allow_flash_call);
    self
  }
  ///  Pass true if the phone number is used on the current device. Ignored if allow_flash_call is false. 
  pub fn is_current_phone_number(&mut self, is_current_phone_number: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_is_current_phone_number(is_current_phone_number);
    self
  }
  

  
}


///  Sends a code to verify a phone number to be added to a user's Telegram Passport. 
#[derive(Debug, Clone)]
pub struct TGSendPhoneNumberVerificationCode {
  inner: SendPhoneNumberVerificationCode
}

impl TDFB for TGSendPhoneNumberVerificationCode {}

impl AsRef<TGSendPhoneNumberVerificationCode> for TGSendPhoneNumberVerificationCode {
  fn as_ref(&self) -> &TGSendPhoneNumberVerificationCode { self }
}

impl AsRef<TGSendPhoneNumberVerificationCode> for _TGSendPhoneNumberVerificationCodeBuilder {
  fn as_ref(&self) -> &TGSendPhoneNumberVerificationCode { &self.inner }
}

impl TGSendPhoneNumberVerificationCode {

  pub fn builder() -> _TGSendPhoneNumberVerificationCodeBuilder {
    _TGSendPhoneNumberVerificationCodeBuilder { inner: Self::new(SendPhoneNumberVerificationCode::_new()) }
  }

  pub fn new(inner: SendPhoneNumberVerificationCode) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SendPhoneNumberVerificationCode { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SendPhoneNumberVerificationCode { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetAccountTtlBuilder { inner: TGSetAccountTtl }

impl _TGSetAccountTtlBuilder {

  pub fn build(&self) -> TGSetAccountTtl { self.inner.clone() }

  

  
  // [ttl] type is [AccountTtl], is not support, need add manully.
  #[doc(hidden)] pub fn _ttl(&mut self, ttl: AccountTtl) -> &mut Self {
    self.inner.td_origin_mut()._set_ttl(ttl);
    self
  }
  
}


///  Changes the period of inactivity after which the account of the current user will automatically be deleted. 
#[derive(Debug, Clone)]
pub struct TGSetAccountTtl {
  inner: SetAccountTtl
}

impl TDFB for TGSetAccountTtl {}

impl AsRef<TGSetAccountTtl> for TGSetAccountTtl {
  fn as_ref(&self) -> &TGSetAccountTtl { self }
}

impl AsRef<TGSetAccountTtl> for _TGSetAccountTtlBuilder {
  fn as_ref(&self) -> &TGSetAccountTtl { &self.inner }
}

impl TGSetAccountTtl {

  pub fn builder() -> _TGSetAccountTtlBuilder {
    _TGSetAccountTtlBuilder { inner: Self::new(SetAccountTtl::_new()) }
  }

  pub fn new(inner: SetAccountTtl) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetAccountTtl { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetAccountTtl { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetAlarmBuilder { inner: TGSetAlarm }

impl _TGSetAlarmBuilder {

  pub fn build(&self) -> TGSetAlarm { self.inner.clone() }

  ///  Number of seconds before the function returns. 
  pub fn seconds(&mut self, seconds: f64) -> &mut Self {
    self.inner.td_origin_mut()._set_seconds(seconds);
    self
  }
  

  
}


///  Succeeds after a specified amount of time has passed. Can be called before authorization. Can be called before initialization. 
#[derive(Debug, Clone)]
pub struct TGSetAlarm {
  inner: SetAlarm
}

impl TDFB for TGSetAlarm {}

impl AsRef<TGSetAlarm> for TGSetAlarm {
  fn as_ref(&self) -> &TGSetAlarm { self }
}

impl AsRef<TGSetAlarm> for _TGSetAlarmBuilder {
  fn as_ref(&self) -> &TGSetAlarm { &self.inner }
}

impl TGSetAlarm {

  pub fn builder() -> _TGSetAlarmBuilder {
    _TGSetAlarmBuilder { inner: Self::new(SetAlarm::_new()) }
  }

  pub fn new(inner: SetAlarm) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetAlarm { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetAlarm { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetAuthenticationPhoneNumberBuilder { inner: TGSetAuthenticationPhoneNumber }

impl _TGSetAuthenticationPhoneNumberBuilder {

  pub fn build(&self) -> TGSetAuthenticationPhoneNumber { self.inner.clone() }

  ///  The phone number of the user, in international format. 
  pub fn phone_number<S: AsRef<str>>(&mut self, phone_number: S) -> &mut Self {
    self.inner.td_origin_mut()._set_phone_number(phone_number.as_ref().to_string());
    self
  }
  ///  Pass true if the authentication code may be sent via flash call to the specified phone number. 
  pub fn allow_flash_call(&mut self, allow_flash_call: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_allow_flash_call(allow_flash_call);
    self
  }
  ///  Pass true if the phone number is used on the current device. Ignored if allow_flash_call is false. 
  pub fn is_current_phone_number(&mut self, is_current_phone_number: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_is_current_phone_number(is_current_phone_number);
    self
  }
  

  
}


///  Sets the phone number of the user and sends an authentication code to the user. Works only when the current authorization state is  
#[derive(Debug, Clone)]
pub struct TGSetAuthenticationPhoneNumber {
  inner: SetAuthenticationPhoneNumber
}

impl TDFB for TGSetAuthenticationPhoneNumber {}

impl AsRef<TGSetAuthenticationPhoneNumber> for TGSetAuthenticationPhoneNumber {
  fn as_ref(&self) -> &TGSetAuthenticationPhoneNumber { self }
}

impl AsRef<TGSetAuthenticationPhoneNumber> for _TGSetAuthenticationPhoneNumberBuilder {
  fn as_ref(&self) -> &TGSetAuthenticationPhoneNumber { &self.inner }
}

impl TGSetAuthenticationPhoneNumber {

  pub fn builder() -> _TGSetAuthenticationPhoneNumberBuilder {
    _TGSetAuthenticationPhoneNumberBuilder { inner: Self::new(SetAuthenticationPhoneNumber::_new()) }
  }

  pub fn new(inner: SetAuthenticationPhoneNumber) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetAuthenticationPhoneNumber { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetAuthenticationPhoneNumber { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetBioBuilder { inner: TGSetBio }

impl _TGSetBioBuilder {

  pub fn build(&self) -> TGSetBio { self.inner.clone() }

  ///  The new value of the user bio; 0-70 characters without line feeds. 
  pub fn bio<S: AsRef<str>>(&mut self, bio: S) -> &mut Self {
    self.inner.td_origin_mut()._set_bio(bio.as_ref().to_string());
    self
  }
  

  
}


///  Changes the bio of the current user. 
#[derive(Debug, Clone)]
pub struct TGSetBio {
  inner: SetBio
}

impl TDFB for TGSetBio {}

impl AsRef<TGSetBio> for TGSetBio {
  fn as_ref(&self) -> &TGSetBio { self }
}

impl AsRef<TGSetBio> for _TGSetBioBuilder {
  fn as_ref(&self) -> &TGSetBio { &self.inner }
}

impl TGSetBio {

  pub fn builder() -> _TGSetBioBuilder {
    _TGSetBioBuilder { inner: Self::new(SetBio::_new()) }
  }

  pub fn new(inner: SetBio) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetBio { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetBio { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetBotUpdatesStatusBuilder { inner: TGSetBotUpdatesStatus }

impl _TGSetBotUpdatesStatusBuilder {

  pub fn build(&self) -> TGSetBotUpdatesStatus { self.inner.clone() }

  ///  The number of pending updates. 
  pub fn pending_update_count(&mut self, pending_update_count: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_pending_update_count(pending_update_count);
    self
  }
  ///  The last error message. 
  pub fn error_message<S: AsRef<str>>(&mut self, error_message: S) -> &mut Self {
    self.inner.td_origin_mut()._set_error_message(error_message.as_ref().to_string());
    self
  }
  

  
}


///  Informs the server about the number of pending bot updates if they haven't been processed for a long time; for bots only. 
#[derive(Debug, Clone)]
pub struct TGSetBotUpdatesStatus {
  inner: SetBotUpdatesStatus
}

impl TDFB for TGSetBotUpdatesStatus {}

impl AsRef<TGSetBotUpdatesStatus> for TGSetBotUpdatesStatus {
  fn as_ref(&self) -> &TGSetBotUpdatesStatus { self }
}

impl AsRef<TGSetBotUpdatesStatus> for _TGSetBotUpdatesStatusBuilder {
  fn as_ref(&self) -> &TGSetBotUpdatesStatus { &self.inner }
}

impl TGSetBotUpdatesStatus {

  pub fn builder() -> _TGSetBotUpdatesStatusBuilder {
    _TGSetBotUpdatesStatusBuilder { inner: Self::new(SetBotUpdatesStatus::_new()) }
  }

  pub fn new(inner: SetBotUpdatesStatus) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetBotUpdatesStatus { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetBotUpdatesStatus { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetChatClientDataBuilder { inner: TGSetChatClientData }

impl _TGSetChatClientDataBuilder {

  pub fn build(&self) -> TGSetChatClientData { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  New value of client_data. 
  pub fn client_data<S: AsRef<str>>(&mut self, client_data: S) -> &mut Self {
    self.inner.td_origin_mut()._set_client_data(client_data.as_ref().to_string());
    self
  }
  

  
}


///  Changes client data associated with a chat. 
#[derive(Debug, Clone)]
pub struct TGSetChatClientData {
  inner: SetChatClientData
}

impl TDFB for TGSetChatClientData {}

impl AsRef<TGSetChatClientData> for TGSetChatClientData {
  fn as_ref(&self) -> &TGSetChatClientData { self }
}

impl AsRef<TGSetChatClientData> for _TGSetChatClientDataBuilder {
  fn as_ref(&self) -> &TGSetChatClientData { &self.inner }
}

impl TGSetChatClientData {

  pub fn builder() -> _TGSetChatClientDataBuilder {
    _TGSetChatClientDataBuilder { inner: Self::new(SetChatClientData::_new()) }
  }

  pub fn new(inner: SetChatClientData) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetChatClientData { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetChatClientData { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetChatDraftMessageBuilder { inner: TGSetChatDraftMessage }

impl _TGSetChatDraftMessageBuilder {

  pub fn build(&self) -> TGSetChatDraftMessage { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  

  
  // [draft_message] type is [DraftMessage], is not support, need add manully.
  #[doc(hidden)] pub fn _draft_message(&mut self, draft_message: DraftMessage) -> &mut Self {
    self.inner.td_origin_mut()._set_draft_message(draft_message);
    self
  }
  
}


///  Changes the draft message in a chat. 
#[derive(Debug, Clone)]
pub struct TGSetChatDraftMessage {
  inner: SetChatDraftMessage
}

impl TDFB for TGSetChatDraftMessage {}

impl AsRef<TGSetChatDraftMessage> for TGSetChatDraftMessage {
  fn as_ref(&self) -> &TGSetChatDraftMessage { self }
}

impl AsRef<TGSetChatDraftMessage> for _TGSetChatDraftMessageBuilder {
  fn as_ref(&self) -> &TGSetChatDraftMessage { &self.inner }
}

impl TGSetChatDraftMessage {

  pub fn builder() -> _TGSetChatDraftMessageBuilder {
    _TGSetChatDraftMessageBuilder { inner: Self::new(SetChatDraftMessage::_new()) }
  }

  pub fn new(inner: SetChatDraftMessage) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetChatDraftMessage { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetChatDraftMessage { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetChatMemberStatusBuilder { inner: TGSetChatMemberStatus }

impl _TGSetChatMemberStatusBuilder {

  pub fn build(&self) -> TGSetChatMemberStatus { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  User identifier. 
  pub fn user_id(&mut self, user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_user_id(user_id);
    self
  }
  

  
  // [status] type is [Box<ChatMemberStatus>], is not support, need add manully.
  #[doc(hidden)] pub fn _status(&mut self, status: Box<ChatMemberStatus>) -> &mut Self {
    self.inner.td_origin_mut()._set_status(status);
    self
  }
  
}


///  Changes the status of a chat member, needs appropriate privileges. This function is currently not suitable for adding new members to the chat; instead, use  
#[derive(Debug, Clone)]
pub struct TGSetChatMemberStatus {
  inner: SetChatMemberStatus
}

impl TDFB for TGSetChatMemberStatus {}

impl AsRef<TGSetChatMemberStatus> for TGSetChatMemberStatus {
  fn as_ref(&self) -> &TGSetChatMemberStatus { self }
}

impl AsRef<TGSetChatMemberStatus> for _TGSetChatMemberStatusBuilder {
  fn as_ref(&self) -> &TGSetChatMemberStatus { &self.inner }
}

impl TGSetChatMemberStatus {

  pub fn builder() -> _TGSetChatMemberStatusBuilder {
    _TGSetChatMemberStatusBuilder { inner: Self::new(SetChatMemberStatus::_new()) }
  }

  pub fn new(inner: SetChatMemberStatus) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetChatMemberStatus { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetChatMemberStatus { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetChatNotificationSettingsBuilder { inner: TGSetChatNotificationSettings }

impl _TGSetChatNotificationSettingsBuilder {

  pub fn build(&self) -> TGSetChatNotificationSettings { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  

  
  // [notification_settings] type is [ChatNotificationSettings], is not support, need add manully.
  #[doc(hidden)] pub fn _notification_settings(&mut self, notification_settings: ChatNotificationSettings) -> &mut Self {
    self.inner.td_origin_mut()._set_notification_settings(notification_settings);
    self
  }
  
}


///  Changes the notification settings of a chat. 
#[derive(Debug, Clone)]
pub struct TGSetChatNotificationSettings {
  inner: SetChatNotificationSettings
}

impl TDFB for TGSetChatNotificationSettings {}

impl AsRef<TGSetChatNotificationSettings> for TGSetChatNotificationSettings {
  fn as_ref(&self) -> &TGSetChatNotificationSettings { self }
}

impl AsRef<TGSetChatNotificationSettings> for _TGSetChatNotificationSettingsBuilder {
  fn as_ref(&self) -> &TGSetChatNotificationSettings { &self.inner }
}

impl TGSetChatNotificationSettings {

  pub fn builder() -> _TGSetChatNotificationSettingsBuilder {
    _TGSetChatNotificationSettingsBuilder { inner: Self::new(SetChatNotificationSettings::_new()) }
  }

  pub fn new(inner: SetChatNotificationSettings) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetChatNotificationSettings { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetChatNotificationSettings { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetChatPhotoBuilder { inner: TGSetChatPhoto }

impl _TGSetChatPhotoBuilder {

  pub fn build(&self) -> TGSetChatPhoto { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  

  
  // [photo] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _photo(&mut self, photo: Box<InputFile>) -> &mut Self {
    self.inner.td_origin_mut()._set_photo(photo);
    self
  }
  
}


///  Changes the photo of a chat. Supported only for basic groups, supergroups and channels. Requires administrator rights in basic groups and the appropriate administrator rights in supergroups and channels. The photo will not be changed before request to the server has been completed. 
#[derive(Debug, Clone)]
pub struct TGSetChatPhoto {
  inner: SetChatPhoto
}

impl TDFB for TGSetChatPhoto {}

impl AsRef<TGSetChatPhoto> for TGSetChatPhoto {
  fn as_ref(&self) -> &TGSetChatPhoto { self }
}

impl AsRef<TGSetChatPhoto> for _TGSetChatPhotoBuilder {
  fn as_ref(&self) -> &TGSetChatPhoto { &self.inner }
}

impl TGSetChatPhoto {

  pub fn builder() -> _TGSetChatPhotoBuilder {
    _TGSetChatPhotoBuilder { inner: Self::new(SetChatPhoto::_new()) }
  }

  pub fn new(inner: SetChatPhoto) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetChatPhoto { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetChatPhoto { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetChatTitleBuilder { inner: TGSetChatTitle }

impl _TGSetChatTitleBuilder {

  pub fn build(&self) -> TGSetChatTitle { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  New title of the chat; 1-128 characters. 
  pub fn title<S: AsRef<str>>(&mut self, title: S) -> &mut Self {
    self.inner.td_origin_mut()._set_title(title.as_ref().to_string());
    self
  }
  

  
}


///  Changes the chat title. Supported only for basic groups, supergroups and channels. Requires administrator rights in basic groups and the appropriate administrator rights in supergroups and channels. The title will not be changed until the request to the server has been completed. 
#[derive(Debug, Clone)]
pub struct TGSetChatTitle {
  inner: SetChatTitle
}

impl TDFB for TGSetChatTitle {}

impl AsRef<TGSetChatTitle> for TGSetChatTitle {
  fn as_ref(&self) -> &TGSetChatTitle { self }
}

impl AsRef<TGSetChatTitle> for _TGSetChatTitleBuilder {
  fn as_ref(&self) -> &TGSetChatTitle { &self.inner }
}

impl TGSetChatTitle {

  pub fn builder() -> _TGSetChatTitleBuilder {
    _TGSetChatTitleBuilder { inner: Self::new(SetChatTitle::_new()) }
  }

  pub fn new(inner: SetChatTitle) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetChatTitle { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetChatTitle { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetCustomLanguagePackBuilder { inner: TGSetCustomLanguagePack }

impl _TGSetCustomLanguagePackBuilder {

  pub fn build(&self) -> TGSetCustomLanguagePack { self.inner.clone() }

  

  
  // [info] type is [LanguagePackInfo], is not support, need add manully.
  #[doc(hidden)] pub fn _info(&mut self, info: LanguagePackInfo) -> &mut Self {
    self.inner.td_origin_mut()._set_info(info);
    self
  }
  
  // [strings] type is [Vec<LanguagePackString>], is not support, need add manully.
  #[doc(hidden)] pub fn _strings(&mut self, strings: Vec<LanguagePackString>) -> &mut Self {
    self.inner.td_origin_mut()._set_strings(strings);
    self
  }
  
}


///  Adds or changes a custom local language pack to the current localization target. 
#[derive(Debug, Clone)]
pub struct TGSetCustomLanguagePack {
  inner: SetCustomLanguagePack
}

impl TDFB for TGSetCustomLanguagePack {}

impl AsRef<TGSetCustomLanguagePack> for TGSetCustomLanguagePack {
  fn as_ref(&self) -> &TGSetCustomLanguagePack { self }
}

impl AsRef<TGSetCustomLanguagePack> for _TGSetCustomLanguagePackBuilder {
  fn as_ref(&self) -> &TGSetCustomLanguagePack { &self.inner }
}

impl TGSetCustomLanguagePack {

  pub fn builder() -> _TGSetCustomLanguagePackBuilder {
    _TGSetCustomLanguagePackBuilder { inner: Self::new(SetCustomLanguagePack::_new()) }
  }

  pub fn new(inner: SetCustomLanguagePack) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetCustomLanguagePack { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetCustomLanguagePack { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetCustomLanguagePackStringBuilder { inner: TGSetCustomLanguagePackString }

impl _TGSetCustomLanguagePackStringBuilder {

  pub fn build(&self) -> TGSetCustomLanguagePackString { self.inner.clone() }

  ///  Identifier of a previously added custom local language pack in the current localization target. 
  pub fn language_pack_id<S: AsRef<str>>(&mut self, language_pack_id: S) -> &mut Self {
    self.inner.td_origin_mut()._set_language_pack_id(language_pack_id.as_ref().to_string());
    self
  }
  

  
  // [new_string] type is [LanguagePackString], is not support, need add manully.
  #[doc(hidden)] pub fn _new_string(&mut self, new_string: LanguagePackString) -> &mut Self {
    self.inner.td_origin_mut()._set_new_string(new_string);
    self
  }
  
}


///  Adds, edits or deletes a string in a custom local language pack. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGSetCustomLanguagePackString {
  inner: SetCustomLanguagePackString
}

impl TDFB for TGSetCustomLanguagePackString {}

impl AsRef<TGSetCustomLanguagePackString> for TGSetCustomLanguagePackString {
  fn as_ref(&self) -> &TGSetCustomLanguagePackString { self }
}

impl AsRef<TGSetCustomLanguagePackString> for _TGSetCustomLanguagePackStringBuilder {
  fn as_ref(&self) -> &TGSetCustomLanguagePackString { &self.inner }
}

impl TGSetCustomLanguagePackString {

  pub fn builder() -> _TGSetCustomLanguagePackStringBuilder {
    _TGSetCustomLanguagePackStringBuilder { inner: Self::new(SetCustomLanguagePackString::_new()) }
  }

  pub fn new(inner: SetCustomLanguagePackString) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetCustomLanguagePackString { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetCustomLanguagePackString { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetDatabaseEncryptionKeyBuilder { inner: TGSetDatabaseEncryptionKey }

impl _TGSetDatabaseEncryptionKeyBuilder {

  pub fn build(&self) -> TGSetDatabaseEncryptionKey { self.inner.clone() }

  ///  New encryption key. 
  pub fn new_encryption_key<S: AsRef<str>>(&mut self, new_encryption_key: S) -> &mut Self {
    self.inner.td_origin_mut()._set_new_encryption_key(new_encryption_key.as_ref().to_string());
    self
  }
  

  
}


///  Changes the database encryption key. Usually the encryption key is never changed and is stored in some OS keychain. 
#[derive(Debug, Clone)]
pub struct TGSetDatabaseEncryptionKey {
  inner: SetDatabaseEncryptionKey
}

impl TDFB for TGSetDatabaseEncryptionKey {}

impl AsRef<TGSetDatabaseEncryptionKey> for TGSetDatabaseEncryptionKey {
  fn as_ref(&self) -> &TGSetDatabaseEncryptionKey { self }
}

impl AsRef<TGSetDatabaseEncryptionKey> for _TGSetDatabaseEncryptionKeyBuilder {
  fn as_ref(&self) -> &TGSetDatabaseEncryptionKey { &self.inner }
}

impl TGSetDatabaseEncryptionKey {

  pub fn builder() -> _TGSetDatabaseEncryptionKeyBuilder {
    _TGSetDatabaseEncryptionKeyBuilder { inner: Self::new(SetDatabaseEncryptionKey::_new()) }
  }

  pub fn new(inner: SetDatabaseEncryptionKey) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetDatabaseEncryptionKey { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetDatabaseEncryptionKey { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetFileGenerationProgressBuilder { inner: TGSetFileGenerationProgress }

impl _TGSetFileGenerationProgressBuilder {

  pub fn build(&self) -> TGSetFileGenerationProgress { self.inner.clone() }

  ///  The identifier of the generation process. 
  pub fn generation_id(&mut self, generation_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_generation_id(generation_id);
    self
  }
  ///  Expected size of the generated file, in bytes; 0 if unknown. 
  pub fn expected_size(&mut self, expected_size: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_expected_size(expected_size);
    self
  }
  ///  The number of bytes already generated. 
  pub fn local_prefix_size(&mut self, local_prefix_size: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_local_prefix_size(local_prefix_size);
    self
  }
  

  
}


///  Informs TDLib on a file generation prograss. 
#[derive(Debug, Clone)]
pub struct TGSetFileGenerationProgress {
  inner: SetFileGenerationProgress
}

impl TDFB for TGSetFileGenerationProgress {}

impl AsRef<TGSetFileGenerationProgress> for TGSetFileGenerationProgress {
  fn as_ref(&self) -> &TGSetFileGenerationProgress { self }
}

impl AsRef<TGSetFileGenerationProgress> for _TGSetFileGenerationProgressBuilder {
  fn as_ref(&self) -> &TGSetFileGenerationProgress { &self.inner }
}

impl TGSetFileGenerationProgress {

  pub fn builder() -> _TGSetFileGenerationProgressBuilder {
    _TGSetFileGenerationProgressBuilder { inner: Self::new(SetFileGenerationProgress::_new()) }
  }

  pub fn new(inner: SetFileGenerationProgress) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetFileGenerationProgress { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetFileGenerationProgress { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetGameScoreBuilder { inner: TGSetGameScore }

impl _TGSetGameScoreBuilder {

  pub fn build(&self) -> TGSetGameScore { self.inner.clone() }

  ///  The chat to which the message with the game belongs. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Identifier of the message. 
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_message_id(message_id);
    self
  }
  ///  True, if the message should be edited. 
  pub fn edit_message(&mut self, edit_message: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_edit_message(edit_message);
    self
  }
  ///  User identifier. 
  pub fn user_id(&mut self, user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_user_id(user_id);
    self
  }
  ///  The new score. 
  pub fn score(&mut self, score: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_score(score);
    self
  }
  ///  Pass true to update the score even if it decreases. If the score is 0, the user will be deleted from the high score table. 
  pub fn force(&mut self, force: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_force(force);
    self
  }
  

  
}


///  Updates the game score of the specified user in the game; for bots only. 
#[derive(Debug, Clone)]
pub struct TGSetGameScore {
  inner: SetGameScore
}

impl TDFB for TGSetGameScore {}

impl AsRef<TGSetGameScore> for TGSetGameScore {
  fn as_ref(&self) -> &TGSetGameScore { self }
}

impl AsRef<TGSetGameScore> for _TGSetGameScoreBuilder {
  fn as_ref(&self) -> &TGSetGameScore { &self.inner }
}

impl TGSetGameScore {

  pub fn builder() -> _TGSetGameScoreBuilder {
    _TGSetGameScoreBuilder { inner: Self::new(SetGameScore::_new()) }
  }

  pub fn new(inner: SetGameScore) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetGameScore { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetGameScore { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetInlineGameScoreBuilder { inner: TGSetInlineGameScore }

impl _TGSetInlineGameScoreBuilder {

  pub fn build(&self) -> TGSetInlineGameScore { self.inner.clone() }

  ///  Inline message identifier. 
  pub fn inline_message_id<S: AsRef<str>>(&mut self, inline_message_id: S) -> &mut Self {
    self.inner.td_origin_mut()._set_inline_message_id(inline_message_id.as_ref().to_string());
    self
  }
  ///  True, if the message should be edited. 
  pub fn edit_message(&mut self, edit_message: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_edit_message(edit_message);
    self
  }
  ///  User identifier. 
  pub fn user_id(&mut self, user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_user_id(user_id);
    self
  }
  ///  The new score. 
  pub fn score(&mut self, score: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_score(score);
    self
  }
  ///  Pass true to update the score even if it decreases. If the score is 0, the user will be deleted from the high score table. 
  pub fn force(&mut self, force: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_force(force);
    self
  }
  

  
}


///  Updates the game score of the specified user in a game; for bots only. 
#[derive(Debug, Clone)]
pub struct TGSetInlineGameScore {
  inner: SetInlineGameScore
}

impl TDFB for TGSetInlineGameScore {}

impl AsRef<TGSetInlineGameScore> for TGSetInlineGameScore {
  fn as_ref(&self) -> &TGSetInlineGameScore { self }
}

impl AsRef<TGSetInlineGameScore> for _TGSetInlineGameScoreBuilder {
  fn as_ref(&self) -> &TGSetInlineGameScore { &self.inner }
}

impl TGSetInlineGameScore {

  pub fn builder() -> _TGSetInlineGameScoreBuilder {
    _TGSetInlineGameScoreBuilder { inner: Self::new(SetInlineGameScore::_new()) }
  }

  pub fn new(inner: SetInlineGameScore) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetInlineGameScore { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetInlineGameScore { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetLogStreamBuilder { inner: TGSetLogStream }

impl _TGSetLogStreamBuilder {

  pub fn build(&self) -> TGSetLogStream { self.inner.clone() }

  

  
  // [log_stream] type is [Box<LogStream>], is not support, need add manully.
  #[doc(hidden)] pub fn _log_stream(&mut self, log_stream: Box<LogStream>) -> &mut Self {
    self.inner.td_origin_mut()._set_log_stream(log_stream);
    self
  }
  
}


///  Sets new log stream for internal logging of TDLib. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGSetLogStream {
  inner: SetLogStream
}

impl TDFB for TGSetLogStream {}

impl AsRef<TGSetLogStream> for TGSetLogStream {
  fn as_ref(&self) -> &TGSetLogStream { self }
}

impl AsRef<TGSetLogStream> for _TGSetLogStreamBuilder {
  fn as_ref(&self) -> &TGSetLogStream { &self.inner }
}

impl TGSetLogStream {

  pub fn builder() -> _TGSetLogStreamBuilder {
    _TGSetLogStreamBuilder { inner: Self::new(SetLogStream::_new()) }
  }

  pub fn new(inner: SetLogStream) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetLogStream { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetLogStream { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetLogTagVerbosityLevelBuilder { inner: TGSetLogTagVerbosityLevel }

impl _TGSetLogTagVerbosityLevelBuilder {

  pub fn build(&self) -> TGSetLogTagVerbosityLevel { self.inner.clone() }

  ///  Logging tag to change verbosity level. 
  pub fn tag<S: AsRef<str>>(&mut self, tag: S) -> &mut Self {
    self.inner.td_origin_mut()._set_tag(tag.as_ref().to_string());
    self
  }
  ///  New verbosity level; 1-1024. 
  pub fn new_verbosity_level(&mut self, new_verbosity_level: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_new_verbosity_level(new_verbosity_level);
    self
  }
  

  
}


///  Sets the verbosity level for a specified TDLib internal log tag. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGSetLogTagVerbosityLevel {
  inner: SetLogTagVerbosityLevel
}

impl TDFB for TGSetLogTagVerbosityLevel {}

impl AsRef<TGSetLogTagVerbosityLevel> for TGSetLogTagVerbosityLevel {
  fn as_ref(&self) -> &TGSetLogTagVerbosityLevel { self }
}

impl AsRef<TGSetLogTagVerbosityLevel> for _TGSetLogTagVerbosityLevelBuilder {
  fn as_ref(&self) -> &TGSetLogTagVerbosityLevel { &self.inner }
}

impl TGSetLogTagVerbosityLevel {

  pub fn builder() -> _TGSetLogTagVerbosityLevelBuilder {
    _TGSetLogTagVerbosityLevelBuilder { inner: Self::new(SetLogTagVerbosityLevel::_new()) }
  }

  pub fn new(inner: SetLogTagVerbosityLevel) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetLogTagVerbosityLevel { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetLogTagVerbosityLevel { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetLogVerbosityLevelBuilder { inner: TGSetLogVerbosityLevel }

impl _TGSetLogVerbosityLevelBuilder {

  pub fn build(&self) -> TGSetLogVerbosityLevel { self.inner.clone() }

  ///  New value of the verbosity level for logging. Value 0 corresponds to fatal errors, value 1 corresponds to errors, value 2 corresponds to warnings and debug warnings, value 3 corresponds to informational, value 4 corresponds to debug, value 5 corresponds to verbose debug, value greater than 5 and up to 1023 can be used to enable even more logging. 
  pub fn new_verbosity_level(&mut self, new_verbosity_level: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_new_verbosity_level(new_verbosity_level);
    self
  }
  

  
}


///  Sets the verbosity level of the internal logging of TDLib. This is an offline method. Can be called before authorization. Can be called synchronously. 
#[derive(Debug, Clone)]
pub struct TGSetLogVerbosityLevel {
  inner: SetLogVerbosityLevel
}

impl TDFB for TGSetLogVerbosityLevel {}

impl AsRef<TGSetLogVerbosityLevel> for TGSetLogVerbosityLevel {
  fn as_ref(&self) -> &TGSetLogVerbosityLevel { self }
}

impl AsRef<TGSetLogVerbosityLevel> for _TGSetLogVerbosityLevelBuilder {
  fn as_ref(&self) -> &TGSetLogVerbosityLevel { &self.inner }
}

impl TGSetLogVerbosityLevel {

  pub fn builder() -> _TGSetLogVerbosityLevelBuilder {
    _TGSetLogVerbosityLevelBuilder { inner: Self::new(SetLogVerbosityLevel::_new()) }
  }

  pub fn new(inner: SetLogVerbosityLevel) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetLogVerbosityLevel { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetLogVerbosityLevel { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetNameBuilder { inner: TGSetName }

impl _TGSetNameBuilder {

  pub fn build(&self) -> TGSetName { self.inner.clone() }

  ///  The new value of the first name for the user; 1-64 characters. 
  pub fn first_name<S: AsRef<str>>(&mut self, first_name: S) -> &mut Self {
    self.inner.td_origin_mut()._set_first_name(first_name.as_ref().to_string());
    self
  }
  ///  The new value of the optional last name for the user; 0-64 characters. 
  pub fn last_name<S: AsRef<str>>(&mut self, last_name: S) -> &mut Self {
    self.inner.td_origin_mut()._set_last_name(last_name.as_ref().to_string());
    self
  }
  

  
}


///  Changes the first and last name of the current user. If something changes,  
#[derive(Debug, Clone)]
pub struct TGSetName {
  inner: SetName
}

impl TDFB for TGSetName {}

impl AsRef<TGSetName> for TGSetName {
  fn as_ref(&self) -> &TGSetName { self }
}

impl AsRef<TGSetName> for _TGSetNameBuilder {
  fn as_ref(&self) -> &TGSetName { &self.inner }
}

impl TGSetName {

  pub fn builder() -> _TGSetNameBuilder {
    _TGSetNameBuilder { inner: Self::new(SetName::_new()) }
  }

  pub fn new(inner: SetName) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetName { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetName { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetNetworkTypeBuilder { inner: TGSetNetworkType }

impl _TGSetNetworkTypeBuilder {

  pub fn build(&self) -> TGSetNetworkType { self.inner.clone() }

  

  
  // [type_] type is [Box<NetworkType>], is not support, need add manully.
  #[doc(hidden)] pub fn _type_(&mut self, type_: Box<NetworkType>) -> &mut Self {
    self.inner.td_origin_mut()._set_type_(type_);
    self
  }
  
}


///  Sets the current network type. Can be called before authorization. Calling this method forces all network connections to reopen, mitigating the delay in switching between different networks, so it should be called whenever the network is changed, even if the network type remains the same. Network type is used to check whether the library can use the network at all and also for collecting detailed network data usage statistics. 
#[derive(Debug, Clone)]
pub struct TGSetNetworkType {
  inner: SetNetworkType
}

impl TDFB for TGSetNetworkType {}

impl AsRef<TGSetNetworkType> for TGSetNetworkType {
  fn as_ref(&self) -> &TGSetNetworkType { self }
}

impl AsRef<TGSetNetworkType> for _TGSetNetworkTypeBuilder {
  fn as_ref(&self) -> &TGSetNetworkType { &self.inner }
}

impl TGSetNetworkType {

  pub fn builder() -> _TGSetNetworkTypeBuilder {
    _TGSetNetworkTypeBuilder { inner: Self::new(SetNetworkType::_new()) }
  }

  pub fn new(inner: SetNetworkType) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetNetworkType { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetNetworkType { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetOptionBuilder { inner: TGSetOption }

impl _TGSetOptionBuilder {

  pub fn build(&self) -> TGSetOption { self.inner.clone() }

  ///  The name of the option. 
  pub fn name<S: AsRef<str>>(&mut self, name: S) -> &mut Self {
    self.inner.td_origin_mut()._set_name(name.as_ref().to_string());
    self
  }
  

  
  // [value] type is [Box<OptionValue>], is not support, need add manully.
  #[doc(hidden)] pub fn _value(&mut self, value: Box<OptionValue>) -> &mut Self {
    self.inner.td_origin_mut()._set_value(value);
    self
  }
  
}


///  Sets the value of an option. (Check the list of available options on  
#[derive(Debug, Clone)]
pub struct TGSetOption {
  inner: SetOption
}

impl TDFB for TGSetOption {}

impl AsRef<TGSetOption> for TGSetOption {
  fn as_ref(&self) -> &TGSetOption { self }
}

impl AsRef<TGSetOption> for _TGSetOptionBuilder {
  fn as_ref(&self) -> &TGSetOption { &self.inner }
}

impl TGSetOption {

  pub fn builder() -> _TGSetOptionBuilder {
    _TGSetOptionBuilder { inner: Self::new(SetOption::_new()) }
  }

  pub fn new(inner: SetOption) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetOption { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetOption { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetPassportElementBuilder { inner: TGSetPassportElement }

impl _TGSetPassportElementBuilder {

  pub fn build(&self) -> TGSetPassportElement { self.inner.clone() }

  ///  Password of the current user. 
  pub fn password<S: AsRef<str>>(&mut self, password: S) -> &mut Self {
    self.inner.td_origin_mut()._set_password(password.as_ref().to_string());
    self
  }
  

  
  // [element] type is [Box<InputPassportElement>], is not support, need add manully.
  #[doc(hidden)] pub fn _element(&mut self, element: Box<InputPassportElement>) -> &mut Self {
    self.inner.td_origin_mut()._set_element(element);
    self
  }
  
}


///  Adds an element to the user's Telegram Passport. May return an error with a message "PHONE_VERIFICATION_NEEDED" or "EMAIL_VERIFICATION_NEEDED" if the chosen phone number or the chosen email address must be verified first. 
#[derive(Debug, Clone)]
pub struct TGSetPassportElement {
  inner: SetPassportElement
}

impl TDFB for TGSetPassportElement {}

impl AsRef<TGSetPassportElement> for TGSetPassportElement {
  fn as_ref(&self) -> &TGSetPassportElement { self }
}

impl AsRef<TGSetPassportElement> for _TGSetPassportElementBuilder {
  fn as_ref(&self) -> &TGSetPassportElement { &self.inner }
}

impl TGSetPassportElement {

  pub fn builder() -> _TGSetPassportElementBuilder {
    _TGSetPassportElementBuilder { inner: Self::new(SetPassportElement::_new()) }
  }

  pub fn new(inner: SetPassportElement) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetPassportElement { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetPassportElement { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetPassportElementErrorsBuilder { inner: TGSetPassportElementErrors }

impl _TGSetPassportElementErrorsBuilder {

  pub fn build(&self) -> TGSetPassportElementErrors { self.inner.clone() }

  ///  User identifier. 
  pub fn user_id(&mut self, user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_user_id(user_id);
    self
  }
  

  
  // [errors] type is [Vec<InputPassportElementError>], is not support, need add manully.
  #[doc(hidden)] pub fn _errors(&mut self, errors: Vec<InputPassportElementError>) -> &mut Self {
    self.inner.td_origin_mut()._set_errors(errors);
    self
  }
  
}


///  Informs the user that some of the elements in their Telegram Passport contain errors; for bots only. The user will not be able to resend the elements, until the errors are fixed. 
#[derive(Debug, Clone)]
pub struct TGSetPassportElementErrors {
  inner: SetPassportElementErrors
}

impl TDFB for TGSetPassportElementErrors {}

impl AsRef<TGSetPassportElementErrors> for TGSetPassportElementErrors {
  fn as_ref(&self) -> &TGSetPassportElementErrors { self }
}

impl AsRef<TGSetPassportElementErrors> for _TGSetPassportElementErrorsBuilder {
  fn as_ref(&self) -> &TGSetPassportElementErrors { &self.inner }
}

impl TGSetPassportElementErrors {

  pub fn builder() -> _TGSetPassportElementErrorsBuilder {
    _TGSetPassportElementErrorsBuilder { inner: Self::new(SetPassportElementErrors::_new()) }
  }

  pub fn new(inner: SetPassportElementErrors) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetPassportElementErrors { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetPassportElementErrors { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetPasswordBuilder { inner: TGSetPassword }

impl _TGSetPasswordBuilder {

  pub fn build(&self) -> TGSetPassword { self.inner.clone() }

  ///  Previous password of the user. 
  pub fn old_password<S: AsRef<str>>(&mut self, old_password: S) -> &mut Self {
    self.inner.td_origin_mut()._set_old_password(old_password.as_ref().to_string());
    self
  }
  ///  New password of the user; may be empty to remove the password. 
  pub fn new_password<S: AsRef<str>>(&mut self, new_password: S) -> &mut Self {
    self.inner.td_origin_mut()._set_new_password(new_password.as_ref().to_string());
    self
  }
  ///  New password hint; may be empty. 
  pub fn new_hint<S: AsRef<str>>(&mut self, new_hint: S) -> &mut Self {
    self.inner.td_origin_mut()._set_new_hint(new_hint.as_ref().to_string());
    self
  }
  ///  Pass true if the recovery email address should be changed. 
  pub fn set_recovery_email_address(&mut self, set_recovery_email_address: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_set_recovery_email_address(set_recovery_email_address);
    self
  }
  ///  New recovery email address; may be empty. 
  pub fn new_recovery_email_address<S: AsRef<str>>(&mut self, new_recovery_email_address: S) -> &mut Self {
    self.inner.td_origin_mut()._set_new_recovery_email_address(new_recovery_email_address.as_ref().to_string());
    self
  }
  

  
}


///  Changes the password for the user. If a new recovery email address is specified, then the change will not be applied until the new recovery email address is confirmed. 
#[derive(Debug, Clone)]
pub struct TGSetPassword {
  inner: SetPassword
}

impl TDFB for TGSetPassword {}

impl AsRef<TGSetPassword> for TGSetPassword {
  fn as_ref(&self) -> &TGSetPassword { self }
}

impl AsRef<TGSetPassword> for _TGSetPasswordBuilder {
  fn as_ref(&self) -> &TGSetPassword { &self.inner }
}

impl TGSetPassword {

  pub fn builder() -> _TGSetPasswordBuilder {
    _TGSetPasswordBuilder { inner: Self::new(SetPassword::_new()) }
  }

  pub fn new(inner: SetPassword) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetPassword { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetPassword { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetPinnedChatsBuilder { inner: TGSetPinnedChats }

impl _TGSetPinnedChatsBuilder {

  pub fn build(&self) -> TGSetPinnedChats { self.inner.clone() }

  ///  The new list of pinned chats. 
  pub fn chat_ids(&mut self, chat_ids: Vec<i64>) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_ids(chat_ids);
    self
  }
  

  
}


///  Changes the order of pinned chats. 
#[derive(Debug, Clone)]
pub struct TGSetPinnedChats {
  inner: SetPinnedChats
}

impl TDFB for TGSetPinnedChats {}

impl AsRef<TGSetPinnedChats> for TGSetPinnedChats {
  fn as_ref(&self) -> &TGSetPinnedChats { self }
}

impl AsRef<TGSetPinnedChats> for _TGSetPinnedChatsBuilder {
  fn as_ref(&self) -> &TGSetPinnedChats { &self.inner }
}

impl TGSetPinnedChats {

  pub fn builder() -> _TGSetPinnedChatsBuilder {
    _TGSetPinnedChatsBuilder { inner: Self::new(SetPinnedChats::_new()) }
  }

  pub fn new(inner: SetPinnedChats) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetPinnedChats { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetPinnedChats { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetPollAnswerBuilder { inner: TGSetPollAnswer }

impl _TGSetPollAnswerBuilder {

  pub fn build(&self) -> TGSetPollAnswer { self.inner.clone() }

  ///  Identifier of the chat to which the poll belongs. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Identifier of the message containing the poll. 
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_message_id(message_id);
    self
  }
  ///  0-based identifiers of options, chosen by the user. Currently user can't choose more than 1 option. 
  pub fn option_ids(&mut self, option_ids: Vec<i32>) -> &mut Self {
    self.inner.td_origin_mut()._set_option_ids(option_ids);
    self
  }
  

  
}


///  Changes user answer to a poll. 
#[derive(Debug, Clone)]
pub struct TGSetPollAnswer {
  inner: SetPollAnswer
}

impl TDFB for TGSetPollAnswer {}

impl AsRef<TGSetPollAnswer> for TGSetPollAnswer {
  fn as_ref(&self) -> &TGSetPollAnswer { self }
}

impl AsRef<TGSetPollAnswer> for _TGSetPollAnswerBuilder {
  fn as_ref(&self) -> &TGSetPollAnswer { &self.inner }
}

impl TGSetPollAnswer {

  pub fn builder() -> _TGSetPollAnswerBuilder {
    _TGSetPollAnswerBuilder { inner: Self::new(SetPollAnswer::_new()) }
  }

  pub fn new(inner: SetPollAnswer) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetPollAnswer { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetPollAnswer { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetProfilePhotoBuilder { inner: TGSetProfilePhoto }

impl _TGSetProfilePhotoBuilder {

  pub fn build(&self) -> TGSetProfilePhoto { self.inner.clone() }

  

  
  // [photo] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _photo(&mut self, photo: Box<InputFile>) -> &mut Self {
    self.inner.td_origin_mut()._set_photo(photo);
    self
  }
  
}


///  Uploads a new profile photo for the current user. If something changes,  
#[derive(Debug, Clone)]
pub struct TGSetProfilePhoto {
  inner: SetProfilePhoto
}

impl TDFB for TGSetProfilePhoto {}

impl AsRef<TGSetProfilePhoto> for TGSetProfilePhoto {
  fn as_ref(&self) -> &TGSetProfilePhoto { self }
}

impl AsRef<TGSetProfilePhoto> for _TGSetProfilePhotoBuilder {
  fn as_ref(&self) -> &TGSetProfilePhoto { &self.inner }
}

impl TGSetProfilePhoto {

  pub fn builder() -> _TGSetProfilePhotoBuilder {
    _TGSetProfilePhotoBuilder { inner: Self::new(SetProfilePhoto::_new()) }
  }

  pub fn new(inner: SetProfilePhoto) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetProfilePhoto { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetProfilePhoto { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetRecoveryEmailAddressBuilder { inner: TGSetRecoveryEmailAddress }

impl _TGSetRecoveryEmailAddressBuilder {

  pub fn build(&self) -> TGSetRecoveryEmailAddress { self.inner.clone() }

  ///  Password of the current user. 
  pub fn password<S: AsRef<str>>(&mut self, password: S) -> &mut Self {
    self.inner.td_origin_mut()._set_password(password.as_ref().to_string());
    self
  }
  ///  New recovery email address. 
  pub fn new_recovery_email_address<S: AsRef<str>>(&mut self, new_recovery_email_address: S) -> &mut Self {
    self.inner.td_origin_mut()._set_new_recovery_email_address(new_recovery_email_address.as_ref().to_string());
    self
  }
  

  
}


///  Changes the 2-step verification recovery email address of the user. If a new recovery email address is specified, then the change will not be applied until the new recovery email address is confirmed If new_recovery_email_address is the same as the email address that is currently set up, this call succeeds immediately and aborts all other requests waiting for an email confirmation. 
#[derive(Debug, Clone)]
pub struct TGSetRecoveryEmailAddress {
  inner: SetRecoveryEmailAddress
}

impl TDFB for TGSetRecoveryEmailAddress {}

impl AsRef<TGSetRecoveryEmailAddress> for TGSetRecoveryEmailAddress {
  fn as_ref(&self) -> &TGSetRecoveryEmailAddress { self }
}

impl AsRef<TGSetRecoveryEmailAddress> for _TGSetRecoveryEmailAddressBuilder {
  fn as_ref(&self) -> &TGSetRecoveryEmailAddress { &self.inner }
}

impl TGSetRecoveryEmailAddress {

  pub fn builder() -> _TGSetRecoveryEmailAddressBuilder {
    _TGSetRecoveryEmailAddressBuilder { inner: Self::new(SetRecoveryEmailAddress::_new()) }
  }

  pub fn new(inner: SetRecoveryEmailAddress) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetRecoveryEmailAddress { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetRecoveryEmailAddress { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetScopeNotificationSettingsBuilder { inner: TGSetScopeNotificationSettings }

impl _TGSetScopeNotificationSettingsBuilder {

  pub fn build(&self) -> TGSetScopeNotificationSettings { self.inner.clone() }

  

  
  // [scope] type is [Box<NotificationSettingsScope>], is not support, need add manully.
  #[doc(hidden)] pub fn _scope(&mut self, scope: Box<NotificationSettingsScope>) -> &mut Self {
    self.inner.td_origin_mut()._set_scope(scope);
    self
  }
  
  // [notification_settings] type is [ScopeNotificationSettings], is not support, need add manully.
  #[doc(hidden)] pub fn _notification_settings(&mut self, notification_settings: ScopeNotificationSettings) -> &mut Self {
    self.inner.td_origin_mut()._set_notification_settings(notification_settings);
    self
  }
  
}


///  Changes notification settings for chats of a given type. 
#[derive(Debug, Clone)]
pub struct TGSetScopeNotificationSettings {
  inner: SetScopeNotificationSettings
}

impl TDFB for TGSetScopeNotificationSettings {}

impl AsRef<TGSetScopeNotificationSettings> for TGSetScopeNotificationSettings {
  fn as_ref(&self) -> &TGSetScopeNotificationSettings { self }
}

impl AsRef<TGSetScopeNotificationSettings> for _TGSetScopeNotificationSettingsBuilder {
  fn as_ref(&self) -> &TGSetScopeNotificationSettings { &self.inner }
}

impl TGSetScopeNotificationSettings {

  pub fn builder() -> _TGSetScopeNotificationSettingsBuilder {
    _TGSetScopeNotificationSettingsBuilder { inner: Self::new(SetScopeNotificationSettings::_new()) }
  }

  pub fn new(inner: SetScopeNotificationSettings) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetScopeNotificationSettings { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetScopeNotificationSettings { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetStickerPositionInSetBuilder { inner: TGSetStickerPositionInSet }

impl _TGSetStickerPositionInSetBuilder {

  pub fn build(&self) -> TGSetStickerPositionInSet { self.inner.clone() }

  ///  New position of the sticker in the set, zero-based. 
  pub fn position(&mut self, position: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_position(position);
    self
  }
  

  
  // [sticker] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _sticker(&mut self, sticker: Box<InputFile>) -> &mut Self {
    self.inner.td_origin_mut()._set_sticker(sticker);
    self
  }
  
}


///  Changes the position of a sticker in the set to which it belongs; for bots only. The sticker set must have been created by the bot. 
#[derive(Debug, Clone)]
pub struct TGSetStickerPositionInSet {
  inner: SetStickerPositionInSet
}

impl TDFB for TGSetStickerPositionInSet {}

impl AsRef<TGSetStickerPositionInSet> for TGSetStickerPositionInSet {
  fn as_ref(&self) -> &TGSetStickerPositionInSet { self }
}

impl AsRef<TGSetStickerPositionInSet> for _TGSetStickerPositionInSetBuilder {
  fn as_ref(&self) -> &TGSetStickerPositionInSet { &self.inner }
}

impl TGSetStickerPositionInSet {

  pub fn builder() -> _TGSetStickerPositionInSetBuilder {
    _TGSetStickerPositionInSetBuilder { inner: Self::new(SetStickerPositionInSet::_new()) }
  }

  pub fn new(inner: SetStickerPositionInSet) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetStickerPositionInSet { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetStickerPositionInSet { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetSupergroupDescriptionBuilder { inner: TGSetSupergroupDescription }

impl _TGSetSupergroupDescriptionBuilder {

  pub fn build(&self) -> TGSetSupergroupDescription { self.inner.clone() }

  ///  Identifier of the supergroup or channel. 
  pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_supergroup_id(supergroup_id);
    self
  }
  ///  New supergroup or channel description; 0-255 characters. 
  pub fn description<S: AsRef<str>>(&mut self, description: S) -> &mut Self {
    self.inner.td_origin_mut()._set_description(description.as_ref().to_string());
    self
  }
  

  
}


///  Changes information about a supergroup or channel; requires appropriate administrator rights. 
#[derive(Debug, Clone)]
pub struct TGSetSupergroupDescription {
  inner: SetSupergroupDescription
}

impl TDFB for TGSetSupergroupDescription {}

impl AsRef<TGSetSupergroupDescription> for TGSetSupergroupDescription {
  fn as_ref(&self) -> &TGSetSupergroupDescription { self }
}

impl AsRef<TGSetSupergroupDescription> for _TGSetSupergroupDescriptionBuilder {
  fn as_ref(&self) -> &TGSetSupergroupDescription { &self.inner }
}

impl TGSetSupergroupDescription {

  pub fn builder() -> _TGSetSupergroupDescriptionBuilder {
    _TGSetSupergroupDescriptionBuilder { inner: Self::new(SetSupergroupDescription::_new()) }
  }

  pub fn new(inner: SetSupergroupDescription) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetSupergroupDescription { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetSupergroupDescription { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetSupergroupStickerSetBuilder { inner: TGSetSupergroupStickerSet }

impl _TGSetSupergroupStickerSetBuilder {

  pub fn build(&self) -> TGSetSupergroupStickerSet { self.inner.clone() }

  ///  Identifier of the supergroup. 
  pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_supergroup_id(supergroup_id);
    self
  }
  ///  New value of the supergroup sticker set identifier. Use 0 to remove the supergroup sticker set. 
  pub fn sticker_set_id(&mut self, sticker_set_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_sticker_set_id(sticker_set_id);
    self
  }
  

  
}


///  Changes the sticker set of a supergroup; requires appropriate rights in the supergroup. 
#[derive(Debug, Clone)]
pub struct TGSetSupergroupStickerSet {
  inner: SetSupergroupStickerSet
}

impl TDFB for TGSetSupergroupStickerSet {}

impl AsRef<TGSetSupergroupStickerSet> for TGSetSupergroupStickerSet {
  fn as_ref(&self) -> &TGSetSupergroupStickerSet { self }
}

impl AsRef<TGSetSupergroupStickerSet> for _TGSetSupergroupStickerSetBuilder {
  fn as_ref(&self) -> &TGSetSupergroupStickerSet { &self.inner }
}

impl TGSetSupergroupStickerSet {

  pub fn builder() -> _TGSetSupergroupStickerSetBuilder {
    _TGSetSupergroupStickerSetBuilder { inner: Self::new(SetSupergroupStickerSet::_new()) }
  }

  pub fn new(inner: SetSupergroupStickerSet) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetSupergroupStickerSet { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetSupergroupStickerSet { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetSupergroupUsernameBuilder { inner: TGSetSupergroupUsername }

impl _TGSetSupergroupUsernameBuilder {

  pub fn build(&self) -> TGSetSupergroupUsername { self.inner.clone() }

  ///  Identifier of the supergroup or channel. 
  pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_supergroup_id(supergroup_id);
    self
  }
  ///  New value of the username. Use an empty string to remove the username. 
  pub fn username<S: AsRef<str>>(&mut self, username: S) -> &mut Self {
    self.inner.td_origin_mut()._set_username(username.as_ref().to_string());
    self
  }
  

  
}


///  Changes the username of a supergroup or channel, requires creator privileges in the supergroup or channel. 
#[derive(Debug, Clone)]
pub struct TGSetSupergroupUsername {
  inner: SetSupergroupUsername
}

impl TDFB for TGSetSupergroupUsername {}

impl AsRef<TGSetSupergroupUsername> for TGSetSupergroupUsername {
  fn as_ref(&self) -> &TGSetSupergroupUsername { self }
}

impl AsRef<TGSetSupergroupUsername> for _TGSetSupergroupUsernameBuilder {
  fn as_ref(&self) -> &TGSetSupergroupUsername { &self.inner }
}

impl TGSetSupergroupUsername {

  pub fn builder() -> _TGSetSupergroupUsernameBuilder {
    _TGSetSupergroupUsernameBuilder { inner: Self::new(SetSupergroupUsername::_new()) }
  }

  pub fn new(inner: SetSupergroupUsername) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetSupergroupUsername { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetSupergroupUsername { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetTdlibParametersBuilder { inner: TGSetTdlibParameters }

impl _TGSetTdlibParametersBuilder {

  pub fn build(&self) -> TGSetTdlibParameters { self.inner.clone() }

  

  
  // [parameters] type is [TdlibParameters], is not support, need add manully.
  #[doc(hidden)] pub fn _parameters(&mut self, parameters: TdlibParameters) -> &mut Self {
    self.inner.td_origin_mut()._set_parameters(parameters);
    self
  }
  
}


///  Sets the parameters for TDLib initialization. Works only when the current authorization state is  
#[derive(Debug, Clone)]
pub struct TGSetTdlibParameters {
  inner: SetTdlibParameters
}

impl TDFB for TGSetTdlibParameters {}

impl AsRef<TGSetTdlibParameters> for TGSetTdlibParameters {
  fn as_ref(&self) -> &TGSetTdlibParameters { self }
}

impl AsRef<TGSetTdlibParameters> for _TGSetTdlibParametersBuilder {
  fn as_ref(&self) -> &TGSetTdlibParameters { &self.inner }
}

impl TGSetTdlibParameters {

  pub fn builder() -> _TGSetTdlibParametersBuilder {
    _TGSetTdlibParametersBuilder { inner: Self::new(SetTdlibParameters::_new()) }
  }

  pub fn new(inner: SetTdlibParameters) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetTdlibParameters { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetTdlibParameters { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetUsernameBuilder { inner: TGSetUsername }

impl _TGSetUsernameBuilder {

  pub fn build(&self) -> TGSetUsername { self.inner.clone() }

  ///  The new value of the username. Use an empty string to remove the username. 
  pub fn username<S: AsRef<str>>(&mut self, username: S) -> &mut Self {
    self.inner.td_origin_mut()._set_username(username.as_ref().to_string());
    self
  }
  

  
}


///  Changes the username of the current user. If something changes,  
#[derive(Debug, Clone)]
pub struct TGSetUsername {
  inner: SetUsername
}

impl TDFB for TGSetUsername {}

impl AsRef<TGSetUsername> for TGSetUsername {
  fn as_ref(&self) -> &TGSetUsername { self }
}

impl AsRef<TGSetUsername> for _TGSetUsernameBuilder {
  fn as_ref(&self) -> &TGSetUsername { &self.inner }
}

impl TGSetUsername {

  pub fn builder() -> _TGSetUsernameBuilder {
    _TGSetUsernameBuilder { inner: Self::new(SetUsername::_new()) }
  }

  pub fn new(inner: SetUsername) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetUsername { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetUsername { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSetUserPrivacySettingRulesBuilder { inner: TGSetUserPrivacySettingRules }

impl _TGSetUserPrivacySettingRulesBuilder {

  pub fn build(&self) -> TGSetUserPrivacySettingRules { self.inner.clone() }

  

  
  // [setting] type is [Box<UserPrivacySetting>], is not support, need add manully.
  #[doc(hidden)] pub fn _setting(&mut self, setting: Box<UserPrivacySetting>) -> &mut Self {
    self.inner.td_origin_mut()._set_setting(setting);
    self
  }
  
  // [rules] type is [UserPrivacySettingRules], is not support, need add manully.
  #[doc(hidden)] pub fn _rules(&mut self, rules: UserPrivacySettingRules) -> &mut Self {
    self.inner.td_origin_mut()._set_rules(rules);
    self
  }
  
}


///  Changes user privacy settings. 
#[derive(Debug, Clone)]
pub struct TGSetUserPrivacySettingRules {
  inner: SetUserPrivacySettingRules
}

impl TDFB for TGSetUserPrivacySettingRules {}

impl AsRef<TGSetUserPrivacySettingRules> for TGSetUserPrivacySettingRules {
  fn as_ref(&self) -> &TGSetUserPrivacySettingRules { self }
}

impl AsRef<TGSetUserPrivacySettingRules> for _TGSetUserPrivacySettingRulesBuilder {
  fn as_ref(&self) -> &TGSetUserPrivacySettingRules { &self.inner }
}

impl TGSetUserPrivacySettingRules {

  pub fn builder() -> _TGSetUserPrivacySettingRulesBuilder {
    _TGSetUserPrivacySettingRulesBuilder { inner: Self::new(SetUserPrivacySettingRules::_new()) }
  }

  pub fn new(inner: SetUserPrivacySettingRules) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SetUserPrivacySettingRules { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SetUserPrivacySettingRules { &mut self.inner }

}


#[doc(hidden)] pub struct _TGStopPollBuilder { inner: TGStopPoll }

impl _TGStopPollBuilder {

  pub fn build(&self) -> TGStopPoll { self.inner.clone() }

  ///  Identifier of the chat to which the poll belongs. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Identifier of the message containing the poll. 
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_message_id(message_id);
    self
  }
  

  
  // [reply_markup] type is [Box<ReplyMarkup>], is not support, need add manully.
  #[doc(hidden)] pub fn _reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self {
    self.inner.td_origin_mut()._set_reply_markup(reply_markup);
    self
  }
  
}


///  Stops a poll. A poll in a message can be stopped when the message has can_be_edited flag set. 
#[derive(Debug, Clone)]
pub struct TGStopPoll {
  inner: StopPoll
}

impl TDFB for TGStopPoll {}

impl AsRef<TGStopPoll> for TGStopPoll {
  fn as_ref(&self) -> &TGStopPoll { self }
}

impl AsRef<TGStopPoll> for _TGStopPollBuilder {
  fn as_ref(&self) -> &TGStopPoll { &self.inner }
}

impl TGStopPoll {

  pub fn builder() -> _TGStopPollBuilder {
    _TGStopPollBuilder { inner: Self::new(StopPoll::_new()) }
  }

  pub fn new(inner: StopPoll) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &StopPoll { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut StopPoll { &mut self.inner }

}


#[doc(hidden)] pub struct _TGSynchronizeLanguagePackBuilder { inner: TGSynchronizeLanguagePack }

impl _TGSynchronizeLanguagePackBuilder {

  pub fn build(&self) -> TGSynchronizeLanguagePack { self.inner.clone() }

  ///  Language pack identifier. 
  pub fn language_pack_id<S: AsRef<str>>(&mut self, language_pack_id: S) -> &mut Self {
    self.inner.td_origin_mut()._set_language_pack_id(language_pack_id.as_ref().to_string());
    self
  }
  

  
}


///  Fetches the latest versions of all strings from a language pack in the current localization target from the server. This method doesn't need to be called explicitly for the current used/base language packs. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGSynchronizeLanguagePack {
  inner: SynchronizeLanguagePack
}

impl TDFB for TGSynchronizeLanguagePack {}

impl AsRef<TGSynchronizeLanguagePack> for TGSynchronizeLanguagePack {
  fn as_ref(&self) -> &TGSynchronizeLanguagePack { self }
}

impl AsRef<TGSynchronizeLanguagePack> for _TGSynchronizeLanguagePackBuilder {
  fn as_ref(&self) -> &TGSynchronizeLanguagePack { &self.inner }
}

impl TGSynchronizeLanguagePack {

  pub fn builder() -> _TGSynchronizeLanguagePackBuilder {
    _TGSynchronizeLanguagePackBuilder { inner: Self::new(SynchronizeLanguagePack::_new()) }
  }

  pub fn new(inner: SynchronizeLanguagePack) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &SynchronizeLanguagePack { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut SynchronizeLanguagePack { &mut self.inner }

}


#[doc(hidden)] pub struct _TGTerminateAllOtherSessionsBuilder { inner: TGTerminateAllOtherSessions }

impl _TGTerminateAllOtherSessionsBuilder {

  pub fn build(&self) -> TGTerminateAllOtherSessions { self.inner.clone() }

  

  
}


///  Terminates all other sessions of the current user. 
#[derive(Debug, Clone)]
pub struct TGTerminateAllOtherSessions {
  inner: TerminateAllOtherSessions
}

impl TDFB for TGTerminateAllOtherSessions {}

impl AsRef<TGTerminateAllOtherSessions> for TGTerminateAllOtherSessions {
  fn as_ref(&self) -> &TGTerminateAllOtherSessions { self }
}

impl AsRef<TGTerminateAllOtherSessions> for _TGTerminateAllOtherSessionsBuilder {
  fn as_ref(&self) -> &TGTerminateAllOtherSessions { &self.inner }
}

impl TGTerminateAllOtherSessions {

  pub fn builder() -> _TGTerminateAllOtherSessionsBuilder {
    _TGTerminateAllOtherSessionsBuilder { inner: Self::new(TerminateAllOtherSessions::_new()) }
  }

  pub fn new(inner: TerminateAllOtherSessions) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &TerminateAllOtherSessions { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut TerminateAllOtherSessions { &mut self.inner }

}


#[doc(hidden)] pub struct _TGTerminateSessionBuilder { inner: TGTerminateSession }

impl _TGTerminateSessionBuilder {

  pub fn build(&self) -> TGTerminateSession { self.inner.clone() }

  ///  Session identifier. 
  pub fn session_id(&mut self, session_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_session_id(session_id);
    self
  }
  

  
}


///  Terminates a session of the current user. 
#[derive(Debug, Clone)]
pub struct TGTerminateSession {
  inner: TerminateSession
}

impl TDFB for TGTerminateSession {}

impl AsRef<TGTerminateSession> for TGTerminateSession {
  fn as_ref(&self) -> &TGTerminateSession { self }
}

impl AsRef<TGTerminateSession> for _TGTerminateSessionBuilder {
  fn as_ref(&self) -> &TGTerminateSession { &self.inner }
}

impl TGTerminateSession {

  pub fn builder() -> _TGTerminateSessionBuilder {
    _TGTerminateSessionBuilder { inner: Self::new(TerminateSession::_new()) }
  }

  pub fn new(inner: TerminateSession) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &TerminateSession { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut TerminateSession { &mut self.inner }

}


#[doc(hidden)] pub struct _TGTestCallBytesBuilder { inner: TGTestCallBytes }

impl _TGTestCallBytesBuilder {

  pub fn build(&self) -> TGTestCallBytes { self.inner.clone() }

  ///  Bytes to return. 
  pub fn x<S: AsRef<str>>(&mut self, x: S) -> &mut Self {
    self.inner.td_origin_mut()._set_x(x.as_ref().to_string());
    self
  }
  

  
}


///  Returns the received bytes; for testing only. This is an offline method. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGTestCallBytes {
  inner: TestCallBytes
}

impl TDFB for TGTestCallBytes {}

impl AsRef<TGTestCallBytes> for TGTestCallBytes {
  fn as_ref(&self) -> &TGTestCallBytes { self }
}

impl AsRef<TGTestCallBytes> for _TGTestCallBytesBuilder {
  fn as_ref(&self) -> &TGTestCallBytes { &self.inner }
}

impl TGTestCallBytes {

  pub fn builder() -> _TGTestCallBytesBuilder {
    _TGTestCallBytesBuilder { inner: Self::new(TestCallBytes::_new()) }
  }

  pub fn new(inner: TestCallBytes) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &TestCallBytes { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut TestCallBytes { &mut self.inner }

}


#[doc(hidden)] pub struct _TGTestCallEmptyBuilder { inner: TGTestCallEmpty }

impl _TGTestCallEmptyBuilder {

  pub fn build(&self) -> TGTestCallEmpty { self.inner.clone() }

  

  
}


///  Does nothing; for testing only. This is an offline method. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGTestCallEmpty {
  inner: TestCallEmpty
}

impl TDFB for TGTestCallEmpty {}

impl AsRef<TGTestCallEmpty> for TGTestCallEmpty {
  fn as_ref(&self) -> &TGTestCallEmpty { self }
}

impl AsRef<TGTestCallEmpty> for _TGTestCallEmptyBuilder {
  fn as_ref(&self) -> &TGTestCallEmpty { &self.inner }
}

impl TGTestCallEmpty {

  pub fn builder() -> _TGTestCallEmptyBuilder {
    _TGTestCallEmptyBuilder { inner: Self::new(TestCallEmpty::_new()) }
  }

  pub fn new(inner: TestCallEmpty) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &TestCallEmpty { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut TestCallEmpty { &mut self.inner }

}


#[doc(hidden)] pub struct _TGTestCallStringBuilder { inner: TGTestCallString }

impl _TGTestCallStringBuilder {

  pub fn build(&self) -> TGTestCallString { self.inner.clone() }

  ///  String to return. 
  pub fn x<S: AsRef<str>>(&mut self, x: S) -> &mut Self {
    self.inner.td_origin_mut()._set_x(x.as_ref().to_string());
    self
  }
  

  
}


///  Returns the received string; for testing only. This is an offline method. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGTestCallString {
  inner: TestCallString
}

impl TDFB for TGTestCallString {}

impl AsRef<TGTestCallString> for TGTestCallString {
  fn as_ref(&self) -> &TGTestCallString { self }
}

impl AsRef<TGTestCallString> for _TGTestCallStringBuilder {
  fn as_ref(&self) -> &TGTestCallString { &self.inner }
}

impl TGTestCallString {

  pub fn builder() -> _TGTestCallStringBuilder {
    _TGTestCallStringBuilder { inner: Self::new(TestCallString::_new()) }
  }

  pub fn new(inner: TestCallString) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &TestCallString { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut TestCallString { &mut self.inner }

}


#[doc(hidden)] pub struct _TGTestCallVectorIntBuilder { inner: TGTestCallVectorInt }

impl _TGTestCallVectorIntBuilder {

  pub fn build(&self) -> TGTestCallVectorInt { self.inner.clone() }

  ///  Vector of numbers to return. 
  pub fn x(&mut self, x: Vec<i32>) -> &mut Self {
    self.inner.td_origin_mut()._set_x(x);
    self
  }
  

  
}


///  Returns the received vector of numbers; for testing only. This is an offline method. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGTestCallVectorInt {
  inner: TestCallVectorInt
}

impl TDFB for TGTestCallVectorInt {}

impl AsRef<TGTestCallVectorInt> for TGTestCallVectorInt {
  fn as_ref(&self) -> &TGTestCallVectorInt { self }
}

impl AsRef<TGTestCallVectorInt> for _TGTestCallVectorIntBuilder {
  fn as_ref(&self) -> &TGTestCallVectorInt { &self.inner }
}

impl TGTestCallVectorInt {

  pub fn builder() -> _TGTestCallVectorIntBuilder {
    _TGTestCallVectorIntBuilder { inner: Self::new(TestCallVectorInt::_new()) }
  }

  pub fn new(inner: TestCallVectorInt) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &TestCallVectorInt { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut TestCallVectorInt { &mut self.inner }

}


#[doc(hidden)] pub struct _TGTestCallVectorIntObjectBuilder { inner: TGTestCallVectorIntObject }

impl _TGTestCallVectorIntObjectBuilder {

  pub fn build(&self) -> TGTestCallVectorIntObject { self.inner.clone() }

  

  
  // [x] type is [Vec<TestInt>], is not support, need add manully.
  #[doc(hidden)] pub fn _x(&mut self, x: Vec<TestInt>) -> &mut Self {
    self.inner.td_origin_mut()._set_x(x);
    self
  }
  
}


///  Returns the received vector of objects containing a number; for testing only. This is an offline method. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGTestCallVectorIntObject {
  inner: TestCallVectorIntObject
}

impl TDFB for TGTestCallVectorIntObject {}

impl AsRef<TGTestCallVectorIntObject> for TGTestCallVectorIntObject {
  fn as_ref(&self) -> &TGTestCallVectorIntObject { self }
}

impl AsRef<TGTestCallVectorIntObject> for _TGTestCallVectorIntObjectBuilder {
  fn as_ref(&self) -> &TGTestCallVectorIntObject { &self.inner }
}

impl TGTestCallVectorIntObject {

  pub fn builder() -> _TGTestCallVectorIntObjectBuilder {
    _TGTestCallVectorIntObjectBuilder { inner: Self::new(TestCallVectorIntObject::_new()) }
  }

  pub fn new(inner: TestCallVectorIntObject) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &TestCallVectorIntObject { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut TestCallVectorIntObject { &mut self.inner }

}


#[doc(hidden)] pub struct _TGTestCallVectorStringBuilder { inner: TGTestCallVectorString }

impl _TGTestCallVectorStringBuilder {

  pub fn build(&self) -> TGTestCallVectorString { self.inner.clone() }

  

  
  // [x] type is [Vec<String>], is not support, need add manully.
  #[doc(hidden)] pub fn _x(&mut self, x: Vec<String>) -> &mut Self {
    self.inner.td_origin_mut()._set_x(x);
    self
  }
  
}


///  Returns the received vector of strings; for testing only. This is an offline method. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGTestCallVectorString {
  inner: TestCallVectorString
}

impl TDFB for TGTestCallVectorString {}

impl AsRef<TGTestCallVectorString> for TGTestCallVectorString {
  fn as_ref(&self) -> &TGTestCallVectorString { self }
}

impl AsRef<TGTestCallVectorString> for _TGTestCallVectorStringBuilder {
  fn as_ref(&self) -> &TGTestCallVectorString { &self.inner }
}

impl TGTestCallVectorString {

  pub fn builder() -> _TGTestCallVectorStringBuilder {
    _TGTestCallVectorStringBuilder { inner: Self::new(TestCallVectorString::_new()) }
  }

  pub fn new(inner: TestCallVectorString) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &TestCallVectorString { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut TestCallVectorString { &mut self.inner }

}


#[doc(hidden)] pub struct _TGTestCallVectorStringObjectBuilder { inner: TGTestCallVectorStringObject }

impl _TGTestCallVectorStringObjectBuilder {

  pub fn build(&self) -> TGTestCallVectorStringObject { self.inner.clone() }

  

  
  // [x] type is [Vec<TestString>], is not support, need add manully.
  #[doc(hidden)] pub fn _x(&mut self, x: Vec<TestString>) -> &mut Self {
    self.inner.td_origin_mut()._set_x(x);
    self
  }
  
}


///  Returns the received vector of objects containing a string; for testing only. This is an offline method. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGTestCallVectorStringObject {
  inner: TestCallVectorStringObject
}

impl TDFB for TGTestCallVectorStringObject {}

impl AsRef<TGTestCallVectorStringObject> for TGTestCallVectorStringObject {
  fn as_ref(&self) -> &TGTestCallVectorStringObject { self }
}

impl AsRef<TGTestCallVectorStringObject> for _TGTestCallVectorStringObjectBuilder {
  fn as_ref(&self) -> &TGTestCallVectorStringObject { &self.inner }
}

impl TGTestCallVectorStringObject {

  pub fn builder() -> _TGTestCallVectorStringObjectBuilder {
    _TGTestCallVectorStringObjectBuilder { inner: Self::new(TestCallVectorStringObject::_new()) }
  }

  pub fn new(inner: TestCallVectorStringObject) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &TestCallVectorStringObject { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut TestCallVectorStringObject { &mut self.inner }

}


#[doc(hidden)] pub struct _TGTestGetDifferenceBuilder { inner: TGTestGetDifference }

impl _TGTestGetDifferenceBuilder {

  pub fn build(&self) -> TGTestGetDifference { self.inner.clone() }

  

  
}


///  Forces an updates.getDifference call to the Telegram servers; for testing only. 
#[derive(Debug, Clone)]
pub struct TGTestGetDifference {
  inner: TestGetDifference
}

impl TDFB for TGTestGetDifference {}

impl AsRef<TGTestGetDifference> for TGTestGetDifference {
  fn as_ref(&self) -> &TGTestGetDifference { self }
}

impl AsRef<TGTestGetDifference> for _TGTestGetDifferenceBuilder {
  fn as_ref(&self) -> &TGTestGetDifference { &self.inner }
}

impl TGTestGetDifference {

  pub fn builder() -> _TGTestGetDifferenceBuilder {
    _TGTestGetDifferenceBuilder { inner: Self::new(TestGetDifference::_new()) }
  }

  pub fn new(inner: TestGetDifference) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &TestGetDifference { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut TestGetDifference { &mut self.inner }

}


#[doc(hidden)] pub struct _TGTestNetworkBuilder { inner: TGTestNetwork }

impl _TGTestNetworkBuilder {

  pub fn build(&self) -> TGTestNetwork { self.inner.clone() }

  

  
}


///  Sends a simple network request to the Telegram servers; for testing only. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGTestNetwork {
  inner: TestNetwork
}

impl TDFB for TGTestNetwork {}

impl AsRef<TGTestNetwork> for TGTestNetwork {
  fn as_ref(&self) -> &TGTestNetwork { self }
}

impl AsRef<TGTestNetwork> for _TGTestNetworkBuilder {
  fn as_ref(&self) -> &TGTestNetwork { &self.inner }
}

impl TGTestNetwork {

  pub fn builder() -> _TGTestNetworkBuilder {
    _TGTestNetworkBuilder { inner: Self::new(TestNetwork::_new()) }
  }

  pub fn new(inner: TestNetwork) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &TestNetwork { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut TestNetwork { &mut self.inner }

}


#[doc(hidden)] pub struct _TGTestSquareIntBuilder { inner: TGTestSquareInt }

impl _TGTestSquareIntBuilder {

  pub fn build(&self) -> TGTestSquareInt { self.inner.clone() }

  ///  Number to square. 
  pub fn x(&mut self, x: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_x(x);
    self
  }
  

  
}


///  Returns the squared received number; for testing only. This is an offline method. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGTestSquareInt {
  inner: TestSquareInt
}

impl TDFB for TGTestSquareInt {}

impl AsRef<TGTestSquareInt> for TGTestSquareInt {
  fn as_ref(&self) -> &TGTestSquareInt { self }
}

impl AsRef<TGTestSquareInt> for _TGTestSquareIntBuilder {
  fn as_ref(&self) -> &TGTestSquareInt { &self.inner }
}

impl TGTestSquareInt {

  pub fn builder() -> _TGTestSquareIntBuilder {
    _TGTestSquareIntBuilder { inner: Self::new(TestSquareInt::_new()) }
  }

  pub fn new(inner: TestSquareInt) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &TestSquareInt { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut TestSquareInt { &mut self.inner }

}


#[doc(hidden)] pub struct _TGTestUseErrorBuilder { inner: TGTestUseError }

impl _TGTestUseErrorBuilder {

  pub fn build(&self) -> TGTestUseError { self.inner.clone() }

  

  
}


///  Does nothing and ensures that the Error object is used; for testing only. This is an offline method. Can be called before authorization. 
#[derive(Debug, Clone)]
pub struct TGTestUseError {
  inner: TestUseError
}

impl TDFB for TGTestUseError {}

impl AsRef<TGTestUseError> for TGTestUseError {
  fn as_ref(&self) -> &TGTestUseError { self }
}

impl AsRef<TGTestUseError> for _TGTestUseErrorBuilder {
  fn as_ref(&self) -> &TGTestUseError { &self.inner }
}

impl TGTestUseError {

  pub fn builder() -> _TGTestUseErrorBuilder {
    _TGTestUseErrorBuilder { inner: Self::new(TestUseError::_new()) }
  }

  pub fn new(inner: TestUseError) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &TestUseError { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut TestUseError { &mut self.inner }

}


#[doc(hidden)] pub struct _TGTestUseUpdateBuilder { inner: TGTestUseUpdate }

impl _TGTestUseUpdateBuilder {

  pub fn build(&self) -> TGTestUseUpdate { self.inner.clone() }

  

  
}


///  Does nothing and ensures that the  
#[derive(Debug, Clone)]
pub struct TGTestUseUpdate {
  inner: TestUseUpdate
}

impl TDFB for TGTestUseUpdate {}

impl AsRef<TGTestUseUpdate> for TGTestUseUpdate {
  fn as_ref(&self) -> &TGTestUseUpdate { self }
}

impl AsRef<TGTestUseUpdate> for _TGTestUseUpdateBuilder {
  fn as_ref(&self) -> &TGTestUseUpdate { &self.inner }
}

impl TGTestUseUpdate {

  pub fn builder() -> _TGTestUseUpdateBuilder {
    _TGTestUseUpdateBuilder { inner: Self::new(TestUseUpdate::_new()) }
  }

  pub fn new(inner: TestUseUpdate) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &TestUseUpdate { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut TestUseUpdate { &mut self.inner }

}


#[doc(hidden)] pub struct _TGToggleBasicGroupAdministratorsBuilder { inner: TGToggleBasicGroupAdministrators }

impl _TGToggleBasicGroupAdministratorsBuilder {

  pub fn build(&self) -> TGToggleBasicGroupAdministrators { self.inner.clone() }

  ///  Identifier of the basic group. 
  pub fn basic_group_id(&mut self, basic_group_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_basic_group_id(basic_group_id);
    self
  }
  ///  New value of everyone_is_administrator. 
  pub fn everyone_is_administrator(&mut self, everyone_is_administrator: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_everyone_is_administrator(everyone_is_administrator);
    self
  }
  

  
}


///  Toggles the "All members are admins" setting in basic groups; requires creator privileges in the group. 
#[derive(Debug, Clone)]
pub struct TGToggleBasicGroupAdministrators {
  inner: ToggleBasicGroupAdministrators
}

impl TDFB for TGToggleBasicGroupAdministrators {}

impl AsRef<TGToggleBasicGroupAdministrators> for TGToggleBasicGroupAdministrators {
  fn as_ref(&self) -> &TGToggleBasicGroupAdministrators { self }
}

impl AsRef<TGToggleBasicGroupAdministrators> for _TGToggleBasicGroupAdministratorsBuilder {
  fn as_ref(&self) -> &TGToggleBasicGroupAdministrators { &self.inner }
}

impl TGToggleBasicGroupAdministrators {

  pub fn builder() -> _TGToggleBasicGroupAdministratorsBuilder {
    _TGToggleBasicGroupAdministratorsBuilder { inner: Self::new(ToggleBasicGroupAdministrators::_new()) }
  }

  pub fn new(inner: ToggleBasicGroupAdministrators) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ToggleBasicGroupAdministrators { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ToggleBasicGroupAdministrators { &mut self.inner }

}


#[doc(hidden)] pub struct _TGToggleChatDefaultDisableNotificationBuilder { inner: TGToggleChatDefaultDisableNotification }

impl _TGToggleChatDefaultDisableNotificationBuilder {

  pub fn build(&self) -> TGToggleChatDefaultDisableNotification { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  New value of default_disable_notification. 
  pub fn default_disable_notification(&mut self, default_disable_notification: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_default_disable_notification(default_disable_notification);
    self
  }
  

  
}


///  Changes the value of the default disable_notification parameter, used when a message is sent to a chat. 
#[derive(Debug, Clone)]
pub struct TGToggleChatDefaultDisableNotification {
  inner: ToggleChatDefaultDisableNotification
}

impl TDFB for TGToggleChatDefaultDisableNotification {}

impl AsRef<TGToggleChatDefaultDisableNotification> for TGToggleChatDefaultDisableNotification {
  fn as_ref(&self) -> &TGToggleChatDefaultDisableNotification { self }
}

impl AsRef<TGToggleChatDefaultDisableNotification> for _TGToggleChatDefaultDisableNotificationBuilder {
  fn as_ref(&self) -> &TGToggleChatDefaultDisableNotification { &self.inner }
}

impl TGToggleChatDefaultDisableNotification {

  pub fn builder() -> _TGToggleChatDefaultDisableNotificationBuilder {
    _TGToggleChatDefaultDisableNotificationBuilder { inner: Self::new(ToggleChatDefaultDisableNotification::_new()) }
  }

  pub fn new(inner: ToggleChatDefaultDisableNotification) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ToggleChatDefaultDisableNotification { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ToggleChatDefaultDisableNotification { &mut self.inner }

}


#[doc(hidden)] pub struct _TGToggleChatIsMarkedAsUnreadBuilder { inner: TGToggleChatIsMarkedAsUnread }

impl _TGToggleChatIsMarkedAsUnreadBuilder {

  pub fn build(&self) -> TGToggleChatIsMarkedAsUnread { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  New value of is_marked_as_unread. 
  pub fn is_marked_as_unread(&mut self, is_marked_as_unread: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_is_marked_as_unread(is_marked_as_unread);
    self
  }
  

  
}


///  Changes the marked as unread state of a chat. 
#[derive(Debug, Clone)]
pub struct TGToggleChatIsMarkedAsUnread {
  inner: ToggleChatIsMarkedAsUnread
}

impl TDFB for TGToggleChatIsMarkedAsUnread {}

impl AsRef<TGToggleChatIsMarkedAsUnread> for TGToggleChatIsMarkedAsUnread {
  fn as_ref(&self) -> &TGToggleChatIsMarkedAsUnread { self }
}

impl AsRef<TGToggleChatIsMarkedAsUnread> for _TGToggleChatIsMarkedAsUnreadBuilder {
  fn as_ref(&self) -> &TGToggleChatIsMarkedAsUnread { &self.inner }
}

impl TGToggleChatIsMarkedAsUnread {

  pub fn builder() -> _TGToggleChatIsMarkedAsUnreadBuilder {
    _TGToggleChatIsMarkedAsUnreadBuilder { inner: Self::new(ToggleChatIsMarkedAsUnread::_new()) }
  }

  pub fn new(inner: ToggleChatIsMarkedAsUnread) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ToggleChatIsMarkedAsUnread { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ToggleChatIsMarkedAsUnread { &mut self.inner }

}


#[doc(hidden)] pub struct _TGToggleChatIsPinnedBuilder { inner: TGToggleChatIsPinned }

impl _TGToggleChatIsPinnedBuilder {

  pub fn build(&self) -> TGToggleChatIsPinned { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  New value of is_pinned. 
  pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_is_pinned(is_pinned);
    self
  }
  

  
}


///  Changes the pinned state of a chat. You can pin up to GetOption("pinned_chat_count_max") non-secret chats and the same number of secret chats. 
#[derive(Debug, Clone)]
pub struct TGToggleChatIsPinned {
  inner: ToggleChatIsPinned
}

impl TDFB for TGToggleChatIsPinned {}

impl AsRef<TGToggleChatIsPinned> for TGToggleChatIsPinned {
  fn as_ref(&self) -> &TGToggleChatIsPinned { self }
}

impl AsRef<TGToggleChatIsPinned> for _TGToggleChatIsPinnedBuilder {
  fn as_ref(&self) -> &TGToggleChatIsPinned { &self.inner }
}

impl TGToggleChatIsPinned {

  pub fn builder() -> _TGToggleChatIsPinnedBuilder {
    _TGToggleChatIsPinnedBuilder { inner: Self::new(ToggleChatIsPinned::_new()) }
  }

  pub fn new(inner: ToggleChatIsPinned) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ToggleChatIsPinned { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ToggleChatIsPinned { &mut self.inner }

}


#[doc(hidden)] pub struct _TGToggleSupergroupInvitesBuilder { inner: TGToggleSupergroupInvites }

impl _TGToggleSupergroupInvitesBuilder {

  pub fn build(&self) -> TGToggleSupergroupInvites { self.inner.clone() }

  ///  Identifier of the supergroup. 
  pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_supergroup_id(supergroup_id);
    self
  }
  ///  New value of anyone_can_invite. 
  pub fn anyone_can_invite(&mut self, anyone_can_invite: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_anyone_can_invite(anyone_can_invite);
    self
  }
  

  
}


///  Toggles whether all members of a supergroup can add new members; requires appropriate administrator rights in the supergroup. 
#[derive(Debug, Clone)]
pub struct TGToggleSupergroupInvites {
  inner: ToggleSupergroupInvites
}

impl TDFB for TGToggleSupergroupInvites {}

impl AsRef<TGToggleSupergroupInvites> for TGToggleSupergroupInvites {
  fn as_ref(&self) -> &TGToggleSupergroupInvites { self }
}

impl AsRef<TGToggleSupergroupInvites> for _TGToggleSupergroupInvitesBuilder {
  fn as_ref(&self) -> &TGToggleSupergroupInvites { &self.inner }
}

impl TGToggleSupergroupInvites {

  pub fn builder() -> _TGToggleSupergroupInvitesBuilder {
    _TGToggleSupergroupInvitesBuilder { inner: Self::new(ToggleSupergroupInvites::_new()) }
  }

  pub fn new(inner: ToggleSupergroupInvites) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ToggleSupergroupInvites { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ToggleSupergroupInvites { &mut self.inner }

}


#[doc(hidden)] pub struct _TGToggleSupergroupIsAllHistoryAvailableBuilder { inner: TGToggleSupergroupIsAllHistoryAvailable }

impl _TGToggleSupergroupIsAllHistoryAvailableBuilder {

  pub fn build(&self) -> TGToggleSupergroupIsAllHistoryAvailable { self.inner.clone() }

  ///  The identifier of the supergroup. 
  pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_supergroup_id(supergroup_id);
    self
  }
  ///  The new value of is_all_history_available. 
  pub fn is_all_history_available(&mut self, is_all_history_available: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_is_all_history_available(is_all_history_available);
    self
  }
  

  
}


///  Toggles whether the message history of a supergroup is available to new members; requires appropriate administrator rights in the supergroup. 
#[derive(Debug, Clone)]
pub struct TGToggleSupergroupIsAllHistoryAvailable {
  inner: ToggleSupergroupIsAllHistoryAvailable
}

impl TDFB for TGToggleSupergroupIsAllHistoryAvailable {}

impl AsRef<TGToggleSupergroupIsAllHistoryAvailable> for TGToggleSupergroupIsAllHistoryAvailable {
  fn as_ref(&self) -> &TGToggleSupergroupIsAllHistoryAvailable { self }
}

impl AsRef<TGToggleSupergroupIsAllHistoryAvailable> for _TGToggleSupergroupIsAllHistoryAvailableBuilder {
  fn as_ref(&self) -> &TGToggleSupergroupIsAllHistoryAvailable { &self.inner }
}

impl TGToggleSupergroupIsAllHistoryAvailable {

  pub fn builder() -> _TGToggleSupergroupIsAllHistoryAvailableBuilder {
    _TGToggleSupergroupIsAllHistoryAvailableBuilder { inner: Self::new(ToggleSupergroupIsAllHistoryAvailable::_new()) }
  }

  pub fn new(inner: ToggleSupergroupIsAllHistoryAvailable) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ToggleSupergroupIsAllHistoryAvailable { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ToggleSupergroupIsAllHistoryAvailable { &mut self.inner }

}


#[doc(hidden)] pub struct _TGToggleSupergroupSignMessagesBuilder { inner: TGToggleSupergroupSignMessages }

impl _TGToggleSupergroupSignMessagesBuilder {

  pub fn build(&self) -> TGToggleSupergroupSignMessages { self.inner.clone() }

  ///  Identifier of the channel. 
  pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_supergroup_id(supergroup_id);
    self
  }
  ///  New value of sign_messages. 
  pub fn sign_messages(&mut self, sign_messages: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_sign_messages(sign_messages);
    self
  }
  

  
}


///  Toggles sender signatures messages sent in a channel; requires appropriate administrator rights in the channel. 
#[derive(Debug, Clone)]
pub struct TGToggleSupergroupSignMessages {
  inner: ToggleSupergroupSignMessages
}

impl TDFB for TGToggleSupergroupSignMessages {}

impl AsRef<TGToggleSupergroupSignMessages> for TGToggleSupergroupSignMessages {
  fn as_ref(&self) -> &TGToggleSupergroupSignMessages { self }
}

impl AsRef<TGToggleSupergroupSignMessages> for _TGToggleSupergroupSignMessagesBuilder {
  fn as_ref(&self) -> &TGToggleSupergroupSignMessages { &self.inner }
}

impl TGToggleSupergroupSignMessages {

  pub fn builder() -> _TGToggleSupergroupSignMessagesBuilder {
    _TGToggleSupergroupSignMessagesBuilder { inner: Self::new(ToggleSupergroupSignMessages::_new()) }
  }

  pub fn new(inner: ToggleSupergroupSignMessages) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ToggleSupergroupSignMessages { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ToggleSupergroupSignMessages { &mut self.inner }

}


#[doc(hidden)] pub struct _TGUnblockUserBuilder { inner: TGUnblockUser }

impl _TGUnblockUserBuilder {

  pub fn build(&self) -> TGUnblockUser { self.inner.clone() }

  ///  User identifier. 
  pub fn user_id(&mut self, user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_user_id(user_id);
    self
  }
  

  
}


///  Removes a user from the blacklist. 
#[derive(Debug, Clone)]
pub struct TGUnblockUser {
  inner: UnblockUser
}

impl TDFB for TGUnblockUser {}

impl AsRef<TGUnblockUser> for TGUnblockUser {
  fn as_ref(&self) -> &TGUnblockUser { self }
}

impl AsRef<TGUnblockUser> for _TGUnblockUserBuilder {
  fn as_ref(&self) -> &TGUnblockUser { &self.inner }
}

impl TGUnblockUser {

  pub fn builder() -> _TGUnblockUserBuilder {
    _TGUnblockUserBuilder { inner: Self::new(UnblockUser::_new()) }
  }

  pub fn new(inner: UnblockUser) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &UnblockUser { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut UnblockUser { &mut self.inner }

}


#[doc(hidden)] pub struct _TGUnpinChatMessageBuilder { inner: TGUnpinChatMessage }

impl _TGUnpinChatMessageBuilder {

  pub fn build(&self) -> TGUnpinChatMessage { self.inner.clone() }

  ///  Identifier of the chat. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  

  
}


///  Removes the pinned message from a chat; requires appropriate administrator rights in the group or channel. 
#[derive(Debug, Clone)]
pub struct TGUnpinChatMessage {
  inner: UnpinChatMessage
}

impl TDFB for TGUnpinChatMessage {}

impl AsRef<TGUnpinChatMessage> for TGUnpinChatMessage {
  fn as_ref(&self) -> &TGUnpinChatMessage { self }
}

impl AsRef<TGUnpinChatMessage> for _TGUnpinChatMessageBuilder {
  fn as_ref(&self) -> &TGUnpinChatMessage { &self.inner }
}

impl TGUnpinChatMessage {

  pub fn builder() -> _TGUnpinChatMessageBuilder {
    _TGUnpinChatMessageBuilder { inner: Self::new(UnpinChatMessage::_new()) }
  }

  pub fn new(inner: UnpinChatMessage) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &UnpinChatMessage { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut UnpinChatMessage { &mut self.inner }

}


#[doc(hidden)] pub struct _TGUpgradeBasicGroupChatToSupergroupChatBuilder { inner: TGUpgradeBasicGroupChatToSupergroupChat }

impl _TGUpgradeBasicGroupChatToSupergroupChatBuilder {

  pub fn build(&self) -> TGUpgradeBasicGroupChatToSupergroupChat { self.inner.clone() }

  ///  Identifier of the chat to upgrade. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  

  
}


///  Creates a new supergroup from an existing basic group and sends a corresponding  
#[derive(Debug, Clone)]
pub struct TGUpgradeBasicGroupChatToSupergroupChat {
  inner: UpgradeBasicGroupChatToSupergroupChat
}

impl TDFB for TGUpgradeBasicGroupChatToSupergroupChat {}

impl AsRef<TGUpgradeBasicGroupChatToSupergroupChat> for TGUpgradeBasicGroupChatToSupergroupChat {
  fn as_ref(&self) -> &TGUpgradeBasicGroupChatToSupergroupChat { self }
}

impl AsRef<TGUpgradeBasicGroupChatToSupergroupChat> for _TGUpgradeBasicGroupChatToSupergroupChatBuilder {
  fn as_ref(&self) -> &TGUpgradeBasicGroupChatToSupergroupChat { &self.inner }
}

impl TGUpgradeBasicGroupChatToSupergroupChat {

  pub fn builder() -> _TGUpgradeBasicGroupChatToSupergroupChatBuilder {
    _TGUpgradeBasicGroupChatToSupergroupChatBuilder { inner: Self::new(UpgradeBasicGroupChatToSupergroupChat::_new()) }
  }

  pub fn new(inner: UpgradeBasicGroupChatToSupergroupChat) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &UpgradeBasicGroupChatToSupergroupChat { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut UpgradeBasicGroupChatToSupergroupChat { &mut self.inner }

}


#[doc(hidden)] pub struct _TGUploadFileBuilder { inner: TGUploadFile }

impl _TGUploadFileBuilder {

  pub fn build(&self) -> TGUploadFile { self.inner.clone() }

  ///  Priority of the upload (1-32). The higher the priority, the earlier the file will be uploaded. If the priorities of two files are equal, then the first one for which uploadFile was called will be uploaded first. 
  pub fn priority(&mut self, priority: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_priority(priority);
    self
  }
  

  
  // [file] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _file(&mut self, file: Box<InputFile>) -> &mut Self {
    self.inner.td_origin_mut()._set_file(file);
    self
  }
  
  // [file_type] type is [Box<FileType>], is not support, need add manully.
  #[doc(hidden)] pub fn _file_type(&mut self, file_type: Box<FileType>) -> &mut Self {
    self.inner.td_origin_mut()._set_file_type(file_type);
    self
  }
  
}


///  Asynchronously uploads a file to the cloud without sending it in a message.  
#[derive(Debug, Clone)]
pub struct TGUploadFile {
  inner: UploadFile
}

impl TDFB for TGUploadFile {}

impl AsRef<TGUploadFile> for TGUploadFile {
  fn as_ref(&self) -> &TGUploadFile { self }
}

impl AsRef<TGUploadFile> for _TGUploadFileBuilder {
  fn as_ref(&self) -> &TGUploadFile { &self.inner }
}

impl TGUploadFile {

  pub fn builder() -> _TGUploadFileBuilder {
    _TGUploadFileBuilder { inner: Self::new(UploadFile::_new()) }
  }

  pub fn new(inner: UploadFile) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &UploadFile { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut UploadFile { &mut self.inner }

}


#[doc(hidden)] pub struct _TGUploadStickerFileBuilder { inner: TGUploadStickerFile }

impl _TGUploadStickerFileBuilder {

  pub fn build(&self) -> TGUploadStickerFile { self.inner.clone() }

  ///  Sticker file owner. 
  pub fn user_id(&mut self, user_id: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_user_id(user_id);
    self
  }
  

  
  // [png_sticker] type is [Box<InputFile>], is not support, need add manully.
  #[doc(hidden)] pub fn _png_sticker(&mut self, png_sticker: Box<InputFile>) -> &mut Self {
    self.inner.td_origin_mut()._set_png_sticker(png_sticker);
    self
  }
  
}


///  Uploads a PNG image with a sticker; for bots only; returns the uploaded file. 
#[derive(Debug, Clone)]
pub struct TGUploadStickerFile {
  inner: UploadStickerFile
}

impl TDFB for TGUploadStickerFile {}

impl AsRef<TGUploadStickerFile> for TGUploadStickerFile {
  fn as_ref(&self) -> &TGUploadStickerFile { self }
}

impl AsRef<TGUploadStickerFile> for _TGUploadStickerFileBuilder {
  fn as_ref(&self) -> &TGUploadStickerFile { &self.inner }
}

impl TGUploadStickerFile {

  pub fn builder() -> _TGUploadStickerFileBuilder {
    _TGUploadStickerFileBuilder { inner: Self::new(UploadStickerFile::_new()) }
  }

  pub fn new(inner: UploadStickerFile) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &UploadStickerFile { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut UploadStickerFile { &mut self.inner }

}


#[doc(hidden)] pub struct _TGValidateOrderInfoBuilder { inner: TGValidateOrderInfo }

impl _TGValidateOrderInfoBuilder {

  pub fn build(&self) -> TGValidateOrderInfo { self.inner.clone() }

  ///  Chat identifier of the Invoice message. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  Message identifier. 
  pub fn message_id(&mut self, message_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_message_id(message_id);
    self
  }
  ///  True, if the order information can be saved. 
  pub fn allow_save(&mut self, allow_save: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_allow_save(allow_save);
    self
  }
  

  
  // [order_info] type is [OrderInfo], is not support, need add manully.
  #[doc(hidden)] pub fn _order_info(&mut self, order_info: OrderInfo) -> &mut Self {
    self.inner.td_origin_mut()._set_order_info(order_info);
    self
  }
  
}


///  Validates the order information provided by a user and returns the available shipping options for a flexible invoice. 
#[derive(Debug, Clone)]
pub struct TGValidateOrderInfo {
  inner: ValidateOrderInfo
}

impl TDFB for TGValidateOrderInfo {}

impl AsRef<TGValidateOrderInfo> for TGValidateOrderInfo {
  fn as_ref(&self) -> &TGValidateOrderInfo { self }
}

impl AsRef<TGValidateOrderInfo> for _TGValidateOrderInfoBuilder {
  fn as_ref(&self) -> &TGValidateOrderInfo { &self.inner }
}

impl TGValidateOrderInfo {

  pub fn builder() -> _TGValidateOrderInfoBuilder {
    _TGValidateOrderInfoBuilder { inner: Self::new(ValidateOrderInfo::_new()) }
  }

  pub fn new(inner: ValidateOrderInfo) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ValidateOrderInfo { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ValidateOrderInfo { &mut self.inner }

}


#[doc(hidden)] pub struct _TGViewMessagesBuilder { inner: TGViewMessages }

impl _TGViewMessagesBuilder {

  pub fn build(&self) -> TGViewMessages { self.inner.clone() }

  ///  Chat identifier. 
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_chat_id(chat_id);
    self
  }
  ///  The identifiers of the messages being viewed. 
  pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self {
    self.inner.td_origin_mut()._set_message_ids(message_ids);
    self
  }
  ///  True, if messages in closed chats should be marked as read. 
  pub fn force_read(&mut self, force_read: bool) -> &mut Self {
    self.inner.td_origin_mut()._set_force_read(force_read);
    self
  }
  

  
}


///  Informs TDLib that messages are being viewed by the user. Many useful activities depend on whether the messages are currently being viewed or not (e.g., marking messages as read, incrementing a view counter, updating a view counter, removing deleted messages in supergroups and channels). 
#[derive(Debug, Clone)]
pub struct TGViewMessages {
  inner: ViewMessages
}

impl TDFB for TGViewMessages {}

impl AsRef<TGViewMessages> for TGViewMessages {
  fn as_ref(&self) -> &TGViewMessages { self }
}

impl AsRef<TGViewMessages> for _TGViewMessagesBuilder {
  fn as_ref(&self) -> &TGViewMessages { &self.inner }
}

impl TGViewMessages {

  pub fn builder() -> _TGViewMessagesBuilder {
    _TGViewMessagesBuilder { inner: Self::new(ViewMessages::_new()) }
  }

  pub fn new(inner: ViewMessages) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ViewMessages { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ViewMessages { &mut self.inner }

}


#[doc(hidden)] pub struct _TGViewTrendingStickerSetsBuilder { inner: TGViewTrendingStickerSets }

impl _TGViewTrendingStickerSetsBuilder {

  pub fn build(&self) -> TGViewTrendingStickerSets { self.inner.clone() }

  ///  Identifiers of viewed trending sticker sets. 
  pub fn sticker_set_ids(&mut self, sticker_set_ids: Vec<i64>) -> &mut Self {
    self.inner.td_origin_mut()._set_sticker_set_ids(sticker_set_ids);
    self
  }
  

  
}


///  Informs the server that some trending sticker sets have been viewed by the user. 
#[derive(Debug, Clone)]
pub struct TGViewTrendingStickerSets {
  inner: ViewTrendingStickerSets
}

impl TDFB for TGViewTrendingStickerSets {}

impl AsRef<TGViewTrendingStickerSets> for TGViewTrendingStickerSets {
  fn as_ref(&self) -> &TGViewTrendingStickerSets { self }
}

impl AsRef<TGViewTrendingStickerSets> for _TGViewTrendingStickerSetsBuilder {
  fn as_ref(&self) -> &TGViewTrendingStickerSets { &self.inner }
}

impl TGViewTrendingStickerSets {

  pub fn builder() -> _TGViewTrendingStickerSetsBuilder {
    _TGViewTrendingStickerSetsBuilder { inner: Self::new(ViewTrendingStickerSets::_new()) }
  }

  pub fn new(inner: ViewTrendingStickerSets) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &ViewTrendingStickerSets { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut ViewTrendingStickerSets { &mut self.inner }

}


#[doc(hidden)] pub struct _TGWriteGeneratedFilePartBuilder { inner: TGWriteGeneratedFilePart }

impl _TGWriteGeneratedFilePartBuilder {

  pub fn build(&self) -> TGWriteGeneratedFilePart { self.inner.clone() }

  ///  The identifier of the generation process. 
  pub fn generation_id(&mut self, generation_id: i64) -> &mut Self {
    self.inner.td_origin_mut()._set_generation_id(generation_id);
    self
  }
  ///  The offset from which to write the data to the file. 
  pub fn offset(&mut self, offset: i32) -> &mut Self {
    self.inner.td_origin_mut()._set_offset(offset);
    self
  }
  ///  The data to write. 
  pub fn data<S: AsRef<str>>(&mut self, data: S) -> &mut Self {
    self.inner.td_origin_mut()._set_data(data.as_ref().to_string());
    self
  }
  

  
}


///  Writes a part of a generated file. This method is intended to be used only if the client has no direct access to TDLib's file system, because it is usually slower than a direct write to the destination file. 
#[derive(Debug, Clone)]
pub struct TGWriteGeneratedFilePart {
  inner: WriteGeneratedFilePart
}

impl TDFB for TGWriteGeneratedFilePart {}

impl AsRef<TGWriteGeneratedFilePart> for TGWriteGeneratedFilePart {
  fn as_ref(&self) -> &TGWriteGeneratedFilePart { self }
}

impl AsRef<TGWriteGeneratedFilePart> for _TGWriteGeneratedFilePartBuilder {
  fn as_ref(&self) -> &TGWriteGeneratedFilePart { &self.inner }
}

impl TGWriteGeneratedFilePart {

  pub fn builder() -> _TGWriteGeneratedFilePartBuilder {
    _TGWriteGeneratedFilePartBuilder { inner: Self::new(WriteGeneratedFilePart::_new()) }
  }

  pub fn new(inner: WriteGeneratedFilePart) -> Self {
    Self { inner }
  }

  pub fn td_origin(&self) -> &WriteGeneratedFilePart { &self.inner }

  pub fn td_origin_mut(&mut self) -> &mut WriteGeneratedFilePart { &mut self.inner }

}

