/// test_builder.rs

#[macro_use]
extern crate log;
extern crate fern;
extern crate time;

extern crate dawg;

use dawg::dawg::{DawgBuilder};
#[allow(unused_imports)]
use log::LogLevelFilter::{Warn as LogWarn, Info as LogInfo, Trace as LogTrace};

mod common;

#[test]
fn dawg_builder_works() {
    common::init_log(LogTrace);
    let dawg = 
        common::make_dawg()
        .build();
    dawg.print();

    assert_eq!(dawg.hash_term("abra"), 0);
}
