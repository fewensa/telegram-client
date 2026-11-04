telegram-client
===

[![CI](https://github.com/fewensa/telegram-client/actions/workflows/ci.yml/badge.svg)](https://github.com/fewensa/telegram-client/actions/workflows/ci.yml)

Telegram client for Rust.

This crate uses [TDLib](https://github.com/tdlib/td) to call the Telegram client API, including an async API.

The typed surface matches TDLib 1.8.69 via `rtdlib` 1.8.69 (`td_api.tl` at `4c6377aa7386c39217878a9a56d581922ad37ca4`). See [CHANGELOG](./CHANGELOG.md) for the breaks versus 1.8.1.

## Usage

```toml
[dependencies]
telegram-client = "=1.8.69"
```

Rust 1.89 or newer. You also need a `libtdjson` shared library on the linker and loader path (see below). Link the 1.8.69 `libtdjson` (or a build from that TDLib commit). CI uses current stable.

## version

Please read: [version](./version.md)


## Note

Building and running needs the [tdjson](https://github.com/tdlib/td) shared library (`libtdjson.so`, `libtdjson.dylib`, or `tdjson.dll`) on the linker path. `build.rs` also searches `./lib`. See [rtdlib-sys](https://github.com/fewensa/rtdlib-sys).

On Ubuntu 25.10 and newer, `libtdjson` is packaged. On older releases, install a current `libtdjson` build and make sure `libtdjson.so` (not only the versioned soname) is visible to the linker.

## Project

- [Security policy](./SECURITY.md)
- [Contributing](./CONTRIBUTING.md)
- [Code of conduct](./CODE_OF_CONDUCT.md)
- [Changelog](./CHANGELOG.md)

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
listener.on_added_proxy(|(api, pxy)| {
debug!("Proxy info => {:?}", pxy);
Err(TGError::new("some error"))
});
```

