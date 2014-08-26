use lexer::LexerMsg;
use lexer::POption;

struct ParseMsg<'a> {
  prefix: Option<&'a [u8]>,
  command: Command,
  params: Vec<&'a [u8]>,
}

fn parse_msg( msg: LexerMsg ) -> Result<ParseMsg, ()> {
  let prefix = parse_prefix( msg.prefix );
  let cmd = match parse_cmd( msg.command ) {
    Some(c) => c,
    None => return Err( () )
  };
  let params = parse_params( msg.params );

  Ok( ParseMsg{ prefix: prefix, command: cmd, params: params } )
}

fn parse_prefix( buf_prefix: Option<&[u8]> ) -> Option<&[u8]> {
  buf_prefix
}

fn parse_params( buf_params: Vec<&[u8]> ) -> Vec<&[u8]> {
  //buf_params.iter().map( |x| Vec::from_slice( *x ) ).collect()
  buf_params
}

pub enum Command {
  CmdPassword,
  CmdNickname,
  CmdUser,
  CmdOper,
  CmdMode,
  CmdService,
  CmdQuit,
  CmdSQuit,
  CmdJoin,
  CmdPart,
  CmdTopic,
  CmdNames,
  CmdList,
  CmdInvite,
  CmdKick,
  CmdPrivateMsg,
  CmdNotice,
  CmdMsgOfTheDay,
  CmdLUsers,
  CmdVersion,
  CmdStats,
  CmdLinks,
  CmdTime,
  CmdConnect,
  CmdTrace,
  CmdAdmin,
  CmdInfo,
  CmdServiceList,
  CmdServiceQuery,
  CmdWho,
  CmdWhoIs,
  CmdWhoWas,
  CmdKill,
  CmdPing,
  CmdPong,
  CmdError,
  CmdAway,
  CmdReHash,
  CmdDie,
  CmdRestart,
  CmdSummon,
  CmdUsers,
  CmdOperWall,
  CmdUserHost,
  CmdIsOn
}

fn parse_cmd( buf_cmd: &[u8] ) -> Option<Command> {
  match as_lowercase( buf_cmd ).as_slice() {
    b"pass" => Some(CmdPassword),
    b"nick" => Some(CmdNickname),
    b"user" => Some(CmdUser),
    b"oper" => Some(CmdOper),
    b"mode" => Some(CmdMode),
    b"service" => Some(CmdService),
    b"quit" => Some(CmdQuit),
    b"squit" => Some(CmdSQuit),
    b"join" => Some(CmdJoin),
    b"part" => Some(CmdPart),
    b"topic" => Some(CmdTopic),
    b"names" => Some(CmdNames),
    b"list" => Some(CmdList),
    b"invite" => Some(CmdInvite),
    b"kick" => Some(CmdKick),
    b"privmsg" => Some(CmdPrivateMsg),
    b"notice" => Some(CmdNotice),
    b"motd" => Some(CmdMsgOfTheDay),
    b"lusers" => Some(CmdLUsers),
    b"version" => Some(CmdVersion),
    b"stats" => Some(CmdStats),
    b"links" => Some(CmdLinks),
    b"time" => Some(CmdTime),
    b"connect" => Some(CmdConnect),
    b"trace" => Some(CmdTrace),
    b"admin" => Some(CmdAdmin),
    b"info" => Some(CmdInfo),
    b"servlist" => Some(CmdServiceList),
    b"squery" => Some(CmdServiceQuery),
    b"who" => Some(CmdWho),
    b"whois" => Some(CmdWhoIs),
    b"whowas" => Some(CmdWhoWas),
    b"kill" => Some(CmdKill),
    b"ping" => Some(CmdPing),
    b"pong" => Some(CmdPong),
    b"error" => Some(CmdError),
    b"away" => Some(CmdAway),
    b"rehash" => Some(CmdReHash),
    b"die" => Some(CmdDie),
    b"restart" => Some(CmdRestart),
    b"summon" => Some(CmdSummon),
    b"users" => Some(CmdUsers),
    b"wallops" => Some(CmdOperWall),
    b"userhost" => Some(CmdUserHost),
    b"ison" => Some(CmdIsOn),
    _ => None
  }
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

fn as_lowercase( bytes: &[u8] ) -> Vec<u8> {
  let mut new_bytes = Vec::with_capacity( bytes.len() );
  for c in bytes.iter() {
    new_bytes.push( match *c {
      0x41..0x5A => c ^ 0x20,
      a => a
    } );
  }
  new_bytes
}
