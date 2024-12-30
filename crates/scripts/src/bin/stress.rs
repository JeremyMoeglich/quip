use std::{
    hint::black_box,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use scripts::fuzz_possibilities;

fn main() {
    let total = Arc::new(AtomicU64::new(0));
    let handle = thread::spawn({
        let total = Arc::clone(&total);
        move || {
            fuzz_possibilities(|new_tokens| {
                black_box(new_tokens);
                // println!(
                //     "new_tokens: {:?}",
                //     new_tokens
                //         .iter()
                //         .map(|t| t.kind().kind_name())
                //         .collect::<Vec<_>>()
                //         .join(", ")
                // );
                total.fetch_add(new_tokens.len() as u64, Ordering::SeqCst);
            });
        }
    });

    let mut last_total = 0;
    let mut last_time = std::time::Instant::now();

    loop {
        thread::sleep(Duration::from_secs(1));

        if handle.is_finished() {
            break;
        }

        let current_total = total.load(Ordering::SeqCst);
        let current_time = std::time::Instant::now();

        let delta_total = current_total - last_total;
        let delta_time = current_time.duration_since(last_time).as_secs_f64();

        let per_second = delta_total as f64 / delta_time;

        println!(
            "Total: {}, Tokens per second: {:.2}",
            current_total, per_second
        );

        last_total = current_total;
        last_time = current_time;
    }
}
