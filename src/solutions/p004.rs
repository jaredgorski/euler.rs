
/*
*   Problem 4
*
* A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
*
* Find the largest palindrome made from the product of two 3-digit numbers.
*/

pub fn main() {
    println!("Problem 4: {}", solve(3));
}

fn solve(digits: u8) -> u32 {
    let min: u32 = 10_u32.pow(digits as u32 - 1);
    let max: u32 = 10_u32.pow(digits as u32) - 1;
    let mut max_pal: u32 = 0;

    for num1 in min..=max {
        for num2 in min..=max {
            let prod: u32 = num1 * num2;
            let prod_rev: u32 = prod.to_string()
                .chars().rev().collect::<String>()
                .parse::<u32>().unwrap();

            if prod == prod_rev {
                if prod > max_pal {
                    max_pal = prod;
                }
            }
        }
    }

    return max_pal;
}

#[test]
fn p4_test() {
    assert_eq!(
        9009,
        solve(2)
    );

    assert_eq!(
        906609,
        solve(3)
    );
}

