use clap::Parser;

#[derive(Parser,Debug)]
struct OptionsArg{ 
    #[clap(short = 'p', long = "optionspath")]
    OptionsPath: String,

    #[clap(short = 'f', long = "populatefile", default_value = "")]
    PopulateFile: String
}

fn main() {    
    let args = OptionsArg::parse();
    // const OPTIONS: OptionArgs = {
    //     Path: &
    // };

    dbg!(args);
}
