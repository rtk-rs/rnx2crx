mod cli;
use cli::Cli;

use rinex::prelude::{Epoch, Rinex};

fn compression_date_time(cli: &Cli) -> Epoch {
    if let Some((y, m, d)) = cli.custom_date() {
        if let Some((hh, mm, ss)) = cli.custom_time() {
            Epoch::from_gregorian_utc(y, m, d, hh, mm, ss, 0)
        } else {
            Epoch::now().unwrap_or_else(|e| panic!("failed to determine system time: {}", e))
        }
    } else {
        let now = Epoch::now().unwrap_or_else(|e| panic!("failed to determine system time: {}", e));

        let (y, m, d, _, _, _, _) = now.to_gregorian_utc();

        if let Some((hh, mm, ss)) = cli.custom_time() {
            Epoch::from_gregorian_utc(y, m, d, hh, mm, ss, 0)
        } else {
            now
        }
    }
}

fn main() {
    let cli = Cli::new();

    let quiet = cli.quiet();
    let input_path = cli.input_path();

    let mut rinex =
        Rinex::from_file(input_path).unwrap_or_else(|e| panic!("RINEX parsing error: {}", e));

    rinex.rnx2crnx_mut();

    let version_major;

    if let Some(ref mut obs_header) = rinex.header.obs {
        if let Some(ref mut crx_header) = obs_header.crinex {
            version_major = crx_header.version.major;
            crx_header.date = compression_date_time(&cli);
        } else {
            panic!("Internal error: invalid CRINEX content generated.");
        }
    } else {
        panic!("Internal error: parsed invalid OBS RINEX content.");
    }

    let forced_v1 = cli.forced_short_v1() || version_major == 1;

    let output_name = match cli.custom_name() {
        Some(path) => path.clone(), // use customized name
        _ => rinex.standard_filename(forced_v1, None, None),
    };

    let output_path = match cli.custom_prefix() {
        Some(prefix) => format!("{}/{}", prefix, output_name),
        None => output_name.to_string(),
    };

    rinex
        .to_file(&output_path)
        .unwrap_or_else(|e| panic!("CRINEX formatting error: {}", e));

    if !quiet {
        println!("Compressed {}", output_path);
    }
}
