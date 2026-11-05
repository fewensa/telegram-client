//! TDLib keeps a thread-local allocator for every thread that calls
//! `td_json_client_send`, and does not free it when a Rust thread exits.
//! Issue #27 grew RAM because each update was handled on a new thread and
//! `get_chat` sends from that thread.

use std::thread;

use rtdlib::types::GetOption;
use telegram_client::api::Api;
use telegram_client::client::Client;

fn vm_size_kb() -> u64 {
  let status = std::fs::read_to_string("/proc/self/status").expect("status");
  for line in status.lines() {
    if let Some(rest) = line.strip_prefix("VmSize:") {
      return rest
        .split_whitespace()
        .next()
        .expect("VmSize value")
        .parse()
        .expect("VmSize number");
    }
  }
  panic!("VmSize missing from /proc/self/status");
}

fn send_get_option(api: &Api) {
  api
    .send(GetOption::builder().name("version").build())
    .expect("send");
}

#[test]
fn fresh_threads_retain_tdlib_address_space_and_one_thread_does_not() {
  let _ = Client::set_log_verbosity_level(0);
  let api = Api::default();
  send_get_option(&api);

  const N: u64 = 20;
  let before_threads = vm_size_kb();
  for _ in 0..N {
    let api = api.clone();
    thread::spawn(move || send_get_option(&api))
      .join()
      .expect("sender thread");
  }
  let thread_growth = vm_size_kb().saturating_sub(before_threads);
  // Measured on libtdjson 1.8.x: about 750 KiB of address space per thread.
  assert!(
    thread_growth > N * 256,
    "fresh send threads grew VmSize by {thread_growth} KiB, expected more than {} KiB",
    N * 256
  );

  let before_same = vm_size_kb();
  for _ in 0..N {
    send_get_option(&api);
  }
  let same_growth = vm_size_kb().saturating_sub(before_same);
  assert!(
    same_growth < N * 64,
    "repeating send on one thread grew VmSize by {same_growth} KiB"
  );
}
