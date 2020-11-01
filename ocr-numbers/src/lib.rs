#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

const NUMBERS: &[&[&str]] = &[
    &[
        " _ ",
        "| |",
        "|_|"
    ],
    &[
        "   ",
        "  |",
        "  |"
    ],
    &[
        " _ ",
        " _|",
        "|_ "
    ],
    &[
        " _ ",
        " _|",
        " _|"
    ],
    &[
        "   ",
        "|_|",
        "  |"
    ],
    &[
        " _ ",
        "|_ ",
        " _|"
    ],
    &[
        " _ ",
        "|_ ",
        "|_|"
    ],
    &[
        " _ ",
        "  |",
        "  |"
    ],
    &[
        " _ ",
        "|_|",
        "|_|"
    ],
    &[
        " _ ",
        "|_|",
        " _|"
    ]
];

pub fn convert(input: &str) -> Result<String, Error> {
    let all_rows = input.split('\n').collect::<Vec<_>>();
    let rows = all_rows.chunks_exact(4);

    let num_leftover_rows = rows.remainder().len();
    if num_leftover_rows > 0 {
        return Err(Error::InvalidRowCount(num_leftover_rows));
    }

    rows.map(|line_rows| {
        let num_columns = line_rows.first().unwrap().len();

        if num_columns % 3 != 0 {
            return Err(Error::InvalidColumnCount(num_columns));
        }

        Ok((0..num_columns)
            .step_by(3)
            .map(|x_start| {
                let x_end = x_start + 3;

                let num_index = NUMBERS.iter().position(|num_str| {
                    *num_str[0] == line_rows[0][x_start..x_end]
                        && *num_str[1] == line_rows[1][x_start..x_end]
                        && *num_str[2] == line_rows[2][x_start..x_end]
                });

                num_index.map_or_else(|| "?".to_string(), |num| num.to_string())
            })
            .collect::<String>())
    })
    .collect::<Result<Vec<_>, Error>>()
    .map(|result| result.join(","))
}
