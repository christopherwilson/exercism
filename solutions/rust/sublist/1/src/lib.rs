#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let sublist_exists: bool;
    let first_list_bigger = first_list.len() >= second_list.len();
    if first_list_bigger {
        sublist_exists = is_sublist(first_list, second_list);
    } else {
        sublist_exists = is_sublist(second_list, first_list);
    }

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
    if small_list.len() == 0 {
        return true
    }

    for i in 0..(big_list.len() - small_list.len() + 1) {
        if big_list[i..i+small_list.len()] == *small_list {
            return true
        }
    }
    false
}
