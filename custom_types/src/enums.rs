// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.


#[derive(Debug)]
pub enum WebEvent{
    //An 'enum' variant may either be 'unit-like',
    PageLoad,
    PageUnload,
    
    //like tuple structs,
    KeyPress(char),
    Paste(String),

    //or c-like structures,
    Click {x: i64, y: i64},
}

#[derive(Debug)]
pub enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}
//creates a type alias
pub type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl VeryVerboseEnumOfThingsToDoWithNumbers {
   pub fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

//A function which takes a 'WebEvent' enum as an argument and returns nothing
pub fn inspect(event: WebEvent){
    match event{
        WebEvent::PageLoad => println!("page loaded"),    
        WebEvent::PageUnload => println!("Page unloaded"),
        
        //Destructure 'c' from inside the 'enum' variant
        WebEvent::KeyPress(c) => println!("Key pressed: '{}'", c),
        WebEvent::Paste(s) => println!("Pasted: \"{}\"", s),
    
        // Destructure 'Click' into 'x' and 'y'
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}


