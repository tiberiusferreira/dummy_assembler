use std::io;
#[macro_use] extern crate failure;
use failure::Error;
use std::mem;
use std::collections::HashMap;
use std::fmt;

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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct Label{
    name: String,
    address: u8
}

#[derive(Debug, Clone)]
enum ParsedLine{
    Instruction(Instruction),
    Label(String),
    ProcessedLabel(Label),
    Word(u16),
    Comment
}

#[derive(Debug, Clone)]
enum Instruction{
    MV(Register, Register),
    MVI(Register, u16),
    MVILabel(Register, String),
    Add(Register, Register),
    Sub(Register, Register),
    LD(Register, Register),
    ST(Register, Register),
    MVNZ(Register, Register),
}

fn print_reg(reg: Register) -> String{
    let reg_as_u8 = reg as u8;

    return format!("{:03b}", reg_as_u8);
}

fn print_inst(inst: Instruction, current_line: u16) -> (String, u16){
    let current_line_u8 = current_line as u8;
    let mut output_current_line;
    let result = match inst {
        Instruction::MV(reg0, reg1) => {
            output_current_line = current_line + 1;
            format!("{:02X} : 000{}{}0000000;", current_line_u8,  print_reg(reg0), print_reg(reg1))
        },
        Instruction::MVI(reg, literal) => {
            let line1 = format!("{:02X} : 001{}0000000000;", current_line_u8,  print_reg(reg));
            let line2 = format!("{:02X} : {:016b};", current_line_u8+1, literal);
            output_current_line = current_line + 2;
            format!("{}\n{}", line1, line2)
        },
        Instruction::MVILabel(_, _) => {
            output_current_line = current_line;
            "".to_string()
        },
        Instruction::Add(reg0, reg1) => {
            output_current_line = current_line + 1;
            format!("{:02X} : 010{}{}0000000;", current_line_u8,  print_reg(reg0), print_reg(reg1))
        },
        Instruction::Sub(reg0, reg1) => {
            output_current_line = current_line + 1;
            format!("{:02X} : 011{}{}0000000;", current_line_u8,  print_reg(reg0), print_reg(reg1))
        },
        Instruction::LD(reg0, reg1) => {
            output_current_line = current_line + 1;
            format!("{:02X} : 100{}{}0000000;", current_line_u8,  print_reg(reg0), print_reg(reg1))
        },
        Instruction::ST(reg0, reg1) => {
            output_current_line = current_line + 1;
            format!("{:02X} : 101{}{}0000000;", current_line_u8,  print_reg(reg0), print_reg(reg1))
        },
        Instruction::MVNZ(reg0, reg1) => {
            output_current_line = current_line + 1;
            format!("{:02X} : 110{}{}0000000;", current_line_u8,  print_reg(reg0), print_reg(reg1))
        },
    };
    (result, output_current_line)
}

