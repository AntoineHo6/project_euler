use std::{cmp::max, fs};

pub fn problem_18() {
    let content: String = fs::read_to_string("src/problem_18_data.txt").expect("");

    let rows_str: Vec<&str> = content.lines().collect();

    let mut rows: Vec<Vec<i32>> = rows_str
                                        .iter()
                                        .map(
                                            |s_arr| s_arr.split_whitespace().map(
                                                |s| s.parse::<i32>().unwrap()
                                            ).collect()
                                        ).collect();

    for i in (0..=rows.len()-2).rev() {
        for j in 0..rows[i].len() {
            let left_child = rows[i][j] + rows[i+1][j];
            let right_child = rows[i][j] + rows[i+1][j+1];

            rows[i][j] = max(left_child, right_child); 

            print!("{} ", rows[i][j]);
        }
        println!(); 
    }

    println!("{}", rows[0][0]);
}
