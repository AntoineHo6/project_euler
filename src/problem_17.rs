const LETTERS_COUNT_1_TO_9: u32 = 36;
const LETTERS_COUNT_10_TO_19: u32 = 70;
const LETTERS_COUNT_IN_1000: u32 = 11;

pub fn problem_17() {
    /* 
    Count letters from numbers 20 to 99 inclusive
        Twenty: 6
        thirty: 6
        forty: 5
        fifty: 5
        sixty: 5
        seventy: 7
        eighty: 6
        ninety: 6

        They each appear 10 times. 
    */
    let letters_count_20_to_99 = 46*10 + LETTERS_COUNT_1_TO_9*8;

    /*
    Count letters from numbers 100 to 999 inclusive
        one hundred: 3 + 7 = 10 * 100
        two hundred: 3 + 7 = 10 * 100
        three hundred: 5 + 7 = 12 * 100
        four hundred: 4 + 7 = 11 * 100
        five hundred: 4 + 7 = 11 * 100
        six hundred: 3 + 7 = 10 * 100
        seven hundred: 5 + 7 = 12 * 100
        eight hundred: 5 + 7 = 12 * 100
        nine hundred: 4 + 7 = 11 * 100

        hundred's prefix count: (7 * 9) + 36 = 99*100

        

     */

    let hundreds_count = 99*100 + 9*99*3 + 9*(LETTERS_COUNT_1_TO_9 + LETTERS_COUNT_10_TO_19 + letters_count_20_to_99);

    let total = LETTERS_COUNT_1_TO_9 + LETTERS_COUNT_10_TO_19 + letters_count_20_to_99 + hundreds_count + LETTERS_COUNT_IN_1000;

    println!("Total: {}", total);
}