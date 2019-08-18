
/*
*   Problem 9
*
* A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
* a2 + b2 = c2
*
* For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
*
* There exists exactly one Pythagorean triplet for which a + b + c = 1000.
* Find the product abc.
*/

pub fn main() {
    println!("Problem 9: {}", solve(12));
}

fn solve(goal: u32) -> u32 {
    let mut solution: u32 = 0;

    'outer: for num_a in 1..(goal / 3) {
        for num_b in (num_a + 1)..(goal / 2) {
            let num_a_sq: u32 = num_a * num_a;
            let num_b_sq: u32 = num_b * num_b;
            let num_c: u32 = goal - num_a - num_b;
            let num_c_sq: u32 = num_c * num_c;

            if num_a_sq + num_b_sq == num_c_sq {
                solution = num_a * num_b * num_c;
                break 'outer;
            }
        }
    }

    return solution;
}

#[test]
fn p9_test() {
    assert_eq!(
        60,
        solve(12)
    );

    assert_eq!(
        31875000,
        solve(1000)
    );
}
