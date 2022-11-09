/*
A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
*/
pub fn p004() -> u32 {
    let mut largest: u32 = 0;
    for i in 100..1000 {
        for j in 100..1000 {
            let product = i * j;
            if p004_is_palidrome(product) && product > largest {
                largest = product;
            }
        }
    }
    largest
}


fn p004_is_palidrome(x: u32) -> bool {
    let (mut rev, mut org) = (0,x);
    
    while org>0 {
        rev = (rev*10) + org%10;
        org/=10;
    }
    
    rev == x
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p004() {
        assert_eq!(p004(), 906609u32);
        assert_eq!(p004_is_palidrome(906609u32), true);
    }
}