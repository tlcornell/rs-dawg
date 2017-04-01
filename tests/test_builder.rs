/// test_builder.rs

#[macro_use]
extern crate log;
extern crate fern;
extern crate time;

extern crate dawg;

use dawg::dawg::DawgBuilder;
use log::LogLevelFilter::{Warn as LogWarn, Info as LogInfo, Trace as LogTrace};

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
    if let Err(e) = fern::init_global_logger(logger_config, LogWarn) {
        panic!("Failed to initialize global logger: {}", e);
    }

    info!("Running builder_works");
    let dawg = DawgBuilder::new()
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
    
    assert_eq!(dawg.hash_term("abra"), 1);
    assert_eq!(dawg.hash_term("absol"), 2);
    assert_eq!(dawg.hash_term("crobat"), 3);
    assert_eq!(dawg.hash_term("golbat"), 4);
    assert_eq!(dawg.hash_term("kadabra"), 5);
    assert_eq!(dawg.hash_term("mew"), 6);
    assert_eq!(dawg.hash_term("mewtwo"), 7);
    assert_eq!(dawg.hash_term("zubat"), 8);

}
