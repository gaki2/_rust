use std::rc::Rc;
use std::cell::RefCell;
pub trait Messenger {
    // &self 는 위 trait 을 구현한 유닛을 나타낸다.
    fn send(&self, msg: &str);
}

pub struct LimitTracker {
    messenger: Rc<dyn Messenger>,
    value: usize,
    max: usize,
}

impl LimitTracker {
    pub fn new(messenger: Rc<dyn Messenger>, max: usize) -> LimitTracker {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: RefCell::new(vec![]),
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        self.sent_messages.borrow_mut().push(String::from(message));
    }
}

fn main() {
    let mock_messenger = Rc::new(MockMessenger::new());
    let mut tracker = LimitTracker::new(mock_messenger.clone(), 100);
    tracker.set_value(99);
    let t = (*mock_messenger).sent_messages.borrow();
    println!("{:?}", t);
    // let k = t.sent_messages.borrow().len();
    println!("Hello, world!");
}
