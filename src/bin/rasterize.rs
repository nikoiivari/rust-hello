extern crate obj;
extern crate png;

use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter; //writing .png
use obj::{load_obj, Obj, ObjError};

struct Square {
    x: f32,
    y: f32,
    size: f32,
}

//aargh! https://rcoh.me/posts/rust-linked-list-basically-impossible/

// toilet_scaled.obj is from:
// https://opengameart.org/content/modular-bathroom-voxel-art

fn main () -> Result<(), ObjError> {
    let input = BufReader::new(File::open("toilet_scaled.obj")?);
    let mesh: Obj = load_obj(input)?;
    let mut pixels: [u32; 65536] = [0; 65536]; // 256 x 256 = 65536 pixels
    
    let sq = Square {
        x: -1.0,
        y: -1.0,
        size: 2.0,
    };
    let nverts = verts_in_square(&sq, &mesh);
    println!("nverts = {}", nverts);

    //_rasterize(sq, &mesh, &mut pixels, 9);

    for y in 0..=255 {
        for x  in 0..=255 {
            pixel_sample_points(x, y, &mesh, &mut pixels);
        }
    }

    write_png(&mut pixels);

    Ok(())
}

fn pixel_sample_points (x: u8, y: u8, mesh: &Obj, pixels: &mut [u32]) {
    let xf = ((x as f32) - 128.0) / 256.0;
    let yf = ((y as f32) - 128.0) / 256.0;

    let mut z1st: f32 = -2.01;
    //let mut z2nd: f32 = -2.0;
    let z1stcolor: u32 = 0xffffffff;
    //let z2ndcolor: u32 = 0xffffffff;

    for vert in &mesh.vertices {
        if (xf - vert.position[0]).abs() < 0.02 && 
           (yf - vert.position[1]).abs() < 0.02 {
            if z1st < vert.position[2] {
                //z2nd = z1st;
                z1st = vert.position[2];
                //TODO: Get vertex color
            }
        }   
    }

    if z1st > -1.0 {
        pixels[(256 * (y as usize)) + (x as usize)] = z1stcolor;
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

fn verts_in_square (sq: &Square, mesh: &Obj) -> u16 {
    let mut count: u16 = 0;
    
    for vert in &mesh.vertices{
        if  sq.x + sq.size >= vert.position[0] &&
            sq.x <= vert.position[0] &&
            sq.y + sq.size >= vert.position[1] &&
            sq.y <= vert.position[1]
        {
            count = count+1;
        };
    }
    return count;
}
