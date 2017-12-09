fn main() {
  let s = String::from("hello world");
  
  let word = first_word(&s); // Word will get the value of 5

  println!("word length is {}", word);

  let word = first_word("hello world");
  
  println!("word is {}", word);

  // s.clear(); // This empties the string

  // let s = String::from("hello world");
  let hello = &s[0..5]; // Same as &s[..5]
  let world = &s[6..11];

  println!("first word value is {}", hello);
  println!("second word value is {}", world);

  let len = s.len();
  let slice = &s[3..len];
  let slice2 = &s[3..];
  println!("slice is {}, slice2 is {}", slice, slice2);

  let slice3 = &s[..];
  println!("slice3 is {}", slice3);

}

fn first_word (s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[..i];
    }
  }

  &s[..]
}