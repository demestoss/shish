use anyhow::{anyhow, Context};
use chrono::NaiveDateTime;
use clap::Parser;
use std::{
    fs::File,
    time::{self, SystemTime},
};

#[derive(Parser, Debug)]
#[command(about, author, version)]
/// Make file(s) in the current directory. If file exists change the timestamp
pub struct Command {
    /// Skip file creation if it does not exist
    #[arg(short = 'c')]
    skip_create: bool,

    /// Set custom timestamp in the format [[CC]YY]MMDDhhmm[.ss]
    #[arg(short = 't')]
    timestamp: Option<String>,

    /// The name(s) of the file(s) to create
    files: Vec<String>,
}

impl Command {
    pub fn invoke(&self) -> anyhow::Result<()> {
        for file in self.files.iter() {
            if let Err(e) = self.file_proceed(file) {
                eprintln!("touch: Failed {file}: {e}")
            }
        }
        Ok(())
    }

    fn file_proceed(&self, file: &str) -> anyhow::Result<()> {
        let stats = std::fs::metadata(file);

        let file = match stats {
            Err(_) => {
                if self.skip_create {
                    return Ok(());
                }
                std::fs::File::create(file)?
            }
            Ok(_) => std::fs::File::open(file)?,
        };

        self.set_modified(&file)
    }

    fn set_modified(&self, file: &File) -> anyhow::Result<()> {
        let time = match &self.timestamp {
            None => time::SystemTime::now(),
            Some(timestamp) => {
                let datetime = parse_custom_timestamp(timestamp)?;
                SystemTime::UNIX_EPOCH
                    .checked_add(std::time::Duration::from_secs(
                        datetime.and_utc().timestamp() as u64,
                    ))
                    .context("Failed to convert into system time")?
            }
        };

        file.set_modified(time)
            .context("Failed to update file time")
    }
}

/// Parse a custom timestamp in the format [[CC]YY]MMDDhhmm[.ss]
fn parse_custom_timestamp(timestamp: &str) -> anyhow::Result<NaiveDateTime> {
    let format = "%Y%m%d%H%M%S";
    let full_timestamp = if timestamp.len() == 12 {
        format!("{}00", timestamp) // Add seconds if not provided
    } else {
        timestamp.to_string()
    };

    NaiveDateTime::parse_from_str(&full_timestamp, format)
        .map_err(|_| anyhow!("Invalid timestamp format. Use [[CC]YY]MMDDhhmm[.ss]"))
}
