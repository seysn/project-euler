use std::vec::Vec;
use std::iter::Iterator;

#[cfg(test)]
mod tests {
    #[test]
    fn t2() {
        assert_eq!(44, super::solve(50));
    }
}

fn fibo_even(max: i32) -> Vec<i32> {
    let mut a = 1;
    let mut b = 2;
    let mut tmp;

    let mut res = vec![2];

    while b < max {
        tmp = a + b;
        a = b;
        b = tmp;

        if b % 2 == 0 {
            res.push(b);
        }
    }

    res
}

fn solve(max: i32) -> i32 {
    fibo_even(max).iter().sum()
}

fn main() {
    println!("{}", solve(4_000_000));
}
