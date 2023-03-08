use std::{thread, time::Duration};
fn main() {
    let new_thread = thread::spawn(move|| {
        thread::spawn(move || {
            loop {
                println!("I'm a new thread.");
            }
        });
    });

    new_thread.join().unwrap();
    println!("Child thread is finish!");
    thread::sleep(Duration::from_millis(100));
}