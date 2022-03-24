// final katas solution to test
use katas::amount_of_pages;
use rand::distributions::{Distribution, Uniform};

const ADDING: [u32; 6] = [0, 9, 108, 1107, 11106, 111105];
const EDGES: [u32; 6] = [1, 11, 192, 2893, 38894, 488894];

fn author(mut summary: u32) -> u32 {
    for (i, n) in EDGES.into_iter().enumerate() {
        if n > summary {
            summary += ADDING[i - 1];
            return summary / (i as u32);
        }
    }
    unreachable!()
}

#[test]
fn example() {
    assert_eq!(amount_of_pages(5), 5);
    assert_eq!(amount_of_pages(25), 17);
    assert_eq!(amount_of_pages(1095), 401);
    assert_eq!(amount_of_pages(185), 97);
    assert_eq!(amount_of_pages(660), 256);
}

#[test]
fn edges() {
    assert_eq!(amount_of_pages(1), 1);
    assert_eq!(amount_of_pages(2893), 1000);
    assert_eq!(amount_of_pages(2889), 999);
}

#[test]
fn random() {
    let between = Uniform::from(12..100000);
    let mut rng = rand::thread_rng();
    for _ in 0..199 {
        let n = between.sample(&mut rng);
        assert_eq!(amount_of_pages(n), author(n));
    }
}
