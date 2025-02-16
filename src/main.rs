use itertools::Itertools;
use char_iter::new;

mod fahrtocelc;

fn main() {
    // === Basic Range (exclusive on the right) ===
    println!("=== Basic Range (exclusive on the right) ===");
    for i in 1..11 {
        print!("{} ", i);
    }
    println!("\n");

    // === Inclusive Range ===
    println!("=== Inclusive Range ===");
    for i in 1..=10 {
        print!("{} ", i);
    }
    println!("\n");

    // === Discard Pattern and Count ===
    println!("=== Discard Pattern and Count ===");
    let mut n: i32 = 0;
    for _ in 0..10 {
        n += 1;
    }
    println!("num (by loop) = {}", n);
    println!("num (by count) = {}", (0..10).count());
    println!();

    // === Range with Filter (even numbers between 0 and 20) ===
    println!("=== Range with Filter (even numbers between 0 and 20) ===");
    for i in (0..21).filter(|x| x % 2 == 0) {
        print!("{} ", i);
    }
    println!("\n");

    // === Complex Range with Filter (divisible by both 2 and 3) ===
    println!("=== Complex Range with Filter (divisible by both 2 and 3) ===");
    for i in (0..=20).filter(|x| x % 2 == 0 && x % 3 == 0) {
        print!("{} ", i);
    }
    println!("\n");

    // === Reverse Range ===
    println!("=== Reverse Range ===");
    for i in (0..11).rev() {
        print!("{} ", i);
    }
    println!("\n");

    // === Using map() to Produce Squares ===
    println!("=== Using map() to Produce Squares ===");
    for i in (1..11).map(|x| x * x) {
        print!("{} ", i);
    }
    println!("\n");

    // === Using fold() to Sum Squares from 1 to 5 ===
    println!("=== Using fold() to Sum Squares from 1 to 5 ===");
    let result = (1..=5).fold(0, |acc, x| acc + x * x);
    println!("result = {}", result);
    println!();

    // === Directly terating over Arrays with Copy Types ===
    println!("=== Directly Iterating over Arrays with Copy Types ===");
    let cities = ["Toronto", "New York", "Melbourne"];
    for city in cities {
        print!("{}, ", city);
    }
    // output: Toronto, New York, Melbourne,
    println!("Array still available: {:?}", cities);
    println!();

    // === Iterating over Arrays with Non-Copy Types (using .iter()) ===
    println!("=== Iterating over Arrays (using .iter()) with String ===");
    let cities = [
        String::from("Toronto"),
        String::from("New York"),
        String::from("Melbourne"),
    ];
    for city in cities.iter() {
        // `city` is a reference (&String) so the array is not consumed.
        print!("{}, ", city);
    }
    println!("Array still available: {:?}", cities);
    println!("\n");

    // === Iterating over Arrays with Non-Copy Types (using & operator) ===
    println!("=== Iterating over Arrays (using & operator) with String ===");
    let cities = [
        String::from("Toronto"),
        String::from("New York"),
        String::from("Melbourne"),
    ];
    for city in &cities {
        print!("{}, ", city);
    }
    println!("\n");

    // === Reverse Range with Filter ===
    println!("=== Reverse Range with Filter ===");
    for i in (0..=10).rev().filter(|x| x % 2 == 0) {
        print!("{} ", i);
    }
    println!("\n");

    // === Combining Two Ranges with chain() ===
    println!("=== Combining Two Ranges with chain() ===");
    let c = (1..4).chain(6..9);
    for i in c {
        print!("{} ", i);
    }
    println!("\n");

    // === Combo: Filter and chain ===
    println!("=== Combo: Filter and chain ===");
    let r = (1..20).filter(|&x| x % 5 == 0).chain((6..9).rev());
    for i in r {
        print!("{} ", i);
    }
    println!("\n");

    // === Using zip() to Pair Cities with Populations ===
    println!("=== Using zip() to Pair Cities with Populations ===");
    let cities = ["Toronto", "New York", "Melbourne"];
    let populations = [2_615_060, 8_550_405, 4_529_500];
    let matrix = cities.iter().zip(populations.iter());
    for (c, p) in matrix {
        println!("{:10}: population = {}", c, p);
    }
    println!();

    // === Range of Characters using char_iter ===
    println!("=== Range of Characters ===");
    for c in new('Д', 'П') {
        print!("{} ", c);
    }
    println!("\n");

    // === Iterating over a Vector using iter() ===
    println!("=== Iterating over a Vector using iter() ===");
    let nums = vec![1, 2, 3, 4, 5];
    for i in nums.iter() {
        print!("{} ", i);
    }
    println!("\n");

    // === Iterating over a Vector by Borrowing with & ===
    println!("=== Iterating over a Vector by Borrowing with & ===");
    let nums = vec![1, 2, 3, 4, 5];
    for i in &nums {
        print!("{} ", i);
    }
    println!("\n");

    // === Mutable Borrow: Doubling Each Element in a Vector ===
    println!("=== Mutable Borrow: Doubling Each Element in a Vector ===");
    let mut nums = vec![1, 2, 3, 4, 5];
    for i in &mut nums {
        *i *= 2;
    }
    println!("{:?}", nums);
    println!();

    // === Using map() to Double Elements and Collect into a New Vector ===
    println!("=== Using map() to Double Elements and Collect ===");
    let nums = vec![1, 2, 3, 4, 5];
    let doubled = nums.iter().map(|x| x * 2).collect::<Vec<i32>>();
    println!("{:?}", doubled);
    println!();

    // === Creating a Vector from an Iterator with collect() ===
    println!("=== Creating a Vector from an Iterator with collect() ===");
    let v: Vec<i32> = (1..11).collect();
    println!("{:?}", v);
    println!();

    // === Using enumerate() to Get Index and Value ===
    println!("=== Using enumerate() to Get Index and Value ===");
    let v = vec![1, 2, 3];
    for (i, n) in v.iter().enumerate() {
        println!("v[{}] = {}", i, n);
    }
    println!();

    // === Using min() and max() on a Vector ===
    println!("=== Using min() and max() on a Vector ===");
    let v = vec![3, 5, 0, -2, 3, 4, 1];
    let max = v.iter().max();
    let min = v.iter().min();
    println!("max = {:?}, min = {:?}", max, min);
    println!();

    // === Using sum() to Compute the Sum and GPA ===
    println!("=== Using sum() to Compute the Sum and GPA ===");
    let grades = vec![4, 5, 6, 9, 7, 4, 8];
    let sum: i32 = grades.iter().sum();
    let gpa = sum as f32 / grades.len() as f32;
    println!("sum = {}, gpa = {:.2}", sum, gpa);
    println!();

    // === Using an Infinite Iterator (with take()) ===
    println!("=== Using an Infinite Iterator (with take()) ===");
    let v = (1..)
        .map(|x| x * x)
        .filter(|x| x % 5 == 0)
        .take(10)
        .collect::<Vec<i32>>();
    println!("{:?}", v);
    println!();

    // === itertools unique(): Eliminating Duplicates ===
    println!("=== itertools unique(): Eliminating Duplicates ===");
    let data = vec![1, 4, 3, 1, 4, 2, 5];
    let unique = data.iter().unique();
    for d in unique {
        print!("{} ", d);
    }
    println!("\n");

    // === Range with Step using step_by() ===
    println!("=== Range with Step using step_by() ===");
    for i in (0..11).step_by(2) {
        print!("{} ", i);
    }
    println!("\n");

    // === itertools join(): Joining Elements with a Separator ===
    println!("=== itertools join(): Joining Elements with a Separator ===");
    let creatures = vec!["banshee", "basilisk", "centaur"];
    let list = creatures.iter().join(", ");
    println!("In the enchanted forest, we found {}.", list);
    println!();

    // === itertools sorted_by(): Sorting with a Custom Comparator ===
    println!("=== itertools sorted_by(): Sorting with a Custom Comparator ===");
    let happiness_index = vec![
        ("Canada", 7),
        ("Iceland", 4),
        ("Netherlands", 6),
        ("Finland", 1),
        ("New Zealand", 8),
        ("Denmark", 3),
        ("Norway", 2),
        ("Sweden", 9),
        ("Switzerland", 5),
    ];
    let top_countries = happiness_index
        .into_iter()
        .sorted_by(|a, b| a.1.cmp(&b.1))
        .take(5);
    for (country, rating) in top_countries {
        println!("# {}: {}", rating, country);
    }
    println!();

    // === Using Our Own Iterator from the fahrtocelc Module ===
    println!("=== Using Our Own Iterator from the fahrtocelc Module ===");
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
