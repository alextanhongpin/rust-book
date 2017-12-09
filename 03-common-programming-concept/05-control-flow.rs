fn main() {
  let number = 3;

  if number < 5 {
    println!("condition was true");
  } else {
    println!("condition was false");
  }


  let condition = true;
  let number = if condition {
    5 
  } else {
    6
  };
  println!("number is {}", number);

  let mut counter = 0;
  loop {
    println!("again!");
    counter += 1;
    if counter > 10 {
      break;
    }
  }

  let a = [10, 20, 30, 40, 50];

  for element in a.iter() {
    println!("the value is {}", element);
  }

  for number in 1..4 {
    println!("number is {}", number);
  }

  for n in (1..4).rev() {
    println!("reversed number is {}", n);
  }
}