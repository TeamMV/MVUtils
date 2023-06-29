use std::any::Any;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Mutex;
use mvutils::lazy;
use mvutils::utils::{Recover, SplitInto};
use mvutils::once::Lazy;

lazy! {
    static THRESHOLD: Mutex<usize> = 5.into();
}

/// Sets the threshold for parallelization. If the length of the vector is less than or equal to the threshold,
/// the vector will be processed sequentially on a single thread.
pub fn set_threshold(threshold: usize) {
    //We use the `recover` function which I made, all it does it ignore the poison error. This is safe here
    //since we end up changing the value anyways, and it is only a usize, not some complex type, so a poison
    //won't have any effect on the rest of the program.
    *THRESHOLD.lock().recover() = threshold;
}

type PanicValue = Box<dyn Any + Send + 'static>;

pub trait ParallelizeTasks<T: Send + 'static> {
    /// Run a function over all the elements of the vec, and this will run it on multiple threads if the length of
    /// the vector is greater than the threshold.
    fn run_parallel<R: Send + 'static>(self, f: fn(T) -> R) -> Result<Vec<R>, PanicValue>;
}

impl<T: Send + 'static> ParallelizeTasks<T> for Vec<T> {
    fn run_parallel<R: Send + 'static>(self, f: fn(T) -> R) -> Result<Vec<R>, PanicValue> {
        let threshold = *THRESHOLD.lock().unwrap_or_else(|err| {
            let mut inner = err.into_inner();
            *inner = 5;
            inner
        });
        if self.len() <= threshold {
            catch_unwind(AssertUnwindSafe(|| self.into_iter().map(f).collect()))
        } else {
            let threads = num_cpus::get() - 2;
            let chunks = self.split_into(threads);
            let mut handles = Vec::with_capacity(threads);
            for chunk in chunks {
                let handle = std::thread::spawn(move || {
                    chunk.into_iter().map(f).collect::<Vec<R>>()
                });
                handles.push(handle);
            }
            let mut result = Vec::new();
            for handle in handles {
                result.append(&mut handle.join()?);
            }
            Ok(result)
        }
    }
}