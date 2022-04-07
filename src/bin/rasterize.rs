extern crate obj;

use std::fs::File;
use std::io::BufReader;
use obj::{load_obj, Obj, ObjError};

struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

struct Square {
    minx: f32,
    maxx: f32,
    miny: f32,
    maxy: f32,
}

// toilet_scaled.obj is from:
// https://opengameart.org/content/modular-bathroom-voxel-art

fn main () -> Result<(), ObjError> {
    let input = BufReader::new(File::open("toilet_scaled.obj")?);
    let mesh: Obj = load_obj(input)?;

    let sq = Square {
        minx: -1.0,
        maxx:  1.0,
        miny: -1.0,
        maxy:  1.0,
    };
    let nverts = verts_in_square(sq, mesh);
    println!("nverts = {}", nverts);

    let muh_rgba = default_rgba();
    println!("rgba = {}, {}, {}, {}", muh_rgba.r, muh_rgba.g, muh_rgba.b, muh_rgba.a);
    Ok(())
}

fn verts_in_square (sq: Square, mesh: Obj) -> u16 {
    let count: u16 = 0;
    
    for vert in mesh.vertices{

    }
    return count;
}

fn default_rgba () -> RGBA {
    RGBA {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    }
}