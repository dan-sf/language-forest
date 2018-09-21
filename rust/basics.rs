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

}

