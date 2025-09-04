use crate::input::Input;
use crate::logger::log_error;
use std::borrow::Cow;
use std::io::{stdout, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::sync::Arc;
use std::{cmp};
use log::error;
use strum_macros::EnumString;
/*
TODO: Use MemMap2 instead of bufreader
TODO: do memory calculations to avoid messing up system resources
 */

/// Modes of reading
/// Uses strum lib to convert Enums into Strings and parse them
#[derive(Debug, PartialEq, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Mode{
    Normal,
    Chunk,
}

pub fn read_input(mut mode: Mode, input: Input, keyword: &str) {
    if mode == Mode::Chunk && matches!(input, Input::Stdin(_)) {
        log_error("Selected mode (chunk) is impossible with current input (stdin). falling back to normal mode.");
        mode = Mode::Normal;
    }

    let keyword: Cow<[u8]> = Cow::Owned(keyword.as_bytes().to_vec());

    match mode {
        Mode::Normal => {
            let mut writer = BufWriter::new(stdout());
            let mut reader = BufReader::with_capacity(8 * 1024, input.open_file().unwrap());
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
        Mode::Chunk => {
            let input = match input {
                Input::File(file) => Input::File(file),
                Input::Stdin(_) => {
                    error!("Could not use chunk mode while input is STDIN.");
                    panic!("Could not use chunk mode while input is STDIN.");
                },
            };
            let file_size = input.metadata().unwrap().len();
            let num_workers = cmp::max(1, cmp::min(num_cpus::get(), 64)) as u64;
            let chunk_size = if file_size == 0 {0} else {(file_size + num_workers - 1) / num_workers};
            let keyword: Arc<Cow<[u8]>> = Arc::new(keyword);
            let mut handlers = Vec::with_capacity(num_workers as usize);

            for id in 0..num_workers {
                let keyword = keyword.clone();
                let input = input.clone();
                let begin = id * chunk_size;

                if begin >= file_size {
                    continue;
                }

                let end = cmp::max(
                    if id == num_workers - 1 { file_size } else { begin + chunk_size },
                    begin,
                );
                handlers.push(rayon::spawn(move || {
                    let mut reader = BufReader::with_capacity(8 * 1024 * 1024, input.open_file().unwrap());
                    let mut writer = BufWriter::new(stdout().lock());
                    let mut read_buff = [0u8; 8 * 1024];
                    let mut line_buff = Vec::with_capacity(8 * 1024);
                    let mut pos = begin;

                    reader.seek(SeekFrom::Start(begin)).unwrap();

                    if begin > 0 {
                        pos = pos.saturating_add(reader.read(&mut read_buff).unwrap() as u64);
                    }

                    while pos < end {
                        match reader.read(&mut read_buff) {
                            Ok(0) => {
                                writer.write_all(b"\n").expect("Can't write \\n");
                                writer.flush().expect("failed to flush writer");
                                break;
                            },
                            Ok(size) => {
                                pos = pos.saturating_add(size as u64);

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
                            Err(_) => {
                                writer.flush().expect("failed to flush writer");
                            },
                        };
                    }
                }));
            }

            rayon ::scope(|_| {
                for handler in handlers{
                    let _ = handler;
                }
            });
        }
    }
}