pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if input.is_empty() {
        return vec![];
    }

    let num_columns = input[0].len();
    let num_rows = input.len();

    if num_columns == 0 {
        return vec![];
    }

    //create vecs that represent columns for easier iteration
    let mut columns = (0..num_columns)
        .map(|_| vec![&0; num_rows])
        .collect::<Vec<Vec<&u64>>>();

    input.iter().enumerate().for_each(|(y, row)| {
        row.iter()
            .enumerate()
            .for_each(|(x, cell)| columns[x][y] = cell)
    });

    let col_min = &columns
        .iter()
        .map(|col| col.iter().min().copied().copied().unwrap())
        .collect::<Vec<u64>>();

    let row_max = &input
        .iter()
        .map(|row| row.iter().max().copied().unwrap())
        .collect::<Vec<u64>>();

    input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            let min = row_max[y];

            row.iter()
                .enumerate()
                .filter(move |(x, cell)| **cell == min && **cell == col_min[*x])
                .map(move |(x, _)| (y, x))
        })
        .collect()
}
