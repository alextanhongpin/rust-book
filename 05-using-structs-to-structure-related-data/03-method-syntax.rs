#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
  fn square(size: u32) -> Rectangle {
    Rectangle { width: size, height: size }
  }
}

fn main() {
  let rect = Rectangle { width: 30, height: 50 };
  let rect2 = Rectangle { width: 10, height: 5 };
  println!("The area of the rectangle is {} px", rect.area());
  println!("Rectangle 1 can hold rectangle 2 {}", rect.can_hold(&rect2));

  let square = Rectangle::square(10);
  println!("Square is: {:?}", square);
}