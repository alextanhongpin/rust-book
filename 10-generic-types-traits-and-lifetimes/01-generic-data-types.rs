// A point generic over type T
struct Point<T> {
  x: T,
  y: T,
}

// Using generic type in method definitions
impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}

// Declaring concrete type
impl Point<f32> {
  fn x(&self) -> f32 {
    &self.x
  }
}

// A point generic over two types
struct Point<T,U> {
  x: T,
  y: U,
}

impl<T,U> Point<T,U> {
  fn mixup<V,W>(self, other: Point<V,W>) -> Point<V,W> {
    Point {
      x: self.x,
      y: other.y,
    }
  }
}

// A generic type for enum
enum Option<T> {
  Some(T),
  None,
}

enum Result<T,E> {
  Ok(T),
  Err(E),
}



fn main() {
  let number_list = vec![34, 50, 25, 100, 65];

  // let mut largest = number_list[0];

  // for number in number_list {
  //   if number > largest {
  //     largest = number;
  //   }
  // }

  // println!("The largest number is {}", largest);
  println!("The largest number is {}", largest(&number_list));
}

// fn largest(list: &[i32]) -> i32 {
fn largest<T>(list: &[T]) -> T {
  let mut largest = list[0];

  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }

  largest
}