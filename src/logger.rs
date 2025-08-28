use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;
pub fn setup_logger(){
    log4rs::init_config(Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new("[Scanner] {l} {m}\n")))
            .build())))
        .build(Root::builder().appender("stdout").build(LevelFilter::Debug))
        .unwrap()).unwrap();
}