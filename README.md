telegram-client
===

[![Build Status](https://api.travis-ci.org/fewensa/telegram-client.svg)](https://travis-ci.org/fewensa/telegram-client/)

Telegram client for rust.

This crate use [td](https://github.com/tdlib/td) to call telegram client api, td create is [rtdlib](https://crates.io/crates/rtdlib)


# Usage

## latest

```toml
[dependencies]
telegram-client = "100.100"
```

## 1.3.*

```toml
[dependencies]
telegram-client = "1.3"
```

## 1.4.*

```toml
[dependencies]
telegram-client = "1.4"
```


## version

Since the rtdlib version follows [td](https://github.com/tdlib/td), a version number less than 100 is reserved for td release.

Version mapping

| telegram-client    | td      |
|--------------------|---------|
| 100.100.*          | master  |
| 1.3.*              | 1.3.*   |
| 1.4.*              | 1.4.*   |

## Note

Note that you need [libtdjson.so.1.4.0](https://github.com/tdlib/td) in your path for building and running your application. See also [rtdlib](https://github.com/fewensa/rtdlib) for more details.

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



