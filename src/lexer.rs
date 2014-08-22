use std::result::Result;
use std::option::Option;
use parse::{digit_char, letter_char};

struct Message<'a> {
  prefix: Option<&'a [u8]>,
  command: &'a [u8],
  params: Vec<&'a [u8]>
}

enum POption<T> {
  Prefix(T),
  NoPrefix,
  ErrPrefix
}

impl<T> POption<T> {
  fn unwrap<'a>( &'a self ) -> &'a T {
    match *self {
      Prefix(ref p) => p,
      NoPrefix => fail!("No Prefix"),
      ErrPrefix => fail!("Error"),
    }
  }
}

fn lex_prefix<'a>( msg_ref: &mut &'a [u8] ) -> POption<&'a [u8]> {
  let msg = *msg_ref;
  match  msg.get( 0 ) {
    Some(m) if *m == ':' as u8 => {},
    _ => return NoPrefix
  };
  let prefix_end = match msg.iter().skip(1).position( |&x| x == ' ' as u8 ) {
    Some(i) => i + 1, // Accout for skip.
    None => return ErrPrefix
  };
  if prefix_end + 1 >= msg.len() {
    return ErrPrefix;
  }
  match msg.get( prefix_end + 1 ) {
    None => return ErrPrefix,
    _ => {}
  };
  let prefix = msg.slice( 1, prefix_end );
  let new_msg = msg.slice_from( prefix_end + 1 );
  *msg_ref = new_msg;
  Prefix( prefix )
}

fn lex_command<'a>( msg_ref: &mut &'a [u8] ) -> Option<&'a [u8]> {
  let msg = *msg_ref;
  let command_end = match msg.get( 0 ) {
    Some(d) if digit_char( d ) => {
      match msg.iter().position( |x| !digit_char(x) ) {
        Some(i) if i == 3 => i,
        _ => return None
      }
    }
    Some(c) if letter_char( c ) => {
      match msg.iter().position( |x| !letter_char(x) ) {
        Some(i) => i,
        None => return None
      }
    }
    _ => return None
  };
  if command_end + 1 >= msg.len() {
    return None;
  }
  let command = msg.slice_to( command_end );
  let new_msg = msg.slice_from( command_end + 1 );
  *msg_ref = new_msg;
  Some( command )
}

fn lex_params<'a>( msg_ref: &mut &'a [u8] ) -> Vec<&'a [u8]> {
  let mut params = Vec::new();
  let mut msg = *msg_ref;
  while msg.len() > 1 {
    let param_end = match msg.iter().position( |&x| x == ' ' as u8 ) {
      Some(i) => i,
      None => msg.len() - 1
    };
    params.push( msg.slice_to( param_end ) );
    msg = msg.slice_from( param_end + 1 );
  }
  params
}

fn lex_msg<'a>( in_msg: &'a [u8] ) -> Result<Message<'a>, ()> {
  let mut msg = in_msg;
  let prefix = match lex_prefix( &mut msg ) {
    Prefix(p) => Some(p),
    NoPrefix => None,
    ErrPrefix => return Err( () )
  };
  let command = match lex_command( &mut msg ) {
    Some(m) => m,
    None => return Err( () )
  };
  let params = lex_params( &mut msg );

  Ok( Message{ prefix: prefix, command: command, params: params } )
}

fn test_prefix() {
  let vec = vec!(':' as u8, 3, 2, ' ' as u8, 4);
  println!( "{}", lex_prefix(&mut vec.as_slice()).unwrap() );
}

fn test_command() {
  let vec = vec!('a' as u8, 'c' as u8, ' ' as u8, 4);
  println!( "{}", lex_command(&mut vec.as_slice()).unwrap() );
}

fn test_lex() {
  let mut vec = vec!(':' as u8, 1, 2, ' ' as u8, 'a' as u8, 'b' as u8, ' ' as u8, 4, 5, '\n' as u8);
  {
    let msg = lex_msg( vec.as_slice() ).unwrap();
    println!( "{} {} {}", msg.prefix.unwrap(), msg.command, msg.params );
  }
  vec = vec!(':' as u8, 1, 2, ' ' as u8, '1' as u8, '2' as u8, '3' as u8, ' ' as u8, 4, 5, '\n' as u8);
  {
    let msg = lex_msg( vec.as_slice() ).unwrap();
    println!( "{} {} {}", msg.prefix.unwrap(), msg.command, msg.params );
  }
}

#[main]
fn main() {
  test_prefix();
  test_command();
  test_lex();
}
