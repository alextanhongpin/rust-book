#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  let width1 = 30;
  let width2 = 50;

  println!(
    "The area of the rectangle is {} square pixels",
    area(width1, width2)
  );

  let rect1 = (30, 50);
  println!(
    "The area of the rectangle is {} square pixels",
    area2(rect1)
  );

  let rect2 = Rectangle {
    width: 30,
    height: 50,
  };
  println!(
    "The area of the rectangle is {} square pixels",
    area3(&rect2)
  );

  println!("Rectangle is {:#?}", rect2);
  println!("Rectangle is {:?}", rect2);
}

fn area(width: u32, height: u32) -> u32 {
  width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}

fn area3(rect: &Rectangle) -> u32 {
  rect.width * rect.height
}
