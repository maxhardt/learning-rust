// kata solution

pub fn amount_of_pages(summary: u32) -> u32 {

    // max number of single digit pages 1..9 = 9        -> 1 * 9  = 9 total digits
    // max number of double digit pages 10..99 = 90     -> 2 * 90 = 180 total digits
    // max number of triple digit pages 100..999 = 900  -> 3 * 900 = 2700 total digits

    for n_digits_per_page in 1..10 {

        if summary <= max_digits_total(n_digits_per_page) {

            // summary = x * n_digits_per_page + max_digits_total(n_digits_per_page - 1)
            let x = ((summary - max_digits_total(n_digits_per_page - 1)) / n_digits_per_page) as u32;

            // total_pages = x + max_pages(n_digits_per_page - 1)
            let total_pages = x + max_pages(n_digits_per_page - 1);

            return total_pages;

        }
    }
    return 0;
}

// helper functions

fn max_digits_total(n: u32) -> u32 {
    // Computes the maxmimum possible number of total digits
    // in a book with a maximum of `n` digits in a single page.

    // Example for n = 3
    // 3 * 9 * 10^2 + 2 * 9 * 10^1 + 1 * 9 * 10^0 = 2889
    // for i = 1..n: Sum [i * 9 * 10^i-1]
    let mut total_digits: u32 = 0;
    for i in 1..n + 1 {
        total_digits += i * 9 * u32::pow(10, i-1)
    }
    return total_digits;
}

fn max_pages(n: u32) -> u32 {
    // Computes the maxmimum number of total pages
    // in a book with a maximum of `n` digits in a single page.

    // Example for n = 3
    // 9 * 10^2 + 9 * 10^1 + 9 * 10^0 = 999
    // for i = 1..n: Sum [9 * 10^i-1]
    let mut total_pages: u32 = 0;
    for i in 1..n + 1 {
        total_pages += 9 * u32::pow(10, i-1)
    }
    return total_pages;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_digits_total() {
        assert_eq!(max_digits_total(1), 9);
        assert_eq!(max_digits_total(2), 189);
        assert_eq!(max_digits_total(3), 2889);
    }

    #[test]
    fn test_max_pages() {
    assert_eq!(max_pages(1), 9);
        assert_eq!(max_pages(2), 99);
        assert_eq!(max_pages(3), 999);
    }
}
