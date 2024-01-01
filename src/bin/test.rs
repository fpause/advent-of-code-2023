fn main() {
    let mut v: Vec<Vec<i8>> = Vec::new();
    v.push(Vec::from(vec![1i8, 2i8]));
    v.push(Vec::from(vec![4i8, 5i8]));
    let mut vec: Vec<i8> = v.into_iter()
        .flatten()
        .map(|e| vec![e - 1, e, e + 1])
        .flatten()
        .collect();
    vec.sort();
    vec.dedup();
    println!("{:?}", vec);
}