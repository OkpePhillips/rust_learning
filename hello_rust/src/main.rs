fn main() {
    println!("Hello, world!");
    let first_variable = "Godwin";

    println!("The value of  first_variable is {}", first_variable);

    // Mutabilit in rust
    let mut x = 5;
    println!("Initial valuel of x is {}", x);

    x = 10;
    println!("The mutated value of x is {}", x);

    let x = 54; // shadowing the previous varialbe name
    println!(" New variable x holds this value {}", x);
    
}
