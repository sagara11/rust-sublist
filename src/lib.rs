#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let first_list_len = _first_list.len();
    let second_list_len = _second_list.len();

    let smaller_list = if first_list_len > second_list_len {
        _second_list
    } else {
        _first_list
    };

    let larger_list = if first_list_len > second_list_len {
        _first_list
    } else {
        _second_list
    };

    match first_list_len == 0 || second_list_len == 0 {
        true if first_list_len > second_list_len => Comparison::Superlist,
        true if first_list_len < second_list_len => Comparison::Sublist,
        _ => slide_patter(larger_list, smaller_list, _first_list, _second_list),
    }
}

fn slide_patter<T: PartialEq>(
    larger_list: &[T],
    smaller_list: &[T],
    first_list: &[T],
    second_list: &[T],
) -> Comparison {
    let remain_len = larger_list.len() - smaller_list.len();

    for i in 0..=remain_len {
        let mut counting = 0;

        for j in 0..smaller_list.len() {
            if larger_list.get(i + j).unwrap() != smaller_list.get(j).unwrap() {
                break;
            }

            counting += 1;
        }

        match counting == smaller_list.len() {
            true if first_list.len() > second_list.len() => return Comparison::Superlist,
            true if first_list.len() == second_list.len() => return Comparison::Equal,
            true if first_list.len() < second_list.len() => return Comparison::Sublist,
            _ => continue,
        }
    }

    Comparison::Unequal
}
