// ================================
// Simple if-else statement
// ================================
fn main() {
    let num = 2;

    if num > 2 {
        println!("Number is greater than 2");
    } else {
        println!("Number is not greater than 2");
    }
}



// ================================
// if-else with boolean variable
// ================================
fn main() {
    let a = false;

    if a {
        println!("a is 1");
    } else {
        println!("a is 2");
    }
}




// ================================
// else-if ladder example
// ================================
fn main() {
    let num = 11;

    if num % 4 == 0 {
        println!("num is divisible by 4");
    } else if num % 3 == 0 {
        println!("num is divisible by 3");
    } else if num % 2 == 0 {
        println!("num is divisible by 2");
    } else {
        println!("num is divisible by 1");
    }
}




// ================================
// if expression returning a value (bool → int)
// ================================
fn main() {
    let flag = true;
    let num = if flag { 1 } else { 0 };

    println!("value of num is {}", num);
}




// ================================
// if expression returning string value
// ================================
fn main() {
    let num = 12;
    let result = if num % 2 == 0 { "EVEN" } else { "ODD" };

    println!("result {}", result);
}




// ================================
// Infinite loop with labels and nested loop
// ================================
fn main() {
    let mut counter = 0;
    let mut t_iteration = 0;

    'counting_up: loop {
        println!("\nIteration {}: Outer loop, counter = {}", t_iteration + 1, counter);
        let mut remaining = 10;

        loop {
            t_iteration += 1;
            println!("Iteration {}: Inner loop, remaining = {}", t_iteration, remaining);

            if remaining == 9 {
                println!("inner loop break at remaining = 9");
                break;
            }

            if counter == 2 {
                break 'counting_up; // labeled break
            }

            remaining -= 1;
        }

        counter += 1;
    }

    println!("end counter = {}", counter);
    println!("total iteration = {}", t_iteration);
}




// ================================
// while loop counting down
// ================================
fn main() {
    let mut num = 3;

    while num != 0 {
        println!("{} ", num);
        num -= 1;
    }

    println!("LIFT OFF!!!!");
}




// ================================
// while loop to iterate array
// ================================
fn main() {
    let arr = [2, 4, 6, 8, 90];
    let mut index = 0;

    while index < arr.len() {
        println!("{:?}", arr[index]);
        index += 1;
    }
}




// ================================
// For loop to iterate array elements
// ================================
fn main() {
    let a = [3, 5, 6, 8, 7];

    for i in a {
        println!("{:?}", i);
    }
}




// ================================
// For loop with reverse range
// ================================
fn main() {
    for num in (1..4).rev() {
        println!("{:?}", num);
    }

    println!("LIFT OFF!!!");
}
