use std::result::Result;
use std::option::Option;

struct Message {
  prefix: Option<Vec<u8>>,
  command: Vec<u8>,
  params: Vec<Vec<u8>>
}

type ByteString = Vec<u8>;

fn vec_get<'a, T>( vec: &'a Vec<T>, pos: uint ) -> Option<&'a T> {
  if vec.len() >= pos + 1 {
    Some(vec.get(pos))
  } else {
    None
  }
}

fn lex_prefix( msg: &mut Vec<u8> ) -> Option<Vec<u8>> {
  None
  //Some(m) if m == &(':' as u8) => true,
  //let prefix_index = match msg.iter().position(|x| x == &(' ' as u8)) {
}

fn lex_command( msg: &mut Vec<u8> ) -> Option<Vec<u8>> {
  None
}

fn lex_params( msg: &mut Vec<u8> ) -> Vec<Vec<u8>> {
  Vec::new()
}

fn lex_msg( mut msg: Vec<u8> ) -> Result<Message, ()> {
  let prefix = lex_prefix( &mut msg );
  let command = match lex_command( &mut msg ) {
    Some(m) => m,
    None => return Err(())
  };
  let params = lex_params( &mut msg );

  Ok(Message{prefix: prefix, command: command, params: params})
}

