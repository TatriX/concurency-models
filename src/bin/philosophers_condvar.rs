extern crate rand;

use std::thread;
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;
use rand::{thread_rng, Rng};

const THINK_DURATION_MS: u64 = 10;
const EAT_DURATION_MS: u64 = 10;

struct Philosopher {
    id: usize,
    eating: bool,
    think_count: usize,
}

impl Philosopher {
    fn new(id: usize) -> Philosopher {
        Philosopher {
            id,
            think_count: 0,
            eating: false,
        }
    }
}

fn main() {
    const NUM: usize = 5;

    let philosophers = Arc::new(Mutex::new(
        (0..NUM).map(Philosopher::new).collect::<Vec<_>>(),
    ));
    let conditions = Arc::new((0..NUM).map(|_| Condvar::new()).collect::<Vec<_>>());

    fn left(id: usize) -> usize {
        (id + NUM - 1) % NUM
    }

    fn right(id: usize) -> usize {
        (id + 1) % NUM
    }

    (0..NUM)
        .map(|id| {
            let philosophers = philosophers.clone();
            let conditions = conditions.clone();
            thread::spawn(move || {
                let mut rng = thread_rng();
                loop {
                    // think
                    {
                        let philosophers = &mut philosophers.lock().unwrap();
                        let philosopher = &mut philosophers[id];
                        philosopher.eating = false;
                        philosopher.think_count += 1;
                        if philosopher.think_count % 10 == 0 {
                            println!(
                                "Philosopher {} has thought {} times",
                                philosopher.id, philosopher.think_count
                            );
                        }
                    }
                    conditions[left(id)].notify_one();
                    conditions[right(id)].notify_one();
                    thread::sleep(Duration::from_millis(rng.gen_range(0, THINK_DURATION_MS)));

                    // eat
                    {
                        let mut philosophers = philosophers.lock().unwrap();
                        let condition = &conditions[id];
                        while philosophers[left(id)].eating || philosophers[right(id)].eating {
                            philosophers = condition.wait(philosophers).unwrap();
                        }
                        let philosopher = &mut philosophers[id];
                        philosopher.eating = true;
                    }

                    thread::sleep(Duration::from_millis(rng.gen_range(0, EAT_DURATION_MS)));
                }
            })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|handle| handle.join().unwrap());
}
