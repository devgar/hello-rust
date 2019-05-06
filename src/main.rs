use std::env;

fn main() {
  let target = match env::args().nth(1) {
               Some(name) => name,
               None => "world".to_owned()
               };
  println!("Hello, {}!", target);
}
