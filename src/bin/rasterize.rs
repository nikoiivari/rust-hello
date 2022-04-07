use std::fs::File;
use std::io::BufReader;
use obj::{load_obj, Obj};

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
    minx: f32,
}

let input = BufReader::new(File::open("objfile.obj")?);
let mesh: Obj = load_obj(input)?;


fn main () {
    
}

fn verts_in_square (sq: Square) -> u16 {
    let count: u16 = 0;
    
    for vert in mesh.vertices{

    }
}

fn default_rgba () -> RGBA {
    RGBA {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
}