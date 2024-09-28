use core::panic;
use std::error::Error;
use std::fmt::Write;
use std::fs;
use std::io::BufRead;
use crate::ProgOption::{CountBytes, CountLines, CountWords, CountChars};


pub struct ProgArgs {
    flag: Vec<ProgOption>,
    filename: String
}
impl ProgArgs {


    pub fn build(args: impl Iterator<Item = String>) -> Self {
        let args: Vec<String> = args.skip(1).collect();
        let count_bytes = args.iter().any(|arg| arg.eq_ignore_ascii_case("-c"));
        let count_lines = args.iter().any(|arg| arg.eq_ignore_ascii_case("-l"));
        let count_words = args.iter().any(|arg| arg.eq_ignore_ascii_case("-w"));
        let count_chars : bool = args.iter().any(|arg: &String| arg.eq_ignore_ascii_case("-m"));

        if !count_bytes && !count_lines && !count_words && !count_chars {
            panic!("Specify an option either -c or -l or -w or -m")
        }

        let mut opts:Vec<ProgOption> = vec![];

        if count_lines {
            opts.push(CountLines);
        }

        if count_words {
            opts.push(CountWords)
        }

        if count_bytes {
            opts.push(CountBytes);
        }

        if count_chars {
            opts.push(CountChars);
        }

        let file = args.last().unwrap();


        Self {
            flag: opts,
            filename: file.to_owned()
        }
    }
}

pub enum ProgOption {
    CountBytes,
    CountLines,
    CountWords,
    CountChars
}


/// Perform whole program operation. Parsing arguments and producing outputs
pub fn process(prog_args: ProgArgs) -> Result<(),  Box<dyn Error>> {
    let contents = fs::read(&prog_args.filename)?;
    let mut output: String = String::new();

    for arg in prog_args.flag {
        match arg {
            CountLines => {
                let num_lines: usize = count_content_lines(&contents);
                output.write_str(format!("{num_lines} ").as_str()).unwrap()
            }
            CountBytes => {
                let num_bytes = count_content_bytes(&contents);
                output.write_str(format!("{num_bytes} ").as_str()).unwrap();
            }
            CountWords => {
                let num_words: usize = count_content_words(&contents);
                output.write_str(format!("{num_words} ").as_str()).unwrap();
            }
            CountChars => {
                let num_chars: usize = count_content_characters(&contents);
                output.write_str(format!("{num_chars} ").as_str()).unwrap();
            },
        }
    }

    println!("{} {}", output.to_string(), &prog_args.filename);

    Ok(())
}

fn count_content_characters(contents: &[u8]) -> usize {
    String::from_utf8(contents.to_vec()).unwrap().chars().count()
}

fn count_content_words(contents: &[u8]) -> usize {
    let content_as_string: String = String::from_utf8(contents.to_vec()).unwrap();
    content_as_string.split_ascii_whitespace().count()
}

fn count_content_lines(contents: &Vec<u8>) -> usize {
    contents.lines().count()
}

fn count_content_bytes(contents: &Vec<u8>) -> usize {
    contents.len()
}

#[cfg(test)]
mod tests {
    use crate::{count_content_bytes, count_content_lines,
        count_content_words, count_content_characters};

    #[test]
    fn count_bytes_properly() {
        let contents: &str = "This content has length 30 ∂";
        let second_contents: &str = "This is another content that does not have length 26";

        assert_eq!(30, count_content_bytes(&contents.bytes().collect::<Vec<u8>>()));
        assert_ne!(26, count_content_bytes(&second_contents.bytes().collect::<Vec<u8>>()))
    }

    #[test]
    fn count_lines_properly() {
        let contents: &str = "This content has length 26\nAnother Line";
        let second_contents: &str = "This is another content that does not have length 26";

        assert_eq!(2, count_content_lines(&contents.bytes().collect::<Vec<u8>>()));
        assert_eq!(1, count_content_lines(&second_contents.bytes().collect::<Vec<u8>>()))
    }

    #[test]
    fn count_words_properly() {
        let contents: &str = "This content has length 26\nAnother Line";
        let second_contents: &str = "This";
        let third_contents: &str = "";

        assert_eq!(7, count_content_words(&contents.bytes().collect::<Vec<u8>>()));
        assert_eq!(1, count_content_lines(&second_contents.bytes().collect::<Vec<u8>>()));
        assert_eq!(0, count_content_lines(&third_contents.bytes().collect::<Vec<u8>>()));
    }

    #[test]
    fn count_characters_properly() {
        let contents: &str = "This content has length 30 ∂";
        let second_contents: &str = "This is another content that does not have length 26";

        assert_eq!(28, count_content_characters(&contents.bytes().collect::<Vec<u8>>()));
        assert_ne!(1, count_content_characters(&second_contents.bytes().collect::<Vec<u8>>()));
    }
}
