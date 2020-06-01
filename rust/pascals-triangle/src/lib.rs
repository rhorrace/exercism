pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut all_rows: Vec<Vec<u32>> = Vec::new();
        let mut current_row: Vec<u32> = Vec::new();
        for row in 1..=row_count {
            let mut c: u32 = 1;
            for i in 1..=row {
                current_row.push(c);
                c = c * (row - i) / i;
            }
            all_rows.push(current_row.clone());
            current_row.clear();
        }
        PascalsTriangle {
            rows: all_rows,
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
