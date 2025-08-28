use std::cmp;
use std::fs::File;
use std::io::{stdout, BufRead, BufReader, Seek, SeekFrom, Write};
use std::path::{Path};
use clap::Parser;
use log::{error, info};
use std::sync::Arc;
use crate::argument_parser::{Args, ARGUMENTS};

mod logger;
mod argument_parser;

/// Entry point
fn main() {
    let args = ARGUMENTS.get_or_init(|| Args::parse());
    logger::setup_logger();

    info!("Starting up scanner with this arguments: {:?}", args);
    info!("Started scanning with {} method", args.method);
    println!();

    match args.method.as_str() {
        "chunk" => {
            chunk(&args.file, &args.keyword);
        }
        "normal" => {
            normal(&args.file, &args.keyword);
        }
        _ => {
            error!("Method not found: {}", args.method);
        }
    }
}

/// Normal method, uses twoway, is not multithreaded.
fn normal(file: &Path, keyword: &str) {
    let file = File::open(file).unwrap();
    let mut reader = BufReader::with_capacity(8 * 1024 * 1024, file);
    let keyword = keyword.as_bytes().to_vec().into_boxed_slice();
    let mut buff = Vec::with_capacity(8 * 1024);

    loop {
        match reader.read_until(b'\n', &mut buff) {
            Ok(0) => break,
            Ok(_) => {
                let buff = if buff.ends_with(&[b'\n']) {&buff[..buff.len() - 1]}
                else {&buff[..]};
                if keyword.len() <= buff.len() && twoway::find_bytes(buff, &keyword).is_some() {
                    let mut out_line = Vec::with_capacity(buff.len() + 1);
                    out_line.extend_from_slice(buff);
                    out_line.push(b'\n');

                    stdout().write_all(&out_line).expect("Can't write results");
                    stdout().flush().expect("Can't flush the console");
                }
            }
            Err(_) => {}
        }
        buff.clear();
    }
}

/// Chunk method, uses twoway + rayon, is multithreaded.
fn chunk(file: &Path, keyword: &str) {
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
            let mut pos = begin;
            let mut buff = Vec::with_capacity(8 * 1024);

            reader.seek(SeekFrom::Start(begin)).unwrap();

            if begin > 0 {
                buff.clear();
                pos = pos.saturating_add(reader.read_until(b'\n', &mut buff).unwrap() as u64);
            }

            while pos < end {
                buff.clear();
                let bytes_read = reader.read_until(b'\n', &mut buff).unwrap();
                if bytes_read == 0 {
                    break;
                }
                pos = pos.saturating_add(bytes_read as u64);

                let buff = if buff.ends_with(&[b'\n']) {&buff[..buff.len() - 1]}
                else {&buff[..]};

                if keyword.len() <= buff.len() && twoway::find_bytes(buff, &**keyword).is_some() {
                    let mut out_line = Vec::with_capacity(buff.len() + 1);
                    out_line.extend_from_slice(buff);
                    out_line.push(b'\n');

                    stdout().write_all(&out_line).expect("Can't write results");
                    stdout().flush().expect("Can't flush the console");
                }
            }
        }));
    }

    rayon ::scope(|_| {
        for handler in handlers{
            let _ = handler;
        }
    });
}