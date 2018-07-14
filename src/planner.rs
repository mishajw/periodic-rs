use instant_iter::IntoInstantIter;

/// Schedules callbacks to be called at specified times
pub struct Planner {}

impl Planner {
    #[allow(missing_docs)]
    pub fn new() -> Planner {
        Planner {}
    }

    /// Add a callback to be called at `times`
    pub fn add<'a, T>(&mut self, _callback: &Fn() -> (), times: impl IntoInstantIter)
    {
        let _times = times.into_instant_iter();
        unimplemented!();
    }
}
