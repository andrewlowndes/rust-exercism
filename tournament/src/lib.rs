use std::cmp::Ordering;
use std::collections::HashMap;

const WIN_POINTS: u16 = 3;
const DRAW_POINTS: u16 = 1;

const LINE_SEPARATOR: char = '\n';
const COLUMN_SEPARATOR: char = ';';
const PADDING_CHAR: char = ' ';
const COLUMN_JOINT: &str = " | ";

const COLUMN_HEADINGS: &[&str] = &["Team", "MP", "W", "D", "L", "P"];
const COLUMN_WIDTHS: &[u16] = &[30, 2, 2, 2, 2, 2];
const COLUMN_ALIGNMENT: &[Alignment] = &[
    Alignment::LEFT,
    Alignment::RIGHT,
    Alignment::RIGHT,
    Alignment::RIGHT,
    Alignment::RIGHT,
    Alignment::RIGHT,
];

enum Alignment {
    LEFT,
    RIGHT,
}

struct Outcome<'a> {
    team_name: &'a str,
    played: u16,
    won: u16,
    drawn: u16,
    lost: u16,
    points: u16,
}

impl<'a> Outcome<'a> {
    pub fn new(team_name: &'a str) -> Self {
        Outcome {
            team_name,
            won: 0,
            drawn: 0,
            lost: 0,
            played: 0,
            points: 0,
        }
    }
}

pub fn pad_left(str: &str, pad_char: char, num: u16) -> String {
    let str_len = str.chars().count() as u16;

    if num > str_len {
        (vec![pad_char; (num - str_len) as usize])
            .into_iter()
            .collect::<String>()
            + str
    } else {
        str.to_string()
    }
}

pub fn pad_right(str: &str, pad_char: char, num: u16) -> String {
    let str_len = str.chars().count() as u16;

    if num > str_len {
        str.to_string()
            + &(vec![pad_char; (num - str_len) as usize])
                .into_iter()
                .collect::<String>()
    } else {
        str.to_string()
    }
}

pub fn create_table_row(values: &[&str]) -> String {
    values
        .iter()
        .enumerate()
        .map(|(column_index, value)| {
            let column_width = COLUMN_WIDTHS[column_index];

            match COLUMN_ALIGNMENT[column_index] {
                Alignment::LEFT => pad_right(value, PADDING_CHAR, column_width),
                Alignment::RIGHT => pad_left(value, PADDING_CHAR, column_width),
            }
        })
        .collect::<Vec<_>>()
        .join(COLUMN_JOINT)
}

pub fn tally(match_results: &str) -> String {
    let mut results: HashMap<&str, Outcome> = HashMap::new();

    match_results.split(LINE_SEPARATOR).for_each(|match_line| {
        if match_line.is_empty() {
            return;
        }

        let mut items = match_line.split(COLUMN_SEPARATOR);
        let first_team_name = items.next().unwrap();
        let second_team_name = items.next().unwrap();
        let game_result = items.next().unwrap();

        let first_team = results
            .entry(first_team_name)
            .or_insert_with(|| Outcome::new(first_team_name));
        first_team.played += 1;

        match game_result {
            "win" => {
                first_team.won += 1;
                first_team.points += WIN_POINTS;
            }
            "loss" => {
                first_team.lost += 1;
            }
            _ => {
                first_team.drawn += 1;
                first_team.points += DRAW_POINTS;
            }
        }

        let second_team = results
            .entry(second_team_name)
            .or_insert_with(|| Outcome::new(second_team_name));
        second_team.played += 1;

        match game_result {
            "win" => {
                second_team.lost += 1;
            }
            "loss" => {
                second_team.won += 1;
                second_team.points += WIN_POINTS;
            }
            _ => {
                second_team.drawn += 1;
                second_team.points += DRAW_POINTS;
            }
        }
    });

    let mut outcomes = results.values().collect::<Vec<_>>();

    outcomes.sort_unstable_by(|a, b| {
        let mut ordering = b.points.partial_cmp(&a.points).unwrap();

        if ordering == Ordering::Equal {
            ordering = a.team_name.partial_cmp(&b.team_name).unwrap();
        }

        ordering
    });

    let mut table = create_table_row(COLUMN_HEADINGS);

    if !outcomes.is_empty() {
        table += &(LINE_SEPARATOR.to_string()
            + &outcomes
                .iter()
                .map(|outcome| {
                    create_table_row(&[
                        outcome.team_name,
                        &outcome.played.to_string(),
                        &outcome.won.to_string(),
                        &outcome.drawn.to_string(),
                        &outcome.lost.to_string(),
                        &outcome.points.to_string(),
                    ])
                })
                .collect::<Vec<_>>()
                .join(&LINE_SEPARATOR.to_string()));
    }

    table
}
