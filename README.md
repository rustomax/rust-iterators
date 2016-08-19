# rust-iterators
Demonstrates basic Rust iterator use.

[![Build Status](https://travis-ci.org/rustomax/rust-iterators.svg?branch=master)](https://travis-ci.org/rustomax/rust-iterators)

The goal of this tutorial is to provide a handy reference to some of the common iterator patterns. It is not meant to be a replacement for the [Iterator API reference](https://doc.rust-lang.org/std/iter/trait.Iterator.html) or an overview of the core iterator concepts described in [The Book](https://doc.rust-lang.org/book/iterators.html). In fact, it is highly encouraged that you read through both.

> This tutorial assumes that you already have at least cursory familiarity with Rust.<br/>
> Certain features (`step_by()` and inclusive range) require `nightly` compiler.

## Introduction

Life is repetitive and most things in it come as series of items. Programmatically we often need to count, enumerate, and iterate over these sequences. There are several ways to generate repetition in programming languages. One of the most prominent constructs is C-style `for` loop with familiar syntax:

```c
for ( x = 0; x < 10; x++ ) {
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

 The code above will print a series of numbers from `1` to `10`, and not include the last number `11`. In other words, the `..` produces an iterator that is inclusive on the left and exclusive on the right. In order to get a range that is inclusive on both ends, you use the `...` notation. The inclusive range is currently an unstable feature, requiring `nightly` compiler:

```rust
#![feature(inclusive_range_syntax)]

for i in 1...10 {
  print!("{} ", i);
}
// output: 1 2 3 4 5 6 7 8 9 10
```

If you are not planning on using the iterator variable inside the loop, you can avoid instantiating it by leveraging the "discard" `_` pattern. For instance, the following code prints out the number of elements in the iterator:

```rust
let mut n: i32 = 0;
for _ in 0..10 {
  n += 1;
}
println!("num = {}", n);
// output: num = 10
```

The example above is somewhat contrived, since iterators in Rust have `count()` function, which returns the number of elements in the iterator without the need to count them in a loop:

```rust
println!("num = {}", (0..10).count());
// output: num = 10
```

## Digging Deeper

If the basic incremental sequential range does not satisfy your needs, there are plenty of ways in Rust to customize the range iterators. Let's look at a few common ones.

Often, a range needs to be incremented not by `1`, but by a different number. To achieve this, you can modify your range with `step_by()` method, which requires the use of `feature` available in `nightly` compiler only:

```rust
#![feature(step_by)]

for i in (0..11).step_by(2) {
  print!("{} ", i);
}
// output: 0 2 4 6 8 10
```

The `step_by()` method is not the only way to get a custom increment. The same result can be achieved with a filter containing a *closure*. It doesn't require an unstable Rust `feature` and is in general a lot more flexible. The following iterator produces a series of integers in the given range `(0..20)` that divide by `2` and `3` without a remainder:

```rust
#![feature(inclusive_range_syntax)]

for i in (0...20).filter(|x| (x % 2 == 0) && (x % 3 == 0)) {
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

Another common iterator adapter, `map()`, applies a closure to each element, and returns the resulting iterator. Here is an example of an iterator that produces a sequence of squares of numbers from `1` to `10`:

```rust
for i in (1..11).map(|x| x * x) {
    print!("{} ", i);
}
// output: 1 4 9 16 25 36 49 64 81 100
```

## Iterating over Arrays

Similarly to iterating over ranges, we can iterate over an array. The benefit of this is that arrays can contain values of arbitrary types, not just integrals. The only caveat is that array is **not** an iterator. We need to turn it into an iterator using the `iter()` method.

```rust
let a = ["Toronto", "New York", "Melbourne"];

for city in a.iter() {
  print!("{}, ", city);
}
// output: Toronto, New York, Melbourne,
```

## Combining Iterator Adapters

While in the previous sections we have covered a good variety of methods allowing you to generate many different types of iterators, the real power of Rust shines when you start combining these approaches.

What if you wanted an inclusive range between `10` and `0` that is decremented by `2`? This is easily accomplished by combining a feature and a couple of methods into a single iterator:

```rust
#![feature(inclusive_range_syntax)]

for i in (0...10).rev().filter(|x| (x % 2 == 0)) {
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

You can get very creative combining things! Below is an iterator that combines two ranges: the first one is incremented and filtered, another one - decremented. Notice how Rust allows us to visually better represent such statements by splitting them into multiple lines. Not sure what such an abomination could be used for, but here it is nonetheless!

```rust
let r = (1..20).filter(|&x| x % 5 == 0)
  .chain((6..9).rev());

for i in r {
  print!("{} ", i);
}
// output: 5 10 15 8 7 6
```

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

Programs that manipulate strings or text often require the ability to iterate over a range of characters. `char_iter` crate provides convenient way to generate ranges of chars. It supports Unicode characters.

To use the `char_iter`, put the following in your `Cargo.toml`

```
[dependencies]
char-iter = "*"
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

In the simplest case, similarly to how we created an iterator from an array, we can create an iterator from a vector using the `iter()` method:

```rust
let nums = vec![1, 2, 3, 4, 5];

for i in nums.iter() {
   print!("{} ", i);
}
// output: 1 2 3 4 5
```

Now, let's do the opposite - create a vector from an iterator. In order to do that we need what is called a *consumer*. Consumers force lazy iterators to actually produce values.

A basic consumer is `collect()` - a method that takes values from an iterator and converts them into a collection of required type. Here we are taking a range of numbers from 1 to 10 and transforming it into a vector of i32:

```rust
let v = (1..11).collect::<Vec<i32>>();
println!("{:?}", v);
// output: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
```

There are a few ways we can iterate over a vector. A good Rustic style is precisely to use the iterator:

```rust
let v = vec![1, 2, 3];
for n in v.iter() {
  print!("{} ", n);
}
// output: 1 2 3
```

To get both the element and its index, you can use `enumerate()` method, which returns a tuple containing the index and the item on each iteration:

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

## Infinity and Beyond

So far we have dealt with iterators that operated on some finite range of values. Rust generalizes iterators in such a way that it is in fact possible to create an infinite range! Let us consider the following example:

```rust
let r = (1..).collect::<Vec<i32>>();
```

The `(1..)` defines a range that starts with 1 and increments infinitely. In practice, such program compiles and runs, but eventually crashes with the error message: `fatal runtime error: out of memory`. Well, that's not very practical, you might say. Indeed, by themselves infinite ranges are pretty useless. What makes them useful is combining them with other adapters and consumers.

One particularly helpful pattern involves using the `take()` method to limit the number of items returned by the iterator. The following iterator will return the first 10 items in a sequence of squares of integers that are divisible by 5 without a remainder.

```rust
let r = (1..).map(|x| x * x).filter(|x| x % 5 == 0 ).take(10);
for i in r {
  print!("{} ", i);
}
// output: 25 100 225 400 625 900 1225 1600 2025 2500
```

## Itertools Crate

Coming soon...

## Creating Your Own iterators

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

- Function `next()` that actually generates the next `Item`. `next()` takes a mutable reference to `self` and returns an `Option` encapsulating the next value. The reason why we have to return an `Option` and not the item itself is because many iterators need to account for the situation where they have reached the end of the sequence, in which case they return `None`. Since our iterator generates an infinite sequence, our `next()` method will always return `Some(Item)`, more specifically, `Some((f32, f32))`. Thus, our `next()` function declaration looks like this:

```rust
fn next (&mut self) -> Option<(f32, f32)>
```

The `next()` function typically also does some internal housekeeping. Ours will keep track of the last Fahrenheit temperature returned, so that we can increment it by `step` on subsequent iteration. By the way, making these modifications to internal fields is the reason why we need to pass a *mutable* reference to `self` as a parameter to `next()`.

Combining things together, here is the `Iterator` trait implementation:

```rust
impl Iterator for FahrToCelc {
  type Item = (f32, f32);

  fn next (&mut self) -> Option<(f32, f32)> {
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

  fn next (&mut self) -> Option<(f32, f32)> {
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
