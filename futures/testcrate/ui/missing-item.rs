#![allow(warnings)]
#![feature(proc_macro, conservative_impl_trait, generators, pin)]

extern crate futures;

use futures::prelude::*;

#[async_stream]
fn foos(a: String) -> Result<(), u32> {
    Ok(())
}

fn main() {}
