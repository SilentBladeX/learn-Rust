fn main(){

    //immutable variable    
    let x=10;
    // we cannot modify immutable variable
    // if we overwrite immutable variable than it can give an error
    // x=20;
    println!("the value of x is {x}");


    // mutable variable
    let mut y=10;
    y=20;
    println!("the value of y is {y}");


    // Shadowing 
    // In shadow we can also change datatype     
    let a=10;
    let a=a+1;
    {
        let a=20;
        println!("value of a in inner scope is {a}");
        
    } // now inner variable a destroys

    println!("value of a in outer scope is {a}");   //outer variable a active


    // constant
    // type annotation must be used in const variable
    // variable name must be greater
    // constant evaluate on compile time
    const VALUE_OF_PI:f64=3.14159;
    println!("value of pi is {VALUE_OF_PI}");
}