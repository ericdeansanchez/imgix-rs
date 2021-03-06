//! # Generates the top-level cli.
use crate::commands;
use imgix::command_prelude::*;

/// Build an `App`. This `App` is comprised of information read from cargo
/// environment variables, a list of settings, and a list of a list of all
/// supported sub-commands.
pub fn app() -> App {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .settings(&[
            AppSettings::UnifiedHelpMessage,
            AppSettings::DeriveDisplayOrder,
            AppSettings::VersionlessSubcommands,
            AppSettings::AllowExternalSubcommands,
            AppSettings::SubcommandRequiredElseHelp,
        ])
        .subcommands(commands::all_sub_commands())
}
