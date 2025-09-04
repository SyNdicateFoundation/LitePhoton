use crate::logger::log_error;
use std::fs::{File, Metadata};
use std::io;
use std::io::{Error, ErrorKind, Stdin};
use std::path::PathBuf;

/// Define input
/// This is to use BufReader with multiple definitions (cleaner code)
pub enum Input<'r> {
    Stdin(&'r Stdin),
    File(PathBuf),
}

impl Input<'_> {
    pub fn open_file(&self) -> io::Result<File> {
        match self{
            Input::Stdin(_) => {
                log_error("Cannot open_file stdin");
                Err(Error::new(ErrorKind::InvalidInput, "Cannot open_file stdin"))
            },
            Input::File(f) => {
                File::open(f.clone())
            }
        }
    }
    pub fn metadata(&self) -> io::Result<Metadata> {
        match self {
            Input::Stdin(_) => {
                log_error("Cannot metadata stdin");
                Err(Error::new(ErrorKind::InvalidInput, "Cannot metadata stdin"))
            }
            Input::File(f) => f.metadata(),
        }
    }
}

impl<'r> Clone for Input<'r> {
    fn clone(&self) -> Input<'r> {
        match self {
            Input::Stdin(s) => Input::Stdin(*s),
            Input::File(f) => Input::File(f.clone()),
        }
    }
}