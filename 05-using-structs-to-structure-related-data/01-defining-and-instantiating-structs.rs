#[derive(Debug)]
struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

fn build_user(email: String, username: String) -> User {
  User {
    username,
    email,
    active: true,
    sign_in_count: 1,
  }
}

// Tuple structs
#[derive(Debug)]
struct Color(i32, i32, i32);

fn main() {
  let mut user1 = User {
    username: String::from("john doe"),
    email: String::from("john.doe@mail.com"),
    active: true,
    sign_in_count: 1,
  };

  user1.email = String::from("john.doe@gmail.com");
  println!("user is {:?}", user1);

  let user2 = build_user(String::from("jane"), String::from("jane@mail.com"));
  println!("user2 is {:?}", user2);

  let user3 = User {
    username: String::from("test"),
    email: String::from("test@mail.com"),
    ..user1
  };
  println!("user3 is {:?}", user3);
  
  let black = Color(0, 0, 0);
  println!("color is {:?}", black);
}