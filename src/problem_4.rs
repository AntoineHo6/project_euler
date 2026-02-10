/*
Problem 4: 
 */

pub fn problem_4() {
    let mut big_palindrome: u64 = 0;

    for x in (1..1000).rev() {
        for y in (1..1000).rev() {
            let num = x * y;
            if is_palindrome(num) && num > big_palindrome {
                big_palindrome = num;
                println!("largest palindrome so far: {}", num);
            }
        }
    }
}

fn is_palindrome(num: u64) -> bool {
    let num_str = num.to_string();
    let num_str_rev = num_str.chars().rev().collect::<String>();

    num_str == num_str_rev
}
