fn main() {
  // Floating-point types
  let x = 2.0;
  println!("x is {}", x); // f64

  let y: f32 = 3.0;
  println!("y is {}", y); // f32

  // Addition
  let sum = 5 + 10;
  println!("sum is {}", sum);

  // Subtraction
  let difference = 95.5 - 4.3;
  println!("difference is {}", difference);

  // Multiplication
  let product = 4 * 30;
  println!("product is {}", product);

  // Division
  let quotient = 56.7 / 32.2;
  println!("quotient is {}", quotient);

  // Remainder
  let remainder = 43 % 5;
  println!("remainder is {}", remainder);

  // Boolean Type
  let t = true;
  println!("t is {}", t);

  let f: bool = false;
  println!("f is {}", f);

  // Character type
  let heart_eyed_cat = '😻';
  println!("character is {}", heart_eyed_cat);
  
  // Tuples
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  println!("tuples is {:?}", tup);

  let (x, y, z) = tup;
  println!("x is {}, y is {}, z is {}", x, y, z);
  
  let five_hundred = tup.0;
  let six_point_four= tup.1;
  let one = tup.2;
  println!("first is {}, second is {}, third is {}", five_hundred, six_point_four, one);


  // Arrays
  let a = [1, 2, 3, 4, 5];
  println!("array {:?}", a);

  let first = a[0];
  let second = a[1];
  println!("first item in array is {}, second item in array is {}", first, second);
  
}