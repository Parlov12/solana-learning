fn main() {
    println!("Hello, world!");

    another_function(32);

    let x = five();

    let y = plus_one(x);

    println!("x equals {x} and y equals {y}");
}

fn another_function(x : i32) {
    println!("Number {x} is written!");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}