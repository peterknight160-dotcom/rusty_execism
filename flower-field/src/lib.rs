pub fn annotate(garden: &[&str]) -> Vec<String> {
    

    let mut gardens: Vec<Vec<char>> = garden.iter().map(|row| row.chars().collect()).collect();
    for (i, row) in garden.iter().enumerate() {
       
        for (j, col) in row.chars().enumerate() {
            if col == ' ' || col.is_ascii_digit() {
                for k in 0..=2 {
                    for l in 0..=2 {
                        if i  + k < garden.len()+1 
                            && i  + k > 0 
                            && j + l > 0 
                            && j  + l < row.len() + 1
                        {
                            let adjacent_col = garden[i + k - 1 ]
                                .chars()
                                .nth(j + l - 1 )
                                .unwrap();
                            if adjacent_col == '*' {
                                if gardens[i ][j ] == ' ' {
                                    gardens[i ][j ] = '1';
                                } else {
                                    gardens[i ][j ] =
                                        (gardens[i ][j ] as u8 + 1) as char;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    gardens
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect()
}
