fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("{THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let y = 10;
    let y = y + 2;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");

    // Changing the type of vlaue assigned to a variable through shadowing
    let spaces = "   "; // a string
    let spaces = spaces.len(); // an integer
    println!("{spaces}");

    // floating point are of two type f32 and f64(default)
    let float: f64 = 3.0; // f64
    let floatn: f32 = 10.0; // f32

    println!("The values are {float} and {floatn} respectively");

    // addition
    let sum = 5 + 10;

    //subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    // remainder
    let remainder = 43 % 5;

    println!(" The results are {}, {}, {}, {}, {}, and {} respectively", sum, product, difference, quotient, truncated, remainder);

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (a, b, c) = tup;

    println!("The value of a is {}", a);

    // Tuple can also be accessed by index
    let second_element = tup.1;
    println!("The second element is {}", second_element);

    // Array
    let new_array = [2, 3, 4, 5, 6];    
}


