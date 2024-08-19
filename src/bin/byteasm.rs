use std::env;
//use std::path::Path;
use std::fs::File;

fn nand(ina:u64, inb:u64) -> u64
{
    return !(ina & inb);
}

fn test_nand_adder()
{

}

fn main ()
{
    let rval:u64 = nand(0b1111, 0b1111);
    println!("Hello, bytecode assembler. rval = {rval:#b}");

    //commandline args
    let args: Vec<String> = env::args().collect();
    if 1 < args.len()
    {
        //println!("{:?}", args);
        let infilepath = &args[1];
        //let infile = File::open(infilepath).unwrap();
    }

}