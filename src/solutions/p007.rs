
/*
*   Problem 7
*
* By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
*
* What is the 10,001st prime number?
*/

pub fn main() {
    println!("Problem 7: {}", solve(10001));
}

fn solve(max: u32) -> u64 {
  let mut num: u64 = 1;
  let mut count: u32 = 0;

  while count != max {
    num += 1;

    if is_prime(num) {
      count += 1;
    }
  }

  return num;
}

fn is_prime(n: u64) -> bool {
  // check for quick primes
  if n == 2 || n == 3 || n == 5 {
    return true;
  }

  if n % 2 == 0 { 
    return false;
  } else { // check factors up to and including square root of n
    let mut i: u64 = 2;
    while i * i <= n {
      if n % i == 0 {
        return false;
      }

      i = i + 1;
    }

    return true;
  }
}

#[test]
fn p7_test() {
    assert_eq!(
        13,
        solve(6)
    );

    assert_eq!(
        104743,
        solve(10001)
    );
}
