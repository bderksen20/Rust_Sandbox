// Bill Derksen - 8/21
//-- some hittable geometries and objects to be used in scene

use crate::stringable::{Stringable};
use crate::hittable::{Hittable, HitInfo};
use crate::vmaths::{Point, Vec3};
use crate::ray::{Ray};
use crate::material::{Material};

//---- Sphere: follows eq (x-h)^2 + (y-i)^2 + (z-j)^2 = R^2
//-- vector form: ||x - c||^2 = R^2
#[derive(Default)]
pub struct Sphere{
    pub cen: Point,
    pub r: f64,
    pub def_color: Point

} impl Stringable for Sphere{

    fn stringy(&self) -> String{
        return String::from("center = ".to_owned() + &self.cen.stringy() + ", r = " + &self.r.to_string());
    }

} impl Hittable for Sphere{     //-- Ray xXx Sphere: ||x - c||^2 = R^2, solve for t where x = P(t) 

    fn hits(&self, r: &Ray) -> Option<HitInfo>{
        let mut ray = *r;
        ray.dir = ray.dir.unit();                   //-- NOTE: convert to unit vector for calculation... avoid extra comp?
        
        //println!("unit ray: {}", ray.stringy());

        let discrim = (2.0 * ray.dir.dot(ray.origin - self.cen)).powf(2.0) - (4.0 * ray.dir.mag().powf(2.0)) * ((ray.origin - self.cen).mag().powf(2.0) - self.r.powf(2.0));
        
        //println!("discriminant: {}", discrim);
        if discrim < 0.01 && discrim > 0.0 {                                            //-- 1x hit handling
        
            let t = -(ray.dir.dot(ray.origin - self.cen)) / ray.dir.mag().powf(2.0);          //- solve for t
            //Some(HitInfo{ip: ray.at(t), norm: ray.at(t) - self.cen, obj: &self})                        //- solve for incident pt + norm
            Some(HitInfo{ip: ray.at(t), norm: ray.at(t) - self.cen})

        } else if discrim >= 0.01{                                                      //-- 2x hit handling
    
            let t = (-2.0 * (ray.dir.dot(ray.origin - self.cen)) + discrim.sqrt())  / (2.0 * ray.dir.mag().powf(2.0));   //- solve for t (want closer hit)
            //Some(HitInfo{ip: ray.at(t), norm: ray.at(t) - self.cen, obj: &self})                                            //- solve for incident pt + norm
            Some(HitInfo{ip: ray.at(t), norm: ray.at(t) - self.cen})

        } else {                //-- miss handling
            None
        }
    }

    fn get_pos(&self) -> Point{
        self.cen
    }
} 

//-------------------- Rectangles

//-- XYRect: Rectangle on the z-plane
pub struct XYRect{
    pub z: f64,
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
} impl XYRect {

    // For rects that lie on the z-axis
    pub fn gen_z_rect(zz: f64, width: f64, height: f64) -> XYRect{
        XYRect{
            z: zz,
            x0: -0.5*width,
            x1: 0.5*width,
            y0: -0.5*height,
            y1: 0.5*height
        }
    }

    // General rect
    pub fn gen(zz: f64, xl: f64, xr: f64, yb: f64, yt: f64) -> XYRect{
        XYRect{z: zz, x0: xl, x1: xr, y0: yb, y1: yt}
    }

} impl Hittable for XYRect {

    fn hits(&self, ray: &Ray) -> Option<HitInfo> {

        //-- equation: z given, so can solve for t P(t)z = ray.origin.z + t*ray.dir.z
        let t: f64 = (self.z - ray.origin.z) / ray.dir.z;
        let x: f64 = ray.origin.x + t * ray.dir.x;
        let y: f64= ray.origin.y + t * ray.dir.y;

        //-- hit if within rectangle coordinate bounds 
        if (x <= self.x1 && x >= self.x0) && ( y <= self.y1 && y >= self.y0) {
            if ray.dir.z <= 0.0 && self.z <= 0.0 {      //-- TODO: had to add here to make this correct
                Some(HitInfo{ip: ray.at(t), norm: Point::gen(0.0, 0.0, 1.0)})
            } else {
                Some(HitInfo{ip: ray.at(t), norm: Point::gen(0.0, 0.0, -1.0)})
            }
            
        } else { None }
    }

    fn get_pos(&self) -> Point {
        Point::gen(0.0, 0.0, self.z)
    }
}

