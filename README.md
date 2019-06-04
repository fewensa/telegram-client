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


