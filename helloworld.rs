fn main() {
    /*
        The anatomy of a Rust program:
        1. The main function is the entry point of the program.
        2. The println! is a macro that prints a string to the screen.
        3. The ! indicates that this is a macro rather than a function.
        4. The string is surrounded by double quotes.
        5. The semicolon indicates that this is a statement, rather than an expression.
    ---------------------------------------------------------------------------
    What is a macro?
        A macro is an expression that has
        an exclamation mark (!) before the parenthesis () , i.e. macro_name ! ( );

    What are macros used for?
        They are used in metaprogramming, i.e., code that writes code.
        They look like functions in other system programming languages like C and C++,
        but instead of generating a function call like functions,
        they are expanded into source code that gets compiled with the rest of the program.
        In this way, they provide more run-time features.
        https://www.educative.io/api/collection/10370001/5248976175497216/page/4738741308489728/image/4990892697976832

    Types of Macros:
        Rust provides us with some built-in macros, like the println!() above,
        and users can define their own macros as well.
    ---------------------------------------------------------------------------

         */
    println!("Hello World!");
}
