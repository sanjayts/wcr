use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

pub type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    chars: bool,
    bytes: bool,
}

#[derive(Debug, PartialEq)]
struct WcrStats {
    num_lines: usize,
    num_words: usize,
    num_chars: usize,
    num_bytes: usize,
}

impl WcrStats {
    fn new(num_lines: usize, num_words: usize, num_bytes: usize, num_chars: usize) -> WcrStats {
        WcrStats {
            num_lines,
            num_words,
            num_chars,
            num_bytes,
        }
    }

    fn merge_with(&mut self, other: &Self) -> &mut Self {
        self.num_lines += other.num_lines;
        self.num_words += other.num_words;
        self.num_bytes += other.num_bytes;
        self.num_chars += other.num_chars;
        self
    }

}

fn format_field(field_val: usize, show_field: bool) -> String {
    if show_field {
        format!("{:>8}", field_val)
    } else {
        "".to_string()
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let mut all_stats = WcrStats::new(0, 0, 0, 0);
    for file_name in config.files.iter() {
        match open_file(file_name.as_ref()) {
            Ok(reader) => {
                let stats = compute_stats(reader, &config)?;
                println!(
                    "{}{}{}{}{}",
                    format_field(stats.num_lines, config.lines),
                    format_field(stats.num_words, config.words),
                    format_field(stats.num_chars, config.chars),
                    format_field(stats.num_bytes, config.bytes),
                    if file_name == "-" { "".to_owned() } else { format!(" {}", file_name) },
                );
                if config.files.len() > 1 {
                    all_stats.merge_with(&stats);
                }
            }
            Err(e) => eprintln!("wcr: {}: {}", file_name, e),
        }
    }
    if config.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(all_stats.num_lines, config.lines),
            format_field(all_stats.num_words, config.words),
            format_field(all_stats.num_chars, config.chars),
            format_field(all_stats.num_bytes, config.bytes)
        );
    }
    Ok(())
}

fn compute_stats(mut reader: impl BufRead, config: &Config) -> MyResult<WcrStats> {
    let mut stats = WcrStats::new(0, 0, 0, 0);
    let mut buf = String::new();

    loop {
        let bytes_read = reader.read_line(&mut buf)?;
        if bytes_read == 0 {
            break;
        }
        if config.bytes {
            stats.num_bytes += buf.len();
        }
        if config.chars {
            stats.num_chars += buf.chars().count();
        }
        if config.words {
            stats.num_words += buf.split_whitespace().count();
        }
        if config.lines && buf.pop() == Some('\n') {
            stats.num_lines += 1;
        }
        // Very important to clear the buffer since repeated invocations of read append to the buf
        buf.clear();
    }

    Ok(stats)
}

fn open_file(file_path: &str) -> MyResult<Box<dyn BufRead>> {
    match file_path {
        "-" => Ok(Box::new(BufReader::new(stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file_path)?))),
    }
}

pub fn parse_config(cmd_args: Vec<String>) -> MyResult<Config> {
    let matches = App::new("wcr")
        .name("wcr")
        .long_about("Print newline, word, and byte counts for each file")
        .author("sanjayts")
        .version("1.0.0")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input files")
                .multiple_values(true)
                .default_value("-"),
        )
        .arg(
            Arg::new("lines")
                .short('l')
                .long("lines")
                .help("print the newline counts")
                .takes_value(false),
        )
        .arg(
            Arg::new("words")
                .short('w')
                .long("words")
                .help("print the word counts")
                .takes_value(false),
        )
        .arg(
            Arg::new("chars")
                .short('m')
                .long("chars")
                .help("print the character counts")
                .takes_value(false)
                .conflicts_with("bytes")
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .help("print the byte counts")
                .takes_value(false)
        )
        .get_matches_from(cmd_args);

    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut chars = matches.is_present("chars");
    let mut bytes = !chars && matches.is_present("bytes");
    let files = matches
        .get_many::<String>("files")
        .unwrap()
        .map(|s| s.to_owned())
        .collect();

    if !lines && !words && !chars && !bytes {
        lines = true;
        words = true;
        chars = false;
        bytes = true;
    }

    let config = Config {
        files,
        lines,
        words,
        chars,
        bytes,
    };
    Ok(config)
}

#[cfg(test)]
mod lib_tests {
    use crate::{compute_stats, Config, WcrStats};
    use std::io;

    #[test]
    fn test_compute_stats() {
        let config = Config {
            files: vec![],
            lines: true,
            words: true,
            chars: true,
            bytes: false,
        };
        let reader = io::Cursor::new("नमस्ते\n");
        let res = compute_stats(reader, &config);

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), WcrStats::new(1, 1, 0, 7))
    }
}
