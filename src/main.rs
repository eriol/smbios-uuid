use std::process;

use clap::{crate_authors, crate_version, App, Arg};
use colored::Colorize;
use uuid::Uuid;

const ERROR_PARSING: &str = "An error occurred while parsing";

fn main() {
    let matches = App::new("smbios-udid")
        .version(crate_version!())
        .author(crate_authors!())
        .about("")
        .arg(
            Arg::with_name("UUID")
                .help("The UUID to convert")
                .required(true)
                .index(1),
        )
        .get_matches();

    // We can unwrap because UUID is required.
    let uuid = matches.value_of("UUID").unwrap();
    let uuid = match Uuid::parse_str(uuid) {
        Ok(uuid) => uuid,
        Err(err) => {
            println!("{} {}: {}", ERROR_PARSING.red().bold(), &uuid, err);
            process::exit(1);
        }
    };

    // An UUID is defined as follow (RFC4122):
    //
    // 0                   1                   2                   3
    // 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    // |                          time_low                             |
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    // |       time_mid                |         time_hi_and_version   |
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    // |clk_seq_hi_res |  clk_seq_low  |         node (0-1)            |
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    // |                         node (2-5)                            |
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    let (time_low, time_mid, time_hi_and_version, remaining) = uuid.as_fields();

    // RFC4122 recommends network byte order for all fields, but the PC industry
    // has consistently used little-endian byte encoding for the first three
    // fields: time_low, time_mid, time_hi_and_version.
    let uuid = Uuid::from_fields(
        time_low.swap_bytes(),
        time_mid.swap_bytes(),
        time_hi_and_version.swap_bytes(),
        remaining,
    );

    match uuid {
        Ok(uuid) => println!("{}", uuid),
        Err(err) => {
            println!("{}: {}", "Error".red().bold(), err);
            process::exit(1);
        }
    }
}
