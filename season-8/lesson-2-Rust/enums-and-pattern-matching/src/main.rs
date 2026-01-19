fn main() {
    let four = IpAddrVersion::v4;
    let six = IpAddrVersion::v6;

    let home = IpAddr {
        kind: IpAddrVersion::v4,
        address: String::from("127.0.0.1")
    };

    
    let loopback = IpAddr {
        kind: IpAddrVersion::v6,
        address: String::from("::1")
    };

    let new_coin = Coin::Penny;

    value_in_cents(new_coin);

}

enum IpAddrVersion {
    v4,
    v6
}

struct IpAddr {
    kind: IpAddrVersion,
    address: String,
}

#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
    //...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

