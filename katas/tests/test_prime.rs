// final katas solution to test
use katas::is_prime;
use rand;
use rand::seq::SliceRandom;
use rand::Rng;

#[test]
fn basic_tests() {
    assert!(!is_prime(0), "0 is not prime");
    assert!(!is_prime(1), "1 is not prime");
    assert!(is_prime(2), "2 is prime");
    assert!(is_prime(73), "73 is prime");
    assert!(!is_prime(75), "75 is not prime");
    assert!(!is_prime(-1), "-1 is not prime");
}

#[test]
fn prime_tests() {
    assert!(is_prime(3), "3 is prime");
    assert!(is_prime(5), "5 is prime");
    assert!(is_prime(7), "7 is prime");
    assert!(is_prime(41), "41 is prime");
    assert!(is_prime(5099), "5099 is prime");
}

#[test]
fn not_prime_tests() {
    assert!(!is_prime(4), "4 is not prime");
    assert!(!is_prime(6), "6 is not prime");
    assert!(!is_prime(8), "8 is not prime");
    assert!(!is_prime(9), "9 is not prime");
    assert!(!is_prime(45), "45 is not prime");
    assert!(!is_prime(-5), "-5 is not prime");
    assert!(!is_prime(-8), "-8 is not prime");
    assert!(!is_prime(-41), "-41 is not prime");
}

fn get_small_primes() -> Vec<i64> {
    let count = 1 << 16;
    let mut sieve = vec![false; count];
    sieve[0] = true;
    sieve[1] = true;
    for idx in 2..256 {
        if sieve[idx] {
            continue;
        }
        let mut n = idx * idx;
        while n < count {
            sieve[n] = true;
            n += idx;
        }
    }

    sieve
        .iter()
        .enumerate()
        .filter_map(|(i, b)| if !*b { Some(i as i64) } else { None })
        .collect()
}

fn generate() -> Vec<(i64, bool)> {
    let mut rng = rand::thread_rng();
    let small_primes = get_small_primes();

    let check_prime = |num: i64| {
        if num < 3 || (num & 1 == 0) {
            return false;
        }
        let mx = (num as f64).sqrt().ceil() as i64;
        let mut idx = 0;
        while small_primes[idx] <= mx {
            if num % small_primes[idx] == 0 {
                return false;
            }
            idx += 1;
        }
        true
    };

    let next_prime = |num: i64| {
        let mut num = num;
        if num & 1 == 0 {
            num += 1;
        }
        while !check_prime(num) {
            num += 2;
        }
        num
    };

    let mut cases: Vec<(i64, bool)> = Vec::with_capacity(250 + 225 + 20 + 5);
    const MAX: i64 = 2000000000 + 1;

    for _ in 0..250 {
        let num = next_prime(rng.gen_range(5..MAX));
        cases.push((num, true));
    }

    for _ in 0..225 {
        let mut num = next_prime(rng.gen_range(5..MAX)) + 2;
        while check_prime(num) {
            num += 2;
        }
        cases.push((num, false));
    }

    for _ in 0..20 {
        let num = next_prime(rng.gen_range(5..1 + (1 << 15)));
        cases.push((num * num, false));
    }

    for _ in 0..5 {
        let num = next_prime(rng.gen_range(5..MAX));
        cases.push((-num, false));
    }

    cases.shuffle(&mut rng);
    cases
}

#[test]
fn random_tests() {
    let cases = generate();
    for case in cases {
        assert_eq!(
            is_prime(case.0),
            case.1,
            "{} is {}prime",
            case.0,
            if case.1 { "" } else { "not " }
        );
    }
}
