# arith_traits
This crate defines traits defining arithmetic overflow policies in support of fully safe
arithmetic in Rust.  (Fully safe arithmetic is defined as arithmetic which will fail to compile if 
there is any possibility of an overflow not specifically requested by the developer.)

Ergonomic means a value wrapped with one of the following policies
is automatically subject to the specified overflow behavior even when using standard arithmetic
operators.

Fully safe arithmetic is defined as arithmetic which will fail to compile if there is any
possibility of an overflow not specifically requested by the developer.

The arithmetic overflow operations are grouped as follows:
  * `CheckedOps`
  * `OverflowingOps`
  * `PanickingOps`
  * `SaturatingOps`
  * `WrappingOps`

If you've ever wanted your generic function to accept any value that supports, for example,
checked arithmetic, you have discovered that `std` does not define a `trait` that all checked types
implement.

This crate remedies that.  It was developed to simplify the implementation of the
[`arranged`](https://github.com/u007d/arranged) crate and is leveraged by the companion
[`arith_wrappers`](https://github.com/u007d/arith_wrappers) crate.  It may also be of value to other crates performing
generic arithmetic.

NOTE: Currently requires nightly compiler.

## License
Licensed under either:
* MIT license (see LICENSE-MIT file)
* Apache License, Version 2.0 (see LICENSE-APACHE file)
at your option.

## Contributions
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you shall be dual licensed as above, without any additional terms or conditions.

