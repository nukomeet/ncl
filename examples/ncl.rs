extern crate ncl;

use ncl::parse;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut args = env::args();

    if args.len() < 2 {
        panic!("Not enough arguments")
    }

    let mut file = File::open(args.nth(1).unwrap()).unwrap();
    let mut data = vec![];

    file.read_to_end(&mut data).unwrap();

    let root = parse(data);

    println!("{:#?}", root.unwrap());
}
