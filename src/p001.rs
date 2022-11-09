/// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
/*
Find the sum of all the multiples of 3 or 5 below 1000.
*/
pub fn p001() -> i32 {
    let divisors: [i32; 2] = [3, 5];
    let mut multiples = Vec::new();
    p001_find_multiples(&mut multiples, &divisors, 1000);
    let answer = multiples.iter().sum();

    answer
}

fn p001_find_multiples(multiples: &mut Vec<i32>, divisors: &[i32], below: i32) -> () {
    for n in 1..below {
        for d in divisors.iter() {
            if n % d == 0 {
                multiples.push(n);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p001() {
        let expected: i32 = 233168;
        assert_eq!(p001(), expected);
    }
}