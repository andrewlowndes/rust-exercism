pub fn map<T, R>(input: Vec<T>, mut _function: impl FnMut(T) -> R) -> Vec<R> {
    input
        .into_iter()
        .map(|item| _function(item))
        .collect::<Vec<_>>()
}
