/// test_builder.rs

#[macro_use]
extern crate log;
extern crate fern;
extern crate time;

extern crate dawg;

use dawg::dawg::DawgBuilder;

#[test]
fn builder_works() {
    let logger_config = fern::DispatchConfig {
        format: Box::new(
            |msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
                // This is a fairly simple format, though it's possible to do more complicated ones.
                // This closure can contain any code, as long as it produces a String message.
                format!("[{}] {:<8}{}", 
                        time::now().strftime("%Y-%m-%d %H:%M:%S").unwrap(), 
                        level, 
                        msg)
        }),
        output: vec![fern::OutputConfig::stdout()],
        level: log::LogLevelFilter::Trace,
    };
    if let Err(e) = fern::init_global_logger(logger_config, log::LogLevelFilter::Trace) {
        panic!("Failed to initialize global logger: {}", e);
    }

    info!("Running builder_works");
    let builder = DawgBuilder::new();
    let dawg = builder
        .add_word("abra")
        .add_word("absol")
        .add_word("crobat")
        .add_word("zubat")
        .build();
    dawg.print();
}
