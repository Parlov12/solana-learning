fn main() {
    println!("Hello, world!");

    // Variable Scope
    {   // s is not valid here
        let s = "hello"; // s is valid frin this point forward
        println!("{s}");
    }   // scope is over, s is no longer valid
    // println!("{s}"); -> this would cause error

    {
        let mut s = String::from("Hello");
        
        s.push_str(", world!");

        print!("{s}");
    } // Rust calls drop function that clears memory allocated by String s

    {
        let mut s = String::from("hello");
        s = String::from("New hello"); // Rust automatically call drop function on previous value
    }

    let mut new_string1 = gives_ownership();

    let new_string2 = takes_ownership_and_gives_back(new_string1);

    let new_string3 = String::from("Hello world 3");

    let get_new_string3 = print_and_return_string_reference(&new_string3);

    println!("New printing: {get_new_string3}");

}

// method that gives ownership
fn gives_ownership() -> String {

    let some_string = String::from("new string");

    some_string
}

// function that takes ownership
fn takes_ownership_and_gives_back(str: String) -> String {
    // ownership is moved into new_string
    let new_string = str;

    // new_string is returned thus ownership will be moved somewhere else
    new_string
}

// reference
fn print_and_return_string_reference(s: &String) -> &String {
    println!("{s}");
    s
}