#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let first_list_bigger = first_list.len() >= second_list.len();
    let sublist_exists: bool = match first_list_bigger {
        true => is_sublist(first_list, second_list),
        false => is_sublist(second_list, first_list)
    };

    let same_size = second_list.len() == first_list.len();

    if !sublist_exists {
        Comparison::Unequal
    } else if same_size {
        Comparison::Equal
    } else if first_list_bigger {
        Comparison::Superlist
    } else {
        Comparison::Sublist
    }
}

fn is_sublist(big_list: &[i32], small_list: &[i32]) -> bool {
    // empty list is always a sublist
    if small_list.is_empty() {
        return true
    }

    big_list.windows(small_list.len()).any(|x| x == small_list)
}
