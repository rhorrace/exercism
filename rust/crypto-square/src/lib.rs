pub fn encrypt(input: &str) -> String {
    let mut condensed: String = condense(input);

    if condensed.is_empty() {
        return condensed;
    }

    let column_size = get_chunk_value(condensed.len());

    if condensed.len() % column_size != 0 {
        condensed += vec![" "; column_size - condensed.len() % column_size].join("")
            .as_str();
    }

    let segments = segments(condensed, column_size);
    let row_size = segments.len();

    transpose(segments, column_size)
        .iter()
        .flat_map(|chars| chars)
        .map(|c| *c)
        .collect::<Vec<char>>()
        .chunks(row_size)
        .map(|chunk| chunk.iter()
            .collect::<String>()
            .to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn condense(input: &str) -> String {
    input.to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect()
}

fn segments(input: String, chunk_size: usize) -> Vec<String> {
    input.chars()
        .collect::<Vec<char>>()
        .chunks(chunk_size)
        .map(|chunk| chunk.iter()
            .collect::<String>())
        .collect()
}

fn transpose(strings: Vec<String>, chunk_size: usize) -> Vec<Vec<char>> {
    let mut transposed: Vec<Vec<char>> = vec![vec![]; chunk_size];

    for string in strings.iter() {
        for (i, c) in string.chars()
            .enumerate() {
            transposed[i].push(c);
        }
    }
    transposed
}

fn get_chunk_value(n: usize) -> usize {
    let size = (n as f64).sqrt();
    if size.floor() == size {
        size as usize
    } else {
        size as usize + 1
    }
}