//-- XZRect: Rectangle that lies on the y-plane
pub struct XZRect{
    pub y: f64,
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
} impl XZRect {

    //-- on-axis rects
    pub fn gen_y_rect(yy: f64, width: f64, height: f64) -> XZRect{
        XZRect{
            y: yy,
            x0: -0.5*width,
            x1: 0.5*width,
            z0: -0.5*height,
            z1: 0.5*height
        }
    }

    //-- general rects
    pub fn gen(yy: f64, xl: f64, xr: f64, zn: f64, zf: f64) -> XZRect{
        XZRect{y: yy, x0: xl, x1: xr, z0: zn, z1: zf}
    }

} impl Hittable for XZRect {

    fn hits(&self, ray: &Ray) -> Option<HitInfo> {

        //-- equation: y given, so can solve for t P(t)y = ray.origin.y + t*ray.dir.y
        let t: f64 = (self.y - ray.origin.y) / ray.dir.y;
        let x: f64 = ray.origin.x + t * ray.dir.x;
        let z: f64= ray.origin.z + t * ray.dir.z;

        //-- hit if within rectangle coordinate bounds 
        if (x <= self.x1 && x >= self.x0) && ( z <= self.z1 && z >= self.z0) {
            if ray.dir.y <=0.0{          
                Some(HitInfo{ip: ray.at(t), norm: Point::gen(0.0, -1.0, 0.0)})    //-- note: currently just doing opposite of ray component for normal..     
            } else {
                Some(HitInfo{ip: ray.at(t), norm: Point::gen(0.0, 1.0, 0.0)})      
            }
        } else { None }
    }

    fn get_pos(&self) -> Point {
        Point::gen(0.0, self.y, 0.0)
    }
}

//-- YZRect: Rectangle that lies on the x-plane
pub struct YZRect{
    pub x: f64,
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
} impl YZRect {

    //-- on-axis rects
    pub fn gen_x_rect(xx: f64, width: f64, height: f64) -> YZRect{
        YZRect{
            x: xx,
            y0: -0.5*width,
            y1: 0.5*width,
            z0: -0.5*height,
            z1: 0.5*height
        }
    }

    //-- general rects
    pub fn gen(xx: f64, yb: f64, yt: f64, zn: f64, zf: f64) -> YZRect{
        YZRect{x: xx, y0: yb, y1: yt, z0: zn, z1: zf}
    }

} impl Hittable for YZRect {

    fn hits(&self, ray: &Ray) -> Option<HitInfo> {

        //-- equation: x given, so can solve for t P(t)x = ray.origin.x + t*ray.dir.x
        let t: f64 = (self.x - ray.origin.x) / ray.dir.x;
        let y: f64 = ray.origin.y + t * ray.dir.y;
        let z: f64= ray.origin.z + t * ray.dir.z;

        //-- hit if within rectangle coordinate bounds 
        if (y <= self.y1 && y >= self.y0) && ( z <= self.z1 && z >= self.z0) {
            if ray.dir.x <=0.0 {          
                Some(HitInfo{ip: ray.at(t), norm: Point::gen(-1.0, 0.0, 0.0)})    //-- note: currently just doing opposite of ray component for normal..     
            } else {
                Some(HitInfo{ip: ray.at(t), norm: Point::gen(1.0, 0.0, 0.0)})      
            }
        } else { None }
    }

    fn get_pos(&self) -> Point {
        Point::gen(self.x, 0.0, 0.0)
    }
}

