use log::{Record, Level, Metadata};
use std::{fs::OpenOptions, io::Write, path::Path};
use chrono::prelude::*;

pub struct AnnaLogger;

// TODO: check the size of the logfile, cleanup if needed?
impl log::Log for AnnaLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    // TODO: rewrite it so the file is held in memory instead of opened for each instance
    fn log(&self, record: &Record) {
        let logfile = "anna.log";
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
        let create = Path::new(logfile).exists();
        let mut file = OpenOptions::new()
            .create(!create)
            .write(true)
            .append(true)
            .open(logfile)
            .unwrap();
        writeln!(
            file,
            "{}:{} - {} in {}",
            Utc::now(),
            record.level(),
            record.args(),
            record.file().unwrap(),
        ).unwrap();
    }

    fn flush(&self) {}
}