use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(
    author,
    version,
    about,
    long_about = None,
    subcommand_negates_reqs = true,
    args_conflicts_with_subcommands = true
)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Command>,
    #[clap(required = true)]
    pub key: Option<String>,
}

impl Args {
    pub fn parse() -> Args {
        <Args as Parser>::parse()
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[clap(about = "List bookmarks.")]
    List,
    #[clap(about = "Remove a bookmark.")]
    Rm { key: String },
    #[clap(about = "Add a bookmark. Arguments should be in the format KEY USER@IP.")]
    Add {
        #[clap(help = "Key to use for the new bookmark.")]
        key: String,
        #[clap(help = "Addresses should be in the format USER@IP.")]
        val: String,
        #[clap(help = "Custom arguments to pass to the ssh command, e.g. '-i
       │  ~/.ssh/id_rsa'.")]
        ssh_args: Option<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_args() {
        dbg!(
            Args::try_parse_from(["", "add", "foo", "bar@192.168.2.1", r#""-i ~/.ssh/foo""#])
                .unwrap()
        );
    }
}
