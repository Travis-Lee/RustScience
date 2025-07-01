// An attribute to hide warnings for unused code
#![allow(dead_code)]

#[derive(Debug)]
pub struct Person{
    pub name: String,
    pub age: u8,
}

// A unit struct 
#[derive(Debug)]
pub struct Unit;

// A tuple struct 
pub struct Pair(pub i32, pub f32);

// A struct with two fields
#[derive(Debug)]
pub struct Point{
    pub x: f32,
    pub y: f32,
}

//Structs can be reused as fields of another struct 
#[derive(Debug)]
pub struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right icorners are in space
    pub top_left: Point,
    pub bottom_right: Point, 
}

