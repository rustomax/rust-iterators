# rust-iterators

Demonstrates basic Rust iterator use.

[![Build Status](https://travis-ci.org/rustomax/rust-iterators.svg?branch=master)](https://travis-ci.org/rustomax/rust-iterators)

The goal of this tutorial is to provide a handy reference to some of the common iterator patterns. It is not meant to be a replacement for the [Iterator API reference](https://doc.rust-lang.org/std/iter/trait.Iterator.html) or an overview of the core iterator concepts described in [The Book](https://doc.rust-lang.org/book/ch13-02-iterators.html). In fact, this tutorial relies on both resources.

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
- [Infinity and Beyond](#infinity-and-beyond)
- [Itertools](#itertools)
- [Creating Your Own Iterators](#creating-your-own-iterators)

## Introduction

Life is repetitive and most things in it come as series of items. Programmatically we often need to count, enumerate, and iterate over these sequences. There are several ways to generate repetition in programming languages. One of the most prominent constructs is C-style `for` loop with familiar syntax:

```c
for ( x = 0; x < 10; ++x ) {
  // do something
}
```

While this venerable method is powerful and flexible enough to accommodate many scenarios, it is also responsible for a fair share of bugs ranging from misplaced semicolons to unintentionally mutating the iterator variable inside the loop. In the spirit of safety and consistency with other language features, the C-style `for` loop is absent from Rust. Instead, Rust leverages *iterators* to achieve similar goals (and a lot more).

## Basic Ranges

The most basic way to loop over a series of integers in Rust is the range. Range is created using `..` notation and produces an iterator of integers incremented by `1`:

```rust
for i in 1..11 {
    print!("{} ", i);
}
// output: 1 2 3 4 5 6 7 8 9 10
```

 The code above will print a series of numbers from `1` to `10`, and not include the last number `11`. In other words, the `..` produces an iterator that is inclusive on the left and exclusive on the right. In order to get a range that is inclusive on both ends, you use the `..=` notation:

```rust
for i in 1..=10 {
  print!("{} ", i);
}
// output: 1 2 3 4 5 6 7 8 9 10
```

If you do not use the loop iterator variable, you can avoid instantiating it by leveraging the `_` pattern. For instance, the following code prints out the number of elements in the iterator without instantiating a loop iterator variable:

```rust
let mut n: i32 = 0;
for _ in 0..10 {
  n += 1;
}
println!("num = {}", n);
// output: num = 10
```

The example above is somewhat contrived since iterators in Rust have `count()` function, which returns the number of elements in the iterator without the need to count them in a loop:

```rust
println!("num = {}", (0..10).count());
// output: num = 10
```

> You will find that experienced Rust programmers are able to express in very terse iterator language what otherwise would have taken many more lines of conventional looping code. We cover some of these patterns below as we talk about adaptors, consumers and chaining iterator methods into complex statements.

## Digging Deeper

If the basic incremental sequential range does not satisfy your needs, there are plenty of ways in Rust to customize the range iterators. Let's look at a few common ones.

Often, a range needs to be incremented not by `1`, but by a different number. This can be achieved with the `filter()` method. It applies a *closure* that can return either `true` or `false` to each element of an iterator and produces a new iterator that only contains elements for which the closure returns `true`.

The following iterator will produce a sequence of even numbers between 0 and 20.

```rust
for i in (0..21).filter(|x| (x % 2 == 0)) {
  print!("{} ", i);
}
// output: 0 2 4 6 8 10 12 14 16 18 20
```

Because `filter()` uses closures, it is very flexible and can be used to produce iterators that evaluate complex conditions. For instance, the following iterator produces a series of integers in the range between 0 and 20 that divide by both `2` and `3` without a remainder:

```rust
for i in (0..21).filter(|x| (x % 2 == 0) && (x % 3 == 0)) {
  print!("{} ", i);
}
// output: 0 6 12 18
```

While by default ranges are incremental, they can easily be reversed using the `rev()` method.

```rust
for i in (0..11).rev() {
  print!("{} ", i);
}
// output: 10 9 8 7 6 5 4 3 2 1 0
```

Another common iterator adaptor, `map()`, applies a closure to each element, and returns the resulting iterator. Here is an example of an iterator that produces a sequence of squares of numbers from `1` to `10`:

```rust
for i in (1..11).map(|x| x * x) {
    print!("{} ", i);
}
// output: 1 4 9 16 25 36 49 64 81 100
```

`fold()` is a very powerful method. It returns the result of applying a special "accumulator" type of closure to all elements of an iterator resulting in a single value. The following iterator produces a sum of squares of numbers from 1 to 5.

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

let result = acc;
println!("result = {}", result);

// output: result = 55
```

Wow! Isn't the `fold()` version so much more concise and readable?

## Iterating over Arrays

Similarly to iterating over ranges, we can iterate over an array. The benefit of this is that arrays can contain values of arbitrary types, not just integrals. The only caveat is that array is **not** an iterator. We need to turn it into an iterator using the `iter()` method.

```rust
let cities = ["Toronto", "New York", "Melbourne"];

for city in cities.iter() {
  print!("{}, ", city);
}
// output: Toronto, New York, Melbourne,
```

## Combining Iterator Adaptors

While in the previous sections we covered a good variety of methods allowing you to generate many different types of iterators, the real power of Rust shines when you start combining these approaches.

What if you wanted an inclusive range between `10` and `0` that is decremented by `2`? This is easily accomplished by combining a couple of methods into a single iterator:

```rust
for i in (0..=10).rev().filter(|x| (x % 2 == 0)) {
  print!("{} ", i);
}
// output: 10 8 6 4 2 0
```

Need a non-contiguous range (basically a combination of two non-adjacent ranges)? You can combine multiple ranges with the `chain()` method:

```rust
let c = (1..4).chain(6..9);

for i in c {
  print!("{} ", i);
}
// output: 1 2 3 6 7 8
```

You can get very creative combining things! Below is an iterator that combines two ranges: the first one is incremented and filtered, another one - decremented. Not sure what such an abomination could be used for, but here it is nonetheless!

```rust
let r = (1..20)
  .filter(|&x| x % 5 == 0)
  .chain((6..9).rev());

for i in r {
  print!("{} ", i);
}
// output: 5 10 15 8 7 6
```

> Notice how in the example above Rust allows us to visually better represent complex iterator statements by splitting them into multiple lines.

Another handy method is `zip()`. It is somewhat similar to `chain()` in that it combines two iterators into one. By contrast with `chain()`, `zip()` produces not a contiguous iterator, but an iterator of tuples:
![zip() method](https://cloud.githubusercontent.com/assets/20992642/17650212/185c5486-6216-11e6-8fd7-34d2aa976c07.PNG)

```rust
let cities = ["Toronto", "New York", "Melbourne"];
let populations = [2_615_060, 8_550_405, ‎4_529_500];

let matrix = cities.iter().zip(populations.iter());

for (c, p) in matrix {
  println!("{:10}: population = {}", c, p);
}
// output:
// Toronto   : population = 2615060
// New York  : population = 8550405
// Melbourne : population = 4529500
```

## Ranges of Characters

Programs that manipulate strings or text often require the ability to iterate over a range of characters. The [char_iter crate](https://docs.rs/char-iter/0.1.0/char_iter/) provides convenient way to generate such ranges. `char_iter` supports Unicode characters.

To use the `char_iter`, put the following in your `Cargo.toml`

```toml
[dependencies]
char-iter = "0.1"
```

And then generate a character range with `char_iter::new()` method:

```rust
extern crate char_iter;

for c in char_iter::new('Д', 'П') {
  print!("{} ", c);
}
// output: Д Е Ж З И Й К Л М Н О П
```

## Iterating over Vectors

Vector is one of Rust's fundamental structures. By its nature it is well suited to represent series of repetitive items. There are a number of language facilities in Rust that allow using vectors as iterators and vice-versa.

In the simplest case, similarly to how we created an iterator from an array, we can create an iterator from a vector using the `iter()` method. In fact this is considered to be the most idiomatic way in Rust to iterate over a vector.

```rust
let nums = vec![1, 2, 3, 4, 5];

for i in nums.iter() {
   print!("{} ", i);
}
// output: 1 2 3 4 5
```

As a matter of fact, the pattern above is so common that rust provides syntactic sugar for it in the form of the reference operator `&`.

```rust
let nums = vec![1, 2, 3, 4, 5];
for i in &nums {
   print!("{} ", i);
}
// output: 1 2 3 4 5
```

Notice that the borrows above are immutable. In other words, they are read-only. If we want to make changes to our vector, we have to use the mutable borrow `&mut`. For instance, the following code will mutably iterate over a vector doubling each element in the process.

```rust
let mut nums = vec![1, 2, 3, 4, 5];
for i in &mut nums {
    *i *= 2;
}
println!("{:?}", nums);

//output: [2, 4, 6, 8, 10]
```

However, now that you are an iterator ninja, you wouldn't use the `for` loop syntax above. You'd go with a `map()` instead, right?

```rust
let nums = vec![1, 2, 3, 4, 5];
let nums = nums.iter().map(|x| x * 2).collect::<Vec<i32>>();
println!("{:?}", nums);

//output: [2, 4, 6, 8, 10]
```

> A slight digression. What if we wanted to use mutable iterator to add elements to the vector like so:
>
>  ```rust
>  let mut nums = vec![1, 2, 3, 4, 5];
>  for i in &mut nums {
>      nums.push(*i);
>  }
>  println!("{:?}", nums);
>  ```
>
> This won't compile with the error message `cannot borrow nums as mutable more than once at a time.` You see, our iterator (instantiated in the `for` loop) already borrowed `nums` as mutable. The `push` expression tries to do that again, which is prohibited in rust. This is rust's famous safety at work. If we could `push` something into the vector, while iterating over it, this would invalidate the iterator causing undefined behavior. Rust prevents this from happening at compile time. Not only iterators are powerful, but they are also super safe.

Now, let's do the opposite - create a vector from an iterator. In order to do that we need what is called a *consumer*. Consumers force *lazy* iterators to actually produce values.

`collect()` is a common consumer. It takes values from an iterator and converts them into a collection of required type. Below we are taking a range of numbers from `1` to `10` and transforming it into a vector of `i32`:

```rust
let v = (1..11).collect::<Vec<i32>>();
println!("{:?}", v);
// output: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
```

To get both the element of a vector and its index, you can use `enumerate()` method, which returns a tuple containing the index and the item on each iteration:

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

There are a few other functions, that make using iterators on vectors particularly helpful.

`min()` and `max()`, for instance, return options, containing minimum and maximum values of the vector elements respectively:

```rust
let v = vec![3, 5, 0, -2, 3, 4, 1];
let max = v.iter().max();
let min = v.iter().min();

println!("max = {:?}, min = {:?}", max, min);

// output: max = Some(5), min = Some(-2)
```

`sum()` returns a sum of all values in an iterator. The following program leverages the `sum()` method to compute the grade point average of a rather mediocre student:

```rust
let grades = vec![4, 5, 6, 9, 7, 4, 8];
let sum: i32 = grades.iter().sum();
let gpa = sum as f32 / grades.len() as f32;

println!("sum = {}, gpa = {:.2}", sum, gpa);

// output: sum = 43, gpa = 6.14
```

## Infinity and Beyond

So far we have dealt with iterators that operated on some finite range of values. Rust generalizes iterators in such a way that it is in fact possible to create an infinite range! Let us consider the following example:

```rust
let r = (1..).collect::<Vec<i32>>();
```

The `(1..)` defines a range that starts with `1` and increments indefinitely. In practice, such program compiles and runs, but eventually crashes with the error message: `fatal runtime error: out of memory`. Well, that's not very practical, you might say. Indeed, by themselves infinite ranges are pretty useless. What makes them useful is combining them with other adaptors and consumers.

One particularly helpful pattern involves using the `take()` method to limit the number of items returned by the iterator. The following iterator will return the first `10` items in a sequence of squares of integers that are divisible by `5` without a remainder.

```rust
let v = (1..)
  .map(|x| x * x)
  .filter(|x| x % 5 == 0 )
  .take(10)
  .collect::<Vec<i32>>();

println!("{:?} ", v);

// output: [25, 100, 225, 400, 625, 900, 1225, 1600, 2025, 2500]
```

## Itertools

The [itertools crate](https://docs.rs/itertools/0.6.0/itertools) contains powerful additional iterator adaptors. Below are some examples.

To use `itertools`, add the following to your `Cargo.toml`:

```toml
[dependencies]
itertools = "0.6"
```

Remember how we used `filter()` to produce a range of even numbers? Itertools has a handy `step_by()` method for that.

```rust
extern crate itertools;
use itertools::Itertools;

for i in (0..11).step_by(2) {
    print!("{} ", i);
}

//output: 0 2 4 6 8 10
```

The `unique()` adaptor eliminates duplicates from an iterator. The duplicates do not need to be sequential.

```rust
extern crate itertools;
use itertools::Itertools;

let data = vec![1, 4, 3, 1, 4, 2, 5];
let unique = data.iter().unique();

for d in unique {
  print!("{} ", d);
}

//output: 1 4 3 2 5
```

The `join()` adaptor combines iterator elements into a single string with a separator in between the elements.

```rust
extern crate itertools;
use itertools::Itertools;

let creatures = vec!["banshee", "basilisk", "centaur"];
let list = creatures.iter().join(", ");
println!("In the enchanted forest, we found {}.", list);

// output: In the enchanted forest, we found banshee, basilisk, centaur.
```

The `sorted_by()` adaptor applies custom sorting order to iterator elements, returning a sorted vector. The following program will print out top 5 happiest countries, according to [2018 World Happiness Report](http://worldhappiness.report/ed/2018/).

> `sorted_by()` uses [Ordering trait](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html) to sort elements.

```rust
extern crate itertools;
use itertools::Itertools;

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

// output:
// # 1: Denmark
// # 2: Switzerland
// # 3: Iceland
// # 4: Norway
// # 5: Finland
```

## Creating Your Own Iterators

Beautiful thing about Rust is that you can use generic language facilities to extend it. Let us leverage this awesome power and create our own iterator! We will build a very simple iterator that produces a series of pairs of temperatures `(Fahrenheit, Celsius)`, represented by a tuple of floating-point numbers `(f32, f32)`. The temperature is calculated using commonly known formula: `°C = (°F - 32) / 1.8`.

An iterator starts with a `struct`. Whatever we name the `struct` will also be the name of the iterator. We will call ours `FahrToCelc`. The `struct` contains fields that hold useful information that persists between subsequent iterator calls. We will have two `f32` fields - the temperature in Fahrenheit, and the increment step:

```rust
struct FahrToCelc {
  fahr: f32,
  step: f32,
}
```

Next, we will create a convenience method `new()` that initializes the iterator by passing it initial values for temperature in Fahrenheit and the increment step. This method is strictly speaking not necessary and is not part of the iterator implementation, but I find it to be a nice syntactic sugar that improves overall program readability:

```rust
impl FahrToCelc {
  fn new(fahr: f32, step: f32) -> FahrToCelc {
    FahrToCelc { fahr: fahr, step: step }
  }
}
```

Finally, we program the behavior of the iterator by implementing the `Iterator` trait for our `struct`. The trait at a minimum needs to contain the following:

- Definition of the `Item` type. It describes what kind of things the iterator will produce. As mentioned before our iterator produces temperature pairs `(Fahrenheit, Celsius)` represented by a tuple of floating-point numbers `(f32, f32)`, so our `Item` type definition will look like this:

```rust
type Item = (f32, f32);
```

- Function `next()` that actually generates the next `Item`. `next()` takes a mutable reference to `self` and returns an `Option` encapsulating the next value. The reason why we have to return an `Option` and not the item itself is because many iterators need to account for the situation where they have reached the end of the sequence, in which case they return `None`. Since our iterator generates an infinite sequence, our `next()` method will always return `Option<Self::Item>`. Thus, our `next()` function declaration looks like this:

```rust
fn next (&mut self) -> Option<Self::Item>
```

The `next()` function typically also does some internal housekeeping. Ours increments Fahrenheit temperature `fahr` by `step` so that it can be returned on subsequent iteration. Making these modifications to internal fields is the reason why we need to pass a *mutable* reference to `self` as a parameter to `next()`.

Combining things together, here is the `Iterator` trait implementation:

```rust
impl Iterator for FahrToCelc {
  type Item = (f32, f32);

  fn next (&mut self) -> Option<Self::Item> {
    let curr_fahr = self.fahr;
    let curr_celc = (self.fahr - 32.0) / 1.8;
    self.fahr = self.fahr + self.step;
    Some((curr_fahr, curr_celc))
  }
}
```

At last, the complete program:

```rust
struct FahrToCelc {
  fahr: f32,
  step: f32,
}

impl FahrToCelc {
  fn new(fahr: f32, step: f32) -> FahrToCelc {
    FahrToCelc { fahr: fahr, step: step }
  }
}

impl Iterator for FahrToCelc {
  type Item = (f32, f32);

  fn next (&mut self) -> Option<Self::Item> {
    let curr_fahr = self.fahr;
    let curr_celc = (self.fahr - 32.0) / 1.8;
    self.fahr = self.fahr + self.step;
    Some((curr_fahr, curr_celc))
  }
}

fn main() {
  // pass the starting temperature and step to the initializer function
  let ftc = FahrToCelc::new(0.0, 5.0);

  // produce the iterator table of first 5 values
  let temp_table = ftc.take(5);

  // print out the temperature table nicely
  for (f, c) in temp_table {
    println!("{:7.2} °F = {:7.2} °C", f, c);
  }
}

// output:
//  0.00 °F =  -17.78 °C
//  5.00 °F =  -15.00 °C
// 10.00 °F =  -12.22 °C
// 15.00 °F =   -9.44 °C
// 20.00 °F =   -6.67 °C
```
