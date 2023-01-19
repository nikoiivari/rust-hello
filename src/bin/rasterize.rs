#![allow(dead_code)]
extern crate ply_rs;
extern crate png;

use std::u32;
use std::f32;
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

    fn new_with_xyz_nxnynz(x: f32, y: f32, z: f32, nx: f32, ny: f32, nz: f32) -> Self {
        Vertex {
            x: x,
            y: y,
            z: z,
            nx: nx,
            ny: ny,
            nz: nz,
            r: 0x0,
            g: 0x0,
            b: 0x0,
            a: 0xff,
        }
    }

    fn normalize_normal(&mut self) {
        let len: f32  = f32::sqrt(self.nx * self.nx + self.ny * self.ny + self.nz * self.nz);
        self.nx = self.nx * (1.0 / len);
        self.ny = self.ny * (1.0 / len);
        self.nz = self.nz * (1.0 / len);
    }

    fn shade(&mut self, light: Vertex, diffuse: u32, ambient: u32) {
        self.normalize_normal();
        let mut d: f32 = self.nx * light.x + self.ny * light.y + self.nz * light.z; //dot prod.
        if d > 1.0 {d=1.0};
        let difr = (diffuse >> 24) as f32;
        let difg = (diffuse >> 16 & 0x000000ff) as f32;
        let difb = (diffuse >> 8  & 0x000000ff) as f32;
        let ambr = (ambient >> 24) as f32;
        let ambg = (ambient >> 16 & 0x000000ff) as f32;
        let ambb = (ambient >> 8  & 0x000000ff) as f32;
        let mut rd = ((self.r as f32) + difr) * d + ambr; if rd > 255.0 {rd = 255.0};
        let mut gr = ((self.g as f32) + difg) * d + ambg; if gr > 255.0 {gr = 255.0};
        let mut bl = ((self.b as f32) + difb) * d + ambb; if bl > 255.0 {bl = 255.0};
        self.r = rd as u8;
        self.g = gr as u8;
        self.b = bl as u8;
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
        // rotate xyz
        let tmp_x: f32 = self.s * v.x + v.y * self.xy + v.z * self.xz;
        let tmp_y: f32 = self.s * v.y - v.x * self.xy + v.z * self.yz;
        let tmp_z: f32 = self.s * v.z - v.x * self.xz - v.y * self.yz;
        let tri: f32 = v.x * self.yz - v.y * self.xz + v.z * self.xy;
        let r_x: f32 = self.s * tmp_x + tmp_y * self.xy + tmp_z * self.xz + tri   * self.yz;
        let r_y: f32 = self.s * tmp_y - tmp_x * self.xy - tri   * self.xz + tmp_z * self.yz;
        let r_z: f32 = self.s * tmp_z + tri   * self.xy - tmp_x * self.xz - tmp_y * self.yz;
        //let mut vert: Vertex = Vertex::new_with_xyz(r_x, r_y, r_z);
                
        // rotate normal
        let tmpn_x: f32 = self.s * v.nx + v.ny * self.xy + v.nz * self.xz;
        let tmpn_y: f32 = self.s * v.ny - v.nx * self.xy + v.nz * self.yz;
        let tmpn_z: f32 = self.s * v.nz - v.nx * self.xz - v.ny * self.yz;
        let trin: f32 = v.nx * self.yz - v.ny * self.xz + v.nz * self.xy;
        let r_nx: f32 = self.s * tmpn_x + tmpn_y * self.xy + tmpn_z * self.xz + trin   * self.yz;
        let r_ny: f32 = self.s * tmpn_y - tmpn_x * self.xy - trin   * self.xz + tmpn_z * self.yz;
        let r_nz: f32 = self.s * tmpn_z + trin   * self.xy - tmpn_x * self.xz - tmpn_y * self.yz;
        let mut vert: Vertex = Vertex::new_with_xyz_nxnynz(r_x, r_y, r_z, r_nx, r_ny, r_nz);
        // vert.nx = v.nx; vert.ny = v.ny; vert.nz = v.nz;
        vert.r = v.r; vert.g = v.g; vert.b = v.b; vert.a = v.a;
        return vert
    }
    //fn rotate_normal(self, v: &Vertex) -> Vertex {

        //vert.x = v.x; vert.y = v.y; vert.z = v.z;
        //vert.r = v.r; vert.g = v.g; vert.b = v.b; vert.a = v.a;
        //return vert
    //}
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
    // 64 color palette, adapted from DB32
    let _pal64: [u32; 64] = [0x000000ff, 0x222034ff, 0x45283cff, 0x663931ff, 
                            0x8f563bff, 0xdf7126ff, 0xd9a066ff, 0xeec39aff,
                            0xfbf236ff, 0x99e550ff, 0x6abe30ff, 0x37946eff,
                            0x4b692fff, 0x524b24ff, 0x323c39ff, 0x3f3f74ff,
                            0x306082ff, 0x5b6ee1ff, 0x639bffff, 0x5fcde4ff,
                            0xcbdbfcff, 0xffffffff, 0x9badb7ff, 0x847e87ff,
                            0x696a6aff, 0x595652ff, 0x76428aff, 0xac3232ff,
                            0xd95763ff, 0xd77bbaff, 0x8f974aff, 0x8a6f30ff,
                            0x02137eff, 0x1032ffff, 0x1651bbff, 0x3cb0c8ff,
                            0xc27600ff, 0xffc363ff, 0xad5132ff, 0xff9b00ff,
                            0xff6d33ff, 0xb97a60ff, 0xa88a5eff, 0xf3eba5ff,
                            0x28240aff, 0x374a0fff, 0x5b7d73ff, 0x22370eff,
                            0x393413ff, 0x1f2121ff, 0x513940ff, 0x471927ff,
                            0x322d26ff, 0xab00d3ff, 0xc10000ff, 0x7a3045ff,
                            0xff53d4ff, 0x9baa8dff, 0x846d59ff, 0x15120eff,
                            0x411552ff, 0x640000ff, 0x390715ff, 0x5a422dff];
    //commandline args
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    let foldername = &args[1];
    let angle_s = &args[2];
    let angle = angle_s.parse::<f32>().unwrap();
    let directions_s = &args[3];
    let dir_offset_s = &args[4];
    let scale_s = &args[5];
    let vertical_s = &args[6];
    let light_ew_s = &args[7];
    let light_ns_s = &args[8];
    let diffuse_s = &args[9];
    let ambient_s = &args[10];
    let frame_name_s = &args[11];

    let directions = directions_s.parse::<i32>().unwrap();
    let dir_offset = dir_offset_s.parse::<i32>().unwrap();
    let scale = scale_s.parse::<f32>().unwrap();
    let vertical = vertical_s.parse::<f32>().unwrap();
    let light_ew = light_ew_s.parse::<f32>().unwrap();
    let light_ns = light_ns_s.parse::<f32>().unwrap();

    let diffuse = u32::from_str_radix(diffuse_s.trim_start_matches("0x"), 16).unwrap();
    let ambient = u32::from_str_radix(ambient_s.trim_start_matches("0x"), 16).unwrap();

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
            // rotate vertex
            let rotor3: Rotor3 = rotor2.multiply(rotor1);
            let mut rotated_vert: Vertex = rotor3.rotate(&vert);
            // lighting
            let light_from_up: Vertex = Vertex::new_with_xyz(0.0, 1.0, 0.0);
            let lp1: BiVec3 = BiVec3::new(0.0, 0.0, 1.0);
            let rot1 = Rotor3::new_from_angle_and_plane(lp1, light_ns * (PI/180.0f32));
            let lp2: BiVec3 = BiVec3::new(1.0, 0.0, 0.0);
            let rot2 = Rotor3::new_from_angle_and_plane(lp2, light_ew * (PI/180.0f32));
            let rot3: Rotor3 = rot2.multiply(rot1);
            let light: Vertex = rot3.rotate(&light_from_up);
            rotated_vert.shade(light, diffuse, ambient);
            rotated_vertices.push( rotated_vert );
        }


        let mut pixels: [u32; 65536] = [0; 65536]; // 256 x 256 = 65536 pixels

        for y in 0..=255 {
            for x  in 0..=255 {
                pixel_sample_ply(x, y, 0.025, &rotated_vertices, scale, vertical, &mut pixels);
            }
        }

        // write png with frame name and direction angle
        let outpath = foldername.to_owned() + "/" + frame_name_s + "_" + &dir_angle.to_string() + ".png";
        println!("direction {:?}", outpath);
        let path = Path::new(&outpath);
        write_png(&mut pixels, path);
    }
}

fn pixel_sample_ply (x: u8, y: u8, psize: f32, verts: &[Vertex],
                     scale: f32, vertical: f32, pixels: &mut [u32]) {
    let xf = (((x as f32) / 128.0) - 1.0) * scale;
    let yf = (((y as f32) / 128.0) - (1.0 + vertical)) * scale;

    let mut z1st: f32 = -scale; //-2.01
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

    if z1st > -scale { //-1.0
                
        pixels[(256*256-1) - (256 * (y as usize)) + (x as usize)-1] = z1stcolor;
        
    }
}

fn write_png (pixels: &mut[u32], path: &Path) {
    //convert to byte array    
    let mut bytes = Vec::<u8>::new();
    for val in pixels{
        bytes.extend_from_slice(&val.to_be_bytes());
    }

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
