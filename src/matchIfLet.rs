
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
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
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main(){

    value_in_cents(Coin::Quarter(UsState::Alaska) );

    //match staement
    let coin = Coin::Dime;
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    //if let statement with  else for above match statement
    let coin1 = Coin::Quarter(UsState::Alabama);
    let mut count1 = 0;
    if let Coin::Quarter(state) = coin1 {
        println!("State quarter from {:?}!", state);
    } else {
        count1 += 1;
    }


}
