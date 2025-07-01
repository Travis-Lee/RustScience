fn main(){
    // Variables can be type annotated
    let logical: bool = true;
    println!("{logical}");
    
    //Regular annotation
    let a_float: f64=1.0;
    println!("{a_float}");

    //Suffix annotation
    let an_integer = 5i32;
    println!("{an_integer}");
   
    //A type can be also be inferred from context
    let mut inferred_type = 12; //Type i64 is inferred from another line 
    inferred_type =4294967296i64;
    println!("inferred_type:{}",inferred_type);

    //A mutable varaible's value can be changed 
    let mut mutable = 12;
    println!("mutable:{}",mutable);
    mutable =21;
    println!("after change mutable:{}",mutable);
 
   // a default will be used 
   let default_float = 3.0;
   let default_integer = 7;
   println!("default_float:{}",default_float);
   println!("default_integer:{}",default_integer);
   
   // Err, the type of a variable cant be changed 
   let  mutable = true;
   println!("after change mutable type:{}",mutable);

   // compound types - Array and Tuple; Array signature consists of Type T and length as [T;length].
   let my_array:[i32;5]=[1,2,3,4,5];
   println!("array:{:?}",my_array);

  //Tuple is a collection of values of different types and is constructed using parentheses().
  let my_tuple = (5u32, 1u8, true, -5.04f32); 
  println!("tuple:{:?}",my_tuple);

  // Mutability usage
  let _immutable_binding = 1;
  let mut mutable_binding = 1;
  
  println!("Before mutation:{}",mutable_binding);
  
  // if _immutable_binding += 1, the complier will throw a detailed diagnostic about mutability errors. 
  mutable_binding += 1; // it's ok let mutable_binding reassignment value 
  println!("after mutation:{}",mutable_binding);


}

