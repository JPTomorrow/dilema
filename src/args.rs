use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    // text to be manipulated
    #[clap(short, long, value_parser)]
    pub input_text: String,

    // the match pattern to modify input text against
    #[clap(short, long, value_parser, required = true)]
    pub match_pattern: String,
}
