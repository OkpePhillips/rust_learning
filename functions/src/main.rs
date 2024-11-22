fn main() {
    println!("Hello, world!");
    new_function();
    print_labeled_measurement(5, 'j');


    // Expressions evaluate to a result
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn new_function() {
    println!("This is another function");
}


fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}