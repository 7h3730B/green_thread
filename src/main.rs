use green_thread::*;
use std::{ptr, thread::sleep, time::Duration};

const DEFAULT_STACK_SIZE: usize = 1024 * 1024 * 2;

fn main() {
    let mut runtime = Runtime::new(DEFAULT_STACK_SIZE);
    runtime.init();

    runtime.spawn(|| {
        println!("THREAD 1 STARTING");
        let id = 1;
        for i in 0..10 {
            println!("thread: {} counter: {}", id, i);
            yield_thread();
        }
        println!("THREAD 1 FINISHED");
    });

    runtime.spawn(|| {
        println!("THREAD 2 STARTING");
        let id = 2;
        for i in 0..15 {
            println!("thread: {} counter: {}", id, i);
            yield_thread();
        }
        println!("THREAD 2 FINISHED");
    });

    runtime.spawn(|| {
        println!("THREAD 3 STARTING");
        let id = 3;
        for i in 0..15 {
            println!("thread: {} counter: {}", id, i);
            yield_thread();
        }
        println!("THREAD 3 FINISHED");
    });

    runtime.run();
}
