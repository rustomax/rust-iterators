#![feature(step_by)]
#![feature(inclusive_range_syntax)]

#[macro_use] extern crate itertools;
use itertools::Itertools;

fn main() {

    println!("\nBasic Range (exclusive on the right)");
    for i in 1..11 {
        println!("i = {:3}; i*i = {:3}", i, i * i);
    }

    println!("\nBasic Range (inclusive on the right). \
        Note that this requires unstable feature use \
        #![feature(inclusive_range_syntax)] available in nightly only");
    for i in 1...10 {
        println!("i = {:3}; i*i = {:3}", i, i * i);
    }

    println!("\nRange with step. Note that this requires unstable \
        feature use #![feature(step_by)]) available in nightly only");
    for i in (0..11).step_by(2) {
        println!("i = {:2}", i);
    }

    println!("\nSame can be done with a filter using a closure. \
        Doesn't require unstable features and is more flexible.");
    for i in (0..11).filter(|x| x % 2 == 0) {
        println!("i = {:3}", i);
    }

    println!("\nSame using itertools");
    for i in (0..11).step(2) {
        println!("i = {:3}", i);
    }

    println!("\nReverse inclusive range");
    let r = 1...10;
    for i in r.rev() {
        println!("i = {:2}", i);
    }

    println!("\nReverse range with a filter");
    for i in (-10..11).rev().filter(|x| x % 3 == 0) {
        println!("i = {:3}", i);
    }

    println!("\nExample of a consumer (collect). Produces a vector of i32 values.");
    let v = (1..11).collect::<Vec<i32>>();
    println!("v = {:?}", v);

    println!("\nIterate over the vector. Good Rustic style.");
    for n in v.iter() {
        println!("n = {:2}", n);
    }

    println!("\nC-style loop. This is considered bad form in Rust, \
        although can be useful if you want to yield not only vector's elements, \
        but their indexes as well.");
    for i in 0..v.len() {
        println!("v[{}] = {:2}", i, v[i]);
    }

    println!("\nMore ideomatic Rust is to use the enumerate adaptor on the iterator. \
        This yields tuples of (index, value)");
    for (i, n) in v.iter().enumerate() {
        println!("v[{}] = {:2}", i, n);
    }

    println!("\nUsing itertools to sort and dedup a vector");
    let data = vec![1, 4, 2, 3, 3, 2, 5, 1];
    println!("data = {:?}", data);
    let p = data
            .into_iter().sorted()
            .into_iter().dedup()
            .into_iter().collect::<Vec<i32>>();
    println!("data = {:?}", p);

    println!("\nUsing itertools to find min and max values of the vector");
    let (min, max) = min_max(&p);
    println!("min = {}, max = {}", min, max)

}

fn min_max(v: &Vec<i32>) -> (i32, i32) {
    match v.iter().minmax().into_option() {
        None => panic!("Could not find min and max values"),
        Some(minmax) => {
            let (min, max) = minmax;
            (*min, *max)
        },
    }
}

#[test]
fn test_min_max() {
    assert_eq!(min_max(&vec![1i32, 4, 2, 4, 5]), (1, 5));
}
