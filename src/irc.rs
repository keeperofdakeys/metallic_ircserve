extern crate green;
extern crate rustuv;
use irc_tcp::get_tcp_comms;
use irc_tcp::{ConnCreat, Read, Write, ConnClose};
use std::collections::HashSet;

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    green::start(argc, argv, rustuv::event_loop, main)
}

fn main() {
  let (recv, write_conn_recv) = get_tcp_comms("127.0.0.1", 8787);
  let mut conns = HashSet::new();
  loop {
    let line = match recv.recv() {
      ConnCreat(i) => {
        conns.insert(i);
        println!("Connection {} created.", i);
        continue;
      },
      Read(i, m) => format!("{}: {}", i, m),
      ConnClose(i) => {
        if conns.contains(&i) {
          conns.remove(&i);
        } else {
          println!( "Connection {} not in set!", i);
          return;
        }
        continue;
      },
      Write(_, _) => {
        return;
      }
    };
    print!("{}", line);
  }
}
