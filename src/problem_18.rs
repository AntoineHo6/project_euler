use std::fs;

pub fn problem_18() {
    let content: String = fs::read_to_string("src/problem_18_data.txt").expect("");

    let rows_str: Vec<&str> = content.split("<br>").collect();

    let starting_row = rows_str.len() - 2;
    
    let mut rows: Vec<Vec<i32>> = rows_str
                                        .iter()
                                        .map(
                                            |s_arr| s_arr.split_whitespace().map(
                                                |s| s.parse::<i32>().unwrap()
                                            ).collect()
                                        ).collect();
    
    if let Some(row) = rows.get(starting_row) {
        for num in row {
            println!("Value: {}", num);
        }
    }
}

fn calculate_max_sum(row_idx: i32) {
    
}