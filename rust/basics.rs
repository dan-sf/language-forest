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

    // Rust loops!
    println!();

    let mut count = 0;
    let loop_result = loop {
        count += 1;
        if count == 5 {
            break 11
        }
    };

    println!("We can return values from loops: {}", loop_result);

    count = 0;
    while count < 2 {
        println!("While we are here: {}", count);
        count += 1;
    }

    for i in 0..2 {
        println!("For the win: {}", i);
    }

    println!();
    println!("Let's use a for loop to print the elements in an array");
    let arr = ["How", "now", "brown", "cow?"];
        print!("\t");
    for element in arr.iter() {
        print!("{} ", element);
    }
    println!("\n");

    // Memory management

    let mut st = String::from("Hello");
    st.push_str(", my friend");
    // Pass ownership of 'st' String to the 'modify_heap_string_append_char' function, then pass it back to 'st'
    st = modify_heap_string_append_char(st, '!');
    st.push_str(" Greatings from the heap.");
    println!("{}", st);

    // Here we clone a string making a full copy of the heap data, rust doesn't allow multiple vars
    // to point to the same data
    let s1 = String::from("One");
    let s2 = s1.clone();
    println!("This is s1: {}, and this is s2: {}, which is a clone of s1", s1, s2);

    // Since x is a primative type on the stack we can pass it to a function and still use it
    // afterwards. This would result in an error if x was a String.
    let x = 10;
    add_two_numbers(x, x);
    println!("This is x (we can still see it even though it was passed to a function): {}", x);

    // Here we create a string on the heap and pass a reference of it to a function to modify the
    // string in this scope, we can then use that string again after its reference was passed to
    // the function
    println!();
    let mut modify_me = String::from("One");
    println!("This is our unmodified string: {}", modify_me);
    modify_heap_string_add_str_using_ref(&mut modify_me, " time too many.".to_string());
    println!("This is our modified string: {}", modify_me);

    let slice_string = "Testing 123";
    println!("This is a sliced string: {},{}", &slice_string[0..7], &slice_string[7..]);
    println!("Same sliced string using ..=: {},{}", &slice_string[..=6], &slice_string[7..]);

    // Lets work with some structs

    println!("");

    struct Car {
        make: String,
        model: String,
        year: u32,
    }

    let a_car = Car {
        make: String::from("Ford"),
        model: String::from("Taurus"),
        year: 2010,
    };

    println!("Look at this {} {} {}! We can store it in a struct.", a_car.year, a_car.make, a_car.model);

    struct Vector3(i32, i32, i32);
    let v3 = Vector3(1, 2, 3);
    println!("Here we have a v3 as a tuple struct: ({}, {}, {})", v3.0, v3.1, v3.2);

    // We can use the derive annotation to help us debug print this struct
    #[derive(Debug)]
    struct Point {
        x: f32,
        y: f32,
        z: f32,
    };

    let p = Point { x: 1.0, y: 2.0, z: 3.0 };
    println!("Lets print out a struct that uses '#[derive(Debug)]' for debug printing: {:#?}\n", p);

    // We can also add methods to our structs

    struct Cup {
        volume: f32,
        is_full: bool,
        liquid_type: String,
        liquid_amount: f32,
    }

    impl Cup {
        fn fill(&mut self) {
            self.liquid_amount = self.volume;
            self.is_full = true;
        }

        fn print(&self) {
            println!("volume: {}, is_full: {}, liquid_type: {}, liquid_amount: {}",
                     self.volume, self.is_full, self.liquid_type, self.liquid_amount);
        }
    }

    let mut cup = Cup { volume: 5.0, is_full: false, liquid_type: String::from("water"), liquid_amount: 2.0 };
    print!("Here is the cup I have: "); cup.print();
    cup.fill();
    print!("Here is the cup filled: "); cup.print();

    // Next: Enums and Pattern Matching...
}

fn add_two_numbers(a: i32, b: i32) -> i32 {
    return a + b;
}

fn modify_heap_string_append_char(mut string: String, ch: char) -> String {
    // Here we can't just modify the string, we must return it since this function owns the string
    // that is passed in. So to prevent the memory from going away we return the string allowing
    // the caller to then own the string.
    string.push(ch);
    return string;
}

fn modify_heap_string_add_str_using_ref(string: &mut String, added: String) {
    // This function doesn't return anything, it just modifies a reference to a string
    string.push_str(&added);
}

