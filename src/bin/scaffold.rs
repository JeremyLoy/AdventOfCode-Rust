use std::fs;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

use clap::Parser;
use reqwest::header::HeaderMap;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Opts {
    year: String,
    start: i32,
    session: Option<String>,
    end: Option<i32>,
}

fn main() {
    download_input();
    scaffold_files();
}

fn download_input() {
    let Opts {
        start,
        end,
        year,
        session,
    } = Opts::parse();
    let session = session.expect("session is required");

    let get_input_file = |year: &str, day: i32| -> String { format!("input/{year}/{day:02}.txt") };

    let get_url = |year: &str, day: i32| -> String {
        format!("https://adventofcode.com/{year}/day/{day}/input")
    };
    // Send a GET request
    let mut headers = HeaderMap::new();
    headers.insert(
        "Cookie",
        format!("session={session}")
            .parse()
            .expect("unable to make header from session"),
    );
    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .expect("built a client");

    for day in start..=end.unwrap_or(start) {
        let url = get_url(&year, day);
        let path = get_input_file(&year, day);

        let mut resp = client.get(&url).send().expect("made a request");

        // Check for status code 200
        if resp.status().is_success() {
            // Ensure parent directory exists before writing
            let output_path = Path::new(&path);
            if let Some(dir) = output_path.parent() {
                create_dir_all(dir).expect("should have created dir");
            }

            // Write the response bytes to a file
            let mut file = File::create(path).expect("should have created input file");
            resp.copy_to(&mut file).expect("able to write to file");
            println!("Successfully created file");
        } else {
            println!("Received response status: {}", resp.status());
        }
    }
}

fn scaffold_files() {
    let Opts {
        start,
        end,
        year,
        session: _,
    } = Opts::parse();

    let src_directory = format!("src/_{year}");

    create_dir_all(&src_directory).expect("Failed to create src directory");

    let mut mod_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(format!("{}/mod.rs", &src_directory))
        .or_else(|_| File::create(format!("{}/mod.rs", &src_directory)))
        .expect("Unable to open file");

    for i in start..=end.unwrap_or(start) {
        let day = format!("{i:02}");
        let file_name = format!("{src_directory}/_{day}.rs");

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
