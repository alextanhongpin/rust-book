#[derive(Debug)]
enum IpAddrKind {
  V4(String),
  V6(String),
}

#[derive(Debug)]
enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String),
}

fn main() {
  let four = IpAddrKind::V4(String::from("127.0.0.1"));
  let six = IpAddrKind::V6(String::from("::1"));

  println!("{:?}", four);
  println!("{:?}", six);

  let home = IpAddr::V4(127, 0, 0, 1);
  let loopback = IpAddr::V6(String::from("::1"));

  println!("{:?}", home);
  println!("{:?}", loopback);

  let some_number = Some(5);
  println!("some_number value is {:?}", some_number);

  let absent_number: Option<i32> = None;
  println!("absent_number is {:?}", absent_number); 
}