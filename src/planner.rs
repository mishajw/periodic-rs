use std::cmp::Ordering;
use std::collections::binary_heap::BinaryHeap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use instant_iter::IntoInstantIter;

/// Schedules callbacks to be called at specified times
pub struct Planner {
    /// Queue of callbacks to execute, ordered by soonest to be executed
    job_queue: Arc<Mutex<BinaryHeap<Job>>>,
    /// Empty sender to wake threads for processing jobs
    job_processor_tx: Option<mpsc::Sender<()>>,
}

impl Planner {
    #[allow(missing_docs)]
    pub fn new() -> Planner {
        Planner {
            job_queue: Arc::new(Mutex::new(BinaryHeap::new())),
            job_processor_tx: None,
        }
    }

    /// Add a callback to be called at the times specified
    pub fn add<T: Iterator<Item = Instant> + Send + Sync + 'static>(
        &mut self,
        callback: impl Fn() -> () + Send + Sync + 'static,
        times: impl IntoInstantIter<IterType = T>,
    )
    {
        // Create a job and add it to the execution queue
        let job = Job::new(callback, times)
            .expect("Added job with no execution times");
        let job_next_time = job.next_time;
        self.job_queue.lock().unwrap().push(job);

        // Check if the submitted job is the next job in the queue
        let is_earliest_job = match self.job_queue.lock().unwrap().peek() {
            Some(earliest_job) => job_next_time == earliest_job.next_time,
            None => false,
        };

        // If this job is the earliest job, then spawn a thread to wake up the
        // job handler
        if self.is_started() && is_earliest_job {
            Self::spawn_waker(
                self.job_processor_tx.clone(),
                job_next_time.duration_since(Instant::now()),
            );
        }

        // If we have not started executing jobs, start now
        if !self.is_started() {
            self.start();
        }
    }

    /// Start running added callbacks
    pub fn start(&mut self) {
        // If we're already started, don't start again
        if self.is_started() {
            return;
        }

        // Create channels for waking up running thread
        let (job_processor_tx, job_processor_rx) = mpsc::channel();
        self.job_processor_tx = Some(job_processor_tx.clone());
        let job_queue = self.job_queue.clone();

        // Spawn thread for handling callbacks
        Self::spawn("planner", move || loop {
            let mut job_queue_locked = job_queue.lock().unwrap();
            let next_time = job_queue_locked.peek().map(|job| job.next_time);
            let now = Instant::now();

            // If no next job, then we're done and the thread can finish
            if next_time.is_none() {
                break;
            }

            // If the time in the past (or present), then execute the job
            if next_time.unwrap() <= now {
                let job = job_queue_locked.pop().expect(
                    "Job disappeared from queue while queue was locked",
                );
                // Execute job
                let spawn_callback = job.callback.clone();
                Self::spawn("exec_callback", move || (*spawn_callback)());
                // Add back in next time for job
                job.to_next_time()
                    .map(|new_job| job_queue_locked.push(new_job));
                continue;
            }

            // Drop the lock while sleeping
            drop(job_queue_locked);

            // Otherwise, sleep until the next job
            Self::spawn_waker(
                Some(job_processor_tx.clone()),
                next_time.unwrap().duration_since(Instant::now()),
            );
            job_processor_rx
                .recv()
                .expect("Couldn't listen for waking messages");
        });
    }

    fn is_started(&self) -> bool {
        self.job_processor_tx.is_some()
    }

    /// Spawn a thread to wake up the processing thread after a specified time
    fn spawn_waker(
        job_processor_tx: Option<mpsc::Sender<()>>,
        duration: Duration,
    )
    {
        Self::spawn("waker", move || {
            thread::sleep(duration);
            job_processor_tx.map(|tx| tx.send(()));
        });
    }

    /// Spawn a thread with a name prefixed by `periodic_`
    fn spawn(
        name: impl ::std::fmt::Display,
        callback: impl FnOnce() -> () + Send + 'static,
    )
    {
        let name = format!("{}_{}", env!("CARGO_PKG_NAME"), name);
        thread::Builder::new()
            .name(name.into())
            .spawn(callback)
            .expect("Failed to spawn thread with name");
    }
}

// TODO: The function and iterator traits are very verbose - is there an easier
// way to write them? Type aliases don't work. Macros don't work. Composite
// trait would need to be defined for every callback type.

/// Job to be processed by `callback` being called at `next_time`, and then all
/// times in `rest_times`
struct Job {
    callback: Arc<Fn() -> () + Send + Sync + 'static>,
    next_time: Instant,
    rest_times: Box<Iterator<Item = Instant> + Send + Sync>,
}

impl Job {
    /// Create a new job, returns `None` if there is no next item in `times`
    fn new<T: Iterator<Item = Instant> + Send + Sync + 'static>(
        callback: impl Fn() -> () + Send + Sync + 'static,
        times: impl IntoInstantIter<IterType = T>,
    ) -> Option<Job>
    {
        let mut times = times.into_instant_iter();
        times.next().map(|next_time| Job {
            callback: Arc::new(callback),
            next_time,
            rest_times: Box::new(times),
        })
    }

    /// Create a new job with `next_time` set to the next item in `rest_times`
    fn to_next_time(mut self) -> Option<Job> {
        self.rest_times.next().map(|new_next_time| Job {
            callback: self.callback,
            next_time: new_next_time,
            rest_times: self.rest_times,
        })
    }
}

impl Ord for Job {
    fn cmp(&self, other: &Job) -> Ordering {
        other.next_time.cmp(&self.next_time)
    }
}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Job) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Job {}

impl PartialEq for Job {
    fn eq(&self, other: &Job) -> bool { self.next_time == other.next_time }
}
