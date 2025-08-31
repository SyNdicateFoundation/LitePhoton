use crate::argument_parser::ARGUMENTS;
use crate::enviroment::{Environment, ENVIRONMENT};
use crate::input::Input;
use crate::logger::log_info;
use crate::read_util::Mode;
use std::fs::File;
use std::io::stdin;
use std::path::Path;
use std::str::FromStr;
use std::sync::OnceLock;

mod logger;
mod argument_parser;
mod enviroment;
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

    let stdin = Input::Stdin(&stdin());

    read_util::read_input(Mode::from_str(&env.method).unwrap(),
                          if !*IS_STDIN.get().unwrap() && !env.bypass_stdin_check {
                                  stdin
                              } else {
                                  Input::File(File::open(Path::new(&env.file)).unwrap())
                              }
                          , &env.keyword);
}

static IS_STDIN: OnceLock<bool> = OnceLock::new();