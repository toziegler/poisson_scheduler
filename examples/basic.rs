use poisson_scheduler::PoissonScheduler;
use std::time::Duration;
fn main() {
    // Example usage
    let duration_seconds = 5;
    let operations = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let op = operations.clone();
    let t = std::thread::spawn(move || {
        let mut scheduler = PoissonScheduler::new(500.0); // 500 events per second
        scheduler.run(Duration::from_secs(duration_seconds), |_| {
            op.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        });
    });

    for _ in 0..duration_seconds {
        std::thread::sleep(Duration::from_secs(1));
        println!(
            "Operations executed {}",
            operations.load(std::sync::atomic::Ordering::SeqCst)
        );
    }

    t.join().unwrap();

    println!(
        "All Operations executed {}",
        operations.load(std::sync::atomic::Ordering::SeqCst)
    );
}
