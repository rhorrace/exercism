pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let row_max = minefield.len() as isize;
    (0..row_max).map(|row_idx| {
        let col_max = minefield[row_idx as usize].len() as isize;
        (0..col_max).map(|col_idx| {
            match minefield[row_idx as usize].as_bytes()[col_idx as usize] {
                b'*' => '*'.to_string(),
                _ => {
                    let mut count: u8 = 0;
                    (-1..=1).for_each(|row_offset| {
                        (-1..=1).for_each(|col_offset| {
                            let row = row_idx - row_offset;
                            let col = col_idx - col_offset;
                            if row >= 0
                                && col >= 0
                                && row < row_max
                                && col < col_max
                                && !(col_offset == 0 && row_offset == 0)
                                && minefield[(row) as usize].as_bytes()[(col) as usize] == b'*'
                            {
                                count += 1;
                            }
                        });
                    });
                    match
                    count {
                        0 => ' '.to_string(),
                        _ => count.to_string(),
                    }
                }
            }
        }).collect()
    }).collect()
}
