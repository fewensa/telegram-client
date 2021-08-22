#[macro_use]
extern crate log;

use simple_logger::SimpleLogger;

use rtdlib::types::*;
use telegram_client::api::Api;
use telegram_client::client::Client;

fn main() {
  SimpleLogger::new()
    .with_level(log::LevelFilter::Debug)
    .init()
    .unwrap();


  let api = Api::default();
  let mut client = Client::new(api.clone());
  let listener = client.listener().event_listener_mut();

  listener.on_receive(|(api, json)| {
    debug!("receive {}", json);
    Ok(())
  });

  client.daemon("telegram-rs");
}
