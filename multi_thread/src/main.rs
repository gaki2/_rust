use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let m = Arc::new(Mutex::new(0));
    let m_for_sub_thread = m.clone();

    let handler = thread::spawn(move|| {
        let mutex = m_for_sub_thread.clone();
        let mut num = mutex.lock().unwrap();
        *num += 1;
        println!("In spawned thread, {}", num);
    });

    {
        let mut num2 = m.lock().unwrap();
        *num2 += 1;
        println!("In main thread, {}", num2);
    }

    handler.join().unwrap();
}