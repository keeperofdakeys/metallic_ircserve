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
  match vec_get( msg, 0 ) {
    Some(m) if *m == ':' as u8 => {},
    _ => return None
  };
  let prefix_end = match msg.iter().position( |&x| x == ' ' as u8 ) {
    Some(i) => i,
    None => return None
  };
  let prefix = Vec::from_slice( msg.slice( 1, prefix_end ) );
  let new_msg = Vec::from_slice( msg.slice_from( prefix_end + 1 ) );
  msg.truncate( 0 );
  msg.push_all_move( new_msg );
  Some( prefix )
}

fn lex_command( msg: &mut Vec<u8> ) -> Option<Vec<u8>> {
  let command_end = match msg.iter().position( |&x| x == ' ' as u8 ) {
    Some(i) => i,
    None => msg.len() - 2
  };
  let command = Vec::from_slice( msg.slice_to( command_end ) );
  let new_msg = Vec::from_slice( msg.slice_from( command_end + 1 ) );
  msg.truncate( 0 );
  msg.push_all_move( new_msg );
  Some( command )
}

fn lex_params( msg: &mut Vec<u8> ) -> Vec<Vec<u8>> {
  Vec::new()
}

fn lex_msg( in_msg: Vec<u8> ) -> Result<Message, ()> {
  let mut msg = in_msg.clone();
  let prefix = lex_prefix( &mut msg );
  let command = match lex_command( &mut msg ) {
    Some(m) => m,
    None => return Err( () )
  };
  let params = lex_params( &mut msg );

  Ok( Message{ prefix: prefix, command: command, params: params } )
}

fn test_prefix() {
  let mut vec = vec!(':' as u8, 3, 2, ' ' as u8, 4);
  println!( "1 {}", vec );
  println!( "3 {}", lex_prefix(&mut vec).unwrap());
  println!( "2 {}", vec );
}

fn main() {
  test_prefix();
}
