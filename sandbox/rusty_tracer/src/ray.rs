// Bill Derksen - 8/21
//-- ray 

use crate::stringable::{Stringable};
use crate::vmaths::{Point, Vec3};

//---- ray: follows formula P(t) = O + td
#[derive(Copy, Clone)]
pub struct Ray{
    pub origin: Point,
    pub dir: Vec3

} impl Ray{

    pub fn at(&self, t: f64) -> Point{
        let p = self.origin + ( self.dir * t);
        return p;
    }

} impl Stringable for Ray{
    fn stringy(&self) -> String{
        return String::from("P(t) = ".to_owned() + &self.origin.stringy() + " + t" + &self.dir.stringy());
    }
}