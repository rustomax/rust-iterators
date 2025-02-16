# rust-iterators

This tutorial demonstrates basic Rust iterator use with modern idioms and additional techniques. It's goal is to provide a handy reference to some common iterator patterns.

> To take full advantage of the material described here, it is recommended that you have at least cursory familiarity with Rust.

How to compile and run sample code:

```sh
git clone https://github.com/rustomax/rust-iterators.git
cd rust-iterators/
cargo run
```

## Contents

- [Introduction](#introduction)
- [Basic Ranges](#basic-ranges)
- [Digging Deeper](#digging-deeper)
- [Iterating over Arrays](#iterating-over-arrays)
- [Combining Iterator Adaptors](#combining-iterator-adaptors)
- [Ranges of Characters](#ranges-of-characters)
- [Iterating over Vectors](#iterating-over-vectors)
- [Creating Iterators from Collections - A Summary](#creating-iterators-from-collections---a-summary)
- [Infinity and Beyond](#infinity-and-beyond)
- [Itertools](#itertools)
- [Additional Iterator Adaptors](#additional-iterator-adaptors)
- [Creating Your Own Iterators](#creating-your-own-iterators)
- [Conclusion](#conclusion)

---

## Introduction

Life is repetitive, and most things in it come as series of items. Programmatically, we often need to count, enumerate, and iterate over these sequences. Many languages use the familiar C-style `for` loop:

```c
for ( x = 0; x < 10; ++x ) {
  // do something
}
```

While this method is powerful, it can be prone to errors like off-by-one bugs or unintended mutation of the iterator variable. In keeping with Rust’s safety and consistency, there is no C-style `for` loop. Instead, Rust leverages *iterators* to achieve these goals—and much more.

---

## Basic Ranges

The simplest way to loop over a series of integers in Rust is with a range. The range created using `..` produces an iterator of integers incremented by `1`:

```rust
for i in 1..11 {
    print!("{} ", i);
}
// output: 1 2 3 4 5 6 7 8 9 10
```

Note that `1..11` is inclusive at the start and exclusive at the end. If you want a range that includes both endpoints, use the `..=` notation:

```rust
for i in 1..=10 {
  print!("{} ", i);
}
// output: 1 2 3 4 5 6 7 8 9 10
```

If you do not use the loop variable, you can simply use the `_` pattern:

```rust
let mut n: i32 = 0;
for _ in 0..10 {
  n += 1;
}
println!("num = {}", n);
// output: num = 10
```

Or even more idiomatically, use the iterator’s built-in `count()`:

```rust
println!("num = {}", (0..10).count());
// output: num = 10
```

> Experienced Rust programmers often express logic in terse iterator language, turning what might have been several lines of code into a concise chain of adaptors and consumers.

---

## Digging Deeper

Sometimes a basic range isn’t enough. Rust lets you customize your iterator in many ways.

### Stepping Through a Range

Use `step_by()` to increment by a value other than 1:

```rust
for i in (0..11).step_by(2) {
    print!("{} ", i);
}
// output: 0 2 4 6 8 10
```

Alternatively, use `filter()` to achieve similar results. For example, to iterate over even numbers between 0 and 20:

```rust
for i in (0..21).filter(|x| x % 2 == 0) {
  print!("{} ", i);
}
// output: 0 2 4 6 8 10 12 14 16 18 20
```

Or combine conditions:

```rust
for i in (0..21).filter(|x| x % 2 == 0 && x % 3 == 0) {
  print!("{} ", i);
}
// output: 0 6 12 18
```

### Reversing and Mapping

Reverse a range with `rev()`:

```rust
for i in (0..11).rev() {
  print!("{} ", i);
}
// output: 10 9 8 7 6 5 4 3 2 1 0
```

Apply a function to each element with `map()`:

```rust
for i in (1..11).map(|x| x * x) {
    print!("{} ", i);
}
// output: 1 4 9 16 25 36 49 64 81 100
```

And use `fold()` to reduce the iterator to a single value:

```rust
let result = (1..=5).fold(0, |acc, x| acc + x * x);
println!("result = {}", result);
// output: result = 55
```

Perhaps the easiest way to understand what is happening here is to rewrite the example above in a more procedural fashion:

```rust
let mut acc = 0;
for x in 1..=5 {
  acc += x * x;
}
println!("result = {}", acc);
// output: result = 55
```

Wow! Isn't the `fold()` version so much more concise and readable?


---

## Iterating over Arrays

In the past, you had to explicitly call `.iter()` to iterate over an array. Now, arrays in Rust implement `IntoIterator` directly, for example:

```rust
let cities = ["Toronto", "New York", "Melbourne"];

for city in cities {
  print!("{}, ", city);
}
// output: Toronto, New York, Melbourne,
```

It's important to understand what is happening under the hood here. When you iterate over the array directly, the array’s `into_iter()` method is called. If the elements implement the `Copy` trait, they are copied into the loop variable. In the example above, each city is a string slice (`&str`), which implements `Copy`, so during iteration, each city is copied into the variable `city`. This is safe and efficient because string slices are lightweight and implement the `Copy` trait. The array will still be accessible after the iteration, because its elements are copied, not moved. For types that don’t implement `Copy`, such as `String`, the Rust complier doesn't have a choice but to `move` the elements out of the array during iteration, meaning you lose ownership in the original array. In other words, direct iteration over non-`Copy` types, consumes the array. The array becomes unusable after the iteration.

Sometimes you might want to avoid copying or moving the elements altogether, especially if the elements are larger or if they do not implement `Copy`. In that case, you iterate over references to the array elements. You can do this in one of two ways:

**1. Using `.iter()`**

```rust
let cities = [
    String::from("Toronto"),
    String::from("New York"),
    String::from("Melbourne"),
];

for city in cities.iter() {
    // Here, `city` is a reference to a String (&String), so the values aren’t moved.
    print!("{}, ", city);
}
println!();
// output: Toronto, New York, Melbourne,
```

**2. Using the `&` reference operator**

```rust
let cities = [
    String::from("Toronto"),
    String::from("New York"),
    String::from("Melbourne"),
];

for city in &cities {
    // This is equivalent to calling cities.iter().
    print!("{}, ", city);
}
println!();
// output: Toronto, New York, Melbourne,
```

Using either approach, the original `cities` array remains intact because you’re only borrowing the elements rather than moving them.

---

## Combining Iterator Adaptors

The real power of Rust shines when you start combining iterator methods.

### Complex Chains

For example, to create an inclusive range from 10 down to 0 in steps of 2:

```rust
for i in (0..=10).rev().filter(|x| x % 2 == 0) {
  print!("{} ", i);
}
// output: 10 8 6 4 2 0
```

Combine two non-adjacent ranges with `chain()`:

```rust
let c = (1..4).chain(6..9);

for i in c {
  print!("{} ", i);
}
// output: 1 2 3 6 7 8
```

And here’s a creative mix of incremented and reversed ranges:

```rust
let r = (1..20)
  .filter(|&x| x % 5 == 0)
  .chain((6..9).rev());

for i in r {
  print!("{} ", i);
}
// output: 5 10 15 8 7 6
```

### Zipping Iterators

The `zip()` adaptor combines two iterators into one of tuples:

```rust
let cities = ["Toronto", "New York", "Melbourne"];
let populations = [2_615_060, 8_550_405, 4_529_500];

let matrix = cities.iter().zip(populations.iter());

for (c, p) in matrix {
  println!("{:10}: population = {}", c, p);
}
// output:
// Toronto   : population = 2615060
// New York  : population = 8550405
// Melbourne : population = 4529500
```

### Advanced Combinators

Consider also these helpful methods:
- **`find()`**: Returns the first element matching a predicate.
- **`find_map()`**: Combines filtering and mapping.
- **`partition()`**: Splits the elements into two groups based on a predicate.

A quick example of `find()`:

```rust
let nums = [1, 3, 5, 7, 8, 9];
if let Some(even) = nums.iter().find(|&&x| x % 2 == 0) {
    println!("Found even number: {}", even);
}
// output: Found even number: 8
```

---

## Ranges of Characters

For programs that manipulate text, iterating over a range of characters can be useful. The [char_iter crate](https://docs.rs/char-iter/0.1.0/char_iter/) provides a convenient way to generate such ranges (supporting Unicode).

Add this to your `Cargo.toml`:

```toml
[dependencies]
char-iter = "0.1"
```

Then generate a character range:

```rust
use char_iter::new;

for c in new('Д', 'П') {
  print!("{} ", c);
}
// output: Д Е Ж З И Й К Л М Н О П
```

---

## Iterating over Vectors

Vectors are a fundamental collection in Rust. They can be used with many iterator methods.

### Borrowing Patterns

Iterate immutably with `.iter()` or, more succinctly, by borrowing with `&`:

```rust
let nums = vec![1, 2, 3, 4, 5];
for i in &nums {
   print!("{} ", i);
}
// output: 1 2 3 4 5
```

If you need to modify elements, use a mutable borrow:

```rust
let mut nums = vec![1, 2, 3, 4, 5];
for i in nums.iter_mut() {
    *i *= 2;
}
println!("{:?}", nums);
// output: [2, 4, 6, 8, 10]
```

If you wish to consume the vector (taking ownership of its elements), use `.into_iter()`:

```rust
let nums = vec![1, 2, 3, 4, 5];
for i in nums.into_iter() {
    println!("{}", i);
}
```

### Converting Iterators Back into Collections

Use `collect()` to create a vector from an iterator. In many cases, type inference can handle the type, but you can also annotate it:

```rust
let v: Vec<i32> = (1..11).collect();
println!("{:?}", v);
// output: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
```

### Converting References to Owned Values

When your iterator yields references, but you need owned values (for example, to pass to a function that takes ownership, or to build a new collection of owned items), you can use `.copied()` and `.cloned()` helper methods to transform each item into either a copy or a clone respectively. 

```rust
let cities = ["Toronto", "New York", "Melbourne"];
for city in cities.iter().copied() {
    println!("{}", city);
}
```

> Naturally, you'd use `copied()` for items that implement `Copy`, and `cloned()` for items that implement `Clone`

### Other Useful Methods

Get both index and value with `enumerate()`:

```rust
let v = vec![1, 2, 3];
for (i, n) in v.iter().enumerate() {
    println!("v[{}] = {}", i, n);
}
// output:
// v[0] = 1
// v[1] = 2
// v[2] = 3
```

Find minimum or maximum elements:

```rust
let v = vec![3, 5, 0, -2, 3, 4, 1];
let max = v.iter().max();
let min = v.iter().min();

println!("max = {:?}, min = {:?}", max, min);
// output: max = Some(5), min = Some(-2)
```

And sum elements easily:

```rust
let grades = vec![4, 5, 6, 9, 7, 4, 8];
let sum: i32 = grades.iter().sum();
let gpa = sum as f32 / grades.len() as f32;

println!("sum = {}, gpa = {:.2}", sum, gpa);
// output: sum = 43, gpa = 6.14
```

---

## Creating Iterators from Collections - A Summary

If all these ways of creating iterators from collections are confusing, don't worry! Here's a summary of the most common iterator conversion methods in Rust, along with examples and guidance on when to use each one.

### `iter()`

- **What It Does:**  
  The `.iter()` method creates an iterator that borrows each element of the collection as a reference. In other words, it produces items of type `&T`.

- **When to Use It:**  
  Use `.iter()` when you want to read from a collection without taking ownership of its elements. This is ideal when the elements are large, non-`Copy`, or when you need to use the collection later.

- **Example:**

  ```rust
  let numbers = vec![1, 2, 3, 4, 5];
  
  // Borrow each element (immutable reference)
  for num in numbers.iter() {
      println!("Number: {}", num);
  }
  
  // The original vector is still available here
  println!("Numbers: {:?}", numbers);
  ```

### `into_iter()`

- **What It Does:**  
  The `.into_iter()` method creates an iterator that *consumes* the collection. It takes ownership of the collection and produces items of type `T` (the owned type). For some collections (like arrays), the behavior can vary slightly, but for common collections like `Vec<T>`, it consumes the vector.

- **When to Use It:**  
  Use `.into_iter()` when you want to move the elements out of the collection, and you no longer need to use the original collection afterward.

- **Example:**

  ```rust
  let numbers = vec![1, 2, 3, 4, 5];
  
  // Consume the vector; items are owned
  for num in numbers.into_iter() {
      println!("Owned number: {}", num);
  }
  
  // `numbers` cannot be used here anymore because it was moved.
  ```

  > **Note:** When used on a type that implements `Copy`, like an array of integers or string slices, the behavior might be less noticeable because the elements are copied rather than moved.

### `iter_mut()`

- **What It Does:**  
  The `.iter_mut()` method creates an iterator that gives mutable references to each element (`&mut T`). This allows you to modify the elements in place.

- **When to Use It:**  
  Use `.iter_mut()` when you want to change the elements of a collection while iterating over them.

- **Example:**

  ```rust
  let mut numbers = vec![1, 2, 3, 4, 5];
  
  // Get a mutable reference to each element
  for num in numbers.iter_mut() {
      *num *= 2; // double each element
  }
  
  println!("Modified numbers: {:?}", numbers);
  ```

---

## Infinity and Beyond

So far we have dealt with iterators that operated on some finite range of values. Rust generalizes iterators in such a way that it is in fact possible to create an infinite range! Let us consider the following example:

```rust
let r = (1..).collect::<Vec<i32>>();
```

The `(1..)` defines a range that starts with 1 and increments indefinitely. In practice, such program compiles and runs, but eventually crashes with the error message: `fatal runtime error: out of memory`. Well, that's not very practical, you might say. Indeed, by themselves infinite ranges are pretty useless. What makes them useful is combining them with other adaptors and consumers.

One particularly helpful pattern involves using the `take()` method to limit the number of items returned by the iterator. The following iterator will return the first 10 items in a sequence of squares of integers that are divisible by 5 without a remainder.

```rust
let v = (1..)
  .map(|x| x * x)
  .filter(|x| x % 5 == 0)
  .take(10)
  .collect::<Vec<i32>>();

println!("{:?}", v);
// output: [25, 100, 225, 400, 625, 900, 1225, 1600, 2025, 2500]
```

---

## Itertools

The [itertools crate](https://docs.rs/itertools/0.14.0/itertools) offers extra iterator adaptors and methods. Add it to your `Cargo.toml`:

```toml
[dependencies]
itertools = "0.14.0"
```

Then import it in your code:

```rust
use itertools::Itertools;
```

### Unique Elements

Eliminate duplicates (even if non-sequential):

```rust
use itertools::Itertools;

let data = vec![1, 4, 3, 1, 4, 2, 5];
let unique = data.iter().unique();

for d in unique {
  print!("{} ", d);
}
// output: 1 4 3 2 5
```

### Joining Elements

Combine iterator elements into a single string with a separator:

```rust
use itertools::Itertools;

let creatures = vec!["banshee", "basilisk", "centaur"];
let list = creatures.iter().join(", ");
println!("In the enchanted forest, we found {}.", list);
// output: In the enchanted forest, we found banshee, basilisk, centaur.
```

### Custom Sorting

Sort elements using a custom comparator with `sorted_by()`:

```rust
use itertools::Itertools;

let happiness_index = vec![
    ("Canada", 7), ("Iceland", 4), ("Netherlands", 6),
    ("Finland", 1), ("New Zealand", 8), ("Denmark", 3),
    ("Norway", 2), ("Sweden", 9), ("Switzerland", 5)
];

let top_countries = happiness_index
  .into_iter()
  .sorted_by(|a, b| a.1.cmp(&b.1))
  .take(5);

for (country, rating) in top_countries {
  println!("# {}: {}", rating, country);
}

// output:
// # 1: Finland
// # 2: Norway
// # 3: Denmark
// # 4: Iceland
// # 5: Switzerland
```

---

## Additional Iterator Adaptors

Here are a few more adaptors that are handy to know:

### `filter_map()`

Combines filtering and mapping:

```rust
let numbers = vec!["1", "two", "3", "four"];
let parsed: Vec<i32> = numbers
    .iter()
    .filter_map(|s| s.parse().ok())
    .collect();
println!("{:?}", parsed);
// output: [1, 3]
```

### `take_while()` and `skip()`

These allow you to process elements conditionally. For example, take elements while they are less than 5:

```rust
let nums = vec![1, 2, 3, 4, 5, 6, 7];
let taken: Vec<_> = nums.into_iter().take_while(|&x| x < 5).collect();
println!("{:?}", taken);
// output: [1, 2, 3, 4]
```

### `inspect()`

Use `inspect()` for debugging—peek at each value without modifying it:

```rust
(1..5)
  .inspect(|x| println!("About to process: {}", x))
  .for_each(|x| println!("Got: {}", x));
```

---

## Creating Your Own Iterators

One of Rust’s strengths is the ability to create custom iterators. In this example, we create an iterator that produces pairs of temperatures in Fahrenheit and Celsius using the formula:  `°C = (°F - 32) / 1.8`.

### Defining the Iterator

An iterator starts with a struct. Whatever we name the struct will also be the name of the iterator. We will call ours `FahrToCelc`. The struct contains fields that hold useful information that persists between subsequent iterator calls. We will have two `f32` fields - the temperature in Fahrenheit, and the increment step.

```rust
struct FahrToCelc {
  fahr: f32,
  step: f32,
}
```

Next, we will create a convenience method new() that initializes the iterator by passing it initial values for temperature in Fahrenheit and the increment step. This method is strictly speaking not necessary and is not part of the iterator implementation, but I find it to be a nice syntactic sugar that improves overall program readability.

```rust
impl FahrToCelc {
  fn new(fahr: f32, step: f32) -> FahrToCelc {
    FahrToCelc { fahr, step }
  }
}
```

### Implementing the Iterator Trait

> In Rust, traits are a way of defining shared behavior. Think of a trait as a promise or a set of rules: if a type implements a trait, it guarantees that it provides certain methods. This is similar to interfaces in other languages.

The Iterator trait is one of the most central traits in Rust. It requires that a type implement the following method:

```rust
fn next(&mut self) -> Option<Self::Item>
```

* `next()`: Returns an Option—either `Some(item)` if there’s another element in the sequence, or `None` if the iterator is finished.
* `Self::Item`: The type of item the iterator yields.

In our case, the `Item` type is `(f32, f32)` because we will return pairs of Fahrenheit and Celsius temperatures.

Let's go ahead and implement the `Iterator` trait for our operator.

```rust
impl Iterator for FahrToCelc {
  type Item = (f32, f32);

  fn next(&mut self) -> Option<Self::Item> {
    let curr_fahr = self.fahr;
    let curr_celc = (self.fahr - 32.0) / 1.8;
    self.fahr += self.step;
    Some((curr_fahr, curr_celc))
  }
}
```

### Complete Program

```rust
struct FahrToCelc {
  fahr: f32,
  step: f32,
}

impl FahrToCelc {
  fn new(fahr: f32, step: f32) -> FahrToCelc {
    FahrToCelc { fahr, step }
  }
}

impl Iterator for FahrToCelc {
  type Item = (f32, f32);

  fn next(&mut self) -> Option<Self::Item> {
    let curr_fahr = self.fahr;
    let curr_celc = (self.fahr - 32.0) / 1.8;
    self.fahr += self.step;
    Some((curr_fahr, curr_celc))
  }
}

fn main() {
  // Start at 0°F with a step of 5°F.
  let ftc = FahrToCelc::new(0.0, 5.0);

  // Take the first 5 values.
  let temp_table = ftc.take(5);

  // Print the temperature table.
  for (f, c) in temp_table {
    println!("{:7.2} °F = {:7.2} °C", f, c);
  }
}

// output:
//   0.00 °F =  -17.78 °C
//   5.00 °F =  -15.00 °C
//  10.00 °F =  -12.22 °C
//  15.00 °F =   -9.44 °C
//  20.00 °F =   -6.67 °C
```

### Other Common Iterator Traits

Rust provides additional iterator-related traits that can enhance or further specify an iterator’s behavior:

* `DoubleEndedIterator`: This trait is for iterators that can be run from both ends. In addition to `next()`, they implement a method called `next_back()` which returns the next item from the end. For instance, the `rev()` method on an iterator works because many iterators also implement `DoubleEndedIterator`.

* `ExactSizeIterator`: If an iterator knows exactly how many items it contains, it can implement `ExactSizeIterator`. This provides the `len()` method, which returns the exact number of remaining elements. This is useful when you need to preallocate space or perform other size-dependent operations.

* `FusedIterator`: Once an iterator that implements `FusedIterator` returns None from `next()`, it will always return None on every subsequent call. This guarantees predictable behavior after the iterator is exhausted and can allow for certain compiler optimizations.

## Conclusion

Rust iterators empower you to write concise, expressive, and safe code by transforming how you handle sequences of data. Whether you're using built-in adaptors like `map()`, `filter()`, and `fold()`, combining multiple methods with `chain()` or `zip()`, or even creating your own custom iterators, you have a flexible toolkit at your disposal.
 
By understanding when to use `.iter()`, `.into_iter()`, or `.iter_mut()`, you can precisely control ownership and borrowing, ensuring your code is both efficient and bug-free. Experiment with these patterns in your own projects and discover just how much simpler and more elegant your iteration logic can become. 

Finally, Rust iterators are more than just a convenient way to loop over collections—they are low-cost (or even zero-cost) abstractions. This means that the performance of iterator-based code is comparable to handwritten loops, thanks to aggressive inlining and optimizations performed by the Rust compiler. By using iterators, you write clear code without sacrificing runtime efficiency.

Happy iterating!