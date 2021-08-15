// Bill Derksen - 8/21
//-- camera structure and utility functions

use crate::stringable::{Stringable};
use crate::vmaths::{Point, Vec3};

pub struct Camera{
    pub pos: Point,
    pub focl: f64,
    pub w: f64,
    pub h: f64,

    /*
    pub near: f64,      //-- frustum vals
    pub far: f64,
    pub top: f64,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
    pub look_at: Point */

} impl Camera{

    pub fn new() -> Camera{
        Camera {pos: Point{x:0.0, y:0.0, z: -30.0}, focl: 1.0, w: 16.0, h: 9.0 }
        /*
        Camera{ 
            pos: Point{x:0.0, y:0.0, z: -16.0}, focl: 1.0, w: 16.0, h: 9.0 , near: 1.0, far: 40.0, top: 4.5, bottom: -4.5, left: -8.0, right: 8.0, look_at: Point::new()
        }
        */
    }

    /*
    pub fn gen(position: Point, focal_l: f64, width: f64, height: f64, n: f64, f: f64) -> Camera{
        Camera{ 
            pos: position, 
            focl: focal_l,
            w: width,
            h: height,
            near: n,
            far: f,
            top: 0.5 * height,
            bottom: -0.5 * height,
            left: -0.5 * width,
            right: 0.5 * width,
            look_at: Point::new()
        }
    }
    */

} impl Stringable for Camera{
    fn stringy(&self) -> String{
        return String::from("position: ".to_owned() + &self.pos.stringy() + ", w = " + &self.w.to_string() + ", h = "+ &self.h.to_string()); 
    }
}