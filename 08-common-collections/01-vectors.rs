#[derive(Debug)]
enum SpreadsheetCell {
  Int(i32),
  Float(f64),
  Text(String),
}

fn main() {
  // let v: Vec<i32> = Vec::new();

  let v = vec![1, 2, 3];
  
  println!("{:?}", v);

  let mut v = Vec::new();
  v.push(5);
  v.push(6);
  v.push(7);
  println!("{:?}", v);

  let third = &v[2];
  println!("Third item in vector is {:?}", third);

  let third: Option<&i32> = v.get(2);
  println!("Third item in vector is {:?}", third);

  for i in &v {
    println!("Item is {:?}", i);
  }

  let mut j = vec![1, 2, 3];
  for i in &mut j {
    *i += 10;
  }
  println!("j is {:?}", j);

  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
  ];
  println!("row is {:?}", row);
}