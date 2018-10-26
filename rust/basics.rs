// Compile and run this code: rustc basics.rs && ./basics

fn main() {
    println!("Lets do some stuff in rust.\n");

    // Create some static vars with different types of ints
    let x = 5 + 5;
    let y: i32 = 5 + 5;
    let z: u16 = 5 + 5;

    let a = 5.0 + 5.1;
    let b: f32 = 5.0 + 5.1;

    println!("Here is a interpolated string: {}", "Heck yeah!\n");
    println!("These are all the same number using different number types:");
    println!("    Int rust infered: {}, 32-bit int: {}, 16-bit unsigned int: {}", x, y, z);
    println!("    Float rust infered: {}, 32-bit float: {}", a, b);
    println!("");

    println!("Various math operations in rust:");
    println!("10 % 3: {}", 10 % 3); // Modulo
    println!("10 / 3: {}", 10 / 3); // Integer division
    println!("10.0 / 3.0: {}", 10.0 / 3.0); // Float division

    let mut mutable = 10;
    println!("Mutable variable: {}", mutable);
    mutable += 10;
    println!("Mutated mutable variable: {}", mutable);

    let multi_type_tup: (i8, u16, f32) = (-2, 3, 2.34);
    println!("Tuples can have muliple types: ({}, {}, {})", multi_type_tup.0, multi_type_tup.1, multi_type_tup.2);

    let array: [i32; 4] = [ 1, 3, 2, 5 ];
    println!("This is a printed array: [ {}, {}, {}, {} ]", array[0], array[1], array[2], array[3]);

    println!();

    // Rust can introduce arbitrary scopes
    {
        let scoped_var = 10;
        println!("This is a scoped_var: {}", scoped_var);
    }
    let scoped_var = 20;
    println!("We can redefine the scoped_var in the outside scope if we want: {}", scoped_var);

    println!("Lets use a function: 4 + 7 = {}", add_two_numbers(4,7));

    // Lets take a look at rust's control flow
    println!();

    if add_two_numbers(1, 2) == 3 {
        println!("Lets use a function and test it using an if statement");
    } else {
        println!("We are bad a math :(");
    }

    let number = if add_two_numbers(1, 1) == 2 {
        12
    } else {
        13
    };

    println!("There are also ternary statements in rust: {}", if number == 12 { 3 } else { 4 });
}


fn add_two_numbers(a: i32, b: i32) -> i32 {
    return a + b;
}

