/*
Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:

1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...

By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.
*/
pub fn p002() -> i32 {
    let fib = p002_fibonacci(4000000);
    let evens = fib.iter()
        .filter(|&x| x % 2 == 0)
        .copied()
        .collect::<Vec<i32>>(); 
    let answer = evens.iter().sum();

    answer
}

fn p002_fibonacci(below: i32) -> Vec<i32> {
    let mut fib = Vec::new();
    fib.push(1);
    fib.push(2);

    let mut done: bool = false;
    while !done {
        let x = fib[fib.len() - 1];
        let y = fib[fib.len() - 2];

        let n = x + y;
        if n > below {
            done = true;
        } else {
            fib.push(n);
        }
    }
    fib
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p002() {
        let expected: i32 = 4613732;
        assert_eq!(p002(), expected);
    }
}
