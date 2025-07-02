mod structures;
mod enums;
mod constants;

use structures::Point;
use std::env;
use enums::{Stage, Role};

fn parse_stage(input: &str) -> Option<Stage> {
    match input.to_lowercase().as_str() {
        "beginner" => Some(Stage::Beginner),
        "advanced" => Some(Stage::Advanced),
        _ => None,
    }
}

fn parse_role(input: &str) -> Option<Role> {
    match input.to_lowercase().as_str() {
        "student" => Some(Role::Student),
        "teacher" => Some(Role::Teacher),
        _ => None,
    }
}


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

    // unit test for enums 
    let pressed = enums::WebEvent::KeyPress('x');
    println!("pressed:{:?}", pressed);

    // `to_owned()` creates an owned `String` from a string slice
    let pasted = enums::WebEvent::Paste("my_text".to_owned());
    println!("pasted:{:?}", pasted);
    let click   = enums::WebEvent::Click { x: 20, y: 80 };
    println!("click:{:?}", click);
    let load    = enums::WebEvent::PageLoad;
    println!("load:{:?}",load);
    let unload  = enums::WebEvent::PageUnload;
    println!("unload:{:?}", unload);
    
    enums::inspect(pressed);
    enums::inspect(pasted);
    enums::inspect(click);
    enums::inspect(load);
    enums::inspect(unload);
    
    let operation= enums::Operations::Add;
    let result = operation.run(10, 5);
    println!("Result: {}", result);  

    let operation2 = enums::Operations::Subtract;
    let result2 = operation2.run(10, 5);
    println!("Result: {}", result2); 

    // unit test for constants 
    let n = 16;
    //Access constant in the main thread
    println!("This is:{}", constants::LANGUAGE);
    println!("The threshold is:{}", constants::THRESHOLD);
    println!("{} is {}", n, if constants::is_big(n) { "big" } else { "small" });
    // Note: THRESHOLD is const value, cannot modify const variant "THRESHOLD = 25";

    //unit test for use 
   
    // Explicitly `use` each name so they are available without manual scoping.
    use crate::enums::Stage::{Beginner, Advanced};
    
    //Automatically 'use' each name inside 'Role'
    use crate::enums::Role::*;

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!(
        "Usage: {} <role> <stage>\n  role: student | teacher\n  stage: beginner | advanced",
        args[0]
        );
        std::process::exit(1);
    }

    // Equivalent to `Stage::Beginner`.
    let stage = parse_stage(&args[1]).unwrap_or_else(|| {
        eprintln!("Invalid stage: {}", args[1]);
        std::process::exit(1);
    });

    // Equivalent to `Role::Student`.
    let role = parse_role(&args[2]).unwrap_or_else(|| {
        eprintln!("Invalid role: {}", args[2]);
        std::process::exit(1);
    });

    println!("stage:{:?}", stage);
    println!("role:{:?}", role);

    match stage {
        // Note the lack of scoping because of the explicit `use` above.
        Beginner => println!("Beginners are starting their learning journey!"),
        Advanced => println!("Advanced learners are mastering their subjects..."),
    }

    match role {
        // Note again the lack of scoping.
        Student => println!("Students are acquiring knowledge!"),
        Teacher => println!("Teachers are spreading knowledge!"),
    }
}
