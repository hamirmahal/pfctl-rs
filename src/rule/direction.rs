// Copyright 2024 Mullvad VPN AB.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::ffi;

/// Enum describing matching of rule towards packet flow direction.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    #[default]
    Any,
    In,
    Out,
}

impl From<Direction> for u8 {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Any => ffi::pfvar::PF_INOUT as u8,
            Direction::In => ffi::pfvar::PF_IN as u8,
            Direction::Out => ffi::pfvar::PF_OUT as u8,
        }
    }
}
