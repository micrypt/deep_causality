// SPDX-License-Identifier: MIT
// Copyright (c) "2023" . Marvin Hansen <marvin.hansen@gmail.com> All rights reserved.

use crate::prelude::{PointIndex, Storage};

// T Type
// W Width
// H Height
impl<T, const W: usize, const H: usize> Storage<T> for [[T; W]; H]
    where
        T: Copy,
        [[T; W]; H]: Sized,
{
    fn get(&self, p: PointIndex) -> &T { &self[p.y][p.x] }

    fn set(&mut self, p: PointIndex, elem: T) { self[p.y][p.x] = elem }

    fn height(&self) -> Option<&usize> { Some(&H) }

    fn width(&self) -> Option<&usize> { Some(&W) }
}
