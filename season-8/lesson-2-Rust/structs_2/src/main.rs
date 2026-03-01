fn main() {
    println!("Hello, world!");

    let user = User {
        active: true,
        username: "Petar",
        email: "peroprevara9@gmail.com",
        sign_in_count: 0
    }
}

struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64
}
