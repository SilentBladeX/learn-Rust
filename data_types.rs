fn main() {
    // Integer
    //if we don't specify the variable length than it can bydefault i32

    let x: i64 = 32;

    //if we run our program in debug mode than it give an error
    let y: u8 = 255;
    //y=y+1;  //error
    // println!("Value of y is {y}");

    //Rust gives function to handle them explicity

    //overflowing
    let z: u8 = 255;
    let z = z.overflowing_add(1);
    println!("Value of z is {:?}", z);

    println!("------------");

    //wrapping
    let c: u8 = 255;
    let c = c.wrapping_add(1);
    println!("Value of c is {:?}", c);

    println!("------------");

    //saturating
    let d: u8 = 255;
    let d = d.saturating_add(1);
    println!("Value of d is {:?}", d);

    println!("------------");

    //checked
    let e: u8 = 255;
    let e = e.checked_add(1);
    println!("Value of e is {:?}", e);

    println!("------------");

    //Float
    // in rust if length is not specify than it can f64 bydefault

    let g: f64 = 50.00;
    let f: f64 = 34.77;
    println!("result of g+f is {:?}", g + f);

    println!("------------");

    //character
    // in rust char is unicode scalor value
    // it takes 4 bytes in memory

    let i: char = 'h';
    let j: char = '😻';
    println!("i= {i}");
    println!("j= {j}");

    println!("------------");
    //Bool

    let t = true;
    let u: bool = false;
    println!("t= {t}");
    println!("u= {u}");

    println!("------------");

    //tuple
    //we can access tuple by indexing or destuctring,unpacking
    //indexing

    let mut tup = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    tup.9 = 20;
    tup.8 = 15;

    println!(
        "value of 9 index is {}, value of 8 index is {} ",
        tup.9, tup.8
    );

    //destuctring
    let tup1 = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    let fruits = tup1;

    println!("{:?} ", fruits);

    println!("------------");

    //array
    //we can access array by indexing or slicing

    //indexing
    let mut array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    array[1] = 20;
    array[4] = 50;

    println!(
        "value in index 1 is {} and value in index 4 is {}",
        array[1], array[4]
    );

    println!("------------");

    //slicing

    let array1 = &array[2..7];
    println!("{:?}", array1);

    println!("------------");

    //by loop

    for i in &array {
        println!("{i}");
    }

    println!("------------");

    //coincise way
    let array2 = [0; 5];
    println!("{:?}", array2);
}
