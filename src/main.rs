mod bookmark;
mod bookmarks;

use crate::{bookmark::Bookmark, bookmarks::Bookmarks};
use std::ffi::{CStr, CString};

const LIST_SUBCOMMAND: &str = "-l";
const REMOVE_SUBCOMMAND: &str = "-rm";
const ADD_SUBCOMMAND: &str = "-a";
const KEY_ARGNAME: &str = "KEY";
const ADDR_ARGNAME: &str = "VALUE";
const ARGS_ARGNAME: &str = "args";

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{:?}", e);
        std::process::exit(1)
    }
}

fn try_main() -> anyhow::Result<()> {
    let app = make_arg_parser().get_matches();
    let mut bookmarks = Bookmarks::new()?;
    match app.subcommand() {
        (LIST_SUBCOMMAND, _) => {
            println!("{}", bookmarks)
        }
        (REMOVE_SUBCOMMAND, Some(args)) => {
            let k = args.value_of(KEY_ARGNAME).unwrap();
            if bookmarks.remove(k).is_none() {
                return Err(anyhow::anyhow!(r#"Key "{}" not found."#, k));
            }
            bookmarks.save()?;
        }
        (ADD_SUBCOMMAND, Some(args)) => {
            bookmarks.insert(
                args.value_of(KEY_ARGNAME).unwrap(),
                args.value_of(ADDR_ARGNAME).unwrap(),
                args.value_of(ARGS_ARGNAME),
            );
            bookmarks.save()?;
        }
        ("", None) => {
            let key = app.value_of(KEY_ARGNAME).unwrap();
            let val = bookmarks
                .remove(key)
                .ok_or_else(|| anyhow::anyhow!(anyhow::anyhow!(r#"Key "{}" not found."#, key)))?;
            start_ssh(val)?;
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn make_arg_parser() -> clap::App<'static, 'static> {
    use clap::{
        app_from_crate, crate_authors, crate_description, crate_name, crate_version, AppSettings,
        Arg, SubCommand,
    };
    let list_subcommand = SubCommand::with_name(LIST_SUBCOMMAND).about("List bookmarks and exit.");
    let rm_subcommand = SubCommand::with_name(REMOVE_SUBCOMMAND)
        .about("Remove a bookmark.")
        .arg(Arg::with_name(KEY_ARGNAME).required(true));
    let add_subcommand = SubCommand::with_name(ADD_SUBCOMMAND)
        .about("Add a bookmark. Arguments should be in the format KEY USER@IP.")
        .arg(
            Arg::with_name(KEY_ARGNAME)
                .help("Key to use for the new bookmark.")
                .required(true),
        )
        .arg(
            Arg::with_name(ADDR_ARGNAME)
                .help("Addresses should be in the format USER@IP.")
                .required(true),
        )
        .arg(
            Arg::with_name(ARGS_ARGNAME)
                .help("Custom arguments to pass to the ssh command, e.g. '-i ~/.ssh/id_rsa'.")
                .last(true),
        );
    app_from_crate!()
        .settings(&[
            AppSettings::ArgsNegateSubcommands,
            AppSettings::SubcommandsNegateReqs,
            AppSettings::DisableHelpSubcommand,
        ])
        .arg(
            Arg::with_name(KEY_ARGNAME)
                .help("Bookmark to connect to.")
                .required(true),
        )
        .subcommands([list_subcommand, rm_subcommand, add_subcommand])
}

fn start_ssh(b: Bookmark) -> Result<core::convert::Infallible, nix::Error> {
    const SSH_CMD_BYTES: [u8; 4] = *b"ssh\0";
    let bmark_cmd = b.into_cmd().unwrap();
    let ssh_cmd = CStr::from_bytes_with_nul(&SSH_CMD_BYTES).unwrap();
    // NOTE: For whatever reason, likely a bug in the nix crate, the first element of execvp's
    //       argv is ignored. This is why the variable tmp is being used here.
    let mut args = vec![CStr::from_bytes_with_nul(&[0]).unwrap()];
    args.extend(bmark_cmd.iter().map(CString::as_ref));
    nix::unistd::execvp(ssh_cmd, &args)
}
