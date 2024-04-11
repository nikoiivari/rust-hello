pub mod vertex;
use rotor::vertex::Vertex;

pub struct BiVec3 {
    xy: f32,
    xz: f32,
    yz: f32,
}

impl BiVec3 {
    pub fn new(xy: f32, xz: f32, yz: f32) -> Self {
        BiVec3 {
            xy: xy,
            xz: xz,
            yz: yz,
        }
    }
}

pub struct Rotor3 {
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
    pub fn new_from_vert_to_vert(a: Vertex, b: Vertex) -> Self {
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
    pub fn new_from_angle_and_plane(plane: BiVec3, angle_rad: f32) -> Self {
        Rotor3 {
            s: (angle_rad / 2.0).cos(),
            xy: -(angle_rad / 2.0).sin() * plane.xy,
            xz: -(angle_rad / 2.0).sin() * plane.xz,
            yz: -(angle_rad / 2.0).sin() * plane.yz,
        }
    }
    pub fn normalize(&mut self) {
        let lsqr: f32 = self.s * self.s + 
                        self.xy * self.xy + 
                        self.xz * self.xz + 
                        self.yz * self.yz;
        let length: f32 = lsqr.sqrt();
        self.s /= length;
        self.xy /= length; self.xz /= length; self.yz /= length;
    }
    pub fn multiply(self, m: Rotor3) -> Self {
        Rotor3 {
            s: self.s * m.s - self.xy * m.xy - self.xz * m.xz - self.yz * m.yz,
            xy: self.xy * m.s + self.s * m.xy + self.yz * m.xz - self.xz * m.yz,
            xz: self.xz * m.s + self.s * m.xz - self.yz * m.xy + self.xy * m.yz,
            yz: self.yz * m.s + self.s * m.yz + self.xz * m.xy - self.xy * m.xz,
        }
    }
    pub fn rotate(self, v: &Vertex) -> Vertex {
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

pub fn outer3 (a: &Vertex, b: &Vertex) -> BiVec3 {
    let mut c =  BiVec3::new(0.0, 0.0, 0.0);
    c.xy = a.x * b.y - a.y * b.x;
    c.xz = a.x * b.z - a.z * b.x;
    c.yz = a.y * b.z - a.z * b.y;

    return c
}

pub fn dot3 (a: &Vertex, b: &Vertex) -> f32 {
    return a.x*b.x + a.y*b.y + a.z*b.z
}