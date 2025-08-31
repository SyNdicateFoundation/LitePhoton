use std::{cmp, io};
use std::fs::File;
use std::io::{stdout, BufRead, BufReader, BufWriter, Seek, SeekFrom, Write};
use std::path::Path;
use std::sync::Arc;

/// Scans tty, uses twoway, is not multithreaded.
/// TODO: replace BufReader with MemMap2
pub fn tty(keyword: &str) {
    let mut reader = BufReader::with_capacity(8 * 1024 * 1024, io::stdin());
    let mut writer = BufWriter::new(stdout().lock());
    let mut buff = Vec::with_capacity(8 * 1024);
    let keyword = keyword.as_bytes().to_vec().into_boxed_slice();

    loop {
        buff.clear();
        match reader.read_until(b'\n', &mut buff) {
            Ok(0) => {
                writer.flush().expect("failed to flush writer");
                break;
            }
            Ok(_) => {
                if twoway::find_bytes(&buff, &keyword).is_some() {
                    writer.write_all(&buff).expect("Can't write results");
                }
            }
            Err(_) => {
                writer.flush().expect("failed to flush writer");
            }
        }
    }
}

/// Echo the file into the console
/// TODO: replace BufReader with MemMap2
pub fn echo(file: &Path) {
    let file = File::open(file).unwrap();
    let mut reader = BufReader::with_capacity(8 * 1024 * 1024, file);
    let mut writer = BufWriter::new(stdout().lock());
    let mut buff = Vec::with_capacity(8 * 1024);

    loop{
        buff.clear();
        match reader.read_until(b'\n', &mut buff) {
            Ok(0) => {
                writer.flush().expect("failed to flush writer");
                break;
            },
            Ok(_) => {
                writer.write_all(&buff).expect("Can't write results");
            }
            Err(_) => {
                writer.flush().expect("failed to flush writer");
            },
        }
    }
}

/// Normal method, uses twoway, is not multithreaded.
/// TODO: replace BufReader with MemMap2
pub fn normal(file: &Path, keyword: &str) {
    let file = File::open(file).unwrap();
    let mut reader = BufReader::with_capacity(8 * 1024 * 1024, file);
    let mut writer = BufWriter::new(stdout().lock());
    let mut buff = Vec::with_capacity(8 * 1024);
    let keyword = keyword.as_bytes().to_vec().into_boxed_slice();

    loop {
        buff.clear();
        match reader.read_until(b'\n', &mut buff) {
            Ok(0) => {
                writer.flush().expect("failed to flush writer");
                break;
            },
            Ok(_) => {
                if keyword.len() <= buff.len() && twoway::find_bytes(&buff, &keyword).is_some() {
                    writer.write_all(&buff).expect("Can't write results");
                }
            }
            Err(_) => {
                writer.flush().expect("failed to flush writer");
            },
        }
    }
}

/// Chunk method, uses twoway + rayon, is multithreaded.
/// TODO: replace BufReader with MemMap2
pub fn chunk(file: &Path, keyword: &str) {
    let file_size = file.metadata().unwrap().len();
    let num_workers = num_cpus::get().max(1) as u64;
    let chunk_size = if file_size == 0 { 0 } else { file_size / num_workers };
    let keyword = Arc::new(keyword.as_bytes().to_vec().into_boxed_slice());
    let mut handlers = Vec::with_capacity(num_workers as usize);

    for id in 0..num_workers {
        let file = file.to_path_buf();
        let keyword = keyword.clone();
        let begin = id * chunk_size;
        let end = cmp::max(
            if id == num_workers - 1 { file_size } else { begin + chunk_size },
            begin,
        );
        handlers.push(rayon::spawn(move || {
            let file = File::open(file).unwrap();
            let mut reader = BufReader::with_capacity(8 * 1024 * 1024, file);
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
                    Ok(0) => break,
                    Ok(bytes) => {
                        pos = pos.saturating_add(bytes as u64);

                        if keyword.len() <= buff.len() && twoway::find_bytes(&buff, &**keyword).is_some() {
                            writer.write_all(&buff).expect("Can't write results");
                        }
                    }
                    Err(_) => {},
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