use std::env;
//use std::path::Path;
use std::fs::File;

fn main ()
{
    println!("Hello, bytecode assembler.");

    //commandline args
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    let infilepath = &args[1];
    let infile = File::open(infilepath).unwrap();

}