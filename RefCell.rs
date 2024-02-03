#![allow(warnings)]
trait Messenger {
    fn send(&self, msg: &str);
}

// any type T that implement Messenger trait, can be a part
struct LimitTracker<'a, T>
    where
        T: Messenger,
{
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        /// Constructor
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f32 / self.max as f32;

        // send/transfer the message to data_type T(Mock Messenger here)'s send message
        if percentage_of_max >= 1.0 {
            self.messenger.send("Quota Exceed!");
        } else if percentage_of_max >= 9.0 {
            self.messenger.send("Warning! 90% Quota done.");
        }
        else if  percentage_of_max >= 7.5 {
            self.messenger.send("Upto 75% of Quota Done.");
        }
        else {
            self.messenger.send("No Problem anymore.");
        }
    }
}

#[cfg(test)]
mod tests{
    use std::cell::RefCell;
    use crate::{LimitTracker, Messenger};

    struct MockMessenger {
        messages: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                messages: RefCell::new(vec![]),
            }
        }
    }
    // functions/behaviours of Messenger are part of MockMessenger
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            /// borrow messages(RefCell) as mut and push/change something
            self.messages.borrow_mut().push(message.to_string());
            println!("{}",message);
        }
    }
    #[test]
    fn basis() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger,100);

        limit_tracker.set_value(50);
        limit_tracker.set_value(100);

        // testing by lengths
        assert_eq!(mock_messenger.messages.borrow().len(), 2);
    }
}

// TODO RefCell with RC
