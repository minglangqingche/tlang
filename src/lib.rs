pub mod tlang {
    mod read_file;

    pub fn read_file(path: &str) {
        println!("reading {}", path);
        println!("{}", read_file::read_file(path));
    }
}

pub mod tshlle {
    use std::io::{self, Write};

    pub fn tshlle() {
        println!("wellcome!this is tshell!");
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(val) => {
                    if val == 0 {
                        break;
                    }
                    println!("read {} chars:\n{}", val, input);
                }
                Err(_) => {
                    crate::interpreter_error::error_exit("input error!", 1);
                }
            }
        }
    }
}

pub mod interpreter_error {
    pub fn error(massege: &str) {
        eprintln!("interpreter error!\nhere:\n{}", massege);
    }

    pub fn error_exit(massege: &str, exit_code: i32) {
        error(massege);
        std::process::exit(exit_code);
    }
}

pub mod lexical_analysis {
    pub mod token;
    pub mod token_type;
    pub mod scanner;
}
