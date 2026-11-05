//! `ApiBuilder::new` must not open a TDLib client.
//!
//! `Api::new` replaces the builder's client. If the builder already created
//! one, `Api::event` / `Api::rasync` / `Api::new` open a second client and
//! TDLib destroys the first (`Destroy Client`, then `authorizationStateClosing`
//! / `authorizationStateClosed`). The client id is not exposed on `Tdlib`, so
//! this reads TDLib's own log. It does not connect to Telegram.

use std::fs;
use std::path::{Path, PathBuf};
use std::process;

use rtdlib::Tdlib;
use telegram_client::api::Api;
use telegram_client::client::Client;

extern "C" {
  fn dup(fd: i32) -> i32;
  fn dup2(oldfd: i32, newfd: i32) -> i32;
  fn close(fd: i32) -> i32;
}

fn read_log(dir: &Path) -> String {
  let mut files: Vec<PathBuf> = fs::read_dir(dir)
    .unwrap_or_else(|err| panic!("read {}: {err}", dir.display()))
    .map(|entry| entry.expect("log entry").path())
    .filter(|path| path.is_file())
    .collect();
  files.sort();
  let mut out = String::new();
  for path in files {
    let bytes = fs::read(&path).unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
    out.push_str(&String::from_utf8_lossy(&bytes));
  }
  out
}

fn create_count(log: &str) -> usize {
  log
    .lines()
    .filter(|line| line.contains("Create client "))
    .count()
}

fn destroy_count(log: &str) -> usize {
  log
    .lines()
    .filter(|line| line.contains("Destroy Client") && !line.contains("Destroy ClientManager"))
    .count()
}

fn assert_log(dir: &Path, creates: usize, destroys: usize) {
  let log = read_log(dir);
  assert_eq!(
    create_count(&log),
    creates,
    "expected {creates} Create client lines\n{log}"
  );
  assert_eq!(
    destroy_count(&log),
    destroys,
    "expected {destroys} Destroy Client lines\n{log}"
  );
  assert!(
    !log.contains("authorizationStateClosing") && !log.contains("authorizationStateClosed"),
    "TDLib closed a client\n{log}"
  );
}

#[test]
fn constructors_open_one_tdlib_client() {
  let dir = std::env::temp_dir().join(format!(
    "telegram-client-one-client-{}-{}",
    process::id(),
    std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .expect("clock")
      .as_nanos()
  ));
  fs::create_dir_all(&dir).expect("temp dir");
  let log_path = dir.join("tdlib.log");

  // td_set_log_file_path duplicates the log fd onto stderr. Put stderr back
  // so a failure is still reported on the test console.
  let saved = unsafe { dup(2) };
  assert!(saved >= 0, "dup(stderr) failed");
  let level = Client::set_log_verbosity_level(3);
  let switched = Client::set_log_file_path(Some(log_path.to_str().expect("utf8 log path")));
  let restored = unsafe { dup2(saved, 2) };
  unsafe { close(saved) };
  assert_eq!(restored, 2, "failed to restore stderr");
  level.expect("log verbosity");
  assert!(switched, "td_set_log_file_path failed");

  assert_log(&dir, 0, 0);

  let event = Api::event();
  assert_log(&dir, 1, 0);

  let async_api = Api::rasync();
  assert_log(&dir, 2, 0);

  let explicit = Api::new(Tdlib::new());
  assert_log(&dir, 3, 0);

  let from_event = Api::event_with_tdlib(Tdlib::new());
  let from_async = Api::rasync_with_tdlib(Tdlib::new());
  assert_log(&dir, 5, 0);

  // `*_with_api` keeps the caller's client.
  let shared = Api::default();
  let shared_event = Api::event_with_api(shared.clone());
  let shared_async = Api::rasync_with_api(shared.clone());
  assert_log(&dir, 6, 0);

  // First build creates the client. A later build, including on a clone, reuses it.
  let builder = Api::builder();
  let built = builder.build();
  let built_again = builder.clone().build();
  assert_log(&dir, 7, 0);

  drop(event);
  drop(async_api);
  drop(explicit);
  drop(from_event);
  drop(from_async);
  drop(shared_event);
  drop(shared_async);
  drop(shared);
  drop(built);
  // The builder and `built_again` still hold the last client. Closing states
  // from the clients just dropped are expected in the log from here on.
  let log = read_log(&dir);
  assert_eq!(destroy_count(&log), 6, "dropped one client too many\n{log}");

  drop(built_again);
  drop(builder);
  let log = read_log(&dir);
  assert_eq!(create_count(&log), 7, "{log}");
  assert_eq!(destroy_count(&log), 7, "{log}");

  let _ = fs::remove_dir_all(&dir);
}
