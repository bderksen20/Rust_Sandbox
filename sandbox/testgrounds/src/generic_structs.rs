// Bill Derksen - 8/2021
// -- testing generic structures / inheritance alternatives for scene objects in rusty_tracer

use std::f32::consts::PI;
use std::vec;

pub fn generic_structs_test(){

    println!("\nTesting generic structs...");

    //-- init materials
    let mat1: Material = Material{desc: 1.0};
    let mat2: Material = Material{desc: 2.0};
    
    //-- init locations
    let pos1: Point = Point{x: 2.2, y: 1.5};
    let pos2: Point = Point{x: 1.4, y: 3.3};
    
    //-- init shape general data
    let shape1: ShapeData = ShapeData{mat: mat1, loc: pos1, name: String::from("shapearoo 1")};
    let shape2: ShapeData = ShapeData{mat: mat2, loc: pos2, name: String::from("shapearoo 2")};

    //-- init specific shapes...
    let square: Square = Square{sd: shape1, l: 5.0};
    let circle: Circle = Circle{sd: shape2, r: 2.0};

    //-- init vector of "Areable" generics...
    let mut shapes: Vec<Box<Areable>> = Vec::new();             //-- NOTE: size of "areable" unknown, needs box!
    shapes.push(Box::new(square));
    shapes.push(Box::new(circle));

    //-- calcualte area for each shape...
    for shape in shapes{
        println!("Area of a shape in vec is: {}", shape.area());
    }
}


//-- Each specific chape implements
trait Areable{
    fn area(&self)-> f32;
}

//-- General Shape Data Class -> Embed within each shape
//- eg. a Square HAS general Shape Data, as does a Circle!
struct ShapeData{
    mat: Material,
    loc: Point,
    name: String
}

//-- Specific Shapes -> have general shape data + their own specific data
struct Square{
    sd: ShapeData,
    l: f32
} impl Areable for Square{
    fn area(&self) -> f32{ 
        return (self.l * self.l);
    }
} impl Square {
    
}

struct Circle{
    sd: ShapeData,
    r: f32
} impl Areable for Circle{
    fn area(&self) -> f32{
        return ( PI * self.r.powf(2.0) );
    }
}

//-- Point: general shape has one of these for location
#[derive(Copy, Clone, Default)]
struct Point{
    x: f32,
    y: f32
}


//-- Material: general shape has one of these
#[derive(Copy, Clone)]
struct Material{
    desc: f32
} impl Default for Material{
    fn default() -> Material{
        Material{desc: 1.0}
    }
}

/* Generics Approach -> think i'll go with composition over this...
 
//-- Child-shapes implement this trait
trait Areable{
    fn area(&self)-> f32;
}

//-- Material: general shape has one of these
//#[derive(Copy, Clone)]
struct Material{
    desc: String
} impl Default for Material{
    fn default() -> Material{
        Material{desc: String::from("i'm a material!")}
    }
}

//-- Shape General Class: has a child shape 
struct Shape<T: Areable>{
    mat: Material,
    loc: Point,
    data: T
}

//-- Child Shapes: have geometry parameters
#[derive(Copy, Clone)]
struct Square{
    l: f32
} impl Areable for Square{
    fn area(&self) -> f32{ 
        return (self.l * self.l);
    }
}

#[derive(Copy, Clone)]
struct Circle{
    r: f32
} impl Areable for Circle{
    fn area(&self) -> f32{
        return ( PI * self.r.powf(2.0) );
    }
}

//-- Point: general shape has one of these for location
#[derive(Copy, Clone, Default)]
struct Point{
    x: f32,
    y: f32
}
*/
