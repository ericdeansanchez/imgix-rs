use imgix::command_prelude::*;

/// This function exists to export the cli configuration for all sub
/// commands. It works by populating a `Vec` with clap `App`s. Each
/// `App` defines the command line interface (cli) for _it's module_.
pub fn all_sub_commands() -> Vec<App> {
    vec![init::cli(), pre_commit::cli()]
}

pub mod init;
pub mod pre_commit;
