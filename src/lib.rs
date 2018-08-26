//! [Repo](https://github.com/oconnor663/copy_in_place) —
//! [Docs](https://docs.rs/copy_in_place) —
//! [Crate](https://crates.io/crates/copy_in_place)
//!
//! This crate provides a single function, a safe wrapper around [`ptr::copy`]
//! for efficient copying within slices. The goal is to eventually include this
//! as a built-in method slices in libcore ([PR #53652]).
//!
//! # Examples
//!
//! Copying four bytes within a slice:
//!
//! ```
//! # use copy_in_place::copy_in_place;
//! let mut bytes = *b"Hello, World!";
//!
//! copy_in_place(&mut bytes, 1, 8, 4);
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
/// `src` is the starting index of the source region. `dest` is the starting
/// index of the destination region. `count` is the number of elements in
/// both regions. The two regions may overlap.
///
/// The ends of the two regions, `src + count` and `dest + count`, must be
/// less than or equal to `self.len()`.
///
/// # Panics
///
/// This function will panic if either region exceeds the end of the slice.
///
/// # Examples
///
/// Copying four bytes within a slice:
///
/// ```
/// # use copy_in_place::copy_in_place;
/// let mut bytes = *b"Hello, World!";
///
/// copy_in_place(&mut bytes, 1, 8, 4);
///
/// assert_eq!(&bytes, b"Hello, Wello!");
/// ```
pub fn copy_in_place<T: Copy>(slice: &mut [T], src: usize, dest: usize, count: usize) {
    // Avoid computing `src + count` or `dest + count`, which could overflow.
    assert!(count <= slice.len());
    assert!(src <= slice.len() - count);
    assert!(dest <= slice.len() - count);
    unsafe {
        let src_ptr = slice.get_unchecked(src) as *const T;
        let dest_ptr = slice.get_unchecked_mut(dest) as *mut T;
        core::ptr::copy(src_ptr, dest_ptr, count);
    }
}
