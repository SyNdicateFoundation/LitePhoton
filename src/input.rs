use crate::logger::log_error;
use std::fs::File;
use std::io;
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom, Stdin};

/// Define input
/// This is to use BufReader with multiple definitions (cleaner code)
pub enum Input<'r> {
    Stdin(&'r Stdin),
    File(File),
}

impl Read for Input<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Input::Stdin(s) => s.read(buf),
            Input::File(f) => f.read(buf),
        }
    }
}
impl Seek for Input<'_> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        match self {
            Input::Stdin(_) => {
                log_error("Cannot seek stdin");
                Err(Error::new(ErrorKind::InvalidInput, "Cannot seek stdin"))
            }
            Input::File(f) => f.seek(pos),
        }
    }
}

impl<'r> Clone for Input<'r> {
    fn clone(&self) -> Input<'r> {
        match self {
            Input::Stdin(s) => Input::Stdin(*s),
            Input::File(f) => Input::File(f.try_clone().expect("failed to clone")),
        }
    }
}