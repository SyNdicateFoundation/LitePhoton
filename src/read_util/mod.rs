mod common;

use crate::input::Input;
use crate::read_util::common::{fail, map_file, write_all};
use log::error;
use std::cmp;
use std::io::{BufReader, BufWriter, ErrorKind, Read, stdin, stdout};
use std::sync::Arc;
use strum_macros::EnumString;

/// Modes of reading
/// Uses strum lib to convert Enums into Strings and parse them
#[derive(Debug, PartialEq, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Mode {
    Normal,
    Chunk,
}

pub fn read_input(mode: Mode, input: Input, _stable: bool, keyword: &str) {
    let mut writer = BufWriter::new(stdout());
    let keyword = keyword.as_bytes();

    match input {
        // Use BufReader with stdin
        Input::Stdin(_) => {
            let mut reader = BufReader::with_capacity(8 * 1024, stdin());
            let mut read_buff = [0u8; 8 * 1024];
            let mut line_buff = Vec::with_capacity(8 * 1024);

            loop {
                match reader.read(&mut read_buff) {
                    Ok(0) => {
                        fail(&mut writer, b"\n");
                        break;
                    }
                    Ok(size) => {
                        line_buff.extend_from_slice(&read_buff[..size]);

                        let mut begin = 0usize;
                        let mut i = 0usize;

                        while i < line_buff.len() {
                            if line_buff[i] == b'\n' {
                                let line = &line_buff[begin..=i];

                                if keyword.is_empty()
                                    || memchr::memmem::find(line, keyword).is_some()
                                {
                                    write_all(&mut writer, line);
                                }

                                begin = i + 1;
                            }
                            i += 1;
                        }

                        if begin == 0 {
                            continue;
                        }

                        if begin < line_buff.len() {
                            line_buff.drain(0..begin);
                        } else {
                            line_buff.clear();
                        }
                    }
                    Err(_) => {
                        fail(&mut writer, b"\n");
                        break;
                    }
                }
            }
        }
        // Use MemMap2 with with the file
        Input::File(_) => {
            if let Err(err) = input.metadata()
                && (err.kind() == ErrorKind::NotFound
                    || err.kind() == ErrorKind::PermissionDenied
                    || err.kind() == ErrorKind::IsADirectory)
            {
                error!(
                    "Failed to open the file. please, either check file permissions, or either specify a file with -f."
                );
                panic!(
                    "Failed to open the file. please, either check file permissions, or either specify a file with -f."
                );
            }

            match mode {
                Mode::Normal => {
                    let mmap =
                        map_file(input).expect("read_util/mod.rs: Cannot map file to memory");
                    let mut begin = 0usize;
                    let mut i = 0usize;

                    while i < mmap.len() {
                        match memchr::memchr(b'\n', &mmap[i..]) {
                            Some(0) => {
                                fail(&mut writer, b"");
                                break;
                            }
                            Some(pos) => {
                                let end = i + pos;
                                let line = &mmap[begin..=end];
                                if keyword.is_empty()
                                    || memchr::memmem::find(line, keyword).is_some()
                                {
                                    write_all(&mut writer, line);
                                }
                                begin = end + 1;
                                i = begin;
                            }
                            None => {
                                fail(&mut writer, b"");
                                break;
                            }
                        }
                    }
                }
                Mode::Chunk => {
                    let input = match input {
                        Input::File(file) => Input::File(file),
                        Input::Stdin(_) => {
                            error!("Could not use chunk mode while input is STDIN.");
                            panic!("Could not use chunk mode while input is STDIN.");
                        }
                    };
                    let file_size = input
                        .metadata()
                        .expect("read_util/mod.rs: Cannot get file metadata")
                        .len();
                    let mmap =
                        map_file(input).expect("read_util/mod.rs: Cannot map file to memory");
                    let mmap = Arc::new(mmap);
                    let num_workers = num_cpus::get().max(1) as u64;
                    let chunk_size = if file_size == 0 {
                        0
                    } else {
                        file_size / num_workers
                    };
                    let keyword: Arc<&[u8]> = Arc::new(keyword);

                    rayon::scope(move |scope| {
                        for id in 0..num_workers {
                            let keyword = keyword.clone();
                            let mmap = mmap.clone();
                            let begin = id * chunk_size;
                            let end = cmp::max(
                                if id == num_workers - 1 {
                                    file_size
                                } else {
                                    begin + chunk_size
                                },
                                begin,
                            ) as usize;

                            scope.spawn(move |_| {
                                let mut writer = BufWriter::new(stdout());
                                let mut pos = begin as usize;
                                let mmap = &mmap[..];

                                while pos < end {
                                    match memchr::memchr(b'\n', &mmap[pos..end]) {
                                        Some(0) => {
                                            fail(&mut writer, b"");
                                            break;
                                        }
                                        Some(size) => {
                                            let end = pos + size + 1;
                                            let line = &mmap[pos..end];

                                            if keyword.is_empty()
                                                || memchr::memmem::find(line, &keyword).is_some()
                                            {
                                                write_all(&mut writer, line);
                                            }

                                            pos = end;
                                        }
                                        None => {
                                            let slice = &mmap[pos..end];

                                            if !slice.is_empty()
                                                && (keyword.is_empty()
                                                    || memchr::memmem::find(slice, &keyword)
                                                        .is_some())
                                            {
                                                fail(&mut writer, slice);
                                            }
                                            break;
                                        }
                                    }
                                }

                                fail(&mut writer, b"\n");
                            });
                        }
                    });
                }
            }
        }
    }
}
