use ply_rs::ply;
//use ply::Property;
//use ply_rs::parser;

#[derive(Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub nx: f32,
    pub ny: f32,
    pub nz: f32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Vertex {
    pub fn new_with_xyz(x: f32, y: f32, z: f32) -> Self {
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

    pub fn new_with_xyz_nxnynz(x: f32, y: f32, z: f32, nx: f32, ny: f32, nz: f32) -> Self {
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

    pub fn normalize_normal(&mut self) {
        let len: f32  = f32::sqrt(self.nx * self.nx + self.ny * self.ny + self.nz * self.nz);
        self.nx = self.nx * (1.0 / len);
        self.ny = self.ny * (1.0 / len);
        self.nz = self.nz * (1.0 / len);
    }

    pub fn shade(&mut self, light: Vertex, diffuse: u32, ambient: u32) {
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

pub fn pixel_sample_ply (x: u8, y: u8, psize: f32, verts: &[Vertex],
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
        
        let ym = 255 - y;
        pixels[(256 * ym as usize) + (x as usize)] = z1stcolor;
        
    }
}