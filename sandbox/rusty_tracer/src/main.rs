/*  Bill Derksen - 7/21
 *  
 *      Toy ray tracer in Rust!  
 *
 */

use std::ops;

fn main() {

    println!("Welcome to the rusty tracer........");

    //-- image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let img_h: u32 = 1080;
    let img_w = (aspect_ratio * img_h as f32) as u32;

    println!("- img set to {} x {}...", img_w, img_h);

    let v1 = Vec3{x: 1.5, y: -2.5, z: 0.0};
    let v2 = Vec3{x: 0.5, y: 10.0, z: 2.2};
    
    //-- TEST: printing and operator overloads
    println!("\nVec3 Operations Test...\n------------------------------------\nx = {}", v1.stringy());
    println!("y = {}", v2.stringy());
    println!("\n   -x = {}", (-v1).stringy());
    println!("x + y = {}", (v1 + v2).stringy());
    println!("x - y = {}", (v1 - v2).stringy());
    println!("x * y = {}", (v1 * v2).stringy());
    println!("x · y = {}", v1.dot(v2)); 
    println!("||x|| = {}", v1.mag());
    println!("\n'no drop' test: x = {}, y = {}", v1.stringy(), v2.stringy());
    
    //-- TEST: Ray operations and intersection 
    

    //-- camera
    
    //-- trace
    
}

//---- Stringable: Implemented by objects to get description
trait Stringable{
    fn stringy(&self) -> String;
}

//---- Hittable: Implemented by any geometry
trait Hittable{
    fn hits(&self, ray: &Ray) -> bool;
}

//---- Linear Algebra Structs + Functions

//--- Point/Vec3 Struct + Imp
#[derive(Copy, Clone)]
struct Point{
    x: f64,
    y: f64,
    z: f64

} use Point as Vec3;

impl Point{
    
    fn mag(&self) -> f64{
      (&self.x.powi(2) + &self.y.powi(2) + &self.z.powi(2)).abs()
    }

    fn dot(&self, vec: Vec3) -> f64{
        (&self.x * &vec.x + &self.y * &vec.y + &self.z * &vec.z)
    }
}

//-- .stringy impl
impl Stringable for Point {
    fn stringy(&self) -> String{
        return String::from("< ".to_owned() + &self.x.to_string() + ", "+ &self.y.to_string() + ", "+ &self.z.to_string() + " >");  
    }
}

//-- Add + overload
//- NOTE: to use x + y over reference, needed to implement Copy & Clone for struct... are refs to
//member data ok???
impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, vec: Vec3) -> Vec3 {
        Vec3 { x: &self.x + &vec.x, y: &self.y + &vec.y, z: &self.z + &vec.z }
    }
}

//-- Sub - overload
impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, vec: Vec3) -> Vec3 {
        Vec3 { x: &self.x - &vec.x, y: &self.y - &vec.y, z: &self.z - &vec.z }
    }
}

//-- Mul * overload
impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3 { x: &self.x * &vec.x, y: &self.y * &vec.y, z: &self.z * &vec.z }
    }
}

//-- Neg -x overload
impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 { x: -&self.x, y: -&self.y, z: -&self.z}
    }
}


//---- Ray: follows formula P(t) = O + td
struct Ray{
    origin: Point,
    //t: i32,
    dir: Vec3
}

//---- Sphere: follows eq (x-h)^2 + (y-i)^2 + (z-j)^2 = R^2
//-- vector form: ||x - c||^2 = R^2
struct Sphere{
    cen: Point,
    r: i32
}









