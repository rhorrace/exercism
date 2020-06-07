pub fn get_diamond(c: char) -> Vec<String> {
    let nchars = (c as u8 - b'A' + 1) as usize;
    let mut result = Vec::<String>::with_capacity(nchars+nchars-1);
    (0..nchars).for_each(|i| {
        let mut row = vec![b' '; nchars+nchars-1];
        row[nchars-1-i] = b'A' + i as u8;
        row[nchars-1+i] = b'A' + i as u8;
        result.push(String::from_utf8(row).unwrap());
    });
    let mut reversed: Vec<String> = result.clone()
        .into_iter()
        .rev()
        .skip(1)
        .collect();
    result.append(&mut reversed);
    result
}
