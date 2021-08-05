// Bill Derksen - 8/21
// -- Attempt to make sense of vector of mutable references for use in ray tracer!
// --- why? so scene + light data can be altered mid render for cool effects >:D 

// Notes...
// -- issue resolved! related to the lifetime of structs containing some borrows of the scene objects.

use::std::vec;
use::std::rc;

pub fn mut_vec_test(){

    println!("\nIn mut vec test...");
    
    println!("\n\nPlease take your seats as the fantastic Mutable Foos take the stage!!!\n");

    //-- create some mutable foos
    let mut f1: Foo = Foo{f: 0, stuff: FooStuff{yum: String::from("yummy foo!")}};
    let mut f2: Foo = Foo{f: 1, stuff: FooStuff{yum: String::from("yummy goo!")}};
    let mut f3: Foo = Foo{f: 2, stuff: FooStuff{yum: String::from("yummy you!")}};

    //-- create a vec of mut foo refs
    let mut foo_vec: Vec<&mut Foo> = Vec::new();
    foo_vec.push(&mut f1);
    foo_vec.push(&mut f2);
    foo_vec.push(&mut f3);


    //-- iterate over foos >:)
    //-- ERR1 NOTE: need &foo_vec to borrow, otherwise the vector is MOVED and DROPPED after for loop
    //-- ERR2 NOTE: compile error, cannot write to &foo in iteration -> FIX: &mut foo_vec
    for x in 0..3 {
        println!("\nPerformance #{}...", x);
        for foo in &mut foo_vec{
            println!("Foo {} says {}", foo.f, foo.stuff.yum);
            match foo.f {
                0 => { foo.stuff.yum = String::from("thank foo!"); }
                1 => { foo.stuff.yum = String::from("thank goo!"); }
                2 => { foo.stuff.yum = String::from("thank you!"); }
                _ => { println!("uh oh..."); }
            }
        }   
    }

    println!("\nGive a round of applause for the Mutable Foos as they give their parting words...\n");
    
    //-- iterate over foos x2 (make sure they didn't get dropped / booed off the stage)
    for foo in &foo_vec{
        println!("Foo {} says {}", foo.f, foo.stuff.yum);
    }
}

struct Foo{
    f: u32,
    stuff: FooStuff
}

struct FooStuff{
    yum: String
}
