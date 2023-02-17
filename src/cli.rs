use anyhow;
use clap::{crate_authors, crate_description, crate_name, crate_version, Command};

pub struct Cli {}

impl Cli {
    pub async fn parse() -> anyhow::Result<()> {
        let matches = Command::new(crate_name!())
            .author(crate_authors!())
            .about(crate_description!())
            .version(crate_version!())
            .subcommand_required(true)
            .arg_required_else_help(true)
            .get_matches();

        match matches.subcommand() {
            _ => unreachable!(),
        }
    }
}
