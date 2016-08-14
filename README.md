# rust-iterators
Demonstrates basic Rust iterator use. Note that certain features (`step_by()` and inclusive range) require `nightly` compiler.

[![Build Status](https://travis-ci.org/rustomax/rust-iterators.svg?branch=master)](https://travis-ci.org/rustomax/rust-iterators)


## Introduction

Life is repetitive and most things in it come as series of items. Programmatically we often need to count, enumerate, and iterate over these sequences. There are several ways to generate repetition in programming languages. One of the most prominent constructs is C-style `for` loop with familiar syntax:

```c
for ( x = 0; x < 10; x++ ) {
  // do something
}
```

While this venerable method is powerful and flexible enough to accommodate many scenarios, it is also responsible for a fair share of bugs ranging from misplaced semicolons to unintentionally mutating the iterator variable inside the loop. In the spirit of safety and consistency with other language features, the C-style `for` loop is absent from Rust. Instead, Rust leverages *iterators* to achieve similar goals (and a lot more).

## Basic Ranges

The most basic way to loop over a series of items in Rust is the range. Range is created using `..` notation and produces an iterator that increments by one (1):

```rust
for i in 1..11 {
    print!("{} ", i);
}
// output: 1 2 3 4 5 6 7 8 9 10
```

 The code above will print a series of numbers from 1 to 10. The `..` produces an iterator that is inclusive on the left and exclusive on the right. In order to get a range that is inclusive on both ends, you use the `...` notation. The inclusive range is currently an unstable feature, requiring `nightly` compiler:

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

## Complex Ranges

If the basic incremental sequential range does not satisfy your needs, there are plenty of ways in Rust to customize the range iterators. Let's look at a few common ones.

Often, a range needs to be incremented not by one (1), but by a different number. To achieve this, you can modify your range with `step_by()` method, which requires the use of `feature` available in `nightly` compiler only:

```rust
#![feature(step_by)]

for i in (0..11).step_by(2) {
    print!("{} ", i);
}
// output: 0 2 4 6 8 10
```

The `step_by()` method is not the only way to get custom increment. The same result can be achieved with a filter using a *closure*. It doesn't require an unstable Rust `feature` and is in general a lot more flexible.

```rust
for i in (0..11).filter(|x| x % 2 == 0) {
    print!("{} ", i);
}
// output: 0 2 4 6 8 10
```

> Closures are indeed very powerful, but beyond the scope of this tutorial.

And what if you need a non-contiguous range? It is easily accomplished by combining multiple ranges with the `chain()` method:

```rust
let c = (1..4).chain((6..9));

for i in c {
    print!("{:?} ", i);
}
// output: 1 2 3 6 7 8
```

Programs that manipulate strings or text need the ability to iterate over a range of characters. `char_iter` crate provides convenient way to generate ranges of chars. It supports Unicode characters.

To use the `char_iter`, put the following in your `Cargo.toml`

```
[dependencies]
char-iter = "*"
```

And then generate a character range with `char_iter::new()` method:

```rust
extern crate char_iter;

fn main() {
    for c in char_iter::new('А', 'Я') {
        print!("{} ", c);
    }
}
// output: Д Е Ж З И Й К Л М Н О П
```

## Iterating over Vectors

## Itertools Crate

To be continued...
