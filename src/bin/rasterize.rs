#![allow(dead_code)]
extern crate ply_rs;
extern crate png;

use std::f32::consts::PI;
use std::env;
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

impl Vertex {
    fn new_with_xyz(x: f32, y: f32, z: f32) -> Self {
        Vertex {
            x: x,
            y: y,
            z: z,
            nx: 0.0,
            ny: 0.0,
            nz: 1.0,
            r: 0x0,
            g: 0x0,
            b: 0x0,
            a: 0xff,
        }
    }
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
            (k, _) => panic!("Vertex: Unexpected key/value: key: {}", k),
        }
    }
}

struct BiVec3 {
    xy: f32,
    xz: f32,
    yz: f32,
}

impl BiVec3 {
    fn new(xy: f32, xz: f32, yz: f32) -> Self {
        BiVec3 {
            xy: xy,
            xz: xz,
            yz: yz,
        }
    }
}

struct Rotor3 {
    s: f32,
    xy: f32,
    xz: f32,
    yz: f32,    
}

impl Rotor3 {
    /*
    fn new() -> Self {
        Rotor3 {
            s: 0.0,
            xy: 0.0,
            xz: 0.0,
            yz: 0.0,
        }
    }
    */
    fn new_from_vert_to_vert(a: Vertex, b: Vertex) -> Self {
        let bv: BiVec3 = outer3(&a, &b);
        let mut r =  Rotor3 {
            s: 1.0 + dot3(&a, &b),
            xy: bv.xy,
            xz: bv.xz,
            yz: bv.yz,
        };
        r.normalize();
        return r
    }
    fn new_from_angle_and_plane(plane: BiVec3, angle_rad: f32) -> Self {
        Rotor3 {
            s: (angle_rad / 2.0).cos(),
            xy: -(angle_rad / 2.0).sin() * plane.xy,
            xz: -(angle_rad / 2.0).sin() * plane.xz,
            yz: -(angle_rad / 2.0).sin() * plane.yz,
        }
    }
    fn normalize(&mut self) {
        let lsqr: f32 = self.s * self.s + 
                        self.xy * self.xy + 
                        self.xz * self.xz + 
                        self.yz * self.yz;
        let length: f32 = lsqr.sqrt();
        self.s /= length;
        self.xy /= length; self.xz /= length; self.yz /= length;
    }
    fn multiply(self, m: Rotor3) -> Self {
        Rotor3 {
            s: self.s * m.s - self.xy * m.xy - self.xz * m.xz - self.yz * m.yz,
            xy: self.xy * m.s + self.s * m.xy + self.yz * m.xz - self.xz * m.yz,
            xz: self.xz * m.s + self.s * m.xz - self.yz * m.xy + self.xy * m.yz,
            yz: self.yz * m.s + self.s * m.yz + self.xz * m.xy - self.xy * m.xz,
        }
    }
    fn rotate(self, v: &Vertex) -> Vertex {
        let tmp_x: f32 = self.s * v.x + v.y * self.xy + v.z * self.xz;
        let tmp_y: f32 = self.s * v.y - v.x * self.xy + v.z * self.yz;
        let tmp_z: f32 = self.s * v.z - v.x * self.xz - v.y * self.yz;
        let tri: f32 = v.x * self.yz - v.y * self.xz + v.z * self.xy;
        let r_x: f32 = self.s * tmp_x + tmp_y * self.xy + tmp_z * self.xz + tri   * self.yz;
        let r_y: f32 = self.s * tmp_y - tmp_x * self.xy - tri   * self.xz + tmp_z * self.yz;
        let r_z: f32 = self.s * tmp_z + tri   * self.xy - tmp_x * self.xz - tmp_y * self.yz;
        let mut vert: Vertex = Vertex::new_with_xyz(r_x, r_y, r_z);
        vert.nx = v.nx; vert.ny = v.ny; vert.nz = v.nz;
        vert.r = v.r; vert.g = v.g; vert.b = v.b; vert.a = v.a;
        return vert
    }
}

fn outer3 (a: &Vertex, b: &Vertex) -> BiVec3 {
    let mut c =  BiVec3::new(0.0, 0.0, 0.0);
    c.xy = a.x * b.y - a.y * b.x;
    c.xz = a.x * b.z - a.z * b.x;
    c.yz = a.y * b.z - a.z * b.y;

    return c
}

fn dot3 (a: &Vertex, b: &Vertex) -> f32 {
    return a.x*b.x + a.y*b.y + a.z*b.z
}

fn main () {
    
    //commandline args
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    let foldername = &args[1];
    let angle_s = &args[2];
    let angle = angle_s.parse::<f32>().unwrap();
    let directions_s = &args[3];
    let dir_offset_s = &args[4];
    let frame_name_s = &args[5];
    let directions = directions_s.parse::<i32>().unwrap();
    let dir_offset = dir_offset_s.parse::<i32>().unwrap();
    
    //use ply_rs
    let path = foldername.to_owned() + "/" + frame_name_s + ".ply";
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

        
    for i in 0..directions {
        // prepare angles
        let dir_angle = (i * (360/directions))+dir_offset;
        // rotate
        let mut rotated_vertices = Vec::new();
        for vert in &vertices {
            let plane1: BiVec3 = BiVec3::new(0.0, 1.0, 0.0);
            let rotor1 = Rotor3::new_from_angle_and_plane(plane1, (dir_angle as f32) * (PI/180.0f32));
            let plane2: BiVec3 = BiVec3::new(0.0, 0.0, 1.0);
            let rotor2 = Rotor3::new_from_angle_and_plane(plane2, angle * (PI/180.0f32));
            //rotate rotor with rotor
            let rotor3: Rotor3 = rotor2.multiply(rotor1);
            let rotated_vert: Vertex = rotor3.rotate(&vert);
            rotated_vertices.push( rotated_vert );
        }


        let mut pixels: [u32; 65536] = [0; 65536]; // 256 x 256 = 65536 pixels

        for y in 0..=255 {
            for x  in 0..=255 {
                pixel_sample_ply(x, y, 0.025, &rotated_vertices, &mut pixels);
            }
        }

        // write png with frame name and direction angle
        let outpath = foldername.to_owned() + "/" + frame_name_s + &dir_angle.to_string() + ".png";
        println!("direction {:?}", outpath);
        write_png(&mut pixels, outpath);
    }
}

fn pixel_sample_ply (x: u8, y: u8, psize: f32, verts: &[Vertex],
                     pixels: &mut [u32]) {
    let xf = ((x as f32) / 128.0) - 1.0;
    let yf = ((y as f32) / 128.0) - 1.0;

    let mut z1st: f32 = -2.01;
    let mut z1stcolor: u32 = 0x000000ff;
    
    for vert in verts {
        if (xf - vert.x).abs() < psize && 
           (yf - vert.y).abs() < psize {
            if z1st < vert.z {
                z1st = vert.z;
                z1stcolor  = ((vert.r as u32) << 24) +
                             ((vert.g as u32) << 16) + 
                             ((vert.b as u32) << 8) + 0xff;
            }
        }   
    }

    if z1st > -1.0 {
                
        pixels[(256*256) - (256 * (y as usize)) + (x as usize)-1] = z1stcolor;
        
    }
}

fn write_png (pixels: &mut[u32], path: String) {
    //convert to byte array    
    let mut bytes = Vec::<u8>::new();
    for val in pixels{
        bytes.extend_from_slice(&val.to_be_bytes());
    }

    //let path = Path::new(r"raster.png");
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
