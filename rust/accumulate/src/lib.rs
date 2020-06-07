/// What should the type of _function be?
pub fn map<T, S, F: FnMut(T) -> S>(input: Vec<T>, function: F) -> Vec<S> {
    input.into_iter()
        .map(function)
        .collect()
}
