/// The prime factors of 13195 are 5, 7, 13 and 29.
///
/// What is the largest prime factor of the number 600851475143 ?
// **Efficient: Took 0.83s**
pub fn p003(n: u64) -> u64 {
    let mut factors: Vec<u64> = Vec::new();
    let max = (n as f64).sqrt().floor() + 1.0f64;
    for x in 7..(max as u64) {
        if p003_is_prime(x) {
            if n % x == 0 {
                let pair = n / x;
                factors.push(x);
                if p003_is_prime(pair) {
                    factors.push(pair);
                }
            }
        }
    }
    factors.sort();
    *factors.last().unwrap()
}

/// Find the first factor (other than 1) of a number
fn p003_first_fac(x: u64) -> u64 {
    if x % 2 == 0 {
        return 2;
    };
    // TODO: return to step_by
    // for n in (3..).step_by(2).take_while(|m| m*m <= x) {
    for n in (1..).map(|m| 2 * m + 1).take_while(|m| m * m <= x) {
        if x % n == 0 {
            return n;
        };
    }
    // No factor found. It must be prime.
    x
}

/// Test whether a number is prime. Checks every odd number up to `sqrt(n)`.
pub fn p003_is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    p003_first_fac(n) == n
}

/// **Brute force: Took 424.91s**
pub fn p003__ineffectient(input: i64) -> i64 {
    // let input: i64 = 600851475143;
    // let input: i64 = 600851;
    let primes = p003_find_primes(input);
    for x in primes.iter().rev() {
        if input % x == 0 {
            return *x;
        }
    }
    1
}

fn p003_find_primes(num: i64) -> Vec<i64> {
    let mut primes: Vec<i64> = vec![2, 3, 5];

    let max = (num as f64).sqrt().floor() + 1.0f64;
    // println!("max: {}", (max as i64));
    for i in 7..(max as i64) {
        // println!("i: {}", i);
        let mut is_prime = true;
        for p in &primes {
            // println!("i: {}", i);
            // println!("p: {}", p);
            if i % p == 0 {
                is_prime = false;
            }
        }
        if is_prime {
            // println!("prime: {}", i);
            primes.push(i);
        }
    }
    primes
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p003() {
        assert_eq!(p003(600851475143), 6857);
        // assert_eq!(p003(13195), 29);

    }
}
