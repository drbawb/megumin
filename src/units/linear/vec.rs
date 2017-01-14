use std::ops::{Add, Sub, Mul, Neg};
use std::ops::{AddAssign, SubAssign};

#[derive(Copy,Clone,Debug)]
pub struct V2 {
    pub x: f32, pub y: f32,
}

impl V2 {
    pub fn at(x: f32, y: f32) -> Self { V2 { x: x, y: y } }
    
    pub fn dot(self, rhs: V2) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }

    pub fn len(self) -> f32 {
        f32::sqrt(f32::powi(self.x, 2) + f32::powi(self.y, 2))
    }

    pub fn set_len(self, new_len: f32) -> V2 {
        V2 {
            x: self.x * (new_len / self.len()),
            y: self.y * (new_len / self.len()),
        }
    }

    pub fn norm(self) -> V2 {
        let len = self.len();
        if len == 0.0 { return self }

        V2 {
            x: self.x / len,
            y: self.y / len,
        }
    }

    pub fn rot(self, theta: f32) -> V2 {
        let cos_r = theta.cos(); // TODO: ???
        let sin_r = theta.sin(); // TODO: ???

        V2 {
            x: (cos_r * self.x) - (sin_r * self.y),
            y: (sin_r * self.x) + (cos_r * self.y),
        }
    }

    pub fn theta(self) -> f32 {
        f32::atan2(self.y, self.x)
    }
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

// TODO: do we really want scalar mult in here?
impl Mul for V2 {
    type Output = V2;
    fn mul(self, rhs: V2) -> V2 {
        V2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<f32> for V2 {
    type Output = V2;
    fn mul(self, rhs: f32) -> V2 {
        V2 {
            x: self.x * rhs,
            y: self.y * rhs,
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
