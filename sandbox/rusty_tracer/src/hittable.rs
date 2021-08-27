// Bill Derksen - 8/21
//-- hittable trait implemented by geometries

use crate::ray::{Ray};
use crate::vmaths::{Point, Vec3};
use crate::geometry::{Sphere};
use crate::material::{Material};

pub trait Hittable{
    fn hits(&self, ray: &Ray) -> Option<HitInfo>;
    fn get_pos(&self) -> Point;
    //fn get_material(&self) -> &Material;
}

#[derive(Copy, Clone)]
pub struct HitInfo<'a>{ 
    pub ip: Point,
    pub norm: Vec3,
    pub hit_mat: &'a Material //TODO: add material to be populated on hit
}

//---- hit info: returned with a ray hit/intersection
/*
#[derive(Copy, Clone)]
pub struct HitInfo<'a>{ 
    pub ip: Point,
    pub norm: Vec3,
    pub obj: &'a Sphere     //TODO: change to Hittable
}
*/

