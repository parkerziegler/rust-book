pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You've are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up 90% of your quota.");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up 75% of your quota");
        }
    }
}

// In the tests below, we demonstrate the interior mutability pattern
// to allow for creating a MockMessenger struct to test the behavior
// of our LimitTracker.
#[cfg(test)]
mod tests {
    use super::*;
    // Bring RefCell into scope.
    use std::cell::RefCell;

    struct MockMessenger {
        // Use a RefCell to capture the messages sent to MockMessenger.
        // Using a RefCell allows us to mutate the vector internally (i.e.
        // call .push() on it) while signaling to the outside world that
        // the value is immutable.
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
        fn send(&self, msg: &str) {
            // Borrow the sent_messages vector as mutable so we can push to it.
            // Note that, just like mutable references, there can only be one mutable
            // borrow of a RefCell<T> at a time.
            self.sent_messages.borrow_mut().push(String::from(msg));

            // The commented out code below will throw an error at runtime, since
            // RefCell does not perform compile time checks on multiple mutable borrows.
            // let mut borrow_one = self.sent_messages.borrow_mut();
            // let mut borrow_two = self.sent_messages.borrow_mut();

            // borrow_one.push(String::from(msg));
            // borrow_two.push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // We can borrow sent_messages immutably below since we don't need to mutate
        // the underlying vector at all.
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
