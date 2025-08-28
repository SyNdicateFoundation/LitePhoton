use std::path::PathBuf;
use std::sync::OnceLock;
use clap::Parser;

/// Define our arguments and use clap lib
#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short)]
    pub file: PathBuf,
    #[arg(short)]
    pub keyword: String,
    #[arg(short, default_value = "chunk")]
    pub method: String,
}

pub static ARGUMENTS: OnceLock<Args> = OnceLock::new();
