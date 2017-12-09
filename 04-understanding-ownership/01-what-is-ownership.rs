fn main() {
  let mut s = String::from("Hello");

  s.push_str(", world");

  println!("{}", s);

  let s2 = s.clone();
  println!("s2 clone {}", s2);

  takes_ownership(s2); // S2 can no longer be used

  let n = 10;
  makes_copy(n);
  println!("n is still valid {}", n);

  let s1 = gives_ownership();
  println!("gives_ownership {}", s1);

  let s3 = takes_and_gives_back(String::from("hello world"));
  println!("takes_and_gives_back {}", s3);

  let (s4, len) = calculate_length(s3);
  println!("string {} has length {}", s4, len);
}


fn takes_ownership(some_string: String) {
  println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
  println!("{}", some_integer);
}

fn gives_ownership() -> String {
  let some_string = String::from("hello");
  some_string
}

fn takes_and_gives_back(a_string: String) -> String {
  a_string
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len();
  (s, length)
}