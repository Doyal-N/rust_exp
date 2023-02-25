fn print_vec(xs: &Vec<i32>) {
  for n in xs {
    println!("{}", n);
  }
}

fn main() {
  let s = vec![1, 2, 3];

  print_vec(&s);
  print_vec(&s);

}
