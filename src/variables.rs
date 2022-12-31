/*
What are variables in rust?
Variables are used to store values in memory.
In Rust, variables are immutable by default.
We can make a variable mutable by using the keyword mut before the variable name.
We can also use the keyword const to declare a constant.
Constants are always immutable and their type must be annotated.
We can use the keyword let binding to declare a variable.
The let keyword is used to create a variable binding.
The syntax of let binding is:
let <variable_name> = <value>;
The value can be a literal value or an expression.
The type of the variable is inferred from the value.
We can also explicitly specify the type of the variable by using the colon (:) operator.
The syntax of let binding with type annotation is:
let <variable_name>: <type> = <value>;
The type annotation is optional.

What are the rules for naming variables in rust?
1. Variable names must start with a letter or an underscore (_).
2. Variable names can contain only letters, numbers, and underscores.
3. Variable names are case-sensitive.
4. Variable names cannot be a reserved keyword.
5. Variable names cannot contain spaces.
6. Variable names cannot contain special characters.
7. Variable names cannot contain the ASCII character 0.

What are the data types in rust?
1. Scalar Types:
    1. Integer Types:
        1. Signed Integer Types:
            1. i8 // 8-bit signed integer // -128 to 127
            2. i16 // 16-bit signed integer // -32768 to 32767
            3. i32 // 32-bit signed integer  // -2147483648 to 2147483647
            4. i64 // 64-bit signed integer // -9223372036854775808 to 9223372036854775807
            5. i128 // 128-bit signed integer // -170141183460469231731687303715884105728 to 170141183460469231731687303715884105727
            6. isize // pointer-sized signed integer // -2^(n-1) to 2^(n-1) - 1, where n is the number of bits in the pointer
        2. Unsigned Integer Types:
            1. u8 // 8-bit unsigned integer // 0 to 255
            2. u16 // 16-bit unsigned integer // 0 to 65535
            3. u32 // 32-bit unsigned integer // 0 to 4294967295
            4. u64 // 64-bit unsigned integer // 0 to 18446744073709551615
            5. u128 // 128-bit unsigned integer // 0 to 340282366920938463463374607431768211455
            6. usize // pointer-sized unsigned integer // 0 to 2^n - 1, where n is the number of bits in the pointer
    2. Floating Point Types:
        1. f32 // 32-bit floating point number // 1.2E-38 to 3.4E+38
        2. f64 // 64-bit floating point number // 2.3E-308 to 1.2E+308
    3. Boolean Type:
        1. bool // true or false // 1 byte
    4. Character Type:
        1. char // Unicode Scalar Value // 4 bytes
2. Compound Types:
    1. Tuple Type:
        1. (T1, T2, T3, ...) // A tuple is a fixed length list of elements.
    2. Array Type:
        1. [T; N] // An array is a fixed length list of elements.
    3. Slice Type:
        1. [T] // A slice is a variable length list of elements.

What is binding?
Rust refers to declarations as bindings as they bind a name at the time of creation.
let is a kind of declaration statement.
let x = 5; // x is bound to 5
let x = 6; // x is rebound to 6
let x = x + 1; // x is rebound to 7
 */

pub(crate) fn variables() {
    let x = 5; // x is bound to 5
    println!("The value of x is: {}", x);
    let x = 6; // x is rebound to 6
    println!("The value of x is: {}", x);
    let x = x + 1; // x is rebound to 7
    println!("The value of x is: {}", x);
    let mut y = 5; // y is bound to 5
    println!("The value of y is: {}", y);
    y = 6; // y is rebound to 6
    println!("The value of y is: {}", y);
    let (a, b) = (1, 2); // a is bound to 1 and b is bound to 2
    println!("The value of a is: {}\nThe value of b is: {}", a, b);
    // shadowing example: Shadowing is a way to declare a new variable with the same name as a previous variable outside of scope
    // Shadowing is different from rebinding because we’ll get a compile-time error if we accidentally try to rebind with let.
    // By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
    {
        let x = 5;
        println!("The value of x is: {}", x);
    }
        //explicitly define an integer
    let a:i32 = 24;
    let b:u64 = 23;
    let c:usize = 26;
    let d:isize = 29;
    //print the values
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);
    #[allow(unused_variables, unused_mut)]
   //define an array of size 4
   let arr:[i32;4] = [1, 2, 3, 4]; 
   // initialize an array of size 4 with 0
   let arr1 = [0 ; 4]; 
   println!("arr: {:?}", arr);
    println!("arr1: {:?}", arr1);

}
