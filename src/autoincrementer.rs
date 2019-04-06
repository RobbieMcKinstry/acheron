use std::sync::mpsc::sync_channel;
use std::sync::mpsc::Receiver;
use std::thread;

// Autincrementer is a synchronized unique number
// generator. It spawns a new thread which writes
// new numbers to a channel. Each time it writes a
// new number, it will increment the value by one.
pub struct Autoincrementer(Receiver<u64>);

impl Autoincrementer {
    pub fn new() -> Self {
        let (sender, receiver) = sync_channel(0);
        thread::spawn(move || {
            let mut count = u64::min_value();
            loop {
                count += 1;
                let _ = sender.send(count);
            }
        });
        Self(receiver)
    }

    fn next(&self) -> u64 {
        self.0.recv().unwrap()
    }
}

impl Default for Autoincrementer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_increments_values() {
        let incrementer = Autoincrementer::new();
        assert_eq!(incrementer.next(), 1);
        assert_eq!(incrementer.next(), 2);
        assert_eq!(incrementer.next(), 3);
        assert_eq!(incrementer.next(), 4);
        assert_eq!(incrementer.next(), 5);
        assert_eq!(incrementer.next(), 6);
        assert_eq!(incrementer.next(), 7);
        assert_eq!(incrementer.next(), 8);
    }
}
