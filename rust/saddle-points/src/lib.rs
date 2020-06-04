pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let input_t: Vec<Vec<u64>> = (0..input[0].len()).map(|c|
        (0..input.len()).map(|r| input[r][c].clone())
            .collect::<Vec<u64>>())
        .collect();

    let max_vals: Option<Vec<&u64>> = input.clone()
        .iter()
        .map(|r| r.iter()
            .max())
        .collect();

    let min_vals: Option<Vec<&u64>> = input_t.iter()
        .map(|c| c.iter()
            .min())
        .collect();

    max_vals.map(|rows| min_vals.map(|cols|
        input.iter()
            .enumerate()
            .flat_map(|(r, row)|
                row.iter()
                    .enumerate()
                    .map(move |(c, v)| ((r, c), *v)))
            .filter(|((r, c), v)| cols[*c] == v && rows[*r] == v)
            .map(|(q, _)| q)
            .collect()))
        .flatten()
        .unwrap_or(Vec::new())
}
