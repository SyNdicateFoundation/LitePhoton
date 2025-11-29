use crate::logger::log_error;
use std::fs::{File, Metadata};
use std::io;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

pub enum Input {
    Stdin(()),
    File(PathBuf),
}

impl Input {
    pub fn open_file(&self) -> io::Result<File> {
        match self {
            Input::Stdin(_) => {
                log_error("Cannot open_file stdin");
                Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Cannot open_file stdin",
                ))
            }
            Input::File(f) => File::open(f),
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

impl Clone for Input {
    fn clone(&self) -> Input {
        match self {
            Input::Stdin(_) => Input::Stdin(()),
            Input::File(f) => Input::File(f.clone()),
        }
    }
}
