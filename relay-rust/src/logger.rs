/*
 * Copyright (C) 2017 Genymobile
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use chrono::prelude::Local;
use log::*;
use std::io::{self, Write};
use flexi_logger::{Age, Cleanup, Criterion, DeferredNow, FileSpec, Logger, Naming};

static LOGGER: SimpleLogger = SimpleLogger;
const THRESHOLD: LevelFilter = LevelFilter::Info;

pub struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= THRESHOLD
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let date = Local::now();
            let formatted_date = date.format("%Y-%m-%d %H:%M:%S%.3f");
            let msg = format!(
                "{} {} {}: {}",
                formatted_date,
                record.level(),
                record.target(),
                record.args()
            );
            if record.level() == Level::Error {
                eprintln!("{}", msg);
            } else {
                println!("{}", msg);
            }
        }
    }

    fn flush(&self) {
        io::stdout().flush().unwrap();
        io::stderr().flush().unwrap();
    }
}

pub fn init(log_path: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    set_max_level(THRESHOLD);
    if cfg!(debug_assertions) {
        set_logger(&LOGGER)?;
        Ok(())
    }
    else {
        Logger::try_with_str("info")?
            .log_to_file(
                FileSpec::default()
                    .directory(log_path.unwrap_or_else(|| "./logs".into()))
                    .basename("gnirehtet-relay")
                    .suffix("log"),
            )
            .rotate(
                Criterion::Age(Age::Day),
                Naming::Timestamps,
                Cleanup::KeepLogFiles(7),
            )
            .format_for_files(my_format)
            .start()?;
        Ok(())
    }
}


fn my_format(
    w: &mut dyn Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    write!(
        w,
        "[{}] {} [{}] {}",
        Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
        record.level(),
        record.target(),
        record.args()
    )
}
