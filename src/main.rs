// https://preciselab.io/quicksort-implementation-in-rust-typescript-and-go/
use rand::prelude::*;
use std::time::Instant;

fn quick_sort<T: Ord>(arr: &mut Vec<T>) {
    if arr.len() <= 1 {
        return;
    }

    let pivot = arr.remove(0);
    let mut left = vec![];
    let mut right = vec![];

    for item in arr.drain(..) {
        if item <= pivot {
            left.push(item);
        } else {
            right.push(item);
        }
    }

    quick_sort(&mut left);
    quick_sort(&mut right);

    arr.clear();
    arr.extend(left);
    arr.push(pivot);
    arr.extend(right);
}

const MAX_SIZE: usize = 100000000;

fn main() {
    let mut arr: Vec<u32> = vec![0; MAX_SIZE];

    let mut rng = rand::thread_rng();

    for i in 0..arr.len() {
        arr[i] = rng.gen_range(1..=MAX_SIZE) as u32;
    }

    // println!("{:?}", arr);

    let now = Instant::now();

    quick_sort(&mut arr);

    // println!("{:?}", arr);

    let elapsed = now.elapsed();

    println!("Sorted {} numbers in:", arr.len());
    println!("{:?} ms", elapsed.as_millis());
    println!("{:?} seconds", elapsed.as_secs());
}
