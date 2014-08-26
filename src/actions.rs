use parse::*;
use irc::{IRCConfig, User, Channel};
use irc::{DestUser, DestChan};

fn cmd_act<'a>( user: User, chan: Channel, user: User, chan: Channel, config: &IRCConfig, cmd: Command, params: &[&[u8]] ) {
  match cmd {
    CmdPassword => handle_pass( user, chan, config, params ),
    CmdNickname => handle_nick( user, chan, config, params ),
    CmdUser => handle_user( user, chan, config, params ),
    CmdOper => handle_oper( user, chan, config, params ),
    CmdMode => handle_mode( user, chan, config, params ),
    CmdService => handle_service( user, chan, config, params ),
    CmdQuit => handle_quit( user, chan, config, params ),
    CmdSQuit => handle_squit( user, chan, config, params ),
    CmdJoin => handle_join( user, chan, config, params ),
    CmdPart => handle_part( user, chan, config, params ),
    CmdTopic => handle_topic( user, chan, config, params ),
    CmdNames => handle_names( user, chan, config, params ),
    CmdList => handle_list( user, chan, config, params ),
    CmdInvite => handle_invite( user, chan, config, params ),
    CmdKick => handle_kick( user, chan, config, params ),
    CmdPrivateMsg => handle_privmsg( user, chan, config, params ),
    CmdNotice => handle_notice( user, chan, config, params ),
    CmdMsgOfTheDay => handle_motd( user, chan, config, params ),
    CmdLUsers => handle_lusers( user, chan, config, params ),
    CmdVersion => handle_version( user, chan, config, params ),
    CmdStats => handle_stats( user, chan, config, params ),
    CmdLinks => handle_links( user, chan, config, params ),
    CmdTime => handle_time( user, chan, config, params ),
    CmdConnect => handle_connect( user, chan, config, params ),
    CmdTrace => handle_trace( user, chan, config, params ),
    CmdAdmin => handle_admin( user, chan, config, params ),
    CmdInfo => handle_info( user, chan, config, params ),
    CmdServiceList => handle_servlist( user, chan, config, params ),
    CmdServiceQuery => handle_squery( user, chan, config, params ),
    CmdWho => handle_who( user, chan, config, params ),
    CmdWhoIs => handle_whois( user, chan, config, params ),
    CmdWhoWas => handle_whowas( user, chan, config, params ),
    CmdKill => handle_kill( user, chan, config, params ),
    CmdPing => handle_ping( user, chan, config, params ),
    CmdPong => handle_pong( user, chan, config, params ),
    CmdError => handle_error( user, chan, config, params ),
    CmdAway => handle_away( user, chan, config, params ),
    CmdReHash => handle_rehash( user, chan, config, params ),
    CmdDie => handle_die( user, chan, config, params ),
    CmdRestart => handle_restart( user, chan, config, params ),
    CmdSummon => handle_summon( user, chan, config, params ),
    CmdUsers => handle_users( user, chan, config, params ),
    CmdOperWall => handle_wallops( user, chan, config, params ),
    CmdUserHost => handle_userhost( user, chan, config, params ),
    CmdIsOn => handle_ison( user, chan, config, params ),
  }
}


fn handle_pass( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_nick( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_user( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_oper( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_mode( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_service( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_quit( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
  config.kill_user( user );
}

fn handle_squit( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_join( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_part( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_topic( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_names( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_list( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_invite( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_kick( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_privmsg( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
  let dest_str = match params.get( 0 ) {
    Some(s) => *s,
    None => return
  };
  let msg = match params.get( 1 ) {
    Some(m) => *m,
    None => return
  };
  let result = match config.send_msg( user, dest_str, msg ) {
    Ok(_) => {},
    Err(_) => {}
  };
}

fn handle_notice( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_motd( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_lusers( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_version( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_stats( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_links( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_time( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_connect( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_trace( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_admin( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_info( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_servlist( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_squery( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_who( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_whois( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_whowas( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_kill( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_ping( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_pong( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_error( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_away( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_rehash( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_die( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_restart( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_summon( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_users( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_wallops( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_userhost( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_ison( user: User, chan: Channel, config: &IRCConfig, params: &[&[u8]] ) {
}

