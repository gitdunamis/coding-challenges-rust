use std::error::Error;
use std::fs;
use crate::ProgOption::CountBytes;


pub struct ProgArgs {
    flag: ProgOption,
    filename: String
}
impl ProgArgs {


    pub fn build(mut args: impl Iterator<Item = String>) -> Self {
        args.find(|arg| arg.eq_ignore_ascii_case("-c"))
            .expect("only supports -c for now");

        let file = args.next().expect("file to operate on must be next");

        Self {
            flag: CountBytes,
            filename: file
        }
    }
}

pub enum ProgOption {
    CountBytes
}


/// Perform whole program operation. Parsing arguments and producing outputs
pub fn process(prog_args: ProgArgs) -> Result<(),  Box<dyn Error>> {
    let contents = fs::read(&prog_args.filename)?;
    let num_bytes = count_content_length(contents);

    println!("{num_bytes} {}", &prog_args.filename);

    Ok(())
}

fn count_content_length(contents: Vec<u8>) -> usize {
    contents.len()
}

#[cfg(test)]
mod tests {
    use crate::count_content_length;

    #[test]
    fn count_contents_properly() {
        let contents: &str = "This content has length 26";
        let second_contents: &str = "This is another content that does not have length 26";

        assert_eq!(26, count_content_length(contents.bytes().collect::<Vec<u8>>()));
        assert_ne!(26, count_content_length(second_contents.bytes().collect::<Vec<u8>>()))
    }
}
