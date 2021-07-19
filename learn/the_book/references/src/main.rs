fn main() {
    
    //---- References    
    {
        //-- can pass variables as references so original var and data are not dropped with the ref.
        let s1 = String::from("hello");

        let len = calc_len(&s1);                            // &s1 pass ref to string
        println!("The length of '{}' is {}.", s1, len);

        fn calc_len(s: &String) -> usize {                  // receive ref to string
            s.len()
        }                                                   // s out of scope, but since it's a ref, nothing happens (if it was not a ref, s1 would have been "moved" and dropped)
        
    }


    //---- Mutable References
    {
        //-- can change original var by using a mutable reference, for example...
        let mut ss = String::from("Hi, ");

        change_str_by_ref(&mut ss);
        println!("Mutable ref ex: {}", ss);

        fn change_str_by_ref(stringy: &mut String){
            stringy.push_str("i've been altered by reference!");
        }

        //-- NOTE: can only have one mutable ref to a particular var in scope
        //--    below is illegal and won't compile...
        /*
         *      let mut s = String::from("hello");
         *      
         *      let r1 = &mut s;
         *      let r2 = &mut s;
         *      
         *      println!("{}, {}", r1, r2);
         */

        //-- why like this? prevent data races (two ptrs try to access same data at same time and at lesat one is trying to write)
         
        //-- NOTE: cannot have an immutable and mutable reference in same scope!
    }   


    //---- Dangling References
    {
        //-- rust tries not to allow dangling pointers to occur!!!! (reference to var that no longer exists in memory)

        //-- example that won't compile....
        /*
         *      fn dangle() -> &String {                // returns ref to string
         *
         *          let s = String::from("hello");      // s is new string
         *
         *          &s                                  // returns ref to s
         *      }                                       // scope over, s is dropped = dangling ptr!
         */

        //-- solution? return string directly w/ ownership moved out
    }

}
