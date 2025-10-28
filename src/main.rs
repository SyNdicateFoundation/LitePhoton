use crate::argument_parser::ARGUMENTS;
use crate::environment::{Environment, ENVIRONMENT};
use crate::input::Input;
use crate::logger::log_info;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::OnceLock;
use crate::read_util::Mode;

mod logger;
mod argument_parser;
mod environment;
mod read_util;
mod input;

/// Entry point
fn main() {
    IS_STDIN.set(atty::is(atty::Stream::Stdin)).ok();

    argument_parser::parse_arguments();

    Environment::setup(ARGUMENTS.get().unwrap());

    let env = ENVIRONMENT.get().unwrap();

    logger::setup_logger();

    log_info(&format!("Starting up LitePhoton with this environment: {:?}", env));


    // for file in &env.file {
        read_util::read_input(Mode::from_str(&env.method).unwrap(),
                          if !*IS_STDIN.get().unwrap() && !env.bypass_stdin_check {
                                  Input::Stdin(())
                              } else {
                                  Input::File(PathBuf::from(&env.file))
                              }
                          , env.stable, &env.keyword);
    // }
}

static IS_STDIN: OnceLock<bool> = OnceLock::new();