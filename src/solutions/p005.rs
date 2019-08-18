
/*
*   Problem 5
*
* 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
*
* What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
*/

pub fn main() {
    println!("Problem 5: {}", solve(20));
}

fn solve(max: u8) -> u32 {
    let mut num: u32 = 1;

    'outer: loop {
        let mut i: u8 = 1;
        'inner: while i <= max {
            if num % i as u32 != 0 {
                break 'inner;
            }

            if i == max && num % i as u32 == 0 {
                break 'outer;
            }

            i += 1;
        }

        num += 1;
    }

    return num;
}

#[test]
fn p5_test() {
    assert_eq!(
        2520,
        solve(10)
    );

    assert_eq!(
        232792560,
        solve(20)
    );
}


