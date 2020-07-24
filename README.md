# BrainRust
BrainRust is a Brainfuck to C compiler written in Rust.

# Why?
Nah, it's just hobby project I made while I was learning Rust.

# How Does It Work
`cmd_parser` parse the command line arguments then it's passed to `main`. `bf_parser` parse the Brainfuck syntax and filters it out. `compiler` transformes the Brainfuck into C code according to this table:

# Examples
`examples` folder contains a bunch of Brainfuck examples you can run.

# Usage
To compile the Hello World example run:
```
$ cargo run examples/hello-world.bf -o examples/hello-world
$ ./examples/hello-world
```