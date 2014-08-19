use std::result::Result;
use std::option::Option;

struct Message {
  prefix: Option<Vec<u8>>,
  command: Vec<u8>,
  prefixes: Vec<Vec<u8>>
}

type RawMessage = Vec<u8>;

fn vec_get<'a, T>( vec: &'a Vec<T>, pos: uint ) -> Option<&'a T> {
  if vec.len() >= pos + 1 {
    Some(vec.get(pos))
  } else {
    None
  }
}

fn lex_msg( msg: RawMessage ) -> Result<Message, ()> {
  let test_none: Vec<u8> = Vec::new();
  let prefix_present = match vec_get(&msg, 0) {
    Some(m) if m == &(':' as u8) => true,
    None => return Err(()),
    _ => false
  };

  let prefix_index = match msg.iter().position(|x| x == &(' ' as u8)) {
    Some(i) => i,
    None => return Err(())
  };

  //Message{prefix: None, command: test_none.clone(), prefixes: Vec::new()};
  Err(())
}

