extern crate log;
extern crate fern;
extern crate time;

extern crate dawg;

use dawg::dawg::{DawgBuilder};
#[allow(unused_imports)]
use log::LogLevelFilter::{Warn as LogWarn, Info as LogInfo, Trace as LogTrace};


pub fn make_dawg() -> DawgBuilder {
    DawgBuilder::new()
        .add_word("abra")
        .add_word("absol")    // remove this and see how storage of "kadabra" changes
        .add_word("crobat")
        .add_word("golbat")
        .add_word("kadabra")
        .add_word("mew")
        .add_word("mewtwo")
        .add_word("zubat")
}

pub fn init_log(lvl: log::LogLevelFilter) {
    let logger_config = fern::DispatchConfig {
        format: Box::new(
            |msg: &str, lvl: &log::LogLevel, _loc: &log::LogLocation| {
                format!("[{}] {:<8}{}", 
                        time::now().strftime("%Y-%m-%d %H:%M:%S").unwrap(), 
                        lvl, 
                        msg)
        }),
        output: vec![fern::OutputConfig::stdout()],
        level: LogTrace,
    };
    if let Err(e) = fern::init_global_logger(logger_config, lvl) {
        panic!("Failed to initialize global logger: {}", e);
    }
}
