fn main() {
  let mut x = 5;
  println!("The value of x is {}", x);

  x = 6;
  println!("The value of x is {}", x);

  const MAX_POINTS: u32 = 100_000;
  println!("The max point is {}", MAX_POINTS);

  // Shadowing allows us to reuse previous values,
  // and change the variable type
  let y = 10;
  println!("The value of y is {}", y);

  let y = y * 10;
  println!("The value of y is {}", y);

  let spaces = "  ";
  let spaces = spaces.len();
  println!("Space has length of {}", spaces);

  // This will not work
  // let mut spaces = "  ";
  // spaces = spaces.len();
}