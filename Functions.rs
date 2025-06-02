// ----------- Function Call and Return Demo -----------

fn main() {
    // Calling a simple function without parameters
    another_function();

    // Calling a function with parameters and storing the returned tuple
    let g = para_functions(5, 'A', 1.21, true);

    // Printing the returned tuple using debug formatting
    println!("{:?}", g);
}

// A simple function that prints a name
fn another_function() {
    println!("Ahad Chaudhary");
}

// A function with parameters of multiple types, returns a tuple
fn para_functions(a: i32, b: char, c: f64, d: bool) -> (String, char, f64, i32) {
    let a = a.to_string(); // convert integer to string
    let b = (b as u8 + 1) as char; // convert char to next char
    let c = c + 2.21; // add 2.21 to float
    let d = if d { 1 } else { 0 }; // convert bool to int (true = 1, false = 0)

    (a, b, c, d) // return a tuple
}






println!("----");

// ----------- Rust is Expression-Based Language -----------
// In Rust, functions, loops, and even `if` blocks return values.

fn main() {
    // A block expression that calculates and returns a value
    let y = {
        let x = 5;
        x + 1 // returns 6 from the block
    };
    println!("Value of y is {}", y); // Output: 6
}





println!("----");

// ----------- Using return in Loops and Functions -----------

fn main() {
    let a = ret_fun(); // Function returns a value
    println!("value of counter= {}", a);
}

// Function using return to exit early when condition is met
fn ret_fun() -> i32 {
    let mut counter = 0;

    loop {
        if counter == 10 {
            return counter * 2; // returns 20 when counter reaches 10
        }
        counter += 1;
    }
}






println!("----");

// ----------- Using loop with break to return value -----------

fn main() {
    let a = ret_loop(); // loop returns value to function, then to main
    println!("value of a is {}", a);
}

// Function where loop returns a value using break expression
fn ret_loop() -> i32 {
    let mut counter = 0;

    // break with value → loop returns result
    let result = loop {
        if counter == 10 {
            break counter * 2; // returns 20
        }
        counter += 1;
    };

    result // return the loop's result
}
