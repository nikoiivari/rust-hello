#![allow(dead_code)]
extern crate png;
use std::fs::File;
use std::io::BufWriter;
//use std::io::BufReader;
use std::path::Path;
use std::ops;
use std::vec::Vec;
//use std::fmt; //println!

#[derive(Copy, Clone, PartialEq)]
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
    fn normalize(&mut self) {
        let len: f32  = f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
        self.x = self.x * (1.0 / len);
        self.y = self.y * (1.0 / len);
        self.z = self.z * (1.0 / len);
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

impl ops::Mul<f32> for Vec3 { // multiply Vec3 by scalar
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
struct Color {
    y: f32,
    co: f32,
    cg: f32,
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    fn new_from_rgb(r: f32, g: f32, b: f32) -> Self {
        // RGB to YCoCg-R
        let tmp_co = r - b;
        let tmp = b + tmp_co/2.;
        let tmp_cg = g - tmp;
        
        Color {
            y: tmp + tmp_cg/2.,
            co: tmp_co,
            cg: tmp_cg,
            r: r,
            g: g,
            b: b,
        }
    }
    fn new_from_ycocgr(y: f32, co: f32, cg: f32) -> Self {
        // clamp...
        // YCoCg-R ro RGB
        let tmp = y - cg/2.;
        let tmp_g = cg + tmp;
        let tmp_b = tmp - co/2.;
        let tmp_r = tmp_b + co;

        Color {
            y: y,
            co: co,
            cg: cg,
            r: tmp_r,
            g: tmp_g,
            b: tmp_b,
        }
    }
    //fn new_from_hsv(h: f32, s: f32, v: f32) -> Self {
    //    let c = v * s;
        
    //}
}

#[derive(Copy, Clone)]
struct Intersection {
    xyz: Vec3,
    normal: Vec3,
    color: Color,
    happened: bool,
}

impl Intersection {
    fn new_with_xyz_normal_color(xyz: Vec3, normal: Vec3, color: Color) -> Self {
        Intersection {
            xyz: xyz,
            normal: normal,
            color: color,
            happened: true,
        }
    }
    fn new_did_not_happen() -> Self {
        let meh1: Vec3 = Vec3::new_origo();
        let meh2: Vec3 = Vec3::new_origo();
        
        Intersection {
            xyz: meh1,
            normal: meh2,
            color: Color::new_from_rgb(0., 0., 0.),
            happened: false,
        }
    }
    fn shade(&mut self, light: Vec3, diffuse: u32, ambient: u32) {
        //self.normal.normalize();
        //dot prod.
        let mut d: f32 = self.normal.x * light.x + self.normal.y * light.y + self.normal.z * light.z; 
        
        if d > 1.0 {d=1.0};
        
        let difr = (diffuse >> 24) as f32;
        let difg = (diffuse >> 16 & 0x000000ff) as f32;
        let difb = (diffuse >> 8  & 0x000000ff) as f32;
        let _difc = Color::new_from_rgb(difr / 255., difg / 255., difb / 255.);
        let ambr = (ambient >> 24) as f32;
        let ambg = (ambient >> 16 & 0x000000ff) as f32;
        let ambb = (ambient >> 8  & 0x000000ff) as f32;
        let ambc = Color::new_from_rgb(ambr / 255., ambg / 255., ambb / 255.);
        
        let col_d = Color::new_from_rgb(self.color.r * d, self.color.g * d, self.color.b * d);
        let col_a = Color::new_from_ycocgr((col_d.y + ambc.y)/2., col_d.co, col_d.cg);

        //let mut rd = (self.color.r * 255. + difr)/2. * d + ambr; if rd > 255.0 {rd = 255.0};
        //let mut gr = (self.color.g * 255. + difg)/2. * d + ambg; if gr > 255.0 {gr = 255.0};
        //let mut bl = (self.color.b * 255. + difb)/2. * d + ambb; if bl > 255.0 {bl = 255.0};

        let mut rd = col_a.r; if rd > 1.0 {rd = 1.0};
        let mut gr = col_a.g; if gr > 1.0 {gr = 1.0};
        let mut bl = col_a.b; if bl > 1.0 {bl = 1.0};
                
        self.color = Color::new_from_rgb( rd, gr, bl );   
    }
}

#[derive(Copy, Clone, PartialEq)]
struct Sphere {
    xyz: Vec3,
    radius: f32,
    color: Color,
}

impl Sphere {
    fn new_with_xyz_radius_color(xyz: Vec3, radius: f32, color: Color) -> Self {
        Sphere {
            xyz: xyz,
            radius: radius,
            color: color,
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
        // find normal
        let int_n: Vec3 = Vec3 {
            x: (int_p.x - self.xyz.x)/self.radius,
            y: (int_p.y - self.xyz.y)/self.radius,
            z: (int_p.z - self.xyz.z)/self.radius,
        };
        //int_n.normalize();
        let int_color: Color = self.color;
        let inters: Intersection = Intersection::new_with_xyz_normal_color(int_p, int_n, int_color);
        
        inters
    }

    
}

fn dot3 (a: &Vec3, b: &Vec3) -> f32 {
    return a.x*b.x + a.y*b.y + a.z*b.z
}

fn max3 (r: f32, g: f32, b: f32) -> f32 {
    let rrg: bool = r > g;
    let ggb: bool = g > b;
    if rrg & ggb {return r;}
    else if ggb {return g;}
    else {return b;}
}

fn min3 (r: f32, g: f32, b: f32) -> f32 {
    let grr: bool = r > g;
    let bgg: bool = g > b;
    if grr & bgg {return b;}
    else if grr {return g;}
    else {return r;}
}

#[derive(PartialEq)]
enum Codepath {
    Raytrace,
    Spheretrace,
}

fn main() {
    let codepath = Codepath::Raytrace;
    //let slso: [u32; 8] = [0x0d2b45, 0x203c56, 0x544e68, 0x8d697a, 
    //                      0xd08159, 0xffaa5e, 0xffd4a3, 0xffecd6];
    let mut pixels: [u32; 65536] = [0; 65536]; // 256 x 256 = 65536 pixels
    
    // TODO: create a vector of Spheres and loop through them for every pixel.
    let mut spheres: Vec<Sphere> = Vec::new();
    //let oxyz: Vec3 = Vec3::new_origo();
    let rxyz: Vec3 = Vec3::new_with_xyz(0.0, -0.1, 0.2);
    let rsphere: Sphere = Sphere::new_with_xyz_radius_color(rxyz, 0.2, Color::new_from_rgb(0.6, 0.2, 0.2));
    spheres.push(rsphere);
    let gxyz: Vec3 = Vec3::new_with_xyz(0.0, 0.32, 0.0);
    let gsphere: Sphere = Sphere::new_with_xyz_radius_color(gxyz, 0.3, Color::new_from_rgb(0.2, 0.6, 0.2));
    spheres.push(gsphere);

    // directional lightsource
    let light: Vec3 = Vec3::new_with_xyz(1.0, -1.0, 0.8);
    //light = light * -1.0;
    //light.normalize();

    if codepath == Codepath::Raytrace {
        for y in 0..=255 {
            for x  in 0..=255 {
                 
                pixel_sample_rt (x, y, &spheres, light, 0xeeeeeeff, 0x4444eeff, 1.0, 0., &mut pixels);
            }
        }
    }
    let path = Path::new(r"traced.png");
    write_png (&mut pixels, path)

}

fn pixel_sample_rt (x: u8, y: u8, spheres: &[Sphere], mut light: Vec3, diffuse: u32, ambient: u32,
    scale: f32, vertical: f32, pixels: &mut [u32]) {
    
    let xf: f32 = (((x as f32) / 128.0) - 1.0) * scale;
    let yf: f32 = (((y as f32) / 128.0) - (1.0 + vertical)) * scale;

    let mut z1st: f32 = -scale + 0.01; //-2.01
    let mut z1stcolor: u32 = 0x0000ffff;

    for s in spheres {
        let origin: Vec3 = Vec3::new_with_xyz(xf, yf, scale);
        let destination: Vec3 = Vec3::new_with_xyz(xf, yf, -scale);
        // FIXME: Does ray_intersection() flip x & y coordinates?
        let mut inters: Intersection = s.ray_intersection(origin, destination);
        if inters.happened == true {
            if z1st < inters.xyz.z {
                z1st = inters.xyz.z;
                
                // shade intersection point
                let mut shade_light: Vec3 = light;
                shade_light.normalize();
                inters.shade(shade_light, diffuse, ambient);
                // (shaded) intersection color before shadow
                z1stcolor  = (((inters.color.r * 255.0) as u32) << 24) +
                             (((inters.color.g * 255.0) as u32) << 16) + 
                             (((inters.color.b * 255.0) as u32) << 8) + 0xff;
                  
                // find shadows
                for ss in spheres {
                    if ss != s {
                        // FIXME: light  position gets inverted...?
                        light.x = light.x * -1.;
                        light.y = light.y * -1.;
                        let shdw_intr: Intersection = ss.ray_intersection(inters.xyz, light * -1.);
                        if shdw_intr.happened == true {
                            // shadow color
                            z1stcolor  = 0x000000ff; 
                        }
                    }
                }
                
            }
        }
        
    }

    // flat solid Z-plane through origo 
    //z1st = 0.0;
    //if (x as u8) == 128 {println!("slice {:x}", z1stcolor); }
    if z1st > -scale {
        // find shadows for plane
        let ym = 255 - y;       
        pixels[(256 * ym as usize) + (x as usize)] = z1stcolor;
        
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