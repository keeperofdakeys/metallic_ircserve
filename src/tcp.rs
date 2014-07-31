extern crate rustuv;
extern crate rustrt;
extern crate green;
use std::io::net::tcp;
use std::option::Option;
use std::io;
use std::comm::channel;
use std::comm::{channel,Receiver,Sender};
use std::io::{Acceptor, Listener};

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    green::start(argc, argv, rustuv::event_loop, main)
}

fn main() {
  let (err_send, err_recv) = channel();
  let (conn_send, conn_recv) = channel();
  spawn( proc() {
    //let mut io_ref = LocalIo::borrow().unwrap();
    tcp_listen("127.0.0.1", 8787, conn_send, err_send);
  }
  );

  loop {
    let mut input_2 = conn_recv.recv();
    spawn( proc() {
      loop {
        let mut input = io::BufferedReader::new(input_2.clone());
        print!("{}", input.read_line().unwrap());
      }
    });
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
/*
fn get_acceptor(ip: &str, port: u16) -> proc():Send {
  let new_ip = ip.to_string();
  let new_port = port.clone();
  proc() {
    let listener = rustuv::net::TcpListener::bind(new_ip, new_port);
    println!("lol");
  }
}
*/


