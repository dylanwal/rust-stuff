fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for a in 2..n {
        if n % a == 0 {
            return false; // if it is not the last statement you need to use `return`
        }
    }
    true // last value to return
}

fn main() {
    let now = Instant::now();

    let mut x :bool = true;
    for i in 1..200_000 {
        x = is_prime(i);
    }
    sum_from_zero(1_000_000_000);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("Elapsed: {}", x);
}