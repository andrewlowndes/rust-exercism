pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    match size {
        0 => return vec![],
        1 => return vec![vec![1]],
        _ => {}
    }

    let mut results = (0..size)
        .map(|_| vec![0; size as usize])
        .collect::<Vec<_>>();

    let mut left = 0;
    let mut right = (size - 1) as usize;
    let mut top = 0;
    let mut bottom = (size - 1) as usize;

    let mut i = 1;
    let max_i = size * size;

    while i <= max_i {
        (left..=right).for_each(|x| {
            results[top][x] = i;
            i += 1;
        });
        top += 1;

        (top..=bottom).for_each(|y| {
            results[y][right] = i;
            i += 1;
        });
        right -= 1;

        (left..=right).rev().for_each(|x| {
            results[bottom][x] = i;
            i += 1;
        });
        bottom -= 1;

        (top..=bottom).rev().for_each(|y| {
            results[y][left] = i;
            i += 1;
        });
        left += 1;
    }

    results
}
