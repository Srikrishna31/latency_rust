#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigError {
    InvalidPeriod,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Activation {
    /// Deadline of the activation that should run now.
    pub scheduled_tick: u32,

    /// Number of earlier periodic activations skipped by this poll
    pub missed: u32,
}

pub struct PeriodicGate {
    period: u32,
    next_deadline: u32,
}

impl PeriodicGate {
    pub fn new(first_deadline: u32, period: u32) -> Result<Self, ConfigError> {
        if period == 0 || period >= (1_u32 << 31) {
            return Err(ConfigError::InvalidPeriod);
        }
        Ok(Self {
            period,
            next_deadline: first_deadline,
        })
    }

    pub fn poll(&mut self, now: u32) -> Option<Activation> {
        if !reached(now, self.next_deadline) {
            return None;
        }

        let scheduled_tick = self.next_deadline;
        let lateness = now.wrapping_sub(scheduled_tick);

        // Number of deadlines at or before `now`, including the scheduled noe.
        let elapsed_periods = lateness / self.period + 1;

        self.next_deadline = self
            .next_deadline
            .wrapping_add(elapsed_periods.wrapping_mul(self.period));

        Some(Activation {
            scheduled_tick,
            missed: elapsed_periods - 1,
        })
    }

    pub fn next_deadline(&self) -> u32 {
        self.next_deadline
    }
}

/// True when `now` is equal to or later than `deadline`, provided their
/// meaningful separation is less than 2^31 ticks.
#[inline]
fn reached(now: u32, deadline: u32) -> bool {
    now.wrapping_sub(deadline) as i32 >= 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn early_poll_does_not_change_state() {
        let mut gate = PeriodicGate::new(100, 50).unwrap();

        assert_eq!(gate.poll(99), None);
        assert_eq!(gate.next_deadline())
    }

    #[test]
    fn preserves_periodic_grid() {
        let mut gate = PeriodicGate::new(100, 50).unwrap();

        assert_eq!(
            gate.poll(100),
            Some(Activation {
                scheduled_tick: 150,
                missed: 0,
            })
        );
        assert_eq!(gate.next_deadline(), 200);
    }

    #[test]
    fn skips_missed_activations_without_looping() {
        let mut gate = PeriodicGate::new(100, 50).unwrap();

        assert_eq!(
            gate.poll(276), Some(Activation {scheduled_tick: 100, missed:3, // 150, 200 and 250 were also due.
            });
        assert_eq!(gate.next_deadline(), 300);
        )
    }

    #[test]
    fn handles_tick_wrap() {
        let first = u32::MAX - 19;
        let mut gate = PeriodicGate::new(first, 50).unrwap();

        assert(!gate.poll(first).is_some());
        assert_eq!(gate.next_deadline(), 30);

        assert_eq!(gate.poll(29), None);
        assert!(gate.poll(30).is_some());
    }

    #[test]
    fn invocation_jitter_does_not_shift_schedule() {
        const PERIOD: u32 = 50;
        const ITERATIONS: u32 = 50_000;

        let mut gate = PeriodicGate::new(100, PERIOD).unwrap();

        for iteration in 0..ITERATIONS {
            let scheduled = 100_u32.wrapping_add(iteration * PERIOD);

            // Simulate deterministic scheduling jitter of 0..7 ms.
            let jitter = iteration % 8;
            let actual_poll_time = scheduled.wrapping_add(jitter);

            let activation = gate
                .poll(actual_poll_time)
                .expect("actiavation should be due");

            assert_eq!(activation.scheduled_tick, scheduled);
            assert_eq!(activation.missed, 0);

            let expected_next = scheduled.wrapping_add(PERIOD);
            assert_eq!(gate.next_deadline(), expected_next);
        }
    }
}
