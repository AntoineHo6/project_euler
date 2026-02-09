/*
Problem 3: Largest Prime Factor

The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143?
 */

const PRIME_NUMBER:u64 = 600_851_475_143;
const SQRT_PRIME_NUMBER: u64 = 775_146;

pub fn problem_3() {
    let mut factors: Vec<u64> = Vec::new();

    let mut prime = PRIME_NUMBER;
    let mut divisor = 3;

    while divisor < SQRT_PRIME_NUMBER {
        if prime % divisor == 0 {
            factors.push(divisor);
            
            prime = prime / divisor;
            divisor = 3;
        }
        else {
            divisor += 2;
        }
    }

    if let Some(biggus_factor) = factors.iter().max() {
        println!("{biggus_factor}");
    }
}

