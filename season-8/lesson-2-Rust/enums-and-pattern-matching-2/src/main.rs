fn main() {
    let roll : Option<i64> = Some(1);

    // match - covering all cases
    match roll {
        Some(1) => println!("Move player for 1!"),
        Some(i) => println!("Move player for {i}!"),
        None => {
            println!("Dice fell down!")
        }
    }

    match roll {
        Some(1) => println!("Move player for 1!"),
        //Some(1) => add_fancy_hat(),
        other => move_player(other),
    }

    match roll {
        Some(1) => println!("Move player for 1!"),
        _ => ()
    }

    // if let - SYNTAX
    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}")
    }
}

fn move_player(other: Option<i64>) {
    //TO-DO
    println!("{other:?}");
}

fn add_fancy_hat() {
    //TO-DO
}
