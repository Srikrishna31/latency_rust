use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use latency_rust::PeriodicGate;

const CALLS_PER_BATCH: u32 = 50_000;
const PERIOD: u32 = 50;

fn benchmark_poll_batch(c: &mut Criterion) {
    c.bench_function("periodic_gate_poll_50000_due_calls", |bencher| {
        bencher.iter(|| {
            let mut gate = PeriodicGate::new(PERIOD, PERIOD).unwrap();
            let mut now = PERIOD;
            let mut activations = 0_u32;

            for _ in 0..CALLS_PER_BATCH {
                if black_box(gate.poll(black_box(now))).is_some() {
                    activations += 1;
                }
                now = now.wrapping_add(PERIOD);
            }

            black_box(activations);
        });
    });
}

criterion_group!(benches, benchmark_poll_batch);
criterion_main!(benches);
