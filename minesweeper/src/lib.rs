use std::collections::HashSet;

const MINE_CHAR: char = '*';
const EMPTY_CHAR: char = ' ';

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut cells_with_mines = HashSet::new();

    minefield.iter().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, cell)| {
            if cell == MINE_CHAR {
                cells_with_mines.insert((x as i32, y as i32));
            }
        })
    });

    minefield
        .iter()
        .enumerate()
        .map(|(y, row)| {
            let y_pos = y as i32;
            let top = y_pos - 1;
            let bottom = y_pos + 1;

            row.chars()
                .enumerate()
                .map(|(x, cell)| {
                    if cell == MINE_CHAR {
                        return MINE_CHAR.to_string();
                    }

                    let x_pos = x as i32;
                    let left = x_pos - 1;
                    let right = x_pos + 1;

                    let num_neighbour_mines = vec![
                        (left, top),
                        (left, y_pos),
                        (left, bottom),
                        (x_pos, top),
                        (x_pos, bottom),
                        (right, top),
                        (right, y_pos),
                        (right, bottom),
                    ]
                    .iter()
                    .filter(|coords| cells_with_mines.contains(coords))
                    .count();

                    match num_neighbour_mines {
                        0 => EMPTY_CHAR.to_string(),
                        _ => num_neighbour_mines.to_string(),
                    }
                })
                .collect()
        })
        .collect()
}