//---- AABox: composed of 6 rects, 2 parallel for each plane
pub struct AABox{
    pub min_extent: Point,
    pub max_extent: Point,
    pub sides: Vec<Box<dyn Hittable>>   //-- TODO: only allow recs to be contained in here?
} impl AABox {

    //-- generate sides and init sides vec
    pub fn gen(min_corner: Point, max_corner: Point) -> AABox {

        let mut sides_v: Vec<Box<dyn Hittable>> = Vec::new();
        sides_v.push(Box::new(YZRect::gen(min_corner.x, min_corner.y, max_corner.y, min_corner.z, max_corner.z)));  //-- left
        sides_v.push(Box::new(YZRect::gen(max_corner.x, min_corner.y, max_corner.y, min_corner.z, max_corner.z)));  //-- right
        sides_v.push(Box::new(XYRect::gen(min_corner.z, min_corner.x, max_corner.x, min_corner.y, max_corner.y)));  //-- front
        sides_v.push(Box::new(XYRect::gen(max_corner.z, min_corner.x, max_corner.x, min_corner.y, max_corner.y)));  //-- back
        sides_v.push(Box::new(XYRect::gen(max_corner.y, min_corner.x, max_corner.x, min_corner.z, max_corner.z)));  //-- top
        sides_v.push(Box::new(XYRect::gen(min_corner.y, min_corner.x, max_corner.x, min_corner.z, max_corner.z)));  //-- bottom
    
        AABox{min_extent: min_corner, max_extent: max_corner, sides: sides_v}
    }

} impl Hittable for AABox {

    //-- iterate over surfaces checking for hit
    fn hits(&self, ray: &Ray) -> Option<HitInfo> {
        for side in &self.sides{
            match side.hits(&ray) {
                Some(hit_rec) => {
                    return Some(hit_rec);
                }
                None => {}
            }
        }
        None
    }

    fn get_pos(&self) -> Point{
        Point::new()
    }

}

//---- AABB - Axis-Aligned Bounding Box: 
pub struct BBox{
    pub cen: Point,
    pub w: f64,
    pub h: f64,
    pub d: f64,
    pub min_extent: Point,
    pub max_extent: Point
} impl BBox{
    
    pub fn gen(pos: Point, width: f64, height: f64, depth: f64) -> BBox{
        BBox{
            cen: pos, w: width, h: height, d: depth,
            min_extent: Point::gen(pos.x - (0.5*width), pos.y - (0.5*height), pos.z -(0.5*depth)),
            max_extent: Point::gen(pos.x + (0.5*width), pos.y + (0.5*height), pos.z +(0.5*depth)),
        }
    }
} impl Hittable for BBox {

    //-- AABB SLAB method... box is the intersection of 3 slabs (section between box side planes, x slab, y slab, z slab)
    fn hits(&self, ray: &Ray) -> Option<HitInfo> {
        
        let mut tmin = - f64::INFINITY;
        let mut tmax = f64::INFINITY;

        //-- x-axis slab
        if ray.dir.x as i64 != 0 {
            let mut tx1: f64 = (&self.min_extent.x - ray.origin.x) / ray.dir.x;
            let mut tx2: f64 = (&self.max_extent.x - ray.origin.x) / ray.dir.x;
            tmin = tmin.max(tx1.min(tx2));
            tmax = tmax.min(tx1.max(tx2));
        }

        //-- y-axis slab
        if ray.dir.y as i64 != 0 {
            let mut ty1: f64 = (&self.min_extent.y - ray.origin.y) / ray.dir.y;
            let mut ty2: f64 = (&self.max_extent.y - ray.origin.y) / ray.dir.y;
            tmin = tmin.max(ty1.min(ty2));
            tmax = tmax.min(ty1.max(ty2));
        }

        //-- z-axis slab
        if ray.dir.z as i64 != 0 {
            let mut tz1: f64 = (&self.min_extent.z - ray.origin.z) / ray.dir.z;
            let mut tz2: f64 = (&self.max_extent.z - ray.origin.z) / ray.dir.z;
            tmin = tmin.max(tz1.min(tz2));
            tmax = tmax.min(tz1.max(tz2));
        }

        //-- hit, need to calculate normal for shading
        //TODO: FIX
        if tmax >= tmin {
            
            let hit_pt = ray.at(tmin);
            let mut normal: Vec3 = Vec3::new();

            let normal = Point::gen(0.0,0.0,0.0);
            Some(HitInfo{ ip: hit_pt, norm: normal})

        } else { None }
    }

    fn get_pos(&self) -> Point {
        self.cen
    }

} impl Stringable for BBox{
    fn stringy(&self) -> String{
        return String::from("center: ".to_owned() + &self.cen.stringy() + "\nWxHxD = " + &self.w.to_string() + " x " + &self.h.to_string() + " x " + &self.d.to_string() + "\nmin extent: " + &self.min_extent.stringy() + "\nmax extent: " + &self.max_extent.stringy());
    }
}