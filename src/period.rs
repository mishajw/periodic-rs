//! Methods for expressing sequences of times

use std::time::{Duration, Instant};

use instant_iter::IntoInstantIter;

/// Times occuring at fixed intervals
pub struct Every {
    duration: Duration,
    start: Instant,
}

impl Every {
    #[allow(missing_docs)]
    pub fn new(duration: Duration) -> Self {
        Every {
            duration,
            start: Instant::now(),
        }
    }
}

impl Iterator for Every {
    type Item = Instant;
    fn next(&mut self) -> Option<Instant> {
        self.start += self.duration;
        Some(self.start)
    }
}

impl IntoInstantIter for Every {
    type IterType = Self;
    fn into_instant_iter(self) -> Self::IterType { self }
}

/// Single time occuring after a fixed duration
pub struct After {
    duration: Duration,
    now: Instant,
}

impl After {
    #[allow(missing_docs)]
    pub fn new(duration: Duration) -> Self {
        After {
            duration,
            now: Instant::now(),
        }
    }
}

impl IntoInstantIter for After {
    type IterType = ::std::vec::IntoIter<Instant>;
    fn into_instant_iter(self) -> Self::IterType {
        vec![self.now + self.duration].into_iter()
    }
}
