use chrono::Local;
use owo_colors::OwoColorize;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
pub fn time_now() -> String {
    let local_time = Local::now();
    return local_time.format("%Y-%m-%d :: %I:%M:%S %p").to_string();
}

pub fn keywords() -> Vec<String> {
    let file = File::open("./keywords").expect("error at get_keywords");
    let reader = BufReader::new(file);

    reader.lines().map(|line| line.unwrap()).collect()
}
pub struct Config {
    pub announcer: bool,
    pub dm_warning: bool,
    pub do_logs: bool,
    pub do_mutes: bool,
    pub logging_channel: Option<u64>,
}

impl Config {
    pub fn construct() -> Config {
        let config_file = fs::read_to_string("./config.fcl").expect("Failed to read config file.");

        let announcer = config_gen(&config_file, "announcer = true", "announcer = false");
        let dm_warning = config_gen(&config_file, "dm_warning = true", "dm_warning = false");
        let do_logs = config_gen(&config_file, "do_logs = true", "do_logs = false");
        let do_mutes = config_gen(&config_file, "do_mutes = true", "do_mutes = false");

        // Read logging_channel value
        let logging_channel = extract_config_value(&config_file, "logging_channel")
            .and_then(|value| value.parse().ok());

        Config {
            announcer,
            dm_warning,
            do_logs,
            do_mutes,
            logging_channel,
        }
    }
}

pub fn extract_config_value(config_file: &str, key: &str) -> Option<String> {
    let key_with_assignment = format!("{} =", key);
    let start_index = config_file.find(&key_with_assignment)?;
    let rest = &config_file[start_index + key_with_assignment.len()..];
    let end_index = rest.find('\n').unwrap_or_else(|| rest.len());
    Some(rest[..end_index].trim().to_owned())
}

pub fn config_gen(config_file: &str, true_: &str, false_: &str) -> bool {
    if config_file.contains(true_) {
        true
    } else if config_file.contains(false_) {
        false
    } else {
        println!(
            "{} {}",
            "[ Fluorite Config Language]".bold().red(),
            "Panicked at Invalid configuration file. Has everything been included?"
                .bold()
                .yellow()
        );
        false
    }
}

pub fn check_for_guild() -> std::io::Result<()> {
    if let Err(_) = fs::metadata("./config.fcl") {
        println!(
            "{}",
            "[ core ]: You need to set the channel ID for logging!"
                .bold()
                .red()
        );
        std::process::exit(0);
    }
    let file_content = fs::read_to_string("./config.fcl")?;
    if file_content.contains("<channelID here>") {
        println!(
            "{}",
            "[ core ]: You need to set the channel ID for logging!"
                .bold()
                .red()
        );
        std::process::exit(0);
    } else {
        Ok(())
    }
}
