// BrainRust parser
// Takes the brainfuck symbols out of the code text

pub fn parse_bf(code: String) -> Vec<char>{
    // Vector to store the symbols
    let mut syms: Vec<char> = vec![];

    // Array of Brainfuck symbols
    let bf_syntax:[char;8] = ['<', '>', '+', '-', '[', ']', ',', '.'];

    // Store only the recognized symbols in the vector
    for x in code.chars(){
        if bf_syntax.contains(&x){
            syms.push(x);
        }
    }

    // Return the vector of symbols
    syms
}