// Problem 4 : Largest palindrome product

// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-digit numbers.

#[cfg(test)]
mod tests {
    #[test]
    fn t4() {
        assert_eq!(9009, super::solve(2));
    }
}

fn is_palindromic_num(num: i32) -> bool {
    let s : String = num.to_string();
    let s2 : String = num.to_string()
        .chars().rev().collect::<String>();

    s == s2
}

fn solve(digits: i32) -> i32 {
    let max = 10i32.pow(digits as u32);
    let min = max - 10i32.pow(digits as u32 - 1);

    for i in (min..max).rev() {
        for j in (min..max).rev() {
            let x : i32 = i * j;
            if is_palindromic_num(x) {
                return x;
            }
        }
    }

    0
}

fn main() {
    println!("{}", solve(3));
}
