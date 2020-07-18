// BrainRust compiler.
// Compiler/convert brainfuck to C code

pub fn compile_to_c(bf_code: &Vec<char>) -> String{
    // String to store the C code
    let mut c: String = String::new();
    
    // Convert each symbol to it's C equivalent and store it in in 'c' variable
    for com in bf_code.iter(){
        if com == &'>'{ c.push_str("++ptr;\n");}
        else if com == &'<' {c.push_str("--ptr;\n");}
        else if com == &'+' { c.push_str("++*ptr;\n");}
        else if com == &'-' {c.push_str("--*ptr;\n");}
        else if com == &'.' { c.push_str("putchar(*ptr);\n");}
        else if com == &',' { c.push_str("*ptr=getchar();\n");}
        else if com == &'[' { c.push_str("while (*ptr) {\n");}
        else if com == &']' { c.push_str("}\n");}
        
    }

    // Return the C string
    c
}