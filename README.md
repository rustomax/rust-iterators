# rust-iterators
Demonstrates basic Rust iterator use. Note that certain features (`step_by()` and inclusive range) require `nightly` compiler.

[![Build Status](https://travis-ci.org/rustomax/rust-iterators.svg?branch=master)](https://travis-ci.org/rustomax/rust-iterators)


## Introduction

One of the most prominent constructs in C is the `for` loop with a very familiar syntax:

```c
for ( x = 0; x < 10; x++ ) {
  // do something
}
```

While this looping method is powerful and flexible, it is also responsible for a fair share of bugs ranging from misplaced semicolons to unintentionally mutating the index variable inside the loop. In the spirit of safety and consistency with other language features, the C-style `for` loop is absent from Rust. Instead, Rust leverages iterators to achieve the same goals (and a lot more).

## Basic Ranges

The most basic way to loop over a series of items in Rust is the range. Range is created using `..` notation:

```rust
for i in 1..11 {
    print!("{} ", i);
}
```

 The code above will print a series of numbers from 1 to 10. In other words, `..` produces an iterator that is inclusive on the left and exclusive on the right. In order to get a range that is inclusive on both ends, you use the `...` notation. The inclusive range is currently an unstable feature, requiring the use of `nightly` compiler:

```rust
#![feature(inclusive_range_syntax)]

for i in 1...10 {
    print!("{} ", i);
}
```

If you are not planning to use the iterator variable inside the loop, you can discard it with the `_` pattern. For instance, the following code prints out the number of elements in the iterator:

```rust
let mut n: i32 = 0;
for _ in 0..10 {
    n += 1;
}
println!("num of elements = {}", n);
```

To be continued...
