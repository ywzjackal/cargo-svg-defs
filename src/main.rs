extern crate cargo_svg_defs;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("USAGE: cargo svg-defs dir_path out_path");
        println!("{:?}", args);
        return;
    }
    cargo_svg_defs::parse_dir(&args[1], &args[2]).unwrap();
}