/// test_hashing.rs

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
fn hash_builder_works() {
    common::init_log(LogTrace);
    info!("Running builder_works");
    let dawg = 
        common::make_dawg()
        .to_hash_builder()
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
