// Bill Derksen - 8/21
//-- camera structure and utility functions

use crate::stringable::{Stringable};
use crate::vmaths::{Point, Vec3};

pub struct Camera{
    pub pos: Point,
    pub focl: f64,
    pub w: f64,
    pub h: f64
} impl Camera{

    pub fn new() -> Camera{
        Camera{ pos: Point{x:0.0, y:0.0, z: -16.0}, focl: 1.0, w: 16.0, h: 9.0 }
    }

} impl Stringable for Camera{
    fn stringy(&self) -> String{
        return String::from("position: ".to_owned() + &self.pos.stringy() + ", w = " + &self.w.to_string() + ", h = "+ &self.h.to_string()); 
    }
}