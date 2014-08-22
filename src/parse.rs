enum Prefix {
  ServerName(String),
  User_(String),
  UserHost_(String, String)
}

enum Command {
  Password,
  Nickname,
  User,
  Oper,
  UserMode,
  Service,
  Quit,
  SQuit,
  Join,
  Part,
  ChanMode,
  Topic,
  Names,
  List,
  Invite,
  Kick,
  PrivateMsg,
  Notice,
  MsgOfTheDay,
  LUsers,
  Version,
  Stats,
  Links,
  Time,
  Connect,
  Trace,
  Admin,
  Info,
  ServiceList,
  ServiceQuery,
  Who,
  WhoIs,
  WhoWas,
  Kill,
  Ping,
  Pong,
  Error,
  Away,
  ReHash,
  Die,
  Restart,
  Summon,
  Users,
  OperWall,
  UserHost,
  IsOn
}

struct Msg {
  command: Command,
  prefix: Prefix,
  params: Vec<String>,
}

pub fn no_spec_char( char: &u8 ) -> bool {
  match *char {
    0x01..0x09 => true,
    0x0B..0x0C => true,
    0x0E..0x1F => true,
    0x21..0x39 => true,
    0x3B..0xFF => true,
    _ => false
  }
}

pub fn user_char( char: &u8 ) -> bool {
  match *char {
    0x01..0x09 => true,
    0x0B..0x0C => true,
    0x0E..0x1F => true,
    0x21..0x3F => true,
    0x41..0xFF => true,
    _ => false
  }
}

pub fn key_char( char: &u8 ) -> bool {
  match *char {
    0x01..0x05 => true,
    0x07..0x08 => true,
    0x0C => true,
    0x0E..0x1F => true,
    0x21..0x7F => true,
    _ => false
  }
}

pub fn letter_char( char: &u8 ) -> bool {
  match *char {
    0x41..0x5A => true,
    0x61..0x7A => true,
    _ => false
  }
}

pub fn digit_char( char: &u8 ) -> bool {
  match *char {
    0x30..0x39 => true,
    _ => false
  }
}

pub fn hexdigit_char( char: &u8 ) -> bool {
  match *char {
    0x30..0x39 => true,
    0x41..0x46 => true,
    _ => false
  }
}

pub fn special_char( char: &u8 ) -> bool {
  match *char {
    0x5B..0x60 => true,
    0x7B..0x7D => true,
    _ => false
  }
}
