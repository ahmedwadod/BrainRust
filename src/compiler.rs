pub fn compile_to_c(bf_code: &Vec<char>) -> String{
    let mut c: String = String::new();
    
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

    c
}