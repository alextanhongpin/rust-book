pub trait Summarizable {
  fn summary(&self) -> String {
    // String::from("default lol")
    self.other_summary()
  }

  fn other_summary(&self) -> String {
    String::from("test lol")
  }
}

#[derive(Debug)]
pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summarizable for NewsArticle {
  fn summary(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

#[derive(Debug)]
pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}
impl Summarizable for Tweet {
  fn summary(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}

#[derive(Debug)]
pub struct News {}

impl Summarizable for News {}

// notify has trait bound T that specify item must be of type
// that implements the Summarizable trait
fn notify<T: Summarizable>(item: T) {
  println!("Breaking news! {}", item.summary());
}

// Function with multiple generic type
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}
// fn some_function<T, U>(t: T, u: I) -> i32 
//   where T: Display + Clone,
//         U: Clone + Debug 
// {}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
  let mut largest = list[0];

  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }

  largest
}

fn main() {
  let article = NewsArticle{
    headline: "hello world".to_string(),
    location: "earth".to_string(),
    author: "john doe".to_string(),
    content: "something".to_string(),
  };

  // println!("{:?}", article.summary());
  notify(article);

  let tweet = Tweet {
    username: String::from("John Doe"),
    content: String::from("hello world"),
    reply: false,
    retweet: true,
  };

  // println!("{:?}", tweet.summary());
  notify(tweet);
  
  let news = News{};

  // println!("{:?}", news.summary());
  notify(news);

  let numbers = vec![1, 10, 3, 40];
  println!("{:?}", largest(&numbers));

  let chars = vec!['a', 'b', 't'];
  println!("{:?}", largest(&chars));
}