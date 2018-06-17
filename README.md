# dummy_assembler
Dummy assembler for a processor developed during the ES575 course  

## How to use

### 1 - Install Rust: https://www.rust-lang.org/en-US/install.html

### 2 - type cargo build on project directory

### 3 - type cargo run -- sample_program.txt output.txt

#### Caveats:

The assembler uses radix 10 literals and expects inputs in the exact form of "sample_program.txt".
That means:

One Instruction / Word or Label per line.

Registers separated by "," and with a space in between: add	R2, R1

Word literal inline with the .word keyword.
