fn main() {
    println!("Hello, world!");

    // Variable Scope
    {   // s is not valid here
        let s = "hello"; // s is valid frin this point forward
        println!("{s}");
    }   // scope is over, s is no longer valid
    // println!("{s}"); -> this would cause error

    {
        let s = String::from("Hello");
        
        s.push_str(", world!");

        println("{s}");
    } // Rust calls drop function that clears memory allocated by String s

    {
        let s = String::from("hello");
        s = String::from("New hello"); // Rust automatically call drop function on previous value
    }

}
