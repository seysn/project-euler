// Problem 1 : Multiples of 3 and 5

// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.

#[cfg(test)]
mod tests {
    #[test]
    fn t1() {
        assert_eq!(23, super::solve(10));
        assert_eq!(33, super::solve(11));
    }
}

fn solve(max: i32) -> i32 {
    (1..max).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}

fn main() {
    let res : i32 = solve(1000);
    println!("{}", res);
}
