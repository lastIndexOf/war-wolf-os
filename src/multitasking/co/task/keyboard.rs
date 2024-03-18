use core::task::Poll;

use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use futures_util::{task::AtomicWaker, Stream, StreamExt};
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

use crate::{print, println};

pub static SCAN_CODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
pub static WAKER: AtomicWaker = AtomicWaker::new();

pub struct ScanCodeStream {
    _private: (),
}

impl ScanCodeStream {
    pub fn new() -> Self {
        SCAN_CODE_QUEUE
            .try_init_once(|| ArrayQueue::new(100))
            .expect("SCAN_CODE_QUEUE already initialized");

        ScanCodeStream { _private: () }
    }
}

impl Stream for ScanCodeStream {
    type Item = u8;

    fn poll_next(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Option<Self::Item>> {
        let queue = SCAN_CODE_QUEUE
            .get()
            .expect("SCAN_CODE_QUEUE not initialized");

        if let Some(scan_code) = queue.pop() {
            return Poll::Ready(Some(scan_code));
        }

        WAKER.register(cx.waker());
        match queue.pop() {
            Some(scan_code) => {
                WAKER.take();
                Poll::Ready(Some(scan_code))
            }
            None => Poll::Pending,
        }
    }
}

pub(crate) fn add_scan_code_to_queue(scan_code: u8) {
    match SCAN_CODE_QUEUE.get() {
        Some(queue) => match queue.push(scan_code) {
            Ok(_) => {
                WAKER.wake();
            }
            Err(_) => println!("WARNING: SCAN_CODE_QUEUE full; dropping keyboard input"),
        },
        None => {
            println!("WARNING: SCAN_CODE_QUEUE not initialized");
        }
    }
}

pub async fn print_keycode() {
    let mut stream = ScanCodeStream::new();
    let mut keyboard = Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore,
    );

    while let Some(scan_code) = stream.next().await {
        if let Ok(Some(event)) = keyboard.add_byte(scan_code) {
            match keyboard.process_keyevent(event) {
                Some(DecodedKey::Unicode(cr)) => print!("{}", cr),
                Some(DecodedKey::RawKey(key)) => print!("{:?}", key),
                _ => {}
            }
        };
    }
}
