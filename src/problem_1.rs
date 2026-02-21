/* 
Problem 1: Multiples of 3 or 5

<p>If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.</p>
<p>Find the sum of all the multiples of 3 or 5 below 1000.</p>
*/

pub fn problem_1() {
    let mut num: u32 = 1;
    let mut sum: u32 = 0;
    
    loop {
        if num % 3 == 0 || num % 5 == 0 {
            sum += num;
        }

        num += 1;

        if num == 1000 {
            break;
        }
    }

    println!("{sum}");
}