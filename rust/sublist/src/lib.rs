#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (first_list.len(), second_list.len()) {
        (a, b) if a < b => is_sublist(first_list, second_list, Comparison::Sublist),
        (a, b) if a > b => is_sublist(second_list, first_list, Comparison::Superlist),
        _ => is_sublist(first_list, second_list, Comparison::Equal)
    }
}

fn is_sublist<T: PartialEq>(first_list: &[T], second_list: &[T], sublist: Comparison) -> Comparison {
    let size = first_list.len();
    if size == 0 {
        sublist
    } else {
        match second_list.windows(size)
            .any(|list| list == first_list) {
            true => sublist,
            false => Comparison::Unequal
        }
    }
}
