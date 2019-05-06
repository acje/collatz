fn main() {
    let mut n: u64 = 133713371337;
    let mut up: u64 = 0;
    let mut down: u64 = 0;
    while n != 1 {
        match n % 2 == 0 {
            true => {down +=1;
            n = n / 2},
            false => {
                up += 1; 
                n = (3 * n) + 1
            }
        }
        println!("{}", n);
    }
    println!("Steps: {}, Up: {}, Down: {}", up+down, up, down);
}
