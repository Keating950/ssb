mod bookmarks;

use crate::bookmarks::Bookmarks;
use std::ffi::CStr;

const LIST_SUBCOMMAND: &'static str = "-l";
const REMOVE_SUBCOMMAND: &'static str = "-r";
const ADD_SUBCOMMAND: &'static str = "-a";
const KEY_ARGNAME: &'static str = "KEY";
const VALUE_ARGNAME: &'static str = "VALUE";

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
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
                args.value_of(VALUE_ARGNAME).unwrap(),
            );
            bookmarks.save()?;
        }
        ("", None) => {
            let key = app.value_of(KEY_ARGNAME).unwrap();
            let val = bookmarks
                .get(key)
                .ok_or_else(|| anyhow::anyhow!(anyhow::anyhow!(r#"Key "{}" not found."#, key)))?;
            start_ssh(val);
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
    let list_subcommand = SubCommand::with_name(LIST_SUBCOMMAND).help("List bookmarks and exit.");
    let rm_subcommand = SubCommand::with_name(REMOVE_SUBCOMMAND)
        .help("Remove a bookmark.")
        .arg(Arg::with_name(KEY_ARGNAME));
    let add_subcommand = SubCommand::with_name(ADD_SUBCOMMAND)
        .help("Add a bookmark. Arguments should be in the format KEY USER@IP.")
        .arg(Arg::with_name(KEY_ARGNAME))
        .arg(Arg::with_name(VALUE_ARGNAME));
    app_from_crate!()
        .setting(AppSettings::ArgsNegateSubcommands)
        .setting(AppSettings::SubcommandsNegateReqs)
        .arg(
            Arg::with_name(KEY_ARGNAME)
                .help("Bookmark to connect to.")
                .required(true),
        )
        .subcommands([list_subcommand, rm_subcommand, add_subcommand])
}

fn start_ssh(addr: &str) {
    const SSH_CMD_BYTES: [u8; 4] = *b"ssh\0";
    let addr_bytes: Vec<u8> = addr.as_bytes().iter().chain(&[0]).map(|c| *c).collect();
    unsafe {
        let ssh_cmd = CStr::from_bytes_with_nul_unchecked(&SSH_CMD_BYTES);
        let addr_cstr = CStr::from_bytes_with_nul_unchecked(&addr_bytes);
        // NOTE: For whatever reason, likely a bug in the nix crate, the first element of execvp's
        // argv is ignored. This is why the variable tmp is being used here.
        let tmp = CStr::from_bytes_with_nul_unchecked(&[0]);
        match nix::unistd::execvp::<&CStr>(ssh_cmd, &[tmp, addr_cstr]) {
            Ok(_) => (),
            Err(_) => std::hint::unreachable_unchecked(),
        }
    }
}
