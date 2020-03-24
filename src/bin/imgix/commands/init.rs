use imgix::command_prelude::{App, Arg, SubCommand};

pub fn cli() -> App {
    SubCommand::with_name("init")
        .about("Example init command.")
        .arg(
            Arg::with_name("name")
                .help("The name of argument to init.")
                .required(true),
        )
}
