// Implements http://rosettacode.org/wiki/Fibonacci_word
extern crate collections;

use entropy::shannon_entropy;
use std::iter::range_inclusive;
mod entropy {

    // Returns "amount" fibonacci words as a vector of tuples
    // The first value of the tuple is the length of the word
    // and the second one its entropy

    // The first two words (we need to add them manually because
    // they are the base of the sequence)

    // The rest of the words





    extern crate collections;
    use std::str::StrSlice;
    use collections::hashmap::HashMap;
    pub fn shannon_entropy(s: &str) -> f64 {
        let mut map = HashMap::<char, uint>::new();
        for c in s.chars() {
            map.insert_or_update_with(c, 1, |_, v| *v += 1);
        }
        let len = s.len() as f64;
        map.iter().fold(0f64, |mut acc, (_, nb)| { let p = (*nb as f64) / len;
                        acc -= p * p.log2(); acc })
    }
    #[allow(dead_code)]
    #[cfg(not(test))]
    fn main() { println!("{:f}" , shannon_entropy ( "1223334444" )); }
    #[test]
    fn test_entropy() {
        let tests =
            vec!(( "1223334444" , 1.846439344671f64 ) ,
                 ( "1223334444555555555" , 1.969811065121 ) ,
                 ( "122333" , 1.459147917061 ) ,
                 ( "1227774444" , 1.846439344671 ) ,
                 ( "aaBBcccDDDD" , 1.936260027482 ) ,
                 ( "1234567890abcdefghijklmnopqrstuvwxyz" , 5.169925004424 ) ,
                 ( "Rosetta Code" , 3.084962500407 ));
        let epsilon: f64 = 0.0000001;
        for (input, expected) in tests.move_iter() {
            let output = shannon_entropy(input);
            assert!(( output - expected ) . abs ( ) < epsilon);
        }
    }
}
fn fib_words(amount: uint) -> Vec<(uint, f64)> {
    let mut data = Vec::with_capacity(amount);
    let mut previous = StrBuf::from_str("1");
    let mut next = StrBuf::from_str("0");
    data.push((previous.len(), shannon_entropy(previous.as_slice())));
    data.push((next.len(), shannon_entropy(next.as_slice())));
    for _ in range_inclusive(3, amount) {
        let temp = next.clone();
        next.push_str(previous.as_slice());
        previous = temp;
        data.push((next.len(), shannon_entropy(next.as_slice())));
    }
    data
}
#[cfg(not(test))]
fn main() {
    println!("Calculating... This may take a couple of minutes...\n");
    let words = fib_words(18);
    let mut i = 1;
    println!("{:>2}:{:>10} {}" , "N" , "length" , "entropy");
    for &(length, entropy) in words.iter() {
        println!("{:>2i}:{:>10u} {:.15f}" , i , length , entropy);
        i += 1;
    }
}
#[test]
fn test_fibonacii_words() {
    let expected =
        vec!(( 1u , 0.000000000000000 ) , ( 1 , 0.000000000000000 ) ,
             ( 2 , 1.000000000000000 ) , ( 3 , 0.918295834054490 ) ,
             ( 5 , 0.970950594454669 ) , ( 8 , 0.954434002924965 ) ,
             ( 13 , 0.961236604722876 ) , ( 21 , 0.958711882977132 ) ,
             ( 34 , 0.959686893774217 ) , ( 55 , 0.959316032054378 ) ,
             ( 89 , 0.959457915838670 ) , ( 144 , 0.959403754221023 ) ,
             ( 233 , 0.959424446955987 ) , ( 377 , 0.959416543740441 ) ,
             ( 610 , 0.959419562603144 ) , ( 987 , 0.959418409515224 ) ,
             ( 1597 , 0.959418849957810 ) , ( 2584 , 0.959418681724032 ));
    let epsilon = 0.0000000001;
    let output = fib_words(18);
    for ((output_length, output_entropy), (expected_length, expected_entropy))
        in output.move_iter().zip(expected.move_iter()) {
        assert!(output_length == expected_length);
        assert!(( output_entropy - expected_entropy ) . abs ( ) < epsilon);
    }
}
