mod philosopher;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{ Instant};
use std::env;

const INVALID_ARGS_MESSAGE: &str = "인자를 입력해주세요.";

fn get_left_fork(name: u32, total: u32) -> usize {
    (name - 1) as usize
}

fn get_right_fork(name: u32, total: u32) -> usize {
    if name == total {
        return 0;
    }
    name as usize
}

fn main() {
    let num_of_philo = env::args().nth(1).expect(INVALID_ARGS_MESSAGE).parse::<u32>().expect("0 이상의 정수");
    let time_to_die = env::args().nth(2).expect(INVALID_ARGS_MESSAGE).parse::<u32>().expect("0 이상의 정수");
    let eating_duration = env::args().nth(3).expect(INVALID_ARGS_MESSAGE).parse::<u32>().expect("0 이상의 정수");
    let sleeping_duration = env::args().nth(4).expect(INVALID_ARGS_MESSAGE).parse::<u32>().expect("0 이상의 정수");

    let forks = Arc::new((0..num_of_philo).map(|x| Mutex::new(x)).collect::<Vec<_>>());
    let philosophers = Arc::new((0..num_of_philo).map(|x| {
        // 이름은 1부터 시작
        let name = x + 1;
        return philosopher::PhiloSopher::new((name).to_string().as_str(), get_left_fork(name, num_of_philo), get_right_fork(name, num_of_philo), eating_duration, sleeping_duration, time_to_die);
    }).collect::<Vec<_>>());
    let absolute_time = Arc::new(Instant::now());

    let mut handles = vec![];
    // 홀수번째 먼저 시작
    for i in 0..num_of_philo {
        if i % 2 == 0 {
            let forks_clone = forks.clone();
            let philosophers_clone = philosophers.clone();
            let absolute_time_clone = absolute_time.clone();

            handles.push(thread::spawn(move|| {
                philosophers_clone[i as usize].live(forks_clone.as_ref(), absolute_time_clone.as_ref());
            }));
        }
    }

    for j in 0..num_of_philo {
        if j % 2 == 1 {
            let forks_clone = forks.clone();
            let philosophers_clone = philosophers.clone();
            let absolute_time_clone = absolute_time.clone();

            handles.push(thread::spawn(move || {
                philosophers_clone[j as usize].live(&forks_clone.as_ref(), absolute_time_clone.as_ref());
            }));
        }
    }

    for handle in handles {
        handle.join();
    }
}



