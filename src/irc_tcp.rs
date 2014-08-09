extern crate rustrt;
use std::io::net::tcp;
use std::io;
use std::string::String;
use std::comm::{channel,Receiver,Sender};
use std::io::{Acceptor, Listener};

pub enum TcpEvent {
  ConnCreat(uint),
  Read(uint, String),
  Write(uint, String),
  ConnClose(uint)
}

pub type TcpWriter = (uint, Sender<TcpEvent>);

pub fn get_tcp_comms( ip: &str, port: u16 ) -> (Receiver<TcpEvent>, Receiver<TcpWriter>) {
  let (err_send, err_recv) = channel();
  let (conn_send, conn_recv) = channel();
  let ip_new = ip.to_string();
  spawn( proc() {
    //let mut io_ref = LocalIo::borrow().unwrap();
    tcp_listen(ip_new.as_slice(), port, conn_send, err_send);
  }
  );

  let (read_send, read_recv) = channel();
  let (write_conn_send, write_conn_recv) = channel();

  spawn(proc() { tcp_task_spawner( conn_recv, read_send, write_conn_send ) });

  return (read_recv, write_conn_recv);
  
}

fn tcp_task_spawner( conn_recv: Receiver<tcp::TcpStream>, read_send: Sender<TcpEvent>, write_conn_send: Sender<TcpWriter> ) {
  let mut counter = 1u;
  for conn in conn_recv.iter() {
    let read = conn.clone();
    let write = conn.clone();
    let read_send_c = read_send.clone();
    let write_conn_send_c = write_conn_send.clone();
    spawn(proc() { tcp_task_read( &counter, read, read_send_c ) });
    spawn(proc() { tcp_task_write( &counter, write, write_conn_send_c ) });
    counter += 1;
  }
}

fn tcp_task_read( counter: &uint, mut reader: tcp::TcpStream, read_send: Sender<TcpEvent> ) {
  let mut buff = io::BufferedReader::new(reader.clone());
  read_send.send(ConnCreat(*counter));
  loop {
    match buff.read_line() {
      Ok(a) => {
        read_send.send(Read(*counter, a));
      }
      Err(e) => match e.kind {
        io::EndOfFile => {
          tcp_close_conn( &mut reader );
          read_send.send(ConnClose(*counter));
          return;
        },
        _ => {
          tcp_close_conn( &mut reader );
          read_send.send(ConnClose(*counter));
          return;
        }
      },
    };
  }
}

fn tcp_task_write( counter: &uint, mut writer: tcp::TcpStream, write_conn_send: Sender<TcpWriter> ) {
  let (conn_send, conn_recv) = channel();
  write_conn_send.send( (*counter, conn_send) );
  match conn_recv.recv() {
    ConnCreat(i) if i == *counter => { },
    _ => {
      tcp_close_conn( &mut writer );
      return;
    }
  }

  loop {
    let line = match conn_recv.recv() {
      Write(i, ref m) if i == *counter => m.clone(),
      ConnClose(i) if i == *counter => {
        tcp_close_conn( &mut writer );
        return;
      },
      _ => {
        tcp_close_conn( &mut writer );
        return;
      }
    };
    let _ = write!( writer, "{} {}", line, counter );
  }
}

fn tcp_close_conn( tcp_stream: &mut tcp::TcpStream ) {
  tcp_stream.close_read();
  tcp_stream.close_write();
}

fn tcp_listen( ip: &str, port: u16, conn_send: Sender<tcp::TcpStream>, err_send: Sender<io::IoError> ) {
  let listener = match tcp::TcpListener::bind(ip, port) {
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

/*
fn tcp_listen<'a>( io: &'a mut rustrt::rtio::IoFactory, ip: IpAddr, port: u16 ) -> Option<Box<rustrt::rtio::RtioTcpStream+Send>> {
  let listener = match io.tcp_bind(SocketAddr{ip: ip, port: port}) {
*/

