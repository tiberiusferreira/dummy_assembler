const HEADER: &'static str  = "DEPTH = 128;
WIDTH = 16;
ADDRESS_RADIX = HEX;
DATA_RADIX = BIN;
CONTENT
BEGIN
";

const FOOTER: &'static str  = "END
";
fn main() {
    println!("{}", HEADER);

    println!("{}", FOOTER);
}
