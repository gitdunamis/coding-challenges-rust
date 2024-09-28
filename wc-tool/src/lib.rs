use core::panic;
use std::error::Error;
use std::fmt::Write;
use std::fs;
use std::io::BufRead;
use crate::ProgOption::{CountBytes, CountLines};


pub struct ProgArgs {
    flag: Vec<ProgOption>,
    filename: String
}
impl ProgArgs {


    pub fn build(args: impl Iterator<Item = String>) -> Self {
        let args: Vec<String> = args.skip(1).collect();
        let count_char = args.iter().any(|arg| arg.eq_ignore_ascii_case("-c"));
        let count_lines = args.iter().any(|arg| arg.eq_ignore_ascii_case("-l"));

        if !count_char && !count_lines {
            panic!("Specify an option either -c or -l")
        }

        let mut opts:Vec<ProgOption> = vec![];

        if count_lines {
            opts.push(CountLines);
        }

        if count_char {
            opts.push(CountBytes);
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
    CountLines
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
                let num_bytes = count_content_length(&contents);
                output.write_str(format!("{num_bytes} ").as_str()).unwrap();
            }
        }
    }

    println!("{} {}", output.to_string(), &prog_args.filename);

    Ok(())
}

fn count_content_lines(contents: &Vec<u8>) -> usize {
    contents.lines().count()
}

fn count_content_length(contents: &Vec<u8>) -> usize {
    contents.len()
}

#[cfg(test)]
mod tests {
    use crate::{count_content_length, count_content_lines};

    #[test]
    fn count_contents_properly() {
        let contents: &str = "This content has length 26";
        let second_contents: &str = "This is another content that does not have length 26";

        assert_eq!(26, count_content_length(&contents.bytes().collect::<Vec<u8>>()));
        assert_ne!(26, count_content_length(&second_contents.bytes().collect::<Vec<u8>>()))
    }

    #[test]
    fn count_lines_properly() {
        let contents: &str = "This content has length 26\nAnother Line";
        let second_contents: &str = "This is another content that does not have length 26";

        assert_eq!(2, count_content_lines(&contents.bytes().collect::<Vec<u8>>()));
        assert_eq!(1, count_content_lines(&second_contents.bytes().collect::<Vec<u8>>()))
    }


}
