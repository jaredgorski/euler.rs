
/*
*   Problem 10
*
* The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
*
* Find the sum of all the primes below two million.
*/

pub fn main() {
    println!("Problem 9: {}", solve(2000000));
}

fn solve(limit: u32) -> u64 {
    let mut sum: u64 = 0;

    for i in 2..limit {
      if is_prime(i as u64) {
        sum += i as u64;
      }
    }

    return sum;
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
fn p10_test() {
    assert_eq!(
        17,
        solve(10)
    );

    assert_eq!(
        142913828922,
        solve(2000000)
    );
}
