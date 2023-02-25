fn main() {
  let v = vec![3, 4, 2, 10, 21, 22, 40, 2, 45];

  assert_eq!(find_duplicate_num(v), 2);

  let num_v = vec![3, 4, 2, 10, 21, 22, 4, 40, 2, 45, 4];

  assert_eq!(find_duplicate(num_v), [2, 4]);
}

fn find_duplicate(mut v: Vec<i32>) -> Vec<i32> {
  let mut result: Vec<i32> = Vec::new();
  let l = v.len();
  if l == 0 { return result; }

  v.sort();

  for i in 0..(l-1) {
    if v[i] == v[i+1] && !result.contains(&v[i]) {
      result.push(v[i]);
    }
  }
  result
}

fn find_duplicate_num(mut v: Vec<i32>) -> i32 {
  v.sort();

  for i in 0..(v.len() - 1) {
    if v[i] == v[i+1] { return v[i]; }
  }
  0
}
