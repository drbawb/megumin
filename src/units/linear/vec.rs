use std::ops::{Add, Sub, Mul, Neg};
use std::ops::{AddAssign, SubAssign};

#[derive(Copy,Clone)]
pub struct V2 {
    pub x: f32, pub y: f32,
}

impl V2 {
    pub fn at(x: f32, y: f32) -> Self { V2 { x: x, y: y } }
}

impl Add for V2 {
    type Output = V2;
    fn add(self, rhs: V2) -> V2 {
        V2 { 
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for V2 {
    fn add_assign(&mut self, rhs: V2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for V2 {
    type Output = V2;
    fn sub(self, rhs: V2) -> V2 {
        V2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for V2 {
    fn sub_assign(&mut self, rhs: V2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

// TODO: this is *wrong*, real vector math soon
impl Mul for V2 {
    type Output = V2;
    fn mul(self, rhs: V2) -> V2 {
        V2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Neg for V2 {
    type Output = V2;
    fn neg(self) -> V2 {
        V2 {
            x: -self.x,
            y: -self.y,
        }
    }
}


