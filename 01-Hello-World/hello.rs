fn main() {
    // using the println commant to print out Hello World!
    println!("Hello, world!");
    // Adding in a second line to print out I'm a Rustacean
    println!("I'm a Rustacean!")

    /* Printing is handled by a series of macros defined in std::fmt some of which include:

    format!: write formatted text to String
    print!: same as format! but the text is printed to the console (io::stdout).
    println!: same as print! but a newline is appended.
    eprint!: same as print! but the text is printed to the standard error (io::stderr).
    eprintln!: same as eprint! but a newline is appended.
    */
}