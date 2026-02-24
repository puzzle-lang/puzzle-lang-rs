use std::time::{Duration, Instant};

pub fn measure_time<F: Fn()>(block: F) -> Duration {
    let start = Instant::now();
    block();
    start.elapsed()
}

pub struct TimedValue<T> {
    pub duration: Duration,
    pub value: T,
}

pub fn measure_timed_value<T, F: Fn() -> T>(block: F) -> TimedValue<T> {
    let start = Instant::now();
    let value = block();
    TimedValue {
        duration: start.elapsed(),
        value,
    }
}
