use clap::Parser;

#[derive(Parser,Debug)]
pub struct OptionsArg{ 
    #[clap(short = 'p', long = "options-path")]
    options_path: String,

    #[clap(short = 'n', long = "setting-file-name")]
    setting_file_name: String,

    #[clap(short = 'f', long = "populate-file", default_value = "")]
    populate_file: String
}