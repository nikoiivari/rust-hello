extern crate obj;
extern crate png;

use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use obj::{load_obj, Obj, ObjError};

// Replaced toilet_scaled.obj with a subdivided 3/4ths sphere

fn main () -> Result<(), ObjError> {
    let input = BufReader::new(File::open("3of4sphere.obj")?);
    let mesh: Obj = load_obj(input)?;
    let mut pixels: [u32; 65536] = [0; 65536]; // 256 x 256 = 65536 pixels

    for y in 0..=255 {
        for x  in 0..=255 {
            pixel_sample_points(x, y, 0.05, &mesh, &mut pixels);
        }
    }

    write_png(&mut pixels);

    Ok(())
}

fn pixel_sample_points (x: u8, y: u8, pixelsize: f32, mesh: &Obj, pixels: &mut [u32]) {
    let xf = ((x as f32) / 128.0) - 1.0;
    let yf = ((y as f32) / 128.0) - 1.0;

    let mut z1st: f32 = -2.01;
    //let mut z2nd: f32 = -2.0;
    let z1stcolor: u32 = 0x116666ff;
    //let z2ndcolor: u32 = 0xffffffff;

    for vert in &mesh.vertices {
        if (xf - vert.position[0]).abs() < pixelsize && 
           (yf - vert.position[1]).abs() < pixelsize {
            if z1st < vert.position[2] {
                //z2nd = z1st;
                z1st = vert.position[2];
                //TODO: Get vertex color
            }
        }   
    }

    if z1st > -1.0 {
        pixels[(256*256) - (256 * (y as usize)) + (x as usize)] = z1stcolor;
        //TODO: Average z1stcolor and z2ndcolor
    }
}

fn write_png (pixels: &mut[u32]) {
    //convert to byte array    
    let mut bytes = Vec::<u8>::new();
    for val in pixels{
        bytes.extend_from_slice(&val.to_be_bytes());
    }

    let path = Path::new(r"raster.png");
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
