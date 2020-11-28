#[macro_use]
extern crate log;

use std::io::{self, BufRead};
use rtdlib::types::*;
use std::sync::{Arc, Mutex};
use telegram_client::api::Api;
use telegram_client::client::Client;
use tokio::prelude::*;
use rtdlib::Tdlib;

#[tokio::main]
async fn main() {
  simple_logger::init().unwrap();
  log::set_max_level(log::LevelFilter::Debug);
  let api = Api::rasync();

  let mut client = Client::new(api.api().clone());

  client.start();

  let chat = api.get_chat(GetChat::builder().chat_id(1)).await.unwrap();
  println!("{:?}", chat);
}
