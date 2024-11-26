fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2 // semicolon after break expression is optional
        }
    };

    println!("The result is {result}");

    // while loop

    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("Starting the show!!!");

    // For loop is used to iterate over a collection
    let lists = [10, 20, 30, 40, 50];

    for item in lists {
        println!("The value is: {item}");
    }

    // For loop can also be used in place of while loop using range
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("Liftoff!!!")
}
