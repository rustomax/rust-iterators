# rust-iterators
Demonstrates basic Rust iterator use. Note that certain features (`step_by()` and inclusive range) require `rust-nightly`.

[![Build Status](https://travis-ci.org/rustomax/rust-iterators.svg?branch=master)](https://travis-ci.org/rustomax/rust-iterators)


## Introduction

One of the most prominent constructs in C is the `for` loop with a very familiar syntax:

```c
for ( x = 0; x < 10; x++ ) {
  // do something
}
```

While this looping method is powerful and flexible, it is also responsible for a fair share of bugs ranging from misplaced semicolons to unintentionally mutating the index variable inside the loop.

In the spirit of safety, this C-style `for` loop is absent from Rust. Instead, Rust leverages iterators to achieve the same purpose (and more).

## Basic Ranges

The most basic way to loop over a series of items in Rust is the range. It is introduced via an intuitive `..` notation:

```rust
for i in 1..11 {
    print!("{} ", i);
}
```

 The fragment of code above will print a series of numbers from 1 to 10. In other words, the `..` notation produces an iterator that is inclusive on the left and exclusive on the right. In order to get a range that is inclusive on both ends, you would use `...` notation. The inclusive range is currently an unstable feature, requiring the use of `nightly` compiler:

```rust
#![feature(inclusive_range_syntax)]

for i in 1...10 {
    print!("{} ", i);
}
```

To be continued...
