use core::task::{Context, Poll};

use alloc::sync::Arc;
use crossbeam_queue::ArrayQueue;
use futures_util::{task::waker_ref, Future};

use crate::multitasking::co::task::Task;

pub struct Executor {
    _task_queue: Arc<ArrayQueue<Arc<Task>>>,
}

impl Executor {
    pub fn new() -> Executor {
        Executor {
            _task_queue: Arc::new(ArrayQueue::new(100)),
        }
    }

    pub fn spawn(&self, task: impl Future<Output = ()> + Send + Sync + 'static) {
        let task = Arc::new(Task::new(task, self._task_queue.clone()));

        self._task_queue
            .push(task)
            .unwrap_or_else(|_| panic!("task queue full"));
    }

    pub fn run(&self) -> ! {
        loop {
            self.run_ready_task();
            self.sleep_if_idle();
        }
    }

    fn run_ready_task(&self) {
        while let Some(task) = self._task_queue.pop() {
            let waker = waker_ref(&task);
            let mut cx = Context::from_waker(&waker);

            match task.poll(&mut cx) {
                Poll::Pending => {}
                Poll::Ready(_) => {}
            };
        }
    }

    fn sleep_if_idle(&self) {
        x86_64::instructions::interrupts::disable();
        if self._task_queue.is_empty() {
            x86_64::instructions::interrupts::enable_and_hlt();
        } else {
            x86_64::instructions::interrupts::enable();
        }
    }
}
