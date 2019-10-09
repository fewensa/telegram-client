telegram-client
===

[![Build Status](https://api.travis-ci.org/fewensa/telegram-client.svg)](https://travis-ci.org/fewensa/telegram-client/)

Telegram client for rust.

This crate use [td](https://github.com/tdlib/td) to call telegram client api, td create is [rtdlib](https://crates.io/crates/rtdlib)


# Usage

## 1.3.*

```toml
[dependencies]
telegram-client = "1.3.*"
```

## 1.4.*

```toml
[dependencies]
telegram-client = "1.4.*"
```


## version

Since the rtdlib version follows [td](https://github.com/tdlib/td).

Version mapping

| telegram-client    | td      |
|--------------------|---------|
| 1.3.*              | 1.3.*   |
| 1.4.*              | 1.4.*   |

## Note

Note that you need [libtdjson.so](https://github.com/tdlib/td) in your path for building and running your application. See also [rtdlib](https://github.com/fewensa/rtdlib) for more details.

# Examples

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

more [examples](./examples)



# Event

Most of the events are from td, two events of particular concern.

## on_receive

This event is receive everything from td, returned data type is a json string.

## on_exception

When td returned json can not deserialize, or your event handler returned error. will be call is event.

a sample of event handler returned error

```rust
listener.on_proxy(|(api, pxy)| {
  debug!("Proxy info => {:?}", pxy);
  Err(TGError::new("some error"))
});
```
