fn use_memo<F, A, R>(callback: F, args: &[A]) -> R
    where F: FnOnce(&[A]) -> R,
    R: Sized,
{
    callback(args)
}

struct Computation<A, R: Default> {
    callback: Box<dyn Fn(&A) -> R>,
    args: A,
    result: R,
    use_cached: bool,
}

impl<A, R: Default> Computation<A, R> {
    fn prepare(callback: Box<dyn Fn(&A) -> R>, args: A) -> Self {
        Self {
            callback,
            args,
            result: R::default(),
            use_cached: false,
        }
    }

    fn exec(&mut self) -> &R {
        match self.use_cached {
            true => &self.result,
            false => {
                let computation_result = (&self.callback)(&self.args);

                self.use_cached = true;

                self.result = computation_result;

                &self.result
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn most_basic_case_works() {
        struct MyComputationArgs(u64, u32);

        let my_args = MyComputationArgs(7, 3);
        let my_computation_fn = |args: &MyComputationArgs| args.0 * (args.1 as u64);

        let mut my_computiation = Computation::prepare(
            Box::new(my_computation_fn),
            my_args
        );

        let my_result = my_computiation.exec();

        struct MyComputationArgsStr<'a>(&'a str, &'a str);

        let my_str_args = MyComputationArgsStr("hey".into(), "there".into());
        let my_str_computation_fn = |args: &MyComputationArgsStr| {
            let mut r = String::from(args.0);
            r.push_str(args.1);
            r
        };

        let mut my_str_computiation = Computation::prepare(
            Box::new(my_str_computation_fn),
            my_str_args
        );

        // TODO: make sure you can the same computation configuration multiple times,
        // but only the very first call should trigger the computation
        // while the other calls should use cached value

        // let my_str_result = &my_str_computiation.exec();
        // let my_str_result2 = &my_str_computiation.clone().exec();
        //
        // assert!(my_str_result == my_str_result2);

        // let computed_value = use_memo(computation_fn, &[4]);
        //
        // assert!(computation_fn(&[4]) == computed_value);
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
