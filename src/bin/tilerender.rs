#![allow(dead_code)]
extern crate png;
use std::fs::File;
use std::io::BufWriter;
//use std::io::BufReader;
use std::path::Path;
 
fn main() {
    //let slso: [u32; 8] = [0x0d2b45, 0x203c56, 0x544e68, 0x8d697a, 
    //                      0xd08159, 0xffaa5e, 0xffd4a3, 0xffecd6];
    let mut pixels: [u32; 65536] = [0; 65536]; // 256 x 256 = 65536 pixels
    //let mut in_pixels: [u32; 65536] = [0; 65536];
    let path = Path::new(r"traced.png");
    write_png (&mut pixels, path)

}

// fn map_to_palette (pixels: &mut [u32], pal: [u32], num_colors: u32, in_pixels: &mut [u32]) {

//}

//fn fill_tile_gradient (pixels: &mut [u32]) {
    // fill tile with horisontal gradient
    //println!("range {:?}", (256/16);
    // tileline
    //for tilex: u8 in 0..15{
    //    for j: u8 in 0..15{
    //        pixels[(tilex*16)+j] = 0xa + j*4;
    //    }
    //}
//}



fn write_png (pixels: &mut[u32], path: &Path) {
    //convert to byte array    
    let mut bytes = Vec::<u8>::new();
    for val in pixels{
        bytes.extend_from_slice(&val.to_be_bytes());
    }

    //
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, 256, 256);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_trns(vec!(0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8));
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455));
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));
    let source_chromaticities = png::SourceChromaticities::new(
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000)
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&bytes).unwrap();
}