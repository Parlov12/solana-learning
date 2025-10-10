fn main() {

    // variables 

    // variables are by default immutable
    let x = 5; // immutable
    let mut y = 19; // mutable

    println!("y = {}", y);

    y = 20;

    println!("y = {}", y);

    //Data Type
    let a : i32 = 42;   // 32bit integer
    let b : f64 = 3.14; // 64bit float
    let c : bool = true;
    let d : char = 'R';

    let tup : (i32, f64, u8) = (500, 3.14, 3); // tuple : (datatype_1, datatype_2, datatype_n, ...)
    let arr : [i32; 3] = [1, 2, 3]; // arrray : [type; size]

    let mut s = String::from("Hello"); // mutable string
    s.push_str(", world!");            // appending another string

    println!("{}", s);

    let slice:&str = &s[0..5]; // copies string from index 0 to index 5

    println!("Slice: {}", slice);

    // Control Flow

    let x = 10;

    // if statement
    if x > 10 {
        println!("X is greater than 10!");
    } else {
        println!("X is less than or equal to 10");
    }

    // loop
    loop {
        println!("Infinite loop!");
        break; // breaks the loop
    }
    
    // while loop
    let mut n = 3;
    
    println!("While loop:");

    while n > 0 {
        println!("{}", n);
        n -= 1;
    }

    // for loop
    for i in 1..4 {
        println!("{}", i);
    }

    // match (something like switch-case)

    let number = 3;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Default case"),
    }

    // functions

    fn add(a: i32, b: i32) -> i32 {
        return a + b; // return statement
    }

    fn square(a: i32) -> i32 {
        a * a // another way to define return statement
    }

    let result1 = add(5,4);
    let result2 = square(6);

    println!("Result1: {}", result1);
    println!("Result2: {}", result2);

    // structures and enums

    struct User {
        username : String,
        email : String,
        active: bool
    }

    let user1 = User {
        username : String::from("Petar"),
        email: String::from("petar@gmail.com"),
        active: true
    };

    println!("user1: {}", user1.username);

    // enum
    
    enum IpAddr  {
        V4(u8, u8, u8, u8), // four 8-bit integers
        V6(String),         // String type
        Unknown,            // No data
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    let unknwon = IpAddr::Unknown; // no associated data

    // ownersjhip

    let s3 = String::from("New string");

    {
        let s4 = s3; // s3 is moved into this block -> owner is s4 now
        println!("Inside block: {}", s4);
    } // s4 is dropped here and the emmory is freed
    // println!("{}", s3); // it would throw error since s3 is no longer valid - it was moved to s4 inside previous block

    // references and borowing

    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    let s1 = String::from("Hello");
    let len = calculate_length(&s1);

    fn change(s: &mut String) {
        s.push_str(", world");
    }

    let mut s = String::from("hello");

    println!("s: {}", s);

    change(&mut s); // it is neccessary to pass "mut" as well even we declared s as mutable

    println!("new s: {}", s);

    // Lifetime

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // 'a lifetime defines that both references x and y should have lifetime 'a -> so same lifetime
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("short");
        result = longest(string1.as_str(), string2.as_str()); // error occurs here?
    } // string2 goes out of scope here, but result still holds a reference to it
    //println!("The longest string is: {}", result); // trying to use result after string2 is dropped

    // Result

    fn check_length(s: &str, min: usize) -> Result<&str, String> {
        if s.chars().count() >= min { // chars() - return an iterator over the characters of a string 
            return Ok(s);
        } else {
            return Err(format!("'{}' is not long enoguh", s));
        }
    }

    let func_return = check_length("Je li string dovoljno dug?", 5);

    let a_str = match func_return {
        Ok(a_str) => a_str, // If Ok, bind the inner value (a_str) to the variable.
        Err(error) => panic!("Problem running 'check_length':\n {:?}", error), // If Err, crash the program with panic! and show the error message.
    };

    println!("{}", a_str);

    // option
    fn might_print(option: Option<&str>) {
        match option {
            Some(something) => println!("There is something inside this string: {}", something),
            None => println!("String is empty!"),
        }
    }

    let something : Option<&str> = Some("Some string!"); // we 
    let nothing : Option<&str> = None;

    might_print(something);
    might_print(nothing);

    // error handling

    // using result
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("Division with zero!"))
        } else {
            Ok(a/b)
        }
    }

    let c  = divide(5.0, 6.0);

    match c {
        Ok(value) => println!("Result is: {}", value),
        Err(error) => println!("An error occured: {}", error),
    } 

    // using option
    fn find_char(s: &str, c: char) -> Option<usize> {
        // (i, ch) - tuple unpacking of enumarate
        // chars() - returns an iterator over characters in string -> ['a', 'b', 'c', 'g']
        // enumarate() - wrapts that to both the index and the character -> (0, 'a'), (1, 'b'), (2, 'c'), (3, 'g') 
        for (i, ch) in s.chars().enumerate() {
            if ch == c {
                return Some(i);
            }
        }
        None
    }

    let some_string = String::from("Hello world 2");

    let c : Option<usize> = find_char(some_string.as_str(), 'o');
 
    
}