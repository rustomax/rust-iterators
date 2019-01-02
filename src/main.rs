#[allow(unused_imports)]
#[macro_use] extern crate itertools;
extern crate char_iter;

use itertools::Itertools;
mod fahrtocelc;

fn main() {

    // Basic Range (exclusive on the right)
    for i in 1..11 {
        print!("{} ", i);
    }
    println!("");

    // Inclusive range
    for i in 1..=10 {
        print!("{} ", i);
    }
    println!("");

    // use of discard "_" pattern
    let mut n: i32 = 0;
    for _ in 0..10 {
        n += 1;
    }
    println!("num = {}", n);

    // count()
    println!("num = {}", (0..10).count());

    // Range with step using a filter
    for i in (0..21).filter(|x| (x % 2 == 0)) {
      print!("{} ", i);
    }
    println!("");

    // More complex range using a filter
    for i in (0..=20).filter(|x| (x % 2 == 0) && (x % 3 == 0)) {
        print!("{} ", i);
    }
    println!("");

    // Reverse range
    for i in (0..11).rev() {
        print!("{} ", i);
    }
    println!("");

    // map()
    for i in (1..11).map(|x| x * x) {
        print!("{} ", i);
    }
    println!("");

    // fold()
    let result = (1..=5).fold(0, |acc, x| acc + x * x);
    println!("result = {}", result);

    let cities = ["Toronto", "New York", "Melbourne"];

    // iterating over arrays
    for city in cities.iter() {
        print!("{}, ", city);
    }
    println!("");

    // reverse range with a filter
    for i in (0..=10).rev().filter(|x| (x % 2 == 0)) {
        print!("{} ", i);
    }
    println!("");

    // chain()
    let c = (1..4).chain(6..9);
    for i in c {
        print!("{} ", i);
    }
    println!("");

    // combo madness
    let r = (1..20)
        .filter(|&x| x % 5 == 0)
        .chain((6..9).rev());

    for i in r {
        print!("{} ", i);
    }
    println!("");

    // zip()
    let cities = ["Toronto", "New York", "Melbourne"];
    let populations = [2_615_060, 8_550_405, ‎4_529_500];

    let matrix = cities.iter().zip(populations.iter());

    for (c, p) in matrix {
        println!("{:10}: population = {}", c, p);
    }

    // range of chars
    for c in char_iter::new('Д', 'П') {
      print!("{} ", c);
    }
    println!("");

    // iterating over vector
    let nums = vec![1, 2, 3, 4, 5];
    for i in nums.iter() {
       print!("{} ", i);
    }
    println!("");

    // iterating over vector by reference
    let nums = vec![1, 2, 3, 4, 5];
    for i in &nums {
       print!("{} ", i);
    }
    println!("");

    // mutable borrow
    let mut nums = vec![1, 2, 3, 4, 5];
    for i in &mut nums {
        *i *= 2;
    }
    println!("{:?}", nums);

    // same as above, but using map()
    let nums = vec![1, 2, 3, 4, 5];
    let nums = nums.iter().map(|x| x * 2).collect::<Vec<i32>>();
    println!("{:?}", nums);

    // create a vector from an iterator
    let v = (1..11).collect::<Vec<i32>>();
    println!("{:?}", v);

    // enumerate()
    let v = vec![1, 2, 3];
    for (i, n) in v.iter().enumerate() {
        println!("v[{}] = {}", i, n);
    }

    // min() and max()
    let v = vec![3, 5, 0, -2, 3, 4, 1];
    let max = v.iter().max();
    let min = v.iter().min();
    println!("max = {:?}, min = {:?}", max, min);

    // sum()
    let grades = vec![4, 5, 6, 9, 7, 4, 8];
    let sum: i32 = grades.iter().sum();
    let gpa = sum as f32 / grades.len() as f32;
    println!("sum = {}, gpa = {:.2}", sum, gpa);

    // infinite iterator
    let v = (1..)
        .map(|x| x * x)
        .filter(|x| x % 5 == 0 )
        .take(10)
        .collect::<Vec<i32>>();

    println!("{:?} ", v);

    // itertools unique()
    let data = vec![1, 4, 3, 1, 4, 2, 5];
    let unique = data.iter().unique();

    for d in unique {
      print!("{} ", d);
    }
    println!("");

    // Range with step
    for i in (0..11).step_by(2) {
        print!("{} ", i);
    }
    println!("");

    // join()
    let creatures = vec!["banshee", "basilisk", "centaur"];
    let list = creatures.iter().join(", ");
    println!("In the enchanted forest, we found {}.", list);

    // sorted_by()
    let happiness_index = vec![
        ("Canada", 7), ("Iceland", 4), ("Netherlands", 6),
        ("Finland", 1), ("New Zealand", 8), ("Denmark", 3),
        ("Norway", 2), ("Sweden", 9), ("Switzerland", 5)
    ];

    let top_contries = happiness_index
      .into_iter()
      .sorted_by(|a, b| (&a.1).cmp(&b.1))
      .into_iter()
      .take(5);

    for (country, rating) in top_contries {
      println!("# {}: {}", rating, country);
    }

    // our own iterator
    let ftc = fahrtocelc::FahrToCelc::new(0.0, 5.0);
    let temp_table = ftc.take(5);
    for (f, c) in temp_table {
        println!("{:7.2} °F = {:7.2} °C", f, c);
    }
}

#[test]
fn test_ftc() {
    let ftc = fahrtocelc::FahrToCelc::new(0.0, 5.0);
    for (f, c) in ftc.skip(1).take(1) {
        assert_eq!(f, 5.0);
        assert_eq!(c, -15.0);
    }
}
