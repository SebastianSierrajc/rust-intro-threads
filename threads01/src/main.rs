use std::thread;
use std::time::Duration;


fn main() {

    // creacion de un hilo
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the new thread!", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    for i in 1..10 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1000));
    }
}
