//////////////////////////////////////////////////////////////////////////////
// cdict.rs
//
// Compile a dictionary (a list of strings, one per line) into a 
// minimal acyclic finite-state automaton.


extern crate getopts;
#[macro_use]
extern crate log;
extern crate fern;  // logging 
extern crate time;  // logging and profiling
extern crate dawg;


use getopts::Options;
use std::env;
use std::process;

#[allow(unused_imports)]
use log::LogLevelFilter::{Warn as LogWarn, 
                          Info as LogInfo, 
                          Trace as LogTrace,
                          Debug as LogDebug};

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use dawg::dawg::{DAWG,DawgBuilder};


struct AppConfig {
    dict_file: Option<String>,
}

impl AppConfig {
    fn new() -> AppConfig {
        AppConfig { 
            dict_file: None,
        }
    }
}

fn configure() -> AppConfig {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this message and exit");
    opts.optopt("f", "file", "read term list from file", "NAME");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&args[0], &opts);
    }

    let mut cfg: AppConfig = AppConfig::new();
    if matches.free.is_empty() {
        // Expect entry list on stdin
        // Leave as default (None)
    } else {
        cfg.dict_file = Some(matches.free[0].clone());
    }

    cfg
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options] [DICT_FILE]\nIf no DICT_FILE present, expects input on stdin", program);
    print!("{}", opts.usage(&brief));
    process::exit(1);
}



struct DictSource {
    dict_src: String,
}

impl DictSource {
    pub fn new(cfg: &AppConfig) -> DictSource {
        // Get the entries to compile
        let mut txt = String::new();
        match cfg.dict_file {
            None => {
                let stdin = io::stdin();
                stdin.lock().read_to_string(&mut txt).unwrap();
            },
            Some(ref fname) => {
                let fpath = Path::new(&fname);
                let mut f = File::open(fpath).unwrap();
                f.read_to_string(&mut txt).unwrap();
            }
        }  

        DictSource { dict_src: txt }      
    }

    pub fn get_dict_src(&self) -> &str {
        &self.dict_src
    }

    pub fn lines(&self) -> std::str::Lines {
        self.dict_src.lines()
    }
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
        output: vec![fern::OutputConfig::stderr()],
        level: LogTrace,
    };
    if let Err(e) = fern::init_global_logger(logger_config, lvl) {
        panic!("Failed to initialize global logger: {}", e);
    }
}


fn apply(dict_entries: &DictSource) {
    info!("Starting automaton construction");
    let mut builder = DawgBuilder::new();
    for entry in dict_entries.lines() {
        builder = builder.add_word(entry);
    }
    let dawg = builder.to_hash_builder().build();
    info!("Done building automaton");
    dawg.print();
}

fn main() {
    // command line parsing
    let cfg = configure();

    init_log(LogDebug);

    // figure out where we're getting our data from
    let dict_entries = DictSource::new(&cfg);

    // construct DAWG and dump it 
    apply(&dict_entries);

}