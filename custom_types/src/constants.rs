#![allow(dead_code)] 

//Globals are declared outside all other scope
pub static LANGUAGE: &str = "RUST";
pub const THRESHOLD: i32 = 10;

pub fn is_big(n: i32) -> bool{
    //Access constant in some function 
    n > THRESHOLD
}




