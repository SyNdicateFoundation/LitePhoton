use crate::argument_parser::Arguments;
use std::str::FromStr;
use std::sync::OnceLock;

#[derive(Debug)]
pub struct Environment {
    pub debug: bool,
    pub bypass_stdin_check: bool,
    pub unstable: bool,
    pub file: String,
    pub keyword: String,
    pub method: String,
}

impl Environment {
    fn get_val<T>(args: &Arguments, arg: T, config: T) -> T where T: FromStr, T: ToString{
        if args.config {config} else {arg}
    }
    fn get(args: &Arguments) -> Environment {
        Environment {
            debug: Self::get_val(args, args.debug.into(), true),
            bypass_stdin_check: Self::get_val(args, args.bypass_stdin_check.into(), true),
            unstable: Self::get_val(args, args.unstable.into(), true),
            file: Self::get_val(args, args.file.clone(), String::new().into()),
            keyword: Self::get_val(args, args.keyword.clone(), String::new().into()),
            method: Self::get_val(args, args.method.clone(), String::new().into()),
        }
    }
    pub fn setup(args: &Arguments){
        ENVIRONMENT.set(Self::get(args)).unwrap();
    }
}

pub static ENVIRONMENT: OnceLock<Environment> = OnceLock::new();