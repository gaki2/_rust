use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

pub struct PhiloSopher {
    name: String,
    left: usize,
    right: usize,
    eating_duration: u32,
    sleeping_duration: u32,
    time_to_die: u32,
}

impl PhiloSopher {
    pub fn new(name: &str,
           left: usize,
           right: usize,
           eating_duration: u32,
           sleeping_duration: u32,
           time_to_die: u32) -> PhiloSopher {
        PhiloSopher {
            name: name.to_string(),
            left,
            right,
            eating_duration,
            sleeping_duration,
            time_to_die,
        }
    }

    fn start_timer() {

    }

    fn eat(&self, forks: &Vec<Mutex<u32>>, absolute_time: &Instant) {
        let left_fork = forks[self.left].lock().unwrap();
        let right_fork = forks[self.right].lock().unwrap();
        println!("현재시간: {}초: {} is eating.", absolute_time.elapsed().as_secs(), self.name);
        thread::sleep(Duration::from_secs(self.eating_duration as u64));
    }

    fn sleep(&self, absolute_time: &Instant) {
        println!("현재시간: {}초: {} is sleeping.", absolute_time.elapsed().as_secs() ,self.name);
        thread::sleep(Duration::from_secs(self.sleeping_duration as u64));
    }

    fn think(&self, absolute_time: &Instant) {
        println!("현재시간: {}초: {} is thinking.", absolute_time.elapsed().as_secs() ,self.name);
    }

    fn die (&self) {
        println!("{} is died.", self.name);
    }

    pub fn live(&self, forks: &Vec<Mutex<u32>>, absolute_time: &Instant) {
        loop {
            self.think(absolute_time);
            self.eat(forks, absolute_time);
            self.sleep(absolute_time);
        }
    }
}