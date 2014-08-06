

extern crate rustrt;
use std::io::net::tcp;
use std::io;
use std::string::String;
use std::comm::{channel,Receiver,Sender};
use std::io::{Acceptor, Listener};
use std::fmt::radix;

pub fn get_tcp_comms( ip: &str, port: u16 ) -> (Receiver<String>, Receiver<Sender<String>>) {
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

fn tcp_task_spawner( conn_recv: Receiver<tcp::TcpStream>, read_send: Sender<String>, write_conn_send: Sender<Sender<String>> ) {
  let mut counter = 1i;
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

fn tcp_task_read( counter: &int, reader: tcp::TcpStream, read_send: Sender<String> ) {
  let mut buff = io::BufferedReader::new(reader);
  loop {
    match buff.read_line() {
      Ok(a) => {
        read_send.send(a);
        read_send.send(format!("{}{}", radix(*counter,10), "\n"));
      }
      Err(e) => match e.kind {
        io::EndOfFile => return,
        _ => (),
      },
    };
  }
}

fn tcp_task_write( counter: &int, mut writer: tcp::TcpStream, write_conn_send: Sender<Sender<String>> ) {
  let (conn_send, conn_recv) = channel();
  write_conn_send.send(conn_send);
  loop {
    let line = conn_recv.recv();
    let _ = write!( writer, "{} {}", line, counter );
  }
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

