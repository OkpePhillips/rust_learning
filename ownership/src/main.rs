fn main() {
    // A String can be created from a string literal using the from function
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}");

    let first = String::from("Ferris");

    // cloning avoids undefined behavior that would have resulted from a move of heap data
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

// This function takes a reference as a parameter
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
}
