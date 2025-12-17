fn main() {
    println!("Hello, world!");

    let new_user = User {
        name: String::from("Petar"),
        surname: String::from("Parlov"),
        active: true,
        email: String::from("petarparlov@gmail.com"),
        sign_in_count: 0
    };

    println!("Created new user:\nname: {}\nsurname: {}\nemail: {}\nactive: {}\nsign_in_count: {}",
        new_user.name,
        new_user.surname,
        new_user.email,
        new_user.active,
        new_user.sign_in_count
    );

    // tuple struct
    let new_tuple = Point(1, 2, 3);

    let Point(x, y, z) = new_tuple;

    println!("Point values: {}, {}, {}", x, y, z);

    println!("new_tuple = {new_tuple:?}");

    new_user.getData();
}

#[derive(Debug)]
struct User {
    name: String,
    surname: String,
    active: bool,
    email: String,
    sign_in_count: u64
}

impl User {
    fn getData(&self) { // or &mut self
        println!("name: {}\nsurname: {}", self.name, self.surname);
    }
}

// type Color cannot take Point as an argument even though both 
// types are made up of threee i32 values!
#[derive(Debug)]
struct Point(i32, i32, i32);
struct Color(i32, i32, i32);
