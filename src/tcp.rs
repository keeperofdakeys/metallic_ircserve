extern crate rustrt;
use std::io::net::tcp::{TcpStream, TcpListener};
use std::io;
use std::string::String;
use std::comm::{channel, Receiver, Sender, SyncSender};
use std::io::{Acceptor, Listener};
use std::collections::HashMap;
use std::vec::Vec;
use std::string;

pub enum TcpEvent {
  ConnCreat,
  Read(String),
  Write(String),
  ConnClose
}

pub type WorkerPipe<'a> = proc():Send -> |x: Sender<TcpEvent>, y: Receiver<TcpEvent>|:'a -> proc():Send;
pub type WorkerProc = proc(Sender<TcpEvent>, Receiver<TcpEvent>):Send -> proc():Send;
pub type WorkerProcSender = SyncSender< WorkerProc >;
pub type WorkerProcReceiver = Receiver< WorkerProc >;

pub fn start_tcp_handler( ip: &str, port: u16, worker_recv: WorkerProcReceiver ) {
  let (err_send, err_recv) = channel();
  let (conn_send, conn_recv) = channel();
  let ip_new = ip.to_string();
  spawn( proc() {
    tcp_listen( ip_new.as_slice(), port, conn_send, err_send );
  }
  );

  spawn(proc() { tcp_task_spawner( conn_recv, worker_recv ) } );

}

fn tcp_task_spawner( conn_recv: Receiver<TcpStream>, worker_recv: WorkerProcReceiver ) {
  let mut counter = 1u;
  for conn in conn_recv.iter() {
    let read = conn.clone();
    let write = conn.clone();
    
    let (read_send, read_recv) = channel();
    let (write_send, write_recv) = channel();
    let worker_proc = worker_recv.recv();
    spawn( proc() { tcp_task_read( &counter, read, read_send ) } );
    spawn( worker_proc( write_send, read_recv ) );
    spawn( proc() { tcp_task_write( &counter, write, write_recv ) } );
    counter += 1;
  }
}

fn tcp_task_read( counter: &uint, mut reader: TcpStream, read_send: Sender<TcpEvent> ) {
  let mut buf = box () ([0, ..1024]);
  let mut count: uint;
  read_send.send( ConnCreat );
  loop {
    count = match reader.read( *buf ) {
      Ok(a) => a,
      Err(e) => match e.kind {
        io::EndOfFile => {
          println!("read close")
          reader.close_read();
          read_send.send_opt( ConnClose );
          return;
        },
        _ => {
          reader.close_read();
          reader.close_write();
          read_send.send_opt( ConnClose );
          return;
        }
      }
    };
    if count == 0 {
      continue;
    }
    let mut line: String;
    unsafe {
      line = string::raw::from_utf8(Vec::from_slice(*buf));
    }
    line.truncate( count );
    read_send.send( Read(line) );
  }
}

fn tcp_task_write( counter: &uint, mut writer: TcpStream, write_recv: Receiver<TcpEvent> ) {
  match write_recv.recv() {
    ConnCreat => {
      let _ = writeln!( writer, "{} {}", counter, "haha");
    },
    _ => {
      tcp_close_conn( &mut writer );
      return;
    }
  }

  loop {
    let line = match write_recv.recv() {
      Write(ref m) => m.clone(),
      ConnClose => {
        println!("write close");
        tcp_close_conn( &mut writer );
        return;
      },
      _ => {
        tcp_close_conn( &mut writer );
        return;
      }
    };
    let _ = write!( writer, "{}: {}", counter, line );
  }
}

fn tcp_close_conn( tcp_stream: &mut TcpStream ) {
  tcp_stream.close_read();
  tcp_stream.close_write();
}

fn tcp_listen( ip: &str, port: u16, conn_send: Sender<TcpStream>, err_send: Sender<io::IoError> ) {
  let listener = match TcpListener::bind(ip, port) {
    Ok(m) => { m }
    Err(e) => {
      err_send.send(e);
      return;
    }
  };
  let mut acceptor = match listener.listen() {
    Ok(m) => { m }
    Err(e) => {
      err_send.send(e);
      return;
    }
  };
  for conn in acceptor.incoming() {
    match conn {
      Ok(m) => { conn_send.send(m); }
      Err(e) => { err_send.send(e); }
    };
  }
}
