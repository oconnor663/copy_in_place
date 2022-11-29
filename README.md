# copy_in_place

[Repo](https://github.com/oconnor663/copy_in_place) —
[Docs](https://docs.rs/copy_in_place) —
[Crate](https://crates.io/crates/copy_in_place)

This crate provides a single function, a safe wrapper around [`ptr::copy`]
for efficient copying within slices.

**DEPRECATED:** As of Rust 1.37, the standard library provides the equivalent
[`copy_within`](https://doc.rust-lang.org/std/primitive.slice.html#method.copy_within)
method on slices. This crate is deprecated, and it won't receive any further updates or fixes.

## Examples

Copying four bytes within a slice:

```rust
let mut bytes = *b"Hello, World!";

copy_in_place(&mut bytes, 1..5, 8);

assert_eq!(&bytes, b"Hello, Wello!");
```

[`ptr::copy`]: https://doc.rust-lang.org/std/ptr/fn.copy.html
[PR #53652]: https://github.com/rust-lang/rust/pull/53652
