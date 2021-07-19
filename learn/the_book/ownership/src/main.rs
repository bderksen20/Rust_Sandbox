fn main() {
    
    //---- Variable Scope

        {                      // s is not valid here, it’s not yet declared
            let s = "hello";   // s is valid from this point forward

            // do stuff with s
        }                      // this scope is now over, and s is no longer valid


    //---- String Type (not hardcoded string literal)

        //-- stored on the heap, not stack (have unknown amt of text at compile time)
        let mut s = String::from("hello");

        //-- is mutable!
        s.push_str(", world!");     // push_str appends literal to String
        println!("string example: {}", s);


    //---- Memory Allocation

        //-- no garbage collector in Rust, manages heap memory based on ownership and scope
        {
            let s = String::from("hello"); // s is valid from this point forward

            // do stuff with s
        }                                  // this scope is now over, and s is no longer valid
        //-- end of scoper (curly braces) "drop" called to return mem
    

    //---- Variable and Data Interaction - Move

        //-- what happens when String goes out of scope with extra pointers?
        {
        let s1 = String::from("hello");
        let s2 = s1;                                // ptr to s1 data (var contains String ptr, length, and cap [stored on stack])

        // println!("move example: {}, move!", s1);
        }

        //-- results in error! s1 moved to s2 and Rust prevents using the invalidated reference !!!
        //-- this is to make memory management cleaner and safer!
        //-- note: similar to concept of shallow copying, but with additon of invalidating the origin ptrhence the name "move"
    

    //---- Variable and Data Interaction - Clone

        //-- deep copy: use the clone method - heap data is copied and we get a fresh full var
        {
            let s1 = String::from("henlo");
            let s2 = s1.clone();

            println!("clone example: s1 = {}, s2 = {}", s1, s2);

        }

    //---- Stack-Only Data - Copy
    
        //-- why is this code below (no cloning for y) valid?
        {
            let x = 5;
            let y = x;

            println!("copy example: x = {}, y = {}", x, y);
        }

        //-- int types have known size at compile and stored on stack -> copies inexpensive --> no shallow/deep copy diff

        //-- note: ints implement the "Copy" trait
            //-- if type has Copy trait, original variable still usable after assignment
            //-- types that implement Drop trait, they can't implement Copy too


    //---- Ownership and Functions
    
        //-- function variable passing similar to var assignment...
        {
            let ss = String::from("heello");     // s enters scope

            takes_ownership(ss);                  // s value moves into fxn...
                                                // no longer valid here!

            let x = 5;                          // x enters scope
            makes_copy(x);                      // i32 type has Copy, x still valid after
        }

        fn takes_ownership(stringy: String) {                           // stringy in scope
            println!("{fxn ownership ex - move}", stringy);
        }                                                               // stringy out of scope and dropped

        fn makes_copy(inty: i32) {                                      // inty in scope
            println!("fxn ownership ex - i32 cpy: {}", stringy);
        }                                                               // inty out of scope

    //---- Return Values and Scope

        //-- see references.....           
}


