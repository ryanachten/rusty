fn main() {
    let x = 5;
    let x = x + 1; // Shadows the 1st declaration

    {
        let x = x * 2; // Shadows the 2nd declaration
        println!("The value of x in the inner scope is: {x}"); // Would expect the 3rd x to be printed
    }

    println!("The value of x is: {x}"); // Would expect the 2nd x to be printed
}
