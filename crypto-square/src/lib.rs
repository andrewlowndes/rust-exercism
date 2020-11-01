pub fn encrypt(input: &str) -> String {
    let normalised = input
        .chars()
        .filter(|char| char.is_ascii_alphanumeric())
        .map(|char| char.to_ascii_lowercase())
        .collect::<String>();

    let num_rows = (normalised.len() as f64).sqrt().floor() as usize;
    let num_columns = ((normalised.len() as f64) / (num_rows as f64)).ceil() as usize;
    let padding = vec![" "; ((num_columns * num_rows) - normalised.len()) as usize].join("");
    let content = normalised + &padding;

    (0..num_columns)
        .map(|start| {
            content
                .chars()
                .skip(start)
                .step_by(num_columns)
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}
