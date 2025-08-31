use clap::Parser;
use std::sync::OnceLock;

#[derive(Parser, Debug)]
pub struct Arguments {
    #[arg(short, long, default_value = "false")]
    pub config: bool,
    #[arg(short, long, default_value = "false")]
    pub debug: bool,
    #[arg(short, long, default_value = "false")]
    pub bypass_stdin_check: bool,
    #[arg(short, long, default_value = "")]
    pub file: String,
    #[arg(short, long, default_value = "")]
    pub keyword: String,
    #[arg(short, long, default_value = "chunk")]
    pub method: String,
    // unnecessary because tty is different from stdin
    // #[arg(value_parser)]
    // pub last: Vec<String>,
}

impl Arguments {
    pub fn to_lowercase(mut self) -> Self {
        self.file = self.file.to_lowercase();
        self.keyword = self.keyword.to_lowercase();
        self.method = self.method.to_lowercase();
        self
    }
}

pub fn parse_arguments(){
    ARGUMENTS.set(
        Arguments::parse().to_lowercase()
    ).unwrap();
}

pub static ARGUMENTS: OnceLock<Arguments> = OnceLock::new();