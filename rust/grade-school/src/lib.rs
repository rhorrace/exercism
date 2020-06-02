use std::collections::btree_map::BTreeMap;

pub struct School {
    students: BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            students: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if let Some(g) = self.students.get_mut(&grade) {
            g.push(student.to_string());
            g.sort_by(|a, b| a.cmp(b));
        } else {
            self.students.insert(grade, vec![student.to_string()]);
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        self.students.keys()
            .map(|&g| g)
            .collect::<Vec<u32>>()
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        match self.students.get(&grade) {
            Some(g) => Some(g.clone()),
            None => None
        }
    }
}
