// Brain Rust builder.
// Basiclly just run the C compiler on the C file from the compiler

use std::process::Command;

pub fn build(prog_c_name: &str, output_name: &str){
    // Run gcc
    let result = Command::new("gcc")
            .arg(prog_c_name)
            .arg("-o")
            .arg(output_name)
            .output()
            .expect("failed to execute process");
    
    // Take gcc output to a String
    let output = match String::from_utf8(result.stdout){
        Err(e) => panic!("Error: {}", e),
        Ok(r) => r
    };

    // Print the output in case it has an error
    println!("{}", output);
}