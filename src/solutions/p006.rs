
/*
*   Problem 6
*
* The sum of the squares of the first ten natural numbers is,
*                                           1^2 + 2^2 + ... + 10^2 = 385
* The square of the sum of the first ten natural numbers is,
*                                           (1 + 2 + ... + 10)^2 = 55^2 = 3025
* 
* Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 
* 2640.
*
* Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
*/

pub fn main() {
    println!("Problem 6: {}", solve(100));
}

fn solve(nums: u32) -> u32 {
    let mut sum_sq: u32 = 0;
    let mut sq_sum: u32 = 0;

    for num in 1..=nums {
        sum_sq += num * num;
        sq_sum += num;

        if num == nums {
            sq_sum = sq_sum * sq_sum;
        }
    }

    return sq_sum - sum_sq;
}

#[test]
fn p6_test() {
    assert_eq!(
        2640,
        solve(10)
    );

    assert_eq!(
        25164150,
        solve(100)
    );
}


