#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:#?}");
            25
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America."))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn roll_dice(dice_roll: u8) {
    match dice_roll {
        3 => println!("add hat"),
        7 => println!("remove hat"),
        other => println!("roll: {other}"),
    }
}

fn main() {
    let c = Coin::Quarter(UsState::Alabama);

    let r = value_in_cents(c);
    println!("{}", r);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?} {:?} {:?}", five, six, none);

    roll_dice(3);
    roll_dice(5);

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    if let Some(desc) = describe_state_quarter(Coin::Quarter(UsState::Alabama)) {
        println!("{desc}");
    }
}
