use std::time::{Duration, Instant};

/// Schedules callbacks to be called at specified times
pub struct Planner {}

impl Planner {
    #[allow(missing_docs)]
    pub fn new() -> Planner {
        Planner {}
    }

    /// Add a callback to be called at `times`
    pub fn add<'a, T>(&mut self, _callback: &Fn() -> (), times: T)
    where
        T: 'a + IntoInstantIter<'a>,
    {
        let _times = times.into_instant_iter();
        unimplemented!();
    }
}

/// Cast into an `Iterator` of `Instant`s (i.e. times)
pub trait IntoInstantIter<'a> {
    /// The output `Iterator` type
    type IterType: Iterator<Item = Instant>;

    /// Cast into `Self::IterType`
    fn into_instant_iter(self) -> Self::IterType;
}

impl<'a> IntoInstantIter<'a> for Vec<Instant> {
    type IterType = ::std::vec::IntoIter<Instant>;
    fn into_instant_iter(self) -> Self::IterType {
        self.into_iter()
    }
}

impl<'a> IntoInstantIter<'a> for Vec<Duration> {
    type IterType = ::std::vec::IntoIter<Instant>;
    fn into_instant_iter(self) -> Self::IterType {
        let now = Instant::now();
        let added = self
            .into_iter()
            .enumerate()
            .map(&|(i, d): (usize, Duration)| now + (d * i as u32))
            // TODO: Remove `collect` followed by `into_iter`, possibly by
            // `Self::IterType` having a *very* long definition
            .collect::<Vec<Instant>>();
        added.into_iter()
    }
}
