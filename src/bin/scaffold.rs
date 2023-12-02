use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Opts {
    start: i32,
    end: i32,
    year: String,
}

fn main() {
    let Opts { start, end, year } = Opts::parse();

    let file_content = "\
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_sample() {
        assert_eq!(2, 1 + 1);
    }

    #[test]
    fn test_1() {
        assert_eq!(2, 1 + 1);
    }

    #[test]
    fn test_2_sample() {
        assert_eq!(2, 1 + 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(2, 1 + 1);
    }
}";

    let directory = format!("src/_{}", year);

    fs::create_dir_all(&directory).expect("Failed to create directory");

    let mut mod_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(format!("{}/mod.rs", &directory))
        .or_else(|_| fs::File::create(format!("{}/mod.rs", &directory)))
        .expect("Unable to open file");

    for i in start..=end {
        let formatted_index = format!("{:02}", i);
        let file_name = format!("{}/_{}.rs", &directory, &formatted_index);

        fs::write(&file_name, &file_content).expect("Unable to write file");

        writeln!(mod_file, "pub mod _{};", &formatted_index).expect("Unable to write to file");
    }
}
