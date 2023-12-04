use crate::input_parsing::{Input::*, Separator::*};
use itertools::Itertools;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

#[derive(Copy, Clone)]
pub enum Input<'a> {
    Path(&'a str),
    Raw(&'a str),
}

#[derive(Copy, Clone)]
pub enum Separator {
    Comma,
    Newline,
}

pub fn to_lines(input: Input) -> Box<dyn Iterator<Item = String> + '_> {
    match input {
        Path(path) => {
            let file = File::open(path).expect("Failed to open file");
            let reader = BufReader::new(file);
            Box::new(
                reader
                    .lines()
                    .map_while(Result::ok)
                    .map(|s| s.trim().to_owned())
                    .filter(|s| !s.is_empty()),
            )
        }
        Raw(s) => Box::new(
            s.lines()
                .map(|s| s.trim().to_owned())
                .filter(|s| !s.is_empty()),
        ),
    }
}

pub fn to_vec<T>(input: Input, delim: Separator) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let str = match input {
        Path(path) => {
            let mut file = File::open(path).unwrap();
            let mut str = String::new();
            file.read_to_string(&mut str).unwrap_or_default();
            str
        }
        Raw(s) => s.to_string(),
    };
    let string_parser = |s: &str| s.parse::<T>().ok();
    match delim {
        Newline => str
            .lines()
            .map(str::trim)
            .filter_map(string_parser)
            .collect_vec(),
        Comma => str.split(',').filter_map(string_parser).collect_vec(),
    }
}
