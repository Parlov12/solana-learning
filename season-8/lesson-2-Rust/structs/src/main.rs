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

}

struct User {
    name: String,
    surname: String,
    active: bool,
    email: String,
    sign_in_count: u64
}

struct Point(i32, i32, i32);
