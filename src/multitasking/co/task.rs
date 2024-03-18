use core::{
    cell::UnsafeCell,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use alloc::{boxed::Box, sync::Arc};
use crossbeam_queue::ArrayQueue;
use futures_util::task::ArcWake;

pub mod keyboard;

pub struct Task {
    _future: UnsafeCell<Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>>,
    _task_queue: Arc<ArrayQueue<Arc<Task>>>,
}

unsafe impl Send for Task {}
unsafe impl Sync for Task {}

impl Task {
    pub fn new(
        future: impl Future<Output = ()> + Send + Sync + 'static,
        queue: Arc<ArrayQueue<Arc<Task>>>,
    ) -> Task {
        Task {
            _future: UnsafeCell::new(Box::pin(future)),
            _task_queue: queue,
        }
    }

    pub fn poll(&self, cx: &mut Context) -> Poll<()> {
        unsafe { (*self._future.get()).as_mut().poll(cx) }
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &alloc::sync::Arc<Self>) {
        arc_self
            ._task_queue
            .push(arc_self.clone())
            .unwrap_or_else(|_| {
                panic!("task queue full");
            });
    }
}
