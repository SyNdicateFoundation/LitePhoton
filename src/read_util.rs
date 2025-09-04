use crate::input::Input;
use crate::logger::log_error;
use std::borrow::Cow;
use std::io::{stdout, BufWriter, Write};
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

pub fn read_input(mut mode: Mode, input: Input, keyword: &str) {
    if mode == Mode::Chunk && matches!(input, Input::Stdin(_)) {
        log_error("Selected mode (chunk) is impossible with current input (stdin). falling back to normal mode.");
        mode = Mode::Normal;
    }

    let keyword: Cow<[u8]> = Cow::Owned(keyword.as_bytes().to_vec());

    match mode {
        Mode::Normal => {
            let mmap = unsafe { Mmap::map(&input.open_file().unwrap()) }.expect("Failed to map file");
            let mut writer = BufWriter::new(stdout());
            let mut begin = 0usize;

            for (i, &byte) in mmap.iter().enumerate() {
                if byte == b'\n' {
                    let line = &mmap[begin..=i];
                    if keyword.is_empty() || twoway::find_bytes(line, &keyword).is_some() {
                        writer.write_all(line).expect("Can't write results");
                    }
                    begin = i + 1;
                }
            }

            if begin < mmap.len() {
                let line = &mmap[begin..];
                if keyword.is_empty() || twoway::find_bytes(line, &keyword).is_some() {
                    writer.write_all(line).expect("Can't write results");
                    writer.write_all(b"\n").expect("Can't write newline");
                }
            }

            writer.flush().expect("failed to flush writer");
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
            let mmap = unsafe { Mmap::map(&input.open_file().unwrap()).expect("Failed to mmap file") };
            let mmap = Arc::new(mmap);
            let num_workers = cmp::max(1, cmp::min(num_cpus::get(), 64)) as u64;
            let chunk_size = if file_size == 0 {0} else {(file_size + num_workers - 1) / num_workers};
            let keyword: Arc<Cow<[u8]>> = Arc::new(keyword);
            let mut handlers = Vec::with_capacity(num_workers as usize);

            for id in 0..num_workers {
                let keyword = keyword.clone();
                let mmap = mmap.clone();
                let begin = id * chunk_size;

                if begin >= file_size {
                    continue;
                }

                let end = cmp::max(
                    if id == num_workers - 1 { file_size } else { begin + chunk_size },
                    begin,
                );

                handlers.push(rayon::spawn(move || {
                    let mut writer = BufWriter::new(stdout().lock());
                    let mut line_buff = Vec::with_capacity(8 * 1024);
                    let mut pos = begin as usize;

                    while pos < end as usize {
                        let read_buff = mmap[pos];

                        line_buff.push(read_buff);

                        if read_buff == b'\n' {
                            if keyword.is_empty() || twoway::find_bytes(&line_buff, &keyword).is_some() {
                                writer.write_all(&line_buff).expect("Can't write results");
                            }

                            line_buff.clear();
                        }

                        pos += 1;
                    }

                    if !line_buff.is_empty() && (keyword.is_empty() || twoway::find_bytes(&line_buff, &keyword).is_some()) {
                        writer.write_all(&line_buff).expect("Can't write results");
                    }

                    writer.flush().expect("failed to flush writer");
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