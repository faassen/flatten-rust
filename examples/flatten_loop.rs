fn main() {
    let v = [vec![1, 2], vec![3, 4], vec![5, 6]];
    let mut flat = Vec::new();
    for entry in v {
        for j in entry {
            flat.push(j);
        }
    }
    println!("{:?}", flat);
}
