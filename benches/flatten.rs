use divan::{black_box, Bencher};
use flatten_rust::{
    create_random_list_of_lists, flatten_flat_map, flatten_flatten, flatten_fold_extend,
    flatten_fold_iter, flatten_fold_iter_collect, flatten_loop, flatten_loop_extend,
    flatten_reduce_concat,
};

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_flatten_flat_map(bencher: Bencher) {
    let v = create_random_list_of_lists();

    bencher.bench_local(move || flatten_flat_map(v.clone()))
}

#[divan::bench]
fn bench_flatten_flatten(bencher: Bencher) {
    let v = create_random_list_of_lists();

    bencher.bench_local(move || flatten_flatten(v.clone()))
}

#[divan::bench]
fn bench_flatten_loop_extend(bencher: Bencher) {
    let v = create_random_list_of_lists();

    bencher.bench_local(move || flatten_loop_extend(v.clone()))
}

#[divan::bench]
fn bench_flatten_loop(bencher: Bencher) {
    let v = create_random_list_of_lists();

    bencher.bench_local(move || flatten_loop(v.clone()))
}

#[divan::bench]
fn bench_flatten_fold_iter_collect(bencher: Bencher) {
    let v = create_random_list_of_lists();
    bencher.bench_local(move || flatten_fold_iter_collect(v.clone()))
}

#[divan::bench]
fn bench_flatten_fold_iter(bencher: Bencher) {
    let v = create_random_list_of_lists();
    bencher.bench_local(move || flatten_fold_iter(v.clone()))
}

#[divan::bench]
fn bench_flatten_fold_concat(bencher: Bencher) {
    let v = create_random_list_of_lists();
    bencher.bench_local(move || flatten_reduce_concat(v.clone()))
}

#[divan::bench]
fn bench_flatten_reduce_concat(bencher: Bencher) {
    let v = create_random_list_of_lists();
    bencher.bench_local(move || flatten_reduce_concat(v.clone()))
}

#[divan::bench]
fn bench_flatten_fold_extend(bencher: Bencher) {
    let v = create_random_list_of_lists();
    bencher.bench_local(move || flatten_fold_extend(v.clone()))
}

#[divan::bench]
fn bench_transform_loop(bencher: Bencher) {
    let v: Vec<i32> = (0..10000).collect();
    bencher.bench_local(move || {
        let mut new_v = Vec::new();
        for x in v.iter() {
            new_v.push(x + 1);
        }
        new_v
    })
}

#[divan::bench]
fn bench_transform_loop_with_capacity(bencher: Bencher) {
    let v: Vec<i32> = (0..10000).collect();
    bencher.bench_local(move || {
        let mut new_v = Vec::with_capacity(v.len());
        for x in v.iter() {
            new_v.push(x + 1);
        }
        new_v
    })
}

#[divan::bench]
fn bench_transform_loop_with_capacity_different_datatype(bencher: Bencher) {
    let v: Vec<i32> = (0..10000).collect();
    bencher.bench_local(move || {
        let mut new_v: Vec<String> = Vec::with_capacity(v.len());
        for x in v.iter() {
            new_v.push(x.to_string());
        }
        new_v
    })
}

#[divan::bench]
fn bench_transform_map(bencher: Bencher) {
    let v: Vec<i32> = (0..10000).collect();
    bencher.bench_local(move || v.iter().map(|x| x + 1).collect::<Vec<i32>>())
}

#[divan::bench]
fn bench_transform_map_different_datatype(bencher: Bencher) {
    let v: Vec<i32> = (0..10000).collect();
    bencher.bench_local(move || v.iter().map(|x| x.to_string()).collect::<Vec<String>>())
}

#[divan::bench]
fn bench_transform_map_with_transform(bencher: Bencher) {
    let transform = |x: &i32| x + 1;
    let v: Vec<i32> = (0..10000).collect();
    bencher.bench_local(move || v.iter().map(transform).collect::<Vec<i32>>())
}

// #[divan::bench]
// fn bench_transform_map_with_transform_into_iter(bencher: Bencher) {
//     let transform = |x: i32| x + 1;
//     let v: Vec<i32> = (0..10000).collect();
//     bencher.bench_local(move || v.into_iter().map(transform).collect::<Vec<i32>>())
// }
