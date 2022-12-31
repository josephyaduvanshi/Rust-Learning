fn main() {
    println!("{}", 1);
    /*
    In Rust, we cannot directly print numbers or variables within the println!() macro,
    unlike other languages. We need a placeholder {}.
    The placeholder is replaced by the value of the variable or number.
    It is similar like fString in Python.
     */
    println!("{} is a number", 1);
    println!("{value}", value= 7);
    println!("{1} {0} {2}", 1, 2, 3);
    //Placeholder for a Debug Trait implementation: {:?} or {:#?} for pretty print.
    // Debug trait is used to hold mutiple types of data in the parenthesis.
    // We can use it to placehold multiple types of values.
    println!("{:?}", (12, true, "hello"));

    // Placeholder for a float number: {:width.precision}
    println!("Pi is roughly {pi:.3}", pi= 3.141592);
    // Placeholder for a binary number: {:b}
    println!("Binary of {number} is {number:b}", number= 2);
    // Placeholder for a hexadecimal number: {:x}
    println!("Hexadecimal of {number} is {number:x}", number= 10);
    // Placeholder for a octal number: {:o}
    println!("Octal of {number} is {number:o}", number= 10);

    // Placeholder for a padding: {:width}
    // The width is the minimum number of characters that the placeholder will take.
    // If the value is smaller than the width, the value will be padded with spaces.
    // The padding is done on the left side. To pad on the right side, use {:>width}
    println!("{number:>width$}", number= 1, width= 6);

    // Placeholder for a padding with zeros: {:0width}
    // The width is the minimum number of characters that the placeholder will take.
    // If the value is smaller than the width, the value will be padded with zeros.
    // The padding is done on the left side. To pad on the right side, use {:0>width}
    println!("{number:0>width$}", number= 1, width= 6);
    /*
    1. Debug: {:?}
    2. Pretty Print: {:#?}
    3. Display: {}
    4. Binary: {:b}
    5. Octal: {:o}
    6. Lower Hex: {:x}
    7. Upper Hex: {:X}
    8. Lower Exp: {:e}
    9. Upper Exp: {:E}
    10. Pointer: {:p}
    11. Fill: {:width}
    12. Align: {:>width}
     */
    /*
        Printing Styles:
        Macro: println!() it is used to print the output to the console in new line.
        Macro: print!() it is used to print the output to the console in the same line.
        Macro: eprint!() it is used to print the error to the console in the same line.
        Macro: eprintln!() it is used to print the error to the console in new line.
     */
}
