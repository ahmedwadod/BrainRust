use std::process::Command;

pub fn build(prog_c_name: &str, output_name: &str){
    let result = Command::new("gcc")
            .arg(prog_c_name)
            .arg("-o")
            .arg(output_name)
            .output()
            .expect("failed to execute process");
    
    let output = match String::from_utf8(result.stdout){
        Err(e) => panic!("Error: {}", e),
        Ok(r) => r
    };
    println!("{}", output);
}