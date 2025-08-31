use crate::argument_parser::ARGUMENTS;
use crate::enviroment::{Environment, ENVIRONMENT};
use crate::logger::{log_error, log_info};
use std::path::Path;
use std::sync::OnceLock;

mod logger;
mod argument_parser;
mod enviroment;
mod file_util;

/// Entry point
fn main() {
    IS_STDIN.set(atty::is(atty::Stream::Stdin)).unwrap();

    argument_parser::parse_arguments();

    let args = ARGUMENTS.get().unwrap();

    Environment::setup(args);

    let env = ENVIRONMENT.get().unwrap();

    logger::setup_logger();

    log_info(&format!("Starting up LitePhoton with this environment: {:?}", env));

    if !*IS_STDIN.get().unwrap() {
        log_info("Not running in stdin mode, probably | is used in the command to run the program.");
        file_util::tty(&args.keyword);
        if !args.bypass_stdin_check {
            return;
        }
    }

    // FIle mode, either echo the file content or scan it
    if !env.file.is_empty(){
        let file = Path::new(&env.file);

        // echo file
        if env.keyword.is_empty(){
            file_util::echo(file);
            return;
        }

        match env.method.as_str() {
            "chunk" => {
                file_util::chunk(file, &env.keyword);
            }
            "normal" => {
                file_util::normal(file, &env.keyword);
            }
            _ => {
                log_error(&format!("Method not found: {}", env.method));
            }
        }
        return;
    }
}

static IS_STDIN: OnceLock<bool> = OnceLock::new();