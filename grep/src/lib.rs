use failure::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
pub struct Flags {
    print_line_numbers: bool,
    file_names_only: bool,
    case_insensitive: bool,
    negate_search: bool,
    full_line_match: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut print_line_numbers = false;
        let mut file_names_only = false;
        let mut case_insensitive = false;
        let mut negate_search = false;
        let mut full_line_match = false;

        for flag in flags {
            match *flag {
                "-n" => print_line_numbers = true,
                "-l" => file_names_only = true,
                "-i" => case_insensitive = true,
                "-v" => negate_search = true,
                "-x" => full_line_match = true,
                _ => (),
            }
        }

        Flags {
            print_line_numbers,
            file_names_only,
            case_insensitive,
            negate_search,
            full_line_match,
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let match_str = if flags.case_insensitive {
        pattern.to_lowercase()
    } else {
        pattern.to_string()
    };

    let has_multiple_files = files.len() > 1;
    let mut results = vec![];

    'files: for filename in files {
        let file = File::open(filename)?;
        let buf_reader = BufReader::new(file);

        for (line_index, line) in buf_reader.lines().map(|l| l.unwrap()).enumerate() {
            let line_str = if flags.case_insensitive {
                line.to_lowercase()
            } else {
                line.to_string()
            };

            let mut is_match = if flags.full_line_match {
                line_str == match_str
            } else {
                line_str.contains(&match_str)
            };

            if flags.negate_search {
                is_match = !is_match;
            }

            if !is_match {
                continue;
            }

            if flags.file_names_only {
                results.push(filename.to_string());
                continue 'files;
            }

            let mut result_str = String::new();

            if has_multiple_files {
                result_str += &(filename.to_string() + ":");
            }

            if flags.print_line_numbers {
                result_str += &((line_index + 1).to_string() + ":");
            }

            result_str += &line;

            results.push(result_str.to_string());
        }
    }

    Ok(results)
}
