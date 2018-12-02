mod cli;
mod days;

extern crate clap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let matches = cli::build_cli().get_matches();

    match matches.subcommand_name() {
        Some("1") => {
            let submatches = matches.subcommand_matches("1").unwrap();
            let mut content = input_to_file(submatches);
            if submatches.is_present("two") {
                days::first::first_day_part_two(&content);
            } else {
                days::first::first_day(&content);
            }
        }
        Some("2") => {
            let submatches = matches.subcommand_matches("2").unwrap();
            let mut content = input_to_file(submatches);
            if submatches.is_present("two") {
                days::second::second_day_part_two(&content);
            } else {
                days::second::second_day(&content);
            }
        }
        _ => panic!("Bug in clap!"),
    }
}

fn input_to_file(submatches: &clap::ArgMatches<'_>) -> String {
    let input_name = submatches.value_of("INPUT").unwrap();
    let mut input = File::open(input_name).expect("Not found");
    let mut content = String::new();
    input
        .read_to_string(&mut content)
        .expect("Error reading file");
    return content;

}
