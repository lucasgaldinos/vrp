use crate::json::solution::{Statistic, Timing};
use std::ops::Add;

impl Default for Statistic {
    fn default() -> Self {
        Statistic {
            cost: 0.0,
            distance: 0,
            duration: 0,
            times: Timing { driving: 0, serving: 0, waiting: 0, break_time: 0 },
        }
    }
}

impl Add for Statistic {
    type Output = Statistic;

    fn add(self, rhs: Self) -> Self::Output {
        Statistic {
            cost: self.cost + rhs.cost,
            distance: self.distance + rhs.distance,
            duration: self.duration + rhs.duration,
            times: Timing {
                driving: self.times.driving + rhs.times.driving,
                serving: self.times.serving + rhs.times.serving,
                waiting: self.times.waiting + rhs.times.waiting,
                break_time: self.times.break_time + rhs.times.break_time,
            },
        }
    }
}
