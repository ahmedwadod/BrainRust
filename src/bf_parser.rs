
pub fn parse_bf(code: String) -> Vec<char>{
    let mut syms: Vec<char> = vec![];
    let bf_syntax:[char;8] = ['<', '>', '+', '-', '[', ']', ',', '.'];
    for x in code.chars(){
        if bf_syntax.contains(&x){
            syms.push(x);
        }
    }

    syms
}