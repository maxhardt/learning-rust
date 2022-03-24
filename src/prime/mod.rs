// kata solution

pub fn is_prime(x: i64) -> bool {

    // check if evenly divisible by all numbers between 2..sqrt(x)
    // which runs in worst case O(sqrt(x))

    // improvement: perform check only for prime numbers between 2..sqrt(x)
    // Primality Test: https://en.wikipedia.org/wiki/Primality_test

    if x <= 1 {
        return false;
    } else {
        let sqrt_x_upper = (x as f64).sqrt() as i64 + 1;
        for number in 2..sqrt_x_upper {
            if x % number == 0 {
                return false;
            }
        }
        return true;
    }
}
