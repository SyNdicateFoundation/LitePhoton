use std::io::{BufWriter, Write};
use log::error;
use memmap2::Mmap;
use crate::input::Input;
use crate::logger::log_error;

pub fn map_file(input: Input) -> std::io::Result<Mmap> {
    match unsafe { Mmap::map(&input.open_file()?) } {
        Ok(mmap) => Ok(mmap),
        Err(err) => {
            error!("Failed to memory map the file. please, check the error below:");
            error!("{}", err);
            panic!("Failed to memory map the file. please, check the error below:");
        },
    }
}

pub fn write_all<W>(writer: &mut BufWriter<W>, line: &[u8]) where W: Sized + Write {
    match writer.write_all(line) {
        Err(_) => log_error("Cannot write_all console, platform restriction?"),
        _ => {}
    }
}
pub fn flush<W>(writer: &mut BufWriter<W>) where W: Sized + Write {
    match writer.flush() {
        Err(_) => log_error("Cannot flush console, platform restriction?"),
        _ => {}
    }
}

pub fn fail<W>(writer: &mut BufWriter<W>, line: &[u8]) where W: Sized + Write {
    write_all(writer, line);
    flush(writer);
}


