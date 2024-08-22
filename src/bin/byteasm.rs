use std::env;
//use std::path::Path;
//use std::fs::File;

fn nand(ina:u64, inb:u64) -> u64
{
    return !(ina & inb);
}

// add ina to inb using only nand gate
fn test_4bit_nand_adder(ina:u64, inb:u64) -> (u64, u64)
{
    let mut tmp1:u64; let mut tmp2:u64; let mut tmp3:u64; let mut tmp4:u64;
    let mut tmp5:u64; let mut tmp6:u64; let mut tmp7:u64; let mut tmp8sum:u64;
    let mut tmp9cout:u64; let mut cin:u64 = 0;

    let mut mask:u64; let mut sum:u64 = 0; let mut cout:u64 = 0;
    
    for i in 0..=3 {

        mask = 0b1<<i;
        println!("mask = {mask:#b}");
        
        //let inam = ina & mask;
        //let inbm = inb & mask;
        
        tmp1 = nand(ina, inb);
        println!("tmp1: {tmp1:#b}");

        tmp2 = nand(ina, tmp1);
        println!("tmp2: {tmp2:#b}");
        tmp3 = nand(inb, tmp1);
        println!("tmp3: {tmp3:#b}");

        tmp4 = nand(tmp2, tmp3);
        println!("tmp4: {tmp4:#b}");
        
        println!("cin:  {cin:#b}");
        tmp5 = nand(tmp4, cin);
        println!("tmp5: {tmp5:#b}");
        
        tmp6 = nand(tmp4, tmp5);
        println!("tmp6: {tmp6:#b}");
        tmp7 = nand(tmp5, cin);
        println!("tmp7: {tmp7:#b}");

        tmp8sum = nand(tmp6, tmp7);
        println!("tmp8sum: {tmp8sum:#b}");
        tmp9cout = nand(tmp5, tmp1);
        println!("tmp9cout: {tmp9cout:#b}");

        sum = tmp8sum;

        cin = tmp9cout; // carry over to next loop
        cout = tmp9cout;
    }

    return (sum, cout);
}

fn main ()
{
    //let rval:u64 = nand(0b1111, 0b1111);
    let rtuple:(u64, u64) = test_4bit_nand_adder(2, 2);
    let rsum:u64;
    let rcarry:u64;
    (rsum, rcarry) = rtuple;
    println!("Sum: {rsum}\nCarry: {rcarry}");

    //commandline args
    let args: Vec<String> = env::args().collect();
    if 1 < args.len()
    {
        //println!("{:?}", args);
        //let infilepath = &args[1];
        //let infile = File::open(infilepath).unwrap();
    }

}