use std::env;

fn main() {
  let variables = env::vars();

  for (k, v) in variables {
      println!("{} -- {}\n", k, v);
  }
}
