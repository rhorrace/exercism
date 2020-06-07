use std::fmt::Debug;
use std::hash::Hash;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug, PartialEq)]
pub struct CustomSet<T: Eq + Hash + Copy + Debug> {
    // This field is here to make the template compile and not to
    // complain about unused type parameter 'T'. Once you start
    // solving the exercise, delete this field and the 'std::marker::PhantomData'
    // import.
    values: HashSet<T>,
}

impl<T: Eq + Hash + Copy + Debug> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut values: HashSet<T> = HashSet::new();
        input.iter()
            .for_each(|&v| {
                values.insert(v);
            });
        Self {
            values
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.values.contains(element)
    }

    pub fn add(&mut self, element: T) {
        self.values.insert(element);
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.values.is_subset(&other.values)
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.values.is_disjoint(&other.values)
    }

    pub fn intersection(&self, other: &Self) -> Self {
        self.values.intersection(&other.values)
            .map(|&v| v)
            .collect()
    }

    pub fn difference(&self, other: &Self) -> Self {
        self.values.difference(&other.values)
            .map(|&v| v)
            .collect()
    }

    pub fn union(&self, other: &Self) -> Self {
        self.values.union(&other.values)
            .map(|&v| v)
            .collect()
    }
}

impl<T> FromIterator<T> for CustomSet<T>
where
    T: Eq + Hash + Copy + Debug,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut c = CustomSet::new(&[]);

        for i in iter {
            c.add(i);
        }

        c
    }
}