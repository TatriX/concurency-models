extern crate rand;

use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use rand::{thread_rng, Rng};

const THINK_DURATION_MS: u64 = 10;
const EAT_DURATION_MS: u64 = 10;

#[derive(Debug, Clone)]
struct Chopstick {
    id: usize,
}

type ChopstickHandle = Arc<Mutex<Chopstick>>;

#[derive(Debug, Clone)]
struct Philosopher {
    id: usize,
    think_count: usize,
    first: ChopstickHandle,
    second: ChopstickHandle,
}

impl Philosopher {
    fn new(id: usize, first: ChopstickHandle, second: ChopstickHandle) -> Philosopher {
        let (first, second) = if first.lock().unwrap().id > second.lock().unwrap().id {
            (first, second)
        } else {
            (second, first)
        };
        Philosopher {
            think_count: 0,
            id,
            first,
            second,
        }
    }

    fn think(&mut self) {
        self.think_count += 1;
        let mut rng = thread_rng();
        thread::sleep(Duration::from_millis(rng.gen_range(0, THINK_DURATION_MS)));
    }

    fn eat(&self) {
        let mut rng = thread_rng();
        let first = self.first.clone();
        let _first = first.lock().unwrap();
        let second = self.second.clone();
        let _second = second.lock().unwrap();
        thread::sleep(Duration::from_millis(rng.gen_range(0, EAT_DURATION_MS)));
    }
}

fn main() {
    const NUM: usize = 5;

    let chopsticks: Vec<_> = (0..NUM)
        .map(|id| Arc::new(Mutex::new(Chopstick { id })))
        .collect();

    let mut philosopher_handles = vec![];
    for id in 0..NUM {
        let first = chopsticks[id].clone();
        let second = chopsticks[(id + 1) % NUM].clone();

        let mut philosopher = Philosopher::new(id, first, second);
        let handle = thread::spawn(move || loop {
            philosopher.think();
            if philosopher.think_count % 10 == 0 {
                println!(
                    "Philosopher {} has thought {} times",
                    philosopher.id, philosopher.think_count
                );
            }
            philosopher.eat();
        });
        philosopher_handles.push(handle);
    }

    philosopher_handles
        .into_iter()
        .for_each(|handle| handle.join().unwrap());
}
