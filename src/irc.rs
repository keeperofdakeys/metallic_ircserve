extern crate green;
extern crate rustuv;
use irc_tcp::get_tcp_comms;
use irc_tcp::{ConnCreat, Read, Write, ConnClose};

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    green::start(argc, argv, rustuv::event_loop, main)
}

fn main() {
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
