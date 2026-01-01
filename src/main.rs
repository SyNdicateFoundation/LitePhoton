use crate::argument_parser::ARGUMENTS;
use crate::environment::{ENVIRONMENT, Environment};
use crate::input::Input;
use crate::logger::log_info;
use crate::read_util::Mode;
use std::path::PathBuf;
use std::str::FromStr;

mod argument_parser;
mod environment;
mod input;
mod logger;
mod read_util;

/// Entry point
fn main() {
    argument_parser::parse_arguments();

    Environment::setup(ARGUMENTS.get().expect("main.rs: Cannot get environment"));

    let env = ENVIRONMENT.get().expect("main.rs: Cannot get environment");

    logger::setup_logger(env.debug);

    log_info(&format!(
        "Starting up LitePhoton with this environment: {:?}",
        env
    ));

    for file in &env.file {
        read_util::read_input(
            Mode::from_str(&env.method).expect("main.rs: Provided mode not found"),
            if !atty::is(atty::Stream::Stdin) && !env.bypass_stdin_check {
                Input::Stdin(())
            } else {
                Input::File(PathBuf::from(file))
            },
            env.stable,
            &env.keyword,
        );
    }
}
