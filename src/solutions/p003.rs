
/*
*   Problem 3
*   
*   The prime factors of 13195 are 5, 7, 13 and 29.
*
*   What is the largest prime factor of the number 600851475143 ?
*/

pub fn main() {
    println!("Problem 3: {}", solve(600851475143));
}

fn solve(num: u64) -> u64 {
    let mut max: u64 = 0;

    let mut i: u64 = 2;
    while i * i < num {
        if num % i == 0 && i > max {
            if is_prime(i) {
                max = i;
            }
        }

        i += 1;
    }

    return max;
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
fn p3_test() {
    assert_eq!(
        29,
        solve(13195)
    );

    assert_eq!(
        6857,
        solve(600851475143)
    );
}

