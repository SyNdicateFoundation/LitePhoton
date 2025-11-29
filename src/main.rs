use crate::argument_parser::ARGUMENTS;
use crate::environment::{Environment, ENVIRONMENT};
use crate::input::Input;
use crate::logger::log_info;
use crate::read_util::Mode;
use std::path::PathBuf;
use std::str::FromStr;

mod logger;
mod argument_parser;
mod environment;
mod read_util;
mod input;

/// Entry point
fn main() {
    argument_parser::parse_arguments();

    Environment::setup(ARGUMENTS.get().unwrap());

    let env = ENVIRONMENT.get().unwrap();

    logger::setup_logger(env.debug.clone());

    log_info(&format!("Starting up LitePhoton with this environment: {:?}", env));


    for file in &env.file {
        read_util::read_input(Mode::from_str(&env.method).unwrap(),
                          if !atty::is(atty::Stream::Stdin) && !env.bypass_stdin_check {
                                  Input::Stdin(())
                              } else {
                                  Input::File(PathBuf::from(file))
                              }
                          , env.stable, &env.keyword);
    }
}