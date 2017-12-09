fn main() {
  println!("Hello, world");

  another_function(50);
  multiple_parameters(10, 40);

  let y = {
    let x = 3;
    x + 1
  };
  println!("The value of y is {}", y);

  let five = with_return_values();
  println!("five is {}", five);

  let six = plus_one(five);
  println!("six is {}", six);
}

fn another_function(x: i32) {
  println!("The value of x is {}", x);
}

fn multiple_parameters(x: i32, y: i32) {
  println!("x is {}, y is {}", x, y);
}

fn with_return_values() -> i32 {
  5
}

fn plus_one(x: i32) -> i32 {
  x + 1
}