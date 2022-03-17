trait Walk {
    fn walk(&self);
}

trait Wheel {
    fn wheel(&self);
}

struct Mecha {
    can_wheel: bool,
    can_walk: bool,

};

impl Walk for Mecha {
    fn walk(&self) {
        println!("Mecha walk");
    }        
}

impl Wheel for Mecha {
    fn wheel(&self) {
        println!("Mecha wheel");
    }        
}