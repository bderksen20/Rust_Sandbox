// Bill Derksen - 8/21
//-- material struct and functions!!!

use crate::vmaths::{Point, Vec3};

pub struct Material{
    pub desc: String,
    pub kd: f64,
    pub ks: f64,
    pub alpha: f64,
    pub base_color: Point
        
} impl Default for Material{
    
    fn default() -> Material{
        Material{
            desc: String::from("default"),
            kd: 0.3,
            ks: 0.5,
            alpha: 50.0,
            base_color: Point::gen(0.2, 0.2, 0.2)
        }
    }
} impl Material {

    //-- Initializers
    pub fn shiny_red() -> Material{
        Material{
            desc: String::from("default shiny red"),
            kd: 0.3,
            ks: 0.5,
            alpha: 50.0,
            base_color: Point::gen(0.6, 0.2, 0.2)
        }
    }

    pub fn shiny_blue() -> Material{
        Material{
            desc: String::from("default shiny blue"),
            kd: 0.3,
            ks: 0.5,
            alpha: 50.0,
            base_color: Point::gen(0.2, 0.2, 0.6)
        }
    }

    pub fn shiny_green() -> Material{
        Material{
            desc: String::from("default shiny green"),
            kd: 0.3,
            ks: 0.5,
            alpha: 50.0,
            base_color: Point::gen(0.2, 0.6, 0.2)
        }
    }
}