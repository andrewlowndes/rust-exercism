const CORNER_CHAR: char = '+';
const VERTICAL_CHAR: char = '|';
const HORIZONTAL_CHAR: char = '-';

pub fn count(lines: &[&str]) -> u32 {
    if lines.len() < 2 {
        return 0;
    }

    let num_columns = lines[0].len();
    let columns = (0..num_columns)
        .map(|column_index| {
            lines
                .iter()
                .map(|line| line.chars().nth(column_index).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut num_boxes = 0;

    lines.iter().enumerate().for_each(|(row_index, row)| {
        row.chars()
            .enumerate()
            .filter(|(_, char)| *char == CORNER_CHAR)
            .for_each(|(col_index, _)| {
                let column = &columns[col_index];

                //run to the right until we hit another corner, whilst we are on a line
                row.chars()
                    .enumerate()
                    .skip(col_index + 1)
                    .take_while(|(_, char)| *char == HORIZONTAL_CHAR || *char == CORNER_CHAR)
                    .filter(|(_, char)| *char == CORNER_CHAR)
                    .for_each(|(col_end_index, _)| {
                        //trace down and emit more checks if we hit more corners, whilst we are on a line
                        columns[col_end_index]
                            .iter()
                            .enumerate()
                            .skip(row_index + 1)
                            .take_while(|(_, char)| {
                                **char == VERTICAL_CHAR || **char == CORNER_CHAR
                            })
                            .filter(|(_, char)| **char == CORNER_CHAR)
                            .for_each(|(row_end_index, _)| {
                                //check the last corner
                                let has_last_corner = column[row_end_index] == CORNER_CHAR;

                                //ensure there are links for the last half of the box
                                let bottom_row = lines[row_end_index][col_index..col_end_index]
                                    .chars()
                                    .all(|char| char == HORIZONTAL_CHAR || char == CORNER_CHAR);
                                let left_col = columns[col_index][row_index..row_end_index]
                                    .iter()
                                    .all(|char| *char == VERTICAL_CHAR || *char == CORNER_CHAR);

                                if has_last_corner && bottom_row && left_col {
                                    num_boxes += 1;
                                }
                            });
                    });
            });
    });

    num_boxes
}
