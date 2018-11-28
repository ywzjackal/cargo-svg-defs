extern crate cargo_svg_defs;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 4 {
        cargo_svg_defs::parse_dir(&args[2], &args[3]).unwrap();
        return;
    }
    if args.len() == 3 {
        cargo_svg_defs::parse_dir(&args[1], &args[2]).unwrap();
        return;
    }
    println!("USAGE: cargo svg-defs dir_path out_path");
}