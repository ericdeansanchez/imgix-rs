use imgix::command_prelude::{App, SubCommand};
use std::io::{self, Write};

/// Return the `SubCommand` associated with `pre-commit`. This
/// function can be invoked through the imgix-cli like so:
pub fn cli() -> App {
    SubCommand::with_name("pre-commit").about("Set up git pre-commit configuration.")
}

/// Execute the `pre-commit` command.
///
/// This function _does not_ modify any directory or file structure. This
/// function writes instructions and code to stdout. The instructions are
/// to copy the accompanying code and paste it in `.git/hooks/pre-commit`.
///
/// This is a meant to be a convenience, mostly internally. It's easy to
/// forget to format code before committing. This script is used internally
/// to say, "at the time of commit, the code was formatted, built, and tested."
///
/// This command can be extended to support functionality from passing a
/// flag to the pre-commit sub-command like so: `imgix pre-commit --write`
/// This way we avoid writing anything to disk that isn't wanted or warranted.
pub fn exec() -> io::Result<()> {
    let pre_commit_body = r#"
Hey! Thanks for using this pre-commit hook! Your future-self thanks you.
In the repository there's a directory named .git/hooks/ and it contains
executable files:

.git
└──hooks
    ├── pre-commit
    └── other-stuff

Create your own `pre-commit` file if one does not already exist:
    `$ touch .git/hooks/pre-commit`

Place the following code in .git/hooks/pre-commit.
Note: you may have to
    `$ chmod +x .git/hooks/pre-commit`

before the file is executable.

#!/bin/sh
#
# This hook is meant to be helpful and its use is optional. To remove
# this pre-commit hook, delete it from your .git/hooks/ directory.
#
# You can also invoke git-commit as:
# `% git commit --no-verify`
#
# This option bypasses the pre-commit and commit-msg hooks.
#
# Below are the commands that are executed:
# Build for release.
cargo build --release

# Format the repository.
cargo fmt

# Test the repository.
cargo test 
"#;
    io::stdout().write_all(pre_commit_body.as_bytes())?;
    Ok(())
}
