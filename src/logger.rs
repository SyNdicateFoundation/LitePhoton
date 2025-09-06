use crate::environment::ENVIRONMENT;
use log::{error, info, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;

pub fn setup_logger(){
    log4rs::init_config(Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new("[LitePhoton] {l} {m}\n")))
            .build())))
        .build(Root::builder().appender("stdout").build(LevelFilter::Debug))
        .unwrap()).unwrap();
}

pub fn log_info(s: &str){
    if ENVIRONMENT.get().unwrap().debug{
        info!("{}", s);
    }
}
pub fn log_error(s: &str){
    if ENVIRONMENT.get().unwrap().debug{
        error!("{}", s);
    }
}