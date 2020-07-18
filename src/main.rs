// The main file for BrainRust.
// Author: Ahmed Abdelwadod. Date: July 19th 2020.

const VERSION: &str = "0.0.1";
const DESCRIPTION: &str = "BrainRust is Brainfuck to C compiler written in Rust.";

mod cmd_parser;
mod bf_parser;
mod compiler;
mod builder;

use std::fs::File;
use std::path::Path;
use std::{io::Read, io::Write};

const HELPTEXT: &str = "Syntax: bfc <input_file> [-o <output_file>] [-c] [-h]
    []: Optional
    input_file: Brainfuck code file.
    output_file: Name of the executable out file.
    -c: Compile to C only
    -h: help";

const  C_HEADER: &str = "#include<stdio.h>\n
int main(){
char array[30000] = {0};
char *ptr=array;
";

fn main() {
    println!("Welcome to BrainRust Ver {} By Ahmed Abdelwadoud.\n", VERSION);
    let flags = cmd_parser::parse();

    if flags.help_flag{
        println!("{}\n{}", DESCRIPTION, HELPTEXT);
        return;
    }else if flags.error{
        println!("Syntax error!\n{}", HELPTEXT);
        return;
    }else{
        let bf_path = Path::new(flags.file_to_compile.as_str());
        let mut bf_file = match File::open(&bf_path){
            Err(e) => panic!("Error in opening {}: {}", flags.file_to_compile.as_str(), e),
            Ok(f) => f
        };
        let mut bf_string = String::new();
        match bf_file.read_to_string(&mut bf_string) {
            Err(e) => panic!("Error in reading file {}: {}", flags.file_to_compile.as_str(), e),
            Ok(_) => {}
        };

        let code = bf_parser::parse_bf(bf_string);
        let mut c_code: String = String::from(C_HEADER); 
        let in_code = compiler::compile_to_c(&code);
        c_code.push_str(in_code.as_str());
        c_code.push_str("\nreturn 0;\n}");

        let prog_c_path = Path::new("prog.c");
        let mut prog_c_file = match File::create(&prog_c_path){
            Err(e) => panic!("Error in generating prog.c: {}", e),
            Ok(f) => f
        };
        match prog_c_file.write_all(c_code.as_bytes()){
            Err(e) => panic!("Error in generating prog.c: {}", e),
            Ok(_) => print!("Generated prog.c")
        };

        if flags.compile_only_flag{
            return;
        }else{
            let mut output_file_name = String::from("a.out");
            if flags.output_flag{
                output_file_name = flags.output_file.clone();
            }
            builder::build("prog.c", output_file_name.as_str());

            println!("Built {} successfully.", output_file_name);
        }
    }
}