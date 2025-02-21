use clap::{Arg, ArgAction, ArgMatches, ColorChoice, Command};

use std::path::{Path, PathBuf};

pub struct Cli {
    /// arguments passed by user
    pub matches: ArgMatches,
}

impl Cli {
    pub fn new() -> Self {
        Self {
            matches: {
                Command::new("rnx2crx")
                    .author("Guillaume W. Bres <guillaume.bressaix@gmail.com>")
                    .version(env!("CARGO_PKG_VERSION"))
                    .about("RINEX compression tool")
                    .arg_required_else_help(true)
                    .color(ColorChoice::Always)
                    .arg(
                        Arg::new("filepath")
                            .help("Observation RINEX")
                            .value_name("filepath")
                            .required(true)
                    )
                    .arg(
                        Arg::new("quiet")
                            .short('q')
                            .action(ArgAction::SetTrue)
                            .help("Make the tool quiet")
                    )
                    .arg(
                        Arg::new("short")
                            .short('s')
                            .action(ArgAction::SetTrue)
                            .help("Prefer V1 short filename convention")
                    )
                    .arg(
                        Arg::new("output")
                            .short('o')
                            .action(ArgAction::Set)
                            .value_name("filename")
                            .conflicts_with_all(["short"])
                            .help("Define custom output name. Overrides any file name determination logic.")
                    )
                    .arg(
                        Arg::new("prefix")
                            .long("prefix")
                            .action(ArgAction::Set)
                            .help("Define custom output location (directory), that must exist")
                    )
                    .arg(
                        Arg::new("date")
                            .short('d')
                            .help("Set compression date. Example: -d 2024-01-01"),
                    )
                    .arg(
                        Arg::new("time")
                            .short('t')
                            .long("time")
                            .help("Set compression time. Example: -t 00:00:00")
                    )
                    .get_matches()
            },
        }
    }

    pub fn quiet(&self) -> bool {
        self.matches.get_flag("quiet")
    }

    pub fn forced_short_v1(&self) -> bool {
        self.matches.get_flag("short")
    }

    pub fn input_path(&self) -> PathBuf {
        Path::new(self.matches.get_one::<String>("filepath").unwrap()).to_path_buf()
    }

    pub fn custom_name(&self) -> Option<&String> {
        self.matches.get_one::<String>("output")
    }

    pub fn custom_prefix(&self) -> Option<&String> {
        self.matches.get_one::<String>("prefix")
    }

    /// Returns custom date defined by User
    pub fn custom_date(&self) -> Option<(i32, u8, u8)> {
        let date = self.matches.get_one::<String>("date")?;
        let items: Vec<&str> = date.split('/').collect();

        if items.len() != 3 {
            panic!("Invalid date description: expecting \"YYYY/MM/DD\"");
        }

        let yyyy = i32::from_str_radix(items[0], 10)
            .unwrap_or_else(|e| panic!("Year parsing error: {}", e));

        let mm = u8::from_str_radix(items[1], 10)
            .unwrap_or_else(|e| panic!("Month parsing error: {}", e));

        let dd =
            u8::from_str_radix(items[2], 10).unwrap_or_else(|e| panic!("Day parsing error: {}", e));

        Some((yyyy, mm, dd))
    }

    /// Returns custom datetime defined by User
    pub fn custom_time(&self) -> Option<(u8, u8, u8)> {
        let time = self.matches.get_one::<String>("time")?;

        let items: Vec<&str> = time.split(':').collect();
        if items.len() != 3 {
            panic!("Invalid time description: expecting \"HH:MM:SS\"");
        }

        let hh = u8::from_str_radix(items[0], 10)
            .unwrap_or_else(|e| panic!("Hours parsing error: {}", e));

        let mm = u8::from_str_radix(items[1], 10)
            .unwrap_or_else(|e| panic!("Minutes parsing error: {}", e));

        let ss = u8::from_str_radix(items[2], 10)
            .unwrap_or_else(|e| panic!("Seconds parsing error: {}", e));

        Some((hh, mm, ss))
    }
}
