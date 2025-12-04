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

    let slice_first_word = String::from("Hello world");
    let sliced_first_word = first_word(&slice_first_word);
    println!("first word: {sliced_first_word}");

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

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

// slice type
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len() // if there is no empty space, return full string 
}