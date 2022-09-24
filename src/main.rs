use std::str::FromStr;

mod obis;
fn main() {
    let obis = obis::Obis::from_str("010203040506").unwrap();
    println!("{}", obis);
}
