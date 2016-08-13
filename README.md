# rust-iterators
Demonstrates basic Rust iterator use. Note that certain features (`step_by()` and inclusive range) require `nightly` compiler.

[![Build Status](https://travis-ci.org/rustomax/rust-iterators.svg?branch=master)](https://travis-ci.org/rustomax/rust-iterators)


## Introduction

Life is repetitive and most things in it come as series of items. Programmatically we often need to count, enumerate, and iterate over these repetitive sequences. There are several ways to generate sequences in programming languages. One of the most prominent constructs is C-style `for` loop with familiar syntax:

```c
for ( x = 0; x < 10; x++ ) {
  // do something
}
```

While this venerable method is powerful and flexible enough to accommodate for many scenarios, it is also responsible for a fair share of bugs ranging from misplaced semicolons to unintentionally mutating the iterator variable inside the loop. In the spirit of safety and consistency with other language features, the C-style `for` loop is absent from Rust. Instead, Rust leverages **iterators** to achieve the same goals (and a lot more).

## Basic Ranges

The most basic way to loop over a series of items in Rust is the range. Range is created using `..` notation:

```rust
for i in 1..11 {
    print!("{} ", i);
}
```

 The code above will print a series of numbers from 1 to 10. The `..` produces an iterator that is inclusive on the left and exclusive on the right. In order to get a range that is inclusive on both ends, you use the `...` notation. The inclusive range is currently an unstable feature, requiring the use of `nightly` compiler:

```rust
#![feature(inclusive_range_syntax)]

for i in 1...10 {
    print!("{} ", i);
}
```

If you are not planning on using the iterator variable inside the loop, you can avoid instantiating it by leveraging the "discard" `_` pattern. For instance, the following code prints out the number of elements in the iterator:

```rust
let mut n: i32 = 0;
for _ in 0..10 {
    n += 1;
}
println!("num of elements = {}", n);
```

By the way, the example above is somewhat contrived, since iterators in Rust have `count()` function, which returns the number of elements in the iterator without the need to count them in a loop:

```rust
println!("num of elements = {}", (0..10).count());
```

You can combine ranges with the `chain()` method, which is great for iterating over non-contiguous sequences:

```rust
let c = (1..4).chain((6..9));

for i in c {
    print!("{:?} ", i);
}
// prints "1 2 3 6 7 8"
```

## Ranges of Characters

For many types of programs that deal with text, it is not uncommon having to iterate over a range of characters. There are a couple of ways to accomplish this in Rust. We will use the `char_iter` crate, which supports Unicode characters.

To use the `char_iter`, put the following in your `Cargo.toml`

```
[dependencies]
char-iter = "*"
```

And then you can use the `char_iter::new()` method to generate a character range iterator:

```rust
extern crate char_iter;

fn main() {
    for c in char_iter::new('А', 'Я') {
        print!("{} ", c);
    }
}
```

## Iterators based on arrays

## Iterators based on vectors

## Advanced iterators

To be continued...
