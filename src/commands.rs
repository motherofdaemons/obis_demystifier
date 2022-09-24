use crate::args::{Commands, ObisArgs};

fn convert_hex(mut args: ObisArgs) {
    args.code.convert_to_hex();
    println!("{}", args.code);
}

fn convert_dec(mut args: ObisArgs) {
    args.code.convert_to_dec();
    println!("{}", args.code);
}

pub fn run(command: Commands) {
    match command {
        Commands::Hex(args) => convert_hex(args),
        Commands::Dec(args) => convert_dec(args),
        _ => todo!(),
    }
}