// Bill Derksen - 8/21
//-- hittable trait implemented by geometries

use crate::ray::{Ray};
use crate::vmaths::{Point, Vec3};
use crate::geometry::{Sphere};

pub trait Hittable{
    fn hits(&self, ray: &Ray) -> Option<HitInfo>;
    fn get_pos(&self) -> Point;
}

//---- hit info: returned with a ray hit/intersection
#[derive(Copy, Clone)]
pub struct HitInfo<'a>{ 
    pub ip: Point,
    pub norm: Vec3,
    pub obj: &'a Sphere
}