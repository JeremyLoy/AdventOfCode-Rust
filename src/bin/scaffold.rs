use std::fs;
use std::fs::{File, OpenOptions};
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

    let src_directory = format!("src/_{}", year);
    let input_directory = format!("input/{}", year);

    fs::create_dir_all(&src_directory).expect("Failed to create src directory");
    fs::create_dir_all(&input_directory).expect("Failed to create input directory");

    let mut mod_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(format!("{}/mod.rs", &src_directory))
        .or_else(|_| File::create(format!("{}/mod.rs", &src_directory)))
        .expect("Unable to open file");

    for i in start..=end {
        let day = format!("{:02}", i);
        let file_name = format!("{}/_{}.rs", &src_directory, &day);

        File::create(format!("{}/{}.txt", &input_directory, &day))
            .expect("unable to create input file");

        fs::write(
            &file_name,
            format!(
                r##"
#[cfg(test)]
mod tests {{
    use super::*;
    use crate::input_parsing::Input::{{Path, Raw}};

    #[test]
    fn test_1_sample() {{
        let input = Raw("\
");

        assert_eq!(2, 1 + 1);
    }}

    #[test]
    fn test_1() {{
        let input = Path("input/{}/{}.txt");

        assert_eq!(2, 1 + 1);
    }}

    #[test]
    fn test_2_sample() {{
        let input = Raw("\
");

        assert_eq!(2, 1 + 1);
    }}

    #[test]
    fn test_2() {{
        let input = Path("input/{}/{}.txt");

        assert_eq!(2, 1 + 1);
    }}
}}"##,
                year, day, year, day
            ),
        )
        .expect("Unable to write src file");

        writeln!(mod_file, "pub mod _{};", &day).expect("Unable to write to mod line");
    }
}
