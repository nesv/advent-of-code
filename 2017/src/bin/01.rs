extern crate aoc2017;

use aoc2017::input;

fn main() {
    if let Some(data) = input::load(1) {
        let res1 = p1(data.clone().into_bytes());
        println!("{}", res1);

        let res2 = p2(data.clone().into_bytes());
        println!("{}", res2);
    }
}

// Find the sum of all digits that match the next digit in p.
// p is treated as circular, so the digit after the last digit is the first
// digit in p.
fn p1(p: Vec<u8>) -> i16 {
    let mut sum: i16 = 0;
    let n = p.len();
    for i in 0..n {
        if (i == n - 1 && p[i] == p[0]) || (p[i] == p[i+1]) {
            sum += p[i] as i16 - 48;
        } 
    }
    return sum;
}

// Instead of considering the next digit, consider the digit halfway around the
// circular list p.
// That is, if p contains 10 items, only include a digit in your sum if the
// digit 10/2 = 5 steps forward matches it.
fn p2(p: Vec<u8>) -> i16 {
    let mut sum: i16 = 0;
    let n = p.len();
    let h = n / 2;
    for i in 0..n {
        let mut j: usize = i + h;
        if i >= h {
            j = i - h;
        }
        if p[i] == p[j] {
            sum += p[i] as i16 - 48;
        }
    }
    return sum;
}
