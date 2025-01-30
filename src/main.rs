//! # rsabsint
//!
//! Library made for static analysis by abstract interpretation of a small set of the C language.
//! Further syntax support and domains will be implemented : this is my playground for learning.
//! Usage :
//! ```bash
//! ./rsabsint [ARGS]* file.c
//! cargo run [ARGS]* file.c
//! ```
//! 1 and only 1 C file should be precised, and arguments are chosen among :
//! `-concrete`, `-constant`, `-interval`, `-disjonctive`, `-unroll n`, `-delay n` (n : u32)
use std::env;
use rsabsint::frontend::file_parser::*;
use rsabsint::ast::{display_program, Program};

fn help(binary_path : String) {
    println!("usage: {} [ARGS]* [file].c\n
    [ARGS] can be '-concrete', '-constant', '-interval',
    '-disjonctive', '-unroll n', '-delay n' (n : u32)",
    binary_path);
}

fn main() {
    let mut args : Vec<String> = env::args().collect();
    let binary_path = args.remove(0);

    let target_files : Vec<&String> =
        args
        .iter()
        .filter(|arg : &&String| -> bool { arg.contains(".c") })
        .collect();

    if let [target_file] = target_files[..] {
        println!("analyzing {}", target_file);
        let parameters : Vec<String> =
            args
            .iter()
            .filter(|arg: &&String| -> bool { arg != &target_file })
            .cloned()
            .collect();

        let mut i : usize = 0;
        let parameters_length : usize = parameters.len();

        while i < parameters_length {
            let str_parameter = parameters[i].trim();
            match str_parameter {
                "-concrete" => (),
                "-constant" => (),
                "-interval" => (),
                "-disjonctive" => (),
                "-unroll" =>
                    if i + 1 < parameters_length {
                        i += 1;
                        let _unroll_number : u32 =
                            parameters[i].trim().parse().unwrap();
                    }
                    else {
                        help(binary_path);
                        panic!("-unroll without argument");
                    },
                "-delay" =>
                    if i + 1 < parameters_length {
                        i += 1;
                        let _delay_number : u32 =
                            parameters[i].trim().parse().unwrap();
                    }
                    else {
                        help(binary_path);
                        panic!("-delay without argument");
                    },
                _ => {
                    help(binary_path);
                    panic!("unknown option {}", str_parameter);
                }
            }
            i += 1;
        }
        let program: Program =
            parse_file(target_file.to_string()).unwrap();
        display_program(program);
    }
    else {
        help(binary_path);
        panic!("1 [file.c] should be precised.");
    }
}
