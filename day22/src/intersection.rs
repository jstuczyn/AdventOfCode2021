// Copyright 2022 Jedrzej Stuczynski
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::Cuboid;
use std::cmp::{max, min};
use std::ops::RangeInclusive;

pub(crate) trait Intersection: Sized {
    fn intersects(&self, other: &Self) -> bool;

    fn intersection(&self, other: &Self) -> Option<Self>;
}

impl<T> Intersection for RangeInclusive<T>
where
    T: PartialOrd + Ord + Clone,
{
    fn intersects(&self, other: &Self) -> bool {
        !(self.start() > other.end() || other.start() > self.end())
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        if !self.intersects(other) {
            None
        } else {
            let start = max(self.start(), other.start());
            let end = min(self.end(), other.end());
            Some(RangeInclusive::new(start.clone(), end.clone()))
        }
    }
}

impl Intersection for Cuboid {
    fn intersects(&self, other: &Self) -> bool {
        self.x_range.intersects(&other.x_range)
            && self.y_range.intersects(&other.y_range)
            && self.z_range.intersects(&other.z_range)
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        let x_intersection = self.x_range.intersection(&other.x_range)?;
        let y_intersection = self.y_range.intersection(&other.y_range)?;
        let z_intersection = self.z_range.intersection(&other.z_range)?;

        Some(Cuboid {
            x_range: x_intersection,
            y_range: y_intersection,
            z_range: z_intersection,
        })
    }
}
