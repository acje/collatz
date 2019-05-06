fn main() {
    let mut max: u32 = 0;
    for i in 1..64_000_000 {
        let mut n: u64 = i;
        let mut up: u32 = 0;
        while n != 1 {
            match n & 1 == 0 {
                true => n = n / 2,
                false => {
                    up += 1;
                    n = (3 * n) + 1
                }
            }
        }
        if up > max {
            println!("{}: new max up: {}", i, up);
            max = up;
        };
    }
}
