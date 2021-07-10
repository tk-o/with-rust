use std::collections::HashMap;

fn use_memo<F, A, R>(callback: F, args: &[A]) -> R
    where F: FnOnce(&[A]) -> R,
    R: Sized,
{
    callback(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn most_basic_case_works() {
        let computation_fn = |x: &[u8]| x[0] + 2;
        let computed_value = use_memo(computation_fn, &[4]);

        assert!(computation_fn(&[4]) == computed_value);
    }

    #[test]
    fn long_task_case_works() {
        use std::time::{Duration, Instant};

        const LONG_TASK_DURATION: u128 = 3000;

        let long_computation_fn = |x: &[u8]| {
            std::thread::sleep(Duration::from_millis(LONG_TASK_DURATION as u64));

            x[0] + 2
        };

        let start = Instant::now();
        let _computed_value = use_memo(long_computation_fn, &[4]);
        let duration = start.elapsed();

        println!("First run {:?}", duration.as_millis() );

        assert!(duration.as_millis() > LONG_TASK_DURATION);

        let start = Instant::now();
        let computed_value = use_memo(long_computation_fn, &[4]);
        let duration = start.elapsed();

        println!("Second run {:?}",duration.as_millis() );
        // TODO: make this test case work
        assert!(duration.as_millis() < LONG_TASK_DURATION);

        assert!(long_computation_fn(&[4]) == computed_value);
    }
}
