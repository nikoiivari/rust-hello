extern crate ply_rs;
extern crate png;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use std::io::BufReader;
use std::vec::Vec;
use ply_rs::ply;
use ply_rs::parser;

#[derive(Debug)]
struct Vertex {
    x: f32,
    y: f32,
    z: f32,
    nx: f32,
    ny: f32,
    nz: f32,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl ply::PropertyAccess for Vertex {
    fn new() -> Self {
        Vertex {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            nx: 0.0,
            ny: 0.0,
            nz: 1.0,
            r: 0x0,
            g: 0x0,
            b: 0x0,
            a: 0xff,
        }
    }
    fn set_property(&mut self, key: String, property: ply::Property) {
        match (key.as_ref(), property) {
            ("x", ply::Property::Float(v)) => self.x = v,
            ("y", ply::Property::Float(v)) => self.y = v,
            ("z", ply::Property::Float(v)) => self.z = v,
            ("nx", ply::Property::Float(v))=> self.nx = v,
            ("ny", ply::Property::Float(v))=> self.ny = v,
            ("nz", ply::Property::Float(v))=> self.nz = v,
            ("red", ply::Property::UChar(v)) => self.r = v,
            ("green", ply::Property::UChar(v)) => self.g = v,
            ("blue", ply::Property::UChar(v)) => self.b = v,
            ("alpha", ply::Property::UChar(v)) => self.a = v,
            (k, _) => panic!("Vertex: Unexpected key/value combination: key: {}", k),
        }
    }
}


fn main () {
    
    //use ply_rs
    let path = "VertexColorsTest.ply";
    let plyfile = std::fs::File::open(path).unwrap();
    let mut plyfile = BufReader::new(plyfile);

    //parser
    let vertexparser = parser::Parser::<Vertex>::new();
    let header = vertexparser.read_header(&mut plyfile).unwrap();
    let mut vertices = Vec::new();
    for (_ignore_key, element) in &header.elements {
        match element.name.as_ref() {
            "vertex" => {
                vertices = vertexparser.read_payload_for_element(
                    &mut plyfile, &element, &header
                ).unwrap();
            },
            "face" => {},
            _ => panic!("Unexpeced element!"),
        }
    }   

    println!("vertices: {:#?}", vertices);
    
    let mut pixels: [u32; 65536] = [0; 65536]; // 256 x 256 = 65536 pixels

    for y in 0..=255 {
        for x  in 0..=255 {
            pixel_sample_ply(x, y, 0.05, &vertices, &mut pixels);
        }
    }

    write_png(&mut pixels);

}

fn pixel_sample_ply (x: u8, y: u8, psize: f32, verts: &[Vertex], pixels: &mut [u32]) {
    let xf = ((x as f32) / 128.0) - 1.0;
    let yf = ((y as f32) / 128.0) - 1.0;

    let mut z1st: f32 = -2.01;
    let mut z1stcolor: u32 = 0x000000ff;
    let mut z2ndcolor: u32 = 0xffffffff;

    for vert in verts {
        if (xf - vert.x).abs() < psize && 
           (yf - vert.y).abs() < psize {
            if z1st < vert.z {
                z1st = vert.z;
                z2ndcolor = z1stcolor;
                z1stcolor  = ((vert.r as u32) << 24) +
                             ((vert.g as u32) << 16) + 
                             ((vert.b as u32) << 8) + 0xff;
            }
        }   
    }

    if z1st > -1.0 {
        // Average z1stcolor and z2ndcolor
        let r1: u32 = z1stcolor >> 24;
        let r2: u32 = z2ndcolor >> 24;
        let red: u32 = (r1 + r2) >> 1;
        let g1: u32 = (z1stcolor >> 16) & 0x000000ff;
        let g2: u32 = (z2ndcolor >> 16) & 0x000000ff;
        let green: u32 = (g1 + g2) >> 1;
        let b1: u32 = (z1stcolor >> 8) & 0x000000ff;
        let b2: u32 = (z2ndcolor >> 8) & 0x000000ff;
        let blue: u32 = (b1 + b2) >> 1;
        let zfinalcolor = (red << 24) + (green << 16) + (blue << 8) + 0xff;
        pixels[(256*256) - (256 * (y as usize)) + (x as usize)] = zfinalcolor;
        
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
