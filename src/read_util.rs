use crate::input::Input;
use crate::logger::log_error;
use std::borrow::Cow;
use std::io::{stdout, BufRead, BufReader, BufWriter, Seek, SeekFrom, Write};
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

pub fn read_input(mut mode: Mode, input: Input<'_>, keyword: &str) {
    if mode == Mode::Chunk && matches!(input, Input::Stdin(_)) {
        log_error("Selected mode (chunk) is impossible with current input (stdin). falling back to normal mode.");
        mode = Mode::Normal;
    }

    let keyword: Cow<[u8]> = Cow::Owned(keyword.as_bytes().to_vec());

    match mode {
        Mode::Normal => {
            let mut writer = BufWriter::new(stdout());
            let mut reader = BufReader::with_capacity(8 * 1024, input.open_file().unwrap());
            let mut buff = Vec::with_capacity(8 * 1024);

            loop {
                buff.clear();

                match reader.read_until(b'\n', &mut buff) {
                    Ok(0) => {
                        writer.write_all(b"\n").expect("Can't write \\n");
                        writer.flush().expect("failed to flush writer");
                        break;
                    }
                    Ok(_) => {
                        if !keyword.is_empty() && !twoway::find_bytes(&buff, &keyword).is_some(){
                            continue;
                        }

                        writer.write_all(&buff).expect("Can't write results");
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
                    let mut buff = Vec::with_capacity(8 * 1024);
                    let mut pos = begin;

                    reader.seek(SeekFrom::Start(begin)).unwrap();

                    if begin > 0 {
                        buff.clear();
                        pos = pos.saturating_add(reader.read_until(b'\n', &mut buff).unwrap() as u64);
                    }

                    while pos < end {
                        buff.clear();
                        match reader.read_until(b'\n', &mut buff) {
                            Ok(0) => {
                                writer.write_all(b"\n").expect("Can't write \\n");
                                writer.flush().expect("failed to flush writer");
                                break;
                            },
                            Ok(bytes) => {
                                pos = pos.saturating_add(bytes as u64);

                                if !keyword.is_empty() && !twoway::find_bytes(&buff, keyword.as_ref()).is_some(){
                                    continue;
                                }

                                writer.write_all(&buff).expect("Can't write results");
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