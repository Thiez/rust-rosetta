// http://rosettacode.org/wiki/Sequence_of_non-squares
use std::iter::range_inclusive;

// the formula that should produce no perfect squares
fn non_sq(n: u64) -> u64 { (n + ( 0.5 + (n as f64).sqrt()) as u64) }

#[cfg(not(test))]
fn main() {
    // print the first 22 non squares
    for n in range_inclusive(1, 22).map(non_sq) {
        println!("{}", n);
    }
}

#[test]
fn test_no_squares() {
    // check if a number is a square
    let is_square = |n: u64| { let r = (n as f64).sqrt() as u64; r * r == n };
    // verify that there are no squares in the first million of
    // values calculated by non_sq
    for ns in range_inclusive(1, 1000000).map(non_sq) {
        assert!(!is_square(ns));
    }
}