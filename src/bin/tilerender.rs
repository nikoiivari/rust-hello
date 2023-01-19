#![allow(dead_code)]
extern crate png;
use std::fs::File;
use std::io::BufWriter;
//use std::io::BufReader;
use std::path::Path;
use std::ops;

#[derive(Copy, Clone)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn new_with_xyz(x: f32, y: f32, z: f32) -> Self {
        Vec3 {
            x: x,
            y: y,
            z: z, 
        }
    }
    fn new_origo() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0, 
        }
    }
    fn length(&self) -> f32 {
        let len: f32  = f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
        len
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        // add self + other = result
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        // sub self - other = result
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

struct Intersection {
    xyz: Vec3,
    normal: Vec3,
    rgb: Vec3,
    happened: bool,
}

impl Intersection {
    fn new_with_xyz_normal_rgb(xyz: Vec3, normal: Vec3, rgb: Vec3) -> Self {
        Intersection {
            xyz: xyz,
            normal: normal,
            rgb: rgb,
            happened: true,
        }
    }
    fn new_did_not_happen() -> Self {
        let meh1: Vec3 = Vec3::new_origo();
        let meh2: Vec3 = Vec3::new_origo();
        let meh3: Vec3 = Vec3::new_origo();
        Intersection {
            xyz: meh1,
            normal: meh2,
            rgb: meh3,
            happened: false,
        }
    }
}

struct Sphere {
    xyz: Vec3,
    radius: f32,
    rgb: Vec3,
}

impl Sphere {
    fn new_with_xyz_radius_rgb(xyz: Vec3, radius: f32, rgb: Vec3) -> Self {
        Sphere {
            xyz: xyz,
            radius: radius,
            rgb: rgb,
        }
    }

    fn ray_intersection(self, origin: Vec3, destination: Vec3) -> Intersection {
        // https://www.ccs.neu.edu/home/fell/CS4300/Lectures/Ray-TracingFormulas.pdf
        let d: Vec3 =  destination - origin;
        
        let a: f32 = d.x*d.x + d.y*d.y + d.z*d.z;
        let b: f32 = 2.0*d.x*(origin.x - self.xyz.x) + 
                     2.0*d.y*(origin.y - self.xyz.y) + 
                     2.0*d.z*(origin.z - self.xyz.z);
        let c: f32 = self.xyz.x * self.xyz.x + self.xyz.y * self.xyz.y + self.xyz.z * self.xyz.z +
                     origin.x * origin.x + origin.y * origin.y + origin.z * origin.z +
                     -2.0*(self.xyz.x*origin.x + self.xyz.y*origin.y + self.xyz.z*origin.z) - 
                     self.radius*self.radius;
                
        let discriminant: f32 = b*b - 4.0*a*c;
        if discriminant <= 0.0 {
            let inters: Intersection = Intersection::new_did_not_happen();
            return inters;
        }

        // find distance to nearest intersection
        let t: f32 = (-b - f32::sqrt(b*b - 4.0*a*c))/(2.0*a);
        // find coordinates to intersection point
        let int_p: Vec3 = Vec3 {
            x: origin.x + t*d.x,
            y: origin.y + t*d.y,
            z: origin.z + t*d.z,
        };
        // FIXME!!!! normal and rgb
        let int_n: Vec3 = Vec3::new_origo();
        let int_rgb: Vec3 = Vec3::new_origo();
        let inters: Intersection = Intersection::new_with_xyz_normal_rgb(int_p, int_n, int_rgb);
        
        inters
    }
}

fn dot3 (a: &Vec3, b: &Vec3) -> f32 {
    return a.x*b.x + a.y*b.y + a.z*b.z
}

fn main() {
    //let slso: [u32; 8] = [0x0d2b45, 0x203c56, 0x544e68, 0x8d697a, 
    //                      0xd08159, 0xffaa5e, 0xffd4a3, 0xffecd6];
    let mut pixels: [u32; 65536] = [0; 65536]; // 256 x 256 = 65536 pixels
    //let mut in_pixels: [u32; 65536] = [0; 65536];
    let path = Path::new(r"traced.png");
    write_png (&mut pixels, path)

}

fn write_png (pixels: &mut[u32], path: &Path) {
    //convert to byte array    
    let mut bytes = Vec::<u8>::new();
    for val in pixels{
        bytes.extend_from_slice(&val.to_be_bytes());
    }

    //
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