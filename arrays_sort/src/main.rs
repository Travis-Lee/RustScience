mod sorts; // include sorts.rs as module 

fn main() {
    // ex1: integer arrays 
    let int_array = [5, 2, 8, 3, 1];
    let sorted_ints = sorts::sort_array(&int_array);
    println!("Sorted ints: {:?}", sorted_ints);

    // ex2: float arrays 
    let float_array = [3.14, 2.71, 1.41, 0.577];
    let sorted_floats = sorts::sort_array(&float_array);
    println!("Sorted floats: {:?}", sorted_floats);

    // ex3: char arrays
    let char_array = ['z', 'a', 'k', 'b'];
    let sorted_chars = sorts::sort_array(&char_array);
    println!("Sorted chars: {:?}", sorted_chars);
}

