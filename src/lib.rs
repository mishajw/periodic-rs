//! Scheduling tool for running tasks at fixed intervals

#![warn(missing_docs)]

mod planner;
pub use planner::Planner;

mod instant_iter;
