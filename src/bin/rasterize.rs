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
    x: f32,
    y: f32,
    size: f32,
}

// toilet_scaled.obj is from:
// https://opengameart.org/content/modular-bathroom-voxel-art

fn main () -> Result<(), ObjError> {
    let input = BufReader::new(File::open("toilet_scaled.obj")?);
    let mesh: Obj = load_obj(input)?;

    let sq = Square {
        x: -1.0,
        y: -1.0,
        size: 2.0,
    };
    let nverts = verts_in_square(&sq, &mesh);
    println!("nverts = {}", nverts);

    rasterize(sq, &mesh, 8);
    
    let muh_rgba = default_rgba();
    println!("rgba = {}, {}, {}, {}", muh_rgba.r, muh_rgba.g, muh_rgba.b, muh_rgba.a);
    Ok(())
}

fn rasterize (sq: Square, mesh: &Obj, depth: u16) {
    let nverts = verts_in_square(&sq, &mesh);
    println!("depth = {}, nverts = {}", depth, nverts);

    if 0 == depth {return};

    if 0 == nverts {return};
    
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

    rasterize (sub_tl, &mesh, depth - 1);
    rasterize (sub_tr, &mesh, depth - 1);
    rasterize (sub_bl, &mesh, depth - 1);
    rasterize (sub_br, &mesh, depth - 1);

    return;
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

fn default_rgba () -> RGBA {
    RGBA {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    }
}