#![feature(
    box_syntax,
    insert_str,
    plugin,
    type_ascription,
)]

#![plugin(quickcheck_macros)]

extern crate quickcheck;
extern crate viola;

mod gap;
