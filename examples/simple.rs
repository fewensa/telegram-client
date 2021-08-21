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
  let listener = client.listener();

  listener.on_receive(|(api, json)| {
    debug!("receive {}", json);
    Ok(())
  });

  client.daemon("telegram-rs");
}


//fn mainx() {
//  let mut client = Client::default();
//  let listener = client.listener();
//
//  listener.on_receive(|(api: &Api, object: &Box<rtdlib::types::Object>)| {
//    let td_type: rtdlib::types::RTDType = object.td_type();
//    match td_type {
//      rtdlib::types::RTDType::UpdateUser => {
//        rtdlib::types::UpdateUser::from_json(object.to_json()).map(|update_user: rtdlib::types::UpdateUser| {
//          // do some thing
//        });
//      }
//      _ => {}
//    }
//  });
//}
