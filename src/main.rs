use std::env;
use std::process::exit;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;
use rand::Rng;

fn main() {
    // collect the arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage {} [philosophers] [apetite]", args[0]);
        exit(1);
    }

    let philosophers: i32 = args[1].parse::<i32>().unwrap();
    let apetites: i32 = args[2].parse::<i32>().unwrap();

    if philosophers < 2 {
        println!("philosophers parameter should be an integer >= 2");
        exit(1);
    }
    if apetites < 0 {
        println!("philosophers parameter should be an integer >= 2");
        exit(1);
    }

    // array of mutexes instead of semaphores since they're "unsafe"
    let mut forks: Vec<Arc<Mutex<bool>>> = Vec::new();

    // initialize all the forks
    for _ in 0..(philosophers as usize) {
        forks.push(Arc::new(Mutex::new(true)));
    }

    let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();

    for i in 0..(philosophers) {
        // create threads for each of the philosophers
        let left_fork = Arc::clone(&forks[{
            let mut result = i -1;
            if result == -1 {
                result = philosophers - 1;
            }
            // return result in the form of usize to index the array
            result as usize
        }]);
        let right_fork = Arc::clone(&forks[i as usize]);
        threads.push(thread::spawn(move || {
            let mut rng = rand::thread_rng();
            // define logic for an individual philosopher or call a function to handle it
            // println!("LEFT: {}, RIGHT: {}", left_fork_index, right_fork_index);
            let mut eaten = 0;
            while eaten < apetites {
                // these mutexes have to be held in the thread's memory in order to be
                //  recognized as being LOCKED in a thread.
                let _left: MutexGuard<bool>;
                let _right: MutexGuard<bool>;

                // think (delay a second + some random couple hundred milliseconds)
                println!("Philosopher {} is thinking", i + 1);
                thread::sleep(Duration::new(0, rng.gen_range(3..10) * 100000000));

                if i % 2 == 0 {
                    _left = left_fork.lock().unwrap();
                    _right = right_fork.lock().unwrap();
                } else {
                    _right = right_fork.lock().unwrap();
                    _left = left_fork.lock().unwrap();
                }

                // eat (delay a second + some random couple hundred milliseconds)
                println!("Philosopher {} is eating", i + 1);
                thread::sleep(Duration::new(0, rng.gen_range(2..10) * 100000000));
                eaten += 1;
            }
        }));
    }

    // await completion of all threads
    for i in threads {
        i.join().unwrap();
    }
}
