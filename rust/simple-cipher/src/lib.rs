use rand::Rng;

pub fn encode(key: &str, s: &str) -> Option<String> {
    if !is_valid_key(key) {
        return None;
    } else {
        Some(
            s.chars()
                .zip(key.chars().cycle())
                .map(|(c, k)| c as i16 - b'a' as i16 + k as i16 - b'a' as i16)
                .map(|c| c % 26)
                .map(|c| c + b'a' as i16)
                .map(|c| c as u8 as char)
                .collect::<String>()
        )
    }
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if !is_valid_key(key) {
        return None;
    }
    Some(
        s.chars()
            .zip(key.chars().cycle())
            .map(|(c, k)| (c as i16 - b'a' as i16) - (k as i16 - b'a' as i16))
            .map(|c|
                if c < 0 {
                    26 + c
                } else {
                    c % 26
                })
            .map(|c| c + b'a' as i16)
            .map(|c| c as u8 as char)
            .collect::<String>(),
    )
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = rand::thread_rng();
    let key = (0..=100).map(|_| rng.gen_range('a' as u8, 'z' as u8) as char)
        .collect::<String>();

    (key.to_owned(), encode(&key, s).unwrap())
}

pub fn is_valid_key(key: &str) -> bool {
    key.len() > 0
        && key.chars()
        .all(|x| x.is_ascii_lowercase())
}