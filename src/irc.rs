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

pub struct IRCConfig {
  channels: Arc<Mutex<HashMap<String, Channel>>>,
  users: Arc<Mutex<HashMap<String, Sender<TcpEvent>>>>
}

pub enum Destination {
  DestChan( Channel ),
  DestUser( User )
}

impl IRCConfig {
  fn new() -> IRCConfig {
    IRCConfig {
      channels: Arc::new(Mutex::new(HashMap::new())),
      users: Arc::new(Mutex::new(HashMap::new()))
    }
  }

  pub fn send_msg( &self, dest: &[u8], msg: &[u8] ) -> Result<(), ()>{
    Err( () )
  }
}

impl Clone for IRCConfig {
  fn clone(&self) -> IRCConfig {
    IRCConfig {
      channels: self.channels.clone(),
      users: self.users.clone()
    }
  }

  fn clone_from( &mut self, source: &IRCConfig ) {
    self.channels = source.channels.clone();
    self.users = source.users.clone();
  }
}

struct Channel {
  name: String,
  users: Vec<String>
}

struct User {
  a: int
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
  match read_recv.recv() {
    ConnCreat => {
      write_send.send( ConnCreat )
    },
    _ => {
      drop_conn( &write_send );
      return;
    }
  }
  // Authenticate user -> add writer to writer_map in irc_struct
  let user = match accept_conn( &read_recv, &write_send ) {
    Some(a) => a,
    None => {
      drop_conn( &write_send );
      return;
    }
  };
  write_send.send( Write( user ) );

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
        // We should never get write, close connection now.
        write_send.send( ConnClose );
        return;
      }
    };
    print!("{}", line);
  }
}

fn accept_conn( read_recv: &Receiver<TcpEvent>, write_send: &Sender<TcpEvent> ) -> Option<String> {
  let line = match (*read_recv).recv() {
    Read(a) => a,
    _ => return None
  };
  if line.as_slice().starts_with( "nick" ) {
    let offset = match line.as_slice().find(':') {
      Some(a) => a,
      None => return None
    };
    let nick = line.as_slice().slice_from( offset+1 );
    Some( nick.to_string() )
  } else {
    None
  }
}

fn drop_conn( write_send: &Sender<TcpEvent> ) {
  (*write_send).send( Write( "Dropping Connection".to_string() ) );
  (*write_send).send( ConnClose );
  println!("A-OK");
}
