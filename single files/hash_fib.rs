use std::collections::HashMap;

fn main() {
  let mut hash = HashMap::new();

  for elem in 10..81 {
      println!("{}: {}", elem, hash_fib(elem, &mut hash));
  }
}

fn hash_fib(n: u64, hash: &mut HashMap<u64, u64>) -> u64 {
  match n {
    0 | 1 => 1,
    n => {
      if hash.contains_key(&n) {
        *hash.get(&n).unwrap()
      } else {
        let next_val = hash_fib(n-1, hash) + hash_fib(n-2, hash);
        hash.insert(n, next_val);
        next_val
      }
    }
  }
}
