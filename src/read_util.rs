use crate::input::Input;
use std::borrow::Cow;
use std::io::{stdin, stdout, BufReader, BufWriter, Read, Write};
use std::sync::Arc;
use std::{cmp};
use log::error;
use memmap2::Mmap;
use strum_macros::EnumString;

/// Modes of reading
/// Uses strum lib to convert Enums into Strings and parse them
#[derive(Debug, PartialEq, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Mode{
    Normal,
    Chunk,
}

pub fn read_input(mode: Mode, input: Input, _stable: bool, keyword: &str) {
    let mut writer = BufWriter::new(stdout());
    let keyword: Cow<[u8]> = Cow::Owned(keyword.as_bytes().to_vec());

    match input {
        // Use BufReader with stdin
        Input::Stdin(_) => {
            let mut reader = BufReader::with_capacity(8 * 1024, stdin());
            let mut read_buff = [0u8; 8 * 1024];
            let mut line_buff = Vec::with_capacity(8 * 1024);

            loop {
                match reader.read(&mut read_buff) {
                    Ok(0) => {
                        writer.write_all(b"\n").expect("Can't write \\n");
                        writer.flush().expect("failed to flush writer");
                        break;
                    }
                    Ok(size) => {
                        line_buff.extend_from_slice(&read_buff[..size]);

                        let mut begin = 0usize;
                        let mut i = 0usize;

                        while i < line_buff.len() {
                            if line_buff[i] == b'\n' {
                                let line = &line_buff[begin..=i];

                                if keyword.is_empty() || twoway::find_bytes(line, &keyword).is_some() {
                                    writer.write_all(line).expect("Can't write results");
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
                    Err(_) =>{
                        writer.flush().expect("failed to flush writer");
                    }
                }
            }
        }
        // Use MemMap2 with with the file
        Input::File(_) => {
            if let Err(_) = input.open_file() {
                error!("Failed to open the file. please, either check file permissions, or either specify a file with -f.");
                panic!("Failed to open the file. please, either check file permissions, or either specify a file with -f.");
            }

            match mode {
                Mode::Normal => {
                    let mmap = map_file(input).unwrap();
                    let mut begin = 0usize;
                    let mut i = 0usize;

                    while i < mmap.len(){
                        match memchr::memchr(b'\n', &mmap[i..]) {
                            Some(pos) => {
                                let end = i + pos;
                                let line = &mmap[begin..=end];
                                if keyword.is_empty() || twoway::find_bytes(line, &keyword).is_some() {
                                    writer.write_all(line).expect("Can't write \\n");
                                }
                                begin = end + 1;
                                i = begin;
                            }
                            None => {
                                writer.flush().expect("failed to flush writer");
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
                    let file_size = input.metadata().unwrap().len();
                    let mmap = map_file(input).unwrap();
                    let mmap = Arc::new(mmap);
                    let num_workers = num_cpus::get().max(1) as u64;
                    let chunk_size = if file_size == 0 { 0 } else { file_size / num_workers };
                    let keyword: Arc<Cow<[u8]>> = Arc::new(keyword);

                    rayon::scope(move |scope| {
                        for id in 0..num_workers {
                            let keyword = keyword.clone();
                            let mmap = mmap.clone();
                            let begin = id * chunk_size;
                            let end = cmp::max(
                                if id == num_workers - 1 { file_size } else { begin + chunk_size },
                                begin,
                            ) as usize;

                            scope.spawn(move |_| {
                                let mut writer = BufWriter::new(stdout());
                                let mut pos = begin as usize;
                                let mmap = &mmap[..];

                                while pos < end {
                                    match memchr::memchr(b'\n', &mmap[pos..end]) {
                                        Some(size) => {
                                            let end = pos + size + 1;
                                            let line = &mmap[pos..end];

                                            if keyword.is_empty() || twoway::find_bytes(line, &keyword).is_some() {
                                                writer.write_all(line).expect("Can't write results");
                                            }

                                            pos = end;
                                        }
                                        None => {
                                            let slice = &mmap[pos..end];

                                            if !slice.is_empty() && (keyword.is_empty() || twoway::find_bytes(slice, &keyword).is_some()) {
                                                writer.write_all(slice).expect("Can't write results");
                                            }

                                            writer.flush().expect("failed to flush \\n");
                                            break;
                                        }
                                    }
                                }

                                writer.write_all(b"\n").expect("Can't write newline");
                                writer.flush().expect("failed to flush writer");
                            });
                        }
                    });
                }
            }
        }
    }
}

fn map_file(input: Input) -> std::io::Result<Mmap> {
    match unsafe { Mmap::map(&input.open_file()?) } {
        Ok(mmap) => Ok(mmap),
        Err(err) => {
            error!("Failed to memory map the file. please, check the error below:");
            error!("{}", err);
            panic!("Failed to memory map the file. please, check the error below:");
        },
    }
}