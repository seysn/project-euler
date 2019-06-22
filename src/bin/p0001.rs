#[cfg(test)]
mod tests {
    #[test]
    fn t1a() {
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
