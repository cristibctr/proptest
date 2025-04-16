//-
// Copyright 2017, 2018 Jason Lingle
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Proptest Reference Documentation
//!
//! This is the reference documentation for the proptest API.
//!
//! For documentation on how to get started with proptest and general usage
//! advice, please refer to the [Proptest Book](https://proptest-rs.github.io/proptest/intro.html).

#![forbid(future_incompatible)]
#![deny(missing_docs, bare_trait_objects)]
#![no_std]
#![cfg_attr(clippy, allow(
    doc_markdown,
    // We have a lot of these lints for associated types... And we don't care.
    type_complexity
))]
#![cfg_attr(
    feature = "unstable",
    feature(allocator_api, try_trait_v2, coroutine_trait, never_type)
)]
#![cfg_attr(all(feature = "std", feature = "unstable"), feature(ip))]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(any(feature = "std", test))]
#[macro_use]
extern crate std;