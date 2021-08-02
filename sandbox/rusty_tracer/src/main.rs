/*  Bill Derksen - 7/21
 *  
 *      Toy ray tracer in Rust!  
 *
 */

use std::ops;
use std::option;
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use std::vec::Vec;
use std::convert::TryInto;

use indicatif::ProgressBar;
use indicatif::ProgressStyle;

//TODO:
//-- 
//-- 1. Implement "Display" trait for easier printing? Over / with stringable?
//--
//--

fn main() {

    println!("\n----------------------------------\n|  Welcome to the rusty tracer!  |\n----------------------------------");

    //-- image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let img_h: u32 = 1080;
    let img_w = (aspect_ratio * img_h as f32) as u32;

    //-- camera
    let cam = Camera::new();
    let step: f64 = cam.w / (img_w as f64);

    println!("- img set to {} x {}", img_w, img_h);
    println!("- camera: {}", cam.stringy());
    println!("- world ray step size: {}", step);
    
    let v1 = Vec3{x: 1.5, y: -2.5, z: 0.0};
    let v2 = Vec3{x: 0.5, y: 10.0, z: 2.2};
    
    //-- TEST: printing and operator overloads
    println!("\n\nVec3 Operations Test...\n------------------------------------\nx = {}", v1.stringy());
    println!("y = {}", v2.stringy());
    println!("\n   -x = {}", (-v1).stringy());
    println!("x + y = {}", (v1 + v2).stringy());
    println!("x - y = {}", (v1 - v2).stringy());
    println!("x * 2.0 = {}", (v1 * 2.0).stringy());
    println!("x · y = {}", v1.dot(v2)); 
    println!("||x|| = {}", v1.mag());
    println!("\n'no drop' test: x = {}, y = {}", v1.stringy(), v2.stringy());
    
    //-- TEST: Ray operations and intersection
    let mut r1 = Ray{origin: Point{x:0.0,y:0.0,z:-5.0}, dir: Vec3{x:0.0, y:0.0, z:1.0}};   
    let s1 = Sphere{cen: Point{x:0.0, y:0.0, z:0.0}, r: 4.0, def_color: Color{r:0,g:0,b:255}};

    println!("\n\nRay Operations and Sphere Hit Test...\n-------------------------------------");
    println!("Sphere: {}", s1.stringy());
    println!("\n2x hit ray: {}", r1.stringy());
    //println!("hit? : {}", s1.hits(&r1));

    r1.origin.x = -2.0;
    println!("\n1x hit ray: {}", r1.stringy());
    //println!("Discriminant test: {}", s1.hits(&r1));

    r1.origin.x = -2.1;
    println!("\nmiss ray: {}", r1.stringy());
    //println!("Discriminant test: {}", s1.hits(&r1));

    //-- file + png encoder/writer
    let path = Path::new("output/image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, img_w, img_h);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    //-- pixel buffer
    let mut img_buffer: Vec<u8> = Vec::new();
    
    //-- progress bar
    println!("\n\nRendering...");
    let pbar = ProgressBar::new(img_h.into());
    pbar.set_style(ProgressStyle::default_bar().template("[{elapsed_precise}] {bar:50.green/cyan} {msg} {percent}%").progress_chars("=>#"));
    
    //-- light source
    let bulb = Sphere{cen: Point{x:-5.0, y:5.0, z: -5.0}, r: 1.0, def_color: Color{r:255, g:255,b:255}};

    //-- launch rays
    //- launches left->right | top->bottom by step size (prop to img size)
    //NOTE: not factoring in camera focal length
    let mut cool_ray = Ray{origin: cam.pos, dir: Vec3{x: -0.5 * cam.w, y: 0.5 * cam.h, z: 1.0}};
    for y in 0..img_h {
        for x in 0..img_w {
            match s1.hits(&cool_ray) {
                Some(hit_rec) => {
                    let color = phong_single_src(&hit_rec, &cam, &bulb);
                    img_buffer.push(color.r); img_buffer.push(color.g); img_buffer.push(color.b); 
                }

                None => {
                    img_buffer.push(155); img_buffer.push(255); img_buffer.push(255);
                }
                   
            }

            cool_ray.dir.x += step;
        }
        
        cool_ray.dir.x = -0.5 * cam.w;
        cool_ray.dir.y -= step;

        pbar.inc(1);
    }

    //-- cleanup
    pbar.finish();

    //-- convert buff and write to png
    let d: &[u8] = &img_buffer;
    writer.write_image_data(d).unwrap();

}

// <<<<<<<<<<<<<<<<<<<<  HELPER TRAITS, STRUCTS, IMPLS, ETC. >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

//---- Phong Shading
fn phong_single_src(hit_rec: &HitInfo, cam: &Camera, light: &Sphere) -> Color{
    let n: Vec3 = hit_rec.norm.unit();                  //- normalized normal
    let lv: Vec3 = hit_rec.ip - light.cen;              //- light -> hit pt
    let rv: Vec3 = lv - n * (lv.dot(n)) * 2.0;          //- perfect light reflection at hit pt
    let cv: Vec3 = cam.pos - hit_rec.ip;                //- hit pt -> camera "eye
    Color{r:0,g:0,b:0}
}

//---- Stringable: Implemented by objects to get description
trait Stringable{
    fn stringy(&self) -> String;
}

//---- Hittable: Implemented by any geometry
trait Hittable{
    fn hits(&self, ray: &Ray) -> Option<HitInfo>;
}

//---- Color
#[derive(Copy, Clone)]
struct Color{
    r: u8,
    g: u8,
    b: u8
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

    fn unit(&self) -> Vec3{
       *self *  (1.0 / &self.mag())
    }

    fn dot(&self, vec: Vec3) -> f64{
        (&self.x * &vec.x + &self.y * &vec.y + &self.z * &vec.z)
    }

    fn cross(&self, vec: Vec3) -> Vec3{
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
}

//---- Ray: follows formula P(t) = O + td
struct Ray{
    origin: Point,
    dir: Vec3

} impl Ray{

    pub fn at(&self, t: f64) -> Point{
        let p = self.origin - ( self.dir * t);
        return p;
    }

} impl Stringable for Ray{
    fn stringy(&self) -> String{
        return String::from("P(t) = ".to_owned() + &self.origin.stringy() + " + t" + &self.dir.stringy());
    }
}

//---- Sphere: follows eq (x-h)^2 + (y-i)^2 + (z-j)^2 = R^2
//-- vector form: ||x - c||^2 = R^2
struct Sphere{
    cen: Point,
    r: f64,
    def_color: Color

} impl Stringable for Sphere{

    fn stringy(&self) -> String{
        return String::from("center = ".to_owned() + &self.cen.stringy() + ", r = " + &self.r.to_string());
    }

} impl Hittable for Sphere{     //-- Ray xXx Sphere: ||x - c||^2 = R^2, solve for t where x = P(t) 

    fn hits(&self, ray: &Ray) -> Option<HitInfo>{
        let discrim = (2.0 * ray.dir.dot(ray.origin - self.cen)).powi(2) - (4.0 * ray.dir.mag().powi(2)) * ((ray.origin - self.cen).mag() - self.r.powi(2));
        
        if discrim < 0.01 && discrim > 0.0 {                                            //-- 1x hit handling

            let t = -(ray.dir.dot(ray.origin - self.cen)) / ray.dir.mag().powi(2);          //- solve for t
            Some(HitInfo{ip: ray.at(t), norm: ray.at(t) - self.cen})                        //- solve for incident pt + norm

        } else if discrim >= 0.01{                                                      //-- 2x hit handling
                        
            let t = (-2.0 * (ray.dir.dot(ray.origin - self.cen)) - discrim)  / (2.0 * ray.dir.mag().powi(2));   //- solve for t (want closer hit)
            Some(HitInfo{ip: ray.at(t), norm: ray.at(t) - self.cen})                                            //- solve for incident pt + norm

        } else {                //-- miss handling
            None
        }
    }
}

//---- Hit Info -----
struct HitInfo{ 
    ip: Point,
    norm: Vec3
}

//---- Camera ----
struct Camera{
    pos: Point,
    focl: f64,
    w: f64,
    h: f64
} impl Camera{

    pub fn new() -> Camera{
        Camera{ pos: Point{x:0.0, y:0.0, z: -5.0}, focl: 1.0, w: 16.0, h: 9.0 }
    }

} impl Stringable for Camera{
    fn stringy(&self) -> String{
        return String::from("position: ".to_owned() + &self.pos.stringy() + ", w = " + &self.w.to_string() + ", h = "+ &self.h.to_string()); 
    }
}

