//! [Repo](https://github.com/oconnor663/copy_in_place) —
//! [Docs](https://docs.rs/copy_in_place) —
//! [Crate](https://crates.io/crates/copy_in_place)
//!
//! This crate provides a single function, a safe wrapper around [`ptr::copy`]
//! for efficient copying within slices. The goal is to eventually include this
//! as a built-in method on slices in libcore ([PR #53652]). (**Update** 24
//! September 2018: This function has landed in nightly as
//! [`copy_within`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.copy_within).)
//!
//! # Examples
//!
//! Copying four bytes within a slice:
//!
//! ```
//! # use copy_in_place::copy_in_place;
//! let mut bytes = *b"Hello, World!";
//!
//! copy_in_place(&mut bytes, 1..5, 8);
//!
//! assert_eq!(&bytes, b"Hello, Wello!");
//! ```
//!
//! [`ptr::copy`]: https://doc.rust-lang.org/std/ptr/fn.copy.html
//! [PR #53652]: https://github.com/rust-lang/rust/pull/53652

#![no_std]

/// Copies elements from one part of a slice to another part of the same
/// slice, using a memmove.
///
/// `src` is the range within the slice to copy from. `dest` is the starting
/// index of the range within the slice to copy to, which will have the same
/// length as `src`. The two ranges may overlap. The ends of the two ranges must
/// be less than or equal to `slice.len()`.
///
/// # Panics
///
/// This function will panic if either range exceeds the end of the slice, or if
/// the end of `src` is before the start.
///
/// # Examples
///
/// Copying four bytes within a slice:
///
/// ```
/// # use copy_in_place::copy_in_place;
/// let mut bytes = *b"Hello, World!";
///
/// copy_in_place(&mut bytes, 1..5, 8);
///
/// assert_eq!(&bytes, b"Hello, Wello!");
/// ```
pub fn copy_in_place<T, I>(slice: &mut [T], src: I, dest: usize)
where
    T: Copy,
    I: core::slice::SliceIndex<[T], Output = [T]>,
{
    let (src_ptr, src_len) = {
        // This panics if src is out of bounds or inverted.
        let src_slice = &slice[src];
        (src_slice.as_ptr(), src_slice.len())
    };
    assert!(dest <= slice.len() - src_len, "dest is out of bounds");
    unsafe {
        let dest_ptr = slice.as_mut_ptr().add(dest);
        core::ptr::copy(src_ptr, dest_ptr, src_len);
    }
}
