use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::task::AbortHandle;

/// Background tasks indexed by key. Spawning a task for a key that already has one aborts
/// the previous task, so a timer (scheduled closure, reminder...) can never run twice,
/// however many times it gets scheduled.
pub struct KeyedTasks<K> {
    tasks: Mutex<HashMap<K, (u64, AbortHandle)>>,
    next_generation: AtomicU64,
}

impl<K: Eq + Hash + Clone + Send + 'static> KeyedTasks<K> {
    pub fn new() -> Self {
        Self {
            tasks: Mutex::new(HashMap::new()),
            next_generation: AtomicU64::new(0),
        }
    }

    pub fn spawn<F>(&'static self, key: K, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let generation = self.next_generation.fetch_add(1, Ordering::Relaxed);
        let cleanup_key = key.clone();

        // The lock is held across the spawn so the task can't finish (and clean up)
        // before its entry is inserted.
        let mut tasks = self.tasks.lock().expect("keyed tasks lock poisoned");
        let handle = tokio::spawn(async move {
            future.await;
            self.finish(&cleanup_key, generation);
        });

        if let Some((_, previous)) = tasks.insert(key, (generation, handle.abort_handle())) {
            // When a task reschedules itself, this aborts the caller; it has no await
            // left at that point, so it simply completes.
            previous.abort();
        }
    }

    fn finish(&self, key: &K, generation: u64) {
        let mut tasks = self.tasks.lock().expect("keyed tasks lock poisoned");
        if tasks.get(key).is_some_and(|(g, _)| *g == generation) {
            tasks.remove(key);
        }
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.tasks.lock().unwrap().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;
    use std::sync::atomic::AtomicUsize;
    use std::time::Duration;

    #[tokio::test]
    async fn rescheduling_a_key_replaces_the_previous_task() {
        static TASKS: LazyLock<KeyedTasks<u64>> = LazyLock::new(KeyedTasks::new);
        static RUNS: AtomicUsize = AtomicUsize::new(0);

        for _ in 0..5 {
            TASKS.spawn(1, async {
                tokio::time::sleep(Duration::from_millis(50)).await;
                RUNS.fetch_add(1, Ordering::SeqCst);
            });
        }
        TASKS.spawn(2, async {
            RUNS.fetch_add(1, Ordering::SeqCst);
        });

        tokio::time::sleep(Duration::from_millis(200)).await;

        assert_eq!(RUNS.load(Ordering::SeqCst), 2);
        assert_eq!(TASKS.len(), 0);
    }

    #[tokio::test]
    async fn a_task_can_reschedule_itself() {
        static TASKS: LazyLock<KeyedTasks<u64>> = LazyLock::new(KeyedTasks::new);
        static RUNS: AtomicUsize = AtomicUsize::new(0);

        fn schedule(remaining: usize) {
            TASKS.spawn(7, async move {
                tokio::time::sleep(Duration::from_millis(10)).await;
                RUNS.fetch_add(1, Ordering::SeqCst);
                if remaining > 0 {
                    schedule(remaining - 1);
                }
            });
        }

        schedule(2);
        tokio::time::sleep(Duration::from_millis(200)).await;

        assert_eq!(RUNS.load(Ordering::SeqCst), 3);
        assert_eq!(TASKS.len(), 0);
    }
}
