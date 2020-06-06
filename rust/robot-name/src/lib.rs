extern crate rand;

use rand::{Rng, thread_rng};

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        let mut name: String = String::new();
        (0..2).for_each(|_| name.push(thread_rng()
            .gen_range(b'A', b'Z') as char));
        (0..3).for_each(|_| name.push(thread_rng()
            .gen_range(b'0', b'9') as char));
        Self {
            name: name.clone()
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn reset_name(&mut self) {
        loop {
            let mut name: String = String::new();
            (0..2).for_each(|_| name.push(thread_rng()
                .gen_range(b'A', b'Z') as char));
            (0..3).for_each(|_| name.push(thread_rng()
                .gen_range(b'0', b'9') as char));
            if name != self.name {
                self.name = name;
                break;
            }
        }
    }
}
