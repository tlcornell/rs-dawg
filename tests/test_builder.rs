/// test_builder.rs

#[macro_use]
extern crate log;
extern crate fern;
extern crate time;

extern crate dawg;

use dawg::dawg::DawgBuilder;
use log::LogLevelFilter::{Info as LogInfo, Trace as LogTrace};

#[test]
fn builder_works() {
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
    if let Err(e) = fern::init_global_logger(logger_config, LogTrace) {
        panic!("Failed to initialize global logger: {}", e);
    }

    info!("Running builder_works");
    let builder = DawgBuilder::new();
    let dawg = builder
        .add_word("abra")
        .add_word("absol")    // remove this and see how storage of "kadabra" changes
        .add_word("crobat")
        .add_word("golbat")
        .add_word("kadabra")
        .add_word("mew")
        .add_word("mewtwo")
        .add_word("zubat")
        .build();
    dawg.print();
}
