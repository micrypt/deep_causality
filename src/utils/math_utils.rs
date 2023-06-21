/*
 * Copyright (c) 2023. Marvin Hansen <marvin.hansen@gmail.com> All rights reserved.
 */

use crate::types::alias_types::NumericalValue;

pub const ZERO: NumericalValue = 0.0;
pub const MINUS_ONE: NumericalValue = -1.0;

/// returns the absolute value of a numerical value
pub fn abs_num(val: NumericalValue) -> NumericalValue {
    return if val > ZERO { val } else { MINUS_ONE * val };
}
