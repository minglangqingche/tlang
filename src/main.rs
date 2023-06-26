use std::env;
use tlang::interpreter_error;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        interpreter_error::error_exit("Usage: jlox [script]", 64);
    }else if args.len() < 2 {
        // todo run with tshell
        tlang::tshlle::tshlle();
    }else {
        // todo run with tlang
        tlang::tlang::run(&args[1]);
    }
}
