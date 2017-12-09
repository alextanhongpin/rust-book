enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

fn main() {
  let value = value_in_cents(Coin::Penny);
  println!("Value of penny is {}", value);

  let value = value_in_cents(Coin::Nickel);
  println!("Value of penny is {}", value);

  let value = value_in_cents(Coin::Dime);
  println!("Value of penny is {}", value);

  let value = value_in_cents(Coin::Quarter);
  println!("Value of penny is {}", value);

  let five = Some(5);
  let six = plus_one(five);
  println!("six is {:?}", six);

  let none = plus_one(None);
  println!("none is {:?}", none);
}

fn value_in_cents(coin: Coin) -> u32 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}

