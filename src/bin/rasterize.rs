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
//use ply_rs::ply;
use ply_rs::parser;

mod rotor;
use rotor::vertex::Vertex;
use rotor::vertex::pixel_sample_ply;
use rotor::BiVec3;
use rotor::Rotor3;


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
