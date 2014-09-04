enum LuhnState {
    Even,
    Odd,
}

fn digits(n: uint) -> std::iter::Unfold<'static, uint, uint> {
    std::iter::Unfold::new(n, |state| {
        match *state {
            0 => None,
            n => {
                let ret = n % 10;
                *state = n / 10;
                Some(ret)
            },
        }
    })
}

fn luhn_test(n: uint) -> bool {
    let odd_even = [Odd, Even];
    let mut numbers = digits(n).zip(odd_even.iter().cycle().map(|&s|s));
    let sum = numbers.fold(0u, |s,n| {
        s + match n {
            (n, Odd) => n,
            (n, Even) => digits(2*n).fold(0, |s,n|s+n),
        }
    });
    sum % 10 == 0
}

#[cfg(not(test))]
fn main() {
    let nos = [49927398716, 49927398717, 1234567812345678, 1234567812345670];
	for n in nos.iter() {
		if luhn_test(*n) {
			println!("{} passes.", n);
		} else {
		    println!("{} fails.", n);
	    }
	}
}

#[test]
fn test_inputs() {
    assert!(luhn_test(49927398716));
    assert!(!luhn_test(49927398717));
    assert!(!luhn_test(1234567812345678));
    assert!(luhn_test(1234567812345670));
}
