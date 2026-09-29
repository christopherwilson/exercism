const TO_ENGLISH: [&str; 11] = [
    "no",
    "One",
    "Two",
    "Three",
    "Four",
    "Five",
    "Six",
    "Seven",
    "Eight",
    "Nine",
    "Ten",
];

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    // Assuming 10 is max number, since instructions and tests only go that far
    if start_bottles > 10 {
        panic!("start_bottles must be 10 or less");
    }

    let mut verses_recited = 0;
    let mut lines: Vec<String> = Vec::new();

    while take_down > verses_recited {
        let num_bottles = start_bottles - verses_recited;
        let opening: String = match num_bottles {
            1 => "One green bottle hanging on the wall,\n".to_string(),
            _ => format!("{} green bottles hanging on the wall,\n", TO_ENGLISH[num_bottles as usize]),
        };
        lines.push(opening.clone());
        lines.push(opening.clone());
        lines.push("And if one green bottle should accidentally fall,\n".to_string());
        let last = match num_bottles - 1 {
            1 => "There'll be one green bottle hanging on the wall.\n".to_string(),
            _ => format!("There'll be {} green bottles hanging on the wall.\n", TO_ENGLISH[(num_bottles - 1) as usize].to_lowercase()),
        };
        lines.push(last.clone());
        lines.push("\n".to_string());
        verses_recited += 1;
    }
    lines.into_iter().collect()
}
