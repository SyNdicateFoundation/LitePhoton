use clap::Parser;
use std::sync::OnceLock;

#[derive(Parser, Debug)]
pub struct Arguments {
    #[arg(short, long, action = clap::ArgAction::Set, default_value = "false")]
    pub config: bool,
    #[arg(short, long, action = clap::ArgAction::Set, default_value = "true")]
    pub debug: bool,
    #[arg(short, long, action = clap::ArgAction::Set, default_value = "false")]
    pub bypass_stdin_check: bool,
    #[arg(short, long, default_value = "")]
    pub file: String,
    #[arg(short, long, default_value = "")]
    pub keyword: String,
    #[arg(short, long, default_value = "")]
    pub method: String,
    #[arg(value_parser)]
    pub last: Vec<String>,
}

pub fn parse_arguments(){
    ARGUMENTS.set(
        Arguments::parse()
    ).unwrap();
}

pub static ARGUMENTS: OnceLock<Arguments> = OnceLock::new();