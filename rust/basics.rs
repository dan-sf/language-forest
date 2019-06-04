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
    println!("Lets print the Point struct that uses '#[derive(Debug)]' for debug printing: {:#?}\n", p);

    // We can also add methods to our structs

    #[derive(Debug)]
    struct Vector4(i32, i32, i32, i32);

    impl Vector4 {
        fn square_sum(&self) -> i32 {
            self.0 * self.0 + self.1 * self.1 + self.2 * self.2 + self.3 * self.3
        }

        fn get_equi_v4(val: i32) -> Vector4 {
            Vector4(val, val, val, val)
        }
    }

    let v4 = Vector4(4,3,2,1);

    println!("Let's debug print the struct: {:?}", v4);
    println!("Let's debug print the struct in a pretty format: {:#?}", v4);
    println!("We also have a method that can be called on v4: {}", v4.square_sum());
    println!("Let's call an associated function on Vector4 struct: {:?}", Vector4::get_equi_v4(10));
    println!();

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

    // Enums and Pattern Matching

    enum Suit {
        Hearts,
        Clubs,
        Diamonds,
        Spades,
    }

    let _s: Suit = Suit::Spades;
    let _c: Suit = Suit::Clubs;
    let _d: Suit = Suit::Diamonds;
    let _h: Suit = Suit::Hearts;

    // We can create enums of different data types
    enum DifferentTypes {
        Po(Point),
        St(String),
        Su(Suit),
    }

    // That enum of different types can be passed to functions
    fn take_different_types(diff: &DifferentTypes) {
        match diff {
            // When matching on enums that have values, we can pull those values out and operate on
            // them. Even run another match on that value
            DifferentTypes::Po(po) => { println!("We got a point: {:?}.", po); },
            DifferentTypes::St(st) => { println!("We got a string: {}.", st); },
            DifferentTypes::Su(su) => {
                match su {
                    Suit::Hearts => { println!("We got a suit: Hearts."); },
                    Suit::Clubs => { println!("We got a suit: Clubs."); },
                    Suit::Diamonds => { println!("We got a suit: Diamonds."); },
                    Suit::Spades => { println!("We got a suit: Spades."); },
                }
            },
        };
    }

    let dt_string = DifferentTypes::St(String::from("Testing"));
    let dt_suite = DifferentTypes::Su(_s);
    let dt_point = DifferentTypes::Po(p);
    take_different_types(&dt_string);
    take_different_types(&dt_suite);
    take_different_types(&dt_point);
    println!();

    // Option is just an Enum in the stdlib, ( enum Option<T> { Some(T), None, } )
    // Option/Some/None are all automatically included in scope so we don't need to use
    // std::option::Option::Some

    let actual_number = 10;
    let some_number = Some(15);
    let none_number = None;
    let some_number_full_stdlib = std::option::Option::Some(3);
    println!("To use values that are options we must unwrap them or handle both Some and None cases: {}",
             actual_number + some_number.unwrap() + some_number_full_stdlib.unwrap());

    // We can match on optional types just like any other type
    fn square_optional(val: Option<i32>) -> Option<i32> {
        match val {
            Some(i) => Some(i * i),
            None => None,
        }
    }

    // We can return anything we want from matching on an optional
    fn square_optional_return_int(val: Option<i32>) -> i32 {
        match val {
            Some(i) => i * i,
            None => -1,
        }
    }

    println!("Here we square an option (Some) with patern matching: {:?}", square_optional(some_number).unwrap_or(-1));
    println!("Here we square an option (None) with patern matching: {:?}", square_optional(none_number).unwrap_or(-1));
    println!();
    println!("Here we square an option (Some) with patern matching: {}", square_optional_return_int(some_number));
    println!("Here we square an option (None) with patern matching: {}", square_optional_return_int(none_number));
    println!();

    enum Subjects {
        Math,
        Science,
        English,
    }

    let math = Subjects::Math;
    let science = Subjects::Science;
    let english = Subjects::English;

    // We can use if let to just match on a single item, it is basically syntactic sugar for
    // matching with an _ to match all other values. Here we if let else and match with _ to do the
    // same exact thing. The if let doesn't enforce total matching so it can be handy when you only
    // want to match one thing
    fn subject_message(sub: Subjects) {
        match sub  {
            Subjects::English => { println!("Let's go read!"); },
            _ => { println!("Crunch those numbers!"); },
        }

        if let Subjects::English = sub  {
            println!("Let's go read!");
        } else {
            println!("Crunch those numbers!");
        }
    }
    subject_message(math);
    subject_message(english);
    subject_message(science);
    println!();

    // Modules

    // We can create modules within our code, we also could have placed this module outside of the
    // main function
    pub mod tree {
        // We need to make this function public if we want to use it in the outer scope
        pub fn grow() {
            println!("The tree is growing");
        }

        pub mod branch {
            pub fn branch_here() {
                // We can call functions from the parent module using the super keyword, this is
                // kind of like using '..' with relative paths in linux file systems
                super::parent_branch_function();
            }
        }

        // Notice that we don't need to us pub here, the child module will have access to this
        // function but we can't call this function directly outside of the tree module
        fn parent_branch_function() {
            println!("There is a branch here");
        }

        // We can create structs that are public but have private vars within them. This struct can
        // not be created outside the scope of this module
        pub struct EverGreen {
            pub name: String,
            size: i32,
        }

        // We need to create a generator function if we want to create an instance of EverGreen
        // outside this scope, however the caller won't be able to access the size attribute
        pub fn get_ever_green(name: &str) -> EverGreen {
            EverGreen {
                name: String::from(name),
                size: 32,
            }
        }

        pub fn get_ever_green_size(ever_green: &EverGreen) -> i32 {
            ever_green.size
        }

        // Enums on the other hand will be all public if the enum is public
        pub enum Size {
            Small,
            Medium,
            Large,
        }
    }

    tree::grow();
    tree::branch::branch_here();

    let ever_green_tree = tree::get_ever_green("my_tree");
    println!("This is the name of my tree: {}", ever_green_tree.name); // If we tried to access size we would get a compile error

    // To get the size we would need to have the module scope return it in some other way like
    // using the public function get_ever_green_size
    println!("This is the size of my tree: {}", tree::get_ever_green_size(&ever_green_tree));

    // We can anything within the Size enum because it was declaired public
    let _tree_small = tree::Size::Small;
    let _tree_medium = tree::Size::Medium;
    let _tree_large = tree::Size::Large;

    // The use key word is helpful for bringing modules into scope (would work if the tree module
    // was defined outside of the main scope, but since it isn't the below code would error)
    // use tree::branch;
    // branch::branch_here();

    // Collections

    println!();

    let mut vector = vec![1,2,3];
    println!("Here we have a vector: {:?}", vector);

    // Add items to the vector
    vector.push(4);
    vector.push(5);

    println!("We can add to the vector: {:?}", vector);

    let v = vector[2];
    println!("We can also grab items from the vector with indices vector[2]: {}", v);

    println!("Let's interate over that vector:");
    for i in &vector { // We need to use a reference here so we don't lose ownership of vector
        println!("{} ", i);
    }

    let s1 = String::from("Here");
    let s2 = String::from("we");
    let s3 = String::from("go.");
    let s4 = format!("{}, {}, {}", s1, s2, s3);
    println!("We can use the format macro to return strings similar to how the println macro prints them: {}", s4);

    // Note: Strings are more complex in rust than other languages, this is because rust forces the
    // programmer to deal with the complexity of strings upfront rather than have undefined
    // behavior in application code. All rust strings are Vec<u8> UTF-8 encoded strings. This means
    // that strings are really just an array of bytes and accessing those bytes can mean different
    // things for instance if the string has multi-byte chars. In that case you'd want to use the
    // .chars() method for iterating over the chars in a string. Strings in rust require more
    // thought on the programmer side.

    use std::collections::HashMap;

    let mut grades = HashMap::new();
    grades.insert(String::from("Billy"), "B+");
    grades.insert(String::from("Greg"), "A-");
    println!("Rust also has HashMap types: {:?}", grades);
    println!("We can iterate over the key value pairs:");
    for (k, v) in &grades {
        println!("{}: {}", k, v);
    }

    // We can do something similar to the Counter container in python by using
    // .entry(_).or_insert(_) methods for a hash map. It will return a ref that can be updated of
    // the value for the entry
    let mut counter = HashMap::new();
    for ch in "testing".chars() {
        let count = counter.entry(ch).or_insert(0);
        *count += 1;
    }
    println!("We can easily create a counter hash map in rust: {:?}", counter);

    // Errors

    // When performing an operation that could fail (like opening a file) rust will return a Result
    // enum which can be matched on. This forces the programmer to think about the failure case and
    // let rust know what to do
    use std::fs::File;
    let not_there = File::open("abc123.txt");
    match not_there {
        Ok(_) => println!("We got a handle to the file"),
        Err(e) => println!("Unfortunately, that file does not exist. Here is the error we saw: '{:?}'", e),
    };

    // Rust provides a panic! macro that can also be used to create a hard failure. It is useful in
    // situations where we know we want to crash the program due to the state that it has gotten
    // into

    // Generics

    println!();

    // generic_print function outside of main
    generic_print(10);
    generic_print(1.0);
    generic_print("test");

    // Here we create a trait that structs can then implement
    trait Fart {
        // This function does not have a default, every type that implements this trait must
        // implement this function
        fn name(&self) -> String;

        // Here we have a default implementation of the fart function
        fn fart(&self) -> String {
            String::from("FART")
        }
    }

    struct Snake {
        name: String,
    }

    impl Fart for Snake {
        fn fart(&self) -> String {
            String::from("SSSSSS")
        }

        fn name(&self) -> String {
            format!("{} the snake", self.name)
        }
    }

    struct Computer {
        name: String,
    }

    impl Fart for Computer {
        fn fart(&self) -> String {
            String::from("BEEEEEP")
        }

        fn name(&self) -> String {
            format!("{} the computer", self.name)
        }
    }

    struct Bear {
        name: String,
    }

    impl Fart for Bear {
        fn fart(&self) -> String {
            String::from("WOOOMMP")
        }

        fn name(&self) -> String {
            format!("{} the bear", self.name)
        }
    }

    struct Person {
        name: String,
    }

    // We don't implement the fart function, here we use the default defined in the trait block
    impl Fart for Person {
        fn name(&self) -> String {
            format!("{} the person", self.name)
        }
    }

    // Create some dudes
    let barry = Bear{name: String::from("Barry")};
    let sam = Snake{name: String::from("Sam")};
    let chris = Computer{name: String::from("Chris")};
    let peter = Person{name: String::from("Peter")};

    // We can take any type that implements the Fart trait as an arg to this function
    fn break_wind<T: Fart>(dude: T) {
        println!("Hi my name is {}.......{}", dude.name(), dude.fart());
    }

    // Lets... break wind
    break_wind(barry);
    break_wind(sam);
    break_wind(chris);
    break_wind(peter);

    // Lifetimes

    println!();

    let life_over_limit = [12, 34, 22, 4, 1000];
    let life_under_limit = [12, 34, 22, 4, 73];
    println!("Largest from life over limit: '{:?}', is '{}'", life_over_limit, top_limited(&life_over_limit, &120));
    println!("Largest from life under limit: '{:?}', is '{}'", life_under_limit, top_limited(&life_under_limit, &120));

    // Functional

    println!();

    // Here we can define a closure (anonymous functions)
    let closure = |foo| {
        println!("This is foo: {}", foo);
    };
    closure("bar");

    let closure_no_body = |val| println!("We don't need a body if we are only executing one {}", val);
    closure_no_body("line");

    let closure_no_args = || println!("Just use || if we have no args for the closure");
    closure_no_args();

    let main_scope_int = 22;
    let closure_print_from_main_scope = || println!("We can use vars defined in the parent scope: {}", main_scope_int);
    closure_print_from_main_scope(); // This would not work if we has used a function
    println!();

    // We can use iterators and closures together to perform functional (map/filter/reduce) types
    // of operations
    let functional = [1, 2, 3];
    let mapped: Vec<i32> = functional.iter().map(|x| x + 1).collect();
    let reduced: i32 = functional.iter().fold(0, |x, y| x + y);

    println!("Let's use map to add 1 to: '{:?}' -> '{:?}'", functional, mapped);
    println!("We can also fold (reduce in rust) to add the array's elements together: '{:?}' -> '{:?}'", functional, reduced);
    println!();

    let heap_int = Box::new(23);
    println!("We can use the box type to create a pointer to some data allocated on the heap, including things that would normally be on the stack (ie ints): {}", heap_int);

    // We can use the box pointer to create recursive types link linked list nodes
    struct LinkedListNode {
        val: i32,
        next: Option<Box<LinkedListNode>>,
    };

    let linked_list = 
        Some(LinkedListNode {
            val: 10,
            next: Some(Box::new(LinkedListNode {
                val: 5,
                next: Some(Box::new(LinkedListNode {
                    val: 2,
                    next: None,
                })),
            })),
        });

    print!("Here is a manually created linked list: ");
    let mut node = linked_list;
    while node.is_some() {
        let raw_node = node.unwrap();
        print!("{} -> ", raw_node.val);

        node = match raw_node.next {
            Some(data) => Some(*data),
            None => None,
        }

    }
    println!();

    // Struct pattern match

    struct Name {
        first: String,
        middle: String,
        last: String,
    }

    let name = Name { first: String::from("Tim"), middle: String::from("Tam"), last: String::from("Tom") };

    // We can use pattern matching to pull variables out of struct fields
    let Name { first: a, middle: b, last: c } = name;
    println!("First: {}, middle: {}, last: {}", a, b, c);


    // Match guards can be used to add some logic to matching

    let m = 10;
    let g = true;

    // Here we the first match arm will execute only if m is 9 or 10 and g is true
    match m {
        9 | 10 if g => println!("We matched with a guard."),
        _ => println!("Default case"),
    }

    // Here we the first match arm will execute only if m is 9 or 10 and g is false
    let m = 9;
    match m {
        9 | 10 if !g => println!("We matched with a guard."),
        _ => println!("Default case"),
    }

}


// This function won't compile if we don't use lifetimes ('a), it returns the top int from a list
// unless that value is over the limiter, then it will return the given limiter
fn top_limited<'a>(list: &'a [i32], limiter: &'a i32) -> &'a i32 {
    let mut one: &i32 = &list[0];
    for i in list.iter() {
        if i > one {
            one = i;
        }
    }

    // We are not sure how long limiter or one lives so we just specify all the args and output
    // have the same lifetimes, this informs the borrow checker to verify this (we aren't actually
    // changing the lifetimes of these variables)
    if one > limiter {
        limiter
    }
    else {
        one
    }
}

fn generic_print<T: std::fmt::Display>(val: T) {
    // Here we have a function that takes a generic type T and prints it out. The generic types for
    // this function must implement the Display trait so that we can run print on them, which is
    // why that is there. Traits can be shared across types which allows us to use this function
    // for a specific subset of types that implement that trait
    println!("This function can take any type: {}", val);
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

