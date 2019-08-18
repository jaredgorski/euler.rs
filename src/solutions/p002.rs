
/*
*   Problem 2
*   
*   Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:
*
*           1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
*
*   By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.
*/

pub fn main() {
    println!("Problem 2: {}", solve(4000000));
}

fn solve(limit: u32) -> u32 {
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    let mut sum: u32 = 0;

    loop {
        let c: u32 = a + b;
        a = b;
        b = c;
        
        if c > limit {
            break;
        }

        if c % 2 == 0 {
            sum += c;
        }
    }

    return sum;
}

#[test]
fn p2_test() {
    assert_eq!(
        44,
        solve(90)
    );

    assert_eq!(
        4613732,
        solve(4000000)
    );
}