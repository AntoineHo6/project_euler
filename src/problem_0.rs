pub fn problem0() {
    let mut num: u64 = 1;
    let mut sum: u64 = 0;

    loop {
        if num % 2 != 0 {
            println!("{}", num);
            sum += num.pow(2);
        }
        
        num += 1;
        
        if num == 719000 {
            break;
        }
    }

    println!("{}", sum);
}