use primes::is_prime;
use regex::Regex;

fn primes(maximum: u64) -> Vec<u64> {
    let mut primes = vec![2];

    for candidate in 3..maximum {
        let square_root = (candidate as f64).sqrt() as u64 + 1;
        let is_prime = primes
            .iter()
            .take_while(|p| p <= &&square_root)
            .all(|p| candidate % p != 0);
        if is_prime {
            primes.push(candidate);
        }
    }

    primes
}

pub fn generate_primos_adri(digits: u8) -> Vec<u64> {
    let maximum: u64 = 10_u64.pow(digits.into());
    let is_primo_adri = Regex::new(format!(r"^[2,3,5,7]{{{}}}7$", digits - 1).as_str()).unwrap();

    let primos_adri: Vec<_> = primes(maximum)
        .iter()
        .filter(|p| is_primo_adri.is_match(&p.to_string()))
        .copied()
        .collect::<Vec<u64>>();

    primos_adri
}

pub fn check_if_primo_adri(number: u64) -> bool {
    let digits = ((number as f64).log(10.0).floor() as u32) + 1;
    let is_primo_adri = Regex::new(format!(r"^[2,3,5,7]{{{}}}7$", digits - 1).as_str())
        .unwrap()
        .is_match(&number.to_string());

    let is_prime = is_prime(number);

    is_prime && is_primo_adri
}

#[test]
fn primes_works() {
    let p = primes(10_u64);
    assert_eq!(p, vec![2, 3, 5, 7]);

    let p = primes(100_u64);
    assert_eq!(
        p,
        vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97
        ]
    );
}

#[test]
fn check_generate_primos_adri() {
    let pa = generate_primos_adri(5_u8);

    assert_eq!(pa.len(), 66);
    assert!(pa.contains(&22277_u64));
    assert!(pa.contains(&35257_u64));
    assert!(pa.contains(&75377_u64));
}

#[test]
fn check_primo_adri() {
    let pa = generate_primos_adri(5_u8);
    assert!(pa.iter().all(|x| check_if_primo_adri(*x)));

    let pa = generate_primos_adri(7_u8);
    assert!(pa.iter().all(|x| check_if_primo_adri(*x)));
}
