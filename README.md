telegram-client
===

[![Build Status](https://api.travis-ci.org/fewensa/telegram-client.svg)](https://travis-ci.org/fewensa/telegram-client/)

Telegram client for rust.

This crate use [td](https://github.com/tdlib/td) to call telegram client api. support async api.

## Usage

```toml
[dependencies]
telegram-client = "1.7.*"
```

### version

The more informations you can read [Version mapping](https://github.com/fewensa/telegram-client/wiki/Version-mapping)

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

