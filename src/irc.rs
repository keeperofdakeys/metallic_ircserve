extern crate green;
extern crate rustuv;
use irc_tcp::get_tcp_comms;
use irc_tcp::{Read};

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    green::start(argc, argv, rustuv::event_loop, main)
}

fn main() {
  let (recv, write_conn_recv) = get_tcp_comms("127.0.0.1", 8787);
  loop {
    let line = match recv.recv() {
      Read(i, m) => format!("{}: {}", i, m),
      _ => continue,
    };
    print!("{}", line);
  }
}
