// pathfinder/renderer/src/lib.rs
//
// Copyright © 2019 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The CPU portion of Pathfinder's renderer.

pub mod builder;
pub mod gpu_data;
pub mod paint;
pub mod post;
pub mod scene;
pub mod serialization;
pub mod tiles;
pub mod z_buffer;

mod sorted_vector;