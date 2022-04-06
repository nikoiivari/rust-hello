use std::fs::File;
use std::io::BufReader;
use obj::{load_obj, Obj};

let input = BufReader::new(File::open("objfile.obj")?);
let mesh: Obj = load_obj(input)?;

mesh.vertices;
