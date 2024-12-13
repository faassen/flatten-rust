fn main() {
    let v = [vec![1, 2], vec![3, 4], vec![5, 6]];
    let flat = v.clone().into_iter().reduce(|acc, x| [acc, x].concat());
    println!("{:?}", flat);
}
