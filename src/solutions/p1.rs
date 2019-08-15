
/*
*   Problem 1
*   
*   If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
*
*   Find the sum of all the multiples of 3 or 5 below 1000.
*/

pub fn main() {
    println!("Problem 1: {}", solve(1000));
}

fn solve(limit: u32) -> u32 {
    let mut sum = 0;

    for i in 1..limit {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }

    return sum;
}

#[test]
fn p1_test() {
    assert_eq!(
        23,
        solve(10)
    );

    assert_eq!(
        233168,
        solve(1000)
    );
}
