use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Opts {
    year: String,
    start: i32,
    end: Option<i32>,
}

fn main() {
    let Opts { start, end, year } = Opts::parse();

    let src_directory = format!("src/_{year}");
    let input_directory = format!("input/{year}");

    fs::create_dir_all(&src_directory).expect("Failed to create src directory");
    fs::create_dir_all(&input_directory).expect("Failed to create input directory");

    let mut mod_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(format!("{}/mod.rs", &src_directory))
        .or_else(|_| File::create(format!("{}/mod.rs", &src_directory)))
        .expect("Unable to open file");

    for i in start..=end.unwrap_or(start) {
        let day = format!("{i:02}");
        let file_name = format!("{src_directory}/_{day}.rs");

        File::create(format!("{}/{}.txt", &input_directory, &day))
            .expect("unable to create input file");

        fs::write(
            &file_name,
            format!(
                r##"pub fn parse(input: &str) -> i32 {{
    input.parse().unwrap_or(0)
}}

#[cfg(test)]
mod tests {{
    use super::*;
    
    const SAMPLE: &str = "\
";
    const INPUT: &str = include_str!("../../input/{year}/{day}.txt");

    #[test]
    fn test_1_sample() {{
        let input = parse(SAMPLE);

        assert_eq!(input, 1 + 1);
    }}

    #[test]
    fn test_1() {{
        let input = parse(INPUT);

        assert_eq!(input, 1 + 1);
    }}

    #[test]
    fn test_2_sample() {{
        let input = parse(SAMPLE);

        assert_eq!(input, 1 + 1);
    }}

    #[test]
    fn test_2() {{
        let input = parse(INPUT);

        assert_eq!(input, 1 + 1);
    }}
}}"##,
            ),
        )
        .expect("Unable to write src file");

        writeln!(mod_file, "pub mod _{};", &day).expect("Unable to write to mod line");
    }
}
