// define clap command structure here

use clap::{Parser};

#[derive(Parser, Debug)]
#[command(name = "which-tool", version = "0.1.0", author = "learner")]
pub struct Which {
    // argument list
    pub programs: Vec<String>
}
