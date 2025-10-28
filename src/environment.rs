use crate::argument_parser::Arguments;
use std::sync::OnceLock;

#[derive(Debug)]
pub struct Environment {
    pub debug: bool,
    pub bypass_stdin_check: bool,
    pub stable: bool,
    pub file: Vec<String>,
    pub keyword: String,
    pub method: String,
}

impl Environment {
    fn get(args: &Arguments) -> Environment {
        Environment {
            debug: args.debug.into(),
            bypass_stdin_check: args.bypass_stdin_check.into(),
            stable: args.stable.into(),
            file:args.file.clone(),
            keyword:args.keyword.clone(),
            method:args.method.clone(),
        }
    }
    pub fn setup(args: &Arguments){
        ENVIRONMENT.set(Self::get(args)).unwrap();
    }
}

pub static ENVIRONMENT: OnceLock<Environment> = OnceLock::new();