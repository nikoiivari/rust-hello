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
struct QuadSons {
    ptr: Option<Box<QuadSons>>,
    a_verts: u16,
    b_verts: u16,
    c_verts: u16,
    d_verst: u16,
}

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

    rasterize(sq, &mesh, &mut pixels, 9);

    write_png(&mut pixels);

    Ok(())
}

fn quadtree_from_obj(sq: Square, mesh: &Obj,){
//TODO...
}

fn rasterize (sq: Square, mesh: &Obj, mut pixels: &mut [u32], depth: u8) {
    let nverts = verts_in_square(&sq, &mesh);
    
    if 0 == depth {return};

    if 0 == nverts {return};
    
    //println!("depth = {}, nverts = {}", depth, nverts);
    
    let muh_red = 0xFFu8 / depth;
    let muh_rgba: u32 = ((muh_red as u32) << 24) + 0x000000FF;

    draw_square(&sq, muh_rgba, &mut pixels);
    
    let half: f32 =  sq.size / 2.0;
    
    let sub_tl = Square {
        x: sq.x,
        y: sq.y + half,
        size: half,
    };

    let sub_tr = Square {
        x: sq.x + half,
        y: sq.y + half,
        size: half,
    };

    let sub_bl = Square {
        x: sq.x,
        y: sq.y,
        size: half,
    };

    let sub_br = Square {
        x: sq.x + half,
        y: sq.y,
        size: half,
    };

    rasterize (sub_tl, &mesh, &mut pixels, depth - 1);
    rasterize (sub_tr, &mesh, &mut pixels, depth - 1);
    rasterize (sub_bl, &mesh, &mut pixels, depth - 1);
    rasterize (sub_br, &mesh, &mut pixels, depth - 1);

    return;
}

fn draw_square (sq: &Square, color: u32, pixels: &mut[u32]) {
    let x = ((sq.x * 128.0) + 128.0) as u8;
    let y = ((sq.y * 128.0) + 128.0) as u8;
    let siz = (sq.size * 128.0) as u8;
    let size = siz as usize;

    let offset: usize = 256 * y as usize;
    let row_offset: usize = x as usize;
    
    for i in 0..size {
        for j in 0..size {
            pixels[offset + (row_offset * i)+j] = color;
        }
    }

    pixels[1] = 0xffffffff;
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
