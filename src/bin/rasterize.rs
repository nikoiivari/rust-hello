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

    rasterize(sq, &mesh, &mut pixels, 8);

    //convert to byte array    
    let mut bytes = Vec::<u8>::new();
    for val in &pixels{
        bytes.extend_from_slice(&val.to_be_bytes());
    }

    Ok(())
}

fn rasterize (sq: Square, mesh: &Obj, mut pixels: &mut [u32], depth: u8) {
    let nverts = verts_in_square(&sq, &mesh);
    
    if 0 == depth {return};

    if 0 == nverts {return};
    
    println!("depth = {}, nverts = {}", depth, nverts);
    
    let muh_red: u8 = 0xFF >> depth;
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
    pixels[0] = color;
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
