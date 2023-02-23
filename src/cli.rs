use clap::{crate_authors, crate_description, crate_name, crate_version, Command};

use crate::errors::AppError;

/* -------------------------------------------------------------------------- */
/*                                   MODULES                                  */
/* -------------------------------------------------------------------------- */
pub mod serve;

pub struct Cli {}

impl Cli {
    pub async fn parse() -> Result<(), AppError> {
        let matches = Command::new(crate_name!())
            .author(crate_authors!())
            .about(crate_description!())
            .version(crate_version!())
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(serve::command())
            .get_matches();

        match matches.subcommand() {
            Some((serve::COMMAND_NAME, args)) => serve::execute(args).await,
            _ => unreachable!(),
        }
    }
}
