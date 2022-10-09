mod args;
use args::Args;

mod english_dict;
mod ultrawild;
use ultrawild::UltraWild;

#[cfg(debug_assertions)]
fn get_args() -> Args {
    // default testing args if in debug
    Args {
        input_text: "test\ntest2\ntest3\ntest4".to_string(),
        match_pattern: "*.*".to_string(),
    }
}

#[cfg(not(debug_assertions))]
fn get_args() -> Args {
    // get args from command line if in release
    Args::parse()
}

fn main() {
    let args = get_args();

    let _formatter = UltraWild::new(args.input_text, args.match_pattern);

    _formatter.print_origional_lines();
}
