use std::env;
use wc_tool::ProgArgs;

fn main() {
    //read commandline option -c
    let prog_args: ProgArgs  = ProgArgs::build(&mut env::args());

    if let Err(e) = wc_tool::process(prog_args) {
        println!("Process did not complete successfully: {e}")
    }
}

