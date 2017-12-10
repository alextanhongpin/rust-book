fn main() {
  // Initializing a new string
  // let mut s = String::new();

  let data = "initial contents";
  let s = data.to_string();
  println!("s is now {}", s);

  let s = "initial contents".to_string();
  println!("s is now {}", s);

  // Appending a string with push_str
  let mut s = String::from("foo");
  s.push_str("bar");
  println!("s is now {}", s);

  // Appending a string slice with push_str
  let mut s = String::from("foo");
  let s2 = "bar";
  s.push_str(&s2);
  println!("s is now {}", s);

  let mut s = String::from("lo");
  s.push('l');
  println!("s is now {}", s);

  let s1 = String::from("hello");
  let s2 = String::from("world");
  let s3 = s1 + &s2; // Note that s1 has moved here and can no longer be used
  println!("s3 is now {}", s3);

  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");
  let s = format!("{}-{}-{}", s1, s2 , s3);
  println!("Formatted string is {}", s);

  for c in "john doe".chars() {
    println!("{}", c);
  }

  for b in "john doe".bytes() {
    println!("{}", b);
  }
}