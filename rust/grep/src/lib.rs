use failure::Error;
use std::io::{BufReader, BufRead};
use std::fs::File;

/// While using raw slice of str to handle flags is convenient,
/// in the real-world projects it is customary to use a struct,
/// that contains flags-related logic. So in this exercise
/// we ask you to implement a custom struct.
///
/// If you are curious about real-world implementation, refer to the `clap-rs` crate:
/// https://github.com/kbknapp/clap-rs/blob/master/src/args/arg_matches.rs
#[derive(Default, Debug)]
pub struct Flags {
    n: bool,
    l: bool,
    i: bool,
    v: bool,
    x: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut f = Flags::default();
        flags.iter()
            .for_each(|&s| {
                match s {
                    "-n" => f.n = true,
                    "-l" => f.l = true,
                    "-i" => f.i = true,
                    "-v" => f.v = true,
                    "-x" => f.x = true,
                    _ => {}
                }
            });
        f
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut output = Vec::new();

    let pattern_lower;
    let pattern = if flags.i {
        pattern_lower = pattern.to_lowercase();
        &pattern_lower
    } else {
        pattern
    };

    if let [filename] = files {
        let file = File::open(filename)?;
        for (i, line) in BufReader::new(file).lines()
            .enumerate() {
            let line = line?;

            if is_match(&line, pattern, flags) {
                if flags.l {
                    output.push(filename.to_string());
                    break;
                }

                output.push(if flags.n {
                    format!("{}:{}", i + 1, line)
                } else {
                    line
                });
            }
        }
    } else {
        for filename in files {
            let file = File::open(filename)?;
            for (i, line) in BufReader::new(file).lines()
                .enumerate() {
                let line = line?;

                if is_match(&line, pattern, flags) {
                    if flags.l {
                        output.push(filename.to_string());
                        break;
                    }

                    output.push(if flags.n {
                        format!("{}:{}:{}", filename, i + 1, line)
                    } else {
                        format!("{}:{}", filename, line)
                    });
                }
            }
        }
    }

    Ok(output)
}

fn is_match(line: &str, pattern: &str, flags: &Flags) -> bool {
    let line_lower;
    let line = if flags.i {
        line_lower = line.to_lowercase();
        &line_lower
    } else {
        line
    };

    (if flags.x {
        line == pattern
    } else {
        line.contains(pattern)
    }) != flags.v
}
