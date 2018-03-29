extern crate rand;

use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use rand::{thread_rng, Rng};

const THINK_DURATION_MS: u64 = 10;
const EAT_DURATION_MS: u64 = 10;

#[derive(Clone)]
struct Chopstick {}

type ChopstickHandle = Arc<Mutex<Chopstick>>;

struct Philosopher {
    id: usize,
    think_count: usize,
    left: ChopstickHandle,
    right: ChopstickHandle,
}

impl Philosopher {
    fn new(id: usize, left: ChopstickHandle, right: ChopstickHandle) -> Philosopher {
        Philosopher {
            think_count: 0,
            id,
            left,
            right,
        }
    }

    fn think(&mut self) {
        self.think_count += 1;
        let mut rng = thread_rng();
        thread::sleep(Duration::from_millis(rng.gen_range(0, THINK_DURATION_MS)));
    }

    fn eat(&self) {
        let mut rng = thread_rng();
        let left = self.left.clone();
        let _left = left.lock().unwrap();
        let right = self.right.clone();
        let _second = right.lock().unwrap();
        thread::sleep(Duration::from_millis(rng.gen_range(0, EAT_DURATION_MS)));
    }
}

fn main() {
    const NUM: usize = 5;

    let chopsticks = std::iter::repeat(Chopstick {})
        .map(Mutex::new)
        .map(Arc::new)
        .take(NUM)
        .collect::<Vec<_>>();

    (0..NUM)
        .map(|id| {
            let left = chopsticks[id].clone();
            let right = chopsticks[(id + 1) % NUM].clone();

            let mut philosopher = Philosopher::new(id, left, right);
            thread::spawn(move || loop {
                philosopher.think();
                if philosopher.think_count % 10 == 0 {
                    println!(
                        "Philosopher {} has thought {} times",
                        philosopher.id, philosopher.think_count
                    );
                }
                philosopher.eat();
            })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|handle| handle.join().unwrap());
}
