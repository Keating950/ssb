mod bookmark;
mod bookmarks;

use crate::bookmarks::Bookmarks;
use std::{
    ffi::{CStr, CString},
    io::{self, prelude::*},
};

const LIST_SUBCOMMAND: &str = "-l";
const REMOVE_SUBCOMMAND: &str = "-rm";
const ADD_SUBCOMMAND: &str = "-a";
const KEY_ARGNAME: &str = "KEY";
const ADDR_ARGNAME: &str = "VALUE";
const ARGS_ARGNAME: &str = "args";

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(1)
    }
}

fn try_main() -> anyhow::Result<()> {
    let app = make_arg_parser().get_matches();
    let mut bookmarks = Bookmarks::load()?;
    match app.subcommand() {
        Some((LIST_SUBCOMMAND, _)) => {
            println!("{}", bookmarks)
        }
        Some((REMOVE_SUBCOMMAND, args)) => {
            let k = args.value_of(KEY_ARGNAME).unwrap();
            if bookmarks.remove(k).is_none() {
                return Err(anyhow::anyhow!(r#"Key "{}" not found."#, k));
            }
            bookmarks.save()?;
        }
        Some((ADD_SUBCOMMAND, args)) => {
            let key = args.value_of(KEY_ARGNAME).unwrap();
            if let Some(bmark) = bookmarks.get(key) {
                let msg = format!(
                    "A bookmark named \"{}\" already exists. Overwrite it?\n{}\n[y/n]: ",
                    key, bmark
                );
                let ln = get_line(&msg)?;
                if !matches!(ln.as_str(), "Y" | "y") {
                    return Ok(());
                }
            }
            bookmarks.insert(
                key,
                args.value_of(ADDR_ARGNAME).unwrap(),
                args.value_of(ARGS_ARGNAME),
            );
            bookmarks.save()?;
        }
        None => {
            let key = app.value_of(KEY_ARGNAME).unwrap();
            let val = bookmarks
                .remove(key)
                .ok_or_else(|| anyhow::anyhow!(r#"Key "{}" not found."#, key))?;
            let cmd = val.into_cmd()?;
            start_ssh(cmd.iter().map(CString::as_ref))?;
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn make_arg_parser() -> clap::App<'static> {
    use clap::{app_from_crate, App, AppSettings, Arg};
    let list_subcommand = App::new(LIST_SUBCOMMAND).about("List bookmarks and exit.");
    let rm_subcommand = App::new(REMOVE_SUBCOMMAND)
        .about("Remove a bookmark.")
        .arg(Arg::new(KEY_ARGNAME).required(true));
    let add_subcommand = App::new(ADD_SUBCOMMAND)
        .about("Add a bookmark. Arguments should be in the format KEY USER@IP.")
        .arg(
            Arg::new(KEY_ARGNAME)
                .help("Key to use for the new bookmark.")
                .required(true),
        )
        .arg(
            Arg::new(ADDR_ARGNAME)
                .help("Addresses should be in the format USER@IP.")
                .required(true),
        )
        .arg(
            Arg::new(ARGS_ARGNAME)
                .help("Custom arguments to pass to the ssh command, e.g. '-i ~/.ssh/id_rsa'.")
                .last(true),
        );
    app_from_crate!()
        .global_setting(AppSettings::ArgsNegateSubcommands)
        .global_setting(AppSettings::SubcommandsNegateReqs)
        .global_setting(AppSettings::DisableHelpSubcommand)
        .arg(
            Arg::new(KEY_ARGNAME)
                .help("Bookmark to connect to.")
                .required(true),
        )
        .subcommands([list_subcommand, rm_subcommand, add_subcommand])
}

fn get_line(msg: &str) -> io::Result<String> {
    let mut buf = String::default();
    print!("{}", msg);
    io::stdout().flush()?;
    let stdin = io::stdin();
    stdin.lock().read_line(&mut buf)?;
    if !buf.is_empty() {
        buf.truncate(buf.len() - 1);
    }
    Ok(buf)
}

fn start_ssh<'a, T>(cmd: T) -> Result<core::convert::Infallible, nix::Error>
where
    T: Iterator<Item = &'a CStr>,
{
    const SSH_CMD_BYTES: [u8; 4] = *b"ssh\0";
    let ssh_cmd = CStr::from_bytes_with_nul(&SSH_CMD_BYTES).unwrap();
    let mut args = vec![ssh_cmd];
    args.extend(cmd);
    nix::unistd::execvp(ssh_cmd, &args)
}

#[cfg(test)]
#[macro_export]
macro_rules! assert_ok {
    ($v:ident) => {
        assert!($v.is_ok(), "{}", $v.unwrap_err())
    };
    ($e:expr) => {{
        let tmp = $e;
        assert!(tmp.is_ok(), "{}", tmp.unwrap_err())
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arg_parser() {
        let args = [
            vec!["ssb", "foobar_key"],
            vec!["ssb", "-a", "foobar_key", "baz@192.168.0.1"],
            vec!["ssb", "-l"],
            vec!["ssb", "-rm", "foobar_key"],
        ];
        for a in args {
            assert_ok!(make_arg_parser().try_get_matches_from(a));
        }
    }
}
