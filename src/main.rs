use std::str::FromStr;

mod hex_obis;
fn main() {
    let obis = hex_obis::HexObis::from_str("010203040506").unwrap();
    println!("{}", obis);
}
