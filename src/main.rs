fn main() {
    let mut n: u64 = 133713371337;
    while n != 1 {
        match n % 2 == 0 {
            true => n = n / 2,
            false => {
                println!("Up!");
                n = (3 * n) + 1
            }
        }
        println!("{}", n);
    }
}
