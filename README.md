telegram-client
===

Telegram client for rust.

This crate use libtdjson call telegram client api, libtdjson create is [rtdlib](https://crates.io/crates/rtdlib)

> **Notice**
> 
> Currently, not all features are available. about how to support it, see [Advanced](#Advanced)


# Usage

```toml
telegram-client="0.1"
```

# Examples

```rust
fn main() {
  let api = Api::default();
  let mut client = Client::new(api.clone());
  let listener = client.listener();

  listener.on_receive(|(api, object)| {
    println!("receive {:?} => {}", object.td_type(), object.to_json());
  });

  client.daemon("telegram-rs");
}
```

more [examples](./examples)

# Advanced

Because telegram client has a lot of events, this crate may not contain all the event handling code. If found will print a WARN level logger. Guide you to submit an issue.

In addition, you have at least two ways to solve this problem.

## on_receive

When you get `listener` from Client, you can use `listener` add `on_receive` to handle any data from tdlib.

```rust
fn main() {
  let mut client = Client::default();
  let listener = client.listener();
  
  listener.on_receive(|(api: &Api, object: &Box<rtdlib::types::Object>)| {
    let td_type: rtdlib::types::RTDType = object.td_type();
    match td_type {
      rtdlib::types::RTDType::UpdateUser => {
        rtdlib::types::UpdateUser::from_json(object.to_json())
          .map(|update_user: rtdlib::types::UpdateUser| {
            // do some thing
          });
      }
      _ => {}
    }
  });
}
```

`RTDType` is an enum, include all tdlib types. and `on_receive` as long as the callback is called after receiving the data.

Then combine rtdtype and receive to handle all events


## custom event

The code for this crate is designed to be configurable. You can check [build/tpl](build/tpl) directory to see the configuration file.

### gen_listener

Let's take a look at the [listener_rs.tpl.toml](build/tpl/listener_rs.tpl.toml) file.

```toml
[info]

uses = [
  "rtdlib::types as td_types",
  "crate::types as tg_types",
  "crate::api::Api",
]

comment_listener = "Telegram client event listener"
comment_lout = "Get listener"


# tt: listen handle type
# mapper: map tdlib type
# comment: listenr comment

[lin]

receive = { tt = { object = "Box<td_types::Object>" }, comment = "when receive data from tdlib" }

[lin.option]
td_type = "updateOption"
tt = { namespace = "tg_types", object = "TGUpdateOption"}
comment = "An option changed its value."

[lin.authorization_state]
td_type = "updateAuthorizationState"
tt = { namespace = "tg_types", object = "TGAuthorizationState" }
comment = "The user authorization state has changed."
```

This config will be generated `src/listener.rs` and `src/handler/handler_receive.rs` file.

Listen.rs will provide the user with the registration of the event,

- `[lin.name]` name is the name of the event
- `td_type` match the event corresponding to tdlib
- `tt` is the type of parameter that the event should return, this type should Is a struct that implements `rtdlib::types::Object`
- `comment` comment of this listener

Generated `src/listener.rs` like this:

```rust
use std::sync::Arc;

use rtdlib::types as td_types;
use crate::types as tg_types;
use crate::api::Api;

/// Telegram client event listener
#[derive(Clone)]
pub struct Listener {
  l_authorization_state: Option<Arc<Fn((&Api, &tg_types::TGAuthorizationState)) + Send + Sync + 'static>>,
  l_option: Option<Arc<Fn((&Api, &tg_types::TGUpdateOption)) + Send + Sync + 'static>>,
}

impl Listener {
  pub fn new() -> Self {
    Self {
      l_authorization_state: None,
      l_option: None,
    }
  }

  /// when receive data from tdlib
  pub fn on_receive<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&Api, &Box<td_types::Object>)) + Send + Sync + 'static {
    self.l_receive = Some(Arc::new(fnc));
    self
  }
  /// The user authorization state has changed.
  pub fn on_authorization_state<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&Api, &tg_types::TGAuthorizationState)) + Send + Sync + 'static {
    self.l_authorization_state = Some(Arc::new(fnc));
    self
  }
  /// An option changed its value.
  pub fn on_option<F>(&mut self, fnc: F) -> &mut Self where F: Fn((&Api, &tg_types::TGUpdateOption)) + Send + Sync + 'static {
    self.l_option = Some(Arc::new(fnc));
    self
  }
}
```

And then you can use this listener

```rust
fn main() {
  let api = Api::default();
  let mut client = Client::new(api.clone());
  let listener = client.listener();

  listener.on_option(|(api, option)| {
    let value = option.value();
    if value.is_empty() { debug!(exmlog::examples(), "Receive an option {} but it's empty", option.name()) }
    if value.is_string() { debug!(exmlog::examples(), "Receive an option {}: String => {}", option.name(), value.as_string().map_or("None".to_string(), |v| v)) }
    if value.is_integer() { debug!(exmlog::examples(), "Receive an option {}: i32 => {}", option.name(), value.as_integer().map_or(-1, |v| v)) }
    if value.is_bool() { debug!(exmlog::examples(), "Receive an option {}: bool => {}", option.name(), value.as_bool().map_or(false, |v| v)) }

    option.on_name("version", |value| {
      value.as_string().map(|v| { debug!(exmlog::examples(), "VERSION IS {}", v); });
    });
  });

  client.daemon("telegram-rs");
}
```

### gen_types

`rtdtype` has many primitive types, if want to use it better, you can remodel it with gen_types.


```toml
[tmod]

uses = [
  "self::f_update_option::TGOptionValue",
  "self::f_message_content::*",
]

[[tmod.mods]]
name = "tg_macro"
macro_use = true


[tgypes]

## if typen not set, default is TG$inner

[[tgypes.update_option]]
uses = []
typen = "TGUpdateOption"
inner = "UpdateOption"
comment = "An option changed its value."

[[tgypes.message_content]]
inner = "VoiceNote"

[[tgypes.message_content]]
inner = "Venue"
```

You can edit [tg_types.tpl.toml](build/tpl/tg_types.tpl.toml), `tmod` will generate the `mod.rs` file, `tgypes` will generate `t_*.rs` and `f_*.rs`.


`t_*.rs` will be rebuilt each time, `f_*.rs` will only be built once, `f_*.rs` is suitable for writing supplementary code.


The above configuration will generate the following code


**src/types/mod.rs**

```rust
pub use self::t_update_option::*;
pub use self::f_update_option::TGOptionValue;
pub use self::t_message_content::*;
pub use self::f_message_content::*;

#[macro_use] mod tg_macro;

mod t_update_option;
mod f_update_option;
mod t_message_content;
mod f_message_content;
```

**src/types/t_update_option.rs**

```rust
use rtdlib::types::*;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use rtdlib::types::{RObject, RTDType};

/// An option changed its value.
#[derive(Debug, Clone)]
pub struct TGUpdateOption {
  inner: UpdateOption
}

impl RObject for TGUpdateOption {
  fn td_name(&self) -> &'static str {
    self.inner.td_name()
  }

  fn td_type(&self) -> RTDType {
    self.inner.td_type()
  }

  fn to_json(&self) -> String {
    self.inner.to_json()
  }
}

impl Serialize for TGUpdateOption {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    self.inner.serialize(serializer)
  }
}

impl<'de> Deserialize<'de> for TGUpdateOption {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
    UpdateOption::deserialize(deserializer).map(|inner| TGUpdateOption::new(inner))
  }
}

impl TGUpdateOption {
  pub fn new(inner: UpdateOption) -> Self {
    Self { inner }
  }

  pub fn from_json<S: AsRef<str>>(json: S) -> Option<TGUpdateOption> {
    UpdateOption::from_json(json).map(|inner| TGUpdateOption::new(inner))
  }

  pub fn td_origin(&self) -> &UpdateOption {
    &self.inner
  }
}
```

**src/types/t_message_content.rs**

```rust
#[derive(Debug, Clone)]
pub struct TGVoiceNote {
  inner: VoiceNote
}

impl RObject for TGVoiceNote {
  fn td_name(&self) -> &'static str {
    self.inner.td_name()
  }

  fn td_type(&self) -> RTDType {
    self.inner.td_type()
  }

  fn to_json(&self) -> String {
    self.inner.to_json()
  }
}

impl Serialize for TGVoiceNote {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    self.inner.serialize(serializer)
  }
}

impl<'de> Deserialize<'de> for TGVoiceNote {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
    VoiceNote::deserialize(deserializer).map(|inner| TGVoiceNote::new(inner))
  }
}

impl TGVoiceNote {
  pub fn new(inner: VoiceNote) -> Self {
    Self { inner }
  }

  pub fn from_json<S: AsRef<str>>(json: S) -> Option<TGVoiceNote> {
    VoiceNote::from_json(json).map(|inner| TGVoiceNote::new(inner))
  }

  pub fn td_origin(&self) -> &VoiceNote {
    &self.inner
  }
}

#[derive(Debug, Clone)]
pub struct TGVenue {
  inner: Venue
}

impl RObject for TGVenue {
  fn td_name(&self) -> &'static str {
    self.inner.td_name()
  }

  fn td_type(&self) -> RTDType {
    self.inner.td_type()
  }

  fn to_json(&self) -> String {
    self.inner.to_json()
  }
}

impl Serialize for TGVenue {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    self.inner.serialize(serializer)
  }
}

impl<'de> Deserialize<'de> for TGVenue {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
    Venue::deserialize(deserializer).map(|inner| TGVenue::new(inner))
  }
}

impl TGVenue {
  pub fn new(inner: Venue) -> Self {
    Self { inner }
  }

  pub fn from_json<S: AsRef<str>>(json: S) -> Option<TGVenue> {
    Venue::from_json(json).map(|inner| TGVenue::new(inner))
  }

  pub fn td_origin(&self) -> &Venue {
    &self.inner
  }
}
```


The generated code is like this. Next, you can supplement the generated code.

**src/types/f_update_options.rs**


```rust
use crate::types::TGUpdateOption;
use rtdlib::types as td_type;
use crate::errors;

impl TGUpdateOption {

  pub fn name(&self) -> String {
    self.td_origin().name().clone().expect(errors::TELEGRAM_DATA_FAIL)
  }

  pub fn value(&self) -> TGOptionValue {
    TGOptionValue::new(self.td_origin().value())
  }

  pub fn on_name<S: AsRef<str>, F: FnOnce(&TGOptionValue)>(&self, name: S, fnc: F) {
    let value = TGOptionValue::new(self.td_origin().value());
    if &self.name()[..] == name.as_ref() && value.is_some() {
      fnc(&value)
    }
  }

}

pub struct TGOptionValue {
  value: Option<Box<td_type::OptionValue>>
}

macro_rules! option_value_as {
  ($value_class:ident, $retype:tt) => (
    fn ovas(value: &Option<Box<td_type::OptionValue>>) -> Option<$retype> {
      value.clone().filter(|v| v.td_type() == td_type::RTDType::$value_class)
        .map(|v| td_type::$value_class::from_json(v.to_json()))
        .filter(|v| v.is_some())
        .map(|v| v.map(|v| v.value().clone().map(|v| v)))
        .map_or(None, |v| v)
        .map_or(None, |v| v)
    }
  )
}

impl TGOptionValue {

  fn new(value: Option<Box<td_type::OptionValue>>) -> Self {
    Self { value }
  }

  fn is_some(&self) -> bool {
    self.value.is_some()
  }

  pub fn is_bool(&self) -> bool {
    self.value.clone().map(|v| v.td_type() == td_type::RTDType::OptionValueBoolean)
      .map_or(false, |v| v)
  }

  pub fn is_empty(&self) -> bool {
    self.value.clone().map(|v| v.td_type() == td_type::RTDType::OptionValueEmpty)
      .map_or(false, |v| v)
  }

  pub fn is_string(&self) -> bool {
    self.value.clone().map(|v| v.td_type() == td_type::RTDType::OptionValueString)
      .map_or(false, |v| v)
  }

  pub fn is_integer(&self) -> bool {
    self.value.clone().map(|v| v.td_type() == td_type::RTDType::OptionValueInteger)
      .map_or(false, |v| v)
  }

  pub fn as_string(&self) -> Option<String> {
    option_value_as!(OptionValueString, String);
    ovas(&self.value)
  }

  pub fn as_integer(&self) -> Option<i32> {
    option_value_as!(OptionValueInteger, i32);
    ovas(&self.value)
  }

  pub fn as_bool(&self) -> Option<bool> {
    option_value_as!(OptionValueBoolean, bool);
    ovas(&self.value)
  }

}
```




