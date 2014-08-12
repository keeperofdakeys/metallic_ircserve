extern crate green;
extern crate rustuv;
use std::comm::{sync_channel, Receiver, Sender};
use tcp::{start_tcp_handler, WorkerProcSender};
use tcp::{TcpEvent, ConnCreat, Read, Write, ConnClose};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    green::start(argc, argv, rustuv::event_loop, start_irc)
}

struct IRCConfig {
  channels: Arc<Mutex<HashMap<String, Channel>>>,
  users: Arc<Mutex<HashMap<String, Sender<TcpEvent>>>>
}

impl IRCConfig {
  fn new() -> IRCConfig {
    IRCConfig {
      channels: Arc::new(Mutex::new(HashMap::new())),
      users: Arc::new(Mutex::new(HashMap::new()))
    }
  }
}

impl Clone for IRCConfig {
  fn clone(&self) -> IRCConfig {
    IRCConfig {
      channels: self.channels.clone(),
      users: self.users.clone()
    }
  }

  fn clone_from(&mut self, source: &IRCConfig) {
    self.channels = source.channels.clone();
    self.users = source.users.clone();
  }
}

struct Channel {
  name: String,
  users: Vec<String>
}

fn start_irc() {
  let irc_conf = IRCConfig::new();
  let (worker_send, worker_recv) = sync_channel(0);
  let irc_conf_c = irc_conf.clone();
  spawn( proc() irc_worker_gen( irc_conf_c, worker_send ) );
  start_tcp_handler( "127.0.0.1", 8787, worker_recv );
}

fn irc_worker_gen( irc_struct: IRCConfig, worker_send: WorkerProcSender ) {
  loop {
    let irc_struct_c = irc_struct.clone();
    worker_send.send( proc(a, b) {proc() irc_worker_pipe(irc_struct_c, a, b) } );
  }
}

fn irc_worker_pipe( irc_struct: IRCConfig, write_send: Sender<TcpEvent>, read_recv: Receiver<TcpEvent> ) {
  // Authenticate user -> add writer to writer_map in irc_struct

  // loop over lines from line_reader
    // Find sense from the mess
    // If channel, get channel writer, and send message.
    // If a pm, get user writer directly, and send message.
    // If something else, either deal with directly, or make a master thread?

    // Sine all events should be fast or async, we can deal with timing here (though maybe move up).
    // If we are flooding, or something unsavoury happens, we should end this thread.
    // So we send a special message, and either end here, or have just have it after the loop.

  // End a connection here. We need to remove the writer from irc_struct, and such. Then
  // send ConnClose. The write task (hopefully) closes the reader for us. Need to do some testing.
  loop {
    let line = match read_recv.recv() {
      ConnCreat => {
        write_send.send( ConnCreat );
        continue;
      },
      Read(m) => format!("{}", m),
      ConnClose => {
        write_send.send( ConnClose );
        return;
      },
      Write(_) => {
        return;
      }
    };
    print!("{}", line);
  }
}
