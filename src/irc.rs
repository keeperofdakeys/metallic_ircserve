extern crate green;
extern crate rustuv;
use std::comm::{channel,Receiver,Sender};
use tcp::get_tcp_comms;
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
  let irc_task_proc = proc(read, write) {
    irc_task(3i, read, write);
  };
  let (read_recv, write_send) = get_tcp_comms("127.0.0.1", 8787);
  loop {
    let (i, event) = read_recv.recv();
    let line = match event {
      ConnCreat => {
        write_send.send( (i, ConnCreat) );
        continue;
      },
      Read(m) => format!("{}: {}", i, m),
      ConnClose => {
        write_send.send( (i, ConnClose) );
        continue;
      },
      Write(_) => {
        return;
      }
    };
    print!("{}", line);
  }
}

fn irc_task<T>( irc_struct: T, line_reader: Receiver<TcpEvent>, line_writer: Sender<TcpEvent> ) {
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
  
}
