/*
Problem 3: Largest Prime Factor

The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143?
 */

const PRIME_NUMBER:u64 = 600_851_475_143;
const SQRT_PRIME_NUMBER: u64 = 775_146;

pub fn problem_3() {
    let mut prime = PRIME_NUMBER;
    let mut divisor = 3;

    while divisor < SQRT_PRIME_NUMBER {
        // println!("Prime: {prime}, Divisor: {divisor}");
        if prime == divisor {
            println!("biggest prime: {prime}");
            break;
        }
        else if prime % divisor == 0 {
            prime = prime / divisor;
        }
        else {
            divisor += 2;
        }
    }
}

