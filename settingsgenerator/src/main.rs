use clap::Parser;
use structs::options_arg::OptionsArg;

pub mod structs;

fn main() {    
    let args: OptionsArg = OptionsArg::parse();

    dbg!(args);
}
