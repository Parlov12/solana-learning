fn main() {
    println!("Hello, world!");

    let mut counter = 0;

    // loop {
    //     counter = counter + 1;
    //     println!("Counter: {counter}");
    // }

    let counter_1 = 0;
    let counter_2 = 0;

    // 'loop_1: loop {
    //     counter_1 += 1;
    //     counter_2 += 2;

    //     loop {
    //         if counter_2 == 6 {
    //             println!("Breaking first loop!");
    //             break;
    //         }
    //     }
    // }

    for element in (1..5).rev() {
        println!("element: {element}");
    }

    println!("Counter values: \ncounter_1 = {counter_1}\ncounter_2 = {counter_2}");
}
