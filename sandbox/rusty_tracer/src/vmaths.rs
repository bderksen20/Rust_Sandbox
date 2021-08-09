// Bill Derksen - 8/21
//-- vector and matrix linear algebra utility structs/functions

use crate::stringable::{Stringable};
use std::ops;

//---- Linear Algebra Structs + Functions
//--- Point/Vec3 Struct + Imp
#[derive(Copy, Clone, Default)]
pub struct Point{
    pub x: f64,
    pub y: f64,
    pub z: f64
} pub use Point as Vec3;
impl Point{

    pub fn new() -> Point{
        Point{ x:0.0, y:0.0, z:0.0}
    }

    pub fn gen(a: f64, b: f64, c: f64) -> Point{
        Point{ x: a, y: b, z: c}
    }

    pub fn mag(&self) -> f64{
      (&self.x.powf(2.0) + &self.y.powf(2.0) + &self.z.powf(2.0)).abs().sqrt()
    }

    pub fn unit(&self) -> Vec3{
       *self *  (1.0 / self.mag())
    }

    pub fn dot(&self, vec: Vec3) -> f64{
        (&self.x * &vec.x + &self.y * &vec.y + &self.z * &vec.z)
    }

    pub fn cross(&self, vec: Vec3) -> Vec3{
        Vec3{x: (&self.y * vec.z) - (&self.z * vec.y), y: (&self.x * vec.z) - (&self.z * vec.x) , z: (&self.x * vec.y) - (&self.y * vec.x) }
    }

} impl Stringable for Point {
    fn stringy(&self) -> String{
        return String::from("<".to_owned() + &self.x.to_string() + ", "+ &self.y.to_string() + ", "+ &self.z.to_string() + ">");  
    }

} impl ops::Add for Vec3 {            //-- Add overload
    type Output = Vec3;

    fn add(self, vec: Vec3) -> Vec3 {
        Vec3 { x: &self.x + &vec.x, y: &self.y + &vec.y, z: &self.z + &vec.z }
    }

} impl ops::Sub for Vec3 {          //-- Sub - overload
    type Output = Vec3;

    fn sub(self, vec: Vec3) -> Vec3 {
        Vec3 { x: &self.x - &vec.x, y: &self.y - &vec.y, z: &self.z - &vec.z }
    }

} impl ops::Mul<f64> for Vec3 {     //-- SCALAR Mult * overload
    type Output = Vec3;

    fn mul(self, s: f64) -> Vec3 {
        Vec3 { x: &self.x * s, y: &self.y * s, z: &self.z * s }
    }

} impl ops::Neg for Vec3 {          //-- Neg -x overload
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 { x: -&self.x, y: -&self.y, z: -&self.z}
    }
} impl ops::Mul<Vec3> for f64{
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3{
        Vec3 { x: self * vec.x, y: self * vec.y, z: self * vec.z}
    }
}

//----- 3x3 Matrix
#[derive(Copy, Clone, Default)]
pub struct Mat3{
    pub x: Vec3,
    pub y: Vec3,
    pub z: Vec3

} impl Mat3 {
    
    pub fn gen_rotx(theta: f64) -> Mat3 {
        Mat3{x: Vec3::gen(1.0, 0.0, 0.0),
             y: Vec3::gen(0.0, theta.cos(), -theta.sin()),
             z: Vec3::gen(0.0, theta.sin(), theta.cos())
        }
    }

    pub fn gen_roty(theta: f64) -> Mat3 {
        Mat3{x: Vec3::gen(theta.cos(), 0.0, theta.sin()),
             y: Vec3::gen(0.0, 1.0, 0.0),
             z: Vec3::gen(-theta.sin(), 0.0, theta.cos())
        }
    }

} impl Stringable for Mat3{

    fn stringy(&self) -> String{
        return String::from(self.x.stringy().to_owned() + "\n" + &self.y.stringy() + "\n" + &self.z.stringy());
    }

} impl ops::Mul<Vec3> for Mat3{     //-- row major mat3 x vec3...
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3{
        Vec3{x: self.x.dot(vec), y: self.y.dot(vec), z: self.z.dot(vec)}
    }
}