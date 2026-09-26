pub fn annotate(garden: &[&str]) -> Vec<String> {
    let height = garden.len();
    if height == 0 {
        return vec![]
    }
    let width = garden[0].len();

    let mut filled_garden: Vec<Vec<u8>> = vec![vec![b' '; width]; height];

    for i in 0..height {
        let garden_row = garden[i].as_bytes();
        for j in 0..width {
            if garden_row[j] == b'*' {
                populate_neighbours(i, j, &mut filled_garden);
            }
        }
    }
    filled_garden.iter().map(|row| String::from_utf8(row.clone()).unwrap()).collect()
}

fn populate_neighbours(centre_row: usize, centre_col: usize, filled_garden: &mut Vec<Vec<u8>>) -> () {
    for i in -1i8..2 {
        let row_index = (centre_row as i8 + i) as usize;
        if let Some(row) = filled_garden.get_mut(row_index) {
            for j in -1i8..2 {
                let col_index = (centre_col as i8 + j) as usize;
                if let Some(val) = row.get_mut(col_index) {
                    if i == 0 && j == 0 {
                        *val = b'*'
                    } else if *val == b' ' {
                        *val = b'1'
                    } else if *val != b'*' {
                        *val += 1
                    }
                }
            }
        }
    }
}
