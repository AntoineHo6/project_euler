// Problem 2: Even Fibonacci Numbers

const MAX_FIB_VALUE: u32 = 4_000_000;

pub fn problem2() {
    let mut term_1:u32 = 1;
    let mut term_2:u32 = 2;

    let mut sum_even_values = 0;

    while term_2 < MAX_FIB_VALUE {
        if term_2 % 2 == 0 {
            sum_even_values += term_2;
        }

        let temp = term_1;
        term_1 = term_2;

        term_2 += temp;
    }

    println!("{sum_even_values}");
}

