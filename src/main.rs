use clap::{App, Arg};
use regex::{Captures, Regex};
use std::{
    fs::{read_to_string, write},
    path::PathBuf,
};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Markdown template file name")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Markdown generated file name")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("root")
                .short("r")
                .long("root")
                .value_name("FILE")
                .help("Source files root path")
                .takes_value(true)
                .required(false),
        )
        .get_matches();
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();
    let root = PathBuf::from(matches.value_of("root").unwrap_or_default());
    let pattern = r"([\t ]*)```\s*(\w+)\s*:\s*source(\s*@\s*(\S+))?\s+(\S+)\s+```";
    let pattern = Regex::new(pattern)
        .unwrap_or_else(|error| panic!("Could not build pattern: {} | {:?}", pattern, error));
    let pattern_escape = r"(```\s*\w+\s*:\s*)!(\s*source(\s*@\s*\S+)?[\t ]*)";
    let pattern_escape = Regex::new(pattern_escape).unwrap_or_else(|error| {
        panic!(
            "Could not build escape-pattern: {} | {:?}",
            pattern_escape, error
        )
    });
    let pattern_begin = r"//\s*\[\s*md-bakery\s*:\s*begin(\s*@\s*(\S+))?\s*\]";
    let pattern_begin = Regex::new(pattern_begin).unwrap_or_else(|error| {
        panic!(
            "Could not build begin-pattern: {} | {:?}",
            pattern_begin, error
        )
    });
    let pattern_end = r"//\s*\[\s*md-bakery\s*:\s*end\s*\]";
    let pattern_end = Regex::new(pattern_end).unwrap_or_else(|error| {
        panic!("Could not build end-pattern: {} | {:?}", pattern_end, error)
    });
    let content = read_to_string(input)
        .unwrap_or_else(|error| panic!("Could not load input file: {} | {:?}", input, error));
    let content = pattern.replace_all(&content, |captures: &Captures| {
        let indent = captures.get(1).unwrap().as_str();
        let lang = captures.get(2).unwrap().as_str();
        let name = match captures.get(4) {
            Some(name) => name.as_str().to_owned(),
            None => String::new(),
        };
        let path = root.join(captures.get(5).unwrap().as_str().trim());
        let content = read_to_string(&path)
            .unwrap_or_else(|error| panic!("Could not load source file: {:?} | {:?}", path, error));
        let lines = content
            .lines()
            .map(|line| line.to_owned())
            // .map(|line| format!("{}{}", indent, line))
            .collect::<Vec<String>>();
        let found = lines.iter().any(|line| {
            if let Some(captures) = pattern_begin.captures(line) {
                if let Some(n) = captures.get(2) {
                    if n.as_str() == name {
                        return true;
                    }
                }
            }
            false
        });
        let lines = if found {
            let mut record = false;
            lines
                .into_iter()
                .filter(|line| {
                    if record {
                        if pattern_end.is_match(line) {
                            record = false;
                        }
                        record
                    } else {
                        if let Some(captures) = pattern_begin.captures(line) {
                            let n = match captures.get(2) {
                                Some(n) => n.as_str().to_owned(),
                                None => String::new(),
                            };
                            if n == name {
                                record = true;
                            }
                        }
                        false
                    }
                })
                .collect::<Vec<_>>()
        } else {
            lines
                .into_iter()
                .map(|line| format!("{}{}", indent, line))
                .collect::<Vec<_>>()
        };
        let common_prefix = common_whitespace_prefix(&lines);
        let content = lines
            .into_iter()
            .map(|line| {
                format!(
                    "{}{}",
                    indent,
                    line.strip_prefix(&common_prefix).unwrap_or_default()
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        format!("{}```{}\n{}\n{}```", indent, lang, content, indent)
    });
    let content = pattern_escape.replace_all(&content, |captures: &Captures| {
        let first = captures.get(1).unwrap().as_str();
        let second = captures.get(2).unwrap().as_str();
        format!("{}{}", first, second)
    });
    write(output, &*content)
        .unwrap_or_else(|error| panic!("Could not write output file: {} | {:?}", output, error));
}

fn common_whitespace_prefix(lines: &[String]) -> String {
    if lines.is_empty() {
        return String::new();
    }
    let mut result: Option<String> = None;
    for line in lines {
        if !line.is_empty() {
            let prefix = take_whitespaces_prefix(&line);
            if let Some(result) = result.as_mut() {
                *result = result
                    .chars()
                    .zip(prefix.chars())
                    .take_while(|(a, b)| a == b)
                    .map(|(a, _)| a)
                    .collect::<String>();
            } else {
                result = Some(prefix);
            }
        }
    }
    result.unwrap_or_default()
}

fn take_whitespaces_prefix(value: &str) -> String {
    value
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>()
}
