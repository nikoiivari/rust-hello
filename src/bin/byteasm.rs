use std::env;
//use std::path::Path;
//use std::fs::File;

fn nand(ina:u64, inb:u64) -> u64
{
    return !(ina & inb);
}

fn test_2bit_nand_adder(ina:u64, inb:u64) -> (u64, u64)
{
    // Split bits to add them separately using one bit adder.
    let inabit0:u64 = ina & 0b1;
    let inabit1bs:u64 = ina & 0b10;
    let inabit1:u64 = inabit1bs >>1;
    println!("inabit0: {inabit0:#b}");
    println!("inabit1: {inabit1:#b}");
    
    let inbbit0:u64 = inb & 0b1;
    let inbbit1bs:u64 = inb & 0b10;
    let inbbit1:u64 = inbbit1bs >>1;
    println!("inbbit0: {inbbit0:#b}");
    println!("inbbit1: {inbbit1:#b}");

    let mut cin:u64 = 0;
    let mut sum:u64; 
    let cout:u64;

        {
        // 1-bit adder begins
        let tmp1 = nand(inabit0, inbbit0);
        println!("tmp1: {tmp1:#b}");

        let tmp2 = nand(inabit0, tmp1);
        println!("tmp2: {tmp2:#b}");
        let tmp3 = nand(inbbit0, tmp1);
        println!("tmp3: {tmp3:#b}");

        let tmp4 = nand(tmp2, tmp3);
        println!("tmp4: {tmp4:#b}");
        
        println!("cin:  {cin:#b}");
        let tmp5 = nand(tmp4, cin);
        println!("tmp5: {tmp5:#b}");
        
        let tmp6 = nand(tmp4, tmp5);
        println!("tmp6: {tmp6:#b}");
        let tmp7 = nand(tmp5, cin);
        println!("tmp7: {tmp7:#b}");

        let tmp8sum = nand(tmp6, tmp7);
        println!("tmp8sum: {tmp8sum:#b}");
        let tmp9cout = nand(tmp5, tmp1);
        println!("tmp9cout: {tmp9cout:#b}");
        // 1-bit adder ends

        sum = tmp8sum;
        cin = tmp9cout;
        }

        {
        // 1-bit adder begins
        let tmp1 = nand(inabit1, inbbit1);
        println!("tmp1: {tmp1:#b}");

        let tmp2 = nand(inabit1, tmp1);
        println!("tmp2: {tmp2:#b}");
        let tmp3 = nand(inbbit1, tmp1);
        println!("tmp3: {tmp3:#b}");

        let tmp4 = nand(tmp2, tmp3);
        println!("tmp4: {tmp4:#b}");
        
        println!("cin:  {cin:#b}");
        let tmp5 = nand(tmp4, cin);
        println!("tmp5: {tmp5:#b}");
        
        let tmp6 = nand(tmp4, tmp5);
        println!("tmp6: {tmp6:#b}");
        let tmp7 = nand(tmp5, cin);
        println!("tmp7: {tmp7:#b}");

        let tmp8sum = nand(tmp6, tmp7);
        println!("tmp8sum: {tmp8sum:#b}");
        let tmp9cout = nand(tmp5, tmp1);
        println!("tmp9cout: {tmp9cout:#b}");
        // 1-bit adder ends

        //combine bits
        sum = sum | (tmp8sum<<1); //OR-gate can be implemented with NAND-gates
        cout = tmp9cout;
        }
    return (sum, cout);
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
        //let infilepath = &args[1];
        //let infile = File::open(infilepath).unwrap();
    }

}