pub fn problem1() {
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