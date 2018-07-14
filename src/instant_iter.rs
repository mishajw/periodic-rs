use std::time::{Duration, Instant};

/// Cast into an `Iterator` of `Instant`s (i.e. times)
pub trait IntoInstantIter {
    /// The output `Iterator` type
    type IterType: Iterator<Item = Instant>;

    /// Cast into `Self::IterType`
    fn into_instant_iter(self) -> Self::IterType;
}

impl IntoInstantIter for Vec<Instant> {
    type IterType = ::std::vec::IntoIter<Instant>;
    fn into_instant_iter(self) -> Self::IterType {
        self.into_iter()
    }
}

impl IntoInstantIter for Vec<Duration> {
    type IterType = DurationToInstantIter<::std::vec::IntoIter<Duration>>;

    fn into_instant_iter(self) -> Self::IterType {
        DurationToInstantIter::new(self.into_iter())
    }
}

pub struct DurationToInstantIter<I: Iterator<Item = Duration>> {
    prev_iter: I,
    now: Instant,
}

impl<I: Iterator<Item = Duration>> DurationToInstantIter<I> {
    fn new(prev_iter: I) -> Self {
        DurationToInstantIter {
            prev_iter,
            now: Instant::now(),
        }
    }
}

impl<I: Iterator<Item = Duration>> Iterator for DurationToInstantIter<I> {
    type Item = Instant;
    fn next(&mut self) -> Option<Instant> {
        self.prev_iter.next().map(|duration| {
            self.now + duration
        })
    }
}
