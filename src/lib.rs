pub mod tlang {
    mod read_file {
        use std::fs;

        pub fn read_file(path: &str) -> String {
            match fs::read_to_string(&path) {
                Ok(val) => {
                    val
                },
                Err(_) => {
                    crate::interpreter_error::error_exit(&format!("can't read file ' {} '!", path), 64);
                    "".to_string()
                }
            }
        }
    }

    pub mod vm {
        use super::read_file;
        use crate::lexical_analysis::scanner;

        pub fn run(path: &str) {
            let code = read_file::read_file(path);
            let mut scanner = scanner::Scanner::new(code);
            let t = scanner.scann();
            match t {
                Ok(val) => {
                    for t in val {
                        println!("{:?}", t);
                    }
                },
                Err(err) => {
                    crate::interpreter_error::error_exit(&format!("lexical analysis errror!total {} error.", err), 65);
                }
            }
        }
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

pub mod chunk {
    pub mod op;
    pub mod chunk;
    pub mod value;
}

pub mod debug_tools {
    pub mod disassemble;
}

pub mod vm {
    pub mod vm;
    pub mod interpret_result;
}
