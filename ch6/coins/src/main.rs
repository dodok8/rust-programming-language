#[derive(Debug)]
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter,
}

fn main() {
    let test_coin = Coin::Dime;
    println!("{}", value_in_cents(&test_coin));
    println!("{}", is_dime_or_quarter(&test_coin));
    println!("{}", is_dime(&test_coin));
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1, //1은 표현식
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter => {
            println!("쿼티입니다");
            25
        }
    }
}

fn is_dime_or_quarter(coin: &Coin) -> bool {
    match coin {
        Coin::Dime => true,
        Coin::Quarter => true,
        _ => false,
    }
}

fn is_dime(coin: &Coin) -> bool {
    if let Coin::Dime = coin {
        true
    } else {
        false
    }
}
