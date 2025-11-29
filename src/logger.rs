use log::{LevelFilter, error, info};
use log4rs::Config;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::sync::OnceLock;

pub static DEBUG: OnceLock<bool> = OnceLock::new();

pub fn setup_logger(debug: bool) {
    DEBUG.set(debug).unwrap();

    log4rs::init_config(
        Config::builder()
            .appender(
                Appender::builder().build(
                    "stdout",
                    Box::new(
                        ConsoleAppender::builder()
                            .encoder(Box::new(PatternEncoder::new("[LitePhoton] {l} {m}\n")))
                            .build(),
                    ),
                ),
            )
            .build(Root::builder().appender("stdout").build(LevelFilter::Debug))
            .unwrap(),
    )
    .unwrap();
}

pub fn log_info(s: &str) {
    if *DEBUG.get().unwrap() {
        info!("{}", s);
    }
}
pub fn log_error(s: &str) {
    if *DEBUG.get().unwrap() {
        error!("{}", s);
    }
}
