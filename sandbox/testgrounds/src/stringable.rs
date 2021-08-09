// Bill Derksen - 8/21
// -- string utility trait for structs and objects... 


//---- Stringable: Implemented by objects to get description
pub trait Stringable{
    fn stringy(&self) -> String;
}



