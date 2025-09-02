use crate::argument_parser::ARGUMENTS;
use crate::environment::{Environment, ENVIRONMENT};
use crate::input::Input;
use crate::logger::{log_info};
use crate::read_util::Mode;
use std::fs::File;
use std::io::stdin;
use std::path::Path;
use std::str::FromStr;
use std::sync::OnceLock;
use log::error;

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

    let stdin = Input::Stdin(&stdin());

    read_util::read_input(Mode::from_str(&env.method).unwrap(),
                          if !*IS_STDIN.get().unwrap() && !env.bypass_stdin_check {
                                  stdin
                              } else {
                                  let file = match File::open(Path::new(&env.file)) {
                                      Ok(file) => file,
                                      Err(_) => {
                                          error!("Failed to open the file. please, either check file permissions, or either specify a file with -f.");
                                          panic!("Failed to open the file. please, either check file permissions, or either specify a file with -f.");
                                      },
                                  };
                                  Input::File(file)
                              }
                          , &env.keyword);
}

static IS_STDIN: OnceLock<bool> = OnceLock::new();