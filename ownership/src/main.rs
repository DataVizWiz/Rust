fn print_string(s: &String) {
    println!("{}", s);
}

fn modify_string(s: &mut String) {
    s.push_str(", World!");  // Modifying the data
}

fn main() {
    // my_string owns the String data
    let mut my_string = String::from("Hello, Rust!");  
    
    // OWNERSHIP
    // Ownership is transferred to another_string
    // let another_string = my_string;
    // println!("{}", my_string); // This line would cause a "variable moved" error

    // BORROWING
    print_string(&my_string);  // Borrowing my_string immutably
    println!("{}", my_string);  // Still valid because ownership wasn't taken

    // MUTABILITY
    modify_string(&mut my_string);  // Borrowing my_string mutably
    println!("{}", my_string);  // The change is reflected here
}
