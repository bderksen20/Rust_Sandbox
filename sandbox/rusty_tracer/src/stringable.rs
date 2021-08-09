// Bill Derksen - 8/21
//-- string and object description gathering trait. impl'd by camera, geometries, etc...

pub trait Stringable{
    fn stringy(&self) -> String;
}