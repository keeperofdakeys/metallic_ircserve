use std::result::Result;
use std::option::Option;
use parse::{digit_char, letter_char, no_spec_char};

pub struct LexerMsg<'a> {
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
  let new_msg = msg.slice_from( command_end );
  *msg_ref = new_msg;
  Some( command )
}

fn lex_param<'a>( msg_ref: &mut &'a[u8], trailing: bool ) -> Option<&'a [u8]> {
  let msg = *msg_ref;
  let test = match trailing {
    true => |x| !no_spec_char(x) && *x != ' ' as u8 && *x != ':' as u8,
    false => |x| !no_spec_char(x) && *x != ':' as u8
  };
  let end = match msg.iter().position( test ) {
    Some(i) => i,
    None => return None
  };
  if end >= msg.len() {
    return None
  }
  let param = msg.slice_to( end );
  *msg_ref = msg.slice_from( end );
  Some(param)
}

fn lex_params<'a>( msg_ref: &mut &'a [u8] ) -> Vec<&'a [u8]> {
  let mut msg = *msg_ref;
  let mut params = Vec::new();
  for x in range(0u,14) {
    match msg.get( 0 ) {
      Some(c) if *c == ' ' as u8 => {},
      _ => break
    }
    match msg.get( 1 ) {
      Some(c) if *c == ':' as u8 => {
        if msg.len() < 3 {
          break
        }
        msg = msg.slice_from( 2 );
        match lex_param( &mut msg, true ) {
          Some(p) => params.push(p),
          None => {}
        }
        break
      }
      Some(_) => {
        msg = msg.slice_from( 1 );
        match lex_param( &mut msg, false ) {
          Some(p) => params.push(p),
          None => break
        }
      }
      None => break
    }
  }
  params
}

fn lex_msg<'a>( in_msg: &'a [u8] ) -> Result<LexerMsg<'a>, ()> {
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

  Ok( LexerMsg{ prefix: prefix, command: command, params: params } )
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
  let mut vec = vec!(':' as u8, 1, 2, ' ' as u8, 'a' as u8, 'b' as u8, ' ' as u8, 97, 98, ' ' as u8, 98, 87, ' ' as u8, ':' as u8, 54, 51, '\n' as u8);
  {
    let msg = lex_msg( vec.as_slice() ).unwrap();
    println!( "{} {} {}", msg.prefix.unwrap(), msg.command, msg.params );
  }
  vec = vec!(':' as u8, 1, 2, ' ' as u8, '1' as u8, '2' as u8, '3' as u8, ' ' as u8, ':' as u8, 49, 55, '\n' as u8);
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
