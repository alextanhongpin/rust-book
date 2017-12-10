use std::collections::HashMap;

fn main() {
  let mut scores = HashMap::new();

  scores.insert(String::from("blue"), 10);
  scores.insert(String::from("yellow"), 40);
  println!("scores are now {:?}", scores);

  let teams = vec![String::from("Blue"), String::from("Yellow")];
  let initial_scores = vec![10, 20];

  let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
  println!("scores are now {:?}", scores);

  let team_name = String::from("Blue");
  let score = scores.get(&team_name);
  println!("Team name is {} with score {:?}", team_name, score);

  for (key, value) in &scores {
    println!("{}: {}", key, value);
  }

  let mut game_scores = HashMap::new();
  game_scores.insert(String::from("blue"), 100);
  game_scores.insert(String::from("blue"), 10);
  println!("Updated game scores is {:?}", game_scores);

  // Only insert if the key has no value
  game_scores.entry(String::from("blue")).or_insert(50);
  game_scores.entry(String::from("yellow")).or_insert(100);
  println!("The game score is now {:?}", game_scores);

  // Updating value based on the old value
  let text = "hello world wonderful world";
  let mut map = HashMap::new();

  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }
  println!("map is now {:?}", map);
}