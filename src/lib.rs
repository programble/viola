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
    associated_consts,
    box_patterns,
    box_syntax,
    const_fn,
    inclusive_range_syntax,
    pub_restricted,
    question_mark,
    specialization,
    type_ascription,
)]

pub mod gap;
pub mod range_ext;
