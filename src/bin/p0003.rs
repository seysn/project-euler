// Problem 3 : Largest prime factor

// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

#[cfg(test)]
mod tests {
    #[test]
    fn t3() {
        assert_eq!(29, super::solve(13195));
    }

    #[test]
    fn t3_is_prime() {
        assert_eq!(true, super::is_prime(2));
        assert_eq!(false, super::is_prime(4));
        assert_eq!(true, super::is_prime(541));
        assert_eq!(true, super::is_prime(179426549));
        assert_eq!(false, super::is_prime(179426548));
    }
}

// fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
//     if modulus == 1 { return 0 }
//     let mut result = 1;
//     base = base % modulus;
//     while exp > 0 {
//         if exp % 2 == 1 {
//             result = result * base % modulus;
//         }
//         exp = exp >> 1;
//         base = base * base % modulus
//     }
//     result
// }

fn is_prime(num: u64) -> bool {
    if num == 2 || num == 3 {
        return true;
    }

    if num % 2 == 0 || num % 3 == 0 {
        return false;
    }

    let mut i = 5;
    let mut w = 2;

    while i * i <= num {
        if num % i == 0 {
            return false;
        }

        i += w;
        w = 6 - w;
    }

    true
}

fn solve(num: u64) -> u64 {
    let mut n = num;
    for i in 2..num {
        if is_prime(i) && n % i == 0 {
            n = n / i;
            if is_prime(n) {
                return n;
            }
        }
    }

    0
}

fn main() {
    println!("{}", solve(600_851_475_143));
}
