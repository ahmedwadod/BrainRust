use std::env;

// Compiler Flags type
pub struct CompilerFlags{
    pub file_to_compile: String,
    pub output_flag: bool,
    pub output_file: String,
    pub compile_only_flag: bool,
    pub help_flag: bool,
    pub error: bool
}

// Parse the command line arguments
pub fn parse() -> CompilerFlags{
    // Collect the aruments
    let args: Vec<String> = env::args().collect();

    // Flag to raise for setting the output file
    let mut o_flag = false;

    // resp is the return copiler_flags
    let mut resp: CompilerFlags = CompilerFlags{
        file_to_compile: "".to_string(),
        output_flag: false,
        output_file: "".to_string(),
        compile_only_flag: false,
        help_flag: false,
        error: false
    };

    // Check for the length of the arguments and return if there is no arguments
    if args.len() < 2{
        resp.error = true;
        return resp;
    }

    for a in args{
        if a.starts_with("-"){
            if a == "-o"{
                o_flag = true;
                resp.output_flag = true;
            }
            else if a == "-h"{
                resp.help_flag = true;
                return resp;
            } 
            else if a == "-c" && !o_flag{
                resp.compile_only_flag = true;
            } else{
                resp.error = true;
                return resp;
            }
        }else{
            if o_flag{
                resp.output_file = a;
                o_flag = false;
            }else{
                resp.file_to_compile = a;
            }
        }
    }

    resp
}