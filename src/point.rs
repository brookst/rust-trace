#[deriving(Show)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn mag(&self) -> f32 {
        Float::sqrt(self.mag2())
    }

    pub fn mag2(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}
