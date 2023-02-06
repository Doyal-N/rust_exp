fn main() {
    let mut v = vec![2, 4, 3, 100, 6, 11, 2, 8, 56, -2];
    quick_sort(&mut v);

    println!("{:?}", v);
}

fn pivot<T: PartialOrd>(v: &mut [T]) -> usize {
    let mut p = 0;
    for i in 1..v.len() {
        if v[p] > v[i] {
            v.swap(i, p + 1);
            v.swap(p + 1, p);
            p += 1;
        }
    }
    p
}

fn quick_sort<T: PartialOrd + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let mid = pivot(v);

    let (a, b) = v.split_at_mut(mid);
    rayon::join(|| quick_sort(a), || quick_sort(&mut b[1..]));
}
