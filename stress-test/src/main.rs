use api_usecase::api_usecase;
use std::sync::atomic;
use std::time::Instant;
use tokio::task::JoinSet;

mod api_usecase;

const THREAD_COUNT: usize = 1;
const WOKER_COUNT: usize = 8; // per-thread

static TOTAL_API_REQUESTS: atomic::AtomicU64 = atomic::AtomicU64::new(0);
static API_FAILURES: atomic::AtomicU64 = atomic::AtomicU64::new(0);
static CYCLES_SUCCESS: atomic::AtomicU64 = atomic::AtomicU64::new(0);

async fn worker() {
    let mut set: JoinSet<Result<(), reqwest::Error>> = JoinSet::new();

    let spawn_one = |set: &mut JoinSet<Result<(), reqwest::Error>>| {
        TOTAL_API_REQUESTS.fetch_add(1, atomic::Ordering::Relaxed);
        set.spawn(async { api_usecase().await });
    };

    for _ in 0..WOKER_COUNT {
        spawn_one(&mut set);
    }

    loop {
        match set.join_next().await {
            Some(Ok(_)) => {
                CYCLES_SUCCESS.fetch_add(1, atomic::Ordering::Relaxed);
                spawn_one(&mut set);
            }
            Some(Err(_)) => {
                API_FAILURES.fetch_add(1, atomic::Ordering::Relaxed);
                spawn_one(&mut set);
            }
            None => break,
        }
    }
}

fn print_stats(started_at: Instant) {
    let elapsed = started_at.elapsed().as_secs_f64().max(0.000_001);
    let total = TOTAL_API_REQUESTS.load(atomic::Ordering::Relaxed);
    let failures = API_FAILURES.load(atomic::Ordering::Relaxed);
    let cycles = CYCLES_SUCCESS.load(atomic::Ordering::Relaxed);
    let success = total.saturating_sub(failures);
    let rps = total as f64 / elapsed;
    let failure_rate = if total == 0 {
        0.0
    } else {
        failures as f64 / total as f64 * 100.0
    };

    println!("--- stress-test stats ---");
    println!("elapsed_sec={elapsed:.2}");
    println!("requests_total={total}");
    println!("requests_ok={success}");
    println!("requests_failed={failures}");
    println!("request_rate_rps={rps:.2}");
    println!("failure_rate_pct={failure_rate:.2}");
    println!("cycles_success={cycles}");
}

#[tokio::main]
async fn main() {
    let started_at = Instant::now();
    for _ in 0..THREAD_COUNT {
        tokio::spawn(worker());
    }

    // Keep running until Ctrl+C, then print stats and exit.
    if tokio::signal::ctrl_c().await.is_ok() {
        print_stats(started_at);
    }
}
