use std::env;

fn main() {
  let target = env::args().nth(1)
               .map(|name| name)
               .unwrap_or_else(|| "world".to_owned());
  println!("Hello, {}!", target);
}
