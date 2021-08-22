use std::{io, thread};
use std::io::Write;
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::time::Duration;

use colored::Colorize;

use telegram_client::api::Api;

use crate::{tgfn, thelp};

pub fn typed() -> String {
  self::typed_with_message("")
}

pub fn typed_with_message<S: AsRef<str>>(message: S) -> String {
  let message = message.as_ref();
  if !message.is_empty() {
    self::tip(message);
  }
  let mut input = String::new();
  match io::stdin().read_line(&mut input) {
    Ok(_) => input.trim().to_string(),
    Err(e) => panic!("Can not get input value: {:?}", e)
  }
}

#[allow(dead_code)]
pub fn wait_too_many_requests(api: &Api, message: &String) {
  thelp::error(&message);
  regex::Regex::new("(?P<time>\\d+)").map(|regex| {
    regex.captures(&message[..]).map(|caps| {
      let time: String = (&caps["time"].to_string()).clone();
      thelp::tip(format!("Please wait [{}].", time));
      let (tx, rx) = mpsc::channel();
      thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(500));
        match rx.try_recv() {
          Ok(_) | Err(TryRecvError::Disconnected) => {
            thelp::append_end();
            break
          },
          _ => thelp::append_out(".")
        }
      });
      thread::sleep(Duration::from_millis(toolkit::number::as_u64d(time, 0)));
      let _ = tx.send(());
      thelp::append_end();
      thelp::tip("Please type telegram phone number.");
      tgfn::type_phone_number(api);
    })
  }).expect("tdlib 429 error not have wait time");
}

pub fn tip<S: AsRef<str>>(message: S) {
  println!("{} {}", ">>".red().bold(), message.as_ref().blue().bold())
}

#[allow(dead_code)]
pub fn append_out<S: AsRef<str>>(message: S) {
  print!("{}", message.as_ref().yellow());
  std::io::stdout().flush().expect("failed to append output");
}

#[allow(dead_code)]
pub fn append_end() {
  print!("\n");
  std::io::stdout().flush().expect("failed to append output");
}

#[allow(dead_code)]
pub fn error<S: AsRef<str>>(message: S) {
  println!("{} {}", ">>".red().bold(), message.as_ref().red().bold())
}

#[allow(dead_code)]
pub fn unknown<S: AsRef<str>>(code: i64, message: S) {
  println!("{} [{}] {}  << {}",
           ">>".red().bold(),
           &code.to_string()[..].cyan().bold(),
           message.as_ref().cyan().bold(),
           "UNREGISTER TELEGRAM ERROR".magenta().bold()
  )
}
