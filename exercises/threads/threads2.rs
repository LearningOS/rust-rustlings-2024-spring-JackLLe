// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.



use std::sync::{Arc, Mutex};
use std::thread;

struct JobStatus {
    jobs_completed: u32,
}

impl JobStatus {
    fn increment(&mut self) {
        self.jobs_completed += 1;
    }
}

fn main() {
    let total = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));

    let num_threads = 4;

    let mut handles = vec![];

    for _ in 0..num_threads {
        let total_clone = Arc::clone(&total);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let mut status = total_clone.lock().unwrap();
                status.increment();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let total_status = total.lock().unwrap();
    println!("Total jobs completed: {}", total_status.jobs_completed);
}
