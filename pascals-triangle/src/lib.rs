pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn row(n: u32) -> Vec<u32> {
        let mut result = vec![1];
        let next_n = n + 1;

        for k in 1..=n {
            let last_value = *result.last().unwrap();
            let next_value = (last_value as f32 * ((next_n - k) as f32 / k as f32)) as u32;
            result.push(next_value);
        }

        result
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.row_count).map(PascalsTriangle::row).collect()
    }
}
