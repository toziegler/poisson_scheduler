use rand::rngs::ThreadRng;
use rand::thread_rng;
use rand_distr::{Distribution, Exp};
use std::time::{Duration, Instant};

/// # PoissonScheduler
///
/// `PoissonScheduler` provides a mechanism to execute a given function/closure at timestamps determined by a Poisson process.
/// This is particularly useful in simulation and testing scenarios where events or requests need to be generated at a rate
/// that follows the Poisson distribution.
///
/// The Poisson process is a stochastic model that describes events that occur independently and at a fixed average rate.
/// It's a popular model in various fields, including computer science, telecommunications, and finance.
///
/// Read more why you want to use it: https://www.scylladb.com/2021/04/22/on-coordinated-omission/
///
/// ## Examples
///
/// ```
/// use poisson_scheduler::PoissonScheduler;
/// use std::time::Instant;
/// use std::time::Duration;
///
/// let rate = 100.0; // 100 events per second
/// let mut scheduler = PoissonScheduler::new(rate);
///
/// scheduler.run(Duration::new(1,0), |timestamp| {
///     println!("Event scheduled at {:?}", timestamp);
/// });
/// ```
///
pub struct PoissonScheduler {
    rng: ThreadRng,
    exp: Exp<f64>,
}

impl PoissonScheduler {
    /// Creates and returns a new `PoissonScheduler` with the given rate. The rate represents
    /// the average number of events per second. Note that the rate should not exceed 1e9 since
    /// the inter-arrival times are measured in nanoseconds, and we need to maintain precision.
    ///
    /// # Parameters
    ///
    /// * `rate`: The average number of events per second.
    ///
    pub fn new(rate: f64) -> Self {
        if rate > 1e9 {
            panic!("Rate should not exceed 1e9 operations per second")
        }
        let lamda = rate / 1e9; // events per nanosecond
        let exp = Exp::new(lamda).expect("Exponential function could not be created.");
        let rng = thread_rng();
        PoissonScheduler { rng, exp }
    }

    /// Schedules and runs the provided closure based on the Poisson process.
    ///
    /// # Parameters
    ///
    /// * `runtime`: The total duration (Duration) the scheduler should run.
    /// * `action`: A closure that is called each time an event is scheduled. The closure is passed
    /// the scheduled `Instant` as a parameter.
    ///
    pub fn run<F: FnMut(Instant)>(&mut self, runtime: Duration, mut action: F) {
        let end_time = Instant::now() + runtime;

        while Instant::now() < end_time {
            let inter_arrival_time = self.exp.sample(&mut self.rng);
            let next_time = Instant::now() + Duration::from_nanos(inter_arrival_time as u64);
            Self::wait_until(next_time);

            action(next_time);
        }
    }

    fn wait_until(next: Instant) {
        let mut current = Instant::now();
        let mut time_span = next.duration_since(current);

        while time_span.as_secs_f64() > 0.0 {
            current = Instant::now();
            time_span = next.duration_since(current);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PoissonScheduler;
    use std::time::{Duration, Instant};

    #[test]
    fn test_poisson_scheduler_rate() {
        // Given
        let rate = 10.0; // 10 events per second
        let runtime = Duration::new(1, 0); // 1 second
        let mut expected_in_range = 0;
        //
        for _ in 0..10 {
            let mut scheduler = PoissonScheduler::new(rate);

            // When
            let mut counter = 0;
            scheduler.run(runtime, |_| {
                counter += 1;
            });
            if counter >= 3 && counter <= 17 {
                expected_in_range += 1;
            }
        }
        // Hard to test since the process is random, however:
        //Standard Deviation (SD) = 3.16
        //Events in the range of λ±2×SDλ±2×SD (i.e., 10 ± 6.32, or between roughly 3.68 and 16.32) would encompass about 95% of the observations if the underlying process truly
        //follows a Poisson distribution. So, 16 is on the higher end but still within this range.
        assert!(
            expected_in_range >= 9, // A small range around 10 to allow for randomness
            "Expected around 95% events in the range. got {}",
            expected_in_range
        );
    }

    #[test]
    #[should_panic(expected = "Rate should not exceed 1e9 operations per second")]
    fn test_rate_exceeds_limit() {
        let rate = 1e10;
        let _scheduler = PoissonScheduler::new(rate);
    }

    #[test]
    fn test_wait_until() {
        // Given
        let delay = Duration::from_millis(100);
        let target_time = Instant::now() + delay;

        // When
        let start_time = Instant::now();
        PoissonScheduler::wait_until(target_time);
        let elapsed_time = start_time.elapsed();

        // Then
        assert!(
            elapsed_time >= delay,
            "Wait function did not delay for at least the target duration"
        );
    }
}
