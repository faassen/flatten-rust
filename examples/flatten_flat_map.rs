fn main() {
    let v = [vec![1, 2], vec![3, 4], vec![5, 6]];
    let flat: Vec<i32> = v.iter().flat_map(|x| x.iter()).cloned().collect();
    println!("{:?}", flat);
}
