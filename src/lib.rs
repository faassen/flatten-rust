use rand::{Rng, SeedableRng};

pub fn create_random_list_of_lists() -> Vec<Vec<i32>> {
    let seed: u64 = 10;
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);

    // use random() to create a big list of lists with random sizes for sublists
    let mut v = Vec::new();
    for _ in 0..1000 {
        let mut inner = Vec::new();
        for _ in 0..rng.gen_range(0..100) {
            inner.push(rand::random());
        }
        v.push(inner);
    }
    v
}

pub fn flatten_flat_map(v: Vec<Vec<i32>>) -> Vec<i32> {
    v.into_iter().flat_map(|x| x).collect()
}

pub fn flatten_flatten(v: Vec<Vec<i32>>) -> Vec<i32> {
    v.into_iter().flatten().collect()
}

pub fn flatten_loop_extend(v: Vec<Vec<i32>>) -> Vec<i32> {
    let mut flat = Vec::new();
    for entry in v {
        flat.extend(entry);
    }
    flat
}

pub fn flatten_loop(list_of_lists: Vec<Vec<i32>>) -> Vec<i32> {
    let mut flat = Vec::new();
    for entry in list_of_lists {
        for j in entry {
            flat.push(j);
        }
    }
    flat
}

pub fn flatten_fold_concat(v: Vec<Vec<i32>>) -> Vec<i32> {
    v.into_iter().fold(Vec::new(), |acc, x| [acc, x].concat())
}

pub fn flatten_reduce_concat(v: Vec<Vec<i32>>) -> Vec<i32> {
    v.into_iter()
        .reduce(|acc, x| [acc, x].concat())
        .unwrap_or_default()
}

pub fn flatten_fold_extend(v: Vec<Vec<i32>>) -> Vec<i32> {
    v.into_iter().fold(Vec::new(), |mut acc, x| {
        acc.extend(x);
        acc
    })
}

pub fn flatten_fold_iter_collect(v: Vec<Vec<i32>>) -> Vec<i32> {
    v.into_iter().fold(Vec::new(), |accumulator, list| {
        accumulator.into_iter().chain(list).collect()
    })
}

pub fn flatten_fold_iter(v: Vec<Vec<i32>>) -> Vec<i32> {
    let iter: Box<dyn Iterator<Item = i32>> = Box::new(std::iter::empty());
    v.into_iter()
        .fold(iter, |accumulator, list| Box::new(accumulator.chain(list)))
        .collect()
}

#[derive(Debug)]
pub struct Error;

pub fn fallible_flatten(v: Vec<Vec<Result<i32, Error>>>) -> Result<Vec<i32>, Error> {
    let mut flat = Vec::new();
    for entry in v {
        for j in entry {
            flat.push(j?);
        }
    }
    Ok(flat)
}

pub fn fallible_flatten_fold(
    list_of_lists: Vec<Vec<Result<i32, Error>>>,
) -> Result<Vec<i32>, Error> {
    list_of_lists
        .into_iter()
        .try_fold(Vec::new(), |accumulator, entry| {
            let mut result = accumulator.clone();
            let e = entry.into_iter().collect::<Result<Vec<_>, _>>()?;
            result.extend(e);
            // for j in entry {
            //     result.push(j?);
            // }
            Ok(result)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amount_flattened() {
        let v = create_random_list_of_lists();
        assert_eq!(flatten_loop_extend(v.clone()).len(), 51370);
    }

    #[test]
    fn test_flatten_iter() {
        let v = create_random_list_of_lists();
        assert_eq!(flatten_fold_iter(v.clone()).len(), 51370);
    }

    #[test]
    fn test_fallible_flatten_fold() {
        let v = vec![vec![Ok(1), Ok(2)], vec![Ok(3), Ok(4)], vec![Ok(5), Ok(6)]];
        assert_eq!(fallible_flatten_fold(v).unwrap(), vec![1, 2, 3, 4, 5, 6]);
    }
}
