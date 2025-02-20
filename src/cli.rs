use clap::{Arg, ArgAction, ArgMatches, ColorChoice, Command};
use rinex::prelude::*;

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
                            .help("Input Observation RINEX")
                            .value_name("filepath")
                            .required(true)
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
                            .conflicts_with_all(["short"])
                            .help("Define custom output name. Overrides any file name determination logic.")
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
    pub fn input_path(&self) -> &str {
        self.matches.get_one::<String>("filepath").unwrap()
    }
    pub fn output_path(&self) -> Option<&String> {
        self.matches.get_one::<String>("output")
    }
    pub fn custom_date(&self) -> Option<Epoch> {
        if let Some(s) = self.matches.get_one::<String>("date") {
            let items: Vec<&str> = s.split('-').collect();
            if items.len() != 3 {
                println!("failed to parse \"yyyy-mm-dd\"");
                return None;
            } else if let Ok(y) = i32::from_str_radix(items[0], 10) {
                if let Ok(m) = u8::from_str_radix(items[1], 10) {
                    if let Ok(d) = u8::from_str_radix(items[2], 10) {
                        return Some(Epoch::from_gregorian_utc_at_midnight(y, m, d));
                    }
                }
            }
        }
        None
    }
    pub fn custom_time(&self) -> Option<(u8, u8, u8)> {
        if let Some(s) = self.matches.get_one::<String>("time") {
            let items: Vec<&str> = s.split(':').collect();
            if items.len() != 3 {
                println!("failed to parse \"hh:mm:ss\"");
                return None;
            } else if let Ok(h) = u8::from_str_radix(items[0], 10) {
                if let Ok(m) = u8::from_str_radix(items[1], 10) {
                    if let Ok(s) = u8::from_str_radix(items[2], 10) {
                        return Some((h, m, s));
                    }
                }
            }
        }
        None
    }
}
