#![allow(dead_code, unused_imports)]

use std::env;
//use std::path::Path;
use std::fs::File;
use std::io::Read;

fn nand(ina:u64, inb:u64) -> u64
{
    return !(ina & inb);
}

fn main ()
{
    //let rval:u64 = nand(0b1111, 0b1111);
    //println!("nand(0b1111, 0b1111) = {rval:#b}");
    
    //commandline args
    let args: Vec<String> = env::args().collect();
    if 1 < args.len()
    {
        //println!("{:?}", args);
        let infilepath = &args[1];
        let mut infile = File::open(infilepath).unwrap();

        //let reader = BufReader::new(infile);
        let mut s: String = Default::default();
        let _ = infile.read_to_string(&mut s);

        //for line in reader.lines() {
        //    println!("{:?}", line);
        //}
        println!("{}", s);
    }
}

// line -- separate a line to a separate string
fn line (mut s:String) -> (String, String) {
    ("foo".to_string(), "bar".to_string())
}

// code -- separate code from the comment at the end of the line
fn code (mut s:String) -> (String, String) {
    ("foo".to_string(), "bar".to_string())
}