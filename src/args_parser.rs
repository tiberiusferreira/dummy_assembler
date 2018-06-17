extern crate clap;
use self::clap::{Arg, App};

pub struct Args{
    pub input_file: String,
    pub output_file: String,
}
pub fn parse_program_args() -> Args{
    const INPUT_ASSEMBLY_FILE: &'static str  = "ASSEMBLY_FILE";
    const OUTPUT_FILE: &'static str  = "OUTPUT_FILE";

    let matches = App::new("Simple Parser for ES575 VHDL Dummy Processor")
        .version("1.0")
        .author("Tiberio FERREIRA <tiberiusferreira@gmail.com>")
        .about("Parses an assembly file and outputs a memory file initialized with the binary code")
        .arg(Arg::with_name(INPUT_ASSEMBLY_FILE)
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name(OUTPUT_FILE)
            .help("Sets the output file name to use")
            .required(true)
            .index(2))
        .get_matches();

    let input_file = matches.value_of(INPUT_ASSEMBLY_FILE).unwrap().to_string();
    let output_file = matches.value_of(OUTPUT_FILE).unwrap().to_string();

    Args{
        input_file,
        output_file,
    }
}