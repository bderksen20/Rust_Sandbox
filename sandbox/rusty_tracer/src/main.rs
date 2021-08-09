/*  Bill Derksen - 7/21
 *  
 *      Toy ray tracer in Rust!  
 *
 */

 mod hittable;
 mod stringable;
 mod vmaths;
 mod camera;
 mod geometry;
 mod ray;

use std::ops;
use std::option;
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use std::vec::Vec;
use std::convert::TryInto;
use std::process;
use std::f64::consts::PI;

use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use colored::*;

//-- self-contained use
use ray::{Ray};
use geometry::{Sphere};                     //TODO: should not need this here as we want a vector of "hittables"
use camera::{Camera};
use vmaths::{Point, Vec3, Mat3};
use stringable::{Stringable};
use hittable::{Hittable, HitInfo};

//-- TODO For Need!
//-- 1. "Generalize" object model, eg. put varied "Scene Object" geometry structs into single vec
//-- 2. Upgrade to materials rather than just base color

//-- TODO For Fun!
//-- fun1. 3D fractals! fractals out of other geometry???
//-- fun3. path tracer implementation

fn main() {

    println!("{}{}{}", "\n-----------------------------------------------------------------------\n|".purple(),"                    Welcome to the rusty tracer!                     ".green(),"|\n-----------------------------------------------------------------------\n".purple());

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
    let m1 = Mat3::gen_roty((PI/360.0));
    let m2 = Mat3::gen_roty(-(PI/2048.0));

    //-- TEST: printing and operator overloads
    println!("{}{}{}", "\n\nVec3 Operations Test...".green(), "\n-----------------------------------------------------------------------\n".purple(), String::from("x = ") + &v1.stringy());
    println!("y = {}", v2.stringy());
    println!("\n   -x = {}", (-v1).stringy());
    println!("x + y = {}", (v1 + v2).stringy());
    println!("x - y = {}", (v1 - v2).stringy());
    println!("x * 2.0 = {}", (v1 * 2.0).stringy());
    println!("x · y = {}", v1.dot(v2)); 
    println!("||x|| = {}", v1.mag());
    println!("\n'no drop' test: x = {}, y = {}", v1.stringy(), v2.stringy());
    
    println!("{}{}", "\n\nMat3 Operations Test...\n".green(), "-----------------------------------------------------------------------\n".purple());
    println!("1/2pi y-rot mat3 = \n{}", m1.stringy()); 
    println!("^m1 * x = {}", (m1 * v1).stringy());

    //-- scene data init
    let mut r1 = Ray{origin: Point{x:0.0,y:0.0,z:-5.0}, dir: Vec3{x:0.0, y:0.0, z:1.0}};   
    let mut s1: Sphere = Sphere{cen: Point{x:0.0, y:0.0, z:4.0}, r: 4.0, def_color: Point{x: 0.2, y: 0.2, z: 0.6}};     //-- blue sphere, mid cen
    let mut s2: Sphere = Sphere{cen: Point{x:6.0, y:2.0, z: 12.0}, r: 4.0, def_color: Point{x: 0.6, y: 0.2, z: 0.2}};   //-- red sphere, back r
    let mut s3: Sphere = Sphere{cen: Point{x:-6.0, y:2.0, z:12.0}, r: 4.0, def_color: Point{x: 0.2, y: 0.6, z: 0.2}};   //-- green sphere, front l
    let mut dummy_s: Sphere = Sphere::default();
    
    //-- light source
    let mut bulb1 = PointLight{pos: Point{x:-12.5,y:10.0,z:-8.0}, id: Point{x:1.0,y:1.0,z:1.0}, is: Point{x:1.0,y:1.0,z:1.0}};
    let mut bulb2 = PointLight{pos: Point{x:12.5,y:10.0,z:8.0}, id: Point{x:1.0,y:1.0,z:1.0}, is: Point{x:1.0,y:1.0,z:1.0}};
    let mut lights: Vec<&mut PointLight> = Vec::new();
    lights.push(&mut bulb1);
    lights.push(&mut bulb2);

    //-- scene: vector of mutable references   
    let mut scene: Vec<&mut Sphere> = Vec::new();
    scene.push(&mut s1);
    scene.push(&mut s2);
    scene.push(&mut s3);
    //scene.push(& bulb);                           //NOTE: iss2. remove light from scene objs

    //-- frame loop
    let frames = 1;
    for frame in 0..frames{

        //-- file + png encoder/writer
        let mut pathstr = String::from("output/frame_".to_owned() + &frame.to_string() + ".png");
        if frames == 1{
            pathstr = String::from("output/image.png");
        }
        let path = Path::new(&pathstr);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, img_w, img_h);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        //-- pixel buffer
        let mut img_buffer: Vec<u8> = Vec::new();
        
        //-- progress bar
        println!("{}{}", (String::from("\n\nRendering frame: ") + &frame.to_string()).green(), "\n-----------------------------------------------------------------------".purple());
        let pbar = ProgressBar::new(img_h.into());
        pbar.set_style(ProgressStyle::default_bar().template("[{elapsed_precise}] [{bar:50.green/cyan}] {msg} {percent}%").progress_chars("=>#"));
        
        //-- launch rays
        //- launches right-left <- | ^ bottom-top by step size (prop to img size)
        let mut cool_ray = Ray{origin: cam.pos, dir: Vec3{x: 0.5 * cam.w, y: -0.5 * cam.h, z: cam.pos.z + cam.focl}};
        for y in 0..img_h {
            for x in 0..img_w {
                
                let mut first_hit: bool = true;
                let mut closest_hit: HitInfo = HitInfo{ip: Point::default(), norm: Point::default(), obj: &dummy_s};    //-- dummy init to avoid compile errors  
                let mut color: Color = Color::default();
                        
                for obj in &mut scene{                                                                                  //-- for each object in scene....
                       
                    match obj.hits(&cool_ray) {                                                                         //-- check for a hit
                        Some(hit_rec) => {

                            if first_hit {                                                                              //-- if first hit for this ray, init hit ptr
                                first_hit = false;
                                closest_hit = hit_rec;
                            
                            } else {                                                                                    //-- if not, determine closer hit to cam and set
                                if (hit_rec.ip - cam.pos).mag() < (closest_hit.ip - cam.pos).mag() {
                                    closest_hit = hit_rec;
                                }
                            }
                        
                            color = phong_single_src(&closest_hit, &cam, &lights);                                        //-- calculate color w/ Phong model
                        }
                        None => {}
                    }
                }
                
                //scene[1].cen = m1 * scene[1].cen;     //-- can mutate scene data mid-render for effects                                                       
                //scene[2].cen = m1 * scene[2].cen;                                                           
                img_buffer.push(color.r); img_buffer.push(color.g); img_buffer.push(color.b);                           //--write color to buffer
                cool_ray.dir.x -= step;
            }
            
            cool_ray.dir.x = 0.5 * cam.w;
            cool_ray.dir.y += step;

            pbar.inc(1);
        }

        //-- cleanup progress bar
        pbar.finish();

        //-- convert buff and write to png
        println!("\nRender complete! Writing image...");
        let d: &[u8] = &img_buffer;
        writer.write_image_data(d).unwrap();

        //-- video gen... after each frame, rotate the ball by PI/60.0
        //scene[0].cen = m1 * scene[0].cen;                                                  
        //scene[1].cen = m1 * scene[1].cen;
        //scene[2].cen = m1 * scene[2].cen;

        //-- lifetime testing ...
        //println!("Closest hit lifetime test: {}", closest_hit.ip.stringy());
    }
}

