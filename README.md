telegram-client
===

[![Build Status](https://api.travis-ci.org/fewensa/telegram-client.svg)](https://travis-ci.org/fewensa/telegram-client/)

Telegram client for rust.

This crate use [td](https://github.com/tdlib/td) to call telegram client api. support async api.

## Usage

```toml
[dependencies]
telegram-client = "0.8.*"
```

## version

### td and rtdlib/telegram-client

Version mapping

| telegram-client  | td      |
|------------------|---------|
| 0.8.*            | [master@fa8feef](https://github.com/tdlib/td/commit/fa8feefed70d64271945e9d5fd010b957d93c8cd) |
| 1.8.*            | 1.8.*   |

The version `1.3`, `1.4`, `1.5`, `1.6`, `1.7` is outdated. the reason you can read

- [A new telegram client update](https://github.com/fewensa/telegram-client/issues/29)
- [UPDATE_APP_TO_LOGIN](https://github.com/tdlib/td/issues/1758)

### telegram-client and rtdlib

A fixed version is recommended, you can read [Comparison requirements](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#comparison-requirements) about the fixed version.
Because of cargo's dependency mechanism, if you don't specify a specific version, it will be automatically upgraded, but there is usually a dependency between t and a, and the new version cannot be applied.
The current dependencies are as follows:


| telegram-client    | rtdlib      |
|--------------------|-------------|
| =0.8.0             | =0.8.0      |
| =1.8.0             | =1.8.0      |


## Note

Note that you need [tdjson](https://github.com/tdlib/td) dylib file in your path for building and running your application. See also [rtdlib-sys](https://github.com/fewensa/rtdlib-sys) for more details.

## Examples


### block

```rust
fn main() {
  let api = Api::default();
  let mut client = Client::new(api.clone());
  let listener = client.listener();

  listener.on_receive(|(api, json)| {
    debug!("receive {}", json);
    Ok(())
  });

  client.daemon("telegram-rs");
}
```

### async

```rust
#[tokio::main]
async fn main() {
  let api = Api::rasync();

  let mut client = Client::new(api.api().clone());
  let listener = client.listener();

  // listener.on_update_authorization_state...

  client.start();

  let chat = api.get_chat(GetChat::builder().chat_id(1)).await;
  println!("{:#?}", chat);
}
```

### more

more [examples](./examples)

## Event

Most of the events are from td, two events of particular concern.

### on_receive

This event is receive everything from td, returned data type is a json string.

### on_exception

When td returned json can not deserialize, or your event handler returned error. will be call is event.

a sample of event handler returned error

```rust
listener.on_proxy(|(api, pxy)| {
debug!("Proxy info => {:?}", pxy);
Err(TGError::new("some error"))
});
```

