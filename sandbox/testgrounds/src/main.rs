// Bill Derksen - 8/21 
// -- testing grounds main -> import tests as modules and run functions

mod mut_vec;
mod generic_structs;
mod vmaths;
mod stringable;

use vmaths::Point;
use stringable::*;

fn main() {
    
    //-- mutability / borrow testing
    //mut_vec::mut_vec_test();

    //-- generics / struct composition testing
    //generic_structs::generic_structs_test();
    
    //-- ray-tracer modularization test
    let p: Point = Point::default();
    println!("Init + print point from module: {}", p.stringy());     //-- currently: stringable is its own crate, need to pull it in to implement


}



