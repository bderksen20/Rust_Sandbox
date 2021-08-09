// Bill Derksen - 8/21
//-- some hittable geometries to be used in scene

use crate::stringable::{Stringable};
use crate::hittable::{Hittable, HitInfo};
use crate::vmaths::{Point, Vec3};
use crate::ray::{Ray};

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
            Some(HitInfo{ip: ray.at(t), norm: ray.at(t) - self.cen, obj: &self})                        //- solve for incident pt + norm

        } else if discrim >= 0.01{                                                      //-- 2x hit handling
    
            let t = (-2.0 * (ray.dir.dot(ray.origin - self.cen)) + discrim.sqrt())  / (2.0 * ray.dir.mag().powf(2.0));   //- solve for t (want closer hit)
            Some(HitInfo{ip: ray.at(t), norm: ray.at(t) - self.cen, obj: &self})                                            //- solve for incident pt + norm

        } else {                //-- miss handling
            None
        }
    }

    fn get_pos(&self) -> Point{
        self.cen
    }
} 

//---- Cube:
pub struct Cube{
    pub cen: Point,
    pub len: f64
} impl Hittable for Cube{
    
    //-- cube X ray intersection
    //- max{ |x-x0|, |y-y0|, |z-z0|} = a -> where edge length = 2a
    fn hits(&self, ray: &Ray) -> Option<HitInfo> {
    

        // cube at <-2, 0, 2> w edge = 2
        // pt that will be on -> <-3, 0, 2>
        // max{|x+2|, |y|, |z-2|} = 1

        None
    }

    fn get_pos(&self) -> Point {
        self.cen
    }
}