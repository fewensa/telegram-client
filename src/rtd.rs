use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use crate::api::Api;
use crate::handler::Handler;
use crate::listener::lasync::RasyncLout;
use crate::listener::levent::EventLout;
use crate::listener::Listener;
use crate::observer;

/// Debug builds of the generated `TdType` match need more than the default
/// 2 MiB thread stack. This is a fixed size, not one stack per update.
const TD_STACK: usize = 8 * 1024 * 1024;

#[derive(Default)]
pub struct TdRecv {}

impl TdRecv {
  pub fn new() -> TdRecv {
    Self::default()
  }

  pub fn start(
    &self,
    api: Arc<Api>,
    stop_flag: Arc<Mutex<bool>>,
    listener: &Listener,
    warn_unregister_listener: Arc<bool>,
  ) -> JoinHandle<()> {
    let event_lout = listener.event_listener().lout();
    let rasync_lout = listener.rasync_listener().lout();
    let (tx, rx) = mpsc::channel();
    let worker = spawn_worker(
      api.clone(),
      event_lout,
      rasync_lout,
      warn_unregister_listener,
      rx,
    );

    thread::Builder::new()
      .name("telegram-client-recv".into())
      .stack_size(TD_STACK)
      .spawn(move || {
        // Lock only for the flag check. Holding the guard across receive()
        // would block any writer trying to stop the loop.
        while !*stop_flag.lock().unwrap() {
          if let Some(json) = api.receive(2.0) {
            // Wake async callers before the worker sees this object. The
            // worker may be inside get_chat, waiting for exactly this JSON.
            observer::publish(&json);
            if tx.send(json).is_err() {
              break;
            }
          }
        }
        drop(tx);
        let _ = worker.join();
      })
      .expect("failed to start TDLib receive thread")
  }
}

/// Listeners run here, not on a fresh thread per update.
///
/// `td_json_client_send` allocates thread-local buffers (TDLib's stack
/// allocator is 1 MiB). Those buffers are freed only by TDLib's own thread
/// exit hook, which Rust threads never call. `get_chat` from a listener
/// sends on the handler thread, so a new thread per update retained that
/// buffer after every message. One worker keeps a single set of buffers.
fn spawn_worker(
  api: Arc<Api>,
  event_lout: EventLout,
  rasync_lout: RasyncLout,
  warn_unregister_listener: Arc<bool>,
  rx: Receiver<String>,
) -> JoinHandle<()> {
  thread::Builder::new()
    .name("telegram-client".into())
    .stack_size(TD_STACK)
    .spawn(move || {
      while let Ok(json) = rx.recv() {
        let handled = catch_unwind(AssertUnwindSafe(|| {
          futures::executor::block_on(async {
            Handler::new(
              api.as_ref(),
              &event_lout,
              &rasync_lout,
              warn_unregister_listener.as_ref(),
            )
            .handle(&json)
            .await;
          });
        }));
        if handled.is_err() {
          log::error!("update handler panicked");
        }
      }
    })
    .expect("failed to start TDLib handler thread")
}

#[cfg(test)]
fn drive(listener: &Listener, updates: Vec<String>) {
  let _ = crate::client::Client::set_log_verbosity_level(0);
  let api = Arc::new(Api::default());
  let (tx, rx) = mpsc::channel();
  let worker = spawn_worker(
    api,
    listener.event_listener().lout(),
    listener.rasync_listener().lout(),
    Arc::new(false),
    rx,
  );
  for json in updates {
    observer::publish(&json);
    tx.send(json).expect("handler thread exited");
  }
  drop(tx);
  worker.join().expect("handler thread panicked");
}

#[cfg(test)]
mod tests {
  use std::sync::{Arc, Mutex};
  use std::time::Duration;

  use futures::future::FutureExt;
  use rtdlib::types::*;

  use super::*;
  use crate::listener::Listener;

  fn on_td_stack(f: impl FnOnce() + Send + 'static) {
    let result = std::thread::Builder::new()
      .stack_size(TD_STACK)
      .spawn(f)
      .expect("spawn")
      .join();
    if let Err(panic) = result {
      std::panic::resume_unwind(panic);
    }
  }

  #[test]
  fn listeners_that_send_share_one_thread() {
    on_td_stack(listeners_that_send_share_one_thread_body);
  }

  fn listeners_that_send_share_one_thread_body() {
    let _lock = observer::test_lock();
    let _ = crate::client::Client::set_log_verbosity_level(0);
    let ids = Arc::new(Mutex::new(Vec::new()));
    let mut listener = Listener::new();
    let recorded = ids.clone();
    listener
      .event_listener_mut()
      .on_receive(move |(api, _json)| {
        recorded.lock().unwrap().push(std::thread::current().id());
        api
          .send(GetOption::builder().name("version").build())
          .expect("send");
        Ok(())
      });

    let mut updates = Vec::new();
    for index in 0..30 {
      updates.push(format!(r#"{{"@type":"ok","@extra":"upd-{index}"}}"#));
    }
    drive(&listener, updates);

    let ids = ids.lock().unwrap();
    assert_eq!(ids.len(), 30);
    assert!(ids.iter().all(|id| *id == ids[0]));
  }

  #[test]
  fn a_listener_blocked_on_a_response_is_woken_by_publish() {
    on_td_stack(a_listener_blocked_on_a_response_is_woken_by_publish_body);
  }

  fn a_listener_blocked_on_a_response_is_woken_by_publish_body() {
    let _lock = observer::test_lock();
    let _ = crate::client::Client::set_log_verbosity_level(0);
    let (ready_tx, ready_rx) = mpsc::channel::<()>();
    let result = Arc::new(Mutex::new(None));
    let mut listener = Listener::new();
    let result_slot = result.clone();
    listener.rasync_listener_mut().on_receive(move |_| {
      let ready_tx = ready_tx.clone();
      let result_slot = result_slot.clone();
      async move {
        let mut subscription = observer::subscribe("blocked-call");
        let _ = ready_tx.send(());
        let value = subscription.recv().await;
        *result_slot.lock().unwrap() = Some(matches!(value, Some(TdType::Ok(_))));
        Ok(())
      }
      .boxed_local()
    });

    let (tx, rx) = mpsc::channel();
    let worker = spawn_worker(
      Arc::new(Api::default()),
      listener.event_listener().lout(),
      listener.rasync_listener().lout(),
      Arc::new(false),
      rx,
    );
    tx.send("{}".to_string()).expect("send update");
    ready_rx
      .recv_timeout(Duration::from_secs(2))
      .expect("listener did not reach the async wait");
    observer::publish(r#"{"@type":"ok","@extra":"blocked-call"}"#);
    drop(tx);
    worker
      .join()
      .expect("worker stayed blocked after the response was published");
    assert_eq!(*result.lock().unwrap(), Some(true));
  }
}
