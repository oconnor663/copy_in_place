# copy_in_place

[Repo](https://github.com/oconnor663/copy_in_place) —
[Docs](https://docs.rs/copy_in_place) —
[Crate](https://crates.io/crates/copy_in_place)

This crate provides a single function, a safe wrapper around [`ptr::copy`]
for efficient copying within slices. The goal is to eventually include this
as a built-in method on slices in libcore ([PR #53652]).

## Examples

Copying four bytes within a slice:

```rust
let mut bytes = *b"Hello, World!";

copy_in_place(&mut bytes, 1, 8, 4);

assert_eq!(&bytes, b"Hello, Wello!");
```

[`ptr::copy`]: https://doc.rust-lang.org/std/ptr/fn.copy.html
[PR #53652]: https://github.com/rust-lang/rust/pull/53652
