use std::io;
#[macro_use] extern crate failure;
use failure::Error;

const HEADER: &'static str  = "DEPTH = 128;
WIDTH = 16;
ADDRESS_RADIX = HEX;
DATA_RADIX = BIN;
CONTENT
BEGIN
";

const FOOTER: &'static str  = "END
";

#[derive(Debug, Fail)]
enum AssemblerError {
    #[fail(display = "Invalid character")]
    ParsingError,
    #[fail(display = "Error reading input")]
    IOError,
}

#[derive(Debug)]
enum Register{
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
}

#[derive(Debug)]
struct Label{
    name: String,
    address: u8
}

#[derive(Debug)]
enum ParsedLine{
    Instruction(Instruction),
    Label(String),
    Comment
}

#[derive(Debug)]
enum Instruction{
    MV(Register, Register),
    MVI(Register, u8),
    MVILabel(Register, String),
    Add(Register, Register),
    Sub(Register, Register),
    LD(Register, Register),
    ST(Register, Register),
}

fn main() {
    println!("{}", HEADER);
    parse_lines().unwrap();


    println!("{}", FOOTER);

}

fn parse_register(reg: Option<&&str>) -> Register{
    match reg {
        Some(reg_as_str) => {
            let cleaned_reg_as_str = reg_as_str.replace(",", "");
            match cleaned_reg_as_str.as_str() {
                "R0" => Register::R0,
                "R1" => Register::R1,
                "R2" => Register::R2,
                "R3" => Register::R3,
                "R4" => Register::R4,
                "R5" => Register::R5,
                "R6" => Register::R6,
                "R7" => Register::R7,
                _ => panic!("Expecting register as R0 or R1 or R2..., got {}", cleaned_reg_as_str)
            }
        },
        None => panic!("Expecting register as argument of MV, got nothing")
    }
}


fn parse_line(line: String) -> ParsedLine{
    let words: Vec<&str> = line.split_whitespace().collect();
    let mut words = words.iter();
    let first_word = words.next();
    return match first_word {
        Some(&"mv") => {
            let arg0 = words.next();
            let arg1 = words.next();
            let reg0 = parse_register(arg0);
            let reg1 = parse_register(arg1);
            ParsedLine::Instruction(Instruction::MV(reg0, reg1))
        },
        None => panic!("Empty lines are not allowed"),
        _ =>  ParsedLine::Comment,
    }
}

fn parse_lines() -> Result<Vec<ParsedLine>, Error>{
    let mut new_line = String::new();
    let mut parsed_lines = Vec::new();
    while io::stdin().read_line(&mut new_line)? != 0{
        let parsed_line = parse_line(new_line.clone());
        println!("{:?}", parsed_line);
        parsed_lines.push(parsed_line);
        new_line.clear();
    }
    println!("You typed: {}", new_line.trim());
    Ok(parsed_lines)
}


