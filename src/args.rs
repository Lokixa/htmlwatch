use clap::Parser;

/// Simple program to monitor an html page
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_parser)]
    url: String,

    /// On start you will be asked to enter credentials.
    #[clap(short, long, value_parser, default_value_t = false)]
    use_credentials: bool,
}
//TODO fetch credentials and store them encrypted.
pub fn handle_args() -> bool {
    let args = Args::parse();

    return false;
}

//fn print_help() {}
