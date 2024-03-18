use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use alloc::collections::VecDeque;

use crate::multitasking::co::task::Task;

pub struct BaseExecutor {
    _task_queue: VecDeque<Task>,
}

impl BaseExecutor {
    pub fn new() -> BaseExecutor {
        BaseExecutor {
            _task_queue: VecDeque::new(),
        }
    }

    pub fn spawn(&mut self, task: Task) {
        self._task_queue.push_back(task);
    }

    pub fn run(&mut self) {
        while let Some(mut task) = self._task_queue.pop_front() {
            let waker = waker();
            let mut cx = Context::from_waker(&waker);

            match task.poll(&mut cx) {
                Poll::Pending => self._task_queue.push_back(task),
                Poll::Ready(_) => {}
            }
        }
    }
}

fn raw_waker() -> RawWaker {
    fn clone(ptr: *const ()) -> RawWaker {
        raw_waker()
    }
    fn wake(ptr: *const ()) {}
    fn wake_by_ref(ptr: *const ()) {}

    let vtable = &RawWakerVTable::new(clone, wake, wake_by_ref, drop);
    RawWaker::new(0 as *const (), vtable)
}

fn waker() -> Waker {
    unsafe { Waker::from_raw(raw_waker()) }
}
