use parse::*;
use irc::IRCConfig;
use irc::{DestUser, DestChan};

fn cmd_act<'a>( config: &IRCConfig, cmd: Command, params: &[&[u8]] ) {
  match cmd {
    CmdPassword => handle_pass( config, params ),
    CmdNickname => handle_nick( config, params ),
    CmdUser => handle_user( config, params ),
    CmdOper => handle_oper( config, params ),
    CmdMode => handle_mode( config, params ),
    CmdService => handle_service( config, params ),
    CmdQuit => handle_quit( config, params ),
    CmdSQuit => handle_squit( config, params ),
    CmdJoin => handle_join( config, params ),
    CmdPart => handle_part( config, params ),
    CmdTopic => handle_topic( config, params ),
    CmdNames => handle_names( config, params ),
    CmdList => handle_list( config, params ),
    CmdInvite => handle_invite( config, params ),
    CmdKick => handle_kick( config, params ),
    CmdPrivateMsg => handle_privmsg( config, params ),
    CmdNotice => handle_notice( config, params ),
    CmdMsgOfTheDay => handle_motd( config, params ),
    CmdLUsers => handle_lusers( config, params ),
    CmdVersion => handle_version( config, params ),
    CmdStats => handle_stats( config, params ),
    CmdLinks => handle_links( config, params ),
    CmdTime => handle_time( config, params ),
    CmdConnect => handle_connect( config, params ),
    CmdTrace => handle_trace( config, params ),
    CmdAdmin => handle_admin( config, params ),
    CmdInfo => handle_info( config, params ),
    CmdServiceList => handle_servlist( config, params ),
    CmdServiceQuery => handle_squery( config, params ),
    CmdWho => handle_who( config, params ),
    CmdWhoIs => handle_whois( config, params ),
    CmdWhoWas => handle_whowas( config, params ),
    CmdKill => handle_kill( config, params ),
    CmdPing => handle_ping( config, params ),
    CmdPong => handle_pong( config, params ),
    CmdError => handle_error( config, params ),
    CmdAway => handle_away( config, params ),
    CmdReHash => handle_rehash( config, params ),
    CmdDie => handle_die( config, params ),
    CmdRestart => handle_restart( config, params ),
    CmdSummon => handle_summon( config, params ),
    CmdUsers => handle_users( config, params ),
    CmdOperWall => handle_wallops( config, params ),
    CmdUserHost => handle_userhost( config, params ),
    CmdIsOn => handle_ison( config, params ),
  }
}


fn handle_pass( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_nick( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_user( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_oper( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_mode( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_service( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_quit( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_squit( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_join( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_part( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_topic( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_names( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_list( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_invite( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_kick( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_privmsg( config: &IRCConfig, params: &[&[u8]] ) {
  let dest_str = match params.get( 0 ) {
    Some(s) => *s,
    None => return
  };
  let msg = match params.get( 1 ) {
    Some(m) => *m,
    None => return
  };
  let result = match config.send_msg( dest_str, msg ) {
    Ok(_) => {},
    Err(_) => {}
  };
}

fn handle_notice( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_motd( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_lusers( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_version( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_stats( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_links( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_time( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_connect( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_trace( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_admin( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_info( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_servlist( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_squery( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_who( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_whois( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_whowas( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_kill( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_ping( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_pong( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_error( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_away( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_rehash( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_die( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_restart( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_summon( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_users( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_wallops( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_userhost( config: &IRCConfig, params: &[&[u8]] ) {
}

fn handle_ison( config: &IRCConfig, params: &[&[u8]] ) {
}

