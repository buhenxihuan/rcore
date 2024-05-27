//! use log crate to offer the log function

use log::{self, Level, LevelFilter, Log, Metadata, Record};

struct SimpleLogger;

impl Log for SimpleLogger{
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let color = match record.level() {
            Level::Error => 31, //red
            Level::Warn => 93, //bright_yellow
            Level::Info => 34, //blue
            Level::Debug => 32, //green
            Level::Trace => 90, //bright_black
        };

        println!(
            "\u{1b}[{}m[{:>5}] {}\u{1b}[0m",
            color,
            record.level(),
            record.args(),
        );
    }
    
    fn flush(&self) {}

}

pub fn init() {
    static LOGGER: SimpleLogger = SimpleLogger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG"){
        Some(error) if error.eq_ignore_ascii_case("error") => LevelFilter::Error,
        Some(warn) if warn.eq_ignore_ascii_case("warn") => LevelFilter::Warn,
        Some(info) if info.eq_ignore_ascii_case("info") => LevelFilter::Info,
        Some(debug) if debug.eq_ignore_ascii_case("debug") => LevelFilter::Debug,
        Some(trace) if trace.eq_ignore_ascii_case("trace") => LevelFilter::Trace,
        _ => LevelFilter::Off,
    });
}