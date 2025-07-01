mod structures;
use structures::Point;
fn main(){
    //create struct with field init shorthand
    let name = String::from("Peter");
    println!("name:{}",name);
    let age = 27;
    let peter = structures::Person {name, age};
    println!("peter:{:?}",peter);
   
    //Instantiate a "Point"
    let point: Point = structures::Point {x:5.2, y:0.4};
    let another_point: Point = structures::Point {x:10.3, y:0.2};

    //Access the fields of the point 
    println!("point coordinates:({},{})",point.x, point.y); 

    //Make a new point by using struct update syntax to use the fields of our other one
    let bottom_right = structures::Point { x:10.3, ..another_point };
    println!("bottom_right:({},{})",bottom_right.x, bottom_right.y); 
    
    //Destructure the point using a "let" binding
    let Point {x: left_edge, y: top_edge } = point;
    
    println!("Destructure Ponit:({},{})",left_edge, top_edge); 
    
    let _rectangle = structures::Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge},
        bottom_right: bottom_right,
    };
    println!("_rectangle:{:?}", _rectangle);    

    //Instantiate a tuple struct 
    let _unit = structures::Unit;
    println!("_unit:{:?}", _unit);    

    //Instantiate a tuple struct 
    let pair = structures::Pair(1, 0.1);
    
    //Access the fields of a tuple struct
    println!("pair contains:{:?} and {:?}", pair.0, pair.1);

    //Destructure a tuple struct
    let structures::Pair(integer, decimal) = pair;
   
    println!("pair contains:{:?} and {:?}", integer, decimal);
}
