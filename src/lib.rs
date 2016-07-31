//! Vi-like editor.

#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences,
)]

#![feature(
    pub_restricted,
    question_mark,
    type_ascription,
)]

pub mod gap;
pub mod range_ext;
