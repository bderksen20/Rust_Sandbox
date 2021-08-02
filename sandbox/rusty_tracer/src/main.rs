/*  Bill Derksen - 7/21
 *  
 *      Toy ray tracer in Rust!  
 *
 */

use std::ops;
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use std::vec::Vec;
use std::convert::TryInto;

//TODO:
//-- 
//-- 1. Implement "Display" trait for easier printing? Over / with stringable?
//--
//--

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
    let r1 = Ray{origin: Point::new(), dir: Vec3{x:0.0, y:0.0, z:1.0}};   
    let s1 = Sphere{cen: Point{x:0.0, y:0.0, z:0.0}, r: 4.0};
    println!("Discriminant test: {}", s1.hits(&r1));

    //-- camera
    let cam = Camera::new();
    let step: f64 = cam.w / (img_w as f64);

    println!("World step size: {}", step);

    //-- file + png encoder/writer
    let path = Path::new("output/image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, img_w, img_h);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    //-- pixel buffer
    //let img_buff_size: usize = (img_h * img_w * 3).try_into().unwrap();
    //let mut img_buffer = vec![0; img_buff_size];
    let mut img_buffer: Vec<u8> = Vec::new();

    //-- launch rays
    //NOTE: not factoring in camera focal length
    let mut cool_ray = Ray{origin: cam.pos, dir: Vec3{x: -0.5 * cam.w, y: 0.5 * cam.h, z: 20.0}};
    
    for y in 0..img_h {

        for x in 0..img_w {
            //println!("Ray dir: {}", cool_ray.dir.stringy());
            if s1.hits(&cool_ray){
                //println!("HIT");
                img_buffer.push(0); img_buffer.push(0); img_buffer.push(255);       
            } else {
                img_buffer.push(155); img_buffer.push(255); img_buffer.push(255);
            }

            cool_ray.dir.x += step;
            //println!("PX: {}, {}", x, y);
        }
        
        cool_ray.dir.x = 0.0;
        cool_ray.dir.y += step;
    }

    //-- write buff to png
    let d: &[u8] = &img_buffer;
    //let d = &img_buffer[..];
    writer.write_image_data(d).unwrap();

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

    pub fn new() -> Point{
        Point{ x:0.0, y:0.0, z:0.0}
    }

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
    r: f64
}

//-- Ray xXx Sphere: ||x - c||^2 = R^2, solve for t where x = P(t) 
impl Hittable for Sphere{
    fn hits(&self, ray: &Ray) -> bool{
        let discrim = (2.0 * ray.dir.dot(ray.origin - self.cen)).powi(2) - (4.0 * ray.dir.mag().powi(2)) * ((ray.origin - self.cen).mag() - self.r.powi(2));

        if discrim >= 0.0 {
            true
        } else {
            false
        }
        //println!("Discrim: {}",discrim);

    }
}


//---- Camera:
struct Camera{
    pos: Point,
    focl: f64,
    w: f64,
    h: f64
}

impl Camera{

    pub fn new() -> Camera{
        Camera{ pos: Point{x:0.0, y:0.0, z: -5.0}, focl: 1.0, w: 16.0, h: 9.0 }
    }
}








