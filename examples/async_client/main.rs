#[macro_use]
extern crate log;

use std::io::{self, BufRead};
use rtdlib::types::*;
use std::sync::{Arc, Mutex};
use telegram_client::api::Api;
use telegram_client::client::Client;
use tokio::prelude::*;
use rtdlib::Tdlib;
use crate::config::{Config, LogType};

mod thelp;
mod tgfn;
mod config;

#[tokio::main]
async fn main() {
  simple_logger::init().unwrap();
  log::set_max_level(log::LevelFilter::Debug);


  let api_id = env!("API_ID");
  let api_hash = env!("API_HASH");


  let config = Config::default();
  let async_api = Api::rasync();

  let mut client = Client::new(async_api.api().clone());
  client.start();
  let listener = client.listener();



  config.proxy().map(|v| {
    async_api.add_proxy(v);
  });

  config.log().map(|v| {
    Client::set_log_verbosity_level(v.level.clone() as i32).unwrap();
    if v.type_ == LogType::File {
      v.path.clone().map(|v| {
        Client::set_log_file_path(Some(&v[..]));
      });
    }
  });




  listener.on_update_option(|(api, option)| {
    let value = option.value();
    if value.is_empty() { debug!("Receive an option {} but it's empty", option.name()) }
    if value.is_string() { debug!("Receive an option {}: String => {}", option.name(), value.as_string().map_or("None".to_string(), |v| v.value().clone())) }
    if value.is_integer() { debug!("Receive an option {}: i32 => {}", option.name(), value.as_integer().map_or(-1, |v| v.value())) }
    if value.is_boolean() { debug!("Receive an option {}: bool => {}", option.name(), value.as_boolean().map_or(false, |v| v.value())) }

    match &option.name()[..] {
      "version" => { value.as_string().map(|v| debug!("VERSION IS {}", v.value())); }
      _ => {}
    };
    Ok(())
  });

  let chat = async_api.get_chat(GetChat::builder().chat_id(1)).await;

  println!("===============-----> {:?}", chat);



}
