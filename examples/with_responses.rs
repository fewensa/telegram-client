#[macro_use]
extern crate log;

use std::io::{self, BufRead};
use std::sync::{Arc, Mutex};

use simple_logger::SimpleLogger;
use tokio::prelude::*;

use rtdlib::Tdlib;
use rtdlib::types::*;
use telegram_client::api::Api;
use telegram_client::client::Client;

#[tokio::main]
async fn main() {
  SimpleLogger::new()
    .with_level(log::LevelFilter::Debug)
    .init()
    .unwrap();

  let api = Api::rasync();

  let mut client = Client::new(api.api().clone());

  client.start();

  let chat = api.get_chat(GetChat::builder().chat_id(1)).await.unwrap();
  println!("{:?}", chat);
}
