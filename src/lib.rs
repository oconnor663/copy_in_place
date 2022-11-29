//! [Repo](https://github.com/oconnor663/copy_in_place) —
//! [Docs](https://docs.rs/copy_in_place) —
//! [Crate](https://crates.io/crates/copy_in_place)
//!
//! This crate provides a single function, a safe wrapper around [`ptr::copy`]
//! for efficient copying within slices.
//!
//! **Note:** As of Rust 1.37, the standard library provides the equivalent
//! [`copy_within`](https://doc.rust-lang.org/std/primitive.slice.html#method.copy_within)
//! method on slices. This crate is only useful for projects that need to
//! support older versions of Rust.
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

use core::ops::Bound;
use core::ops::RangeBounds;

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
pub fn copy_in_place<T: Copy, R: RangeBounds<usize>>(slice: &mut [T], src: R, dest: usize) {
    let src_start = match src.start_bound() {
        Bound::Included(&n) => n,
        Bound::Excluded(&n) => n.checked_add(1).expect("range bound overflows usize"),
        Bound::Unbounded => 0,
    };
    let src_end = match src.end_bound() {
        Bound::Included(&n) => n.checked_add(1).expect("range bound overflows usize"),
        Bound::Excluded(&n) => n,
        Bound::Unbounded => slice.len(),
    };
    assert!(src_start <= src_end, "src end is before src start");
    assert!(src_end <= slice.len(), "src is out of bounds");
    let count = src_end - src_start;
    assert!(dest <= slice.len() - count, "dest is out of bounds");
    unsafe {
        // Derive both `src_ptr` and `dest_ptr` from the same loan
        let ptr = slice.as_mut_ptr();
        let src_ptr = ptr.add(src_start);
        let dest_ptr = ptr.add(dest);
        core::ptr::copy(src_ptr, dest_ptr, count);
    }
}

#[test]
fn test_happy_path() {
    let mut array = *b"Hello, World!";
    copy_in_place(&mut array, 1..5, 8);
    assert_eq!(&array, b"Hello, Wello!");
}

#[test]
fn test_overlapping() {
    let mut array = *b"Hello, World!";
    copy_in_place(&mut array, 1..5, 2);
    assert_eq!(&array, b"Heello World!");
}

#[test]
#[should_panic]
fn test_out_of_bounds() {
    let mut array = *b"Hello, World!";
    copy_in_place(&mut array, 1..5, 10);
}

#[test]
fn test_empty_range() {
    let mut array = *b"Hello, World!";
    copy_in_place(&mut array, 1..1, 8);
    assert_eq!(&array, b"Hello, World!");
}

#[test]
fn test_empty_slice() {
    let mut array: [u8; 0] = [];
    copy_in_place(&mut array, 0..0, 0);
    assert_eq!(array, []);
}
