/* 
    2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
    What is the smallest positive number that is evenly divisible with no remainder by all of the numbers from 1 to 20?
*/

pub fn problem_5 () {
    let mut num = 20;
    let mut found = true;

    loop {
        for x in 11..21 {
            if num % x != 0 {
                found = false;
                break;
            }
        }

        if found {
            println!("Answer: {num}");
            break
        }

        num += 1;
        found = true;
    }
}
