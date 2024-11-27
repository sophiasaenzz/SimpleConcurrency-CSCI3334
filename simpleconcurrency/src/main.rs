/*Spawning Threads and Joining Them

Spawns 3 threads.
Each thread should print its identifier (e.g., Thread 1, Thread 2, etc.).
The main thread should wait for all threads to complete their execution.
After all threads have finished, the main thread should print "All threads completed."

Assignment 2: Sharing Counter Data Between Threads
Define a shared counter that starts from zero.
Spawn 5 threads where each thread increments the counter by 1, 10 times (just a for loop), which should result in the counter having the value of 50 at the end.
Use Arc and Mutex to share and safely update the counter across threads.
The main thread should print the final value of the counter after all threads have completed their execution. */

use std::thread::JoinHandle;


fn spawning_threads() {
    use std::thread;

    let mut steps = vec![];

    for i in 0..3 {
        let handler = thread::spawn(move || {
            println!("Thread {}", i);
        });
        steps.push(handler);
    }

    for handle in steps {
        handle.join().unwrap();
    }

    println!("All threads completed.");
}

fn sending_data_across_threads() {
    extern crate rand; // 0.8.5
    use std::sync::{Arc, Mutex};

    let counter = Arc::new(Mutex::new(0));
    let mut steps = vec![];

    use std::thread;
    // multiproducer, single consumer
    use std::sync::mpsc::channel;

    let (sender,reciever) = channel();

    for i in 0..10 {
        let sender = sender.clone();
        thread::spawn(move || {
            println!("sending: {}",i);
            sender.send(i).unwrap(); // any data could be passed to reciever
            // as well as sending could fail
        });
    }

    for _ in 0..10 {
        let msg = reciever.recv().unwrap();
        println!(" recieved {}", msg );
    }
    // what is important to notice, data will be send and recieved in random order
    // but you will get them in exact order, just be aware of potential queue

    // basically CPU whim
    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handler = thread::spawn(move || {
            for _ in 0..10 {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        });
        steps.push(handler);
    }

    for handle in steps {
        handle.join().unwrap();
    }

    println!("Counter: {}", *counter.lock().unwrap());

}


fn main() {
    //println!("Hello, world!");
    //spawning_threads();
    sending_data_across_threads();
}
