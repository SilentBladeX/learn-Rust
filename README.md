# learn-Rust

# hello world 

## fn main() { ... }
This defines the main function, which is the entry point of every Rust program. The code inside this function is what runs when the program is executed.

## println!("Hello World");
println! is a macro (note the !) used to print text to the console.
Here, it displays Hello World.


---


# Variables And Mutable

This Rust program demonstrates four important concepts:

Immutable variables (cannot be changed)
Mutable variables (can be changed)
Shadowing (redeclaring a variable with the same name)
Constants (fixed values that never change)

## 1. Immutable Variable

The program starts by declaring a variable x as immutable, with a value of 10. Immutable variables cannot be changed after being assigned.
If you try to write x = 20;, it will give a compile-time error.

A print statement is used to display the value of x.

## 2. Mutable Variable

Next, a mutable variable y is declared. It is first assigned the value 10, and then changed to 20.
The mut keyword allows the variable to be modified later in the program.

The updated value of y is printed.

## 3. Shadowing

A variable named a is declared and assigned the value 10.
Then, another variable with the same name a is declared, which stores the result of a + 1, so its value becomes 11.
This is called shadowing — it replaces the previous variable in the same scope.

Inside a new block (enclosed in {}), another variable a is declared and assigned the value 20.
Within this block, the value of a is 20.
When the block ends, this inner a is destroyed, and the outer a with value 11 becomes active again.

## 4. Constant

A constant named VALUE_OF_PI is declared with a value of 3.14159.
Important rules for constants in Rust:
You must provide a type (in this case, f64)
Constant names are usually written in uppercase
Constants are evaluated at compile time and cannot be changed


---


# Data types (int,float,char,bool,array,tuple)

This Rust program demonstrates the use of multiple fundamental data types and how to safely perform operations (especially with integers) using built-in Rust methods.

## 🧮 Integer Types
By default, integers in Rust are of type i32.
If a type is explicitly declared, like i64 or u8, the variable will hold that specific type.
For example, a variable declared as u8 can store values from 0 to 255.
If we try to exceed the limit (e.g., assign 256 to a u8), it will result in an overflow error in debug mode.

## 🧠 Handling Integer Overflows
Rust provides different methods to safely handle overflows:

### Overflowing Add
Returns a tuple: (result, did_overflow)
The result wraps around from the beginning
.
### Wrapping Add
Wraps around without any panic or error.
255 + 1 becomes 0.

### Saturating Add

If the result exceeds the maximum, it "saturates" at the max value.
255 + 1 remains 255.

### Checked Add

Returns None if there is an overflow.
Otherwise, returns Some(value).

These methods provide safe control over integer operations.

## 🔢 Floating Point Numbers

By default, floats are f64 (64-bit precision).
You can add two floats just like in any other language.
Rust supports decimal numbers with high accuracy.

## 🔡 Characters

Rust uses char type to store Unicode scalar values.
This means you can store both regular characters and emojis.
Each char takes 4 bytes in memory.

## ✅ Boolean

Booleans can be either true or false.
Used for logical conditions and decision-making.

## 📦 Tuples

Tuples can hold multiple values of different types.
You can access tuple elements using indexes like .0, .1, etc.
Tuples can also be destructured into individual variables.
Example: You can assign all tuple values to a single variable or split them.

In this program, one tuple is modified (indexes 9 and 8), and another tuple is destructured and printed.

## 🔢 Arrays

Arrays in Rust have a fixed length and type.
Values can be accessed or modified using indexing (array[0], array[1], etc.).
You can also create slices to view a portion of the array, e.g., from index 2 to 6.
Arrays can be printed using a loop.
You can also create an array using a shortcut: [0; 5] creates an array with five elements, all set to 0.



---



# Functions


This Rust program explores several core concepts in the language:

Calling functions with and without parameters
Returning tuples from functions
Expression-based evaluation in Rust
Using return in loops and functions
Breaking loops with values

## 🧩 Function Calls and Return Values

1. Calling a Function Without Parameters
A basic function is called from main that simply prints a name (Ahad Chaudhary).

2. Calling a Function With Parameters
A more advanced function accepts multiple parameter types:

An integer
A character
A float
A boolean

These values are manipulated inside the function:
Integer is converted to a string
Character is incremented to the next Unicode character
Float is increased by 2.21
Boolean is converted to an integer (true → 1, false → 0)

The function returns a tuple containing the processed values, which is printed using debug formatting ({:?}).

## 📐 Expression-Based Language
Rust is an expression-based language, which means most code blocks can return a value.

### Example:

A variable y is assigned the result of a code block:

Inside the block, x = 5
The block ends with x + 1, which evaluates to 6
So, y = 6

No semicolon at the end of the expression means it is returned by the block.

### 🔁 Using return in Functions with Loops

Function with Explicit Return
A function called ret_fun contains a loop that:
Increments a counter from 0
When the counter reaches 10, it returns counter * 2 using return
Hence, the returned value is 20
This value is then printed in main.

### 🔄 Using break to Return Value from Loop

Rust allows loops to return values using break:
In the ret_loop function, a loop is defined
When the counter reaches 10, it breaks with counter * 2
This breaks out of the loop and returns the value 20 to the function
This is a clean and Rust-idiomatic way to get values out of loops.



---


# COntrol Flows


## Simple if-else Statement

A number is checked to see if it is greater than 2.
If the condition is true, one message prints.
Otherwise, the else block executes.

## 🔘 Boolean Condition in if-else

A boolean variable is used directly inside an if.
If it's true, it prints one value.
If it's false, it prints something else.

## 🪜 else-if Ladder

A number is checked for divisibility using else-if statements.
First checks divisibility by 4, then 3, then 2.
If none match, the default else prints a fallback message.

## 📦 if as an Expression (Boolean to Integer)

Rust allows if to return values.
A boolean flag determines whether to assign 1 or 0 to a variable.
The value is then printed.

## 🔠 if Returning a String

Based on whether a number is even or odd, the result is either "EVEN" or "ODD".
The chosen string is stored in a variable and printed.

## 🔁 Infinite Loop with Labels and Nested Loop

Two loops: an outer (counting_up) and an inner loop.
The inner loop runs until remaining == 9.
If the outer counter becomes 2, a labeled break (break 'counting_up) is used to exit the entire loop structure.
At the end, the outer counter and total inner iterations are printed.

## ⏳ while Loop Counting Down

Starts from 3 and counts down to 1.
After the loop finishes, prints "LIFT OFF!!!".
A good example of countdown logic.

## 📄 while Loop to Iterate an Array

Uses a while loop to print all elements in an array.
Index is manually updated each iteration.

## 🔁 for Loop to Iterate Array

A much cleaner way to print each item of an array.
Automatically iterates through each value without needing manual index handling.

## 🔁 for Loop with Reverse Range

Uses (1..4).rev() to create a reverse range (3, 2, 1).
This is ideal for countdown-style logic.

