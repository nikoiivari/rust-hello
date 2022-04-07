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


fn main () -> Result<(), ObjError> {
    let input = BufReader::new(File::open("objfile.obj")?);
    let mesh: Obj = load_obj(input)?;
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