fn get_instruction_output_size(inst: Instruction) -> u16{
    return match inst {
        Instruction::MV(_, _) => {1},
        Instruction::MVI(_, _) => {2},
        Instruction::MVILabel(_, _) => {2},
        Instruction::Add(_, _) => {1},
        Instruction::Sub(_, _) => {1},
        Instruction::LD(_, _) => {1},
        Instruction::ST(_, _) => {1},
        Instruction::MVNZ(_, _) => {1},
    }
}
fn main() {
    println!("{}", HEADER);
    let parsed_lines_original = parse_lines();
    let mut parsed_lines_copy = parsed_lines_original.clone();
    let mut labels: HashMap<String, u16> = HashMap::new();
    let mut output_current_line: u16 = 0;
    for (i, line) in parsed_lines_original.iter().enumerate(){
        match line {
            ParsedLine::Instruction(inst) => {
                output_current_line = output_current_line + get_instruction_output_size(inst.clone());
            },
            ParsedLine::Label(name) => {
                labels.insert(name.to_string(),output_current_line);
            },
            ParsedLine::Word(_) =>{
                output_current_line = output_current_line + 1;
            },
            _ => {}
        }
    }

    for (i, line) in parsed_lines_original.iter().enumerate(){
        match line {
            ParsedLine::Instruction(Instruction::MVILabel(reg, label)) => {
                let label_addr = match labels.get(label){
                    Some(addr) => addr,
                    None => {
                        panic!("No address for label: {}", label);
                    }
                };
                parsed_lines_copy[i] = ParsedLine::Instruction(Instruction::MVI(reg.clone(), *label_addr))
            },
            _ => {}
        }
    }



    let mut current_line = 0;

    for line in parsed_lines_copy{
//        println!("{:?}", line);
        match line {
            ParsedLine::Instruction(inst) => {
                let (formated, new_line) = print_inst(inst, current_line);
                current_line = new_line;
                println!("{}", formated);
            },
            ParsedLine::Word(word) =>{
                println!("{:02X} : {:016b};", current_line, word);
                current_line = current_line + 1;
            },
            _ => {}
        }
    }





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

fn create_mvi_instruction(reg: Option<&&str>, literal_or_label: Option<&&str>) -> Instruction{
    let reg = parse_register(reg);
    match literal_or_label{
        Some(value_of_literal_or_label) => {
            if !value_of_literal_or_label.starts_with("#"){
                panic!("Literals and labels should begin with #");
            }
            let value_of_literal_or_label = value_of_literal_or_label.replace("#", "");
            let maybe_digit = value_of_literal_or_label.parse::<u16>();
            if maybe_digit.is_ok(){
                return Instruction::MVI(reg, maybe_digit.unwrap());
            }else {
                return Instruction::MVILabel(reg, value_of_literal_or_label);
            }
        },
        None => panic!("Expecting literal or label as argument of MVI, got nothing")
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
        Some(&"mvi") => {
            let arg0 = words.next();
            let arg1 = words.next();
            ParsedLine::Instruction(create_mvi_instruction(arg0, arg1))
        },
        Some(&"add") => {
            let arg0 = words.next();
            let arg1 = words.next();
            let reg0 = parse_register(arg0);
            let reg1 = parse_register(arg1);
            ParsedLine::Instruction(Instruction::Add(reg0, reg1))
        },
        Some(&"sub") => {
            let arg0 = words.next();
            let arg1 = words.next();
            let reg0 = parse_register(arg0);
            let reg1 = parse_register(arg1);
            ParsedLine::Instruction(Instruction::Sub(reg0, reg1))
        },
        Some(&"ld") => {
            let arg0 = words.next();
            let arg1 = words.next();
            let reg0 = parse_register(arg0);
            let reg1 = parse_register(arg1);
            ParsedLine::Instruction(Instruction::LD(reg0, reg1))
        },
        Some(&"st") => {
            let arg0 = words.next();
            let arg1 = words.next();
            let reg0 = parse_register(arg0);
            let reg1 = parse_register(arg1);
            ParsedLine::Instruction(Instruction::ST(reg0, reg1))
        },
        Some(&"mvnz") => {
            let arg0 = words.next();
            let arg1 = words.next();
            let reg0 = parse_register(arg0);
            let reg1 = parse_register(arg1);
            ParsedLine::Instruction(Instruction::MVNZ(reg0, reg1))
        },
        Some(&".word") => {
            let arg0 = words.next().expect("Expected literal after .word");
            if !arg0.starts_with("#"){
                panic!("Word literals should begin with #");
            }
            let arg0 = arg0.replace("#", "");
            let maybe_digit = arg0.parse::<u16>();
            if maybe_digit.is_ok(){
                return ParsedLine::Word(maybe_digit.unwrap());
            }else {
                panic!("Expected literal after word.");
            }
        },
        Some(word) => {
            if word.ends_with(":"){
                let cleaned_word = word.replace(":", "");
                ParsedLine::Label(cleaned_word.to_string())
            }else{
                if word.starts_with("%"){
                    ParsedLine::Comment
                }else {
                    panic!("Line does not begin with either instruction, Label or comment");
                }
            }
        },
        None => panic!("Empty lines are not allowed"),
    }
}

fn parse_lines() -> Vec<ParsedLine>{
    let mut new_line = String::new();
    let mut parsed_lines = Vec::new();
    while io::stdin().read_line(&mut new_line).unwrap() != 0{
        let parsed_line = parse_line(new_line.clone());
        parsed_lines.push(parsed_line);
        new_line.clear();
    }
    parsed_lines
}


