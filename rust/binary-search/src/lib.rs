use std::cmp::Ordering::*;

pub fn find<T: Ord>(array: &[T], key: T) -> Option<usize> {
    let mut l = 0;
    let mut h = array.len();
    let mut m = l + (h - l) / 2;

    while l < h {
        match key.cmp(&array[m]) {
            Less => h = m,
            Greater => l = m + 1,
            Equal => return Some(m),
        }
        m = l + (h - l) / 2;
    }
    None
}
