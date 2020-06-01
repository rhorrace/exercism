use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut converted: BTreeMap<char, i32> = BTreeMap::new();
    for (k, v) in h.iter() {
        for letter in v.iter() {
            let lowercased: char = letter.to_ascii_lowercase();
            converted.insert(lowercased, *k);
        }
    }
    converted
}
