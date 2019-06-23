// Problem 5 : Smallest multiple

// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

#[cfg(test)]
mod tests {
    #[test]
    fn t5() {
        assert_eq!(2520, super::solve(10));
    }
}

fn solve(max: i32) -> i32 {
    let numbers : Vec<i32> = (1..max + 1).collect();

    'main: for i in max..i32::max_value() {
        for n in &numbers {
            if i % n != 0 {
                continue 'main;
            }
        }
        return i;
    }

    0
}

fn main() {
    println!("{}", solve(20));
}
