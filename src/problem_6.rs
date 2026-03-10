/* 
    Sum square difference
*/

pub fn problem_6 () {
    let sum_of_squares: u64 = compute_sum_of_squares(100);
    let square_of_sum: u64 = compute_square_of_sum(100);
    println!("sum of the squares: {}", sum_of_squares);
    println!("Square of the sum: {}", square_of_sum);
    println!("Difference: {}", square_of_sum - sum_of_squares);
}

fn compute_sum_of_squares(num: u64) -> u64 {
    let mut sum: u64 = 0;
    
    for i in 1..=num {
        sum += i.pow(2);
    }

    return sum;
}

fn compute_square_of_sum(num: u64) -> u64 {
    let mut sum: u64 = 0;

    for i in 1..=num {
        sum += i;
    }

    return sum.pow(2);
}
