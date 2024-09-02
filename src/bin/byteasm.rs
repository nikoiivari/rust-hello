use std::env;
//use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn nand(ina:u64, inb:u64) -> u64
{
    return !(ina & inb);
}


fn main ()
{
    //let rval:u64 = nand(0b1111, 0b1111);
    //println!("nand(0b1111, 0b1111) = {rval:#b}");
    let rtuple:(u64, u64) = test_2bit_nand_adder(1, 2);
    //let rtuple:(u64, u64) = ;
    let rsum:u64;
    let rcarry:u64;
    (rsum, rcarry) = rtuple;
    println!("Sum: {rsum}\nCarry: {rcarry}");

    //commandline args
    let args: Vec<String> = env::args().collect();
    if 1 < args.len()
    {
        println!("{:?}", args);
        let infilepath = &args[1];
        let infile = File::open(infilepath).unwrap();

        let reader = BufReader::new(infile);

        for line in reader.lines() {
            println!("{:?}", line);
        }
    }
}