#![warn(clippy::all, clippy::pedantic)]
mod args;
mod bookmark;
mod bookmarks;

use crate::{
    args::{Args, Command},
    bookmarks::Bookmarks,
};
use std::{
    ffi::{CStr, CString},
    io::{self, prelude::*},
};

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{e}");
        std::process::exit(1)
    }
}

fn try_main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut bookmarks = Bookmarks::load()?;
    match args.command {
        Some(Command::List) => {
            println!("{bookmarks}");
        }
        Some(Command::Rm { key }) => {
            if bookmarks.remove(&key).is_none() {
                return Err(anyhow::anyhow!(r#"Key "{}" not found."#, key));
            }
            bookmarks.save()?;
        }
        Some(Command::Add { key, val, ssh_args }) => {
            if let Some(bmark) = bookmarks.get(&key) {
                let msg = format!(
                    "A bookmark named \"{}\" already exists. Overwrite it?\n{}\n[y/n]: ",
                    key, bmark
                );
                let ln = get_line(&msg)?;
                if !matches!(ln.as_str(), "Y" | "y") {
                    return Ok(());
                }
            }
            bookmarks.insert(&key, &val, ssh_args.as_deref());
            bookmarks.save()?;
        }
        None => {
            let key = args.key.unwrap();
            let val = bookmarks
                .remove(&key)
                .ok_or_else(|| anyhow::anyhow!(r#"Key "{}" not found."#, key))?;
            let cmd = val.into_cmd()?;
            start_ssh(cmd.iter().map(CString::as_ref))?;
        }
    }
    Ok(())
}

fn get_line(msg: &str) -> io::Result<String> {
    let mut buf = String::default();
    print!("{msg}");
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
