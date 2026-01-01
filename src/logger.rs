use log::{LevelFilter, error, info};
use log4rs::Config;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::sync::OnceLock;

pub static DEBUG: OnceLock<bool> = OnceLock::new();

pub fn setup_logger(debug: bool) {
    DEBUG
        .set(debug)
        .expect("logger.rs: cannot set DEBUG. already initialized?");

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
            .expect("logger.rs: Cannot initialize log4rs config"),
    )
    .expect("logger.rs: Cannot initialize log4rs config");
}

pub fn log_info(s: &str) {
    if *DEBUG.get().unwrap_or(&false) {
        info!("{}", s);
    }
}
pub fn log_error(s: &str) {
    if *DEBUG.get().unwrap_or(&false) {
        error!("{}", s);
    }
}