// <<<<<<<<<<<<<<<<<<<<  HELPER TRAITS, STRUCTS, IMPLS, ETC. >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

//---- Phong Reflection / Shading Model
//-- Phong Light Model --> illumination at point = sum of ambient, diffuse, and specular light
//- for multiple lights, sum diffuse + specular with respect to each light
fn phong_single_src(hit_rec: &HitInfo, cam: &Camera, lights: &Vec<&mut PointLight>) -> Color{

    //-- temp/test material light constants
    let kd = 0.3;
    let ks = 0.5;
    let alpha = 50.0;                                  //- "shininess" factor
    
    //-- global ambient vals + ambient light calc
    let ia = Point{x:1.0 , y: 1.0, z: 1.0};             //- actually colors, but need to use floats
    let ka = 0.05;
    let ambient = ka * ia;

    //-- init illumination (ambient light + base object color)
    let mut illu = ambient + hit_rec.obj.def_color;

    //-- loop through lights --> calculate diffuse + specular contributions for each
    for light in lights{

        //-- calculate vectors for Phong model comp
        let n: Vec3 = hit_rec.norm.unit();                  //- normalized normal
        let lv: Vec3 = (light.pos - hit_rec.ip).unit();     //- hit pt -> light
        let rv: Vec3 = 2.0 * lv.dot(n) * n - lv;            //- perfect light reflection at hit pt
        let cv: Vec3 = (cam.pos - hit_rec.ip).unit();       //- hit pt -> camera "eye"
    
        //-- get respective light intensities
        let id = light.id;
        let is = light.is;

        //-- calc diffuse/specular light
        let diffuse = (kd * (lv.dot(n)) * is);
        let specular = (ks * (rv.dot(cv).clamp( 0.0, 1.0).powf(alpha) * is));       //-- need to clamp dot product to prevent dual specular

        // TODO: debug why phong lighting on z-axis is inverse, eg. light at -10 does not shine on
        // front of sphere as it should
        

        illu = illu + diffuse + specular;                                                 //-- sum lights + base color of hit object
    }

    //-- normalize color to RGB 0-255 space and return
    Color{r: (illu.x * 255.0) as u8 ,g: (illu.y * 255.0) as u8 , b: (illu.z * 255.0) as u8}
}

//----- Light Struct
struct PointLight{
    pos: Point,
    id: Point,
    is: Point

} impl PointLight {
    
    pub fn new() -> PointLight{
        PointLight{pos: Point::default(), id: Point{x:1.0,y:1.0,z:1.0}, is: Point{x:1.0,y:1.0,z:1.0}}
    }
}

//----- Material
struct Material{
    desc: String,
    kd: f64,
    ks: f64,
    alpha: f64
        
} impl Default for Material{
    
    fn default() -> Material{
        Material{
            desc: String::from("default"),
            kd: 0.3,
            ks: 0.5,
            alpha: 50.0
        }
    }
}

//---- Color
#[derive(Copy, Clone, Default)]
struct Color{
    r: u8,
    g: u8,
    b: u8
}


